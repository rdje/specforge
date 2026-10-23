//! The `TASK-NODE-RETENTION` doctrine (`TASK-NODE-RETENTION.0`), native since `RUSTDOC-AND-XTASK.13.b`: a task node
//! may be renamed into descendants or retired through `doctrine/task_nodes/removals.jsonl`, but it may not vanish.
//! Every node id `HEAD` carries under the task roots must still be in the working tree, have a descendant there
//! (`X` → `X.0`), or be declared removed. Reads `HEAD` through Git and the working tree, writes nothing.
//!
//! The registered script, `scripts/check_task_node_retention.py`, keeps its `argparse` front end (help,
//! abbreviations, refusals) and passes this check exactly one of `--check`, `--report` or `--self-test`.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::pyjson::{self, Py};

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

/// The removals registry as the script read it: its header, the declared removals, and every breach.
#[derive(Debug, Default)]
pub struct Removals {
    /// The `record_type: registry` header, when there is one.
    pub registry: Option<Py>,
    /// Each declared removal's id: the keys of the script's `dict`, so an id declared twice counts once.
    pub declared: Vec<Py>,
    /// One line per shape error, in file order.
    pub breaches: Vec<String>,
}

impl Removals {
    /// The declared ids that are strings, the only ones a node id can match.
    pub fn declared_ids(&self) -> BTreeSet<String> {
        self.declared
            .iter()
            .filter_map(|id| match id {
                Py::Str(text) => Some(pyjson::to_text(text)),
                _ => None,
            })
            .collect()
    }
}

/// Read `doctrine/task_nodes/removals.jsonl` under `root`, parsing each line as Python's `json.loads` does.
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
        let record = match pyjson::loads(line) {
            Ok(record @ Py::Dict(_)) => record,
            Ok(_) => {
                removals
                    .breaches
                    .push(format!("{REMOVALS}:{number} is not a JSON object"));
                continue;
            }
            Err(message) => {
                removals
                    .breaches
                    .push(format!("{REMOVALS}:{number} is not valid JSON: {message}"));
                continue;
            }
        };
        let kind = pyjson::get(&record, "record_type");
        let is = |name: &str| matches!(kind, Some(Py::Str(text)) if *text == pyjson::py_str(name));
        if is("registry") {
            let unknown = unknown_fields(&record, &REGISTRY_FIELDS);
            if !unknown.is_empty() {
                removals.breaches.push(format!(
                    "{REMOVALS}:{number} registry has unknown field(s): {}",
                    string_list(unknown)
                ));
            }
            removals.registry = Some(record);
        } else if is("removal") {
            let unknown = unknown_fields(&record, &REMOVAL_FIELDS);
            if !unknown.is_empty() {
                removals.breaches.push(format!(
                    "{REMOVALS}:{number} removal has unknown field(s): {}",
                    string_list(unknown)
                ));
            }
            let mut missing: Vec<pyjson::PyStr> = REMOVAL_FIELDS
                .iter()
                .filter(|field| pyjson::get(&record, field).is_none())
                .map(|field| pyjson::py_str(field))
                .collect();
            missing.sort();
            if !missing.is_empty() {
                removals.breaches.push(format!(
                    "{REMOVALS}:{number} removal is missing: {}",
                    string_list(missing)
                ));
                continue;
            }
            let id = pyjson::get(&record, "id").expect("checked present").clone();
            if !removals
                .declared
                .iter()
                .any(|known| pyjson::py_eq(known, &id))
            {
                removals.declared.push(id);
            }
        } else {
            removals.breaches.push(format!(
                "{REMOVALS}:{number} has unknown record_type {}",
                pyjson::repr(kind.unwrap_or(&Py::None))
            ));
        }
    }
    let count = Py::Int(removals.declared.len().to_string());
    match removals
        .registry
        .as_ref()
        .and_then(|registry| pyjson::get(registry, "expected_removals"))
    {
        None | Some(Py::None) => removals
            .breaches
            .push(format!("{REMOVALS} registry declares no expected_removals")),
        Some(expected) if !pyjson::py_eq(expected, &count) => removals.breaches.push(format!(
            "{REMOVALS} declares expected_removals {} but carries {} — a removal was added or dropped without moving the declared total",
            pyjson::str_of(expected),
            removals.declared.len()
        )),
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

/// A record's keys outside `known`, sorted as Python sorts a set of strings.
fn unknown_fields(record: &Py, known: &[&str]) -> Vec<pyjson::PyStr> {
    let Py::Dict(pairs) = record else {
        return Vec::new();
    };
    let mut unknown: Vec<pyjson::PyStr> = pairs
        .iter()
        .map(|(key, _)| key.clone())
        .filter(|key| !known.iter().any(|field| pyjson::py_str(field) == *key))
        .collect();
    unknown.sort();
    unknown
}

/// A list of strings as Python prints it.
fn string_list(items: Vec<pyjson::PyStr>) -> String {
    pyjson::repr(&Py::List(items.into_iter().map(Py::Str).collect()))
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
    for node in audit(head.as_ref(), &present, &removals.declared_ids()) {
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
    let vanished = audit(head.as_ref(), &present, &removals.declared_ids());
    let count = |size: usize| Py::Int(size.to_string());
    let expected = removals
        .registry
        .as_ref()
        .and_then(|registry| pyjson::get(registry, "expected_removals"))
        .cloned()
        .unwrap_or(Py::None);
    let census = Py::Dict(vec![
        (
            pyjson::py_str("declared_removals"),
            count(removals.declared.len()),
        ),
        (pyjson::py_str("expected_removals"), expected),
        (
            pyjson::py_str("head_nodes"),
            head.as_ref().map_or(Py::None, |head| count(head.len())),
        ),
        (pyjson::py_str("present_nodes"), count(present.len())),
        (
            pyjson::py_str("vanished_unaccounted"),
            Py::List(
                vanished
                    .iter()
                    .map(|node| Py::Str(pyjson::py_str(node)))
                    .collect(),
            ),
        ),
    ]);
    // `json.dump(…, indent=2, sort_keys=True)`: the separators an indent implies, non-ASCII escaped.
    let encoder = pyjson::Encoder {
        indent: Some(2),
        sort_keys: true,
        ensure_ascii: true,
        item_separator: ",",
        key_separator: ": ",
    };
    println!("{}", pyjson::dumps(&census, &encoder).expect("ASCII-safe"));
    0
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
            let list =
                |ids: Vec<String>| string_list(ids.iter().map(|id| pyjson::py_str(id)).collect());
            eprintln!(
                "self-test FAIL {case_id}: expected {}, got {}",
                list(expected.iter().map(|id| id.to_string()).collect()),
                list(actual)
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
    fn the_census_prints_as_python_wrote_it() {
        let ids =
            |items: &[&str]| string_list(items.iter().map(|item| pyjson::py_str(item)).collect());
        assert_eq!(ids(&["alpha", "it's"]), "['alpha', \"it's\"]");
        let text = |value: &str| Py::Str(pyjson::py_str(value));
        let census = Py::Dict(vec![
            (pyjson::py_str("b"), Py::List(Vec::new())),
            (pyjson::py_str("a"), Py::List(vec![text("x"), text("é")])),
            (pyjson::py_str("c"), Py::None),
        ]);
        let encoder = pyjson::Encoder {
            indent: Some(2),
            sort_keys: true,
            ensure_ascii: true,
            item_separator: ",",
            key_separator: ": ",
        };
        assert_eq!(
            pyjson::dumps(&census, &encoder).as_deref(),
            Some(
                "{\n  \"a\": [\n    \"x\",\n    \"\\u00e9\"\n  ],\n  \"b\": [],\n  \"c\": null\n}"
            )
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
                "is not valid JSON: Expecting value: line 1 column 17 (char 16)"
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
