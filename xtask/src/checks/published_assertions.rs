//! The `PUBLISHED-ASSERTIONS` doctrine (`CLAIM-VERIFICATION-ADOPTION.7.1`), native since `RUSTDOC-AND-XTASK.13.e`:
//! every published value in a claim-annotated region of a governed surface is listed by an assertion record in
//! `doctrine/claim_verification/published_assertions.jsonl` with one of four outcomes — `derived` (its producer is
//! re-executed and the named field must still equal it), `gated` (a control with a known-bad case bound to an exact
//! source region), `authored` (a tracked decision) or `dated` (a resolving revision) — and no two surfaces publish
//! different values for one producer field. The governed population is derived on every run from the live-document
//! surface registry, never stored.
//!
//! The registered script, `scripts/check_published_assertions.pl`, keeps its argument handling and root resolution
//! and passes this check `--root DIR --contract PATH` and one of `--check`, `--report`, `--produce` or
//! `--self-test`. The check reproduces the script's Perl semantics wherever its output shows them: values as
//! `JSON::PP` decodes and Perl stringifies them, byte lengths as `encode_utf8` of a raw byte string counts them,
//! and the numeral grammar's backtracking.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Encoder, Py};

const LABEL: &str = "published-assertions";
const HARD_RECORDS: f64 = 512.0;
const HARD_BYTES: u64 = 262_144;
const HARD_RECORD_BYTES: usize = 8_192;

/// Run the check with the arguments the shim normalized.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    let mut mode = "--check";
    let mut base = None;
    let mut contract = "doctrine/claim_verification/published_assertions.jsonl".to_string();
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" | "--report" | "--produce" | "--self-test" => mode = argument.as_str(),
            "--root" => base = arguments.next().map(PathBuf::from),
            "--contract" => match arguments.next() {
                Some(path) => contract = path.clone(),
                None => return usage(),
            },
            _ => return usage(),
        }
    }
    if mode == "--self-test" {
        return self_test(root);
    }
    let Some(base) = base else {
        return usage();
    };
    let result = validate_registry(&base, &contract, true);
    if !result.errors.is_empty() {
        for error in &result.errors {
            eprintln!("{LABEL}: {error}");
        }
        eprintln!("{LABEL}: FAILED with {} violation(s).", result.errors.len());
        return 1;
    }
    let summary = result.summary.unwrap_or_default();
    match mode {
        "--produce" => {
            for value in &result.unlisted {
                println!(
                    "{{\"line\":{},\"path\":{},\"value\":{}}}",
                    value.line,
                    json_pp_string(&value.path),
                    json_pp_string(&value.value)
                );
            }
        }
        "--report" => println!(
            "{{\"assertions\":{},\"exempt_files\":{},\"governed_files\":{},\"governed_regions\":{},\"outcomes\":{{\"authored\":{},\"dated\":{},\"derived\":{},\"gated\":{}}},\"phase\":{},\"unlisted\":{}}}",
            summary.assertions,
            summary.exempt_files,
            summary.governed_files,
            summary.governed_regions,
            summary.outcomes["authored"],
            summary.outcomes["dated"],
            summary.outcomes["derived"],
            summary.outcomes["gated"],
            json_pp_string(&summary.phase),
            summary.unlisted
        ),
        _ => eprintln!(
            "{LABEL}: {} assertion(s) — {} derived, {} gated, {} authored, {} dated — over {} governed region(s) in {} governed and {} exempt claim-annotated file(s), '{}' phase, {} unlisted value(s).",
            summary.assertions,
            summary.outcomes["derived"],
            summary.outcomes["gated"],
            summary.outcomes["authored"],
            summary.outcomes["dated"],
            summary.governed_regions,
            summary.governed_files,
            summary.exempt_files,
            summary.phase,
            summary.unlisted
        ),
    }
    0
}

fn usage() -> i32 {
    eprintln!(
        "Usage: cargo xtask check published-assertions --root DIR [--contract PATH] [--check|--report|--produce|--self-test]"
    );
    2
}

/// A string as `JSON::PP` encodes it: `"` and `\` escaped, control characters as named or `\u00XX` escapes.
fn json_pp_string(text: &str) -> String {
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
            character if (character as u32) < 0x20 => {
                out.push_str(&format!("\\u{:04x}", character as u32))
            }
            character => out.push(character),
        }
    }
    out.push('"');
    out
}

/// What a run found.
#[derive(Debug, Default)]
pub struct Outcome {
    /// Every violation, in the order the script reports them.
    pub errors: Vec<String>,
    /// The counts the script reports; absent when the contract has no records.
    pub summary: Option<Summary>,
    /// The published values no record lists.
    pub unlisted: Vec<Unlisted>,
}

/// The script's summary.
#[derive(Debug, Default)]
pub struct Summary {
    /// Assertion records that validated.
    pub assertions: usize,
    /// Their outcomes, counted.
    pub outcomes: BTreeMap<&'static str, usize>,
    /// Claim-annotated paragraphs in governed files.
    pub governed_regions: usize,
    /// Claim-annotated files on governed surfaces.
    pub governed_files: usize,
    /// Claim-annotated files on exempt surfaces.
    pub exempt_files: usize,
    /// Published values no record lists.
    pub unlisted: usize,
    /// The registry's phase.
    pub phase: String,
}

/// A published value no record lists.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unlisted {
    /// Its file.
    pub path: String,
    /// Its line, from 1.
    pub line: usize,
    /// The numeral as published.
    pub value: String,
}

struct Assertion {
    id: String,
    path: String,
    value: Option<String>,
    outcome: String,
    region: Option<Py>,
    producer: Option<Py>,
}

fn get<'a>(record: &'a Py, key: &str) -> Option<&'a Py> {
    pyjson::get(record, key).filter(|value| !matches!(value, Py::None))
}

fn absolute(base: &Path, relative: &str) -> PathBuf {
    if Path::new(relative).is_absolute() {
        PathBuf::from(relative)
    } else {
        base.join(relative)
    }
}

fn is_regular(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_file())
}

fn read_raw(path: &Path, errors: Option<&mut Vec<String>>, label: &str) -> Vec<u8> {
    match fs::read(path) {
        Ok(bytes) => bytes,
        Err(error) => {
            if let Some(errors) = errors {
                errors.push(format!("{label} cannot be read: {}", perl_os_error(&error)));
            }
            Vec::new()
        }
    }
}

/// An I/O error as Perl's `$!` prints it.
fn perl_os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn reject_unknown(record: &Py, label: &str, errors: &mut Vec<String>, allowed: &[&str]) {
    let Py::Dict(pairs) = record else {
        return;
    };
    let mut keys: Vec<String> = pairs.iter().map(|(key, _)| pyjson::to_text(key)).collect();
    keys.sort();
    for key in keys {
        if !allowed.contains(&key.as_str()) {
            errors.push(format!("{label} has unknown field '{key}'"));
        }
    }
}

fn required_scalar(
    record: &Py,
    key: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    let value = get(record, key);
    let text = match value {
        Some(value) if !perl::is_reference(value) => perl::scalar(Some(value)),
        _ => None,
    };
    match text {
        Some(text) if !text.is_empty() => Some(text),
        _ => {
            errors.push(format!("{label} lacks a nonempty '{key}'"));
            None
        }
    }
}

fn required_slug(record: &Py, key: &str, label: &str, errors: &mut Vec<String>) -> Option<String> {
    let value = required_scalar(record, key, label, errors)?;
    let mut parts = value.split('-');
    let slug = parts.all(|part| {
        !part.is_empty()
            && part
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    });
    if !slug {
        errors.push(format!(
            "{label} '{key}' must be a lowercase hyphenated slug"
        ));
        return None;
    }
    Some(value)
}

fn exact_scalar(value: Option<&Py>, want: &str, label: &str, errors: &mut Vec<String>) {
    let text = match value {
        Some(value) if !perl::is_reference(value) => perl::scalar(Some(value)),
        _ => None,
    };
    if text.as_deref() != Some(want) {
        errors.push(format!("{label} must be '{want}'"));
    }
}

fn positive_integer(value: Option<&Py>, label: &str, errors: &mut Vec<String>) -> Option<f64> {
    let text = match value {
        Some(value) if !perl::is_reference(value) => perl::scalar(Some(value)),
        _ => None,
    };
    match text {
        Some(text)
            if text.starts_with(|first: char| ('1'..='9').contains(&first))
                && text.bytes().all(|byte| byte.is_ascii_digit()) =>
        {
            Some(text.parse().unwrap_or(f64::MAX))
        }
        _ => {
            errors.push(format!("{label} must be a positive integer"));
            None
        }
    }
}

fn scalar_array(value: Option<&Py>, label: &str, errors: &mut Vec<String>) -> Vec<String> {
    let Some(Py::List(items)) = value else {
        errors.push(format!("{label} must be an array"));
        return Vec::new();
    };
    let mut out = Vec::new();
    for item in items {
        if matches!(item, Py::None) || perl::is_reference(item) {
            errors.push(format!("{label} must contain only scalars"));
            continue;
        }
        out.push(perl::scalar(Some(item)).unwrap_or_default());
    }
    out
}

/// `split /\n/, $raw, -1`: every piece, the empty one after a final newline included.
fn split_keeping_trailing(raw: &[u8]) -> Vec<&[u8]> {
    if raw.is_empty() {
        return Vec::new();
    }
    raw.split(|byte| *byte == b'\n').collect()
}

/// `split /\n/, $raw`: the pieces, trailing empty ones dropped.
fn split_dropping_trailing(raw: &[u8]) -> Vec<&[u8]> {
    let mut pieces = split_keeping_trailing(raw);
    while pieces.last().is_some_and(|piece| piece.is_empty()) {
        pieces.pop();
    }
    pieces
}

/// `split_lines`: each line with its newline, a last one without.
fn split_lines(raw: &[u8]) -> Vec<&[u8]> {
    raw.split_inclusive(|byte| *byte == b'\n').collect()
}

fn decode(line: &[u8]) -> Option<Py> {
    pyjson::loads_strict(&String::from_utf8_lossy(line))
        .ok()
        .filter(|value| matches!(value, Py::Dict(_)))
}

fn has_non_space(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .any(|byte| !matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c))
}

fn read_bounded_jsonl(path: &Path, label: &str, errors: &mut Vec<String>) -> (Option<Py>, Vec<Py>) {
    if !is_regular(path) {
        errors.push(format!("{label} is missing or not a regular file"));
        return (None, Vec::new());
    }
    let file_bytes = fs::metadata(path).map_or(0, |metadata| metadata.len());
    if file_bytes > HARD_BYTES {
        errors.push(format!(
            "{label} exceeds portable hard byte cap {HARD_BYTES}"
        ));
    }
    let raw = read_raw(path, Some(errors), label);
    let (mut records, mut lengths) = (Vec::new(), Vec::new());
    for (index, line) in split_keeping_trailing(&raw).into_iter().enumerate() {
        let number = index + 1;
        if line.is_empty() && number > 1 {
            continue;
        }
        if line.is_empty() {
            errors.push(format!("{label} line {number} is blank"));
            continue;
        }
        if perl::utf8_length(line) > HARD_RECORD_BYTES {
            errors.push(format!(
                "{label} line {number} exceeds portable raw-record cap {HARD_RECORD_BYTES}"
            ));
        }
        let Some(record) = decode(line) else {
            errors.push(format!("{label} line {number} is not one JSON object"));
            continue;
        };
        records.push(record);
        lengths.push(perl::utf8_length(line));
    }
    if records.is_empty() {
        return (None, Vec::new());
    }
    let meta = records.remove(0);
    let cap = |name: &str, errors: &mut Vec<String>| {
        positive_integer(get(&meta, name), &format!("{label} {name}"), errors)
    };
    let max_records = cap("max_records", errors);
    let max_bytes = cap("max_bytes", errors);
    let max_record_bytes = cap("max_record_bytes", errors);
    let max_array = cap("max_array_items", errors);
    let max_scalar = cap("max_scalar_bytes", errors);
    if max_records.is_some_and(|cap| cap > HARD_RECORDS) {
        errors.push(format!("{label} max_records exceeds portable hard cap 512"));
    }
    if max_bytes.is_some_and(|cap| cap > HARD_BYTES as f64) {
        errors.push(format!(
            "{label} max_bytes exceeds portable hard cap {HARD_BYTES}"
        ));
    }
    if max_record_bytes.is_some_and(|cap| cap > HARD_RECORD_BYTES as f64) {
        errors.push(format!(
            "{label} max_record_bytes exceeds portable hard cap {HARD_RECORD_BYTES}"
        ));
    }
    if max_array.is_some_and(|cap| cap > 64.0) {
        errors.push(format!(
            "{label} max_array_items exceeds portable hard cap 64"
        ));
    }
    if max_scalar.is_some_and(|cap| cap > 2048.0) {
        errors.push(format!(
            "{label} max_scalar_bytes exceeds portable hard cap 2048"
        ));
    }
    if max_records.is_some_and(|cap| records.len() as f64 > cap) {
        errors.push(format!("{label} has more data records than max_records"));
    }
    if max_bytes.is_some_and(|cap| file_bytes as f64 > cap) {
        errors.push(format!("{label} exceeds declared max_bytes"));
    }
    if max_record_bytes.is_some_and(|cap| lengths.iter().any(|length| *length as f64 > cap)) {
        errors.push(format!("{label} has a record above max_record_bytes"));
    }
    if let (Some(max_array), Some(max_scalar)) = (max_array, max_scalar) {
        value_bounds(
            &meta,
            &format!("{label} registry record"),
            max_array,
            max_scalar,
            errors,
        );
        for record in &records {
            value_bounds(record, label, max_array, max_scalar, errors);
        }
    }
    (Some(meta), records)
}

fn value_bounds(
    value: &Py,
    label: &str,
    max_array: f64,
    max_scalar: f64,
    errors: &mut Vec<String>,
) {
    match value {
        Py::Dict(pairs) => {
            let mut keyed: Vec<(String, &Py)> = pairs
                .iter()
                .map(|(key, item)| (pyjson::to_text(key), item))
                .collect();
            keyed.sort_by(|left, right| left.0.cmp(&right.0));
            for (key, item) in keyed {
                value_bounds(
                    item,
                    &format!("{label}.{key}"),
                    max_array,
                    max_scalar,
                    errors,
                );
            }
        }
        Py::List(items) => {
            if items.len() as f64 > max_array {
                errors.push(format!("{label} array exceeds {max_array} items"));
            }
            for item in items {
                value_bounds(item, label, max_array, max_scalar, errors);
            }
        }
        Py::Bool(_) | Py::None => {}
        scalar => {
            let text = perl::scalar(Some(scalar)).unwrap_or_default();
            if perl::utf8_length(text.as_bytes()) as f64 > max_scalar {
                errors.push(format!("{label} scalar exceeds {max_scalar} bytes"));
            }
        }
    }
}

/// The run the gate makes: the registry validated against the tree under `base`, producers executed when `execute`.
pub fn validate_registry(base: &Path, contract: &str, execute: bool) -> Outcome {
    let mut errors = Vec::new();
    let (meta, records) = read_bounded_jsonl(
        &absolute(base, contract),
        &format!("{LABEL} contract"),
        &mut errors,
    );
    let Some(meta) = meta else {
        return Outcome {
            errors,
            summary: None,
            unlisted: Vec::new(),
        };
    };
    reject_unknown(
        &meta,
        &format!("{LABEL} registry record"),
        &mut errors,
        &[
            "record_type",
            "schema_version",
            "phase",
            "max_records",
            "max_bytes",
            "max_record_bytes",
            "max_array_items",
            "max_scalar_bytes",
            "milestones",
        ],
    );
    exact_scalar(
        get(&meta, "record_type"),
        "registry",
        &format!("{LABEL} registry record_type"),
        &mut errors,
    );
    exact_scalar(
        get(&meta, "schema_version"),
        "1",
        &format!("{LABEL} registry schema_version"),
        &mut errors,
    );
    let phase = perl::scalar(get(&meta, "phase")).unwrap_or_default();
    if phase != "inventory" && phase != "frozen" {
        errors.push(format!(
            "{LABEL} registry phase must be 'inventory' or 'frozen'"
        ));
    }
    let tracked = tracked_paths(base, &mut errors);
    let mut assertions: Vec<Assertion> = Vec::new();
    let mut ids = BTreeSet::new();
    let mut by_field: BTreeMap<String, Vec<String>> = BTreeMap::new();
    let mut disposition: BTreeMap<String, String> = BTreeMap::new();
    let mut surface_source = None;
    for record in &records {
        let kind = perl::scalar(get(record, "record_type")).unwrap_or_default();
        match kind.as_str() {
            "source" => {
                surface_source = validate_surface_source(base, record, &tracked, &mut errors)
            }
            "surface_disposition" => {
                let Some((sid, chosen)) = validate_disposition(record, &mut errors) else {
                    continue;
                };
                if disposition.contains_key(&sid) {
                    errors.push(format!(
                        "{LABEL} surface_disposition '{sid}' is declared twice"
                    ));
                    continue;
                }
                disposition.insert(sid, chosen);
            }
            "assertion" => {
                let Some(assertion) =
                    validate_assertion(base, record, &tracked, execute, &mut errors)
                else {
                    continue;
                };
                if !ids.insert(assertion.id.clone()) {
                    errors.push(format!(
                        "{LABEL} assertion_id '{}' is duplicated",
                        assertion.id
                    ));
                }
                if assertion.outcome == "derived" {
                    let producer = assertion.producer.as_ref();
                    let mut parts: Vec<String> =
                        match producer.and_then(|producer| get(producer, "argv")) {
                            Some(Py::List(items)) => items
                                .iter()
                                .map(|item| perl::scalar(Some(item)).unwrap_or_default())
                                .collect(),
                            _ => Vec::new(),
                        };
                    parts.push(
                        perl::scalar(producer.and_then(|producer| get(producer, "field")))
                            .unwrap_or_default(),
                    );
                    by_field
                        .entry(parts.join("\0"))
                        .or_default()
                        .push(assertion.value.clone().unwrap_or_default());
                }
                assertions.push(assertion);
            }
            _ => errors.push(format!("{LABEL} record_type '{kind}' is unknown")),
        }
    }
    for (key, values) in &by_field {
        let distinct: BTreeSet<&String> = values.iter().collect();
        if distinct.len() > 1 {
            errors.push(format!(
                "{LABEL} cross-surface disagreement on '{}': {}",
                key.replace('\0', " "),
                distinct
                    .iter()
                    .map(|value| format!("'{value}'"))
                    .collect::<Vec<_>>()
                    .join(" vs ")
            ));
        }
    }
    let surfaces = load_surface_map(base, surface_source.as_deref(), &mut errors);
    let (claim_files, unclassified) =
        classify_claim_files(base, &tracked, surfaces.as_ref(), &disposition);
    errors.extend(unclassified);
    if let Some(surfaces) = &surfaces {
        for sid in disposition.keys() {
            if !surfaces.ids.contains(sid) {
                errors.push(format!("{LABEL} surface_disposition '{sid}' names no surface in the live-document registry"));
            }
        }
    }
    let files_with = |wanted: &str| -> Vec<String> {
        claim_files
            .iter()
            .filter(|(_, sid)| disposition.get(*sid).map(String::as_str) == Some(wanted))
            .map(|(path, _)| path.clone())
            .collect()
    };
    let governed_files = files_with("governed");
    let exempt_files = files_with("exempt");
    let (regions, unlisted) = governed_coverage(base, &governed_files, &assertions);
    if phase == "frozen" {
        for value in &unlisted {
            errors.push(format!(
                "{LABEL} published value '{}' at {}:{} sits in a claim-annotated region that no assertion record lists",
                value.value, value.path, value.line
            ));
        }
    }
    let mut outcomes =
        BTreeMap::from([("derived", 0), ("gated", 0), ("authored", 0), ("dated", 0)]);
    for assertion in &assertions {
        if let Some(count) = outcomes.get_mut(assertion.outcome.as_str()) {
            *count += 1;
        }
    }
    let summary = Summary {
        assertions: assertions.len(),
        outcomes,
        governed_regions: regions,
        governed_files: governed_files.len(),
        exempt_files: exempt_files.len(),
        unlisted: unlisted.len(),
        phase,
    };
    Outcome {
        errors,
        summary: Some(summary),
        unlisted,
    }
}

fn validate_disposition(record: &Py, errors: &mut Vec<String>) -> Option<(String, String)> {
    reject_unknown(
        record,
        &format!("{LABEL} surface_disposition"),
        errors,
        &[
            "record_type",
            "schema_version",
            "surface_id",
            "disposition",
            "reason",
        ],
    );
    exact_scalar(
        get(record, "schema_version"),
        "1",
        &format!("{LABEL} surface_disposition schema_version"),
        errors,
    );
    let sid = required_scalar(
        record,
        "surface_id",
        &format!("{LABEL} surface_disposition"),
        errors,
    )?;
    let snake = sid.starts_with(|first: char| first.is_ascii_lowercase() || first.is_ascii_digit())
        && sid
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_');
    if !snake {
        errors.push(format!("{LABEL} surface_disposition surface_id '{sid}' is not a lowercase snake_case identifier"));
        return None;
    }
    let chosen = required_scalar(
        record,
        "disposition",
        &format!("{LABEL} surface_disposition '{sid}'"),
        errors,
    )
    .unwrap_or_default();
    if chosen != "governed" && chosen != "exempt" {
        errors.push(format!("{LABEL} surface_disposition '{sid}' disposition '{chosen}' is not 'governed' or 'exempt'"));
    }
    let reason = get(record, "reason");
    if chosen == "exempt" {
        let stated = reason.is_some_and(|reason| {
            !perl::is_reference(reason)
                && has_non_space(perl::scalar(Some(reason)).unwrap_or_default().as_bytes())
        });
        if !stated {
            errors.push(format!(
                "{LABEL} surface_disposition '{sid}' is exempt and must state a reason"
            ));
        }
    } else if reason.is_some() {
        errors.push(format!(
            "{LABEL} surface_disposition '{sid}' is governed and must not state an exemption reason"
        ));
    }
    Some((sid, chosen))
}

fn validate_surface_source(
    base: &Path,
    record: &Py,
    tracked: &BTreeSet<String>,
    errors: &mut Vec<String>,
) -> Option<String> {
    reject_unknown(
        record,
        &format!("{LABEL} source"),
        errors,
        &[
            "record_type",
            "schema_version",
            "source_id",
            "path",
            "sha256",
        ],
    );
    exact_scalar(
        get(record, "schema_version"),
        "1",
        &format!("{LABEL} source schema_version"),
        errors,
    );
    exact_scalar(
        get(record, "source_id"),
        "surface_registry",
        &format!("{LABEL} source source_id"),
        errors,
    );
    let path = required_scalar(record, "path", &format!("{LABEL} source"), errors)?;
    if !tracked.contains(&path) {
        errors.push(format!(
            "{LABEL} source path '{path}' is not repository-tracked"
        ));
    }
    let file = absolute(base, &path);
    if !is_regular(&file) {
        errors.push(format!(
            "{LABEL} source path '{path}' is missing or not a regular file"
        ));
        return None;
    }
    let want = match get(record, "sha256") {
        Some(Py::Str(text)) => pyjson::to_text(text),
        Some(other) if !perl::is_reference(other) => perl::scalar(Some(other)).unwrap_or_default(),
        _ => String::new(),
    };
    if !is_sha256(&want) {
        errors.push(format!(
            "{LABEL} source sha256 must be 64 lowercase hex characters"
        ));
        return None;
    }
    let actual = sha256_hex(&read_raw(&file, Some(errors), &format!("{LABEL} source")));
    if actual != want {
        errors.push(format!(
            "{LABEL} source '{path}' is stale: SHA-256 {actual} != {want}"
        ));
    }
    Some(path)
}

fn is_sha256(text: &str) -> bool {
    text.len() == 64
        && text
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

struct Surfaces {
    ids: BTreeSet<String>,
    rules: Vec<(String, Vec<Glob>)>,
}

fn load_surface_map(
    base: &Path,
    source: Option<&str>,
    errors: &mut Vec<String>,
) -> Option<Surfaces> {
    let Some(source) = source else {
        errors.push(format!(
            "{LABEL} contract must declare one 'surface_registry' source record"
        ));
        return None;
    };
    let raw = read_raw(
        &absolute(base, source),
        Some(errors),
        &format!("{LABEL} surface registry"),
    );
    let mut surfaces = Surfaces {
        ids: BTreeSet::new(),
        rules: Vec::new(),
    };
    for line in split_dropping_trailing(&raw) {
        if !has_non_space(line) {
            continue;
        }
        let Some(record) = decode(line) else {
            errors.push(format!(
                "{LABEL} surface registry has a line that is not one JSON object"
            ));
            return None;
        };
        let sid = match get(&record, "surface_id") {
            Some(sid) if !perl::is_reference(sid) => perl::scalar(Some(sid)).unwrap_or_default(),
            _ => continue,
        };
        surfaces.ids.insert(sid.clone());
        if let Some(Py::List(targets)) = get(&record, "targets") {
            for target in targets.iter().filter(|target| !perl::is_reference(target)) {
                surfaces.rules.push((
                    sid.clone(),
                    glob(&perl::scalar(Some(target)).unwrap_or_default()),
                ));
            }
        }
    }
    Some(surfaces)
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Glob {
    /// `**`, with the `/` after it if any: anything.
    Any,
    /// `*`: anything without a `/`.
    Segment,
    /// `?`: one character other than `/`.
    One,
    Literal(char),
}

/// `glob_regex`: the target pattern as the script compiled it.
fn glob(pattern: &str) -> Vec<Glob> {
    let characters: Vec<char> = pattern.chars().collect();
    let mut tokens = Vec::new();
    let mut index = 0;
    while index < characters.len() {
        let character = characters[index];
        if character == '*' && index + 1 < characters.len() && characters[index + 1] == '*' {
            index += 1;
            if index + 1 < characters.len() && characters[index + 1] == '/' {
                index += 1;
            }
            tokens.push(Glob::Any);
        } else if character == '*' {
            tokens.push(Glob::Segment);
        } else if character == '?' {
            tokens.push(Glob::One);
        } else {
            tokens.push(Glob::Literal(character));
        }
        index += 1;
    }
    tokens
}

fn glob_matches(tokens: &[Glob], text: &[char]) -> bool {
    match tokens.split_first() {
        None => text.is_empty(),
        Some((Glob::Literal(wanted), rest)) => {
            text.first() == Some(wanted) && glob_matches(rest, &text[1..])
        }
        Some((Glob::One, rest)) => {
            text.first().is_some_and(|character| *character != '/')
                && glob_matches(rest, &text[1..])
        }
        Some((Glob::Segment, rest)) => {
            let reach = text
                .iter()
                .position(|character| *character == '/')
                .unwrap_or(text.len());
            (0..=reach)
                .rev()
                .any(|taken| glob_matches(rest, &text[taken..]))
        }
        Some((Glob::Any, rest)) => (0..=text.len())
            .rev()
            .any(|taken| glob_matches(rest, &text[taken..])),
    }
}

/// The tracked Markdown files carrying a claim tag, each with the surface that owns it, and the files no
/// surface owns or whose surface declares no disposition.
fn classify_claim_files(
    base: &Path,
    tracked: &BTreeSet<String>,
    surfaces: Option<&Surfaces>,
    disposition: &BTreeMap<String, String>,
) -> (BTreeMap<String, String>, Vec<String>) {
    let (mut owner, mut problems) = (BTreeMap::new(), Vec::new());
    for relative in tracked.iter().filter(|path| path.ends_with(".md")) {
        let file = absolute(base, relative);
        if !is_regular(&file) {
            continue;
        }
        if !has_claim_tag(&read_raw(&file, None, relative)) {
            continue;
        }
        let characters: Vec<char> = relative.chars().collect();
        let owning = surfaces.and_then(|surfaces| {
            surfaces
                .rules
                .iter()
                .find(|(_, tokens)| glob_matches(tokens, &characters))
                .map(|(sid, _)| sid.clone())
        });
        let Some(sid) = owning else {
            problems.push(format!(
                "{LABEL} claim-annotated file '{relative}' belongs to no live-document surface, so its disposition cannot be decided"
            ));
            continue;
        };
        if !disposition.contains_key(&sid) {
            problems.push(format!(
                "{LABEL} claim-annotated file '{relative}' belongs to surface '{sid}', which declares no disposition"
            ));
        }
        owner.insert(relative.clone(), sid);
    }
    (owner, problems)
}

/// `\[claim:\s*[a-z0-9][a-z0-9-]*\s*\]` anywhere in the bytes.
fn has_claim_tag(bytes: &[u8]) -> bool {
    (0..bytes.len()).any(|start| claim_tag_at(bytes, start).is_some())
}

fn claim_tag_at(bytes: &[u8], start: usize) -> Option<usize> {
    let rest = bytes.get(start..)?;
    let mut index = b"[claim:".len();
    if !rest.starts_with(b"[claim:") {
        return None;
    }
    let space = |byte: &u8| matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c);
    while rest.get(index).is_some_and(space) {
        index += 1;
    }
    if !rest
        .get(index)
        .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
    {
        return None;
    }
    index += 1;
    while rest
        .get(index)
        .is_some_and(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
    {
        index += 1;
    }
    while rest.get(index).is_some_and(space) {
        index += 1;
    }
    (rest.get(index) == Some(&b']')).then_some(start + index + 1)
}

fn validate_assertion(
    base: &Path,
    record: &Py,
    tracked: &BTreeSet<String>,
    execute: bool,
    errors: &mut Vec<String>,
) -> Option<Assertion> {
    reject_unknown(
        record,
        &format!("{LABEL} assertion"),
        errors,
        &[
            "record_type",
            "schema_version",
            "assertion_id",
            "path",
            "region",
            "value",
            "outcome",
            "producer",
            "control",
            "decision",
            "revision",
            "membership",
            "excludes_self",
            "adjudicated_against",
        ],
    );
    exact_scalar(
        get(record, "schema_version"),
        "1",
        &format!("{LABEL} assertion schema_version"),
        errors,
    );
    let id = required_slug(
        record,
        "assertion_id",
        &format!("{LABEL} assertion"),
        errors,
    )?;
    let label = format!("{LABEL} assertion '{id}'");
    let path = required_scalar(record, "path", &label, errors)?;
    if !tracked.contains(&path) {
        errors.push(format!("{label} path '{path}' is not repository-tracked"));
    }
    let value = required_scalar(record, "value", &label, errors);
    let outcome = required_scalar(record, "outcome", &label, errors).unwrap_or_default();
    if !["derived", "gated", "authored", "dated"].contains(&outcome.as_str()) {
        errors.push(format!(
            "{label} outcome '{outcome}' is not one of derived/gated/authored/dated"
        ));
    }
    let slice = validate_region(base, Some(&path), get(record, "region"), &label, errors);
    if let (Some(slice), Some(value)) = (&slice, &value)
        && !String::from_utf8_lossy(slice).contains(value.as_str())
    {
        errors.push(format!(
            "{label} value '{value}' does not appear in its own governed region"
        ));
    }
    match outcome.as_str() {
        "derived" => {
            let Some(producer @ Py::Dict(_)) = get(record, "producer") else {
                errors.push(format!(
                    "{label} outcome 'derived' requires a producer object"
                ));
                return None;
            };
            reject_unknown(
                producer,
                &format!("{label} producer"),
                errors,
                &["argv", "field", "expected_exit"],
            );
            let argv = scalar_array(
                get(producer, "argv"),
                &format!("{label} producer argv"),
                errors,
            );
            let field = required_scalar(producer, "field", &format!("{label} producer"), errors);
            if argv.is_empty() {
                errors.push(format!("{label} producer argv must not be empty"));
            }
            if argv
                .first()
                .is_some_and(|first| ["sh", "bash", "zsh", "env"].contains(&first.as_str()))
            {
                errors.push(format!(
                    "{label} producer '{}' must be an interpreter, not a shell",
                    argv[0]
                ));
            }
            if !argv.iter().any(|item| tracked.contains(item)) {
                errors.push(format!("{label} producer names no repository-tracked path"));
            }
            if execute
                && !argv.is_empty()
                && let Some(field) = &field
            {
                let (stdout, _, exit) = capture_command(base, &argv);
                let want =
                    get(producer, "expected_exit").map_or(0.0, |want| perl::numeric(Some(want)));
                if f64::from(exit) != want {
                    errors.push(format!(
                        "{label} producer exited {exit}, expected {}",
                        perl::number(want)
                    ));
                } else {
                    match decode(&stdout) {
                        None => errors.push(format!("{label} producer stdout is not one JSON object")),
                        Some(report) => match extract_field(&report, field) {
                            None => errors.push(format!("{label} producer report has no field '{field}'")),
                            Some(actual) if !values_agree(&actual, value.as_deref()) => errors.push(format!(
                                "{label} is stale: '{field}' re-derives to '{actual}', published '{}'",
                                value.as_deref().unwrap_or("")
                            )),
                            Some(_) => {}
                        },
                    }
                }
            }
        }
        "gated" => {
            let Some(control @ Py::Dict(_)) = get(record, "control") else {
                errors.push(format!("{label} outcome 'gated' requires a control object"));
                return None;
            };
            reject_unknown(
                control,
                &format!("{label} control"),
                errors,
                &["argv", "diagnostic", "red_case"],
            );
            let argv = scalar_array(
                get(control, "argv"),
                &format!("{label} control argv"),
                errors,
            );
            if argv.is_empty() {
                errors.push(format!("{label} control argv must not be empty"));
            }
            required_scalar(control, "diagnostic", &format!("{label} control"), errors);
            match get(control, "red_case") {
                Some(case @ Py::Dict(_)) => {
                    reject_unknown(
                        case,
                        &format!("{label} control red_case"),
                        errors,
                        &["path", "start_line", "end_line", "sha256"],
                    );
                    let case_path =
                        required_scalar(case, "path", &format!("{label} control red_case"), errors);
                    if let Some(case_path) = &case_path {
                        if !tracked.contains(case_path) {
                            errors.push(format!("{label} control red_case path '{case_path}' is not repository-tracked"));
                        }
                        let field = |name: &str| get(case, name).cloned().unwrap_or(Py::None);
                        let region = Py::Dict(vec![
                            (
                                pyjson::py_str("kind"),
                                Py::Str(pyjson::py_str("line_range_sha256")),
                            ),
                            (pyjson::py_str("start_line"), field("start_line")),
                            (pyjson::py_str("end_line"), field("end_line")),
                            (pyjson::py_str("sha256"), field("sha256")),
                        ]);
                        validate_region(
                            base,
                            Some(case_path),
                            Some(&region),
                            &format!("{label} control red_case"),
                            errors,
                        );
                    }
                }
                _ => errors.push(format!(
                    "{label} control has no known-bad case bound to an exact source region"
                )),
            }
        }
        "authored" => {
            if let Some(decision) = required_scalar(record, "decision", &label, errors)
                && !tracked.contains(&decision)
            {
                errors.push(format!(
                    "{label} decision '{decision}' is not repository-tracked"
                ));
            }
        }
        "dated" => {
            if let Some(revision) = required_scalar(record, "revision", &label, errors) {
                let hex = (7..=40).contains(&revision.len())
                    && revision
                        .bytes()
                        .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte));
                if !hex {
                    errors.push(format!(
                        "{label} revision '{revision}' is not a hexadecimal object name"
                    ));
                }
                if execute {
                    let argv = ["git", "rev-parse", "--verify", "--quiet"]
                        .map(String::from)
                        .to_vec();
                    let (_, _, exit) = capture_command(
                        base,
                        &[argv, vec![format!("{revision}^{{commit}}")]].concat(),
                    );
                    if exit != 0 {
                        errors.push(format!(
                            "{label} revision '{revision}' does not resolve in this repository"
                        ));
                    }
                }
            }
        }
        _ => {}
    }
    match get(record, "membership") {
        Some(membership @ Py::Dict(_)) => {
            reject_unknown(
                membership,
                &format!("{label} membership"),
                errors,
                &["enumerator", "expected"],
            );
            let argv = scalar_array(
                get(membership, "enumerator"),
                &format!("{label} membership enumerator"),
                errors,
            );
            let expected = scalar_array(
                get(membership, "expected"),
                &format!("{label} membership expected"),
                errors,
            );
            if argv.is_empty() {
                errors.push(format!("{label} membership enumerator must not be empty"));
            }
            if execute && !argv.is_empty() {
                let (stdout, _, exit) = capture_command(base, &argv);
                if exit != 0 {
                    errors.push(format!("{label} membership enumerator exited {exit}"));
                } else {
                    let text = String::from_utf8_lossy(&stdout).into_owned();
                    let mut actual: Vec<String> = split_dropping_trailing(text.as_bytes())
                        .into_iter()
                        .map(|line| {
                            String::from_utf8_lossy(line)
                                .trim_end_matches(|c: char| c.is_ascii_whitespace())
                                .to_string()
                        })
                        .filter(|line| !line.is_empty())
                        .collect();
                    actual.sort();
                    let mut want = expected.clone();
                    want.sort();
                    if actual != want {
                        let extra: Vec<&String> =
                            actual.iter().filter(|item| !want.contains(item)).collect();
                        let missing: Vec<&String> =
                            want.iter().filter(|item| !actual.contains(item)).collect();
                        let list = |items: &[&String]| {
                            items
                                .iter()
                                .map(|item| item.as_str())
                                .collect::<Vec<_>>()
                                .join(", ")
                        };
                        errors.push(format!(
                            "{label} membership enumeration drifted{}{}",
                            if extra.is_empty() {
                                String::new()
                            } else {
                                format!(" (unexpected: {})", list(&extra))
                            },
                            if missing.is_empty() {
                                String::new()
                            } else {
                                format!(" (absent: {})", list(&missing))
                            }
                        ));
                    }
                    let counts_self = actual.contains(&path);
                    let declared = perl::truthy(get(record, "excludes_self"));
                    if counts_self && !declared {
                        errors.push(format!(
                            "{label} membership counts its own publishing surface '{path}' and does not declare excludes_self"
                        ));
                    } else if declared && counts_self {
                        errors.push(format!("{label} declares excludes_self but its enumerator still returns '{path}'"));
                    }
                }
            }
        }
        Some(_) => errors.push(format!("{label} membership must be an object")),
        None if perl::truthy(get(record, "excludes_self")) => errors.push(format!(
            "{label} declares excludes_self without a membership enumeration to exclude from"
        )),
        None => {}
    }
    Some(Assertion {
        id,
        path,
        value,
        outcome,
        region: get(record, "region").cloned(),
        producer: get(record, "producer").cloned(),
    })
}

/// The paragraph-scoped coverage: each claim tag governs its paragraph, and every numeral there that no record
/// lists is reported. Returns the number of governed paragraphs and the unlisted values.
fn governed_coverage(
    base: &Path,
    files: &[String],
    assertions: &[Assertion],
) -> (usize, Vec<Unlisted>) {
    let mut listed = BTreeSet::new();
    for assertion in assertions {
        let region = assertion.region.as_ref();
        let line_of = |name: &str| {
            region
                .and_then(|region| get(region, name))
                .map(|value| perl::numeric(Some(value)) as i64)
        };
        let start = line_of("start_line").unwrap_or(0);
        let end = line_of("end_line").unwrap_or(start);
        for line in start..=end {
            listed.insert(format!(
                "{}:{line}:{}",
                assertion.path,
                assertion.value.as_deref().unwrap_or("")
            ));
        }
    }
    let (mut regions, mut unlisted) = (0, Vec::new());
    for relative in files {
        let file = absolute(base, relative);
        if !is_regular(&file) {
            continue;
        }
        let raw = read_raw(&file, None, relative);
        let lines = split_keeping_trailing(&raw);
        let mut seen = BTreeSet::new();
        for index in 0..lines.len() {
            if !has_claim_tag(lines[index]) {
                continue;
            }
            let (mut from, mut to) = (index, index);
            while from > 0 && has_non_space(lines[from - 1]) {
                from -= 1;
            }
            while to + 1 < lines.len() && has_non_space(lines[to + 1]) {
                to += 1;
            }
            if !seen.insert((from, to)) {
                continue;
            }
            regions += 1;
            for (offset, line) in lines[from..=to].iter().enumerate() {
                let number = from + offset + 1;
                let scan = strip_inline_code(&strip_claim_tags(line));
                for token in numerals(&scan) {
                    if !listed.contains(&format!("{relative}:{number}:{token}")) {
                        unlisted.push(Unlisted {
                            path: relative.clone(),
                            line: number,
                            value: token,
                        });
                    }
                }
            }
        }
    }
    (regions, unlisted)
}

/// `s/\[claim:[^\]]*\]//g`.
fn strip_claim_tags(line: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut index = 0;
    while index < line.len() {
        if line[index..].starts_with(b"[claim:")
            && let Some(close) = line[index + 7..].iter().position(|byte| *byte == b']')
        {
            index += 7 + close + 1;
            continue;
        }
        out.push(line[index]);
        index += 1;
    }
    out
}

/// `` s/`[^`]*`//g ``.
fn strip_inline_code(line: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut index = 0;
    while index < line.len() {
        if line[index] == b'`'
            && let Some(close) = line[index + 1..].iter().position(|byte| *byte == b'`')
        {
            index += 1 + close + 1;
            continue;
        }
        out.push(line[index]);
        index += 1;
    }
    out
}

fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

/// `(?<![\w.\/-])(\d+(?:,\d{3})*(?:\.\d+)?%?)(?![\w.]|-\d)`, global, with the regex's backtracking: at each start the
/// candidates are tried longest first, and the first whose end the lookahead admits is the match.
pub fn numerals(line: &[u8]) -> Vec<String> {
    let digit = |index: usize| line.get(index).is_some_and(u8::is_ascii_digit);
    let admits = |end: usize| {
        let next = line.get(end).copied();
        !(next.is_some_and(|byte| is_word(byte) || byte == b'.')
            || (next == Some(b'-') && digit(end + 1)))
    };
    let mut tokens = Vec::new();
    let mut start = 0;
    'scan: while start < line.len() {
        let behind_ok = start == 0
            || !(is_word(line[start - 1]) || matches!(line[start - 1], b'.' | b'/' | b'-'));
        if behind_ok && digit(start) {
            let mut run = start;
            while digit(run) {
                run += 1;
            }
            for digits_end in (start + 1..=run).rev() {
                let mut group_ends = vec![digits_end];
                let mut at = digits_end;
                while line.get(at) == Some(&b',') && digit(at + 1) && digit(at + 2) && digit(at + 3)
                {
                    at += 4;
                    group_ends.push(at);
                }
                for &groups_end in group_ends.iter().rev() {
                    let mut fraction_ends = Vec::new();
                    if line.get(groups_end) == Some(&b'.') && digit(groups_end + 1) {
                        let mut fraction = groups_end + 1;
                        while digit(fraction) {
                            fraction += 1;
                        }
                        fraction_ends.extend((groups_end + 2..=fraction).rev());
                    }
                    fraction_ends.push(groups_end);
                    for &fraction_end in &fraction_ends {
                        let mut ends = Vec::new();
                        if line.get(fraction_end) == Some(&b'%') {
                            ends.push(fraction_end + 1);
                        }
                        ends.push(fraction_end);
                        if let Some(&end) = ends.iter().find(|end| admits(**end)) {
                            tokens.push(String::from_utf8_lossy(&line[start..end]).into_owned());
                            start = end;
                            continue 'scan;
                        }
                    }
                }
            }
        }
        start += 1;
    }
    tokens
}

/// `extract_field`: a dotted path through objects and array indexes to a scalar, as Perl stringifies it; `None` when
/// the path is absent or ends on a reference (an array, an object or a boolean).
fn extract_field(report: &Py, field: &str) -> Option<String> {
    let mut node = report;
    let mut parts: Vec<&str> = field.split('.').collect();
    while parts.last().is_some_and(|part| part.is_empty()) {
        parts.pop();
    }
    for part in parts {
        node = match node {
            Py::Dict(_) => pyjson::get(node, part)?,
            Py::List(items)
                if !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit()) =>
            {
                items.get(part.parse::<usize>().ok()?)?
            }
            _ => return None,
        };
    }
    if perl::is_reference(node) {
        return None;
    }
    Some(perl::scalar(Some(node)).unwrap_or_default())
}

/// `values_agree`: equal strings, or equal numbers once commas and a trailing `%` are dropped.
fn values_agree(actual: &str, published: Option<&str>) -> bool {
    let Some(published) = published else {
        return false;
    };
    if actual == published {
        return true;
    }
    let normal = |text: &str| -> Option<f64> {
        let text = text.replace(',', "");
        let text = text.strip_suffix('%').unwrap_or(&text);
        let body = text.strip_prefix('-').unwrap_or(text);
        let (whole, fraction) = body
            .split_once('.')
            .map_or((body, None), |(whole, fraction)| (whole, Some(fraction)));
        let digits =
            |part: &str| !part.is_empty() && part.bytes().all(|byte| byte.is_ascii_digit());
        (digits(whole) && fraction.is_none_or(digits))
            .then(|| text.parse().ok())
            .flatten()
    };
    matches!((normal(actual), normal(published)), (Some(left), Some(right)) if left == right)
}

fn validate_region(
    base: &Path,
    path: Option<&str>,
    region: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<Vec<u8>> {
    let Some(region @ Py::Dict(_)) = region else {
        errors.push(format!("{label} region must be an object"));
        return None;
    };
    reject_unknown(
        region,
        &format!("{label} region"),
        errors,
        &["kind", "start_line", "end_line", "sha256"],
    );
    exact_scalar(
        get(region, "kind"),
        "line_range_sha256",
        &format!("{label} region kind"),
        errors,
    );
    let start = positive_integer(
        get(region, "start_line"),
        &format!("{label} region start_line"),
        errors,
    )
    .unwrap_or(0.0);
    let end = positive_integer(
        get(region, "end_line"),
        &format!("{label} region end_line"),
        errors,
    )
    .unwrap_or(0.0);
    if end < start {
        errors.push(format!("{label} region end_line precedes start_line"));
    }
    let path = path?;
    if start < 1.0 || end < start {
        return None;
    }
    let file = absolute(base, path);
    if !is_regular(&file) {
        errors.push(format!(
            "{label} region path '{path}' is missing or not a regular file"
        ));
        return None;
    }
    let raw = read_raw(&file, Some(errors), label);
    let lines = split_lines(&raw);
    if end as usize > lines.len() {
        errors.push(format!(
            "{label} region {}..{} is outside 1..{}",
            start as u64,
            end as u64,
            lines.len()
        ));
        return None;
    }
    let slice: Vec<u8> = lines[start as usize - 1..end as usize].concat();
    let expected = match get(region, "sha256") {
        Some(Py::Str(text)) => pyjson::to_text(text),
        Some(other) if !perl::is_reference(other) => perl::scalar(Some(other)).unwrap_or_default(),
        _ => String::new(),
    };
    if !is_sha256(&expected) {
        errors.push(format!(
            "{label} region sha256 must be 64 lowercase hex characters"
        ));
        return Some(slice);
    }
    let actual = sha256_hex(&slice);
    if actual != expected {
        errors.push(format!(
            "{label} exact region is stale: SHA-256 {actual} != {expected}"
        ));
    }
    Some(slice)
}

fn tracked_paths(base: &Path, errors: &mut Vec<String>) -> BTreeSet<String> {
    let argv = ["git", "ls-files", "-z"].map(String::from);
    let (stdout, stderr, exit) = capture_command(base, &argv);
    if exit != 0 {
        errors.push(format!(
            "{LABEL} cannot list tracked files: {}",
            String::from_utf8_lossy(&stderr)
        ));
        return BTreeSet::new();
    }
    stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).into_owned())
        .collect()
}

/// `capture_command`: `argv` run in `base`, its stdout, stderr and exit status as Perl's `$? >> 8` reads it (0 for a
/// signal), or 127 when it cannot start.
fn capture_command(base: &Path, argv: &[String]) -> (Vec<u8>, Vec<u8>, i32) {
    let Some((program, arguments)) = argv.split_first() else {
        return (Vec::new(), Vec::new(), 127);
    };
    match Command::new(program)
        .args(arguments)
        .current_dir(base)
        .stdin(Stdio::null())
        .output()
    {
        Ok(output) => (
            output.stdout,
            output.stderr,
            output.status.code().unwrap_or(0),
        ),
        Err(error) => (
            Vec::new(),
            format!("cannot execute '{program}': {error}").into_bytes(),
            127,
        ),
    }
}

/// The script's self-test: 25 cases on a fixture repository under `generated/`, each fault in
/// `CLAIM-VERIFICATION-ADOPTION.7.0`'s matrix driven RED before the real tree's PASS is trusted.
fn self_test(root: &Path) -> i32 {
    let fixture = root.join(format!(
        "generated/.published-assertions-self-test.{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&fixture);
    let cases = self_test_cases(&fixture);
    let _ = fs::remove_dir_all(&fixture);
    let cases = match cases {
        Ok(cases) => cases,
        Err(message) => {
            eprintln!("{LABEL} self-test: {message}");
            return 1;
        }
    };
    let passed = cases.iter().filter(|(_, ok, _)| *ok).count();
    for (name, ok, errors) in &cases {
        if *ok {
            continue;
        }
        eprintln!("{LABEL} self-test FAILED case '{name}'");
        for error in errors {
            eprintln!("  saw: {error}");
        }
    }
    if passed != cases.len() {
        eprintln!("{LABEL}: self-test {passed}/{} cases pass.", cases.len());
        return 1;
    }
    eprintln!(
        "{LABEL}: self-test {passed}/{} positive, drift, wrong-field, unlisted, control, self-reference, disagreement, enumeration, region, producer, outcome, duplicate, bound, compound-adjective, ratio, absorbed-punctuation, undeclared-surface, orphan-file, reasonless-exemption, unknown-surface, stale-surface-source, and load-bearing-exemption cases pass.",
        cases.len()
    );
    0
}

type Case = (String, bool, Vec<String>);

fn write(path: &Path, content: &str) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(path, content).map_err(|error| format!("cannot write {}: {error}", path.display()))
}

fn git(fixture: &Path, arguments: &[&str]) -> Result<String, String> {
    let argv: Vec<String> = std::iter::once("git")
        .chain(arguments.iter().copied())
        .map(String::from)
        .collect();
    let (stdout, stderr, exit) = capture_command(fixture, &argv);
    if exit != 0 {
        return Err(format!(
            "'{}' failed: {}",
            argv.join(" "),
            String::from_utf8_lossy(&stderr)
        ));
    }
    Ok(String::from_utf8_lossy(&stdout).into_owned())
}

/// A record built from JSON text, so the fixture reads as the script's literals do.
fn record(text: &str) -> Py {
    pyjson::loads(text).expect("a fixture record")
}

fn set(record: &mut Py, key: &str, value: Py) {
    let Py::Dict(pairs) = record else {
        return;
    };
    let key = pyjson::py_str(key);
    match pairs.iter_mut().find(|(name, _)| *name == key) {
        Some(pair) => pair.1 = value,
        None => pairs.push((key, value)),
    }
}

fn remove(record: &mut Py, key: &str) {
    if let Py::Dict(pairs) = record {
        let key = pyjson::py_str(key);
        pairs.retain(|(name, _)| *name != key);
    }
}

fn text(value: &str) -> Py {
    Py::Str(pyjson::py_str(value))
}

/// `JSON::PP->new->canonical(1)->encode`: compact, keys sorted, non-ASCII written as is.
fn canonical(value: &Py) -> String {
    let encoder = Encoder {
        indent: None,
        sort_keys: true,
        ensure_ascii: false,
        item_separator: ",",
        key_separator: ":",
    };
    pyjson::dumps(value, &encoder).expect("a fixture value")
}

fn self_test_cases(fixture: &Path) -> Result<Vec<Case>, String> {
    let file = |relative: &str| fixture.join(relative);
    write(
        &file("scripts/probe.pl"),
        "#!/usr/bin/env perl\nprint qq({\"counts\":{\"units\":8,\"views\":5},\"unresolved\":0}\\n);\n",
    )?;
    write(
        &file("scripts/members.pl"),
        "#!/usr/bin/env perl\nprint qq(alpha.md\\nbeta.md\\n);\n",
    )?;
    write(
        &file("scripts/control.pl"),
        "#!/usr/bin/env perl\nprint qq{control PASS\\n};\n",
    )?;
    write(
        &file("decision.md"),
        "# Decision\n\nFive views are authored.\n",
    )?;
    write(&file("alpha.md"), "alpha\n")?;
    write(&file("beta.md"), "beta\n")?;
    fs::create_dir_all(file("doctrine/claim_verification")).map_err(|error| error.to_string())?;
    let doc = "# Fixture\n\nThe census reports 8 units. [claim: fixture-claim]\n\nUnresolved is 0 because the control fails otherwise. [claim: fixture-claim]\n\nThere are 5 views by decision. [claim: fixture-claim]\n\nMeasured 2 at the boundary. [claim: fixture-claim]\n\nThe governed members are 2 files. [claim: fixture-claim]\n\nPunctuated 40, plus 1,922 and 922 today. [claim: fixture-claim]\n";
    write(&file("surface.md"), doc)?;
    write(
        &file("other.md"),
        "Elsewhere the census reports 8 units. [claim: fixture-claim]\n",
    )?;
    write(
        &file("archive.md"),
        "Sealed on 2026-01-01: 4321 units. [claim: fixture-claim]\n",
    )?;
    git(fixture, &["init", "-q"])?;
    git(
        fixture,
        &[
            "config",
            "user.email",
            "assertions-self-test@example.invalid",
        ],
    )?;
    git(fixture, &["config", "user.name", "Assertions Self Test"])?;
    git(fixture, &["add", "-A"])?;
    git(fixture, &["commit", "-q", "-m", "FIXTURE.0 — seed"])?;
    let revision = git(fixture, &["rev-parse", "HEAD"])?.trim_end().to_string();
    let lines: Vec<String> = split_lines(doc.as_bytes())
        .iter()
        .map(|line| String::from_utf8_lossy(line).into_owned())
        .collect();
    let region = |number: usize| {
        record(&format!(
            "{{\"kind\":\"line_range_sha256\",\"start_line\":{number},\"end_line\":{number},\"sha256\":\"{}\"}}",
            sha256_hex(lines[number - 1].as_bytes())
        ))
    };
    let control_line = "print qq{control PASS\\n};\n";
    let meta = record(
        "{\"record_type\":\"registry\",\"schema_version\":1,\"phase\":\"frozen\",\"max_records\":64,\"max_bytes\":32768,\"max_record_bytes\":4096,\"max_array_items\":8,\"max_scalar_bytes\":512}",
    );
    let surface_rel = "doctrine/live_document_size/surfaces.jsonl";
    write(
        &file(surface_rel),
        &format!(
            "{}\n{}\n",
            canonical(&record(
                "{\"surface_id\":\"fixture_governed\",\"targets\":[\"surface.md\",\"other.md\"]}"
            )),
            canonical(&record(
                "{\"surface_id\":\"fixture_exempt\",\"targets\":[\"archive.md\"]}"
            ))
        ),
    )?;
    git(fixture, &["add", "-A"])?;
    git(
        fixture,
        &["commit", "-q", "-m", "FIXTURE.1 — surface registry"],
    )?;
    let surface_source = || -> Py {
        let digest = sha256_hex(&fs::read(file(surface_rel)).unwrap_or_default());
        record(&format!(
            "{{\"record_type\":\"source\",\"schema_version\":1,\"source_id\":\"surface_registry\",\"path\":\"{surface_rel}\",\"sha256\":\"{digest}\"}}"
        ))
    };
    let governed = record(
        "{\"record_type\":\"surface_disposition\",\"schema_version\":1,\"surface_id\":\"fixture_governed\",\"disposition\":\"governed\"}",
    );
    let exempt = record(
        "{\"record_type\":\"surface_disposition\",\"schema_version\":1,\"surface_id\":\"fixture_exempt\",\"disposition\":\"exempt\",\"reason\":\"Sealed dated evidence capture whose currentness is not asserted.\"}",
    );
    let mut derived = record(
        "{\"record_type\":\"assertion\",\"schema_version\":1,\"assertion_id\":\"units-derived\",\"path\":\"surface.md\",\"value\":\"8\",\"outcome\":\"derived\",\"producer\":{\"argv\":[\"perl\",\"scripts/probe.pl\"],\"field\":\"counts.units\",\"expected_exit\":0}}",
    );
    set(&mut derived, "region", region(3));
    let mut gated = record(&format!(
        "{{\"record_type\":\"assertion\",\"schema_version\":1,\"assertion_id\":\"unresolved-gated\",\"path\":\"surface.md\",\"value\":\"0\",\"outcome\":\"gated\",\"control\":{{\"argv\":[\"perl\",\"scripts/control.pl\"],\"diagnostic\":\"control PASS\",\"red_case\":{{\"path\":\"scripts/control.pl\",\"start_line\":2,\"end_line\":2,\"sha256\":\"{}\"}}}}}}",
        sha256_hex(control_line.as_bytes())
    ));
    set(&mut gated, "region", region(5));
    let mut authored = record(
        "{\"record_type\":\"assertion\",\"schema_version\":1,\"assertion_id\":\"views-authored\",\"path\":\"surface.md\",\"value\":\"5\",\"outcome\":\"authored\",\"decision\":\"decision.md\"}",
    );
    set(&mut authored, "region", region(7));
    let dated_record = |id: &str, line: usize, value: &str| {
        let mut built = record(&format!(
            "{{\"record_type\":\"assertion\",\"schema_version\":1,\"assertion_id\":\"{id}\",\"path\":\"surface.md\",\"value\":\"{value}\",\"outcome\":\"dated\",\"revision\":\"{revision}\"}}"
        ));
        set(&mut built, "region", region(line));
        built
    };
    let dated = dated_record("boundary-dated", 9, "2");
    let mut membership = dated_record("members-derived", 11, "2");
    set(
        &mut membership,
        "membership",
        record(
            "{\"enumerator\":[\"perl\",\"scripts/members.pl\"],\"expected\":[\"alpha.md\",\"beta.md\"]}",
        ),
    );
    let other_line = "Elsewhere the census reports 8 units. [claim: fixture-claim]\n";
    let other = record(&format!(
        "{{\"record_type\":\"assertion\",\"schema_version\":1,\"assertion_id\":\"units-derived-other\",\"path\":\"other.md\",\"value\":\"8\",\"outcome\":\"derived\",\"region\":{{\"kind\":\"line_range_sha256\",\"start_line\":1,\"end_line\":1,\"sha256\":\"{}\"}},\"producer\":{{\"argv\":[\"perl\",\"scripts/probe.pl\"],\"field\":\"counts.units\",\"expected_exit\":0}}}}",
        sha256_hex(other_line.as_bytes())
    ));
    let punct_comma = dated_record("punct-comma-dated", 13, "40");
    let punct_thousands = dated_record("punct-thousands-dated", 13, "1,922");
    let punct_bare = dated_record("punct-bare-dated", 13, "922");
    let base: Vec<Py> = vec![
        derived.clone(),
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
        punct_comma.clone(),
        punct_thousands.clone(),
        punct_bare.clone(),
    ];
    let contract_rel = "doctrine/claim_verification/published_assertions.jsonl";
    let write_records = |records: Vec<Py>| -> Result<(), String> {
        write(
            &file(contract_rel),
            &records
                .iter()
                .map(|record| format!("{}\n", canonical(record)))
                .collect::<String>(),
        )
    };
    let with_scope = |scope: Vec<Py>, rest: &[Py]| -> Vec<Py> {
        std::iter::once(meta.clone())
            .chain(scope)
            .chain(rest.iter().cloned())
            .collect()
    };
    let full = |rest: &[Py]| {
        with_scope(
            vec![surface_source(), governed.clone(), exempt.clone()],
            rest,
        )
    };
    let mut cases: Vec<Case> = Vec::new();
    let mut case = |name: &str, want_fail: bool, pattern: &str| {
        let found = validate_registry(fixture, contract_rel, true);
        let failed = !found.errors.is_empty();
        let mut ok = failed == want_fail;
        if ok && want_fail && !pattern.is_empty() {
            ok = found
                .errors
                .iter()
                .any(|error| pattern_matches(pattern, error));
        }
        cases.push((name.to_string(), ok, found.errors));
    };
    write_records(full(&base))?;
    case(
        "positive: all four outcomes plus a membership enumeration",
        false,
        "",
    );
    // 1 — a drifted value.
    let drifted_doc: String = [
        &lines[..2],
        &["The census reports 9 units. [claim: fixture-claim]\n".to_string()],
        &lines[3..],
    ]
    .concat()
    .concat();
    write(&file("surface.md"), &drifted_doc)?;
    let mut bad = derived.clone();
    set(&mut bad, "value", text("9"));
    set(
        &mut bad,
        "region",
        record(&format!(
            "{{\"kind\":\"line_range_sha256\",\"start_line\":3,\"end_line\":3,\"sha256\":\"{}\"}}",
            sha256_hex(b"The census reports 9 units. [claim: fixture-claim]\n")
        )),
    );
    write_records(full(&[
        bad,
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: published value drifted from its producer field",
        true,
        "is stale: 'counts.units'",
    );
    write(&file("surface.md"), doc)?;
    // 2 — the wrong field.
    let mut bad = derived.clone();
    if let Some(Py::Dict(_)) = pyjson::get(&bad, "producer") {
        let mut producer = pyjson::get(&bad, "producer").cloned().expect("producer");
        set(&mut producer, "field", text("counts.views"));
        set(&mut bad, "producer", producer);
    }
    write_records(full(&[
        bad,
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: value bound to the wrong producer field",
        true,
        "is stale: 'counts.views'",
    );
    // 3 — an unlisted value.
    write_records(full(&[
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: unlisted published value in a claim-annotated region",
        true,
        "no assertion record lists",
    );
    // 4 — no known-bad case.
    let mut bad = gated.clone();
    let mut control = pyjson::get(&bad, "control").cloned().expect("control");
    remove(&mut control, "red_case");
    set(&mut bad, "control", control);
    write_records(full(&[
        derived.clone(),
        bad,
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: gated record with no known-bad control region",
        true,
        "no known-bad case",
    );
    // 5 — self-reference without excludes_self.
    write(
        &file("scripts/members.pl"),
        "#!/usr/bin/env perl\nprint qq(alpha.md\\nbeta.md\\nsurface.md\\n);\n",
    )?;
    let mut bad = membership.clone();
    set(
        &mut bad,
        "membership",
        record(
            "{\"enumerator\":[\"perl\",\"scripts/members.pl\"],\"expected\":[\"alpha.md\",\"beta.md\",\"surface.md\"]}",
        ),
    );
    write_records(full(&[
        derived.clone(),
        gated.clone(),
        authored.clone(),
        dated.clone(),
        bad,
        other.clone(),
    ]))?;
    case(
        "RED: membership counts its own publishing surface without excludes_self",
        true,
        "counts its own publishing surface",
    );
    write(
        &file("scripts/members.pl"),
        "#!/usr/bin/env perl\nprint qq(alpha.md\\nbeta.md\\n);\n",
    )?;
    // 6 — cross-surface disagreement.
    let other_seven = "Elsewhere the census reports 7 units. [claim: fixture-claim]\n";
    write(&file("other.md"), other_seven)?;
    let mut bad = other.clone();
    set(&mut bad, "value", text("7"));
    let mut bad_region = pyjson::get(&bad, "region").cloned().expect("region");
    set(
        &mut bad_region,
        "sha256",
        text(&sha256_hex(other_seven.as_bytes())),
    );
    set(&mut bad, "region", bad_region);
    write_records(full(&[
        derived.clone(),
        bad,
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
    ]))?;
    case(
        "RED: two surfaces disagree on one producer field",
        true,
        "cross-surface disagreement",
    );
    write(&file("other.md"), other_line)?;
    // 7 — enumeration drift.
    write(
        &file("scripts/members.pl"),
        "#!/usr/bin/env perl\nprint qq(alpha.md\\ngamma.md\\n);\n",
    )?;
    write_records(full(&[
        derived.clone(),
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: membership enumeration drifted while its size held",
        true,
        "membership enumeration drifted",
    );
    write(
        &file("scripts/members.pl"),
        "#!/usr/bin/env perl\nprint qq(alpha.md\\nbeta.md\\n);\n",
    )?;
    // 8 — a stale region.
    let mut bad = derived.clone();
    let mut bad_region = pyjson::get(&bad, "region").cloned().expect("region");
    set(
        &mut bad_region,
        "sha256",
        text(&sha256_hex(b"not the region")),
    );
    set(&mut bad, "region", bad_region);
    write_records(full(&[
        bad,
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: exact region digest is stale",
        true,
        "exact region is stale",
    );
    // 9 — an untracked producer.
    let mut bad = derived.clone();
    let mut producer = pyjson::get(&bad, "producer").cloned().expect("producer");
    set(
        &mut producer,
        "argv",
        record("[\"perl\",\"-e\",\"print q({\\\"counts\\\":{\\\"units\\\":8}})\"]"),
    );
    set(&mut bad, "producer", producer);
    write_records(full(&[
        bad,
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: producer names no tracked path",
        true,
        "names no repository-tracked path",
    );
    // 10 — a fifth outcome.
    let mut bad = derived.clone();
    set(&mut bad, "outcome", text("trajectory"));
    remove(&mut bad, "producer");
    write_records(full(&[
        bad,
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: an outcome outside the four is refused",
        true,
        "is not one of derived/gated/authored/dated",
    );
    // 11 — excludes_self alone.
    let mut bad = authored.clone();
    set(&mut bad, "excludes_self", Py::Bool(true));
    write_records(full(&[
        derived.clone(),
        gated.clone(),
        bad,
        dated.clone(),
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: excludes_self declared with no membership enumeration",
        true,
        "without a membership",
    );
    // 12 — a duplicate id.
    let mut duplicate = gated.clone();
    set(&mut duplicate, "assertion_id", text("units-derived"));
    write_records(full(&[
        derived.clone(),
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
        duplicate,
    ]))?;
    case("RED: duplicated assertion_id", true, "is duplicated");
    // 13 — a value absent from its region.
    let mut bad = dated.clone();
    set(&mut bad, "value", text("42"));
    write_records(full(&[
        derived.clone(),
        gated.clone(),
        authored.clone(),
        bad,
        membership.clone(),
        other.clone(),
    ]))?;
    case(
        "RED: value absent from its own governed region",
        true,
        "does not appear in its own governed region",
    );
    // 14 — bounds.
    let mut small = meta.clone();
    set(&mut small, "max_records", Py::Int("1".to_string()));
    write_records(std::iter::once(small).chain(base.iter().cloned()).collect())?;
    case(
        "RED: record count exceeds the declared bound",
        true,
        "more data records than max_records",
    );
    // 15 — a compound adjective.
    write(
        &file("surface.md"),
        &format!("{doc}\nThe 27-case self-test covers it. [claim: fixture-claim]\n"),
    )?;
    write_records(full(&base))?;
    case(
        "RED: compound-adjective value is visible to the coverage grammar",
        true,
        "published value '27' at surface.md:* sits",
    );
    write(&file("surface.md"), doc)?;
    // 16 — a ratio.
    write(
        &file("surface.md"),
        &format!("{doc}\nControls pass 15/15 today. [claim: fixture-claim]\n"),
    )?;
    write_records(full(&base))?;
    case(
        "RED: ratio-form value is visible to the coverage grammar",
        true,
        "published value '15' at surface.md:* sits",
    );
    write(&file("surface.md"), doc)?;
    // 17 — absorbed punctuation.
    let mut bad = punct_comma.clone();
    set(&mut bad, "value", text("40,"));
    write_records(full(&[
        derived.clone(),
        gated.clone(),
        authored.clone(),
        dated.clone(),
        membership.clone(),
        other.clone(),
        bad,
        punct_thousands.clone(),
        punct_bare.clone(),
    ]))?;
    case(
        "RED: a record whose value absorbed sentence punctuation covers nothing",
        true,
        "published value '40' at surface.md:* sits",
    );
    // 18 — no disposition.
    write_records(with_scope(vec![surface_source(), governed.clone()], &base))?;
    case(
        "RED: claim-annotated file on a surface with no declared disposition",
        true,
        "declares no disposition",
    );
    // 19 — an orphan file.
    write(
        &file("orphan.md"),
        "An orphan publishes 77 units. [claim: fixture-claim]\n",
    )?;
    git(fixture, &["add", "orphan.md"])?;
    write_records(full(&base))?;
    case(
        "RED: claim-annotated file owned by no live-document surface",
        true,
        "belongs to no live-document surface",
    );
    let _ = fs::remove_file(file("orphan.md"));
    git(fixture, &["rm", "-q", "--cached", "orphan.md"])?;
    // 20 — an exemption without a reason.
    let mut bad = exempt.clone();
    remove(&mut bad, "reason");
    write_records(with_scope(
        vec![surface_source(), governed.clone(), bad],
        &base,
    ))?;
    case(
        "RED: exempt surface without a stated reason",
        true,
        "must state a reason",
    );
    // 21 — an unknown surface.
    let mut bad = exempt.clone();
    set(&mut bad, "surface_id", text("fixture_typo"));
    write_records(with_scope(
        vec![surface_source(), governed.clone(), exempt.clone(), bad],
        &base,
    ))?;
    case(
        "RED: disposition names an unknown surface",
        true,
        "names no surface in the live-document",
    );
    // 22 — a stale surface source.
    let mut stale = surface_source();
    set(
        &mut stale,
        "sha256",
        text(&sha256_hex(b"not the surface registry")),
    );
    write_records(with_scope(
        vec![stale, governed.clone(), exempt.clone()],
        &base,
    ))?;
    case(
        "RED: surface registry moved under a stale source digest",
        true,
        "source * is stale",
    );
    // 23 — the exemption is load-bearing.
    let mut flip = exempt.clone();
    set(&mut flip, "disposition", text("governed"));
    remove(&mut flip, "reason");
    write_records(with_scope(
        vec![surface_source(), governed.clone(), flip],
        &base,
    ))?;
    case(
        "RED: a value under an exemption is reported the moment that surface is governed",
        true,
        "published value '4321' at archive.md:1 sits",
    );
    write_records(full(&base))?;
    case("positive: restored contract is green again", false, "");
    Ok(cases)
}

/// The script's case patterns: literal text where `*` stands for any run of characters.
fn pattern_matches(pattern: &str, text: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();
    (0..text.len())
        .filter(|start| text.is_char_boundary(*start))
        .any(|start| {
            let mut at = start;
            for (index, part) in parts.iter().enumerate() {
                if index == 0 {
                    if !text[at..].starts_with(part) {
                        return false;
                    }
                    at += part.len();
                } else {
                    match text[at..].find(part) {
                        Some(offset) => at += offset + part.len(),
                        None => return false,
                    }
                }
            }
            true
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_self_test_cases_keep_their_verdicts() {
        let fixture =
            crate::repository_root().join(".project-data/tmp/xtask-published-assertions-self-test");
        let _ = fs::remove_dir_all(&fixture);
        let cases = self_test_cases(&fixture).expect("fixture");
        fs::remove_dir_all(&fixture).expect("cleanup");
        let failed: Vec<&Case> = cases.iter().filter(|(_, ok, _)| !ok).collect();
        assert!(failed.is_empty(), "{failed:#?}");
        assert_eq!(cases.len(), 25);
    }

    #[test]
    fn the_numeral_grammar_backtracks_as_the_regex_does() {
        let found = |line: &str| numerals(line.as_bytes());
        assert_eq!(
            found("Punctuated 40, plus 1,922 and 922 today."),
            ["40", "1,922", "922"]
        );
        // Perl's own answer (`perl -e`): after `1`, the match restarts at the comma, and `9224` follows it.
        assert_eq!(
            found("1,9224 and 5%a and 7% and 27-case and 15/15"),
            ["1", "9224", "5", "7%", "27", "15"]
        );
        assert_eq!(
            found("SHA-256 .7.2 1.95.0 2026-08-28 segment-0013 v2 12.5x 3.25"),
            ["3.25"]
        );
    }

    #[test]
    fn tags_and_inline_code_are_not_published_values() {
        let line = b"Totals `99` and 12 units [claim: a-1] then 3 [claim: open";
        let scan = strip_inline_code(&strip_claim_tags(line));
        assert_eq!(numerals(&scan), ["12", "3"]);
        assert!(has_claim_tag(b"x [claim:  fixture-claim ] y") && !has_claim_tag(b"[claim: -x]"));
    }

    #[test]
    fn coverage_governs_the_tagged_paragraph_without_its_inline_code() {
        let base =
            crate::repository_root().join(".project-data/tmp/xtask-published-assertions-coverage");
        let _ = fs::remove_dir_all(&base);
        fs::create_dir_all(&base).expect("fixture");
        fs::write(
            base.join("doc.md"),
            "Intro 1.\n\nTotals `99` and 12\nunits, 3 more. [claim: x]\n\nAfter 4.\n",
        )
        .expect("doc");
        let (regions, unlisted) = governed_coverage(&base, &["doc.md".to_string()], &[]);
        fs::remove_dir_all(&base).expect("cleanup");
        let found: Vec<(usize, &str)> = unlisted
            .iter()
            .map(|value| (value.line, value.value.as_str()))
            .collect();
        assert_eq!((regions, found), (1, vec![(3, "12"), (4, "3")]));
    }

    #[test]
    fn fields_and_values_compare_as_the_script_compares_them() {
        let report = pyjson::loads("{\"counts\":{\"units\":8,\"rate\":1.50,\"ok\":true,\"none\":null},\"list\":[\"a\",\"b\"]}").expect("report");
        assert_eq!(extract_field(&report, "counts.units").as_deref(), Some("8"));
        assert_eq!(
            extract_field(&report, "counts.rate").as_deref(),
            Some("1.5")
        );
        assert_eq!(extract_field(&report, "counts.ok"), None);
        assert_eq!(extract_field(&report, "counts.none").as_deref(), Some(""));
        assert_eq!(extract_field(&report, "list.1").as_deref(), Some("b"));
        assert_eq!(extract_field(&report, "list.2"), None);
        assert!(
            values_agree("1922", Some("1,922"))
                && values_agree("50", Some("50%"))
                && !values_agree("8", Some("9"))
        );
        assert!(!values_agree("x", Some("x,")) && !values_agree("8", None));
    }

    #[test]
    fn globs_follow_the_scripts_compilation() {
        let matches = |pattern: &str, path: &str| {
            glob_matches(&glob(pattern), &path.chars().collect::<Vec<_>>())
        };
        assert!(matches("docs/**/*.md", "docs/a/b/c.md") && matches("docs/**/*.md", "docs/c.md"));
        assert!(matches("docs/*.md", "docs/c.md") && !matches("docs/*.md", "docs/a/c.md"));
        assert!(matches("a?c", "abc") && !matches("a?c", "a/c"));
    }
}
