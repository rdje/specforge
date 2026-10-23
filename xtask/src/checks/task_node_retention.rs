//! The `TASK-NODE-RETENTION` doctrine (`TASK-NODE-RETENTION.0`), native since `RUSTDOC-AND-XTASK.13.b`: a task node
//! may be renamed into descendants or retired through `doctrine/task_nodes/removals.jsonl`, but it may not vanish.
//! Every node id `HEAD` carries under the task roots must still be in the working tree, have a descendant there
//! (`X` → `X.0`), or be declared removed. Reads `HEAD` through Git and the working tree, writes nothing.
//!
//! The registered script, `scripts/check_task_node_retention.py`, keeps its `argparse` front end (help,
//! abbreviations, refusals) and passes this check exactly one of `--check`, `--report` or `--self-test`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

use serde_json::Value;

/// The task roots whose Markdown carries node ids.
const ROOTS: [&str; 3] = ["docs/tasks", "docs/task-catalog", "docs/archive"];
const REMOVALS: &str = "doctrine/task_nodes/removals.jsonl";
const NODE_PREFIX: &str = "- ID: `";
const REGISTRY_FIELDS: [&str; 6] = [
    "record_type",
    "schema_version",
    "max_records",
    "max_record_bytes",
    "expected_removals",
    "milestones",
];
const REMOVAL_FIELDS: [&str; 6] = [
    "record_type",
    "schema_version",
    "id",
    "owning_leaf",
    "date",
    "reason",
];
/// The byte cap on one registry line, as the script hard-coded it.
const RECORD_BYTE_CAP: usize = 4096;
const EXPECTED_SELF_TEST_CASES: usize = 6;

/// Run the check from `root`: `--check` (the default), `--report` (the census as JSON) or `--self-test`.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    match arguments {
        [] => check(root),
        [mode] if mode == "--check" => check(root),
        [mode] if mode == "--report" => report(root),
        [mode] if mode == "--self-test" => self_test(),
        _ => {
            eprintln!(
                "Usage: cargo xtask check task-node-retention [--check|--report|--self-test]"
            );
            2
        }
    }
}

/// The removals registry as the script read it: its header, the declared removals by id, and every breach.
#[derive(Debug, Default)]
pub struct Removals {
    /// The `record_type: registry` header, or an empty object when there is none.
    pub registry: serde_json::Map<String, Value>,
    /// Each declared removal's id.
    pub declared: BTreeSet<String>,
    /// One line per shape error, in file order.
    pub breaches: Vec<String>,
}

/// Read `doctrine/task_nodes/removals.jsonl` under `root`.
pub fn load_removals(root: &Path) -> Removals {
    let mut removals = Removals::default();
    let path = root.join(REMOVALS);
    if !path.is_file() {
        removals.breaches.push(format!("{REMOVALS} is missing"));
        return removals;
    }
    let text = match fs::read(&path).map(String::from_utf8) {
        Ok(Ok(text)) => text,
        _ => {
            removals
                .breaches
                .push(format!("{REMOVALS} is not readable UTF-8"));
            return removals;
        }
    };
    for (index, line) in universal_lines(&text).iter().enumerate() {
        let number = index + 1;
        if line.trim().is_empty() {
            continue;
        }
        if line.len() > RECORD_BYTE_CAP {
            removals.breaches.push(format!(
                "{REMOVALS}:{number} exceeds the declared record byte cap"
            ));
            continue;
        }
        let record = match serde_json::from_str::<Value>(line) {
            Ok(Value::Object(record)) => record,
            Ok(_) => {
                removals
                    .breaches
                    .push(format!("{REMOVALS}:{number} is not a JSON object"));
                continue;
            }
            Err(error) => {
                removals
                    .breaches
                    .push(format!("{REMOVALS}:{number} is not valid JSON: {error}"));
                continue;
            }
        };
        match record.get("record_type").and_then(Value::as_str) {
            Some("registry") => {
                let unknown = unknown_fields(&record, &REGISTRY_FIELDS);
                if !unknown.is_empty() {
                    removals.breaches.push(format!(
                        "{REMOVALS}:{number} registry has unknown field(s): {}",
                        python_list(&unknown)
                    ));
                }
                removals.registry = record;
            }
            Some("removal") => {
                let unknown = unknown_fields(&record, &REMOVAL_FIELDS);
                if !unknown.is_empty() {
                    removals.breaches.push(format!(
                        "{REMOVALS}:{number} removal has unknown field(s): {}",
                        python_list(&unknown)
                    ));
                }
                let mut missing: Vec<String> = REMOVAL_FIELDS
                    .iter()
                    .filter(|field| !record.contains_key(**field))
                    .map(|field| field.to_string())
                    .collect();
                missing.sort();
                if !missing.is_empty() {
                    removals.breaches.push(format!(
                        "{REMOVALS}:{number} removal is missing: {}",
                        python_list(&missing)
                    ));
                    continue;
                }
                removals.declared.insert(python_str(&record["id"]));
            }
            _ => removals.breaches.push(format!(
                "{REMOVALS}:{number} has unknown record_type {}",
                python_repr(record.get("record_type").unwrap_or(&Value::Null))
            )),
        }
    }
    match removals.registry.get("expected_removals") {
        None | Some(Value::Null) => removals
            .breaches
            .push(format!("{REMOVALS} registry declares no expected_removals")),
        Some(expected) if !python_equals_count(expected, removals.declared.len()) => {
            removals.breaches.push(format!(
                "{REMOVALS} declares expected_removals {} but carries {} — a removal was added or dropped without moving the declared total",
                python_str(expected),
                removals.declared.len()
            ))
        }
        Some(_) => {}
    }
    removals
}

/// A file's lines as Python's text mode yields them: `\r\n` and a lone `\r` read as `\n`, each line keeping it.
fn universal_lines(text: &str) -> Vec<String> {
    text.replace("\r\n", "\n")
        .replace('\r', "\n")
        .split_inclusive('\n')
        .map(str::to_string)
        .collect()
}

fn unknown_fields(record: &serde_json::Map<String, Value>, known: &[&str]) -> Vec<String> {
    record
        .keys()
        .filter(|key| !known.contains(&key.as_str()))
        .cloned()
        .collect()
}

/// Whether a JSON value equals a count as Python's `==` compares them (`1.0 == 1`, `True == 1`).
fn python_equals_count(value: &Value, count: usize) -> bool {
    match value {
        Value::Number(number) => number.as_f64() == Some(count as f64),
        Value::Bool(flag) => usize::from(*flag) == count,
        _ => false,
    }
}

/// A JSON value as Python's `str()` prints the object `json.loads` made of it.
pub fn python_str(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        other => python_repr(other),
    }
}

/// A JSON value as Python's `repr()` prints the object `json.loads` made of it.
pub fn python_repr(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(true) => "True".to_string(),
        Value::Bool(false) => "False".to_string(),
        Value::Number(number) => number.to_string(),
        Value::String(text) => python_string_repr(text),
        Value::Array(items) => format!(
            "[{}]",
            items.iter().map(python_repr).collect::<Vec<_>>().join(", ")
        ),
        Value::Object(map) => format!(
            "{{{}}}",
            map.iter()
                .map(|(key, value)| format!("{}: {}", python_string_repr(key), python_repr(value)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

/// A list of strings as Python's `repr()` prints it.
pub fn python_list(items: &[String]) -> String {
    format!(
        "[{}]",
        items
            .iter()
            .map(|item| python_string_repr(item))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// A string as Python's `repr()` prints it: single quotes unless it holds one and no double quote.
fn python_string_repr(text: &str) -> String {
    let quote = if text.contains('\'') && !text.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::from(quote);
    for character in text.chars() {
        match character {
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            character if character == quote => {
                out.push('\\');
                out.push(character);
            }
            character if (character as u32) < 0x20 || character as u32 == 0x7f => {
                out.push_str(&format!("\\x{:02x}", character as u32))
            }
            character => out.push(character),
        }
    }
    out.push(quote);
    out
}

/// The node ids in `text`: each `` - ID: `…` `` at the start of a line, the id running to the next backtick, newlines
/// included, as the script's multi-line regex read it.
pub fn ids_in_text(text: &str) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    let mut position = 0;
    while position < text.len() {
        let line_start = position == 0 || text.as_bytes()[position - 1] == b'\n';
        if line_start && text[position..].starts_with(NODE_PREFIX) {
            let start = position + NODE_PREFIX.len();
            if let Some(length) = text[start..].find('`')
                && length > 0
            {
                ids.insert(text[start..start + length].to_string());
                position = start + length + 1;
                continue;
            }
        }
        position = match text[position..].find('\n') {
            Some(offset) => position + offset + 1,
            None => text.len(),
        };
    }
    ids
}

/// Every node id `HEAD` carries under the task roots, or `None` when there is no `HEAD`.
pub fn ids_at_head(root: &Path) -> Option<BTreeSet<String>> {
    let head = Command::new("git")
        .args(["rev-parse", "--verify", "HEAD"])
        .current_dir(root)
        .output()
        .ok()?;
    if !head.status.success() {
        return None;
    }
    let found = Command::new("git")
        .args(["grep", "-h", "-oE", "^- ID: `[^`]*`", "HEAD", "--"])
        .args(ROOTS)
        .current_dir(root)
        .output()
        .ok()?;
    Some(ids_in_text(&String::from_utf8_lossy(&found.stdout)))
}

/// Every node id in the working tree's task Markdown; `Err` names a file that is not UTF-8.
pub fn ids_in_worktree(root: &Path) -> Result<BTreeSet<String>, String> {
    let mut ids = BTreeSet::new();
    for top in ROOTS {
        collect_ids(&root.join(top), &mut ids)?;
    }
    Ok(ids)
}

fn collect_ids(directory: &Path, ids: &mut BTreeSet<String>) -> Result<(), String> {
    let Ok(entries) = fs::read_dir(directory) else {
        return Ok(());
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(kind) = entry.file_type() else {
            continue;
        };
        if kind.is_dir() {
            collect_ids(&path, ids)?;
        } else if kind.is_symlink() && path.is_dir() {
            // `os.walk` lists a link to a directory among the directories and does not follow it.
            continue;
        } else if path.to_string_lossy().ends_with(".md") {
            let bytes = fs::read(&path).map_err(|error| format!("{}: {error}", path.display()))?;
            let text = String::from_utf8(bytes)
                .map_err(|_| format!("{} is not valid UTF-8", path.display()))?;
            ids.extend(ids_in_text(&text));
        }
    }
    Ok(())
}

/// Every `HEAD` id that is gone from `present`, has no descendant there, and is not declared removed.
pub fn audit(
    head: Option<&BTreeSet<String>>,
    present: &BTreeSet<String>,
    declared: &BTreeSet<String>,
) -> Vec<String> {
    let Some(head) = head else {
        return Vec::new();
    };
    head.difference(present)
        .filter(|node| {
            let prefix = format!("{node}.");
            !present.iter().any(|other| other.starts_with(&prefix)) && !declared.contains(*node)
        })
        .cloned()
        .collect()
}

fn check(root: &Path) -> i32 {
    let mut removals = load_removals(root);
    let head = ids_at_head(root);
    let present = match ids_in_worktree(root) {
        Ok(present) => present,
        Err(message) => {
            eprintln!("task-node-retention: {message}");
            return 1;
        }
    };
    for node in audit(head.as_ref(), &present, &removals.declared) {
        removals.breaches.push(format!(
            "task node `{node}` is in HEAD and in no tracked task surface now. A node may be renamed into descendants or retired through doctrine/task_nodes/removals.jsonl; it may not vanish."
        ));
    }
    for breach in &removals.breaches {
        eprintln!("task-node-retention: {breach}");
    }
    if !removals.breaches.is_empty() {
        return 1;
    }
    let Some(head) = head else {
        println!("task-node-retention: no HEAD to compare against; nothing to retain yet.");
        return 0;
    };
    println!(
        "task-node-retention: {} task nodes present, {} moved or retired since HEAD, all accounted for ({} declared removal(s)).",
        present.len(),
        head.difference(&present).count(),
        removals.declared.len()
    );
    0
}

fn report(root: &Path) -> i32 {
    let removals = load_removals(root);
    let head = ids_at_head(root);
    let present = match ids_in_worktree(root) {
        Ok(present) => present,
        Err(message) => {
            eprintln!("task-node-retention: {message}");
            return 1;
        }
    };
    let vanished = audit(head.as_ref(), &present, &removals.declared);
    let census = BTreeMap::from([
        ("declared_removals", Value::from(removals.declared.len())),
        (
            "expected_removals",
            removals
                .registry
                .get("expected_removals")
                .cloned()
                .unwrap_or(Value::Null),
        ),
        (
            "head_nodes",
            head.as_ref()
                .map_or(Value::Null, |head| Value::from(head.len())),
        ),
        ("present_nodes", Value::from(present.len())),
        ("vanished_unaccounted", Value::from(vanished)),
    ]);
    let census: serde_json::Map<String, Value> = census
        .into_iter()
        .map(|(key, value)| (key.to_string(), value))
        .collect();
    println!("{}", python_json(&Value::Object(census), 0));
    0
}

/// A JSON value as Python's `json.dump(…, indent=2, sort_keys=True)` writes it, non-ASCII escaped.
pub fn python_json(value: &Value, depth: usize) -> String {
    let indent = |depth: usize| "  ".repeat(depth);
    match value {
        Value::Array(items) if items.is_empty() => "[]".to_string(),
        Value::Object(map) if map.is_empty() => "{}".to_string(),
        Value::Array(items) => format!(
            "[\n{}\n{}]",
            items
                .iter()
                .map(|item| format!("{}{}", indent(depth + 1), python_json(item, depth + 1)))
                .collect::<Vec<_>>()
                .join(",\n"),
            indent(depth)
        ),
        Value::Object(map) => {
            let sorted: BTreeMap<&String, &Value> = map.iter().collect();
            format!(
                "{{\n{}\n{}}}",
                sorted
                    .iter()
                    .map(|(key, value)| format!(
                        "{}{}: {}",
                        indent(depth + 1),
                        ascii_json_string(key),
                        python_json(value, depth + 1)
                    ))
                    .collect::<Vec<_>>()
                    .join(",\n"),
                indent(depth)
            )
        }
        Value::String(text) => ascii_json_string(text),
        other => other.to_string(),
    }
}

/// A JSON string literal with every non-ASCII character escaped, as Python's `ensure_ascii` writes it.
fn ascii_json_string(text: &str) -> String {
    let mut out = String::from('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            character if (character as u32) < 0x20 || !character.is_ascii() => {
                let mut units = [0u16; 2];
                for unit in character.encode_utf16(&mut units) {
                    out.push_str(&format!("\\u{unit:04x}"));
                }
            }
            character => out.push(character),
        }
    }
    out.push('"');
    out
}

type SelfTestCase = (&'static str, fn() -> Vec<String>, &'static [&'static str]);

fn set(ids: &[&str]) -> BTreeSet<String> {
    ids.iter().map(|id| id.to_string()).collect()
}

/// The script's discriminator cases, each with its expected verdict: the measured split (`3c17ae5c`) passes and the
/// measured deletion (`ab2c6ee0`) fails.
const SELF_TEST_CASES: [SelfTestCase; EXPECTED_SELF_TEST_CASES] = [
    (
        "a-split-into-descendants-is-not-a-loss",
        || {
            audit(
                Some(&set(&["A.7.2"])),
                &set(&["A.7.2.0", "A.7.2.1"]),
                &set(&[]),
            )
        },
        &[],
    ),
    (
        "an-outright-deletion-is-a-loss",
        || {
            audit(
                Some(&set(&["A.3j", "A.3k.2c"])),
                &set(&["A.3k.2j"]),
                &set(&[]),
            )
        },
        &["A.3j", "A.3k.2c"],
    ),
    (
        "a-declared-removal-is-admitted-only-for-its-own-id",
        || audit(Some(&set(&["A.1", "A.2"])), &set(&[]), &set(&["A.1"])),
        &["A.2"],
    ),
    (
        "a-sibling-descendant-does-not-cover-a-lost-node",
        || audit(Some(&set(&["A.1"])), &set(&["A.2.0"]), &set(&[])),
        &["A.1"],
    ),
    (
        "a-longer-sibling-id-is-not-a-descendant",
        || audit(Some(&set(&["A.1"])), &set(&["A.10"]), &set(&[])),
        &["A.1"],
    ),
    (
        "no-head-is-not-a-breach",
        || audit(None, &set(&["A.1"]), &set(&[])),
        &[],
    ),
];

fn self_test() -> i32 {
    for (case_id, run, expected) in SELF_TEST_CASES {
        let actual = run();
        if actual != *expected {
            let expected: Vec<String> = expected.iter().map(|id| id.to_string()).collect();
            eprintln!(
                "self-test FAIL {case_id}: expected {}, got {}",
                python_list(&expected),
                python_list(&actual)
            );
            return 1;
        }
    }
    println!(
        "task-node-retention: {EXPECTED_SELF_TEST_CASES}/{EXPECTED_SELF_TEST_CASES} cases pass."
    );
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_self_test_case_keeps_its_verdict() {
        for (case_id, run, expected) in SELF_TEST_CASES {
            assert_eq!(run(), *expected, "{case_id}");
        }
        assert_eq!(self_test(), 0);
    }

    #[test]
    fn node_ids_start_a_line_and_run_to_the_next_backtick() {
        let text = "- ID: `A.1`\n  - ID: `indented`\n- ID: ``\n- ID: `wrapped\nid`\n- ID: `B` and - ID: `inline`";
        assert_eq!(ids_in_text(text), set(&["A.1", "wrapped\nid", "B"]));
    }

    #[test]
    fn python_formatting_is_reproduced() {
        assert_eq!(
            python_list(&["alpha".to_string(), "it's".to_string()]),
            "['alpha', \"it's\"]"
        );
        assert_eq!(python_repr(&Value::Null), "None");
        assert_eq!(python_repr(&serde_json::json!("tombstone")), "'tombstone'");
        assert_eq!(python_str(&serde_json::json!("1")), "1");
        assert!(!python_equals_count(&serde_json::json!("1"), 1));
        assert!(python_equals_count(&serde_json::json!(1.0), 1));
        assert!(python_equals_count(&serde_json::json!(true), 1));
        let census = serde_json::json!({"b": [], "a": ["x", "é"], "c": null});
        assert_eq!(
            python_json(&census, 0),
            "{\n  \"a\": [\n    \"x\",\n    \"\\u00e9\"\n  ],\n  \"b\": [],\n  \"c\": null\n}"
        );
    }

    #[test]
    fn python_text_mode_lines_are_reproduced() {
        assert_eq!(universal_lines("a\r\nb\rc\nd"), ["a\n", "b\n", "c\n", "d"]);
    }

    #[test]
    fn each_registry_refusal_names_its_problem() {
        let root =
            crate::repository_root().join(".project-data/tmp/xtask-task-node-retention-registry");
        let breaches = |text: Option<&str>| -> Vec<String> {
            let _ = fs::remove_dir_all(&root);
            fs::create_dir_all(root.join("doctrine/task_nodes")).expect("fixture");
            if let Some(text) = text {
                fs::write(root.join(REMOVALS), text).expect("registry");
            }
            let found = load_removals(&root).breaches;
            fs::remove_dir_all(&root).expect("cleanup");
            found
        };
        let header = "{\"record_type\":\"registry\",\"expected_removals\":0}\n";
        let removal = "{\"record_type\":\"removal\",\"schema_version\":1,\"id\":\"A.1\",\"owning_leaf\":\"B\",\"date\":\"d\",\"reason\":\"r\"}\n";
        let at = |line: usize, rest: &str| format!("{REMOVALS}:{line} {rest}");
        assert_eq!(breaches(None), [format!("{REMOVALS} is missing")]);
        assert_eq!(breaches(Some(header)), Vec::<String>::new());
        assert_eq!(
            breaches(Some(&format!("{header}{{\"record_type\": x}}\n"))),
            [at(
                2,
                "is not valid JSON: expected value at line 1 column 17"
            )]
        );
        let long = format!("{header}\"{}\"\n", "x".repeat(RECORD_BYTE_CAP));
        assert_eq!(
            breaches(Some(&long)),
            [at(2, "exceeds the declared record byte cap")]
        );
        assert_eq!(
            breaches(Some(
                "{\"record_type\":\"registry\",\"expected_removals\":0,\"zeta\":1,\"alpha\":2}\n"
            )),
            [at(1, "registry has unknown field(s): ['alpha', 'zeta']")]
        );
        assert_eq!(
            breaches(Some(&format!(
                "{header}{{\"record_type\":\"removal\",\"schema_version\":1}}\n"
            ))),
            [at(
                2,
                "removal is missing: ['date', 'id', 'owning_leaf', 'reason']"
            ),]
        );
        assert_eq!(
            breaches(Some(&format!("{header}{{\"id\":\"x\"}}\n"))),
            [at(2, "has unknown record_type None")]
        );
        assert_eq!(
            breaches(Some(&format!(
                "{}{removal}",
                header.replace(":0", ":\"1\"")
            ))),
            [format!(
                "{REMOVALS} declares expected_removals 1 but carries 1 — a removal was added or dropped without moving the declared total"
            )]
        );
        assert_eq!(
            breaches(Some("{\"record_type\":\"registry\"}\n")),
            [format!("{REMOVALS} registry declares no expected_removals")]
        );
    }

    #[test]
    fn the_recorded_registry_reads_cleanly() {
        let removals = load_removals(&crate::repository_root());
        assert!(removals.breaches.is_empty(), "{:?}", removals.breaches);
    }

    #[test]
    fn an_unknown_mode_exits_2() {
        assert_eq!(run(&crate::repository_root(), &["--self".to_string()]), 2);
    }
}
