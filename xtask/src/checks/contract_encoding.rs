//! The `CONTRACT-ENCODING` doctrine (`LIVE-DOCUMENT-PRESSURE-HEADROOM.39a`), native since `RUSTDOC-AND-XTASK.13.c`: a
//! doctrine contract may not be silently re-encoded. Every JSON contract under `doctrine/` declares, in
//! `doctrine/contract_encoding/encoders.jsonl`, the `json.dumps` options that wrote it, or that it is hand-authored,
//! and must still round-trip under that declaration; a deliberate migration updates the record in the same commit.
//!
//! The registered script, `scripts/check_contract_encoding.py`, keeps its `argparse` front end and passes this check
//! the flags it was given: `--check`, `--report`, `--write` and `--self-test`.

use std::path::Path;
use std::process::Command;
use std::{fs, process};

use crate::pyjson::{self, Encoder, Py};

const REGISTRY: &str = "doctrine/contract_encoding/encoders.jsonl";
const INDENTS: [Option<usize>; 5] = [None, Some(1), Some(2), Some(3), Some(4)];
const SEPARATORS: [(&str, &str); 2] = [(",", ": "), (",", " : ")];

/// The `json.dumps` options that reproduce a contract, as `derive_encoder` reports them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Derived {
    /// The options.
    pub encoder: Encoder,
    /// Whether the file ends with one newline after the encoded text.
    pub trailing_newline: bool,
}

impl Derived {
    /// The encoder as the registry records it: `indent`, `sort_keys`, `ensure_ascii`, `separators`,
    /// `trailing_newline`, in that insertion order.
    pub fn record(&self) -> Py {
        let text = |value: &str| Py::Str(pyjson::py_str(value));
        Py::Dict(vec![
            (
                pyjson::py_str("indent"),
                self.encoder
                    .indent
                    .map_or(Py::None, |indent| Py::Int(indent.to_string())),
            ),
            (
                pyjson::py_str("sort_keys"),
                Py::Bool(self.encoder.sort_keys),
            ),
            (
                pyjson::py_str("ensure_ascii"),
                Py::Bool(self.encoder.ensure_ascii),
            ),
            (
                pyjson::py_str("separators"),
                Py::List(vec![
                    text(self.encoder.item_separator),
                    text(self.encoder.key_separator),
                ]),
            ),
            (
                pyjson::py_str("trailing_newline"),
                Py::Bool(self.trailing_newline),
            ),
        ])
    }
}

/// The first of the script's 80 option sets, in its order, whose `json.dumps` output (with or without one trailing
/// newline) is exactly `raw`; `None` when none is, or `raw` does not parse.
pub fn derive_encoder(raw: &[u8]) -> Option<Derived> {
    let value = pyjson::loads_bytes(raw).ok()?;
    for indent in INDENTS {
        for sort_keys in [true, false] {
            for ensure_ascii in [true, false] {
                for (item_separator, key_separator) in SEPARATORS {
                    let encoder = Encoder {
                        indent,
                        sort_keys,
                        ensure_ascii,
                        item_separator,
                        key_separator,
                    };
                    let Some(body) = pyjson::dumps(&value, &encoder) else {
                        continue;
                    };
                    for trailing_newline in [true, false] {
                        let matches = match raw.strip_prefix(body.as_bytes()) {
                            Some(rest) if trailing_newline => rest == b"\n",
                            Some(rest) => rest.is_empty(),
                            None => false,
                        };
                        if matches {
                            return Some(Derived {
                                encoder,
                                trailing_newline,
                            });
                        }
                    }
                }
            }
        }
    }
    None
}

/// `encoder_label`: the declared or derived encoder as the script names it.
fn label(encoder: Option<&Py>) -> String {
    let Some(encoder) = encoder.filter(|encoder| !matches!(encoder, Py::None)) else {
        return "hand-authored (no encoder reproduces it)".to_string();
    };
    let field = |name: &str| pyjson::get(encoder, name).cloned().unwrap_or(Py::None);
    let separators = match field("separators") {
        Py::List(items) if items.len() == 1 => format!("({},)", pyjson::repr(&items[0])),
        Py::List(items) => format!(
            "({})",
            items
                .iter()
                .map(pyjson::repr)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        other => pyjson::repr(&other),
    };
    format!(
        "indent={} sort_keys={} ensure_ascii={} separators={separators} trailing_newline={}",
        pyjson::str_of(&field("indent")),
        pyjson::str_of(&field("sort_keys")),
        pyjson::str_of(&field("ensure_ascii")),
        pyjson::str_of(&field("trailing_newline"))
    )
}

/// The JSON contracts under `base`: `doctrine/**/*.json` that are files, as repository-relative POSIX paths, sorted.
pub fn contracts(base: &Path) -> Vec<String> {
    let mut found = Vec::new();
    collect(base, &base.join("doctrine"), &mut found);
    found.sort();
    found
}

fn collect(base: &Path, directory: &Path, found: &mut Vec<String>) {
    let Ok(entries) = fs::read_dir(directory) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(kind) = entry.file_type() else {
            continue;
        };
        // `**` does not follow a link to a directory; `*.json` matches any name, then `is_file()` follows links.
        if kind.is_dir() {
            collect(base, &path, found);
        }
        if entry.file_name().to_string_lossy().ends_with(".json")
            && path.is_file()
            && let Ok(relative) = path.strip_prefix(base)
        {
            found.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// The registry's records by path, in file order, and its shape errors.
fn read_registry(base: &Path, errors: &mut Vec<String>) -> Vec<(String, Py)> {
    let path = base.join(REGISTRY);
    if !path.exists() {
        errors.push(format!("registry '{REGISTRY}' is missing"));
        return Vec::new();
    }
    let text = match fs::read(&path).map(String::from_utf8) {
        Ok(Ok(text)) => text.replace("\r\n", "\n").replace('\r', "\n"),
        _ => {
            errors.push(format!("registry '{REGISTRY}' is not readable UTF-8"));
            return Vec::new();
        }
    };
    let mut records: Vec<(String, Py)> = Vec::new();
    for (index, line) in text.split('\n').enumerate() {
        let number = index + 1;
        if line.trim().is_empty() {
            continue;
        }
        let record = match pyjson::loads(line) {
            Ok(record) => record,
            Err(message) => {
                errors.push(format!("registry line {number} is not JSON: {message}"));
                continue;
            }
        };
        let kind = pyjson::get(&record, "record_type");
        if !matches!(kind, Some(Py::Str(text)) if *text == pyjson::py_str("contract")) {
            errors.push(format!(
                "registry line {number} has unknown record_type '{}'",
                pyjson::str_of(kind.unwrap_or(&Py::None))
            ));
            continue;
        }
        let target = pyjson::get(&record, "path");
        let Some(target) = target.filter(|target| pyjson::truthy(target)) else {
            errors.push(format!("registry line {number} lacks a path"));
            continue;
        };
        let target = pyjson::str_of(target);
        match records.iter_mut().find(|(path, _)| *path == target) {
            Some(existing) => {
                errors.push(format!("registry declares '{target}' twice"));
                existing.1 = record;
            }
            None => records.push((target, record)),
        }
    }
    records
}

/// One contract's line in `--report`'s table: its path, then what was declared and what was found.
type Finding = (String, String, String);

/// `evaluate`: the errors, the findings, and the declared records.
fn evaluate(base: &Path) -> (Vec<String>, Vec<Finding>, Vec<(String, Py)>) {
    let mut errors = Vec::new();
    let declared = read_registry(base, &mut errors);
    let present = contracts(base);
    let mut findings = Vec::new();
    for relative in &present {
        let raw = fs::read(base.join(relative)).unwrap_or_default();
        let actual = derive_encoder(&raw).map(|derived| derived.record());
        let Some((_, record)) = declared.iter().find(|(path, _)| path == relative) else {
            errors.push(format!(
                "contract '{relative}' is not declared in {REGISTRY} — a new contract must declare the encoder that wrote it ({})",
                label(actual.as_ref())
            ));
            continue;
        };
        if pyjson::get(record, "hand_authored").is_some_and(pyjson::truthy) {
            if actual.is_some() {
                errors.push(format!(
                    "contract '{relative}' is declared hand-authored but is now reproduced by {} — it has been re-encoded, and its inline layout is gone",
                    label(actual.as_ref())
                ));
            }
            findings.push((
                relative.clone(),
                "hand-authored".to_string(),
                "hand-authored".to_string(),
            ));
            continue;
        }
        let expected = pyjson::get(record, "encoder");
        match &actual {
            None => errors.push(format!(
                "contract '{relative}' no longer round-trips under any encoder; declared {}",
                label(expected)
            )),
            Some(actual) if !expected.is_some_and(|expected| pyjson::py_eq(actual, expected)) => {
                errors.push(format!(
                    "contract '{relative}' was re-encoded: declared {}, found {}. Preserve the declared encoder, or update this record in the same commit so the migration is visible in the diff",
                    label(expected),
                    label(Some(actual))
                ))
            }
            Some(_) => {}
        }
        findings.push((relative.clone(), label(expected), label(actual.as_ref())));
    }
    let mut vanished: Vec<&String> = declared
        .iter()
        .map(|(path, _)| path)
        .filter(|path| !present.contains(path))
        .collect();
    vanished.sort();
    for relative in vanished {
        errors.push(format!(
            "registry declares '{relative}', which does not exist"
        ));
    }
    (errors, findings, declared)
}

/// `write_registry`: declare every contract's derived encoder, sorted keys and ASCII as the script wrote them.
fn write_registry(base: &Path) -> std::io::Result<usize> {
    let encoder = Encoder {
        indent: None,
        sort_keys: true,
        ensure_ascii: true,
        item_separator: ", ",
        key_separator: ": ",
    };
    let mut lines = Vec::new();
    for relative in contracts(base) {
        let derived = derive_encoder(&fs::read(base.join(&relative))?);
        let mut record = vec![
            (pyjson::py_str("path"), Py::Str(pyjson::py_str(&relative))),
            (
                pyjson::py_str("record_type"),
                Py::Str(pyjson::py_str("contract")),
            ),
            (pyjson::py_str("schema_version"), Py::Int("1".to_string())),
        ];
        match derived {
            None => record.push((pyjson::py_str("hand_authored"), Py::Bool(true))),
            Some(derived) => record.push((pyjson::py_str("encoder"), derived.record())),
        }
        lines.push(pyjson::dumps(&Py::Dict(record), &encoder).expect("ASCII"));
    }
    let target = base.join(REGISTRY);
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&target, format!("{}\n", lines.join("\n")))?;
    Ok(lines.len())
}

/// `churn_ratio`: the last commit to touch a contract, its changed lines, and its changed leaf fields.
fn churn_ratio(base: &Path, relative: &str) -> Option<(String, Option<u64>, usize)> {
    let git = |arguments: &[&str]| {
        Command::new("git")
            .arg("-C")
            .arg(base)
            .args(arguments)
            .output()
            .ok()
    };
    let revision = git(&["log", "-1", "--format=%h", "--", relative])?;
    let revision = String::from_utf8_lossy(&revision.stdout)
        .split_whitespace()
        .next()?
        .to_string();
    let load = |reference: String| -> Option<Py> {
        let output = git(&["show", &format!("{reference}:{relative}")])?;
        if !output.status.success() || output.stdout.is_empty() {
            return None;
        }
        pyjson::loads_bytes(&output.stdout).ok()
    };
    let before = load(format!("{revision}^"))?;
    let after = load(revision.clone())?;
    let (before, after) = (flatten(&before), flatten(&after));
    let mut keys: Vec<&String> = before.iter().map(|(key, _)| key).collect();
    for (key, _) in &after {
        if !keys.contains(&key) {
            keys.push(key);
        }
    }
    let lookup = |pairs: &[(String, Py)], key: &String| {
        pairs
            .iter()
            .find(|(name, _)| name == key)
            .map_or(Py::None, |(_, value)| value.clone())
    };
    let fields = keys
        .iter()
        .filter(|key| !pyjson::py_eq(&lookup(&before, key), &lookup(&after, key)))
        .count();
    let stat = git(&[
        "diff",
        "--numstat",
        &format!("{revision}^"),
        &revision,
        "--",
        relative,
    ])?;
    let stat = String::from_utf8_lossy(&stat.stdout).to_string();
    let words: Vec<&str> = stat.split_whitespace().collect();
    let churn = match words.as_slice() {
        [added, removed, ..] if added.chars().all(|c| c.is_ascii_digit()) && !added.is_empty() => {
            removed
                .parse::<u64>()
                .ok()
                .map(|removed| added.parse::<u64>().unwrap_or(0) + removed)
        }
        _ => None,
    };
    Some((revision, churn, fields))
}

/// The leaves of a value under `.key` and `[index]` paths, a repeated path keeping its first position and last value
/// as Python's `dict()` does.
fn flatten(value: &Py) -> Vec<(String, Py)> {
    let mut leaves: Vec<(String, Py)> = Vec::new();
    let mut push = |path: String, leaf: Py| match leaves.iter_mut().find(|(name, _)| *name == path)
    {
        Some(existing) => existing.1 = leaf,
        None => leaves.push((path, leaf)),
    };
    fn walk(value: &Py, prefix: String, push: &mut dyn FnMut(String, Py)) {
        match value {
            Py::Dict(pairs) => {
                for (key, item) in pairs {
                    walk(item, format!("{prefix}.{}", pyjson::to_text(key)), push);
                }
            }
            Py::List(items) => {
                for (index, item) in items.iter().enumerate() {
                    walk(item, format!("{prefix}[{index}]"), push);
                }
            }
            leaf => push(prefix, leaf.clone()),
        }
    }
    walk(value, String::new(), &mut push);
    leaves
}

/// Run the check from `root` with the flags the shim passed (`--check`, `--report`, `--write`, `--self-test`).
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    let known = ["--check", "--report", "--write", "--self-test"];
    if let Some(unknown) = arguments
        .iter()
        .find(|argument| !known.contains(&argument.as_str()))
    {
        eprintln!(
            "Usage: cargo xtask check contract-encoding [--check] [--report] [--write] [--self-test] (not {unknown})"
        );
        return 2;
    }
    let flag = |name: &str| arguments.iter().any(|argument| argument == name);
    if flag("--self-test") {
        return self_test(root);
    }
    if flag("--write") {
        match write_registry(root) {
            Ok(count) => println!("contract-encoding: declared {count} contract(s) in {REGISTRY}."),
            Err(error) => {
                eprintln!("contract-encoding: cannot write {REGISTRY}: {error}");
                return 1;
            }
        }
    }
    let (errors, findings, declared) = evaluate(root);
    if flag("--report") {
        for (relative, _, _) in &findings {
            if let Some((revision, Some(churn), fields)) = churn_ratio(root, relative)
                && churn > 0
            {
                let ratio = if fields > 0 {
                    format!("{:.1}x", churn as f64 / fields as f64)
                } else {
                    "inf (0 fields)".to_string()
                };
                println!(
                    "  {relative:58} last={revision} churn={churn:5} fields={fields:3} {ratio}"
                );
            }
        }
    }
    if !errors.is_empty() {
        for error in &errors {
            eprintln!("contract-encoding: {error}");
        }
        eprintln!(
            "contract-encoding: FAILED with {} violation(s).",
            errors.len()
        );
        return 1;
    }
    let hand = declared
        .iter()
        .filter(|(_, record)| pyjson::get(record, "hand_authored").is_some_and(pyjson::truthy))
        .count();
    println!(
        "contract-encoding: {} doctrine JSON contract(s) still round-trip under their declared encoder ({hand} hand-authored).",
        findings.len()
    );
    0
}

/// The script's self-test: on a scratch tree under `generated/`, a freshly written registry is green, each refusal
/// goes RED (a re-encoded contract, an undeclared one, a vanished one, a hand-authored one re-encoded), the restored
/// tree is green again, and the derivation tells the separators and the trailing newline apart.
fn self_test(root: &Path) -> i32 {
    let workspace = root.join(format!(
        "generated/.contract-encoding-self-test.{}",
        process::id()
    ));
    let result = self_test_cases(&workspace);
    let _ = fs::remove_dir_all(&workspace);
    match result {
        Ok(passed) => {
            println!(
                "contract-encoding: self-test {passed}/{passed} derivation and refusal cases pass."
            );
            0
        }
        Err(failure) => {
            eprintln!("contract-encoding: self-test FAILED: {failure}");
            1
        }
    }
}

fn self_test_cases(base: &Path) -> Result<usize, String> {
    let io = |error: std::io::Error| error.to_string();
    let sub = base.join("doctrine/sub");
    fs::create_dir_all(&sub).map_err(io)?;
    let pair = pyjson::loads("{\"b\": 1, \"a\": 2}").expect("fixture");
    let encode = |indent: usize| {
        let encoder = Encoder {
            indent: Some(indent),
            sort_keys: true,
            ensure_ascii: true,
            item_separator: ",",
            key_separator: ": ",
        };
        format!("{}\n", pyjson::dumps(&pair, &encoder).expect("ASCII"))
    };
    let good = sub.join("good.json");
    let hand = base.join("doctrine/hand.json");
    let hand_layout = "{\n  \"inline\": {\"a\": 1, \"b\": 2}\n}\n";
    fs::write(&good, encode(2)).map_err(io)?;
    fs::write(&hand, hand_layout).map_err(io)?;
    write_registry(base).map_err(io)?;
    let errors = || evaluate(base).0;
    let expect = |case: &str, errors: Vec<String>, wanted: &str| -> Result<(), String> {
        if errors.iter().any(|error| error.contains(wanted)) {
            Ok(())
        } else {
            Err(format!("{case}: no '{wanted}' in {errors:?}"))
        }
    };
    let mut passed = 0;
    if !errors().is_empty() {
        return Err(format!(
            "a freshly written registry must be green: {:?}",
            errors()
        ));
    }
    passed += 1;
    fs::write(&good, encode(4)).map_err(io)?;
    expect("re-encoded", errors(), "was re-encoded")?;
    passed += 1;
    fs::write(&good, encode(2)).map_err(io)?;
    let fresh = base.join("doctrine/fresh.json");
    fs::write(&fresh, "{}\n").map_err(io)?;
    expect("undeclared", errors(), "is not declared")?;
    passed += 1;
    fs::remove_file(&fresh).map_err(io)?;
    fs::remove_file(&good).map_err(io)?;
    expect("vanished", errors(), "does not exist")?;
    passed += 1;
    fs::write(&good, encode(2)).map_err(io)?;
    let machine = Encoder {
        indent: Some(2),
        sort_keys: false,
        ensure_ascii: true,
        item_separator: ",",
        key_separator: ": ",
    };
    let inline = pyjson::loads(hand_layout).expect("fixture");
    fs::write(
        &hand,
        format!("{}\n", pyjson::dumps(&inline, &machine).expect("ASCII")),
    )
    .map_err(io)?;
    expect("hand-authored", errors(), "declared hand-authored")?;
    passed += 1;
    fs::write(&hand, hand_layout).map_err(io)?;
    if !errors().is_empty() {
        return Err(format!("restored tree must be green again: {:?}", errors()));
    }
    passed += 1;
    let spaced = Encoder {
        indent: Some(3),
        sort_keys: true,
        ensure_ascii: true,
        item_separator: ",",
        key_separator: " : ",
    };
    let pretty =
        pyjson::dumps(&pyjson::loads("{\"a\": 1}").expect("fixture"), &spaced).expect("ASCII");
    let derived = derive_encoder(format!("{pretty}\n").as_bytes());
    if derived.map(|derived| derived.encoder.key_separator) != Some(" : ")
        || derive_encoder(pretty.as_bytes()).map(|derived| derived.trailing_newline) != Some(false)
        || derive_encoder(b"{\"a\": 1,}\n").is_some()
    {
        return Err(
            "the derivation does not tell separators and the trailing newline apart".to_string(),
        );
    }
    passed += 1;
    Ok(passed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_self_test_cases_keep_their_verdicts() {
        let workspace =
            crate::repository_root().join(".project-data/tmp/xtask-contract-encoding-self-test");
        let _ = fs::remove_dir_all(&workspace);
        let passed = self_test_cases(&workspace);
        fs::remove_dir_all(&workspace).expect("cleanup");
        assert_eq!(passed, Ok(7));
    }

    #[test]
    fn the_derivation_follows_the_scripts_order() {
        let first = derive_encoder(b"{\"a\": 1}\n").expect("compact");
        assert_eq!(first.encoder.indent, None);
        assert!(first.encoder.sort_keys && first.encoder.ensure_ascii && first.trailing_newline);
        let unsorted = derive_encoder(b"{\n \"b\": 1,\n \"a\": \"\\u00e9\"\n}").expect("indent 1");
        assert_eq!(unsorted.encoder.indent, Some(1));
        assert!(
            !unsorted.encoder.sort_keys
                && unsorted.encoder.ensure_ascii
                && !unsorted.trailing_newline
        );
        assert!(derive_encoder(b"{\"a\": 1}\n\n").is_none());
    }

    #[test]
    fn a_label_names_the_encoder_as_the_script_does() {
        let derived = derive_encoder(b"{\n  \"a\" : 1\n}\n").expect("spaced");
        assert_eq!(
            label(Some(&derived.record())),
            "indent=2 sort_keys=True ensure_ascii=True separators=(',', ' : ') trailing_newline=True"
        );
        assert_eq!(label(None), "hand-authored (no encoder reproduces it)");
    }

    #[test]
    fn the_churn_ratio_rounds_as_python_formats_it() {
        for (value, python) in [
            (0.25, "0.2"),
            (0.35, "0.3"),
            (2.5, "2.5"),
            (1.0 / 3.0, "0.3"),
            (3.5, "3.5"),
            (0.05, "0.1"),
        ] {
            assert_eq!(format!("{value:.1}"), python, "{value}");
        }
    }

    #[test]
    fn flatten_keeps_the_first_position_and_the_last_value_of_a_repeated_path() {
        let value =
            pyjson::loads("{\"a\": {\"b\": 1}, \"a.b\": 2, \"c\": [true, {}]}").expect("value");
        let leaves: Vec<(String, String)> = flatten(&value)
            .into_iter()
            .map(|(path, leaf)| (path, pyjson::repr(&leaf)))
            .collect();
        assert_eq!(
            leaves,
            [
                (".a.b".to_string(), "2".to_string()),
                (".c[0]".to_string(), "True".to_string())
            ]
        );
    }
}
