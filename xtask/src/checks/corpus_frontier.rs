//! The `CORPUS-FRONTIER` doctrine (`CORPUS-COVERAGE.4.1`), native since `RUSTDOC-AND-XTASK.14.a`: "how much of the
//! corpus is still unrefreshed" is a derived quantity a declaration is checked against, never a number carried from
//! slice to slice. The cohort is derived from `generated/source_ir/<key>/source_ir.json` (each document's
//! `source.requested_path`, outside the declared excluded prefixes) and checked against the exact lifecycle
//! partition in `doctrine/corpus_frontier/census.json`: identity, membership against the retained bundles, omission,
//! and the root task file's prose. It skips loudly when the corpus root is absent.
//!
//! Two registered scripts are shims over it: `scripts/check_corpus_frontier.sh`, the doctrine's adapter, which
//! passes `--root DIR`, runs the self-test first when the root is the repository and checks another tree with that
//! tree's own census script, as the adapter always did; and
//! `scripts/check_corpus_frontier_census.pl`, the core the records cite, which keeps its argument loop and passes
//! `--census` with its arguments.

use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

use crate::perl;
use crate::pyjson::{self, Encoder, Py};

const LABEL: &str = "corpus-frontier-census";
const DEFAULT_CONTRACT: &str = "doctrine/corpus_frontier/census.json";
const PREFIX_BYTES: u64 = 8192;
const MAX_PREFIX_BYTES: u64 = 4 * 1024 * 1024;
const EXPECTED_SELF_TEST_CASES: usize = 13;

/// Run the adapter (`--root DIR`) or, with `--census`, the core's own command line.
pub fn run(repository: &Path, arguments: &[String]) -> i32 {
    if arguments.first().is_some_and(|first| first == "--census") {
        return census_command(repository, &arguments[1..]);
    }
    let root = match arguments {
        [] => repository.to_path_buf(),
        [flag, root] if flag == "--root" => PathBuf::from(root),
        _ => {
            eprintln!(
                "Usage: cargo xtask check corpus-frontier [--root PROJECT_ROOT] | --census [ARGUMENTS...]"
            );
            return 2;
        }
    };
    let same = fs::canonicalize(&root).ok() == fs::canonicalize(repository).ok();
    if !same {
        // The adapter checks another tree with that tree's own core, as the script did: its census script, run
        // by `perl` against its own contract, whatever version that tree carries.
        let core = root.join("scripts/check_corpus_frontier_census.pl");
        let status = std::process::Command::new("perl")
            .arg(&core)
            .arg("--root")
            .arg(&root)
            .args(["--contract", DEFAULT_CONTRACT, "--check"])
            .status();
        return i32::from(!status.is_ok_and(|status| status.success()));
    }
    let mut failed = !self_test(repository);
    if check(&root, DEFAULT_CONTRACT, false) != 0 {
        failed = true;
    }
    i32::from(failed)
}

/// The core's command line, as the Perl shim forwards it: `--caller-dir DIR` first (the directory it was run from,
/// against which a relative `--root` and contract resolve, as the script resolved them), then `--root DIR`,
/// `--contract FILE` and the modes.
fn census_command(repository: &Path, arguments: &[String]) -> i32 {
    let mut arguments = arguments;
    if let [flag, directory, rest @ ..] = arguments
        && flag == "--caller-dir"
    {
        if let Err(error) = std::env::set_current_dir(directory) {
            eprintln!("{LABEL}: cannot return to the calling directory {directory}: {error}");
            return 2;
        }
        arguments = rest;
    }
    let mut root = repository.to_path_buf();
    let mut contract = std::env::var("SPECFORGE_CORPUS_FRONTIER_CONTRACT")
        .unwrap_or_else(|_| DEFAULT_CONTRACT.to_string());
    let (mut self_test_mode, mut report) = (false, false);
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => match arguments.next() {
                Some(value) => root = PathBuf::from(value),
                None => return census_usage(),
            },
            "--contract" => match arguments.next() {
                Some(value) => contract = value.clone(),
                None => return census_usage(),
            },
            "--self-test" => self_test_mode = true,
            "--report" => report = true,
            "--check" => {}
            _ => return census_usage(),
        }
    }
    if self_test_mode {
        return i32::from(!self_test(repository));
    }
    check(&root, &contract, report)
}

fn census_usage() -> i32 {
    eprintln!(
        "usage: check_corpus_frontier_census.pl [--root DIR] [--contract FILE] [--check|--report|--self-test]"
    );
    2
}

/// One run of the core against `root`, printing as the script printed.
fn check(root: &Path, contract: &str, report: bool) -> i32 {
    let mut errors = Vec::new();
    let result = run_census(root, contract, &mut errors);
    if !errors.is_empty() {
        for error in &errors {
            eprintln!("{LABEL}: {error}");
        }
        eprintln!("{LABEL}: FAILED with {} violation(s).", errors.len());
        return 1;
    }
    let Some(result) = result else {
        println!("{LABEL}: SKIPPED - no corpus root; the derivation does not govern this tree.");
        return 0;
    };
    if report {
        // `JSON::PP->new->canonical(1)->pretty(1)`: keys sorted, three-space indent, ` : ` between key and value.
        let encoder = Encoder {
            indent: Some(3),
            sort_keys: true,
            ensure_ascii: false,
            item_separator: ",",
            key_separator: " : ",
        };
        let keys = |keys: &[String]| {
            Py::List(
                keys.iter()
                    .map(|key| Py::Str(pyjson::py_str(key)))
                    .collect(),
            )
        };
        let value = Py::Dict(vec![
            (pyjson::py_str("cohort"), Py::Int(result.cohort.to_string())),
            (
                pyjson::py_str("refreshed"),
                Py::Int(result.refreshed.to_string()),
            ),
            (
                pyjson::py_str("remaining"),
                Py::Int(result.remaining.to_string()),
            ),
            (
                pyjson::py_str("refreshed_keys"),
                keys(&result.refreshed_keys),
            ),
            (
                pyjson::py_str("remaining_keys"),
                keys(&result.remaining_keys),
            ),
        ]);
        println!("{}", pyjson::dumps(&value, &encoder).expect("text"));
        return 0;
    }
    println!(
        "{LABEL}: {} cohort = {} refreshed + {} remaining; declaration, retention, and the root frontier agree.",
        result.cohort, result.refreshed, result.remaining
    );
    0
}

/// The derived census.
#[derive(Debug, PartialEq, Eq)]
pub struct Census {
    /// Cohort documents.
    pub cohort: usize,
    /// Distinct declared-refreshed keys.
    pub refreshed: usize,
    /// Distinct declared-remaining keys.
    pub remaining: usize,
    /// The refreshed keys, sorted.
    pub refreshed_keys: Vec<String>,
    /// The remaining keys, sorted.
    pub remaining_keys: Vec<String>,
}

fn get<'a>(value: &'a Py, key: &str) -> Option<&'a Py> {
    pyjson::get(value, key).filter(|value| !matches!(value, Py::None))
}

/// `$value` as Perl prints it, or the empty string for `undef`.
fn scalar(value: Option<&Py>) -> String {
    perl::scalar(value).unwrap_or_default()
}

/// A defined scalar (not a reference).
fn is_scalar(value: Option<&Py>) -> bool {
    value.is_some_and(|value| !perl::is_reference(value))
}

fn read_json_file(path: &str, label: &str, errors: &mut Vec<String>) -> Option<Py> {
    let Ok(raw) = fs::read(path) else {
        errors.push(format!("{label} is missing or unreadable: {path}"));
        return None;
    };
    match pyjson::loads_strict(&String::from_utf8_lossy(&raw)) {
        Ok(value) => Some(value),
        Err(_) => {
            errors.push(format!("{label} is not valid JSON: {path}"));
            None
        }
    }
}

fn validate_contract(contract: &Py, errors: &mut Vec<String>) -> bool {
    for field in [
        "schema_version",
        "contract_id",
        "owner_leaf",
        "authority",
        "declared_on",
    ] {
        if !is_scalar(get(contract, field)) {
            errors.push(format!("contract field '{field}' is missing"));
        }
    }
    let Some(rule @ Py::Dict(_)) = get(contract, "cohort_rule") else {
        errors.push("contract cohort_rule must be an object".to_string());
        return false;
    };
    for field in [
        "corpus_root",
        "artifact_name",
        "source_field",
        "retention_contract",
    ] {
        if !is_scalar(get(rule, field)) {
            errors.push(format!("contract cohort_rule.{field} is missing"));
        }
    }
    if !matches!(get(rule, "excluded_source_prefixes"), Some(Py::List(items)) if !items.is_empty())
    {
        errors.push(
            "contract cohort_rule.excluded_source_prefixes must be a non-empty array".to_string(),
        );
    }
    let Some(expected @ Py::Dict(_)) = get(contract, "expected") else {
        errors.push("contract expected must be an object".to_string());
        return false;
    };
    for field in ["cohort", "refreshed"] {
        let value = get(expected, field);
        let digits = value.is_some() && {
            let text = scalar(value);
            !text.is_empty() && text.bytes().all(|byte| byte.is_ascii_digit())
        };
        if !digits {
            errors.push(format!(
                "contract expected.{field} must be a non-negative integer"
            ));
        }
    }
    for field in ["refreshed", "remaining"] {
        if !matches!(get(contract, field), Some(Py::List(_))) {
            errors.push(format!("contract {field} must be an array"));
        }
    }
    let Some(claim @ Py::Dict(_)) = get(contract, "root_claim") else {
        errors.push("contract root_claim must be an object".to_string());
        return false;
    };
    for field in ["path", "completed_sentence", "remaining_phrase"] {
        if !is_scalar(get(claim, field)) {
            errors.push(format!("contract root_claim.{field} is missing"));
        }
    }
    errors.is_empty()
}

/// The text of a `"source"` object read from a bounded prefix of the artifact, grown until the object is complete.
fn extract_source_object(path: &str, errors: &mut Vec<String>) -> Option<Vec<u8>> {
    let size = fs::metadata(path).ok()?.len();
    let mut want = PREFIX_BYTES;
    loop {
        want = want.min(size);
        let mut buffer = Vec::new();
        match fs::File::open(path) {
            Ok(file) => {
                let _ = file.take(want).read_to_end(&mut buffer);
            }
            Err(error) => {
                errors.push(format!("cannot read '{path}': {}", perl_error(&error)));
                return None;
            }
        }
        if let Some(object) = slice_json_object(&buffer, b"\"source\"") {
            return Some(object);
        }
        if want >= size || want >= MAX_PREFIX_BYTES {
            break;
        }
        want *= 2;
    }
    errors.push(format!(
        "'{path}' has no complete top-level \"source\" object"
    ));
    None
}

fn perl_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

/// The object after `key`: from its `{` to the matching `}`, braces inside strings ignored.
fn slice_json_object(buffer: &[u8], key: &[u8]) -> Option<Vec<u8>> {
    let at = buffer.windows(key.len()).position(|window| window == key)?;
    let open = at
        + key.len()
        + buffer[at + key.len()..]
            .iter()
            .position(|byte| *byte == b'{')?;
    let (mut depth, mut in_string, mut escaped) = (0usize, false, false);
    for (index, byte) in buffer.iter().enumerate().skip(open) {
        if in_string {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' => depth += 1,
            b'}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(buffer[open..=index].to_vec());
                }
            }
            _ => {}
        }
    }
    None
}

fn strings(value: Option<&Py>) -> Vec<String> {
    match value {
        Some(Py::List(items)) => items.iter().map(|item| scalar(Some(item))).collect(),
        _ => Vec::new(),
    }
}

/// `run_census`: the derived census, `None` when the corpus root is absent or an error stopped the derivation.
fn run_census(base: &Path, contract_rel: &str, errors: &mut Vec<String>) -> Option<Census> {
    let base_text = base.to_string_lossy().into_owned();
    let contract = read_json_file(
        &format!("{base_text}/{contract_rel}"),
        "corpus frontier contract",
        errors,
    )?;
    if !validate_contract(&contract, errors) {
        return None;
    }
    let rule = get(&contract, "cohort_rule").expect("validated");
    let corpus_root = std::env::var("SPECFORGE_CORPUS_FRONTIER_ROOT")
        .unwrap_or_else(|_| format!("{base_text}/{}", scalar(get(rule, "corpus_root"))));
    if !Path::new(&corpus_root).is_dir() {
        return None;
    }
    let retained = read_json_file(
        &format!("{base_text}/{}", scalar(get(rule, "retention_contract"))),
        "retention contract",
        errors,
    )?;
    if !matches!(get(&retained, "retained"), Some(Py::List(_))) {
        errors.push("retention contract has no retained array".to_string());
        return None;
    }
    let retained: BTreeSet<String> = strings(get(&retained, "retained")).into_iter().collect();
    let entries = match fs::read_dir(&corpus_root) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!(
                "cannot list corpus root '{corpus_root}': {}",
                perl_error(&error)
            ));
            return None;
        }
    };
    let mut keys: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| Path::new(&format!("{corpus_root}/{name}")).is_dir())
        .collect();
    keys.sort();
    let artifact_name = scalar(get(rule, "artifact_name"));
    let source_field = scalar(get(rule, "source_field"));
    let prefixes = strings(get(rule, "excluded_source_prefixes"));
    let mut cohort = Vec::new();
    for key in keys {
        let artifact = format!("{corpus_root}/{key}/{artifact_name}");
        if !Path::new(&artifact).is_file() {
            continue;
        }
        let Some(raw) = extract_source_object(&artifact, errors) else {
            continue;
        };
        let source = match pyjson::loads_strict(&String::from_utf8_lossy(&raw)) {
            Ok(source @ Py::Dict(_)) => source,
            _ => {
                errors.push(format!("'{artifact}' has an unreadable \"source\" object"));
                continue;
            }
        };
        let requested = get(&source, &source_field);
        if !is_scalar(requested) {
            errors.push(format!("'{artifact}' has no scalar source.{source_field}"));
            continue;
        }
        let requested = scalar(requested);
        if prefixes
            .iter()
            .any(|prefix| requested.starts_with(prefix.as_str()))
        {
            continue;
        }
        cohort.push(key);
    }
    if !errors.is_empty() {
        return None;
    }
    let cohort_set: BTreeSet<&String> = cohort.iter().collect();
    let refreshed_list = strings(get(&contract, "refreshed"));
    let remaining_list = strings(get(&contract, "remaining"));
    let refreshed: BTreeSet<String> = refreshed_list.iter().cloned().collect();
    let remaining: BTreeSet<String> = remaining_list.iter().cloned().collect();
    if refreshed.len() != refreshed_list.len() {
        errors.push("contract refreshed contains a duplicate key".to_string());
    }
    if remaining.len() != remaining_list.len() {
        errors.push("contract remaining contains a duplicate key".to_string());
    }
    let expected = get(&contract, "expected").expect("validated");
    let (cohort_n, refreshed_n, remaining_n) = (cohort.len(), refreshed.len(), remaining.len());
    if cohort_n as f64 != perl::numeric(get(expected, "cohort")) {
        errors.push(format!(
            "derived cohort {cohort_n} disagrees with declared expected.cohort {}",
            scalar(get(expected, "cohort"))
        ));
    }
    if refreshed_n as f64 != perl::numeric(get(expected, "refreshed")) {
        errors.push(format!(
            "derived refreshed {refreshed_n} disagrees with declared expected.refreshed {}",
            scalar(get(expected, "refreshed"))
        ));
    }
    if refreshed_n + remaining_n != cohort_n {
        errors.push(format!(
            "declared lifecycle partition has {refreshed_n} refreshed + {remaining_n} remaining, not cohort {cohort_n}"
        ));
    }
    for key in &refreshed {
        if !cohort_set.contains(key) {
            errors.push(format!(
                "declared-refreshed '{key}' is not a cohort document"
            ));
        }
        if remaining.contains(key) {
            errors.push(format!(
                "lifecycle key '{key}' is both refreshed and remaining"
            ));
        }
    }
    for key in &remaining {
        if !cohort_set.contains(key) {
            errors.push(format!(
                "declared-remaining '{key}' is not a cohort document"
            ));
            continue;
        }
        if retained.contains(key) {
            errors.push(format!(
                "declared-remaining '{key}' retains a normalized bundle, so it is refreshed"
            ));
        }
    }
    for key in &retained {
        if cohort_set.contains(key) && !refreshed.contains(key) {
            errors.push(format!(
                "retained cohort document '{key}' is not declared refreshed"
            ));
        }
    }
    for key in &cohort {
        if !refreshed.contains(key) && !remaining.contains(key) {
            errors.push(format!(
                "cohort document '{key}' is absent from the declared lifecycle partition"
            ));
        }
    }
    let claim = get(&contract, "root_claim").expect("validated");
    let claim_path = scalar(get(claim, "path"));
    match fs::read(format!("{base_text}/{claim_path}")) {
        Ok(text) => {
            let text = String::from_utf8_lossy(&text);
            for field in ["completed_sentence", "remaining_phrase"] {
                let wanted = scalar(get(claim, field));
                if !text.contains(wanted.as_str()) {
                    errors.push(format!(
                        "root frontier '{claim_path}' does not state \"{wanted}\""
                    ));
                }
            }
        }
        Err(error) => errors.push(format!(
            "root frontier '{claim_path}' is unreadable: {}",
            perl_error(&error)
        )),
    }
    Some(Census {
        cohort: cohort_n,
        refreshed: refreshed_n,
        remaining: remaining_n,
        refreshed_keys: refreshed.into_iter().collect(),
        remaining_keys: remaining.into_iter().collect(),
    })
}

/// The overrides one self-test case applies to the clean fixture.
#[derive(Default)]
struct Overrides {
    docs: Vec<(&'static str, &'static str)>,
    retained: Option<Vec<&'static str>>,
    expected: Option<(usize, usize)>,
    refreshed: Option<Vec<&'static str>>,
    remaining: Option<Vec<&'static str>>,
    root_text: Option<&'static str>,
}

fn pretty(value: &Py) -> String {
    let encoder = Encoder {
        indent: Some(3),
        sort_keys: true,
        ensure_ascii: false,
        item_separator: ",",
        key_separator: " : ",
    };
    format!("{}\n", pyjson::dumps(value, &encoder).expect("text"))
}

fn write(path: &Path, content: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, content)
}

fn str_list(items: &[&str]) -> String {
    format!(
        "[{}]",
        items
            .iter()
            .map(|item| format!("\"{item}\""))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

/// The fixture corpus: two in-repo documents outside the cohort, two refreshed (one with a retained bundle), two
/// remaining, every external path under one synthetic root.
fn build(tree: &Path, overrides: &Overrides) -> std::io::Result<()> {
    let _ = fs::remove_dir_all(tree);
    let mut docs: Vec<(&str, &str)> = vec![
        ("gold_a", "corpus/vendor/gold_a.pdf"),
        ("gold_b", "corpus/vendor/gold_b.pdf"),
        ("moved", "/Volumes/SSD/library/moved.pdf"),
        ("kept_bundle", "/Volumes/SSD/library/kept_bundle.pdf"),
        ("left_one", "/Volumes/SSD/library/left_one.pdf"),
        ("left_two", "/Volumes/SSD/library/left_two.pdf"),
    ];
    for (key, path) in &overrides.docs {
        match docs.iter_mut().find(|(existing, _)| existing == key) {
            Some(entry) => entry.1 = path,
            None => docs.push((key, path)),
        }
    }
    for (key, path) in &docs {
        let source = pyjson::loads(&format!(
            "{{\"schema_version\": 1, \"stage\": \"source_ir\", \"source\": {{\"requested_path\": \"{path}\", \"canonical_path\": \"{path}\"}}}}"
        ))
        .expect("fixture");
        write(
            &tree.join(format!("generated/source_ir/{key}/source_ir.json")),
            &pretty(&source),
        )?;
    }
    let retained = overrides
        .retained
        .clone()
        .unwrap_or_else(|| vec!["kept_bundle"]);
    write(
        &tree.join("doctrine/chain_currency/retained_bundles.json"),
        &pretty(
            &pyjson::loads(&format!("{{\"retained\": {}}}", str_list(&retained))).expect("fixture"),
        ),
    )?;
    let (cohort, refreshed_count) = overrides.expected.unwrap_or((4, 2));
    let contract = format!(
        "{{\"schema_version\": 1, \"contract_id\": \"corpus-frontier-census\", \"owner_leaf\": \"FIXTURE.1\", \"authority\": \"fixture\", \"declared_on\": \"2026-08-11\", \"cohort_rule\": {{\"corpus_root\": \"generated/source_ir\", \"artifact_name\": \"source_ir.json\", \"source_field\": \"requested_path\", \"excluded_source_prefixes\": [\"corpus/\"], \"retention_contract\": \"doctrine/chain_currency/retained_bundles.json\"}}, \"expected\": {{\"cohort\": {cohort}, \"refreshed\": {refreshed_count}}}, \"refreshed\": {}, \"remaining\": {}, \"root_claim\": {{\"path\": \"docs/tasks/FIXTURE.md\", \"completed_sentence\": \"2 of 4 real chip-spec refreshes are complete.\", \"remaining_phrase\": \"two real documents remaining\"}}}}",
        str_list(
            &overrides
                .refreshed
                .clone()
                .unwrap_or_else(|| vec!["kept_bundle", "moved"])
        ),
        str_list(
            &overrides
                .remaining
                .clone()
                .unwrap_or_else(|| vec!["left_one", "left_two"])
        ),
    );
    write(
        &tree.join("doctrine/corpus_frontier/census.json"),
        &pretty(&pyjson::loads(&contract).expect("fixture")),
    )?;
    write(
        &tree.join("docs/tasks/FIXTURE.md"),
        overrides.root_text.unwrap_or("# fixture\n\n2 of 4 real chip-spec refreshes are complete.\n\nWith two real documents remaining.\n"),
    )
}

/// The self-test's cases: their names, pass or failure counts, and the first failure's detail.
fn self_test_cases(dir: &Path) -> Result<(usize, Vec<(String, String)>), String> {
    let tree = dir.join("tree");
    let io = |error: std::io::Error| error.to_string();
    let mut passed = 0;
    let mut failures: Vec<(String, String)> = Vec::new();
    let cases: Vec<(&str, Option<&str>, Overrides)> = vec![
        ("clean fixture passes", None, Overrides::default()),
        (
            "a dropped remaining document fails closed",
            Some("absent from the declared lifecycle partition"),
            Overrides {
                remaining: Some(vec!["left_one"]),
                ..Overrides::default()
            },
        ),
        (
            "an inflated denominator fails closed",
            Some("derived cohort 4 disagrees"),
            Overrides {
                expected: Some((5, 2)),
                ..Overrides::default()
            },
        ),
        (
            "a refreshed count that does not fit the identity fails closed",
            Some("derived refreshed 2 disagrees"),
            Overrides {
                expected: Some((4, 1)),
                ..Overrides::default()
            },
        ),
        (
            "a declared-remaining document that kept its bundle fails closed",
            Some("retains a normalized bundle"),
            Overrides {
                retained: Some(vec!["kept_bundle", "left_one"]),
                ..Overrides::default()
            },
        ),
        (
            "a declared-remaining key outside the cohort fails closed",
            Some("is not a cohort document"),
            Overrides {
                remaining: Some(vec!["left_one", "gold_a"]),
                ..Overrides::default()
            },
        ),
        (
            "a declared-refreshed key outside the cohort fails closed",
            Some("declared-refreshed 'gold_a' is not a cohort document"),
            Overrides {
                refreshed: Some(vec!["kept_bundle", "gold_a"]),
                ..Overrides::default()
            },
        ),
        (
            "overlapping lifecycle sets fail closed",
            Some("is both refreshed and remaining"),
            Overrides {
                refreshed: Some(vec!["kept_bundle", "moved", "left_one"]),
                ..Overrides::default()
            },
        ),
        (
            "a root frontier stating another count fails closed",
            Some("does not state"),
            Overrides {
                root_text: Some(
                    "# fixture\n\n3 of 4 real chip-spec refreshes are complete.\n\nWith two real documents remaining.\n",
                ),
                ..Overrides::default()
            },
        ),
        (
            "a duplicate remaining key fails closed",
            Some("duplicate key"),
            Overrides {
                remaining: Some(vec!["left_one", "left_one", "left_two"]),
                ..Overrides::default()
            },
        ),
        (
            "a duplicate refreshed key fails closed",
            Some("duplicate key"),
            Overrides {
                refreshed: Some(vec!["kept_bundle", "moved", "moved"]),
                ..Overrides::default()
            },
        ),
    ];
    for (name, wanted, overrides) in &cases {
        build(&tree, overrides).map_err(io)?;
        let mut errors = Vec::new();
        let result = run_census(&tree, DEFAULT_CONTRACT, &mut errors);
        let ok = match wanted {
            None => {
                errors.is_empty()
                    && result
                        .as_ref()
                        .is_some_and(|census| census.cohort == 4 && census.refreshed == 2)
            }
            Some(wanted) => errors.iter().any(|error| error.contains(wanted)),
        };
        if ok {
            passed += 1;
        } else {
            let detail = if errors.is_empty() {
                "(none)".to_string()
            } else {
                errors.join("; ")
            };
            failures.push((name.to_string(), detail));
        }
    }
    // The absent-corpus skip, proven end to end rather than assumed.
    build(&tree, &Overrides::default()).map_err(io)?;
    fs::remove_dir_all(tree.join("generated/source_ir")).map_err(io)?;
    let mut skip_errors = Vec::new();
    if run_census(&tree, DEFAULT_CONTRACT, &mut skip_errors).is_none() && skip_errors.is_empty() {
        passed += 1;
    } else {
        failures.push((
            "absent corpus root must skip, not fail".to_string(),
            String::new(),
        ));
    }
    // The brace scanner must not be fooled by a brace inside a path string.
    let tricky = slice_json_object(
        br#"{"source": {"requested_path": "/Volumes/a{b}/x.pdf"}, "next": 1}"#,
        b"\"source\"",
    );
    if tricky.as_deref() == Some(br#"{"requested_path": "/Volumes/a{b}/x.pdf"}"#.as_slice()) {
        passed += 1;
    } else {
        failures.push((
            "brace scan mishandled a braced string".to_string(),
            String::new(),
        ));
    }
    Ok((passed, failures))
}

/// The script's self-test; true when every case holds.
fn self_test(repository: &Path) -> bool {
    let dir = repository.join(format!(
        ".project-data/tmp/corpus-frontier-self-test.{}",
        std::process::id()
    ));
    let outcome = self_test_cases(&dir);
    let _ = fs::remove_dir_all(&dir);
    let (passed, failures) = match outcome {
        Ok(outcome) => outcome,
        Err(message) => {
            eprintln!("{LABEL} self-test: {message}");
            return false;
        }
    };
    let total = passed + failures.len();
    if !failures.is_empty() {
        for (name, detail) in &failures {
            eprintln!("{LABEL} self-test FAILED: {name}");
            if !detail.is_empty() {
                eprintln!("  errors: {detail}");
            }
        }
        eprintln!(
            "{LABEL}: self-test {passed}/{total} passed, {} FAILED.",
            failures.len()
        );
        return false;
    }
    if total != EXPECTED_SELF_TEST_CASES {
        eprintln!(
            "{LABEL}: self-test ran {total} cases, declaration expects {EXPECTED_SELF_TEST_CASES} — re-derive the declaration beside the suite"
        );
        return false;
    }
    println!("{LABEL}: self-test {passed}/{total} passed.");
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_self_test_case_holds() {
        let dir =
            crate::repository_root().join(".project-data/tmp/xtask-corpus-frontier-self-test");
        let outcome = self_test_cases(&dir);
        let _ = fs::remove_dir_all(&dir);
        let (passed, failures) = outcome.expect("fixture");
        assert!(failures.is_empty(), "{failures:?}");
        assert_eq!(passed, EXPECTED_SELF_TEST_CASES);
    }

    #[test]
    fn the_source_object_is_sliced_by_its_braces() {
        assert_eq!(
            slice_json_object(
                br#"{"a": 1, "source": {"x": {"y": "}"}}, "z": 2}"#,
                b"\"source\""
            ),
            Some(br#"{"x": {"y": "}"}}"#.to_vec())
        );
        assert_eq!(
            slice_json_object(br#"{"source": {"open": 1"#, b"\"source\""),
            None
        );
        assert_eq!(slice_json_object(br#"{"other": {}}"#, b"\"source\""), None);
    }
}
