//! The `CONSTRAINT-PART-SPAN` doctrine (`EXTRACTION-QUALITY-GAUGE.3k.2j`), native since `RUSTDOC-AND-XTASK.14.b`.
//! `EXTRACTION-QUALITY-GAUGE.3k`'s census of which span of its statement each published signal constraint read its
//! kind from is stratified by the producer that minted each record, so it holds only while the kind classifier's
//! call-site topology in `crates/specforge/src/ir/evidence.rs` is the one declared here: which production functions
//! reach `classify_signal_constraint_kind_typed`, over which span, and that the untyped
//! `classify_signal_constraint_kind` has exactly one way in. `--check` fails closed when either moves.
//!
//! The census classifies a population by mirroring the extractor's grammar. It is not a check on the Rust
//! implementation: a mirror agreeing with its original carries no information (`CLAIM_VERIFICATION.md` section 2).
//!
//! Two registered scripts are shims over it: `scripts/check_constraint_part_span.sh`, the doctrine's adapter, which
//! runs the self-test and then the check; and `scripts/measure_constraint_part_span.py`, the census the records cite,
//! which keeps its `argparse` front end and passes `--measure` with its flags.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use crate::pyjson::{self, Encoder, Py, PyStr};

const LABEL: &str = "constraint-part-span";
const EVIDENCE_SOURCE: &str = "crates/specforge/src/ir/evidence.rs";
const CLASSIFIER: &str = "classify_signal_constraint_kind";
/// The gateway every producer goes through (`EXTRACTION-QUALITY-GAUGE.3k.2a` for the statement path, `.3k.2e` for
/// the table-row path); the bare classifier is its implementation.
const CLASSIFIER_ENTRY: &str = "classify_signal_constraint_kind_typed";

/// The call-site topology the stratification depends on: which enclosing function reaches the kind classifier,
/// and whether that call is already narrowed to one obligation clause. A literal, declared independently of what
/// the scan finds, so a new caller fails the check instead of being folded into an existing stratum. Re-derived by
/// `EXTRACTION-QUALITY-GAUGE.3k.2e`; `EXTRACTION-GAP-FIX.5b` added the third caller, whose two-cell obligation row
/// states exactly one obligation, so the sentence it reads is already a clause.
const EXPECTED_CLASSIFIER_CALLERS: &[(&str, &str)] = &[
    ("extract_signal_description_row_constraints", "clause"),
    ("extract_signal_keyed_obligation_row_constraints", "clause"),
    ("extract_signal_constraints", "whole"),
];
/// The one function that may reach the untyped classifier: the gateway itself.
const EXPECTED_UNTYPED_CALLERS: &[&str] = &[CLASSIFIER_ENTRY];

/// Constraint-id prefix, stratum label, and whether that producer reaches the kind classifier.
const PRODUCERS: &[(&str, &str, bool)] = &[
    (
        "sigcon",
        "pattern statement path (extract_signal_constraints)",
        true,
    ),
    (
        "row_sigcon",
        "table-row path (extract_signal_description_row_constraints)",
        true,
    ),
    (
        "keyed_row_sigcon",
        "signal-keyed obligation row path (extract_signal_keyed_obligation_row_constraints)",
        true,
    ),
    (
        "dyn_sigcon",
        "dynamic value-binding path (extract_dynamic_signal_constraints)",
        false,
    ),
    (
        "llm_sigcon",
        "LLM-primary path (constraint_extract_llm.rs)",
        false,
    ),
];

// Mirrors of the extractor's own grammar (`evidence.rs`).

/// `constraint_bearing_sentence` splits on exactly these, and returns the first part with a modal.
const CLAUSE_SPLIT: [char; 4] = ['.', ';', '•', '\n'];
const MODALS: [&str; 2] = ["must", "shall"];
/// `classify_signal_constraint_kind`, arm by arm and in its own order: the published kind, the arm's label (two
/// arms can both yield `must_be_stable`, and only one is a phrase the document wrote), and its phrases.
const CLASSIFIER_ARMS: &[(&str, &str, &[&str])] = &[
    (
        "must_not_change",
        "must_not_change",
        &[
            "must not change",
            "shall not change",
            "must not be changed",
            "shall not be changed",
            "must remain stable",
            "shall remain stable",
        ],
    ),
    (
        "must_be_stable",
        "stable_phrase",
        &[
            "must be stable",
            "shall be stable",
            "must hold",
            "shall hold",
        ],
    ),
    (
        "must_be_high",
        "must_be_high",
        &[
            "must be high",
            "shall be high",
            "must remain high",
            "shall remain high",
            "must be driven high",
        ],
    ),
    (
        "must_be_low",
        "must_be_low",
        &[
            "must be low",
            "shall be low",
            "must remain low",
            "shall remain low",
            "must be driven low",
        ],
    ),
    (
        "must_be_asserted",
        "must_be_asserted",
        &[
            "must be asserted",
            "shall be asserted",
            "must remain asserted",
            "shall remain asserted",
        ],
    ),
    (
        "must_be_deasserted",
        "must_be_deasserted",
        &[
            "must be deasserted",
            "shall be deasserted",
            "must remain deasserted",
            "shall remain deasserted",
            "must not be asserted",
            "shall not be asserted",
            "must not be active",
            "shall not be active",
        ],
    ),
];
const VALID_PHRASES: [&str; 2] = ["must be valid", "shall be valid"];
const VALUE_BINDERS: [&str; 4] = ["must be ", "shall be ", "must remain ", "shall remain "];
const VALUE_FILLERS: [&str; 12] = [
    "a", "an", "the", "set", "driven", "to", "equal", "held", "kept", "in", "at", "its",
];
const NEGATORS: [&str; 6] = [
    "must not",
    "shall not",
    "must never",
    "shall never",
    "cannot",
    "will not",
];
/// A comparative magnitude whose right operand is a reference rather than a literal has no typed slot, as `.3d`'s
/// inter-operand equality has none; these phrases introduce it.
const MAGNITUDE_PHRASES: [&str; 5] = [
    "greater than",
    "less than",
    "larger than",
    "smaller than",
    "more than",
];
/// What makes the right operand a reference: the magnitude is stated against something the document names.
const REFERENCE_OPERAND_PHRASES: [&str; 6] = [
    "than the size indicated by",
    "than the value of",
    "than the value indicated by",
    "than the number of",
    "than the size of",
    "than that indicated by",
];
/// The two kind arms that publish `must_be_stable` without the document writing a stability phrase.
const UNTYPED_DEFAULT_ARMS: [&str; 2] = ["untyped_default", "valid_no_value"];

/// Deleting a self-test case must fail the suite: the expected total is declared apart from the cases
/// (`PRODUCTION-GRAPH-CENSUS-PIN.3`).
const EXPECTED_SELF_TEST_CASES: usize = 10;

/// Run the adapter (no arguments: the self-test, then the check) or, after `--measure`, the census script's own
/// command line (`[--root DIR] [--json] [--check] [--self-test]`, as its shim forwards it).
pub fn run(repository: &Path, arguments: &[String]) -> i32 {
    match arguments.split_first() {
        None => {
            let mut failed = self_test() != 0;
            if check(repository) != 0 {
                failed = true;
            }
            i32::from(failed)
        }
        Some((first, rest)) if first == "--measure" => measure(repository, rest),
        _ => usage(),
    }
}

fn usage() -> i32 {
    eprintln!(
        "Usage: cargo xtask check constraint-part-span [--measure [--root DIR] [--json] [--check] [--self-test]]"
    );
    2
}

/// The census script's modes, in its precedence: the self-test, then the check, then the JSON census, then the
/// report.
fn measure(repository: &Path, arguments: &[String]) -> i32 {
    let mut root = repository.to_path_buf();
    let mut arguments = arguments;
    if let [flag, directory, rest @ ..] = arguments
        && flag == "--root"
    {
        root = PathBuf::from(directory);
        arguments = rest;
    }
    let (mut json, mut check_mode, mut self_test_mode) = (false, false, false);
    for argument in arguments {
        match argument.as_str() {
            "--json" => json = true,
            "--check" => check_mode = true,
            "--self-test" => self_test_mode = true,
            _ => return usage(),
        }
    }
    if self_test_mode {
        return self_test();
    }
    if check_mode {
        return check(&root);
    }
    let sites = match call_sites(&root, CLASSIFIER_ENTRY) {
        Ok(sites) => sites,
        Err(error) => return refuse(&error),
    };
    let result = match census(&root) {
        Ok(result) => result,
        Err(error) => return refuse(&error),
    };
    if json {
        let encoder = Encoder {
            indent: Some(2),
            sort_keys: true,
            ensure_ascii: true,
            item_separator: ",",
            key_separator: ": ",
        };
        let document = result.to_py(&sites);
        println!(
            "{}",
            pyjson::dumps(&document, &encoder).expect("ensure_ascii escapes every surrogate")
        );
        return 0;
    }
    print!("{}", report(&result, &sites));
    0
}

/// Where the script would stop with a traceback (exit 1), the port names what it could not read.
fn refuse(error: &str) -> i32 {
    eprintln!("{LABEL}: {error}");
    1
}

// Python's view of text.

/// `str.isspace()`: Rust's white space plus the four information separators Python also counts.
fn is_space(code: u32) -> bool {
    char::from_u32(code).is_some_and(|character| {
        character.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&character)
    })
}

/// `str.strip()`.
fn strip(text: &[u32]) -> &[u32] {
    let start = text
        .iter()
        .position(|code| !is_space(*code))
        .unwrap_or(text.len());
    let end = text
        .iter()
        .rposition(|code| !is_space(*code))
        .map_or(start, |last| last + 1);
    &text[start..end.max(start)]
}

/// `str.lower()`, for matching only: every phrase is ASCII, so a lone surrogate, which no phrase can match, reads
/// as U+FFFD, and the final-sigma rule, which changes one non-ASCII letter into another, is not applied.
fn lower(text: &[u32]) -> String {
    let mut out = String::with_capacity(text.len());
    for code in text {
        match char::from_u32(*code) {
            Some(character) => out.extend(character.to_lowercase()),
            None => out.push('\u{fffd}'),
        }
    }
    out
}

/// A file read as Python's text mode reads it: strict UTF-8, with `\r\n` and `\r` turned into `\n`.
fn read_text(path: &Path) -> Result<String, String> {
    let bytes =
        fs::read(path).map_err(|error| format!("cannot read '{}': {error}", path.display()))?;
    let text = String::from_utf8(bytes)
        .map_err(|error| format!("'{}' is not UTF-8: {error}", path.display()))?;
    Ok(text.replace("\r\n", "\n").replace('\r', "\n"))
}

// The mirrors.

fn constraint_bearing_sentence(text: &[u32]) -> &[u32] {
    let split: Vec<u32> = CLAUSE_SPLIT
        .iter()
        .map(|character| u32::from(*character))
        .collect();
    for sentence in text.split(|code| split.contains(code)) {
        let lowered = lower(sentence);
        if MODALS.iter().any(|modal| lowered.contains(modal)) {
            return sentence;
        }
    }
    text
}

fn extract_protocol_state_value(lowered: &str) -> Option<String> {
    for binder in VALUE_BINDERS {
        let Some(position) = lowered.find(binder) else {
            continue;
        };
        let rest = &lowered[position + binder.len()..];
        let word = rest
            .split(|character: char| {
                !(character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_')
            })
            .find(|word| !word.is_empty() && !VALUE_FILLERS.contains(word));
        if let Some(word) = word {
            return Some(word.to_ascii_uppercase());
        }
    }
    None
}

/// The published kind and the arm, exactly as `classify_signal_constraint_kind` would choose them.
fn classify_kind(lowered: &str) -> (String, &'static str) {
    for (kind, arm, phrases) in CLASSIFIER_ARMS {
        if phrases.iter().any(|phrase| lowered.contains(phrase)) {
            return ((*kind).to_string(), arm);
        }
    }
    if VALID_PHRASES.iter().any(|phrase| lowered.contains(phrase)) {
        return match extract_protocol_state_value(lowered) {
            Some(value) => (format!("must_be_value:{value}"), "valid_value"),
            None => ("must_be_stable".to_string(), "valid_no_value"),
        };
    }
    match extract_protocol_state_value(lowered) {
        Some(value) => (format!("must_be_value:{value}"), "generic_value"),
        None => ("must_be_stable".to_string(), "untyped_default"),
    }
}

fn obligation_is_negated(lowered: &str) -> bool {
    NEGATORS.iter().any(|negator| lowered.contains(negator))
}

fn is_reference_magnitude(lowered: &str) -> bool {
    MAGNITUDE_PHRASES
        .iter()
        .any(|phrase| lowered.contains(phrase))
        && REFERENCE_OPERAND_PHRASES
            .iter()
            .any(|phrase| lowered.contains(phrase))
}

// The call-site topology.

/// `str.strip()` and `str.lstrip()` of a source line.
fn strip_line(line: &str) -> &str {
    line.trim_matches(|character: char| is_space(u32::from(character)))
}

fn lstrip_line(line: &str) -> &str {
    line.trim_start_matches(|character: char| is_space(u32::from(character)))
}

/// The lines (0-based) of an item a `#[cfg(test)]` attribute guards: a `mod`, or a single helper `fn`, bounded by
/// the closing brace at the item's own indentation, which rustfmt guarantees. A test caller must not fail the
/// check; a production caller still does. The attribute guards only the item after it: scanning ahead to the next
/// `mod` would swallow every line between a `#[cfg(test)] fn` and the next test module (`.3k.2e`).
fn test_module_lines(lines: &[String]) -> BTreeSet<usize> {
    let mut inside = BTreeSet::new();
    let mut index = 0;
    while index < lines.len() {
        if strip_line(&lines[index]) == "#[cfg(test)]" {
            let mut item = index + 1;
            while item < lines.len() && strip_line(&lines[item]).is_empty() {
                item += 1;
            }
            let head = lines.get(item).map_or("", |line| lstrip_line(line));
            if ["mod ", "fn ", "pub fn ", "pub(super) fn ", "pub(crate) fn "]
                .iter()
                .any(|start| head.starts_with(start))
            {
                let indent = lines[item].chars().count() - head.chars().count();
                let closing = format!("{}}}", " ".repeat(indent));
                let mut end = item + 1;
                while end < lines.len() && lines[end].trim_end_matches('\n') != closing {
                    end += 1;
                }
                inside.extend(index..(end + 1).min(lines.len()));
                index = end + 1;
                continue;
            }
        }
        index += 1;
    }
    inside
}

/// The leading run of Python white space skipped, or `None` when there is none (`\s+`).
fn skip_spaces(text: &str) -> Option<&str> {
    let rest = lstrip_line(text);
    (rest.len() < text.len()).then_some(rest)
}

/// The name a line defines, as `^\s*(?:pub(?:\([^)]*\))?\s+)?fn\s+([A-Za-z0-9_]+)` matches it.
fn definition(line: &str) -> Option<&str> {
    let rest = lstrip_line(line);
    let after_pub = rest.strip_prefix("pub").and_then(|tail| {
        let restricted = tail.strip_prefix('(').and_then(|inner| {
            let close = inner.find(')')?;
            skip_spaces(&inner[close + 1..])
        });
        restricted.or_else(|| skip_spaces(tail))
    });
    after_pub
        .into_iter()
        .chain(std::iter::once(rest))
        .find_map(|candidate| {
            let after = skip_spaces(candidate.strip_prefix("fn")?)?;
            let length = after
                .find(|character: char| !(character.is_ascii_alphanumeric() || character == '_'))
                .unwrap_or(after.len());
            (length > 0).then(|| &after[..length])
        })
}

/// Each enclosing production function that calls `symbol`, in the order first found, with the span it passes:
/// `clause` when the call's argument, or the `let lowered` binding before it, is narrowed by
/// `constraint_bearing_sentence` or names a `clause`; `whole` otherwise.
fn call_sites(root: &Path, symbol: &str) -> Result<Vec<(String, String)>, String> {
    let text = read_text(&root.join(EVIDENCE_SOURCE))?;
    let lines: Vec<String> = text.split_inclusive('\n').map(str::to_string).collect();
    let test_lines = test_module_lines(&lines);
    let call = format!("{symbol}(");
    let mut sites: Vec<(String, String)> = Vec::new();
    let mut enclosing = "<file scope>".to_string();
    let mut pending_binding = String::new();
    for (number, line) in lines.iter().enumerate() {
        let function = definition(line);
        if let Some(name) = function {
            enclosing = name.to_string();
            pending_binding.clear();
        }
        let stripped = strip_line(line);
        if stripped.starts_with("let lowered") {
            pending_binding = stripped.to_string();
        }
        let Some((_, argument)) = line.split_once(&call) else {
            continue;
        };
        if function.is_some() || test_lines.contains(&number) {
            continue;
        }
        let source = format!("{argument} {pending_binding}");
        let scope = if source.contains("constraint_bearing_sentence") || source.contains("clause") {
            "clause"
        } else {
            "whole"
        };
        match sites.iter_mut().find(|(name, _)| *name == enclosing) {
            Some(site) => site.1 = scope.to_string(),
            None => sites.push((enclosing.clone(), scope.to_string())),
        }
    }
    Ok(sites)
}

/// A `dict[str, str]`'s `repr()`.
fn dict_repr<'a>(pairs: impl Iterator<Item = (&'a str, &'a str)>) -> String {
    let items: Vec<String> = pairs
        .map(|(key, value)| format!("{}: {}", text_repr(key), text_repr(value)))
        .collect();
    format!("{{{}}}", items.join(", "))
}

/// A `set[str]`'s `repr()`, its members sorted where Python's order would follow string hashes.
fn set_repr(members: &BTreeSet<String>) -> String {
    if members.is_empty() {
        return "set()".to_string();
    }
    let items: Vec<String> = members.iter().map(|member| text_repr(member)).collect();
    format!("{{{}}}", items.join(", "))
}

fn text_repr(text: &str) -> String {
    pyjson::string_repr(&pyjson::py_str(text))
}

/// `--check`: the declared topology against the scanned one, then the untyped arm's single way in.
fn check(root: &Path) -> i32 {
    let sites = match call_sites(root, CLASSIFIER_ENTRY) {
        Ok(sites) => sites,
        Err(error) => return refuse(&error),
    };
    let untyped: BTreeSet<String> = match call_sites(root, CLASSIFIER) {
        Ok(callers) => callers.into_iter().map(|(name, _)| name).collect(),
        Err(error) => return refuse(&error),
    };
    let found: BTreeMap<&str, &str> = sites
        .iter()
        .map(|(name, scope)| (name.as_str(), scope.as_str()))
        .collect();
    let expected: BTreeMap<&str, &str> = EXPECTED_CLASSIFIER_CALLERS.iter().copied().collect();
    if found != expected {
        println!(
            "{LABEL}: the kind classifier's call sites moved.\n  expected {}\n  found    {}\nThis census stratifies \
             by which producer reaches the classifier; re-derive it.",
            dict_repr(EXPECTED_CLASSIFIER_CALLERS.iter().copied()),
            dict_repr(
                sites
                    .iter()
                    .map(|(name, scope)| (name.as_str(), scope.as_str()))
            ),
        );
        return 1;
    }
    let expected_untyped: BTreeSet<String> = EXPECTED_UNTYPED_CALLERS
        .iter()
        .map(|name| (*name).to_string())
        .collect();
    if untyped != expected_untyped {
        println!(
            "{LABEL}: a producer reaches the UNTYPED classifier directly.\n  expected {}\n  found    {}\nThe terminal \
             arm publishes a kind no document stated; every producer must go through the typed gateway \
             (EXTRACTION-QUALITY-GAUGE.3k.2a/.3k.2e).",
            set_repr(&expected_untyped),
            set_repr(&untyped),
        );
        return 1;
    }
    let whole = sites.iter().filter(|(_, scope)| scope == "whole").count();
    println!(
        "{LABEL}: kind-classifier call sites unchanged ({} producers, {whole} reading the whole statement; the \
         untyped arm is reached only by {}).",
        sites.len(),
        untyped.first().expect("the declared caller")
    );
    0
}

// The census.

/// One record a stratum lists.
struct Entry {
    document: String,
    constraint_id: PyStr,
    subject_signal: Py,
    published_kind: Py,
    whole_kind: String,
    whole_arm: &'static str,
    clause_kind: String,
    clause_arm: &'static str,
    clause: PyStr,
}

/// One producer's records.
struct Stratum {
    prefix: &'static str,
    label: &'static str,
    reaches_classifier: bool,
    records: usize,
    kind_span_differs: Vec<Entry>,
    negation_on_untyped_default: Vec<Entry>,
    reference_magnitude: Vec<Entry>,
}

impl Stratum {
    /// The three lists, under the keys the census names them by.
    fn lists(&self) -> [(&'static str, &[Entry]); 3] {
        [
            ("kind_span_differs", &self.kind_span_differs),
            (
                "negation_on_untyped_default",
                &self.negation_on_untyped_default,
            ),
            ("reference_magnitude", &self.reference_magnitude),
        ]
    }
}

struct Census {
    documents: usize,
    strata: Vec<Stratum>,
    /// Unknown constraint-id prefixes with their counts, in the order first seen.
    unknown: Vec<(PyStr, usize)>,
}

fn text(value: &str) -> Py {
    Py::Str(pyjson::py_str(value))
}

fn int(value: usize) -> Py {
    Py::Int(value.to_string())
}

fn dict(pairs: Vec<(&str, Py)>) -> Py {
    Py::Dict(
        pairs
            .into_iter()
            .map(|(key, value)| (pyjson::py_str(key), value))
            .collect(),
    )
}

impl Entry {
    fn to_py(&self) -> Py {
        dict(vec![
            ("document", text(&self.document)),
            ("constraint_id", Py::Str(self.constraint_id.clone())),
            ("subject_signal", self.subject_signal.clone()),
            ("published_kind", self.published_kind.clone()),
            ("whole_kind", text(&self.whole_kind)),
            ("whole_arm", text(self.whole_arm)),
            ("clause_kind", text(&self.clause_kind)),
            ("clause_arm", text(self.clause_arm)),
            ("clause", Py::Str(self.clause.clone())),
        ])
    }
}

impl Census {
    fn records(&self) -> usize {
        self.strata.iter().map(|stratum| stratum.records).sum()
    }

    /// `{"call_sites": sites, **result}`, as `--json` writes it.
    fn to_py(&self, sites: &[(String, String)]) -> Py {
        let strata = Py::Dict(
            self.strata
                .iter()
                .map(|stratum| {
                    let mut pairs = vec![
                        ("label", text(stratum.label)),
                        ("reaches_classifier", Py::Bool(stratum.reaches_classifier)),
                        ("records", int(stratum.records)),
                    ];
                    for (key, entries) in stratum.lists() {
                        pairs.push((key, Py::List(entries.iter().map(Entry::to_py).collect())));
                    }
                    (pyjson::py_str(stratum.prefix), dict(pairs))
                })
                .collect(),
        );
        dict(vec![
            (
                "call_sites",
                Py::Dict(
                    sites
                        .iter()
                        .map(|(name, scope)| (pyjson::py_str(name), text(scope)))
                        .collect(),
                ),
            ),
            ("documents", int(self.documents)),
            ("records", int(self.records())),
            ("strata", strata),
            (
                "unknown_prefixes",
                Py::Dict(
                    self.unknown
                        .iter()
                        .map(|(prefix, count)| (prefix.clone(), int(*count)))
                        .collect(),
                ),
            ),
        ])
    }
}

/// `re.sub(r"_\d+$", "", constraint_id)`: a final `_` and digits removed, before a final newline if there is one.
/// Python's `\d` also takes non-ASCII decimal digits, which no extractor id carries; this takes ASCII digits.
fn id_prefix(id: &[u32]) -> PyStr {
    let stop = if id.last() == Some(&u32::from('\n')) {
        id.len() - 1
    } else {
        id.len()
    };
    let digits = id[..stop]
        .iter()
        .rev()
        .take_while(|code| (u32::from('0')..=u32::from('9')).contains(code))
        .count();
    let start = stop - digits;
    if digits > 0 && start > 0 && id[start - 1] == u32::from('_') {
        let mut prefix = id[..start - 1].to_vec();
        prefix.extend_from_slice(&id[stop..]);
        prefix
    } else {
        id.to_vec()
    }
}

/// `sorted(glob("<root>/generated/evidence_ir/*/evidence_ir.json"))`: each visible directory's artifact, sorted by
/// its whole path, as Python sorts the strings glob returns.
fn evidence_paths(root: &Path) -> Vec<PathBuf> {
    let corpus = root.join("generated/evidence_ir");
    let Ok(entries) = fs::read_dir(&corpus) else {
        return Vec::new();
    };
    let mut paths: Vec<(String, PathBuf)> = entries
        .flatten()
        .filter(|entry| !entry.file_name().to_string_lossy().starts_with('.'))
        .filter(|entry| entry.path().is_dir())
        .map(|entry| entry.path().join("evidence_ir.json"))
        .filter(|path| path.symlink_metadata().is_ok())
        .map(|path| (path.to_string_lossy().into_owned(), path))
        .collect();
    paths.sort();
    paths.into_iter().map(|(_, path)| path).collect()
}

/// The records `evidence.get("signal_constraints", [])` iterates; an empty `dict` or `str` iterates nothing, and
/// anything else that is not a list stops the script.
fn signal_constraints<'a>(evidence: &'a Py, path: &Path) -> Result<&'a [Py], String> {
    let Py::Dict(_) = evidence else {
        return Err(format!("'{}' is not a JSON object", path.display()));
    };
    match pyjson::get(evidence, "signal_constraints") {
        None => Ok(&[]),
        Some(Py::List(records)) => Ok(records),
        Some(Py::Dict(pairs)) if pairs.is_empty() => Ok(&[]),
        Some(Py::Str(characters)) if characters.is_empty() => Ok(&[]),
        Some(_) => Err(format!(
            "'{}' has a signal_constraints that is not a list of records",
            path.display()
        )),
    }
}

fn census(root: &Path) -> Result<Census, String> {
    let mut strata: Vec<Stratum> = PRODUCERS
        .iter()
        .map(|(prefix, label, reaches)| Stratum {
            prefix,
            label,
            reaches_classifier: *reaches,
            records: 0,
            kind_span_differs: Vec::new(),
            negation_on_untyped_default: Vec::new(),
            reference_magnitude: Vec::new(),
        })
        .collect();
    let mut unknown: Vec<(PyStr, usize)> = Vec::new();
    let paths = evidence_paths(root);
    for path in &paths {
        let document = path
            .parent()
            .and_then(Path::file_name)
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();
        let evidence = pyjson::loads(&read_text(path)?)
            .map_err(|error| format!("'{}' is not JSON: {error}", path.display()))?;
        for record in signal_constraints(&evidence, path)? {
            let Some(Py::Str(constraint_id)) = pyjson::get(record, "constraint_id") else {
                return Err(format!(
                    "'{}' has a signal constraint without a string constraint_id",
                    path.display()
                ));
            };
            let prefix = id_prefix(constraint_id);
            let Some(stratum) = strata
                .iter_mut()
                .find(|stratum| pyjson::py_str(stratum.prefix) == prefix)
            else {
                match unknown.iter_mut().find(|(seen, _)| *seen == prefix) {
                    Some(seen) => seen.1 += 1,
                    None => unknown.push((prefix, 1)),
                }
                continue;
            };
            stratum.records += 1;
            let Some(Py::Str(source_text)) = pyjson::get(record, "source_text") else {
                return Err(format!(
                    "'{}' has a signal constraint without a string source_text",
                    path.display()
                ));
            };
            let clause = constraint_bearing_sentence(source_text);
            let clause_lowered = lower(clause);
            let (whole_kind, whole_arm) = classify_kind(&lower(source_text));
            let (clause_kind, clause_arm) = classify_kind(&clause_lowered);
            let differs = whole_kind != clause_kind;
            let negated = obligation_is_negated(&clause_lowered)
                && UNTYPED_DEFAULT_ARMS.contains(&clause_arm);
            let magnitude = is_reference_magnitude(&clause_lowered);
            let entry = || Entry {
                document: document.clone(),
                constraint_id: constraint_id.clone(),
                subject_signal: pyjson::get(record, "subject_signal")
                    .cloned()
                    .unwrap_or(Py::None),
                published_kind: pyjson::get(record, "constraint_kind")
                    .cloned()
                    .unwrap_or(Py::None),
                whole_kind: whole_kind.clone(),
                whole_arm,
                clause_kind: clause_kind.clone(),
                clause_arm,
                clause: strip(clause).to_vec(),
            };
            if differs {
                stratum.kind_span_differs.push(entry());
            }
            if negated {
                stratum.negation_on_untyped_default.push(entry());
            }
            if magnitude {
                stratum.reference_magnitude.push(entry());
            }
        }
    }
    Ok(Census {
        documents: paths.len(),
        strata,
        unknown,
    })
}

/// The census as the script prints it by default.
fn report(result: &Census, sites: &[(String, String)]) -> String {
    let mut out = String::new();
    let mut line = |text: String| {
        out.push_str(&text);
        out.push('\n');
    };
    line(format!(
        "constraint-part-span census over {} persisted evidence artifacts",
        result.documents
    ));
    line(format!("  {} signal-constraint records", result.records()));
    line(String::new());
    line("kind-classifier call sites (the population a kind-span change can move):".to_string());
    let mut sorted_sites: Vec<&(String, String)> = sites.iter().collect();
    sorted_sites.sort();
    for (name, scope) in sorted_sites {
        line(format!("  {name:<48} reads the {scope}"));
    }
    line(String::new());
    let mut strata: Vec<&Stratum> = result.strata.iter().collect();
    strata.sort_by_key(|stratum| stratum.prefix);
    for stratum in strata {
        let reaches = stratum.reaches_classifier;
        // A row this producer's kind never passes through cannot be a population for a change to that classifier:
        // `.3k` opened on a count read off strata the classifier never runs on.
        let reach = if reaches {
            "reaches the kind classifier"
        } else {
            "DOES NOT reach the kind classifier — every classifier-derived row below is a MIRROR ARTIFACT, not a \
             population"
        };
        let mark = if reaches { "" } else { "[n/a] " };
        line(format!("[{}_*] {}", stratum.prefix, stratum.label));
        line(format!("    {} records — {reach}", stratum.records));
        line(format!(
            "    {mark}kind differs clause vs whole : {}",
            stratum.kind_span_differs.len()
        ));
        line(format!(
            "    {mark}negation on untyped default  : {}",
            stratum.negation_on_untyped_default.len()
        ));
        line(format!(
            "    reference-operand magnitude  : {}",
            stratum.reference_magnitude.len()
        ));
        for (key, entries) in stratum.lists() {
            let entry_mark = if reaches || key == "reference_magnitude" {
                ""
            } else {
                "[n/a] "
            };
            for entry in entries {
                line(format!(
                    "      · {entry_mark}{key} {} {} subject={} whole={}({}) clause={}({})",
                    entry.document,
                    pyjson::to_text(&entry.constraint_id),
                    pyjson::str_of(&entry.subject_signal),
                    entry.whole_kind,
                    entry.whole_arm,
                    entry.clause_kind,
                    entry.clause_arm
                ));
                let shown = &entry.clause[..entry.clause.len().min(200)];
                line(format!("        clause: {}", pyjson::to_text(shown)));
            }
        }
        line(String::new());
    }
    if !result.unknown.is_empty() {
        let items: Vec<String> = result
            .unknown
            .iter()
            .map(|(prefix, count)| format!("{}: {count}", pyjson::string_repr(prefix)))
            .collect();
        line(format!(
            "unknown constraint-id prefixes: {{{}}}",
            items.join(", ")
        ));
    }
    out
}

// The self-test.

fn bool_repr(value: bool) -> String {
    if value { "True" } else { "False" }.to_string()
}

fn pair_repr(left: &str, right: &str) -> String {
    format!("({left}, {right})")
}

/// Each case: its id, the `repr()` of what the mirror returned, and the `repr()` it must return. The mirrors are
/// the only thing worth testing: the census itself is a fold over them.
fn self_test_cases() -> Vec<(&'static str, String, String)> {
    let sentence = |input: &str| {
        let input = pyjson::py_str(input);
        pyjson::to_text(constraint_bearing_sentence(&input))
    };
    let stripped_sentence = |input: &str| {
        let input = pyjson::py_str(input);
        pyjson::to_text(strip(constraint_bearing_sentence(&input)))
    };
    let guarded: Vec<String> = [
        "#[cfg(test)]\n",
        "pub(super) fn helper() -> u8 {\n",
        "    0\n",
        "}\n",
        "fn production() -> u8 {\n",
        "    1\n",
        "}\n",
        "#[cfg(test)]\n",
        "mod tests {\n",
        "    fn control() {}\n",
        "}\n",
    ]
    .iter()
    .map(|line| (*line).to_string())
    .collect();
    let guarded: Vec<String> = test_module_lines(&guarded)
        .iter()
        .map(usize::to_string)
        .collect();
    vec![
        (
            "clause-is-the-first-modal-sentence",
            text_repr(&stripped_sentence(
                "PREADY is asserted. PADDR must be stable. HSELx must be asserted.",
            )),
            text_repr("PADDR must be stable"),
        ),
        (
            "clause-falls-back-to-the-whole-text-without-a-modal",
            text_repr(&sentence("PSEL is tied HIGH")),
            text_repr("PSEL is tied HIGH"),
        ),
        (
            "kind-phrase-in-a-later-sentence-is-outside-the-clause",
            pair_repr(
                &text_repr(
                    classify_kind("when the subordinate is selected, it must also monitor hreadyx").1,
                ),
                &text_repr(
                    classify_kind(
                        "when the subordinate is selected, it must also monitor hreadyx. zetaselx must be \
                         asserted in the same cycle",
                    )
                    .1,
                ),
            ),
            pair_repr(&text_repr("untyped_default"), &text_repr("must_be_asserted")),
        ),
        (
            "negated-spelling-types-the-obligation",
            text_repr(&classify_kind("zetastrb must not be asserted during a read").0),
            text_repr("must_be_deasserted"),
        ),
        (
            "stability-phrase-is-not-the-untyped-default",
            text_repr(classify_kind("zetaaddr must be stable").1),
            text_repr("stable_phrase"),
        ),
        (
            "value-binder-lifts-the-bound-word-not-a-filler",
            extract_protocol_state_value("the zeta field shall be set to the reserved value")
                .map_or_else(|| "None".to_string(), |value| text_repr(&value)),
            text_repr("RESERVED"),
        ),
        (
            "negation-is-read-from-whatever-span-it-is-given",
            pair_repr(
                &bool_repr(obligation_is_negated("the zeta operand must be 1 for zetadir")),
                &bool_repr(obligation_is_negated(
                    "a command must not be issued while the queue is full",
                )),
            ),
            pair_repr(&bool_repr(false), &bool_repr(true)),
        ),
        (
            "reference-operand-magnitude-is-recognized",
            bool_repr(is_reference_magnitude(
                "the range given by this field must not be greater than the size indicated by the oas field",
            )),
            bool_repr(true),
        ),
        (
            "literal-operand-magnitude-is-not-a-reference-magnitude",
            bool_repr(is_reference_magnitude("this field must be greater than 0")),
            bool_repr(false),
        ),
        (
            "cfg-test-marks-the-item-it-guards-not-everything-until-the-next-mod",
            format!("[{}]", guarded.join(", ")),
            "[0, 1, 2, 3, 7, 8, 9, 10]".to_string(),
        ),
    ]
}

/// The script's `--self-test`: stops at the first case that fails, then holds the count to its declaration.
fn self_test() -> i32 {
    let mut passed = 0;
    for (case_id, actual, expected) in self_test_cases() {
        if actual != expected {
            println!("self-test FAIL {case_id}: expected {expected}, got {actual}");
            return 1;
        }
        passed += 1;
    }
    if passed != EXPECTED_SELF_TEST_CASES {
        println!(
            "self-test FAIL: ran {passed} cases, expected {EXPECTED_SELF_TEST_CASES} — a case was removed; restore \
             it or move the declared total deliberately."
        );
        return 1;
    }
    println!("{LABEL}: {passed}/{EXPECTED_SELF_TEST_CASES} mirror cases pass.");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_self_test_case_holds_and_the_real_topology_is_the_declared_one() {
        let cases = self_test_cases();
        assert_eq!(cases.len(), EXPECTED_SELF_TEST_CASES);
        for (case_id, actual, expected) in cases {
            assert_eq!(actual, expected, "{case_id}");
        }
        assert_eq!(check(&crate::repository_root()), 0);
    }

    #[test]
    fn definitions_read_as_the_regex_reads_them() {
        for (line, name) in [
            ("pub(crate) fn alpha(x: u8) {\n", Some("alpha")),
            ("    fn beta<T>() {\n", Some("beta")),
            ("pub\tfn gamma()\n", Some("gamma")),
            ("pub (crate) fn delta()\n", None),
            ("pub(super)fn epsilon()\n", None),
            ("async fn zeta()\n", None),
            ("fnord()\n", None),
            ("fn  eta_1()\n", Some("eta_1")),
        ] {
            assert_eq!(definition(line), name, "{line:?}");
        }
    }

    #[test]
    fn an_id_loses_only_its_final_number() {
        for (id, prefix) in [
            ("sigcon_0001", "sigcon"),
            ("keyed_row_sigcon_12", "keyed_row_sigcon"),
            ("x_12_34", "x_12"),
            ("x_1\n", "x\n"),
            ("x_", "x_"),
            ("_5", ""),
            ("x5", "x5"),
        ] {
            assert_eq!(
                id_prefix(&pyjson::py_str(id)),
                pyjson::py_str(prefix),
                "{id:?}"
            );
        }
    }

    /// The declared topology in miniature: each producer's call and its span, and the gateway's one call into the
    /// untyped arm.
    const DECLARED: &str = "fn extract_signal_constraints(text: &str) {
    let lowered = text.to_lowercase();
    classify_signal_constraint_kind_typed(&lowered, &values);
}
fn extract_signal_description_row_constraints(cell: &str) {
    classify_signal_constraint_kind_typed(&clause, &values);
}
fn extract_signal_keyed_obligation_row_constraints(row: &str) {
    let lowered = constraint_bearing_sentence(row);
    classify_signal_constraint_kind_typed(&lowered, &values);
}
fn classify_signal_constraint_kind_typed(lowered: &str, values: &Values) -> Kind {
    let kind = classify_signal_constraint_kind(lowered, values);
}
";

    #[test]
    fn a_moved_topology_or_a_second_way_into_the_untyped_arm_fails_closed() {
        let root =
            crate::repository_root().join(".project-data/tmp/xtask-constraint-part-span-check");
        let source = root.join(EVIDENCE_SOURCE);
        fs::create_dir_all(source.parent().expect("a parent")).expect("the fixture's directories");
        let mut verdicts = Vec::new();
        for (name, extra, expected) in [
            ("declared", "", 0),
            (
                "a test helper's call",
                "#[cfg(test)]\nfn helper() {\n    classify_signal_constraint_kind_typed(&x, &v);\n}\n",
                0,
            ),
            (
                "a new producer",
                "fn extract_rogue_constraints(text: &str) {\n    classify_signal_constraint_kind_typed(&text, &v);\n}\n",
                1,
            ),
            (
                "a second way into the untyped arm",
                "fn rogue_reader(text: &str) {\n    classify_signal_constraint_kind(&text, &v);\n}\n",
                1,
            ),
        ] {
            fs::write(&source, format!("{DECLARED}{extra}")).expect("the fixture's source");
            verdicts.push((name, check(&root), expected));
        }
        fs::remove_dir_all(&root).expect("cleanup");
        for (name, actual, expected) in verdicts {
            assert_eq!(actual, expected, "{name}");
        }
    }

    #[test]
    fn the_census_stratifies_by_producer_and_counts_the_unknown_prefixes() {
        let root =
            crate::repository_root().join(".project-data/tmp/xtask-constraint-part-span-census");
        let artifact = root.join("generated/evidence_ir/doc/evidence_ir.json");
        fs::create_dir_all(artifact.parent().expect("a parent"))
            .expect("the fixture's directories");
        let records = [
            (
                "sigcon_0001",
                "ZETA must also monitor ETA. ZETA must be high",
            ),
            ("dyn_sigcon_0002", "ZETA cannot be driven"),
            ("zz_0003", "unread"),
        ]
        .iter()
        .map(|(id, text)| format!("{{\"constraint_id\": \"{id}\", \"source_text\": \"{text}\"}}"))
        .collect::<Vec<_>>()
        .join(", ");
        fs::write(
            &artifact,
            format!("{{\"signal_constraints\": [{records}]}}"),
        )
        .expect("the fixture's artifact");
        let result = census(&root);
        fs::remove_dir_all(&root).expect("cleanup");
        let result = result.expect("a census");
        let stratum = |prefix: &str| {
            result
                .strata
                .iter()
                .find(|stratum| stratum.prefix == prefix)
                .expect("a declared producer")
        };
        assert_eq!((result.documents, result.records()), (1, 2));
        assert_eq!(stratum("sigcon").kind_span_differs.len(), 1);
        assert_eq!(stratum("dyn_sigcon").negation_on_untyped_default.len(), 1);
        assert_eq!(result.unknown, vec![(pyjson::py_str("zz"), 1)]);
    }
}
