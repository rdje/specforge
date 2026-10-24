//! `PRODUCTION-GENERICITY`'s `BEHAVIORAL-CONTRACT` component, native since `RUSTDOC-AND-XTASK.15.d`: the frozen
//! behavioral-genericity population and its relational oracle (`SPEC-TO-INTENT-ALIGNMENT.6d.ii.f`) still hold. It
//! checks the contract's closed authority set, relations, stage comparison, input planes, failure taxonomy and
//! evidence contract; the reviewed recipes and the semantic-negative matrix; each population row against its
//! SourceIR, normalized Markdown, live source and current stage artifacts; the population assertions and frozen
//! census; the held-out evidence's split, attempts, coverage, strata and Wilson intervals; and that every retained
//! bundle above the frozen floor is declared an unqualified residual (ADR 0050).
//!
//! The script compared, counted and printed values as Python does, and so does this: equality across `int`, `float`
//! and `bool`, `isinstance(value, int)` admitting a boolean, `round` to even, `sum`'s order of addition, Python's
//! `repr` of strings, lists and tuples, its `OSError` and `UnicodeDecodeError` text, universal newlines, `csv`'s tab
//! dialect and `PurePath`'s normalization. Where the script would stop with a traceback, this names the reason and
//! exits 1, as the traceback did. Three differences are deliberate: a row's non-integer fields are reported in their
//! declared order (the script iterated a `set`, so its order changed from run to run), and a population integer
//! beyond `i64` and a non-ASCII digit are refused where Python would have read them.
//!
//! `scripts/check_behavioral_genericity_contract.py` is a shim that keeps its `argparse` front end and passes
//! `--self-test`.

use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use sha2::{Digest, Sha256};

use super::{Output, Terminal};
use crate::pyjson::{self, Py, PyStr};

const LABEL: &str = "behavioral-genericity-contract";
const CONTRACT_PATH: &str = "doctrine/production_genericity/behavioral_qualification.json";
const POPULATION_PATH: &str = "doctrine/production_genericity/behavioral_population.tsv";
const RECIPE_MANIFEST_PATH: &str = "doctrine/production_genericity/reviewed_recipe_manifest.json";
const NEGATIVE_MATRIX_PATH: &str = "doctrine/production_genericity/semantic_negative_matrix.json";
const HELD_OUT_EVIDENCE_PATH: &str =
    "doctrine/production_genericity/behavioral_holdout_evidence.json";
const BEHAVIORAL_TOOL_PATH: &str = "crates/specforge-conformance/src/behavioral_genericity.rs";
/// ADR 0050: retained keys above the frozen floor are declared here, outside the frozen contract.
const POST_BOUNDARY_PATH: &str = "doctrine/production_genericity/post_boundary_retention.json";

const POPULATION_FIELDS: [&str; 19] = [
    "document_key",
    "source_origin",
    "source_locator",
    "source_sha256",
    "source_bytes",
    "normalized_markdown_path",
    "normalized_markdown_sha256",
    "pages",
    "visual_assets",
    "structured_tables",
    "content_elements",
    "sections",
    "vendor",
    "family",
    "category",
    "layout",
    "review_role",
    "text_semantic_records",
    "text_intent_records",
];
const INTEGER_FIELDS: [&str; 8] = [
    "source_bytes",
    "pages",
    "visual_assets",
    "structured_tables",
    "content_elements",
    "sections",
    "text_semantic_records",
    "text_intent_records",
];
const EXPECTED_RELATIONS: [&str; 6] = [
    "unchanged_source",
    "adversarial_identity",
    "symbol_alpha",
    "structure_preserving_paraphrase",
    "harmless_layout",
    "semantic_negative",
];
const EXPECTED_STAGES: [&str; 5] = [
    "source_ir",
    "evidence_ir",
    "semantic_ir",
    "intent_ir",
    "isf_adapter",
];
const EXPECTED_PLANES: [&str; 3] = [
    "pdf_full_capture",
    "normalized_text_projection",
    "reviewed_variant",
];
const EXPECTED_PLANE_RELATIONS: [(&str, &[&str]); 3] = [
    (
        "pdf_full_capture",
        &["unchanged_source", "adversarial_identity"],
    ),
    ("normalized_text_projection", &["symbol_alpha"]),
    (
        "reviewed_variant",
        &[
            "structure_preserving_paraphrase",
            "harmless_layout",
            "semantic_negative",
        ],
    ),
];
const EXPECTED_EVIDENCE_IDENTITY: [&str; 6] = [
    "contract_sha256",
    "production_revision",
    "source_sha256",
    "transform_recipe_sha256",
    "prior_memory_sha256",
    "tool_sha256",
];
const EXPECTED_EVIDENCE_COVERAGE: [&str; 8] = [
    "declared_documents",
    "attempted_documents",
    "completed_documents",
    "unmeasurable_documents",
    "all_stage_fields",
    "all_proof_claims",
    "all_expected_deltas",
    "all_unaffected_complements",
];
const EXPECTED_FAILURES: [(&str, &str); 11] = [
    ("authority_unavailable", "unmeasurable"),
    ("provider_unavailable", "unmeasurable"),
    ("vacuous_baseline", "unmeasurable"),
    ("eligible_symbol_surface_absent", "unmeasurable"),
    ("stale_contract_or_population", "invalid"),
    ("ambiguous_or_nonbijective_transform", "invalid"),
    ("partial_or_escaped_run", "invalid"),
    ("missing_expected_delta", "fail"),
    ("undeclared_semantic_delta", "fail"),
    ("proof_or_provenance_regression", "fail"),
    ("validation_or_lowering_regression", "fail"),
];
const EXPECTED_REVIEWED_RECIPE_RELATIONS: [&str; 3] = [
    "structure_preserving_paraphrase",
    "harmless_layout",
    "semantic_negative",
];
const EXPECTED_REVIEWED_CHANGE_KINDS: [(&str, &[&str]); 3] = [
    ("structure_preserving_paraphrase", &["sentence_paraphrase"]),
    (
        "harmless_layout",
        &[
            "heading_layout",
            "table_layout",
            "whitespace_layout",
            "formatting_layout",
        ],
    ),
    ("semantic_negative", &["semantic_timing_change"]),
];
const EXPECTED_NEGATIVE_CONTROL_KINDS: [&str; 9] = [
    "omission",
    "contradiction",
    "relation_reversal",
    "value_change",
    "timing_change",
    "undeclared_symbol",
    "misleading_name",
    "proof_corruption",
    "disabled_stage",
];
const EXPECTED_ATTEMPT_DISPOSITIONS: [(&str, &str); 6] = [
    ("authority_unavailable", "unmeasurable"),
    ("provider_unavailable", "unmeasurable"),
    ("vacuous_baseline", "unmeasurable"),
    ("stale_contract_or_population", "invalid"),
    ("ambiguous_or_nonbijective_transform", "invalid"),
    ("partial_or_escaped_run", "invalid"),
];
const SAFE_REVIEWED_PROVENANCE_FIELDS: [&str; 6] = [
    "conclusion",
    "normalized_markdown",
    "responsibilities",
    "source_text",
    "statement",
    "text",
];
const RICH_CAPTURE_EXCLUSIONS: [&str; 5] = [
    "content_elements",
    "document_sections",
    "page_artifacts",
    "structured_tables",
    "visual_assets",
];
const REVIEWED_COMPLEMENT: &str = "all_unlisted_leaf_values_and_all_proof_topology_exact";
const ARTIFACT_PATHS: [(&str, &str); 5] = [
    ("source_ir", "generated/source_ir/{key}/source_ir.json"),
    (
        "evidence_ir",
        "generated/evidence_ir/{key}/evidence_ir.json",
    ),
    (
        "semantic_ir",
        "generated/semantic_ir/{key}/semantic_ir.json",
    ),
    ("intent_ir", "generated/intent_ir/{key}/intent_ir.json"),
    ("isf_adapter", "generated/adapters/isf/{key}/adapter.json"),
];
const HELD_OUT_RELATIONS: [&str; 3] = ["unchanged_source", "adversarial_identity", "symbol_alpha"];
const STATES: [&str; 4] = ["pass", "fail", "unmeasurable", "invalid"];

// Python's view of values.

/// An exception the script did not catch: a traceback there, a named refusal (exit 1) here.
#[derive(Debug)]
struct Uncaught(String);

type Checked<T> = Result<T, Uncaught>;

static EMPTY: Py = Py::Dict(Vec::new());

fn s(text: &str) -> Py {
    Py::Str(pyjson::py_str(text))
}

fn text(value: &[u32]) -> String {
    pyjson::to_text(value)
}

fn int(value: i64) -> Py {
    Py::Int(value.to_string())
}

fn is_str(value: Option<&Py>) -> bool {
    matches!(value, Some(Py::Str(_)))
}

fn as_str(value: Option<&Py>) -> Option<String> {
    match value {
        Some(Py::Str(characters)) => Some(text(characters)),
        _ => None,
    }
}

fn is_dict(value: Option<&Py>) -> bool {
    matches!(value, Some(Py::Dict(_)))
}

/// `isinstance(value, int)`: a boolean is an `int` in Python.
fn is_int(value: Option<&Py>) -> bool {
    matches!(value, Some(Py::Int(_) | Py::Bool(_)))
}

fn is_none(value: Option<&Py>) -> bool {
    matches!(value, None | Some(Py::None))
}

/// `value == expected` under Python's equality, a missing value being `None`.
fn eq(value: Option<&Py>, expected: &Py) -> bool {
    pyjson::py_eq(value.unwrap_or(&Py::None), expected)
}

fn eq_str(value: Option<&Py>, expected: &str) -> bool {
    eq(value, &s(expected))
}

fn py_repr(value: Option<&Py>) -> String {
    pyjson::repr(value.unwrap_or(&Py::None))
}

fn str_repr(value: &str) -> String {
    pyjson::string_repr(&pyjson::py_str(value))
}

fn list_repr<S: AsRef<str>>(items: &[S]) -> String {
    let items: Vec<String> = items.iter().map(|item| str_repr(item.as_ref())).collect();
    format!("[{}]", items.join(", "))
}

/// `dict.get(key)`, refusing a value that is not a dict as Python's `AttributeError` would.
fn attr<'a>(value: &'a Py, key: &str) -> Checked<Option<&'a Py>> {
    match value {
        Py::Dict(_) => Ok(pyjson::get(value, key)),
        other => Err(Uncaught(format!(
            "'{}' object has no attribute 'get'",
            type_name(other)
        ))),
    }
}

/// `dict.get(key, {})`.
fn attr_or_empty<'a>(value: &'a Py, key: &str) -> Checked<&'a Py> {
    Ok(attr(value, key)?.unwrap_or(&EMPTY))
}

fn type_name(value: &Py) -> &'static str {
    match value {
        Py::None => "NoneType",
        Py::Bool(_) => "bool",
        Py::Int(_) => "int",
        Py::Float(_) => "float",
        Py::Str(_) => "str",
        Py::List(_) => "list",
        Py::Dict(_) => "dict",
    }
}

fn keys(value: &Py) -> Vec<String> {
    match value {
        Py::Dict(pairs) => pairs.iter().map(|(key, _)| text(key)).collect(),
        _ => Vec::new(),
    }
}

/// `set(value) == expected` for a dict's keys.
fn key_set_is(value: &Py, expected: &[&str]) -> bool {
    let actual: BTreeSet<String> = keys(value).into_iter().collect();
    let expected: BTreeSet<String> = expected.iter().map(|key| (*key).to_string()).collect();
    actual == expected
}

/// `set(values)`: the distinct hashable values, refusing a list or dict as Python's `TypeError` would.
fn py_set(values: &[Py]) -> Checked<Vec<Py>> {
    let mut distinct: Vec<Py> = Vec::new();
    for value in values {
        if matches!(value, Py::List(_) | Py::Dict(_)) {
            return Err(Uncaught(format!("unhashable type: '{}'", type_name(value))));
        }
        if !distinct.iter().any(|seen| pyjson::py_eq(seen, value)) {
            distinct.push(value.clone());
        }
    }
    Ok(distinct)
}

/// `set(values) == expected`, the expected set being strings.
fn set_is(values: &[Py], expected: &[&str]) -> Checked<bool> {
    let distinct = py_set(values)?;
    Ok(
        distinct.len() == expected.iter().collect::<BTreeSet<_>>().len()
            && distinct
                .iter()
                .all(|value| expected.iter().any(|item| pyjson::py_eq(value, &s(item))))
            && expected
                .iter()
                .all(|item| distinct.iter().any(|value| pyjson::py_eq(value, &s(item)))),
    )
}

/// `len(values) != len(set(values))`.
fn has_duplicates(values: &[Py]) -> Checked<bool> {
    Ok(py_set(values)?.len() != values.len())
}

/// `needle in container`: a substring of a `str`, a member of a `list`, a key of a `dict`.
fn contains(container: &Py, needle: &str) -> Checked<bool> {
    match container {
        Py::Str(characters) => Ok(text(characters).contains(needle)),
        Py::List(items) => Ok(items.iter().any(|item| pyjson::py_eq(item, &s(needle)))),
        Py::Dict(_) => Ok(pyjson::get(container, needle).is_some()),
        other => Err(Uncaught(format!(
            "argument of type '{}' is not iterable",
            type_name(other)
        ))),
    }
}

/// Python's `str.strip()` is empty.
fn blank(value: &str) -> bool {
    value.chars().all(is_python_space)
}

/// `re.fullmatch(r"[0-9a-f]{N}", value)`.
fn is_hex(value: Option<&Py>, length: usize) -> bool {
    as_str(value).is_some_and(|value| {
        value.len() == length
            && value
                .bytes()
                .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
    })
}

/// A number in Python's generic `+`: an `int` (here within `i128`) or a `float`.
#[derive(Clone, Copy)]
enum Number {
    Int(i128),
    Float(f64),
}

impl Number {
    fn of(value: &Py) -> Checked<Self> {
        match value {
            Py::Bool(flag) => Ok(Number::Int(i128::from(*flag))),
            Py::Int(digits) => digits
                .parse()
                .map(Number::Int)
                .map_err(|_| Uncaught(format!("integer too large: {digits}"))),
            Py::Float(number) => Ok(Number::Float(*number)),
            other => Err(Uncaught(format!(
                "unsupported operand type(s) for +: 'int' and '{}'",
                type_name(other)
            ))),
        }
    }

    /// `self + other`, an `int` converted to `float` (rounded to nearest) beside a `float`.
    fn add(self, other: Number) -> Checked<Number> {
        Ok(match (self, other) {
            (Number::Int(left), Number::Int(right)) => Number::Int(
                left.checked_add(right)
                    .ok_or_else(|| Uncaught("integer too large".to_string()))?,
            ),
            (Number::Int(left), Number::Float(right)) => Number::Float(left as f64 + right),
            (Number::Float(left), Number::Int(right)) => Number::Float(left + right as f64),
            (Number::Float(left), Number::Float(right)) => Number::Float(left + right),
        })
    }

    fn into_py(self) -> Py {
        match self {
            Number::Int(value) => Py::Int(value.to_string()),
            Number::Float(value) => Py::Float(value),
        }
    }
}

/// Python's `sum(values)` as CPython 3.12 on computes it: `int`s (booleans counting) exactly while the total fits a
/// C `long`; from the first `float`, Neumaier's compensated summation, an `int` that fits a `long` added
/// uncompensated and the compensation folded in at the end; past an overflow, Python's generic `+` one term at a
/// time. A term that is not a number is Python's `TypeError`.
fn py_sum(values: impl IntoIterator<Item = Py>) -> Checked<Py> {
    enum State {
        Long(i64),
        Float { total: f64, compensation: f64 },
        Generic(Number),
    }
    let fold = |total: f64, compensation: f64| {
        if compensation != 0.0 && compensation.is_finite() {
            total + compensation
        } else {
            total
        }
    };
    let mut state = State::Long(0);
    for value in values {
        let term = Number::of(&value)?;
        state = match (state, term) {
            (State::Long(total), Number::Int(item)) => match i64::try_from(item)
                .ok()
                .and_then(|item| total.checked_add(item))
            {
                Some(sum) => State::Long(sum),
                None => State::Generic(Number::Int(i128::from(total)).add(term)?),
            },
            (State::Long(total), Number::Float(item)) => State::Float {
                total: total as f64 + item,
                compensation: 0.0,
            },
            (
                State::Float {
                    total,
                    mut compensation,
                },
                Number::Float(item),
            ) => {
                let sum = total + item;
                if total.abs() >= item.abs() {
                    compensation += (total - sum) + item;
                } else {
                    compensation += (item - sum) + total;
                }
                State::Float {
                    total: sum,
                    compensation,
                }
            }
            (
                State::Float {
                    total,
                    compensation,
                },
                Number::Int(item),
            ) => match i64::try_from(item) {
                Ok(item) => State::Float {
                    total: total + item as f64,
                    compensation,
                },
                Err(_) => State::Generic(Number::Float(fold(total, compensation)).add(term)?),
            },
            (State::Generic(total), _) => State::Generic(total.add(term)?),
        };
    }
    Ok(match state {
        State::Long(total) => Py::Int(total.to_string()),
        State::Float {
            total,
            compensation,
        } => Py::Float(fold(total, compensation)),
        State::Generic(total) => total.into_py(),
    })
}

/// `sum(coverage.get(field, 0) for …)`: one field across the attempts' coverage records.
fn coverage_total(coverages: &[&Py], field: &str) -> Checked<Py> {
    py_sum(
        coverages
            .iter()
            .map(|coverage| pyjson::get(coverage, field).cloned().unwrap_or(int(0))),
    )
}

/// `sum(sum(coverage.get(field, 0) for field in fields) for …)`: each record's fields summed, then the records, as
/// two sums round differently from one.
fn coverage_nested_total(coverages: &[&Py], fields: &[&str]) -> Checked<Py> {
    let totals = coverages
        .iter()
        .map(|coverage| {
            py_sum(
                fields
                    .iter()
                    .map(|field| pyjson::get(coverage, field).cloned().unwrap_or(int(0))),
            )
        })
        .collect::<Checked<Vec<Py>>>()?;
    py_sum(totals)
}

/// A tuple's `repr`.
fn tuple_repr(parts: &[Option<&Py>]) -> String {
    let parts: Vec<String> = parts.iter().map(|part| py_repr(*part)).collect();
    format!("({})", parts.join(", "))
}

// Paths as `pathlib.PurePosixPath` sees them.

/// A path's root and parts, `..` kept and empty and `.` components dropped. POSIX keeps a root of exactly two
/// slashes, and so does `PurePosixPath`; three or more are one.
struct PurePath {
    root: &'static str,
    parts: Vec<String>,
}

impl PurePath {
    fn new(value: &str) -> Self {
        let root = if value.starts_with("//") && !value.starts_with("///") {
            "//"
        } else if value.starts_with('/') {
            "/"
        } else {
            ""
        };
        PurePath {
            root,
            parts: value
                .split('/')
                .filter(|part| !part.is_empty() && *part != ".")
                .map(str::to_string)
                .collect(),
        }
    }

    fn absolute(&self) -> bool {
        !self.root.is_empty()
    }

    fn climbs(&self) -> bool {
        self.parts.iter().any(|part| part == "..")
    }

    fn as_posix(&self) -> String {
        let joined = self.parts.join("/");
        if self.root.is_empty() && joined.is_empty() {
            ".".to_string()
        } else {
            format!("{}{joined}", self.root)
        }
    }

    fn name(&self) -> String {
        self.parts.last().cloned().unwrap_or_default()
    }

    fn is_relative_to(&self, other: &PurePath) -> bool {
        self.root == other.root && self.parts.starts_with(&other.parts)
    }
}

/// `safe_relative_path`: a non-empty string that is neither absolute nor climbing.
fn safe_relative_path(value: Option<&Py>) -> Option<PurePath> {
    let value = as_str(value).filter(|value| !value.is_empty())?;
    let path = PurePath::new(&value);
    (!path.absolute() && !path.climbs()).then_some(path)
}

// Reading files as the script read them.

/// Why a JSON file could not be read: an `OSError` and a `JSONDecodeError`, which the guarded reads caught, and a
/// `UnicodeDecodeError`, which only the entry point's `ValueError` did.
enum ReadError {
    Os(String),
    Json(String),
    Decode(String),
}

impl ReadError {
    fn message(&self) -> &str {
        match self {
            ReadError::Os(message) | ReadError::Json(message) | ReadError::Decode(message) => {
                message
            }
        }
    }
}

/// Python's `UnicodeDecodeError` text for the first invalid sequence in `bytes`.
fn utf8_error(bytes: &[u8], error: &std::str::Utf8Error) -> String {
    let start = error.valid_up_to();
    let (end, reason) = match error.error_len() {
        None => (bytes.len(), "unexpected end of data"),
        Some(1) if !(0xc2..=0xf4).contains(&bytes[start]) => (start + 1, "invalid start byte"),
        Some(length) => (start + length, "invalid continuation byte"),
    };
    if end - start == 1 {
        format!(
            "'utf-8' codec can't decode byte 0x{:02x} in position {start}: {reason}",
            bytes[start]
        )
    } else {
        format!(
            "'utf-8' codec can't decode bytes in position {start}-{}: {reason}",
            end - 1
        )
    }
}

/// `path.open(encoding="utf-8").read()`: UTF-8, and `\r\n` and `\r` read as `\n` (universal newlines).
fn read_text(path: &Path) -> Result<String, ReadError> {
    let bytes = fs::read(path).map_err(|error| ReadError::Os(os_error(&error, path)))?;
    let text = std::str::from_utf8(&bytes)
        .map_err(|error| ReadError::Decode(utf8_error(&bytes, &error)))?;
    Ok(if text.contains('\r') {
        text.replace("\r\n", "\n").replace('\r', "\n")
    } else {
        text.to_string()
    })
}

/// `TextIOWrapper`'s chunk: the population is decoded 8,192 bytes at a time, so a decoding error counts its
/// position from the start of its chunk (measured: an invalid byte at 9,000 reports position 808).
const TEXT_CHUNK: usize = 8192;

/// Incremental UTF-8 decoding as `TextIOWrapper` reads a file: the text, or the text decoded before the failing
/// chunk, that chunk's index and the error.
fn decode_chunked(bytes: &[u8]) -> Result<String, (String, usize, String)> {
    let mut text = String::new();
    let mut pending: Vec<u8> = Vec::new();
    for (index, chunk) in bytes.chunks(TEXT_CHUNK).enumerate() {
        let mut input = std::mem::take(&mut pending);
        input.extend_from_slice(chunk);
        match std::str::from_utf8(&input) {
            Ok(valid) => text.push_str(valid),
            Err(error) if error.error_len().is_none() => {
                text.push_str(&String::from_utf8_lossy(&input[..error.valid_up_to()]));
                pending = input[error.valid_up_to()..].to_vec();
            }
            Err(error) => return Err((text, index, utf8_error(&input, &error))),
        }
    }
    match std::str::from_utf8(&pending) {
        Err(error) => Err((
            text,
            bytes.len().div_ceil(TEXT_CHUNK),
            utf8_error(&pending, &error),
        )),
        Ok(_) => Ok(text),
    }
}

/// Python's `str(OSError)`: `[Errno N] strerror: 'path'`.
fn os_error(error: &std::io::Error, path: &Path) -> String {
    let text = error.to_string();
    format!(
        "[Errno {}] {}: {}",
        error.raw_os_error().unwrap_or(0),
        text.split(" (os error").next().unwrap_or(&text),
        str_repr(&path.to_string_lossy())
    )
}

fn parse_json_file(path: &Path) -> Result<Py, ReadError> {
    pyjson::loads(&read_text(path)?).map_err(ReadError::Json)
}

/// The sections whose length the check compares with the population's counts.
const SOURCE_IR_SECTIONS: [&str; 5] = [
    "page_artifacts",
    "visual_assets",
    "structured_tables",
    "content_elements",
    "document_sections",
];

/// A value of the same Python type and `len()` with nothing inside: what a `len()` or an `isinstance` sees.
fn hollow(value: Py) -> Py {
    match value {
        Py::List(items) => Py::List(vec![Py::None; items.len()]),
        Py::Dict(pairs) => Py::Dict(pairs.into_iter().map(|(key, _)| (key, Py::None)).collect()),
        Py::Str(characters) => Py::Str(vec![0; characters.len()]),
        other => other,
    }
}

/// What the check reads of a SourceIR, and no more: its `source`, `normalization_plan`, `document_identity` and
/// `stage` whole, each counted section hollowed, and of its proof ledger whether `claims` is a non-empty list. Every
/// `.get`, `len()` and `isinstance` the check makes answers on this as on the whole document, refusals included.
fn project_source_ir(value: Py) -> Py {
    let Py::Dict(pairs) = value else {
        return hollow(value);
    };
    let projected = pairs
        .into_iter()
        .filter_map(|(key, value)| {
            let name = text(&key);
            let kept = match name.as_str() {
                "source" | "normalization_plan" | "document_identity" | "stage" => value,
                section if SOURCE_IR_SECTIONS.contains(&section) => hollow(value),
                "proof_ledger" => match value {
                    Py::Dict(ledger) => Py::Dict(
                        ledger
                            .into_iter()
                            .filter(|(field, _)| text(field) == "claims")
                            .map(|(field, claims)| match claims {
                                Py::List(items) => {
                                    (field, Py::List(items.into_iter().take(1).collect()))
                                }
                                _ => (field, Py::None),
                            })
                            .collect(),
                    ),
                    other => hollow(other),
                },
                _ => return None,
            };
            Some((key, kept))
        })
        .collect();
    Py::Dict(projected)
}

/// The files of one run, the authorities parsed once.
struct Repository {
    root: PathBuf,
    json: HashMap<PathBuf, Rc<Py>>,
    source_irs: HashMap<PathBuf, Rc<Py>>,
    digests: RefCell<HashMap<PathBuf, String>>,
}

impl Repository {
    fn new(root: &Path) -> Self {
        Repository {
            root: root.to_path_buf(),
            json: HashMap::new(),
            source_irs: HashMap::new(),
            digests: RefCell::new(HashMap::new()),
        }
    }

    fn path(&self, relative: &str) -> PathBuf {
        self.root.join(relative)
    }

    fn read_json(&mut self, path: &Path) -> Result<Rc<Py>, ReadError> {
        if let Some(value) = self.json.get(path) {
            return Ok(Rc::clone(value));
        }
        let value = Rc::new(parse_json_file(path)?);
        self.json.insert(path.to_path_buf(), Rc::clone(&value));
        Ok(value)
    }

    /// A stage artifact, parsed and not kept: the population's are 950 MB, and kept they peaked at 5.6 GB.
    fn read_artifact(&self, path: &Path) -> Checked<Py> {
        parse_json_file(path).map_err(|error| Uncaught(error.message().to_string()))
    }

    /// A SourceIR's projection (`project_source_ir`), parsed once a run: the self-test validates the population
    /// sixteen times, and its 89 MB of SourceIR parsed that often took five seconds.
    fn read_source_ir(&mut self, path: &Path) -> Checked<Rc<Py>> {
        if let Some(value) = self.source_irs.get(path) {
            return Ok(Rc::clone(value));
        }
        let value = Rc::new(project_source_ir(self.read_artifact(path)?));
        self.source_irs
            .insert(path.to_path_buf(), Rc::clone(&value));
        Ok(value)
    }

    /// A file's SHA-256, hashed once a run: the self-test validates the population sixteen times, and the dev
    /// profile's `sha2` is unoptimized.
    fn sha256(&self, path: &Path) -> Checked<String> {
        if let Some(digest) = self.digests.borrow().get(path) {
            return Ok(digest.clone());
        }
        let bytes = fs::read(path).map_err(|error| Uncaught(os_error(&error, path)))?;
        let digest: String = Sha256::digest(&bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        self.digests
            .borrow_mut()
            .insert(path.to_path_buf(), digest.clone());
        Ok(digest)
    }

    /// A read the script did not guard: any failure is a traceback there.
    fn read_json_unguarded(&mut self, path: &Path) -> Checked<Rc<Py>> {
        self.read_json(path)
            .map_err(|error| Uncaught(error.message().to_string()))
    }

    /// A read the script guarded with `except (OSError, json.JSONDecodeError)`.
    fn read_json_guarded(&mut self, path: &Path) -> Checked<Result<Rc<Py>, String>> {
        match self.read_json(path) {
            Ok(value) => Ok(Ok(value)),
            Err(ReadError::Os(message) | ReadError::Json(message)) => Ok(Err(message)),
            Err(ReadError::Decode(message)) => Err(Uncaught(message)),
        }
    }
}

fn is_file(path: &Path) -> bool {
    path.is_file()
}

// The population, as `csv.DictReader` with a tab delimiter reads it.

/// One population row: each header field's value, `None` where the row was short (`restval`).
type Row = BTreeMap<String, Option<String>>;

/// `csv.field_size_limit()`'s default: a longer field is a `csv.Error`, which the script did not catch.
const CSV_FIELD_LIMIT: usize = 131_072;

/// `csv.reader` under the `excel-tab` dialect: tab-delimited, `"`-quoted with doubled quotes, rows ending at an
/// unquoted `\n`, `\r\n` or `\r`.
fn csv_records(text: &str) -> Checked<Vec<Vec<String>>> {
    let records = csv_parse(text);
    if records
        .iter()
        .flatten()
        .any(|field| field.chars().count() > CSV_FIELD_LIMIT)
    {
        return Err(Uncaught(format!(
            "field larger than field limit ({CSV_FIELD_LIMIT})"
        )));
    }
    Ok(records)
}

fn csv_parse(text: &str) -> Vec<Vec<String>> {
    let mut records = Vec::new();
    let mut record: Vec<String> = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut at_field_start = true;
    let mut row_started = false;
    let mut characters = text.chars().peekable();
    while let Some(character) = characters.next() {
        if quoted {
            if character == '"' {
                if characters.peek() == Some(&'"') {
                    characters.next();
                    field.push('"');
                } else {
                    quoted = false;
                }
            } else {
                field.push(character);
            }
            continue;
        }
        match character {
            '"' if at_field_start => {
                quoted = true;
                at_field_start = false;
                row_started = true;
            }
            '\t' => {
                record.push(std::mem::take(&mut field));
                at_field_start = true;
                row_started = true;
            }
            '\n' | '\r' => {
                if character == '\r' && characters.peek() == Some(&'\n') {
                    characters.next();
                }
                if row_started || !field.is_empty() {
                    record.push(std::mem::take(&mut field));
                }
                records.push(std::mem::take(&mut record));
                at_field_start = true;
                row_started = false;
            }
            other => {
                field.push(other);
                at_field_start = false;
                row_started = true;
            }
        }
    }
    if row_started || !field.is_empty() || !record.is_empty() {
        record.push(field);
        records.push(record);
    }
    records
}

/// `read_population`: the rows, the `OSError` or `ValueError` text the entry point printed, or the `csv.Error` it
/// did not catch. The header is the first record, blank or not; the data rows skip blank records.
fn read_population(path: &Path) -> Checked<Result<Vec<Row>, String>> {
    let bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => return Ok(Err(os_error(&error, path))),
    };
    // The header is read from the first chunk, so a decoding error in a later one surfaces after its check.
    let (text, late_failure) = match decode_chunked(&bytes) {
        Ok(text) => (text, None),
        Err((_, 0, message)) => return Ok(Err(message)),
        Err((prefix, _, message)) => (prefix, Some(message)),
    };
    let mut records = csv_records(&text)?.into_iter();
    let header = records.next();
    let expected: Vec<String> = POPULATION_FIELDS
        .iter()
        .map(|field| (*field).to_string())
        .collect();
    if header.as_ref() != Some(&expected) {
        let found = header.map_or_else(|| "None".to_string(), |header| list_repr(&header));
        return Ok(Err(format!(
            "population header differs: {found} != {}",
            list_repr(&POPULATION_FIELDS)
        )));
    }
    if let Some(message) = late_failure {
        return Ok(Err(message));
    }
    Ok(Ok(records
        .filter(|record| !record.is_empty())
        .map(|record| {
            POPULATION_FIELDS
                .iter()
                .enumerate()
                .map(|(index, field)| ((*field).to_string(), record.get(index).cloned()))
                .collect()
        })
        .collect()))
}

/// `row.get(field)`: the value, or `None` where the row was short.
fn row_get<'a>(row: &'a Row, field: &str) -> Option<&'a str> {
    row.get(field).and_then(|value| value.as_deref())
}

/// `row.get(field, "")` used as a string: missing is `""`, a short row's `None` would raise.
fn row_text<'a>(row: &'a Row, field: &str) -> Checked<&'a str> {
    match row.get(field) {
        None => Ok(""),
        Some(Some(value)) => Ok(value),
        Some(None) => Err(Uncaught(
            "expected string or bytes-like object, got 'NoneType'".to_string(),
        )),
    }
}

/// `row[field]`, raising as Python's `KeyError` or a later `TypeError` would on a short row.
fn row_index<'a>(row: &'a Row, field: &str) -> Checked<&'a str> {
    match row.get(field) {
        Some(Some(value)) => Ok(value),
        _ => Err(Uncaught(format!("KeyError: {}", str_repr(field)))),
    }
}

/// `re.fullmatch(r"0|[1-9][0-9]*", value)`, then `int(value)`.
fn integer(row: &Row, field: &str, problems: &mut Vec<String>) -> Checked<i64> {
    let value = row_text(row, field)?;
    let valid = value == "0"
        || (value.starts_with(|character: char| ('1'..='9').contains(&character))
            && value.bytes().all(|byte| byte.is_ascii_digit()));
    if !valid {
        problems.push(format!(
            "{}: {field} is not an integer",
            row_get(row, "document_key").unwrap_or("<unknown>")
        ));
        return Ok(0);
    }
    value
        .parse()
        .map_err(|_| Uncaught(format!("integer too large: {value}")))
}

/// Python's `str.isspace()`: Unicode white space and the four information separators.
fn is_python_space(character: char) -> bool {
    character.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&character)
}

/// Python's `int(text)`: white space around, an optional sign, and digits with single underscores between them,
/// as an exact `int`. (Python also reads non-ASCII decimal digits; a population value with them is refused here.)
fn python_int(value: &str) -> Checked<Py> {
    let refused = || {
        Uncaught(format!(
            "invalid literal for int() with base 10: {}",
            str_repr(value)
        ))
    };
    let trimmed = value.trim_matches(is_python_space);
    let (negative, body) = match trimmed.strip_prefix('-') {
        Some(rest) => (true, rest),
        None => (false, trimmed.strip_prefix('+').unwrap_or(trimmed)),
    };
    if body.is_empty()
        || body.starts_with('_')
        || body.ends_with('_')
        || body.contains("__")
        || !body
            .chars()
            .all(|character| character.is_ascii_digit() || character == '_')
    {
        return Err(refused());
    }
    let digits: String = body.chars().filter(|character| *character != '_').collect();
    if digits.len() > 4300 {
        return Err(Uncaught(
            "Exceeds the limit (4300 digits) for integer string conversion".to_string(),
        ));
    }
    let digits = digits.trim_start_matches('0');
    Ok(Py::Int(match (digits.is_empty(), negative) {
        (true, _) => "0".to_string(),
        (false, true) => format!("-{digits}"),
        (false, false) => digits.to_string(),
    }))
}

fn expected_layout(pages: i64) -> &'static str {
    if pages <= 32 {
        "compact"
    } else if pages <= 96 {
        "medium"
    } else {
        "long"
    }
}

// The checks.

/// `unique_ids`: each row's string `key`, reporting non-rows and duplicates.
fn unique_ids(
    rows: Option<&Py>,
    key: &str,
    label: &str,
    problems: &mut Vec<String>,
) -> BTreeSet<String> {
    let Some(Py::List(rows)) = rows else {
        problems.push(format!("{label} must be an array"));
        return BTreeSet::new();
    };
    let mut values = Vec::new();
    for (index, row) in rows.iter().enumerate() {
        match pyjson::get(row, key) {
            Some(Py::Str(value)) if matches!(row, Py::Dict(_)) => values.push(text(value)),
            _ => problems.push(format!("{label}[{index}] lacks string {key}")),
        }
    }
    let distinct: BTreeSet<String> = values.iter().cloned().collect();
    if distinct.len() != values.len() {
        problems.push(format!("{label} contains duplicate {key} values"));
    }
    distinct
}

fn check_file_identity(
    repository: &Repository,
    declaration: Option<&Py>,
    label: &str,
    problems: &mut Vec<String>,
) -> Checked<()> {
    let Some(declaration @ Py::Dict(_)) = declaration else {
        problems.push(format!("frozen census {label} must be an object"));
        return Ok(());
    };
    let Some(relative) = safe_relative_path(pyjson::get(declaration, "path")) else {
        problems.push(format!(
            "frozen census {label} path is not repository-relative"
        ));
        return Ok(());
    };
    let path = repository.path(&relative.as_posix());
    if !is_file(&path) {
        problems.push(format!(
            "frozen census {label} is missing: {}",
            relative.as_posix()
        ));
    } else if !is_str(pyjson::get(declaration, "sha256"))
        || !eq_str(
            pyjson::get(declaration, "sha256"),
            &repository.sha256(&path)?,
        )
    {
        problems.push(format!(
            "frozen census {label} SHA-256 differs: {}",
            relative.as_posix()
        ));
    }
    Ok(())
}

fn validate_reviewed_recipes(
    repository: &mut Repository,
    rows: &[Row],
    problems: &mut Vec<String>,
) -> Checked<()> {
    let manifest = match repository.read_json_guarded(&repository.path(RECIPE_MANIFEST_PATH))? {
        Ok(manifest) => manifest,
        Err(error) => {
            problems.push(format!("reviewed recipe manifest is unavailable: {error}"));
            return Ok(());
        }
    };
    if !matches!(*manifest, Py::Dict(_)) {
        problems.push("reviewed recipe manifest must be an object".to_string());
        return Ok(());
    }
    if !eq(pyjson::get(&manifest, "schema_version"), &int(1)) {
        problems.push("reviewed recipe manifest schema_version must be 1".to_string());
    }
    if !eq_str(
        pyjson::get(&manifest, "owner"),
        "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii",
    ) {
        problems.push("reviewed recipe manifest owner differs from .f.ii parent".to_string());
    }
    let recipes = pyjson::get(&manifest, "recipes");
    let recipe_ids = unique_ids(recipes, "recipe_id", "reviewed recipes", problems);
    if recipe_ids.is_empty() {
        problems.push("reviewed recipe manifest is empty".to_string());
    }
    let Some(Py::List(recipes)) = recipes else {
        return Ok(());
    };
    let relations: Vec<Py> = recipes
        .iter()
        .filter_map(|row| match pyjson::get(row, "relation") {
            Some(relation @ Py::Str(_)) if matches!(row, Py::Dict(_)) => Some(relation.clone()),
            _ => None,
        })
        .collect();
    if !set_is(&relations, &EXPECTED_REVIEWED_RECIPE_RELATIONS)? {
        problems.push("reviewed recipe relations differ from the released .f.ii set".to_string());
    }
    // `{row.get("normalized_markdown_path"): row for row in rows}`: the last row wins a repeated path.
    let mut population_by_markdown: HashMap<Option<&str>, &Row> = HashMap::new();
    for row in rows {
        population_by_markdown.insert(row_get(row, "normalized_markdown_path"), row);
    }
    for (index, declaration) in recipes.iter().enumerate() {
        if !matches!(declaration, Py::Dict(_)) {
            problems.push(format!("reviewed recipes[{index}] must be an object"));
            continue;
        }
        let recipe_id = match pyjson::get(declaration, "recipe_id") {
            Some(value) => pyjson::str_of(value),
            None => format!("index-{index}"),
        };
        let relative = safe_relative_path(pyjson::get(declaration, "path")).filter(|path| {
            path.is_relative_to(&PurePath::new(
                "doctrine/production_genericity/reviewed_recipes",
            ))
        });
        let Some(relative) = relative else {
            problems.push(format!("reviewed recipe {recipe_id} path is unsafe"));
            continue;
        };
        let recipe_path = repository.path(&relative.as_posix());
        if !is_file(&recipe_path) {
            problems.push(format!(
                "reviewed recipe {recipe_id} is missing: {}",
                relative.as_posix()
            ));
            continue;
        }
        if !eq_str(
            pyjson::get(declaration, "sha256"),
            &repository.sha256(&recipe_path)?,
        ) {
            problems.push(format!("reviewed recipe {recipe_id} SHA-256 differs"));
        }
        let recipe = match repository.read_json_guarded(&recipe_path)? {
            Ok(recipe) => recipe,
            Err(error) => {
                problems.push(format!(
                    "reviewed recipe {recipe_id} is invalid JSON: {error}"
                ));
                continue;
            }
        };
        if !matches!(*recipe, Py::Dict(_)) {
            problems.push(format!("reviewed recipe {recipe_id} must be an object"));
            continue;
        }
        let mut expected_keys = vec![
            "schema_version",
            "recipe_id",
            "relation",
            "source_authority",
            "source_sha256",
            "review_status",
            "changed_spans",
            "preserved_conclusions",
            "unaffected_complement",
            "unmeasurable_source_surfaces",
        ];
        let negative = eq_str(pyjson::get(&recipe, "relation"), "semantic_negative");
        if negative {
            expected_keys.extend([
                "semantic_negative_kind",
                "required_deltas",
                "dependent_proof_deltas",
            ]);
        }
        if !key_set_is(&recipe, &expected_keys) {
            problems.push(format!(
                "reviewed recipe {recipe_id} has an open or incomplete schema"
            ));
        }
        for field in ["recipe_id", "relation", "source_authority", "source_sha256"] {
            if !pyjson::py_eq(
                pyjson::get(&recipe, field).unwrap_or(&Py::None),
                pyjson::get(declaration, field).unwrap_or(&Py::None),
            ) {
                problems.push(format!(
                    "reviewed recipe {recipe_id} differs from manifest {field}"
                ));
            }
        }
        if !eq(pyjson::get(&recipe, "schema_version"), &int(1))
            || !eq_str(pyjson::get(&recipe, "review_status"), "approved")
        {
            problems.push(format!(
                "reviewed recipe {recipe_id} is not approved schema 1"
            ));
        }
        if !eq_str(
            pyjson::get(declaration, "review_role"),
            "reviewed_calibration",
        ) {
            problems.push(format!(
                "reviewed recipe {recipe_id} is not calibration-owned"
            ));
        }
        let authority = pyjson::get(declaration, "source_authority");
        let source_row = match authority {
            None | Some(Py::None) => population_by_markdown.get(&None).copied(),
            Some(Py::Str(characters)) => population_by_markdown
                .get(&Some(text(characters).as_str()))
                .copied(),
            Some(Py::List(_) | Py::Dict(_)) => {
                return Err(Uncaught(format!(
                    "unhashable type: '{}'",
                    type_name(authority.expect("present"))
                )));
            }
            Some(_) => None,
        };
        match source_row {
            Some(row) if row_get(row, "review_role") == Some("reviewed_calibration") => {
                let digest = row_get(row, "normalized_markdown_sha256");
                let matches = match pyjson::get(declaration, "source_sha256") {
                    Some(Py::Str(characters)) => digest == Some(text(characters).as_str()),
                    None | Some(Py::None) => digest.is_none(),
                    Some(_) => false,
                };
                if !matches {
                    problems.push(format!(
                        "reviewed recipe {recipe_id} source digest differs from population"
                    ));
                }
            }
            _ => problems.push(format!(
                "reviewed recipe {recipe_id} source is not frozen calibration"
            )),
        }
        let source_relative = safe_relative_path(pyjson::get(&recipe, "source_authority"));
        let source_path = source_relative
            .as_ref()
            .map(|relative| repository.path(&relative.as_posix()));
        let mut source_text: Option<String> = None;
        match &source_path {
            Some(path) if is_file(path) => {
                if !eq_str(
                    pyjson::get(&recipe, "source_sha256"),
                    &repository.sha256(path)?,
                ) {
                    problems.push(format!(
                        "reviewed recipe {recipe_id} source SHA-256 differs"
                    ));
                } else {
                    source_text = Some(
                        read_text(path).map_err(|error| Uncaught(error.message().to_string()))?,
                    );
                }
            }
            _ => problems.push(format!(
                "reviewed recipe {recipe_id} source is missing or unsafe"
            )),
        }
        let changes = pyjson::get(&recipe, "changed_spans");
        let change_ids = unique_ids(
            changes,
            "change_id",
            &format!("recipe {recipe_id} changes"),
            problems,
        );
        let Some(Py::List(changes)) = changes.filter(|_| !change_ids.is_empty()) else {
            problems.push(format!(
                "reviewed recipe {recipe_id} has no exhaustive changed spans"
            ));
            continue;
        };
        let mut kinds: BTreeSet<String> = BTreeSet::new();
        for (change_index, change) in changes.iter().enumerate() {
            if !matches!(change, Py::Dict(_)) {
                problems.push(format!(
                    "recipe {recipe_id} change {change_index} must be an object"
                ));
                continue;
            }
            if !key_set_is(
                change,
                &[
                    "change_id",
                    "kind",
                    "exact_before",
                    "exact_after",
                    "expected_occurrences",
                    "allowed_provenance_fields",
                    "allow_source_bound_identifier_projection",
                ],
            ) {
                problems.push(format!(
                    "recipe {recipe_id} change {change_index} schema is open"
                ));
            }
            if let Some(kind) = as_str(pyjson::get(change, "kind")) {
                kinds.insert(kind);
            }
            let before =
                as_str(pyjson::get(change, "exact_before")).filter(|value| !value.is_empty());
            let after =
                as_str(pyjson::get(change, "exact_after")).filter(|value| !value.is_empty());
            match (&before, &after) {
                (Some(before), Some(after))
                    if before != after
                        && eq(pyjson::get(change, "expected_occurrences"), &int(1)) =>
                {
                    if let Some(source) = &source_text
                        && (source.matches(before.as_str()).count() != 1
                            || source.matches(after.as_str()).count() != 0)
                    {
                        problems.push(format!(
                            "recipe {recipe_id} change {change_index} is ambiguous or stale"
                        ));
                    }
                }
                _ => problems.push(format!(
                    "recipe {recipe_id} change {change_index} is not one exact delta"
                )),
            }
            let fields = pyjson::get(change, "allowed_provenance_fields");
            let unsafe_fields = match fields {
                Some(Py::List(fields)) if !fields.is_empty() => {
                    has_duplicates(fields)?
                        || !py_set(fields)?.iter().all(|field| {
                            SAFE_REVIEWED_PROVENANCE_FIELDS
                                .iter()
                                .any(|safe| pyjson::py_eq(field, &s(safe)))
                        })
                }
                _ => true,
            };
            if unsafe_fields {
                problems.push(format!(
                    "recipe {recipe_id} change {change_index} opens unsafe fields"
                ));
            }
            if !matches!(
                pyjson::get(change, "allow_source_bound_identifier_projection"),
                Some(Py::Bool(true))
            ) {
                problems.push(format!(
                    "recipe {recipe_id} change {change_index} lacks closed identifier projection"
                ));
            }
        }
        let expected_kinds: BTreeSet<String> = match pyjson::get(&recipe, "relation") {
            Some(Py::Str(relation)) => EXPECTED_REVIEWED_CHANGE_KINDS
                .iter()
                .find(|(name, _)| *name == text(relation))
                .map(|(_, kinds)| kinds.iter().map(|kind| (*kind).to_string()).collect())
                .unwrap_or_default(),
            Some(Py::List(_) | Py::Dict(_)) => {
                return Err(Uncaught("unhashable type".to_string()));
            }
            _ => BTreeSet::new(),
        };
        if kinds != expected_kinds {
            problems.push(format!(
                "reviewed recipe {recipe_id} change-kind coverage differs"
            ));
        }
        let conclusions = pyjson::get(&recipe, "preserved_conclusions");
        let conclusion_ids = unique_ids(
            conclusions,
            "conclusion_id",
            &format!("recipe {recipe_id} conclusions"),
            problems,
        );
        match conclusions {
            Some(Py::List(conclusions)) if !conclusion_ids.is_empty() => {
                for (conclusion_index, conclusion) in conclusions.iter().enumerate() {
                    if !matches!(conclusion, Py::Dict(_))
                        || !key_set_is(
                            conclusion,
                            &[
                                "conclusion_id",
                                "stage",
                                "baseline_pointer",
                                "transformed_pointer",
                                "baseline_value",
                                "transformed_value",
                            ],
                        )
                    {
                        problems.push(format!(
                            "recipe {recipe_id} conclusion {conclusion_index} schema is open"
                        ));
                    } else if !stage_known(pyjson::get(conclusion, "stage"))? {
                        problems.push(format!(
                            "recipe {recipe_id} conclusion {conclusion_index} stage is invalid"
                        ));
                    }
                }
            }
            _ => problems.push(format!(
                "reviewed recipe {recipe_id} has no preserved conclusions"
            )),
        }
        if negative {
            validate_semantic_negative_recipe(&recipe_id, &recipe, problems)?;
        }
        if !eq_str(
            pyjson::get(&recipe, "unaffected_complement"),
            REVIEWED_COMPLEMENT,
        ) {
            problems.push(format!(
                "reviewed recipe {recipe_id} complement is not closed"
            ));
        }
        let exclusions_ok = match pyjson::get(&recipe, "unmeasurable_source_surfaces") {
            Some(Py::List(exclusions)) => {
                set_is(exclusions, &RICH_CAPTURE_EXCLUSIONS)? && !has_duplicates(exclusions)?
            }
            _ => false,
        };
        if !exclusions_ok {
            problems.push(format!(
                "reviewed recipe {recipe_id} rich-capture exclusions differ"
            ));
        }
    }
    Ok(())
}

/// `value not in EXPECTED_STAGES`'s negation, refusing an unhashable value.
fn stage_known(value: Option<&Py>) -> Checked<bool> {
    match value {
        Some(Py::List(_) | Py::Dict(_)) => Err(Uncaught("unhashable type".to_string())),
        Some(value) => Ok(EXPECTED_STAGES
            .iter()
            .any(|stage| pyjson::py_eq(value, &s(stage)))),
        None => Ok(false),
    }
}

fn in_set(value: Option<&Py>, set: &[&str]) -> Checked<bool> {
    match value {
        Some(Py::List(_) | Py::Dict(_)) => Err(Uncaught("unhashable type".to_string())),
        Some(value) => Ok(set.iter().any(|item| pyjson::py_eq(value, &s(item)))),
        None => Ok(false),
    }
}

fn validate_semantic_negative_recipe(
    recipe_id: &str,
    recipe: &Py,
    problems: &mut Vec<String>,
) -> Checked<()> {
    if !eq_str(
        pyjson::get(recipe, "semantic_negative_kind"),
        "timing_grammar_admission",
    ) {
        problems.push(format!("semantic-negative recipe {recipe_id} kind differs"));
    }
    let deltas = pyjson::get(recipe, "required_deltas");
    let delta_ids = unique_ids(
        deltas,
        "delta_id",
        &format!("semantic-negative recipe {recipe_id} deltas"),
        problems,
    );
    match deltas {
        Some(Py::List(deltas)) if !delta_ids.is_empty() => {
            for (index, delta) in deltas.iter().enumerate() {
                if !matches!(delta, Py::Dict(_))
                    || !key_set_is(
                        delta,
                        &[
                            "delta_id",
                            "stage",
                            "kind",
                            "baseline_pointer",
                            "transformed_pointer",
                            "baseline_value",
                            "transformed_value",
                        ],
                    )
                {
                    problems.push(format!(
                        "semantic-negative recipe {recipe_id} delta {index} schema is open"
                    ));
                    continue;
                }
                let baseline_pointer = pyjson::get(delta, "baseline_pointer");
                let transformed_pointer = pyjson::get(delta, "transformed_pointer");
                let pointers_ok = [baseline_pointer, transformed_pointer]
                    .into_iter()
                    .filter(|pointer| !is_none(*pointer))
                    .all(|pointer| as_str(pointer).is_some_and(|pointer| pointer.starts_with('/')));
                if !in_set(pyjson::get(delta, "stage"), &["semantic_ir", "intent_ir"])?
                    || !pointers_ok
                {
                    problems.push(format!(
                        "semantic-negative recipe {recipe_id} delta {index} path/stage differs"
                    ));
                }
                let baseline_value = pyjson::get(delta, "baseline_value");
                let transformed_value = pyjson::get(delta, "transformed_value");
                let kind = pyjson::get(delta, "kind");
                let valid_shape = if eq_str(kind, "added") {
                    is_none(baseline_pointer)
                        && is_none(baseline_value)
                        && is_str(transformed_pointer)
                        && !is_none(transformed_value)
                } else if eq_str(kind, "changed") {
                    is_str(baseline_pointer)
                        && !is_none(baseline_value)
                        && is_str(transformed_pointer)
                        && !is_none(transformed_value)
                        && !pyjson::py_eq(
                            baseline_value.unwrap_or(&Py::None),
                            transformed_value.unwrap_or(&Py::None),
                        )
                } else if eq_str(kind, "removed") {
                    is_str(baseline_pointer)
                        && !is_none(baseline_value)
                        && is_none(transformed_pointer)
                        && is_none(transformed_value)
                } else {
                    false
                };
                if !valid_shape {
                    problems.push(format!(
                        "semantic-negative recipe {recipe_id} delta {index} shape differs"
                    ));
                }
            }
        }
        _ => problems.push(format!(
            "semantic-negative recipe {recipe_id} has no required delta"
        )),
    }
    let proof_deltas = pyjson::get(recipe, "dependent_proof_deltas");
    let proof_ids = unique_ids(
        proof_deltas,
        "delta_id",
        &format!("semantic-negative recipe {recipe_id} proof deltas"),
        problems,
    );
    let Some(Py::List(proof_deltas)) = proof_deltas.filter(|_| !proof_ids.is_empty()) else {
        problems.push(format!(
            "semantic-negative recipe {recipe_id} has no proof delta"
        ));
        return Ok(());
    };
    for (index, proof) in proof_deltas.iter().enumerate() {
        if !matches!(proof, Py::Dict(_))
            || !key_set_is(proof, &["delta_id", "stages", "rule_id", "address"])
        {
            problems.push(format!(
                "semantic-negative recipe {recipe_id} proof delta {index} schema is open"
            ));
            continue;
        }
        let address = pyjson::get(proof, "address");
        let Some(address @ Py::Dict(_)) = address.filter(|address| {
            key_set_is(
                address,
                &["stage", "surface", "stable_record_key", "field_path"],
            )
        }) else {
            problems.push(format!(
                "semantic-negative recipe {recipe_id} proof delta {index} address differs"
            ));
            continue;
        };
        let expected_stages: Option<&[&str]> = match pyjson::get(address, "stage") {
            Some(Py::List(_) | Py::Dict(_)) => return Err(Uncaught("unhashable type".to_string())),
            stage if eq_str(stage, "evidence_ir") => {
                Some(&["evidence_ir", "semantic_ir", "intent_ir", "isf_adapter"])
            }
            stage if eq_str(stage, "semantic_ir") => {
                Some(&["semantic_ir", "intent_ir", "isf_adapter"])
            }
            stage if eq_str(stage, "intent_ir") => Some(&["intent_ir", "isf_adapter"]),
            _ => None,
        };
        let nonempty = |value: Option<&Py>| as_str(value).is_some_and(|value| !value.is_empty());
        let complete = match (pyjson::get(proof, "stages"), expected_stages) {
            (Some(Py::List(stages)), Some(expected)) => {
                set_is(stages, expected)?
                    && !has_duplicates(stages)?
                    && nonempty(pyjson::get(proof, "rule_id"))
                    && nonempty(pyjson::get(address, "surface"))
                    && nonempty(pyjson::get(address, "stable_record_key"))
            }
            _ => false,
        };
        if !complete {
            problems.push(format!(
                "semantic-negative recipe {recipe_id} proof delta {index} is incomplete"
            ));
        }
    }
    Ok(())
}

fn dict_of(pairs: &[(&str, &str)]) -> Py {
    Py::Dict(
        pairs
            .iter()
            .map(|(key, value)| (pyjson::py_str(key), s(value)))
            .collect(),
    )
}

fn validate_negative_sensitivity_matrix(
    repository: &mut Repository,
    problems: &mut Vec<String>,
) -> Checked<()> {
    let matrix = match repository.read_json_guarded(&repository.path(NEGATIVE_MATRIX_PATH))? {
        Ok(matrix) => matrix,
        Err(error) => {
            problems.push(format!("semantic-negative matrix is unavailable: {error}"));
            return Ok(());
        }
    };
    if !matches!(*matrix, Py::Dict(_))
        || !key_set_is(
            &matrix,
            &[
                "schema_version",
                "owner",
                "unaffected_complement",
                "controls",
                "attempt_dispositions",
            ],
        )
    {
        problems.push("semantic-negative matrix schema is open or incomplete".to_string());
        return Ok(());
    }
    if !eq(pyjson::get(&matrix, "schema_version"), &int(1))
        || !eq_str(
            pyjson::get(&matrix, "owner"),
            "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.c",
        )
        || !eq_str(
            pyjson::get(&matrix, "unaffected_complement"),
            REVIEWED_COMPLEMENT,
        )
    {
        problems.push("semantic-negative matrix identity or complement differs".to_string());
    }
    let controls = pyjson::get(&matrix, "controls");
    let control_ids = unique_ids(
        controls,
        "control_id",
        "semantic-negative controls",
        problems,
    );
    let Some(Py::List(controls)) = controls.filter(|_| !control_ids.is_empty()) else {
        problems.push("semantic-negative matrix has no controls".to_string());
        return Ok(());
    };
    let kinds: Vec<Py> = controls
        .iter()
        .filter_map(|control| match pyjson::get(control, "kind") {
            Some(kind @ Py::Str(_)) if matches!(control, Py::Dict(_)) => Some(kind.clone()),
            _ => None,
        })
        .collect();
    if !set_is(&kinds, &EXPECTED_NEGATIVE_CONTROL_KINDS)? {
        problems.push("semantic-negative matrix control-kind coverage differs".to_string());
    }
    for (index, control) in controls.iter().enumerate() {
        if !matches!(control, Py::Dict(_))
            || !key_set_is(
                control,
                &[
                    "control_id",
                    "kind",
                    "stage",
                    "operation",
                    "path",
                    "baseline_value",
                    "transformed_value",
                    "expected_state",
                    "expected_failure_id",
                ],
            )
        {
            problems.push(format!("semantic-negative control {index} schema is open"));
            continue;
        }
        let get = |key: &str| pyjson::get(control, key);
        if !stage_known(get("stage"))? {
            problems.push(format!("semantic-negative control {index} stage differs"));
        }
        let valid = if eq_str(get("kind"), "disabled_stage") {
            eq_str(get("operation"), "disable_stage")
                && is_none(get("path"))
                && is_none(get("baseline_value"))
                && is_none(get("transformed_value"))
                && eq_str(get("expected_state"), "invalid")
                && eq_str(get("expected_failure_id"), "partial_or_escaped_run")
        } else {
            let operation = get("operation");
            let remove = eq_str(operation, "remove");
            let replace = eq_str(operation, "replace");
            in_set(operation, &["remove", "replace"])?
                && as_str(get("path")).is_some_and(|path| path.starts_with('/'))
                && !is_none(get("baseline_value"))
                && ((remove && is_none(get("transformed_value")))
                    || (replace && !is_none(get("transformed_value"))))
                && eq_str(get("expected_state"), "fail")
                && eq_str(get("expected_failure_id"), "undeclared_semantic_delta")
        };
        if !valid {
            problems.push(format!(
                "semantic-negative control {index} disposition differs"
            ));
        }
    }
    if !eq(
        pyjson::get(&matrix, "attempt_dispositions"),
        &dict_of(&EXPECTED_ATTEMPT_DISPOSITIONS),
    ) {
        problems.push("semantic-negative attempt dispositions differ".to_string());
    }
    Ok(())
}

/// The Wilson score interval at 95%, in parts per million, rounded as Python's `round` rounds: half to even.
fn wilson_parts_per_million(passes: i64, completed: i64) -> Option<(i64, i64, i64)> {
    if completed == 0 {
        return None;
    }
    let completed = completed as f64;
    let proportion = passes as f64 / completed;
    let z_value = 1.959963984540054_f64;
    let z_squared = z_value * z_value;
    let denominator = 1.0 + z_squared / completed;
    let center = (proportion + z_squared / (2.0 * completed)) / denominator;
    let margin = z_value
        * (proportion * (1.0 - proportion) / completed + z_squared / (4.0 * completed * completed))
            .sqrt()
        / denominator;
    let scaled = |value: f64| (value.clamp(0.0, 1.0) * 1_000_000.0).round_ties_even() as i64;
    Some((
        scaled(proportion),
        scaled(center - margin),
        scaled(center + margin),
    ))
}

/// Every string anywhere in the report that is absolute or names `/Volumes/`, by JSON-pointer-like path.
fn find_absolute(value: &Py, path: &str, found: &mut Vec<String>) {
    match value {
        Py::Dict(pairs) => {
            for (field, child) in pairs {
                find_absolute(child, &format!("{path}/{}", text(field)), found);
            }
        }
        Py::List(children) => {
            for (index, child) in children.iter().enumerate() {
                find_absolute(child, &format!("{path}/{index}"), found);
            }
        }
        Py::Str(characters) => {
            let value = text(characters);
            if value.starts_with('/') || value.contains("/Volumes/") {
                found.push(path.to_string());
            }
        }
        _ => {}
    }
}

#[allow(clippy::too_many_lines)]
fn validate_held_out_evidence(
    repository: &mut Repository,
    contract: &Py,
    rows: &[Row],
    problems: &mut Vec<String>,
    report_override: Option<&Py>,
) -> Checked<()> {
    let loaded;
    let report: &Py = match report_override {
        Some(report) => report,
        None => match repository.read_json_guarded(&repository.path(HELD_OUT_EVIDENCE_PATH))? {
            Ok(report) => {
                loaded = report;
                &loaded
            }
            Err(error) => {
                problems.push(format!("held-out evidence is unreadable: {error}"));
                return Ok(());
            }
        },
    };
    if !matches!(report, Py::Dict(_)) {
        problems.push("held-out evidence must be an object".to_string());
        return Ok(());
    }
    let get = |key: &str| pyjson::get(report, key);
    if !eq(get("schema_version"), &int(2)) {
        problems.push("held-out evidence schema_version must be 2".to_string());
    }
    if !eq_str(get("owner"), "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii") {
        problems.push("held-out evidence owner differs".to_string());
    }
    let prior_memory = attr(
        attr_or_empty(attr_or_empty(contract, "frozen_census")?, "prior_memory")?,
        "sha256",
    )?
    .cloned()
    .unwrap_or(Py::None);
    let expected_identity: Vec<(&str, Py)> = vec![
        ("contract_path", s(CONTRACT_PATH)),
        (
            "contract_sha256",
            s(&repository.sha256(&repository.path(CONTRACT_PATH))?),
        ),
        ("population_path", s(POPULATION_PATH)),
        (
            "population_sha256",
            s(&repository.sha256(&repository.path(POPULATION_PATH))?),
        ),
        ("prior_memory_sha256", prior_memory),
        (
            "tool_sha256",
            s(&repository.sha256(&repository.path(BEHAVIORAL_TOOL_PATH))?),
        ),
    ];
    for (field, expected) in &expected_identity {
        if !eq(get(field), expected) {
            problems.push(format!(
                "held-out evidence {field} differs: {} != {}",
                py_repr(get(field)),
                pyjson::repr(expected)
            ));
        }
    }
    let retained_path_ok = as_str(get("retained_evidence_path")).is_some_and(|path| {
        let pure = PurePath::new(&path);
        !pure.absolute()
            && !pure.climbs()
            && path.starts_with(".project-data/tmp/")
            && path.ends_with("/behavioral_holdout_evidence.json")
    });
    if !retained_path_ok {
        problems.push(
            "held-out retained evidence path is not safe and repository-relative".to_string(),
        );
    }
    if !is_hex(get("retained_evidence_sha256"), 64) {
        problems.push("held-out retained evidence digest is invalid".to_string());
    }
    if !is_hex(get("retained_tool_sha256"), 64) {
        problems.push("held-out retained tool digest is invalid".to_string());
    }
    if !is_hex(get("production_revision"), 40) {
        problems.push("held-out production revision is not a full Git id".to_string());
    }
    let leakage_rule = attr(attr_or_empty(contract, "held_out_policy")?, "leakage_rule")?;
    if !eq(get("leakage_boundary"), leakage_rule.unwrap_or(&Py::None)) {
        problems.push("held-out evidence leakage boundary differs".to_string());
    }
    let expected_relations = Py::List(
        HELD_OUT_RELATIONS
            .iter()
            .map(|relation| s(relation))
            .collect(),
    );
    if !eq(get("eligible_relations"), &expected_relations) {
        problems.push("held-out evidence eligible relations differ".to_string());
    }
    if !matches!(get("final_signoff_deferred"), Some(Py::Bool(true))) {
        problems.push("held-out evidence prematurely claims final signoff".to_string());
    }

    let calibration: Vec<&Row> = rows
        .iter()
        .filter(|row| row_get(row, "review_role") == Some("reviewed_calibration"))
        .collect();
    let prospective: Vec<&Row> = rows
        .iter()
        .filter(|row| row_get(row, "review_role") == Some("prospective_holdout"))
        .collect();
    let mut calibration_keys: Vec<String> = calibration
        .iter()
        .map(|row| row_index(row, "document_key").map(str::to_string))
        .collect::<Checked<_>>()?;
    calibration_keys.sort();
    let mut prospective_keys: Vec<String> = prospective
        .iter()
        .map(|row| row_index(row, "document_key").map(str::to_string))
        .collect::<Checked<_>>()?;
    prospective_keys.sort();
    let string_list = |keys: &[String]| Py::List(keys.iter().map(|key| s(key)).collect());
    match get("split") {
        Some(split @ Py::Dict(_)) => {
            let split_checks: Vec<(&str, Py)> = vec![
                (
                    "selection_boundary_commit",
                    pyjson::get(contract, "selection_boundary_commit")
                        .cloned()
                        .unwrap_or(Py::None),
                ),
                ("calibration_document_keys", string_list(&calibration_keys)),
                ("prospective_document_keys", string_list(&prospective_keys)),
                ("overlapping_document_keys", Py::List(Vec::new())),
                ("overlapping_source_sha256", Py::List(Vec::new())),
                (
                    "overlapping_normalized_markdown_sha256",
                    Py::List(Vec::new()),
                ),
                ("identity_disjoint", Py::Bool(true)),
            ];
            for (field, expected) in &split_checks {
                if !eq(pyjson::get(split, field), expected) {
                    problems.push(format!("held-out split {field} differs"));
                }
            }
        }
        _ => problems.push("held-out evidence split must be an object".to_string()),
    }

    let calibration_vendors: BTreeSet<&str> = calibration
        .iter()
        .map(|row| row_index(row, "vendor"))
        .collect::<Checked<_>>()?;
    let calibration_families: BTreeSet<&str> = calibration
        .iter()
        .map(|row| row_index(row, "family"))
        .collect::<Checked<_>>()?;
    let mut prospective_by_key: BTreeMap<&str, &Row> = BTreeMap::new();
    for row in &prospective {
        prospective_by_key.insert(row_index(row, "document_key")?, row);
    }
    let documents: &[Py] = match get("documents") {
        Some(Py::List(documents)) => documents,
        _ => {
            problems.push("held-out documents must be an array".to_string());
            &[]
        }
    };
    // `document_by_key` keeps insertion order with a repeated key's last row, as a Python dict does.
    let mut document_order: Vec<String> = Vec::new();
    let mut document_by_key: HashMap<String, &Py> = HashMap::new();
    for document in documents {
        let Some(key) = as_str(pyjson::get(document, "document_key"))
            .filter(|_| matches!(document, Py::Dict(_)))
        else {
            problems.push("held-out document row is malformed".to_string());
            continue;
        };
        if document_by_key.contains_key(&key) {
            problems.push(format!("held-out document is duplicated: {key}"));
        } else {
            document_order.push(key.clone());
        }
        document_by_key.insert(key.clone(), document);
        let Some(population) = prospective_by_key.get(key.as_str()) else {
            problems.push(format!(
                "held-out evidence includes a non-prospective document: {key}"
            ));
            continue;
        };
        for field in [
            "source_origin",
            "source_locator",
            "source_sha256",
            "normalized_markdown_path",
            "normalized_markdown_sha256",
            "vendor",
            "family",
            "category",
            "layout",
        ] {
            let expected = row_get(population, field).map_or(Py::None, s);
            if !eq(pyjson::get(document, field), &expected) {
                problems.push(format!("{key}: held-out document {field} differs"));
            }
        }
        for field in ["text_semantic_records", "text_intent_records"] {
            let expected = python_int(row_index(population, field)?)?;
            if !eq(pyjson::get(document, field), &expected) {
                problems.push(format!("{key}: held-out document {field} differs"));
            }
        }
        let vendor_novel = !calibration_vendors.contains(row_index(population, "vendor")?);
        if !eq(
            pyjson::get(document, "vendor_novel"),
            &Py::Bool(vendor_novel),
        ) {
            problems.push(format!("{key}: held-out vendor novelty differs"));
        }
        let family_novel = !calibration_families.contains(row_index(population, "family")?);
        if !eq(
            pyjson::get(document, "family_novel"),
            &Py::Bool(family_novel),
        ) {
            problems.push(format!("{key}: held-out family novelty differs"));
        }
    }
    let mut sorted_documents: Vec<String> = document_by_key.keys().cloned().collect();
    sorted_documents.sort();
    if sorted_documents != prospective_keys {
        problems
            .push("held-out evidence document set differs from prospective population".to_string());
    }

    let attempts: &[Py] = match get("attempts") {
        Some(Py::List(attempts)) => attempts,
        _ => {
            problems.push("held-out attempts must be an array".to_string());
            &[]
        }
    };
    let mut attempt_by_pair: BTreeMap<(String, String), &Py> = BTreeMap::new();
    for attempt in attempts {
        if !matches!(attempt, Py::Dict(_)) {
            problems.push("held-out attempt row is malformed".to_string());
            continue;
        }
        let key_value = pyjson::get(attempt, "document_key");
        let relation_value = pyjson::get(attempt, "relation");
        let pair_repr = tuple_repr(&[key_value, relation_value]);
        // `key not in prospective_by_key or relation not in …`: the relation is only hashed for a known key.
        if matches!(key_value, Some(Py::List(_) | Py::Dict(_))) {
            return Err(Uncaught("unhashable type".to_string()));
        }
        let Some(key) =
            as_str(key_value).filter(|key| prospective_by_key.contains_key(key.as_str()))
        else {
            problems.push(format!(
                "held-out attempt identity is outside the matrix: {pair_repr}"
            ));
            continue;
        };
        if matches!(relation_value, Some(Py::List(_) | Py::Dict(_))) {
            return Err(Uncaught("unhashable type".to_string()));
        }
        let Some(relation) = as_str(relation_value)
            .filter(|relation| HELD_OUT_RELATIONS.contains(&relation.as_str()))
        else {
            problems.push(format!(
                "held-out attempt identity is outside the matrix: {pair_repr}"
            ));
            continue;
        };
        let pair = (key.clone(), relation.clone());
        if attempt_by_pair.contains_key(&pair) {
            problems.push(format!("held-out attempt is duplicated: {pair_repr}"));
        }
        attempt_by_pair.insert(pair, attempt);
        let field = |name: &str| pyjson::get(attempt, name);
        let expected_plane = if relation == "symbol_alpha" {
            "normalized_text_projection"
        } else {
            "pdf_full_capture"
        };
        if !eq_str(field("input_plane"), expected_plane) {
            problems.push(format!("held-out attempt input plane differs: {pair_repr}"));
        }
        let state = field("state");
        if !in_set(state, &STATES)? {
            problems.push(format!("held-out attempt state is invalid: {pair_repr}"));
        }
        let completed = in_set(state, &["pass", "fail"])?;
        let execution_mode = field("execution_mode");
        let expected_mode =
            if relation == "unchanged_source" || relation == "adversarial_identity" || completed {
                "fresh_pipeline"
            } else {
                "eligibility_preflight"
            };
        if !eq_str(execution_mode, expected_mode) {
            problems.push(format!(
                "held-out attempt execution mode differs: {pair_repr}"
            ));
        }
        let identity = field("identity");
        let coverage = field("coverage");
        let report_digest = field("attempt_report_sha256");
        if completed {
            match (identity, coverage) {
                (Some(identity @ Py::Dict(_)), Some(Py::Dict(_))) => {
                    let source_field = if relation == "symbol_alpha" {
                        "normalized_markdown_sha256"
                    } else {
                        "source_sha256"
                    };
                    let tool = if eq_str(execution_mode, "retained_report_revalidated") {
                        get("retained_tool_sha256")
                    } else {
                        get("tool_sha256")
                    };
                    let source = s(row_index(prospective_by_key[key.as_str()], source_field)?);
                    let identity_checks: Vec<(&str, Py)> = vec![
                        (
                            "contract_sha256",
                            get("contract_sha256").cloned().unwrap_or(Py::None),
                        ),
                        (
                            "production_revision",
                            get("production_revision").cloned().unwrap_or(Py::None),
                        ),
                        ("source_sha256", source),
                        (
                            "prior_memory_sha256",
                            get("prior_memory_sha256").cloned().unwrap_or(Py::None),
                        ),
                        ("tool_sha256", tool.cloned().unwrap_or(Py::None)),
                    ];
                    for (name, expected) in &identity_checks {
                        if !eq(pyjson::get(identity, name), expected) {
                            problems.push(format!(
                                "held-out attempt {pair_repr} identity {name} differs"
                            ));
                        }
                    }
                }
                _ => problems.push(format!(
                    "completed held-out attempt lacks identity/coverage: {pair_repr}"
                )),
            }
            if !is_hex(report_digest, 64) {
                problems.push(format!(
                    "completed held-out attempt lacks report digest: {pair_repr}"
                ));
            }
            if !eq(field("completed_stages"), &int(5)) {
                problems.push(format!(
                    "completed held-out attempt lacks five stages: {pair_repr}"
                ));
            }
        } else {
            if !is_none(identity) || !is_none(coverage) || !is_none(report_digest) {
                problems.push(format!(
                    "non-completed held-out attempt carries a report: {pair_repr}"
                ));
            }
            if !eq(field("completed_stages"), &int(0)) {
                problems.push(format!(
                    "non-completed held-out attempt claims completed stages: {pair_repr}"
                ));
            }
        }
        let passed = eq_str(state, "pass");
        if passed && !is_none(field("failure_id")) {
            problems.push(format!(
                "passing held-out attempt carries a failure id: {pair_repr}"
            ));
        }
        if !passed && !is_str(field("failure_id")) {
            problems.push(format!(
                "non-passing held-out attempt lacks a failure id: {pair_repr}"
            ));
        }
        if in_set(state, &["unmeasurable", "invalid"])? && !is_str(field("detail")) {
            problems.push(format!(
                "non-completed held-out attempt lacks detail: {pair_repr}"
            ));
        }
        if completed && !is_none(field("detail")) {
            problems.push(format!(
                "completed held-out attempt carries error detail: {pair_repr}"
            ));
        }
        if !eq(field("all_completed_stages_passed"), &Py::Bool(passed)) {
            problems.push(format!(
                "held-out all-stage disposition differs: {pair_repr}"
            ));
        }
    }
    let expected_pairs: BTreeSet<(String, String)> = prospective_keys
        .iter()
        .flat_map(|key| {
            HELD_OUT_RELATIONS
                .iter()
                .map(move |relation| (key.clone(), (*relation).to_string()))
        })
        .collect();
    let actual_pairs: BTreeSet<(String, String)> = attempt_by_pair.keys().cloned().collect();
    if actual_pairs != expected_pairs {
        problems.push("held-out evidence attempt matrix is incomplete".to_string());
    }

    let state_of = |pair: &(String, String)| {
        attempt_by_pair
            .get(pair)
            .and_then(|attempt| pyjson::get(attempt, "state"))
    };
    match get("coverage") {
        Some(coverage @ Py::Dict(_)) => {
            // The counts call `attempt.get` on every attempt, the malformed ones the loop above skipped included.
            for attempt in attempts {
                attr(attempt, "state")?;
            }
            let count_state = |state: &str| {
                attempts
                    .iter()
                    .filter(|attempt| eq_str(pyjson::get(attempt, "state"), state))
                    .count() as i64
            };
            let coverage_checks: Vec<(&str, Py)> = vec![
                ("declared_documents", string_list(&prospective_keys)),
                ("attempted_documents", string_list(&prospective_keys)),
                ("declared_attempts", int(expected_pairs.len() as i64)),
                ("pass_attempts", int(count_state("pass"))),
                ("fail_attempts", int(count_state("fail"))),
                ("unmeasurable_attempts", int(count_state("unmeasurable"))),
                ("invalid_attempts", int(count_state("invalid"))),
            ];
            for (field, expected) in &coverage_checks {
                if !eq(pyjson::get(coverage, field), expected) {
                    problems.push(format!("held-out aggregate coverage {field} differs"));
                }
            }
            let relation_count = HELD_OUT_RELATIONS.len();
            let completed_count = |key: &String| -> Checked<usize> {
                let mut count = 0;
                for relation in HELD_OUT_RELATIONS {
                    if in_set(
                        state_of(&(key.clone(), relation.to_string())),
                        &["pass", "fail"],
                    )? {
                        count += 1;
                    }
                }
                Ok(count)
            };
            // `completed_by_key` was a dict, so a repeated prospective key counts once here (the document-state
            // lists below iterate the keys themselves, repeats included).
            let mut fully = Vec::new();
            let mut partially = Vec::new();
            let distinct: BTreeSet<&String> = prospective_keys.iter().collect();
            for key in distinct {
                let count = completed_count(key)?;
                if count == relation_count {
                    fully.push(key.clone());
                } else if count > 0 {
                    partially.push(key.clone());
                }
            }
            let with_state = |state: &str| -> Vec<String> {
                prospective_keys
                    .iter()
                    .filter(|key| {
                        HELD_OUT_RELATIONS.iter().any(|relation| {
                            eq_str(state_of(&((*key).clone(), relation.to_string())), state)
                        })
                    })
                    .cloned()
                    .collect()
            };
            let set_checks: Vec<(&str, Vec<String>)> = vec![
                ("fully_completed_documents", fully),
                ("partially_completed_documents", partially),
                ("unmeasurable_documents", with_state("unmeasurable")),
                ("invalid_documents", with_state("invalid")),
            ];
            for (field, expected) in &set_checks {
                if !eq(pyjson::get(coverage, field), &string_list(expected)) {
                    problems.push(format!("held-out aggregate coverage {field} differs"));
                }
            }
            let coverages: Vec<&Py> = attempts
                .iter()
                .filter_map(|attempt| {
                    pyjson::get(attempt, "coverage")
                        .filter(|coverage| matches!(coverage, Py::Dict(_)))
                })
                .collect();
            for field in [
                "baseline_top_level_fields",
                "transformed_top_level_fields",
                "baseline_proof_claims",
                "transformed_proof_claims",
                "compared_leaf_values",
            ] {
                let expected = coverage_total(&coverages, field)?;
                if !eq(pyjson::get(coverage, field), &expected) {
                    problems.push(format!("held-out aggregate coverage {field} differs"));
                }
            }
            let expected_deltas = coverage_nested_total(
                &coverages,
                &[
                    "expected_symbol_deltas",
                    "expected_reviewed_span_deltas",
                    "expected_semantic_deltas",
                ],
            )?;
            let observed_deltas = coverage_nested_total(
                &coverages,
                &[
                    "observed_symbol_deltas",
                    "observed_reviewed_span_deltas",
                    "observed_semantic_deltas",
                ],
            )?;
            if !eq(pyjson::get(coverage, "expected_deltas"), &expected_deltas) {
                problems.push("held-out aggregate expected-delta coverage differs".to_string());
            }
            if !eq(pyjson::get(coverage, "observed_deltas"), &observed_deltas) {
                problems.push("held-out aggregate observed-delta coverage differs".to_string());
            }
        }
        _ => problems.push("held-out aggregate coverage must be an object".to_string()),
    }

    let vendors: BTreeSet<String> = prospective
        .iter()
        .map(|row| row_index(row, "vendor").map(str::to_string))
        .collect::<Checked<_>>()?;
    let families: BTreeSet<String> = prospective
        .iter()
        .map(|row| row_index(row, "family").map(str::to_string))
        .collect::<Checked<_>>()?;
    let mut expected_values: Vec<(&str, BTreeSet<String>)> =
        vec![("overall", ["all".to_string()].into())];
    expected_values.push(("vendor", vendors));
    expected_values.push((
        "vendor_novelty",
        ["seen_in_calibration".to_string(), "novel".to_string()].into(),
    ));
    expected_values.push(("family", families));
    expected_values.push((
        "family_novelty",
        ["seen_in_calibration".to_string(), "novel".to_string()].into(),
    ));
    expected_values.push((
        "category",
        [
            "cpu-isa",
            "methodology-guide",
            "physical-link",
            "platform-system-ip",
            "register-ip",
            "wire-protocol",
        ]
        .iter()
        .map(|value| (*value).to_string())
        .collect(),
    ));
    expected_values.push((
        "layout",
        ["compact", "medium", "long"]
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    ));
    let expected_strata: BTreeSet<(String, String, String)> = HELD_OUT_RELATIONS
        .iter()
        .flat_map(|relation| {
            expected_values.iter().flat_map(move |(dimension, values)| {
                values.iter().map(move |value| {
                    (
                        (*relation).to_string(),
                        (*dimension).to_string(),
                        value.clone(),
                    )
                })
            })
        })
        .collect();
    let strata: &[Py] = match get("strata") {
        Some(Py::List(strata)) => strata,
        _ => {
            problems.push("held-out strata must be an array".to_string());
            &[]
        }
    };
    // Identities as Python saw them: any hashable value, compared by Python's equality.
    let mut stratum_identities: Vec<[Py; 3]> = Vec::new();
    for stratum in strata {
        if !matches!(stratum, Py::Dict(_)) {
            problems.push("held-out stratum row is malformed".to_string());
            continue;
        }
        let parts = [
            pyjson::get(stratum, "relation")
                .cloned()
                .unwrap_or(Py::None),
            pyjson::get(stratum, "dimension")
                .cloned()
                .unwrap_or(Py::None),
            pyjson::get(stratum, "value").cloned().unwrap_or(Py::None),
        ];
        if parts
            .iter()
            .any(|part| matches!(part, Py::List(_) | Py::Dict(_)))
        {
            return Err(Uncaught("unhashable type".to_string()));
        }
        let identity_repr = tuple_repr(&[Some(&parts[0]), Some(&parts[1]), Some(&parts[2])]);
        let seen = stratum_identities.iter().any(|known| {
            known
                .iter()
                .zip(&parts)
                .all(|(left, right)| pyjson::py_eq(left, right))
        });
        if seen {
            problems.push(format!("held-out stratum is duplicated: {identity_repr}"));
        } else {
            stratum_identities.push(parts.clone());
        }
        let as_identity = match (&parts[0], &parts[1], &parts[2]) {
            (Py::Str(relation), Py::Str(dimension), Py::Str(value)) => {
                let identity = (text(relation), text(dimension), text(value));
                expected_strata.contains(&identity).then_some(identity)
            }
            _ => None,
        };
        let Some((relation, dimension, value)) = as_identity else {
            problems.push(format!(
                "held-out stratum is outside the closed matrix: {identity_repr}"
            ));
            continue;
        };
        let mut selected_keys: Vec<String> = Vec::new();
        for key in &document_order {
            let document = document_by_key[key];
            let field = |name: &str| pyjson::get(document, name);
            let included = match dimension.as_str() {
                "overall" => value == "all",
                "vendor" => eq_str(field("vendor"), &value),
                "family" => eq_str(field("family"), &value),
                "category" => eq_str(field("category"), &value),
                "layout" => eq_str(field("layout"), &value),
                "vendor_novelty" => eq(field("vendor_novel"), &Py::Bool(value == "novel")),
                "family_novelty" => eq(field("family_novel"), &Py::Bool(value == "novel")),
                _ => false,
            };
            if included {
                selected_keys.push(key.clone());
            }
        }
        selected_keys.sort();
        let selected: Vec<&Py> = selected_keys
            .iter()
            .filter_map(|key| {
                attempt_by_pair
                    .get(&(key.clone(), relation.clone()))
                    .copied()
            })
            .collect();
        let count = |state: &str| {
            selected
                .iter()
                .filter(|attempt| eq_str(pyjson::get(attempt, "state"), state))
                .count() as i64
        };
        let (passes, fails, unmeasurable, invalid) = (
            count("pass"),
            count("fail"),
            count("unmeasurable"),
            count("invalid"),
        );
        let expected_state = if invalid > 0 {
            "invalid"
        } else if fails > 0 {
            "fail"
        } else if selected_keys.is_empty() || unmeasurable > 0 {
            "unmeasurable"
        } else {
            "pass"
        };
        let stratum_checks: Vec<(&str, Py)> = vec![
            ("document_keys", string_list(&selected_keys)),
            ("declared_documents", int(selected_keys.len() as i64)),
            ("passes", int(passes)),
            ("failures", int(fails)),
            ("unmeasurable", int(unmeasurable)),
            ("invalid", int(invalid)),
            ("state", s(expected_state)),
        ];
        for (field, expected) in &stratum_checks {
            if !eq(pyjson::get(stratum, field), expected) {
                problems.push(format!("held-out stratum {identity_repr} {field} differs"));
            }
        }
        let expected_limitation = if selected_keys.is_empty() {
            s("no_prospective_denominator")
        } else if invalid > 0 {
            s("invalid_attempts_present")
        } else if unmeasurable > 0 {
            s("unmeasurable_attempts_excluded_from_interval")
        } else {
            Py::None
        };
        if !eq(pyjson::get(stratum, "limitation"), &expected_limitation) {
            problems.push(format!(
                "held-out stratum {identity_repr} limitation differs"
            ));
        }
        let uncertainty = pyjson::get(stratum, "uncertainty");
        let completed = passes + fails;
        match (uncertainty, wilson_parts_per_million(passes, completed)) {
            (Some(uncertainty @ Py::Dict(_)), None) => {
                let field = |name: &str| pyjson::get(uncertainty, name);
                if !eq_str(field("method"), "unavailable_no_completed_denominator")
                    || !eq(field("sample_size"), &int(0))
                    || !is_none(field("confidence_basis_points"))
                    || [
                        "point_estimate_parts_per_million",
                        "lower_parts_per_million",
                        "upper_parts_per_million",
                    ]
                    .iter()
                    .any(|name| !is_none(field(name)))
                {
                    problems.push(format!(
                        "held-out stratum {identity_repr} unavailable interval differs"
                    ));
                }
            }
            (Some(uncertainty @ Py::Dict(_)), Some((point, lower, upper))) => {
                let uncertainty_checks: Vec<(&str, Py)> = vec![
                    ("method", s("wilson_score_95_percent")),
                    ("confidence_basis_points", int(9500)),
                    ("sample_size", int(completed)),
                    ("pass_numerator", int(passes)),
                    ("point_estimate_parts_per_million", int(point)),
                    ("lower_parts_per_million", int(lower)),
                    ("upper_parts_per_million", int(upper)),
                ];
                for (field, expected) in &uncertainty_checks {
                    if !eq(pyjson::get(uncertainty, field), expected) {
                        problems.push(format!(
                            "held-out stratum {identity_repr} uncertainty {field} differs"
                        ));
                    }
                }
            }
            _ => problems.push(format!(
                "held-out stratum {identity_repr} uncertainty is absent"
            )),
        }
        let scoped = match uncertainty {
            Some(uncertainty @ Py::Dict(_)) => {
                let scope = pyjson::get(uncertainty, "scope_limit")
                    .cloned()
                    .unwrap_or(s(""));
                pyjson::str_of(&scope).contains("frozen population")
            }
            _ => false,
        };
        if !scoped {
            problems.push(format!(
                "held-out stratum {identity_repr} lacks scope limitation"
            ));
        }
    }
    let actual_strata: Vec<Option<(String, String, String)>> = stratum_identities
        .iter()
        .map(|parts| match parts {
            [Py::Str(relation), Py::Str(dimension), Py::Str(value)] => {
                Some((text(relation), text(dimension), text(value)))
            }
            _ => None,
        })
        .collect();
    let complete = actual_strata.len() == expected_strata.len()
        && actual_strata.iter().all(|identity| {
            identity
                .as_ref()
                .is_some_and(|identity| expected_strata.contains(identity))
        });
    if !complete {
        problems.push("held-out evidence stratum matrix is incomplete".to_string());
    }
    let mut absolute_paths = Vec::new();
    find_absolute(report, "", &mut absolute_paths);
    if !absolute_paths.is_empty() {
        absolute_paths.truncate(5);
        problems.push(format!(
            "held-out evidence persists absolute paths: {}",
            list_repr(&absolute_paths)
        ));
    }
    Ok(())
}

fn validate_post_boundary_retention(
    repository: &mut Repository,
    contract: &Py,
    unqualified_retained: &[String],
    frozen_keys: &BTreeSet<String>,
    problems: &mut Vec<String>,
    declaration: Option<&Py>,
) -> Checked<()> {
    let loaded;
    let block: &Py = match declaration {
        Some(block) => block,
        None => match repository.read_json_guarded(&repository.path(POST_BOUNDARY_PATH))? {
            Ok(block) => {
                loaded = block;
                &loaded
            }
            Err(error) => {
                problems.push(format!(
                    "post-boundary retention declaration is unavailable: {error}"
                ));
                return Ok(());
            }
        },
    };
    if !matches!(block, Py::Dict(_)) {
        problems.push("post-boundary retention declaration must be an object".to_string());
        return Ok(());
    }
    if !eq(pyjson::get(block, "schema_version"), &int(1)) {
        problems.push("post-boundary retention schema_version must be 1".to_string());
    }
    let authority = pyjson::get(block, "authority").cloned().unwrap_or(s(""));
    if !contains(&authority, "subset floor")? {
        problems
            .push("post-boundary retention does not state the subset-floor authority".to_string());
    }
    let Some(Py::List(declared)) = pyjson::get(block, "unqualified_keys") else {
        problems.push("post-boundary retention unqualified_keys must be an array".to_string());
        return Ok(());
    };
    let eligible = attr_or_empty(contract, "held_out_execution").unwrap_or(&EMPTY);
    let eligible_relations: Vec<Py> = if matches!(eligible, Py::Dict(_)) {
        match pyjson::get(eligible, "eligible_relations") {
            Some(Py::List(relations)) => py_set(relations)?,
            Some(other @ (Py::Str(_) | Py::Dict(_))) => py_set(&iterate(other))?,
            None => Vec::new(),
            Some(other) => {
                return Err(Uncaught(format!(
                    "'{}' object is not iterable",
                    type_name(other)
                )));
            }
        }
    } else {
        Vec::new()
    };
    let mut declared_keys: Vec<String> = Vec::new();
    for (index, entry) in declared.iter().enumerate() {
        let label = format!("post-boundary retention entry {index}");
        if !matches!(entry, Py::Dict(_)) {
            problems.push(format!("{label} must be an object"));
            continue;
        }
        let Some(key) = as_str(pyjson::get(entry, "document_key")).filter(|key| !key.is_empty())
        else {
            problems.push(format!("{label} has no document_key"));
            continue;
        };
        declared_keys.push(key.clone());
        if !unqualified_retained.contains(&key) {
            if frozen_keys.contains(&key) {
                problems.push(format!(
                    "{label}: {key} is inside the frozen qualified population and cannot be declared unqualified"
                ));
            } else {
                problems.push(format!("{label}: {key} is not a retained bundle"));
            }
        }
        let owed_ok = match pyjson::get(entry, "owed_relations") {
            Some(Py::List(owed)) if !owed.is_empty() => {
                py_set(owed)?.iter().all(|relation| {
                    eligible_relations
                        .iter()
                        .any(|eligible| pyjson::py_eq(relation, eligible))
                }) && !has_duplicates(owed)?
            }
            _ => false,
        };
        if !owed_ok {
            problems.push(format!(
                "{label}: owed_relations must be a distinct non-empty subset of {}",
                sorted_repr(&eligible_relations)?
            ));
        }
        if as_str(pyjson::get(entry, "owing_leaf")).is_none_or(|leaf| leaf.is_empty()) {
            problems.push(format!(
                "{label}: owing_leaf must name the leaf that owes the relations"
            ));
        }
    }
    let distinct: BTreeSet<&String> = declared_keys.iter().collect();
    if distinct.len() != declared_keys.len() {
        problems.push("post-boundary retention declares a document_key more than once".to_string());
    }
    let declared_set: BTreeSet<&String> = declared_keys.iter().collect();
    let undeclared: Vec<String> = unqualified_retained
        .iter()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|key| !declared_set.contains(key))
        .cloned()
        .collect();
    if !undeclared.is_empty() {
        problems.push(format!(
            "retained bundles are outside the frozen behavioral population and undeclared as unqualified \
             residuals: {}",
            list_repr(&undeclared)
        ));
    }
    Ok(())
}

/// `repr(sorted(values))` of a set's members: strings sort among strings and numbers among numbers; Python cannot
/// order two members of different kinds.
fn sorted_repr(values: &[Py]) -> Checked<String> {
    let number = |value: &Py| match value {
        Py::Bool(flag) => f64::from(u8::from(*flag)),
        Py::Int(digits) => digits.parse().unwrap_or(f64::NAN),
        Py::Float(number) => *number,
        _ => f64::NAN,
    };
    let mut values = values.to_vec();
    if values.iter().all(|value| matches!(value, Py::Str(_))) {
        values.sort_by(|left, right| match (left, right) {
            (Py::Str(left), Py::Str(right)) => left.cmp(right),
            _ => std::cmp::Ordering::Equal,
        });
    } else if values
        .iter()
        .all(|value| matches!(value, Py::Bool(_) | Py::Int(_) | Py::Float(_)))
    {
        values.sort_by(|left, right| {
            number(left)
                .partial_cmp(&number(right))
                .unwrap_or(std::cmp::Ordering::Equal)
        });
    } else if values.len() > 1 {
        return Err(Uncaught(
            "'<' not supported between instances of different types".to_string(),
        ));
    }
    let items: Vec<String> = values.iter().map(pyjson::repr).collect();
    Ok(format!("[{}]", items.join(", ")))
}

/// Iterating a `str` gives its characters and a `dict` its keys.
fn iterate(value: &Py) -> Vec<Py> {
    match value {
        Py::Str(characters) => characters
            .iter()
            .map(|character| Py::Str(vec![*character]))
            .collect(),
        Py::Dict(pairs) => pairs.iter().map(|(key, _)| Py::Str(key.clone())).collect(),
        Py::List(items) => items.clone(),
        _ => Vec::new(),
    }
}

/// What `validate` was asked to check.
#[derive(Clone, Copy)]
struct Scope<'a> {
    verify_artifacts: bool,
    verify_held_out_evidence: bool,
    retained_override: Option<&'a [String]>,
    post_boundary_override: Option<&'a Py>,
}

const FULL: Scope<'static> = Scope {
    verify_artifacts: true,
    verify_held_out_evidence: true,
    retained_override: None,
    post_boundary_override: None,
};

#[allow(clippy::too_many_lines)]
fn validate(
    repository: &mut Repository,
    contract: &Py,
    rows: &[Row],
    scope: Scope,
) -> Checked<(Vec<String>, BTreeMap<String, i64>)> {
    let mut problems: Vec<String> = Vec::new();
    let mut metrics: BTreeMap<String, i64> = BTreeMap::new();
    let get = |key: &str| pyjson::get(contract, key);
    if !eq(get("schema_version"), &int(1)) {
        problems.push("contract schema_version must be 1".to_string());
    }
    if !eq_str(get("owner"), "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f") {
        problems.push("contract owner differs from the behavioral task-tree parent".to_string());
    }
    if !is_hex(get("selection_boundary_commit"), 40) {
        problems.push("selection_boundary_commit must be a full Git object id".to_string());
    }
    if !is_dict(get("declarations")) {
        problems.push("declarations must be an object".to_string());
        return Ok((problems, metrics));
    }
    let expected_declarations: [(&str, &str); 8] = [
        ("current_population", POPULATION_PATH),
        (
            "retained_population",
            "doctrine/chain_currency/retained_bundles.json",
        ),
        (
            "reviewed_population",
            "crates/specforge/test_data/source_to_intent_vertical/reviewed_dataset.json",
        ),
        ("held_out_evidence", HELD_OUT_EVIDENCE_PATH),
        ("reviewed_recipe_manifest", RECIPE_MANIFEST_PATH),
        ("semantic_negative_matrix", NEGATIVE_MATRIX_PATH),
        (
            "claim_family_inventory",
            "doctrine/production_genericity/claim_family_inventory.tsv",
        ),
        (
            "rule_family_inventory",
            "doctrine/production_genericity/rule_family_inventory.tsv",
        ),
    ];
    if !eq(get("declarations"), &dict_of(&expected_declarations)) {
        problems.push("contract declarations differ from the closed authority set".to_string());
    }
    for (label, relative) in expected_declarations {
        let path = safe_relative_path(Some(&s(relative)))
            .map(|relative| repository.path(&relative.as_posix()));
        if !path.is_some_and(|path| is_file(&path)) {
            problems.push(format!("declared {label} is missing or unsafe: {relative}"));
        }
    }
    validate_reviewed_recipes(repository, rows, &mut problems)?;
    validate_negative_sensitivity_matrix(repository, &mut problems)?;

    let relation_ids = unique_ids(get("relations"), "relation_id", "relations", &mut problems);
    let expected_relations: BTreeSet<String> = EXPECTED_RELATIONS
        .iter()
        .map(|relation| (*relation).to_string())
        .collect();
    if relation_ids != expected_relations {
        let missing: Vec<&String> = expected_relations.difference(&relation_ids).collect();
        let extra: Vec<&String> = relation_ids.difference(&expected_relations).collect();
        problems.push(format!(
            "relation ids differ: missing={}, extra={}",
            list_repr(&missing),
            list_repr(&extra)
        ));
    }
    for relation in iterate_or_refuse(get("relations"))? {
        if !matches!(relation, Py::Dict(_)) {
            continue;
        }
        let field = |name: &str| pyjson::get(&relation, name);
        let relation_id =
            field("relation_id").map_or_else(|| "<unknown>".to_string(), pyjson::str_of);
        let expected_plane = match field("relation_id") {
            Some(id @ Py::Str(_)) => EXPECTED_PLANE_RELATIONS
                .iter()
                .find(|(_, relations)| relations.iter().any(|item| pyjson::py_eq(id, &s(item))))
                .map(|(plane, _)| *plane),
            Some(Py::List(_) | Py::Dict(_)) => return Err(Uncaught("unhashable type".to_string())),
            _ => None,
        };
        let expected_plane_value = expected_plane.map_or(Py::None, s);
        if !eq(field("input_plane"), &expected_plane_value) {
            problems.push(format!(
                "relation {relation_id} input plane differs: {} != {}",
                py_repr(field("input_plane")),
                pyjson::repr(&expected_plane_value)
            ));
        }
        for name in ["eligibility", "required_relation"] {
            if as_str(field(name)).is_none_or(|value| blank(&value)) {
                problems.push(format!("relation {relation_id} lacks nonempty {name}"));
            }
        }
        for name in ["allowed_deltas", "required_deltas", "forbidden_deltas"] {
            match field(name) {
                Some(Py::List(deltas))
                    if deltas.iter().all(|delta| {
                        as_str(Some(delta)).is_some_and(|delta| !delta.is_empty())
                    }) =>
                {
                    if has_duplicates(deltas)? {
                        problems.push(format!("relation {relation_id} has duplicate {name}"));
                    }
                }
                _ => problems.push(format!("relation {relation_id} has invalid {name}")),
            }
        }
        if !field("forbidden_deltas").is_some_and(pyjson::truthy) {
            problems.push(format!(
                "relation {relation_id} has no forbidden-delta complement"
            ));
        }
    }

    let stage_ids = unique_ids(
        get("stage_comparison"),
        "stage_id",
        "stage comparison",
        &mut problems,
    );
    let expected_stages: BTreeSet<String> = EXPECTED_STAGES
        .iter()
        .map(|stage| (*stage).to_string())
        .collect();
    if stage_ids != expected_stages {
        let missing: Vec<&String> = expected_stages.difference(&stage_ids).collect();
        let extra: Vec<&String> = stage_ids.difference(&expected_stages).collect();
        problems.push(format!(
            "comparison stages differ: missing={}, extra={}",
            list_repr(&missing),
            list_repr(&extra)
        ));
    }
    for stage in iterate_or_refuse(get("stage_comparison"))? {
        if !matches!(stage, Py::Dict(_)) {
            continue;
        }
        let field = |name: &str| pyjson::get(&stage, name);
        let stage_id = py_display(field("stage_id"));
        if !matches!(field("proof_required"), Some(Py::Bool(true))) {
            problems.push(format!("stage {stage_id} does not require proof"));
        }
        let coverage = pyjson::str_of(field("coverage").unwrap_or(&s("")));
        if !coverage.contains("every serialized top-level field") {
            problems.push(format!("stage {stage_id} has partial field coverage"));
        }
        if !coverage.contains("every proof claim") {
            problems.push(format!("stage {stage_id} has partial proof coverage"));
        }
        if as_str(field("semantic_surfaces")).is_none_or(|value| blank(&value)) {
            problems.push(format!("stage {stage_id} lacks semantic surfaces"));
        }
    }

    let plane_ids = unique_ids(
        get("input_planes"),
        "plane_id",
        "input planes",
        &mut problems,
    );
    let expected_planes: BTreeSet<String> = EXPECTED_PLANES
        .iter()
        .map(|plane| (*plane).to_string())
        .collect();
    if plane_ids != expected_planes {
        let missing: Vec<&String> = expected_planes.difference(&plane_ids).collect();
        let extra: Vec<&String> = plane_ids.difference(&expected_planes).collect();
        problems.push(format!(
            "input planes differ: missing={}, extra={}",
            list_repr(&missing),
            list_repr(&extra)
        ));
    }
    for plane in iterate_or_refuse(get("input_planes"))? {
        if !matches!(plane, Py::Dict(_)) {
            continue;
        }
        let field = |name: &str| pyjson::get(&plane, name);
        let plane_id = field("plane_id").map_or_else(|| "<unknown>".to_string(), pyjson::str_of);
        // The plane id is only hashed once the relations are a list.
        let partition = match field("relations") {
            Some(Py::List(relations)) => {
                py_set(relations)?;
                let expected: &[&str] = match field("plane_id") {
                    Some(Py::List(_) | Py::Dict(_)) => {
                        return Err(Uncaught("unhashable type".to_string()));
                    }
                    Some(id) => EXPECTED_PLANE_RELATIONS
                        .iter()
                        .find(|(name, _)| pyjson::py_eq(id, &s(name)))
                        .map_or(&[], |(_, relations)| relations),
                    None => &[],
                };
                set_is(relations, expected)?.then_some(relations)
            }
            _ => None,
        };
        match partition {
            Some(relations) => {
                if has_duplicates(relations)? {
                    problems.push(format!("input plane {plane_id} repeats a relation"));
                }
            }
            None => problems.push(format!(
                "input plane {plane_id} has an incomplete relation partition"
            )),
        }
        for name in ["authority", "restriction"] {
            if as_str(field(name)).is_none_or(|value| blank(&value)) {
                problems.push(format!("input plane {plane_id} lacks nonempty {name}"));
            }
        }
        if eq_str(field("plane_id"), "reviewed_variant") {
            if !eq_str(field("base_plane"), "normalized_text_projection") {
                problems.push("reviewed variants must derive from normalized text".to_string());
            }
        } else if field("base_plane").is_some() {
            problems.push(format!(
                "input plane {plane_id} has an unexpected base plane"
            ));
        }
    }

    let failure_rows = get("failure_taxonomy");
    unique_ids(
        failure_rows,
        "failure_id",
        "failure taxonomy",
        &mut problems,
    );
    let mut failures: Vec<(PyStr, Py)> = Vec::new();
    if let Some(Py::List(rows)) = failure_rows {
        for row in rows {
            if let (Py::Dict(_), Some(Py::Str(id))) = (row, pyjson::get(row, "failure_id")) {
                let disposition = pyjson::get(row, "disposition").cloned().unwrap_or(s(""));
                match failures.iter_mut().find(|(key, _)| key == id) {
                    Some(entry) => entry.1 = disposition,
                    None => failures.push((id.clone(), disposition)),
                }
            }
        }
    }
    if !pyjson::py_eq(&Py::Dict(failures), &dict_of(&EXPECTED_FAILURES)) {
        problems.push(
            "failure taxonomy differs from the closed fail/invalid/unmeasurable map".to_string(),
        );
    }

    match get("evidence_contract") {
        Some(evidence @ Py::Dict(_)) => {
            let sets: [(&str, &[&str]); 3] = [
                ("required_run_states", &STATES),
                ("required_identity", &EXPECTED_EVIDENCE_IDENTITY),
                ("required_coverage", &EXPECTED_EVIDENCE_COVERAGE),
            ];
            for (field, expected) in sets {
                let complete = match pyjson::get(evidence, field) {
                    Some(Py::List(values)) => set_is(values, expected)? && !has_duplicates(values)?,
                    _ => false,
                };
                if !complete {
                    problems.push(format!("evidence {field} is incomplete or duplicated"));
                }
            }
            for (field, expected) in [
                ("partial_runs", "invalid"),
                ("missing_expected_delta", "fail"),
                ("undeclared_delta", "fail"),
                ("missing_authority_or_provider", "unmeasurable"),
                ("ambiguous_or_escaped_transform", "invalid"),
            ] {
                if !eq_str(pyjson::get(evidence, field), expected) {
                    problems.push(format!(
                        "evidence disposition {field} differs: {} != {}",
                        py_repr(pyjson::get(evidence, field)),
                        str_repr(expected)
                    ));
                }
            }
        }
        _ => problems.push("evidence_contract must be an object".to_string()),
    }

    let reviewed = repository.read_json_unguarded(&repository.path(expected_declarations[2].1))?;
    let retained_keys_value: Option<Py> = match scope.retained_override {
        Some(keys) => Some(Py::List(keys.iter().map(|key| s(key)).collect())),
        None => {
            let retained =
                repository.read_json_unguarded(&repository.path(expected_declarations[1].1))?;
            match &*retained {
                Py::Dict(_) => pyjson::get(&retained, "retained").cloned(),
                _ => None,
            }
        }
    };
    let retained_keys: Vec<String> = match &retained_keys_value {
        Some(Py::List(keys)) if keys.iter().all(|key| matches!(key, Py::Str(_))) => {
            keys.iter().map(pyjson::str_of).collect()
        }
        _ => {
            problems.push("retained population has no string retained array".to_string());
            Vec::new()
        }
    };
    let reviewed_documents: &[Py] = match &*reviewed {
        Py::Dict(_) => match pyjson::get(&reviewed, "documents") {
            Some(Py::List(documents)) => documents,
            _ => {
                problems.push("reviewed population has no documents array".to_string());
                &[]
            }
        },
        _ => {
            problems.push("reviewed population has no documents array".to_string());
            &[]
        }
    };
    let mut reviewed_by_key: HashMap<String, &Py> = HashMap::new();
    for document in reviewed_documents {
        if let (Py::Dict(_), Some(Py::Str(key))) = (document, pyjson::get(document, "document_key"))
        {
            reviewed_by_key.insert(text(key), document);
        }
    }
    let row_keys: Vec<Option<&str>> = rows
        .iter()
        .map(|row| match row.get("document_key") {
            None => Some(""),
            Some(value) => value.as_deref(),
        })
        .collect();
    if row_keys.iter().any(Option::is_none) {
        return Err(Uncaught(
            "'<' not supported between instances of 'NoneType' and 'str'".to_string(),
        ));
    }
    let row_keys: Vec<String> = row_keys
        .into_iter()
        .map(|key| key.unwrap_or_default().to_string())
        .collect();
    let mut sorted_keys = row_keys.clone();
    sorted_keys.sort();
    if row_keys != sorted_keys {
        problems.push("behavioral population rows are not sorted by document_key".to_string());
    }
    let row_set: BTreeSet<String> = row_keys.iter().cloned().collect();
    if row_set.len() != row_keys.len() {
        problems.push("behavioral population contains duplicate document keys".to_string());
    }
    let retained_set: BTreeSet<String> = retained_keys.iter().cloned().collect();
    let vanished: Vec<&String> = row_set.difference(&retained_set).collect();
    if !vanished.is_empty() {
        problems.push(format!(
            "behavioral population rows are no longer retained bundles: {}",
            list_repr(&vanished)
        ));
    }
    let unqualified_retained: Vec<String> = retained_set.difference(&row_set).cloned().collect();
    validate_post_boundary_retention(
        repository,
        contract,
        &unqualified_retained,
        &row_set,
        &mut problems,
        scope.post_boundary_override,
    )?;
    metrics.insert(
        "post_boundary_unqualified_documents".to_string(),
        unqualified_retained.len() as i64,
    );

    let mut calibration_rows: Vec<&Row> = Vec::new();
    let mut prospective_rows: Vec<&Row> = Vec::new();
    let (mut source_verified, mut external_unavailable) = (0i64, 0i64);
    let (mut markdown_bytes, mut semantic_total, mut intent_total) = (0i64, 0i64, 0i64);
    let root_device = device(&repository.root)?;
    for row in rows {
        let key = row_get(row, "document_key")
            .unwrap_or("<unknown>")
            .to_string();
        for field in POPULATION_FIELDS {
            if row.get(field).is_none() || row_get(row, field) == Some("") {
                problems.push(format!("{key}: population field {field} is empty"));
            }
        }
        let mut values: BTreeMap<&str, i64> = BTreeMap::new();
        for field in INTEGER_FIELDS {
            values.insert(field, integer(row, field, &mut problems)?);
        }
        let hex = |value: &str| {
            value.len() == 64
                && value
                    .bytes()
                    .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
        };
        if !hex(row_text(row, "source_sha256")?) {
            problems.push(format!("{key}: source_sha256 is not a lowercase SHA-256"));
        }
        if !hex(row_text(row, "normalized_markdown_sha256")?) {
            problems.push(format!(
                "{key}: normalized_markdown_sha256 is not a lowercase SHA-256"
            ));
        }
        if row_get(row, "layout") != Some(expected_layout(values["pages"])) {
            problems.push(format!("{key}: layout differs from the frozen page bands"));
        }
        let reviewed_document = reviewed_by_key
            .get(&key)
            .copied()
            .filter(|document| pyjson::truthy(document));
        let expected_role = if reviewed_document.is_some() {
            "reviewed_calibration"
        } else {
            "prospective_holdout"
        };
        if row_get(row, "review_role") != Some(expected_role) {
            problems.push(format!("{key}: review_role must be {expected_role}"));
        }
        if let Some(document) = reviewed_document {
            calibration_rows.push(row);
            let category = row_get(row, "category").map_or(Py::None, s);
            if !eq(pyjson::get(document, "category"), &category) {
                problems.push(format!("{key}: category differs from reviewed authority"));
            }
            let source = attr_or_empty(document, "source")?;
            let digest = row_get(row, "source_sha256").map_or(Py::None, s);
            if !eq(attr(source, "sha256")?, &digest) {
                problems.push(format!(
                    "{key}: source digest differs from reviewed authority"
                ));
            }
            let relative = attr(source, "relative_path")?;
            let expected_locator = if relative.is_some_and(pyjson::truthy) {
                relative
            } else {
                attr(source, "portable_id")?
            };
            let locator = row_get(row, "source_locator").map_or(Py::None, s);
            if !eq(expected_locator, &locator) {
                problems.push(format!(
                    "{key}: source locator differs from reviewed authority"
                ));
            }
        } else {
            prospective_rows.push(row);
        }
        let (semantic, intent) = (
            values["text_semantic_records"],
            values["text_intent_records"],
        );
        if semantic == 0 && intent != 0 {
            problems.push(format!(
                "{key}: text projection has IntentIR records without SemanticIR records"
            ));
        }
        if semantic != 0 && intent == 0 {
            problems.push(format!(
                "{key}: text projection silently loses every semantic record"
            ));
        }
        semantic_total += semantic;
        intent_total += intent;

        let source_ir_path = repository.path(&ARTIFACT_PATHS[0].1.replace("{key}", &key));
        if !is_file(&source_ir_path) {
            problems.push(format!("{key}: current SourceIR is missing"));
            continue;
        }
        let source_ir = repository.read_source_ir(&source_ir_path)?;
        let source = attr_or_empty(&source_ir, "source")?;
        let plan = attr_or_empty(&source_ir, "normalization_plan")?;
        if !eq(
            attr(
                attr_or_empty(&source_ir, "document_identity")?,
                "document_key",
            )?,
            &s(&key),
        ) {
            problems.push(format!("{key}: SourceIR document identity differs"));
        }
        let origin = row_get(row, "source_origin").map_or(Py::None, s);
        if !eq(attr(source, "path_origin")?, &origin) {
            problems.push(format!("{key}: source origin differs from SourceIR"));
        }
        if !eq_str(attr(source, "source_kind")?, "pdf") || !eq_str(attr(plan, "status")?, "ready") {
            problems.push(format!("{key}: SourceIR is not a ready PDF capture"));
        }
        if !eq(attr(source, "size_bytes")?, &int(values["source_bytes"])) {
            problems.push(format!("{key}: source byte count differs from SourceIR"));
        }
        for (field, section) in [
            ("pages", "page_artifacts"),
            ("visual_assets", "visual_assets"),
            ("structured_tables", "structured_tables"),
            ("content_elements", "content_elements"),
            ("sections", "document_sections"),
        ] {
            let expected = python_len(attr(&source_ir, section)?)?;
            if values[field] != expected {
                problems.push(format!(
                    "{key}: {field} differs from SourceIR ({} != {expected})",
                    values[field]
                ));
            }
        }
        match row
            .get("normalized_markdown_path")
            .and_then(|value| value.as_deref())
            .filter(|value| !value.is_empty())
            .map(PurePath::new)
            .filter(|path| !path.absolute() && !path.climbs())
        {
            None => problems.push(format!(
                "{key}: normalized Markdown path is not repository-relative"
            )),
            Some(markdown_relative) => {
                if !eq(
                    attr(plan, "promoted_markdown_path")?,
                    &s(&markdown_relative.as_posix()),
                ) {
                    problems.push(format!(
                        "{key}: normalized Markdown path differs from SourceIR"
                    ));
                }
                let markdown_path = repository.path(&markdown_relative.as_posix());
                if !is_file(&markdown_path) {
                    problems.push(format!("{key}: normalized Markdown is missing"));
                } else {
                    markdown_bytes +=
                        fs::metadata(&markdown_path).map_or(0, |metadata| metadata.len() as i64);
                    if row_get(row, "normalized_markdown_sha256")
                        != Some(repository.sha256(&markdown_path)?.as_str())
                    {
                        problems.push(format!("{key}: normalized Markdown digest differs"));
                    }
                }
            }
        }
        match as_str(attr(source, "canonical_path")?) {
            None => problems.push(format!("{key}: SourceIR canonical source path is missing")),
            Some(canonical_text) => {
                let canonical = PurePath::new(&canonical_text);
                let origin = row_get(row, "source_origin");
                if origin == Some("repository_owned") {
                    if canonical.absolute()
                        || row_get(row, "source_locator") != Some(canonical.as_posix().as_str())
                    {
                        problems.push(format!(
                            "{key}: repository source locator is not canonical and relative"
                        ));
                    }
                } else if origin == Some("external_input") {
                    let locator = row_text(row, "source_locator")?;
                    if PurePath::new(locator).name() != locator {
                        problems.push(format!(
                            "{key}: external source locator must be a portable basename"
                        ));
                    }
                    if Some(canonical.name().as_str()) != row_get(row, "source_locator") {
                        problems.push(format!(
                            "{key}: external portable locator differs from SourceIR basename"
                        ));
                    }
                } else {
                    problems.push(format!("{key}: unsupported source_origin"));
                }
                let source_path = if canonical.absolute() {
                    PathBuf::from(&canonical_text)
                } else {
                    repository.root.join(&canonical_text)
                };
                if is_file(&source_path) {
                    source_verified += 1;
                    if device(&source_path)? != root_device {
                        problems.push(format!(
                            "{key}: source authority is off the repository volume"
                        ));
                    }
                    if fs::metadata(&source_path).map_or(-1, |metadata| metadata.len() as i64)
                        != values["source_bytes"]
                    {
                        problems.push(format!("{key}: live source byte count differs"));
                    }
                    if row_get(row, "source_sha256")
                        != Some(repository.sha256(&source_path)?.as_str())
                    {
                        problems.push(format!("{key}: live source digest differs"));
                    }
                } else if origin == Some("repository_owned") {
                    problems.push(format!(
                        "{key}: repository-owned source authority is missing"
                    ));
                } else {
                    external_unavailable += 1;
                }
            }
        }
        if scope.verify_artifacts {
            for (stage_id, template) in ARTIFACT_PATHS {
                let artifact_path = repository.path(&template.replace("{key}", &key));
                if !is_file(&artifact_path) {
                    problems.push(format!("{key}: current {stage_id} artifact is missing"));
                    continue;
                }
                // The script parsed the SourceIR a second time here; its projection answers the same.
                let parsed;
                let artifact = if stage_id == "source_ir" {
                    &*source_ir
                } else {
                    parsed = repository.read_artifact(&artifact_path)?;
                    &parsed
                };
                if !eq_str(attr(artifact, "stage")?, stage_id) {
                    problems.push(format!(
                        "{key}: current {stage_id} artifact names another stage"
                    ));
                }
                if !eq(
                    attr(
                        attr_or_empty(artifact, "document_identity")?,
                        "document_key",
                    )?,
                    &s(&key),
                ) {
                    problems.push(format!(
                        "{key}: current {stage_id} artifact names another document"
                    ));
                }
                let claims = attr(attr_or_empty(artifact, "proof_ledger")?, "claims")?;
                if !matches!(claims, Some(Py::List(claims)) if !claims.is_empty()) {
                    problems.push(format!(
                        "{key}: current {stage_id} artifact has no proof claims"
                    ));
                }
            }
        }
    }
    let count =
        |predicate: &dyn Fn(&Row) -> bool| rows.iter().filter(|row| predicate(row)).count() as i64;
    metrics.insert("current_documents".to_string(), rows.len() as i64);
    metrics.insert(
        "repository_owned_sources".to_string(),
        count(&|row| row_get(row, "source_origin") == Some("repository_owned")),
    );
    metrics.insert(
        "external_input_sources".to_string(),
        count(&|row| row_get(row, "source_origin") == Some("external_input")),
    );
    metrics.insert(
        "reviewed_documents".to_string(),
        reviewed_documents.len() as i64,
    );
    metrics.insert(
        "reviewed_current_overlap".to_string(),
        calibration_rows.len() as i64,
    );
    metrics.insert(
        "prospective_holdout_documents".to_string(),
        prospective_rows.len() as i64,
    );
    let mut scratch = Vec::new();
    let mut measurable = 0i64;
    let mut unmeasurable = 0i64;
    for row in rows {
        let records = integer(row, "text_intent_records", &mut scratch)?;
        if records > 0 {
            measurable += 1;
        }
        if records == 0 {
            unmeasurable += 1;
        }
    }
    metrics.insert(
        "text_projection_measurable_documents".to_string(),
        measurable,
    );
    metrics.insert(
        "text_projection_unmeasurable_documents".to_string(),
        unmeasurable,
    );
    let distinct = |field: &str, rows: &[&Row]| {
        rows.iter()
            .map(|row| row_get(row, field))
            .collect::<BTreeSet<_>>()
            .len() as i64
    };
    let all_rows: Vec<&Row> = rows.iter().collect();
    metrics.insert("vendor_strata".to_string(), distinct("vendor", &all_rows));
    metrics.insert("family_strata".to_string(), distinct("family", &all_rows));
    metrics.insert("layout_strata".to_string(), distinct("layout", &all_rows));
    metrics.insert(
        "prospective_category_strata".to_string(),
        distinct("category", &prospective_rows),
    );
    let reviewed_categories: Vec<Py> = reviewed_documents
        .iter()
        .filter(|document| matches!(document, Py::Dict(_)))
        .map(|document| {
            pyjson::get(document, "category")
                .cloned()
                .unwrap_or(Py::None)
        })
        .collect();
    metrics.insert(
        "reviewed_category_strata".to_string(),
        py_set(&reviewed_categories)?.len() as i64,
    );
    metrics.insert("source_authorities_verified".to_string(), source_verified);
    metrics.insert(
        "external_authorities_unavailable".to_string(),
        external_unavailable,
    );
    let calibration_vendors: BTreeSet<Option<&str>> = calibration_rows
        .iter()
        .map(|row| row_get(row, "vendor"))
        .collect();
    let calibration_families: BTreeSet<Option<&str>> = calibration_rows
        .iter()
        .map(|row| row_get(row, "family"))
        .collect();
    metrics.insert(
        "prospective_vendor_novel_documents".to_string(),
        prospective_rows
            .iter()
            .filter(|row| !calibration_vendors.contains(&row_get(row, "vendor")))
            .count() as i64,
    );
    metrics.insert(
        "prospective_family_novel_documents".to_string(),
        prospective_rows
            .iter()
            .filter(|row| !calibration_families.contains(&row_get(row, "family")))
            .count() as i64,
    );

    match get("population_assertions") {
        Some(assertions @ Py::Dict(_)) => {
            for field in [
                "current_documents",
                "repository_owned_sources",
                "external_input_sources",
                "reviewed_documents",
                "reviewed_current_overlap",
                "prospective_holdout_documents",
                "text_projection_measurable_documents",
                "text_projection_unmeasurable_documents",
                "vendor_strata",
                "prospective_vendor_novel_documents",
                "family_strata",
                "prospective_family_novel_documents",
                "layout_strata",
                "prospective_category_strata",
                "reviewed_category_strata",
            ] {
                let expected = metrics.get(field).map_or(Py::None, |value| int(*value));
                if !eq(pyjson::get(assertions, field), &expected) {
                    problems.push(format!(
                        "population assertion {field} differs: {} != {}",
                        py_repr(pyjson::get(assertions, field)),
                        pyjson::repr(&expected)
                    ));
                }
            }
        }
        _ => problems.push("population_assertions must be an object".to_string()),
    }

    match get("frozen_census") {
        Some(census @ Py::Dict(_)) => {
            for label in ["prior_memory", "replay_library", "replay_entrypoint"] {
                check_file_identity(repository, pyjson::get(census, label), label, &mut problems)?;
            }
            if !eq_str(pyjson::get(census, "provider_calls"), "forbidden") {
                problems.push("frozen text census must forbid provider calls".to_string());
            }
            if !eq_str(
                pyjson::get(census, "raw_output_disposition"),
                "inspected_then_exactly_cleaned",
            ) {
                problems
                    .push("frozen text census does not attest exact scratch cleanup".to_string());
            }
            match pyjson::get(census, "aggregate") {
                Some(aggregate @ Py::Dict(_)) => {
                    for (field, expected) in [
                        ("documents", rows.len() as i64),
                        ("normalized_markdown_bytes", markdown_bytes),
                        ("semantic_records", semantic_total),
                        ("intent_records", intent_total),
                    ] {
                        if !eq(pyjson::get(aggregate, field), &int(expected)) {
                            problems.push(format!(
                                "frozen census aggregate {field} differs: {} != {expected}",
                                py_repr(pyjson::get(aggregate, field))
                            ));
                        }
                    }
                    if !eq(
                        pyjson::get(aggregate, "source_ir_rich_capture_records"),
                        &int(0),
                    ) {
                        problems
                            .push("text census must not claim rich SourceIR capture".to_string());
                    }
                    for field in [
                        "source_ir_proof_claims",
                        "evidence_statements",
                        "evidence_proof_claims",
                        "semantic_proof_claims",
                        "intent_proof_claims",
                    ] {
                        let value = pyjson::get(aggregate, field);
                        let positive = is_int(value)
                            && match value {
                                Some(Py::Int(digits)) => !digits.starts_with('-') && digits != "0",
                                Some(Py::Bool(flag)) => *flag,
                                _ => false,
                            };
                        if !positive {
                            problems
                                .push(format!("frozen census aggregate {field} must be positive"));
                        }
                    }
                }
                _ => problems.push("frozen census aggregate must be an object".to_string()),
            }
        }
        _ => problems.push("frozen_census must be an object".to_string()),
    }

    match get("held_out_policy") {
        Some(policy @ Py::Dict(_)) => {
            let text_of = |name: &str| pyjson::get(policy, name).cloned().unwrap_or(s(""));
            if !contains(
                &text_of("historical_reviewed_status"),
                "not claimed historically unseen",
            )? {
                problems.push("reviewed population historical exposure is overstated".to_string());
            }
            if !contains(&text_of("category_limit"), "four categories")? {
                problems.push("prospective category limitation is not explicit".to_string());
            }
            if !contains(&text_of("leakage_rule"), "Production core cannot read")? {
                problems
                    .push("held-out calibration authority is not isolated from core".to_string());
            }
        }
        _ => problems.push("held_out_policy must be an object".to_string()),
    }

    let expected_execution = Py::Dict(vec![
        (
            pyjson::py_str("owner"),
            s("SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii"),
        ),
        (
            pyjson::py_str("eligible_relations"),
            Py::List(
                HELD_OUT_RELATIONS
                    .iter()
                    .map(|relation| s(relation))
                    .collect(),
            ),
        ),
        (pyjson::py_str("declared_attempts"), int(51)),
        (
            pyjson::py_str("stratum_dimensions"),
            Py::List(
                [
                    "overall",
                    "vendor",
                    "vendor_novelty",
                    "family",
                    "family_novelty",
                    "category",
                    "layout",
                ]
                .iter()
                .map(|dimension| s(dimension))
                .collect(),
            ),
        ),
        (
            pyjson::py_str("uncertainty_method"),
            s("wilson_score_95_percent_over_completed_documents"),
        ),
        (
            pyjson::py_str("uncertainty_scope"),
            s(
                "Descriptive for the frozen non-random prospective population only; unavailable or invalid \
                 attempts are excluded from the interval but remain explicit in its denominator record.",
            ),
        ),
        (
            pyjson::py_str("zero_denominator_disposition"),
            s("unmeasurable"),
        ),
        (
            pyjson::py_str("final_signoff"),
            s("deferred_to_SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.v"),
        ),
    ]);
    if !eq(get("held_out_execution"), &expected_execution) {
        problems.push("held-out execution contract differs from the closed matrix".to_string());
    }
    if scope.verify_held_out_evidence {
        validate_held_out_evidence(repository, contract, rows, &mut problems, None)?;
    }
    Ok((problems, metrics))
}

/// `for item in contract.get(name, [])`: a list's items, a dict's keys, a string's characters.
fn iterate_or_refuse(value: Option<&Py>) -> Checked<Vec<Py>> {
    match value {
        None => Ok(Vec::new()),
        Some(Py::None) => Err(Uncaught("'NoneType' object is not iterable".to_string())),
        Some(value @ (Py::List(_) | Py::Dict(_) | Py::Str(_))) => Ok(iterate(value)),
        Some(other) => Err(Uncaught(format!(
            "'{}' object is not iterable",
            type_name(other)
        ))),
    }
}

/// `f"{value}"` of a `dict.get`.
fn py_display(value: Option<&Py>) -> String {
    value.map_or_else(|| "None".to_string(), pyjson::str_of)
}

/// `len(source_ir.get(section, []))`.
fn python_len(value: Option<&Py>) -> Checked<i64> {
    match value {
        None => Ok(0),
        Some(Py::List(items)) => Ok(items.len() as i64),
        Some(Py::Dict(pairs)) => Ok(pairs.len() as i64),
        Some(Py::Str(characters)) => Ok(characters.len() as i64),
        Some(other) => Err(Uncaught(format!(
            "object of type '{}' has no len()",
            type_name(other)
        ))),
    }
}

fn device(path: &Path) -> Checked<u64> {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path)
        .map(|metadata| metadata.dev())
        .map_err(|error| Uncaught(os_error(&error, path)))
}

// The self-test.

/// `value[key]` on a parsed value, raising where Python's `KeyError` or `TypeError` would.
fn subscript<'a>(value: &'a Py, key: &str) -> Checked<&'a Py> {
    match value {
        Py::Dict(_) => {
            pyjson::get(value, key).ok_or_else(|| Uncaught(format!("KeyError: {}", str_repr(key))))
        }
        other => Err(Uncaught(format!(
            "'{}' object cannot be indexed by a string",
            type_name(other)
        ))),
    }
}

/// `value[key]`, to change it in place.
fn subscript_mut<'a>(value: &'a mut Py, key: &str) -> Checked<&'a mut Py> {
    let wanted = pyjson::py_str(key);
    match value {
        Py::Dict(pairs) => pairs
            .iter_mut()
            .find(|(name, _)| *name == wanted)
            .map(|(_, value)| value)
            .ok_or_else(|| Uncaught(format!("KeyError: {}", str_repr(key)))),
        other => Err(Uncaught(format!(
            "'{}' object cannot be indexed by a string",
            type_name(other)
        ))),
    }
}

/// `value[key] = new`.
fn assign(value: &mut Py, key: &str, new: Py) -> Checked<()> {
    match value {
        Py::Dict(pairs) => {
            let key = pyjson::py_str(key);
            match pairs.iter_mut().find(|(name, _)| *name == key) {
                Some(entry) => entry.1 = new,
                None => pairs.push((key, new)),
            }
            Ok(())
        }
        other => Err(Uncaught(format!(
            "'{}' object does not support item assignment",
            type_name(other)
        ))),
    }
}

/// `value.pop()`: a list's last item, refused on an empty list or anything else.
fn pop_last(value: &mut Py) -> Checked<()> {
    match value {
        Py::List(items) if !items.is_empty() => {
            items.pop();
            Ok(())
        }
        Py::List(_) => Err(Uncaught("pop from empty list".to_string())),
        other => Err(Uncaught(format!(
            "'{}' object cannot pop()",
            type_name(other)
        ))),
    }
}

/// `value.pop(key)`: a dict's entry, refused when it is absent.
fn pop_key(value: &mut Py, key: &str) -> Checked<()> {
    let wanted = pyjson::py_str(key);
    match value {
        Py::Dict(pairs) => match pairs.iter().position(|(name, _)| *name == wanted) {
            Some(index) => {
                pairs.remove(index);
                Ok(())
            }
            None => Err(Uncaught(format!("KeyError: {}", str_repr(key)))),
        },
        other => Err(Uncaught(format!(
            "'{}' object cannot pop(key)",
            type_name(other)
        ))),
    }
}

/// `value[0]`: a list's first item or a string's first character.
fn first_item(value: &Py) -> Checked<Py> {
    match value {
        Py::List(items) if !items.is_empty() => Ok(items[0].clone()),
        Py::Str(characters) if !characters.is_empty() => Ok(Py::Str(vec![characters[0]])),
        Py::Dict(_) => Err(Uncaught("KeyError: 0".to_string())),
        Py::List(_) | Py::Str(_) => Err(Uncaught("index out of range".to_string())),
        other => Err(Uncaught(format!(
            "'{}' object is not subscriptable",
            type_name(other)
        ))),
    }
}

/// `value[0]` of a list, to change it in place; a string's character cannot be changed.
fn first_mut(value: &mut Py) -> Checked<&mut Py> {
    match value {
        Py::List(items) => items
            .first_mut()
            .ok_or_else(|| Uncaught("list index out of range".to_string())),
        other => Err(Uncaught(format!(
            "'{}' object cannot be changed at [0]",
            type_name(other)
        ))),
    }
}

/// `next(item for item in value if wanted(item))` over a list, a dict's keys or a string's characters, with the
/// index of the item found; `wanted` raises where the script's condition did.
fn find_index(value: &Py, wanted: impl Fn(&Py) -> Checked<bool>) -> Checked<usize> {
    for (index, item) in iterate_or_refuse(Some(value))?.iter().enumerate() {
        if wanted(item)? {
            return Ok(index);
        }
    }
    Err(Uncaught("StopIteration".to_string()))
}

/// A list's item found by `find_index`, to change it in place; the item of a dict or string cannot be changed.
fn find_mut(value: &mut Py, wanted: impl Fn(&Py) -> Checked<bool>) -> Checked<&mut Py> {
    let index = find_index(value, wanted)?;
    match value {
        Py::List(items) => Ok(&mut items[index]),
        other => Err(Uncaught(format!(
            "'{}' object's item cannot be changed",
            type_name(other)
        ))),
    }
}

/// `value > 0` for a parsed value.
fn positive(value: &Py) -> Checked<bool> {
    match value {
        Py::Bool(flag) => Ok(*flag),
        Py::Int(digits) => Ok(digits != "0" && !digits.starts_with('-')),
        Py::Float(number) => Ok(*number > 0.0),
        other => Err(Uncaught(format!(
            "'>' not supported between instances of '{}' and 'int'",
            type_name(other)
        ))),
    }
}

/// The self-test's scope: no stage artifacts, no held-out evidence, and the retained set and its declaration given.
fn light<'a>(retained: Option<&'a [String]>, declaration: Option<&'a Py>) -> Scope<'a> {
    Scope {
        verify_artifacts: false,
        verify_held_out_evidence: false,
        retained_override: retained,
        post_boundary_override: declaration,
    }
}

/// The script's RED cases: eight mutations of the contract and population, six of the retained set and its
/// declaration, nine of the held-out evidence, and the one grown retained set ADR 0025 mandates admitting. Each
/// mutation raises where the script's subscript, `pop` or `next` would, and each boundary case must fail with its
/// own binding's problem and no other.
#[allow(clippy::too_many_lines)]
fn run_self_test(
    repository: &mut Repository,
    contract: &Py,
    rows: &[Row],
    output: &mut dyn Output,
) -> Checked<i32> {
    let (baseline, _) = validate(repository, contract, rows, light(None, None))?;
    if !baseline.is_empty() {
        output.err(&format!("{LABEL} self-test baseline failed:\n"));
        for problem in &baseline {
            output.err(&format!("  {problem}\n"));
        }
        return Ok(1);
    }
    if rows.is_empty() {
        return Err(Uncaught("pop from empty list".to_string()));
    }
    let mut mutants: Vec<(&str, Py, Vec<Row>)> = Vec::new();
    let mut missing_row = rows.to_vec();
    missing_row.pop();
    mutants.push(("population omission", contract.clone(), missing_row));
    let mut absolute_locator = rows.to_vec();
    absolute_locator[0].insert(
        "source_locator".to_string(),
        Some("/off-volume/source.pdf".to_string()),
    );
    mutants.push(("absolute locator", contract.clone(), absolute_locator));
    let mut bad_markdown_hash = rows.to_vec();
    bad_markdown_hash[0].insert(
        "normalized_markdown_sha256".to_string(),
        Some("0".repeat(64)),
    );
    mutants.push(("Markdown hash drift", contract.clone(), bad_markdown_hash));
    let stop = || Uncaught("StopIteration".to_string());
    let mut leaked = rows.to_vec();
    let calibration_index = leaked
        .iter()
        .position(|row| row_get(row, "review_role") == Some("reviewed_calibration"))
        .ok_or_else(stop)?;
    leaked[calibration_index].insert(
        "review_role".to_string(),
        Some("prospective_holdout".to_string()),
    );
    mutants.push(("review leakage", contract.clone(), leaked));
    let mut vacuity = rows.to_vec();
    let zero_index = vacuity
        .iter()
        .position(|row| row_get(row, "text_intent_records") == Some("0"))
        .ok_or_else(stop)?;
    vacuity[zero_index].insert("text_intent_records".to_string(), Some("1".to_string()));
    mutants.push(("vacuity laundering", contract.clone(), vacuity));
    let mut missing_relation = contract.clone();
    pop_last(subscript_mut(&mut missing_relation, "relations")?)?;
    mutants.push(("relation omission", missing_relation, rows.to_vec()));
    let mut partial_stage = contract.clone();
    pop_last(subscript_mut(&mut partial_stage, "stage_comparison")?)?;
    mutants.push(("stage omission", partial_stage, rows.to_vec()));
    let mut false_count = contract.clone();
    let assertion = subscript_mut(
        subscript_mut(&mut false_count, "population_assertions")?,
        "prospective_holdout_documents",
    )?;
    *assertion = match assertion {
        Py::Int(digits) => Py::Int(
            (digits
                .parse::<i128>()
                .map_err(|_| Uncaught(format!("integer too large: {digits}")))?
                + 1)
            .to_string(),
        ),
        Py::Float(number) => Py::Float(*number + 1.0),
        Py::Bool(flag) => int(i64::from(*flag) + 1),
        other => {
            return Err(Uncaught(format!(
                "unsupported operand type(s) for +=: '{}' and 'int'",
                type_name(other)
            )));
        }
    };
    mutants.push(("assertion drift", false_count, rows.to_vec()));
    let mut failures = 0;
    for (label, mutant_contract, mutant_rows) in &mutants {
        let (problems, _) = validate(repository, mutant_contract, mutant_rows, light(None, None))?;
        if problems.is_empty() {
            output.err(&format!("{LABEL} self-test missed {label}\n"));
        } else {
            failures += 1;
        }
    }
    if failures != mutants.len() {
        return Ok(1);
    }

    let frozen_keys: Vec<String> = rows
        .iter()
        .map(|row| row_index(row, "document_key").map(str::to_string))
        .collect::<Checked<_>>()?;
    let live_declaration =
        (*repository.read_json_unguarded(&repository.path(POST_BOUNDARY_PATH))?).clone();
    let new_key = "zz_post_boundary_retained_specification".to_string();
    let declared = |keys: &[&str]| -> Checked<Py> {
        let mut block = live_declaration.clone();
        let entries = keys
            .iter()
            .map(|key| {
                Py::Dict(vec![
                    (pyjson::py_str("document_key"), s(key)),
                    (
                        pyjson::py_str("owed_relations"),
                        Py::List(
                            HELD_OUT_RELATIONS
                                .iter()
                                .map(|relation| s(relation))
                                .collect(),
                        ),
                    ),
                    (
                        pyjson::py_str("owing_leaf"),
                        s("SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii"),
                    ),
                ])
            })
            .collect();
        assign(&mut block, "unqualified_keys", Py::List(entries))?;
        Ok(block)
    };
    let with_new = || {
        let mut keys = frozen_keys.clone();
        keys.push(new_key.clone());
        keys
    };
    // Each boundary case carries the problem its own binding writes, and a declaration consistent with its retained
    // set, so that it fails with that problem alone (`.15.d.i`: two cases reused the live declaration, whose keys
    // lie outside their retained sets, and failed whether or not their own binding fired).
    let mut boundary_mutants: Vec<(&str, Vec<String>, Py, &str)> = vec![
        (
            "vanished frozen key",
            frozen_keys[1..].to_vec(),
            declared(&[])?,
            "behavioral population rows are no longer retained bundles",
        ),
        (
            "unreported post-boundary key",
            with_new(),
            declared(&[])?,
            "undeclared as unqualified residuals",
        ),
        (
            "declaration for a key that is not retained",
            frozen_keys.clone(),
            declared(&[&new_key])?,
            "is not a retained bundle",
        ),
        (
            "qualified key declared unqualified",
            with_new(),
            declared(&[&frozen_keys[0], &new_key])?,
            "cannot be declared unqualified",
        ),
    ];
    let mut no_leaf = declared(&[&new_key])?;
    pop_key(
        first_mut(subscript_mut(&mut no_leaf, "unqualified_keys")?)?,
        "owing_leaf",
    )?;
    boundary_mutants.push((
        "declaration without an owing leaf",
        with_new(),
        no_leaf,
        "owing_leaf must name the leaf",
    ));
    let mut bad_relation = declared(&[&new_key])?;
    assign(
        first_mut(subscript_mut(&mut bad_relation, "unqualified_keys")?)?,
        "owed_relations",
        Py::List(vec![s("reviewed_paraphrase")]),
    )?;
    boundary_mutants.push((
        "declaration owing an ineligible relation",
        with_new(),
        bad_relation,
        "owed_relations must be a distinct non-empty subset",
    ));
    for (label, retained, declaration, expected) in &boundary_mutants {
        let (problems, _) = validate(
            repository,
            contract,
            rows,
            light(Some(retained), Some(declaration)),
        )?;
        if !problems.iter().any(|problem| problem.contains(expected)) {
            output.err(&format!("{LABEL} self-test missed {label}\n"));
            return Ok(1);
        }
        if let Some(other) = problems.iter().find(|problem| !problem.contains(expected)) {
            output.err(&format!(
                "{LABEL} self-test caught {label} only beside another problem: {other}\n"
            ));
            return Ok(1);
        }
        failures += 1;
    }
    let golds = [
        "ihi0022_l_2025_08_amba_axi_protocol_specification",
        "ihi0024_e_2023_02_amba_5_apb_protocol_specification",
        "ihi0033_c_2021_09_amba_5_ahb_protocol_specification",
    ];
    let mut grown = frozen_keys.clone();
    grown.extend(golds.iter().map(|key| (*key).to_string()));
    grown.sort();
    let grown_declaration = declared(&golds)?;
    let (admissible_problems, admissible_metrics) = validate(
        repository,
        contract,
        rows,
        light(Some(&grown), Some(&grown_declaration)),
    )?;
    if !admissible_problems.is_empty() {
        output.err(&format!(
            "{LABEL} self-test rejected the mandated grown retained set:\n"
        ));
        for problem in &admissible_problems {
            output.err(&format!("  {problem}\n"));
        }
        return Ok(1);
    }
    if admissible_metrics.get("post_boundary_unqualified_documents") != Some(&(golds.len() as i64))
    {
        output.err(&format!(
            "{LABEL} self-test did not report the declared post-boundary residuals\n"
        ));
        return Ok(1);
    }
    let admitted = 1;

    let report =
        (*repository.read_json_unguarded(&repository.path(HELD_OUT_EVIDENCE_PATH))?).clone();
    let mut evidence_mutants: Vec<(&str, Py)> = Vec::new();
    let mut missing_attempt = report.clone();
    pop_last(subscript_mut(&mut missing_attempt, "attempts")?)?;
    evidence_mutants.push(("held-out attempt omission", missing_attempt));
    let mut overlapping = report.clone();
    let first_prospective = first_item(subscript(
        subscript(&overlapping, "split")?,
        "prospective_document_keys",
    )?)?;
    let split = subscript_mut(&mut overlapping, "split")?;
    assign(
        split,
        "overlapping_document_keys",
        Py::List(vec![first_prospective]),
    )?;
    assign(split, "identity_disjoint", Py::Bool(false))?;
    evidence_mutants.push(("held-out identity overlap", overlapping));
    let mut laundered = report.clone();
    let nonpass = find_mut(subscript_mut(&mut laundered, "attempts")?, |attempt| {
        Ok(!eq_str(Some(subscript(attempt, "state")?), "pass"))
    })?;
    assign(nonpass, "state", s("pass"))?;
    assign(nonpass, "failure_id", Py::None)?;
    evidence_mutants.push(("held-out state laundering", laundered));
    let mut bad_interval = report.clone();
    let measured = find_mut(subscript_mut(&mut bad_interval, "strata")?, |stratum| {
        positive(subscript(
            subscript(stratum, "uncertainty")?,
            "sample_size",
        )?)
    })?;
    assign(
        subscript_mut(measured, "uncertainty")?,
        "lower_parts_per_million",
        int(1_000_000),
    )?;
    evidence_mutants.push(("held-out uncertainty drift", bad_interval));
    let mut absolute = report.clone();
    assign(
        first_mut(subscript_mut(&mut absolute, "documents")?)?,
        "source_locator",
        s("/off-volume/source.pdf"),
    )?;
    evidence_mutants.push(("held-out absolute path", absolute));
    let mut bad_tool = report.clone();
    assign(&mut bad_tool, "tool_sha256", s(&"0".repeat(64)))?;
    evidence_mutants.push(("held-out tool drift", bad_tool));
    let mut bad_mode = report.clone();
    let full_capture = find_mut(subscript_mut(&mut bad_mode, "attempts")?, |attempt| {
        Ok(eq_str(
            Some(subscript(attempt, "relation")?),
            "unchanged_source",
        ))
    })?;
    assign(
        full_capture,
        "execution_mode",
        s("retained_report_revalidated"),
    )?;
    evidence_mutants.push(("held-out full-capture freshness drift", bad_mode));
    let mut missing_detail = report.clone();
    let noncompleted = find_mut(subscript_mut(&mut missing_detail, "attempts")?, |attempt| {
        in_set(
            Some(subscript(attempt, "state")?),
            &["unmeasurable", "invalid"],
        )
    })?;
    pop_key(noncompleted, "detail")?;
    evidence_mutants.push(("held-out disposition detail omission", missing_detail));
    let mut bad_digest = report.clone();
    assign(
        &mut bad_digest,
        "retained_evidence_sha256",
        s(&"0".repeat(63)),
    )?;
    evidence_mutants.push(("held-out retained evidence identity malformed", bad_digest));
    let mut evidence_failures = 0;
    for (label, mutant) in &evidence_mutants {
        let mut problems = Vec::new();
        validate_held_out_evidence(repository, contract, rows, &mut problems, Some(mutant))?;
        if problems.is_empty() {
            output.err(&format!("{LABEL} self-test missed {label}\n"));
        } else {
            evidence_failures += 1;
        }
    }
    if evidence_failures != evidence_mutants.len() {
        return Ok(1);
    }
    let total = failures + evidence_failures;
    let expected_total = mutants.len() + boundary_mutants.len() + evidence_mutants.len();
    output.out(&format!(
        "{LABEL} self-test: {total}/{expected_total} RED cases pass, with {admitted}/1 mandated retained-set growth \
         admitted\n"
    ));
    Ok(0)
}

// The command.

/// `--self-test`, or the check, as the shim forwards it.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut Terminal)
}

/// `run`, writing to `output` (`PRODUCTION-GENERICITY` captures it, `RUSTDOC-AND-XTASK.15.e`).
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn Output) -> i32 {
    let self_test = match arguments {
        [] => false,
        [flag] if flag == "--self-test" => true,
        _ => {
            output.err("Usage: cargo xtask check behavioral-genericity-contract [--self-test]\n");
            return 2;
        }
    };
    let mut repository = Repository::new(root);
    match command(&mut repository, self_test, output) {
        Ok(code) => code,
        Err(Uncaught(reason)) => {
            output.err(&format!("{LABEL}: refused: {reason}\n"));
            1
        }
    }
}

fn command(repository: &mut Repository, self_test: bool, output: &mut dyn Output) -> Checked<i32> {
    let contract = match repository.read_json(&repository.path(CONTRACT_PATH)) {
        Ok(contract) => contract,
        Err(error) => {
            output.err(&format!("{LABEL}: {}\n", error.message()));
            return Ok(1);
        }
    };
    let rows = match read_population(&repository.path(POPULATION_PATH))? {
        Ok(rows) => rows,
        Err(error) => {
            output.err(&format!("{LABEL}: {error}\n"));
            return Ok(1);
        }
    };
    if !matches!(*contract, Py::Dict(_)) {
        output.err(&format!("{LABEL}: contract must be an object\n"));
        return Ok(1);
    }
    if self_test {
        return run_self_test(repository, &contract, &rows, output);
    }
    let (problems, metrics) = validate(repository, &contract, &rows, FULL)?;
    if !problems.is_empty() {
        for problem in &problems {
            output.err(&format!("{LABEL}: {problem}\n"));
        }
        return Ok(1);
    }
    let metric = |name: &str| metrics.get(name).copied().unwrap_or(0);
    output.out(&format!(
        "{LABEL}: {} current documents; {} text-measurable / {} text-unmeasurable; {} reviewed calibration / {} \
         prospective holdout; {} vendor-novel / {} family-novel; {} live source authorities verified / {} external \
         unavailable; {} retained post-boundary and unqualified\n",
        metric("current_documents"),
        metric("text_projection_measurable_documents"),
        metric("text_projection_unmeasurable_documents"),
        metric("reviewed_current_overlap"),
        metric("prospective_holdout_documents"),
        metric("prospective_vendor_novel_documents"),
        metric("prospective_family_novel_documents"),
        metric("source_authorities_verified"),
        metric("external_authorities_unavailable"),
        metric("post_boundary_unqualified_documents")
    ));
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dict(pairs: &[(&str, Py)]) -> Py {
        Py::Dict(
            pairs
                .iter()
                .map(|(key, value)| (pyjson::py_str(key), value.clone()))
                .collect(),
        )
    }

    fn same(left: Checked<Py>, right: Py) -> bool {
        left.is_ok_and(|left| pyjson::repr(&left) == pyjson::repr(&right))
    }

    #[test]
    fn the_real_contract_holds_and_the_self_test_refuses_every_mutation() {
        let root = crate::repository_root();
        let mut repository = Repository::new(&root);
        let mut output = crate::checks::Captured::default();
        assert_eq!(command(&mut repository, false, &mut output).ok(), Some(0));
        assert!(
            output
                .text
                .ends_with(" retained post-boundary and unqualified\n")
        );
        let mut output = crate::checks::Captured::default();
        assert_eq!(command(&mut repository, true, &mut output).ok(), Some(0));
        assert!(output.text.contains(
            " self-test: 23/23 RED cases pass, with 1/1 mandated retained-set growth admitted"
        ));
    }

    #[test]
    fn the_script_short_circuits_and_counts_hold_on_the_real_authorities() {
        let root = crate::repository_root();
        let mut repository = Repository::new(&root);
        let contract = (*repository
            .read_json_unguarded(&repository.path(CONTRACT_PATH))
            .expect("contract"))
        .clone();
        let rows = read_population(&repository.path(POPULATION_PATH))
            .expect("read")
            .expect("rows");
        let report = (*repository
            .read_json_unguarded(&repository.path(HELD_OUT_EVIDENCE_PATH))
            .expect("report"))
        .clone();
        let held_out = |repository: &mut Repository, rows: &[Row], report: &Py| {
            let mut problems = Vec::new();
            validate_held_out_evidence(repository, &contract, rows, &mut problems, Some(report))
                .map(|()| problems)
        };
        // A repeated prospective row counts once among the completed documents (a dict there) and twice among the
        // unmeasurable ones (a list).
        let mut repeated = rows.clone();
        let first = repeated
            .iter()
            .position(|row| row_get(row, "review_role") == Some("prospective_holdout"))
            .expect("a prospective row");
        repeated.insert(first, repeated[first].clone());
        let problems = held_out(&mut repository, &repeated, &report).expect("no refusal");
        assert!(
            problems
                .iter()
                .any(|problem| problem.contains("coverage unmeasurable_documents differs"))
        );
        assert!(
            !problems
                .iter()
                .any(|problem| problem.contains("partially_completed_documents"))
        );
        assert!(
            !problems
                .iter()
                .any(|problem| problem.contains("fully_completed_documents"))
        );
        // An unknown key ends the identity test before the relation is hashed; a known one does not.
        let mut unknown = report.clone();
        let attempt = first_mut(subscript_mut(&mut unknown, "attempts").expect("attempts"))
            .expect("an attempt");
        assign(attempt, "document_key", s("zz")).expect("assigned");
        assign(attempt, "relation", Py::List(Vec::new())).expect("assigned");
        let problems = held_out(&mut repository, &rows, &unknown).expect("no refusal");
        assert!(
            problems
                .iter()
                .any(|problem| problem.contains("identity is outside the matrix: ('zz', [])"))
        );
        let mut known = report.clone();
        let attempt = first_mut(subscript_mut(&mut known, "attempts").expect("attempts"))
            .expect("an attempt");
        assign(attempt, "relation", Py::List(Vec::new())).expect("assigned");
        assert!(held_out(&mut repository, &rows, &known).is_err());
        // The counts call `attempt.get` on a malformed attempt the loop skipped.
        let mut malformed = report.clone();
        if let Ok(Py::List(attempts)) = subscript_mut(&mut malformed, "attempts") {
            attempts.push(s("x"));
        }
        assert!(held_out(&mut repository, &rows, &malformed).is_err());
        // An input plane's id is only hashed once its relations are a list.
        let mut plane = contract.clone();
        let first_plane =
            first_mut(subscript_mut(&mut plane, "input_planes").expect("planes")).expect("a plane");
        assign(first_plane, "relations", s("x")).expect("assigned");
        assign(first_plane, "plane_id", Py::List(vec![s("l")])).expect("assigned");
        let (problems, _) =
            validate(&mut repository, &plane, &rows, light(None, None)).expect("no refusal");
        assert!(
            problems
                .iter()
                .any(|problem| problem == "input plane ['l'] has an incomplete relation partition")
        );
        assign(
            first_mut(subscript_mut(&mut plane, "input_planes").expect("planes")).expect("a plane"),
            "relations",
            Py::List(Vec::new()),
        )
        .expect("assigned");
        assert!(validate(&mut repository, &plane, &rows, light(None, None)).is_err());
    }

    #[test]
    fn sums_add_as_cpython_adds() {
        assert!(same(py_sum([int(1), Py::Bool(true)]), int(2)));
        assert!(same(
            py_sum([int(i64::MAX), int(1)]),
            Py::Int("9223372036854775808".to_string())
        ));
        assert!(same(py_sum([int(1), Py::Float(0.5)]), Py::Float(1.5)));
        assert!(py_sum([int(1), s("x")]).is_err());
        // Neumaier: the two ones survive beside 1e16, where adding left to right loses them.
        let terms = [1.0, 1e16, 0.0, 1.0, 0.0, 0.0].map(Py::Float);
        assert!(same(py_sum(terms), Py::Float(1.000_000_000_000_000_2e16)));
        // Summed per record first, they are lost, as the script's nested `sum` lost them.
        let first = dict(&[
            ("x", Py::Float(1.0)),
            ("y", Py::Float(1e16)),
            ("z", Py::Float(0.0)),
        ]);
        let second = dict(&[("x", Py::Float(1.0))]);
        assert!(same(
            coverage_nested_total(&[&first, &second], &["x", "y", "z"]),
            Py::Float(1e16)
        ));
        assert!(same(
            coverage_total(&[&first, &second], "x"),
            Py::Float(2.0)
        ));
    }

    #[test]
    fn intervals_round_half_to_even() {
        assert_eq!(wilson_parts_per_million(0, 0), None);
        // 1/128 is 7812.5 parts per million: Python's `round` gives 7812.
        assert_eq!(wilson_parts_per_million(1, 128), Some((7812, 1380, 42926)));
        assert_eq!(
            wilson_parts_per_million(15, 17),
            Some((882_353, 656_636, 967_120))
        );
    }

    #[test]
    fn reprs_and_orders_are_pythons() {
        assert_eq!(tuple_repr(&[Some(&s("a")), None]), "('a', None)");
        assert_eq!(list_repr(&["a", "b'c"]), "['a', \"b'c\"]");
        assert_eq!(
            sorted_repr(&[s("b"), s("a")]).ok().as_deref(),
            Some("['a', 'b']")
        );
        assert_eq!(
            sorted_repr(&[int(3), Py::Float(1.5), Py::Bool(true)])
                .ok()
                .as_deref(),
            Some("[True, 1.5, 3]")
        );
        assert_eq!(sorted_repr(&[Py::None]).ok().as_deref(), Some("[None]"));
        assert!(sorted_repr(&[int(1), s("a")]).is_err());
    }

    #[test]
    fn paths_normalize_as_pure_posix_path_does() {
        assert_eq!(PurePath::new("a/./b//c/").as_posix(), "a/b/c");
        assert_eq!(PurePath::new("//x").as_posix(), "//x");
        assert_eq!(PurePath::new("///x").as_posix(), "/x");
        assert_eq!(PurePath::new("").as_posix(), ".");
        assert_eq!(PurePath::new("a/").name(), "a");
        assert!(
            PurePath::new("d/x/reviewed_recipes/y.json")
                .is_relative_to(&PurePath::new("d/x/reviewed_recipes"))
        );
        assert!(!PurePath::new("/d/x").is_relative_to(&PurePath::new("d")));
        assert!(safe_relative_path(Some(&s("a/../b"))).is_none());
    }

    #[test]
    fn text_decodes_as_python_reads_it() {
        let error =
            |bytes: &[u8]| utf8_error(bytes, &std::str::from_utf8(bytes).expect_err("invalid"));
        assert_eq!(
            error(b"ab\xff"),
            "'utf-8' codec can't decode byte 0xff in position 2: invalid start byte"
        );
        assert_eq!(
            error(b"\xe2\x82x"),
            "'utf-8' codec can't decode bytes in position 0-1: invalid continuation byte"
        );
        assert_eq!(
            error(b"\xe2("),
            "'utf-8' codec can't decode byte 0xe2 in position 0: invalid continuation byte"
        );
        assert_eq!(
            error(b"a\xe2\x82"),
            "'utf-8' codec can't decode bytes in position 1-2: unexpected end of data"
        );
        let mut late = vec![b'x'; 9000];
        late.push(0xff);
        assert!(
            matches!(decode_chunked(&late), Err((_, 1, message)) if message.contains("position 808"))
        );
        late[100] = 0xff;
        assert!(
            matches!(decode_chunked(&late), Err((_, 0, message)) if message.contains("position 100"))
        );
    }

    #[test]
    fn integers_parse_as_int_parses() {
        let parsed = |text: &str| python_int(text).ok().map(|value| pyjson::repr(&value));
        assert_eq!(parsed(" 1_0 ").as_deref(), Some("10"));
        assert_eq!(parsed("+7").as_deref(), Some("7"));
        assert_eq!(parsed("-0").as_deref(), Some("0"));
        assert_eq!(parsed("007").as_deref(), Some("7"));
        assert_eq!(parsed("_1"), None);
        assert_eq!(parsed("1__0"), None);
        assert_eq!(parsed(""), None);
    }

    #[test]
    fn the_population_reads_as_dict_reader_reads_it() {
        assert_eq!(
            csv_records("a\tb\n\"x\"\"y\"\t\n\n\"q\"r\ts\"t").ok(),
            Some(vec![
                vec!["a".to_string(), "b".to_string()],
                vec!["x\"y".to_string(), String::new()],
                vec![],
                vec!["qr".to_string(), "s\"t".to_string()],
            ])
        );
        assert!(csv_records(&"v".repeat(CSV_FIELD_LIMIT)).is_ok());
        assert!(csv_records(&"v".repeat(CSV_FIELD_LIMIT + 1)).is_err());
        let scratch =
            crate::repository_root().join(".project-data/tmp/xtask-behavioral-genericity-contract");
        fs::create_dir_all(&scratch).expect("the scratch directory");
        let header = POPULATION_FIELDS.join("\t");
        let read = |name: &str, bytes: &[u8]| {
            let path = scratch.join(name);
            fs::write(&path, bytes).expect("a fixture");
            read_population(&path).ok().and_then(Result::err)
        };
        let expected = list_repr(&POPULATION_FIELDS);
        assert_eq!(
            read("blank.tsv", format!("\n{header}\n").as_bytes()),
            Some(format!("population header differs: [] != {expected}"))
        );
        assert_eq!(
            read("empty.tsv", b""),
            Some(format!("population header differs: None != {expected}"))
        );
        let mut late = format!("{header}\n").into_bytes();
        late.resize(9000, b'x');
        late.push(0xff);
        assert_eq!(
            read("late.tsv", &late),
            Some(
                "'utf-8' codec can't decode byte 0xff in position 808: invalid start byte"
                    .to_string()
            )
        );
        fs::remove_dir_all(&scratch).expect("the scratch directory removed");
    }
}
