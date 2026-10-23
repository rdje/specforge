//! The `CHAIN-CURRENCY` doctrine (`CORPUS-CHAIN-CURRENCY.1`, ADR 0025 decision 2), native since
//! `RUSTDOC-AND-XTASK.14.d`: "the persisted corpus chain reflects the current binary" is a measured property, never
//! an assumption. For every persisted artifact, replaying its stage with the current release build at the fixed
//! persisted input (`--dry-run`, so nothing is written) must reproduce it, content identity excluding the validation
//! reports and the proof metadata `specforge validate` back-annotates. A document whose normalized bundle was
//! reclaimed is unmeasurable at the evidence stage and counted as such, and a deliberately quarantined legacy proof
//! is counted apart from a replay failure. The retained bundles on disk must equal
//! `doctrine/chain_currency/retained_bundles.json` exactly (decision 3). It skips loudly when there is no corpus.
//!
//! The content-identity predicate is defined here once (`.11`, `SOURCE-IR-REPRODUCIBILITY.15`):
//! `scripts/lib/stage_artifact_identity.sh`, which `scripts/rebuild_stage_cascade.sh` and the held-out probe source,
//! is a shim over `--compare-stage-artifact` and `--compare-emitted-isf`. Which build answers stays
//! `scripts/lib/corpus_replay_binary.sh`, shared with the proof-seal gate and the rebuild (`CORPUS-CHAIN-CURRENCY.8`),
//! and this check asks it.
//!
//! `scripts/check_chain_currency.sh`, the registered script, is a shim that keeps its argument loop, its knobs
//! (`SPECFORGE_CHAIN_CURRENCY_GENERATED_ROOT`, `SPECFORGE_CHAIN_CURRENCY_RETENTION_CONTRACT`) and the activation.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::perl;
use crate::project_data::{self, Environment};
use crate::pyjson::{self, Py, PyStr};

/// The cases the self-test runs; `scripts/report_self_test_totals.pl` reads this declaration.
const EXPECTED_SELF_TEST_CASES: usize = 22;
const DEFAULT_GENERATED_ROOT: &str = "generated";
const DEFAULT_RETENTION_CONTRACT: &str = "doctrine/chain_currency/retained_bundles.json";
const STAGES: [&str; 4] = ["evidence", "semantic", "intent", "isf-adapter"];
/// Back-annotated by `specforge validate` after the stage ran, so not part of content identity; the product's
/// `*_ir_fingerprint` helpers clear the same fields before hashing.
const EXCLUDED_SECTIONS: [&str; 3] = ["validation_reports", "proof_context", "proof_ledger"];

fn note(message: &str) {
    println!("[chain-currency] {message}");
}

fn fail_note(message: &str) {
    eprintln!("[chain-currency] FAIL: {message}");
}

/// `--check` (the gate) or `--self-test`, as the shim passes the mode its argument loop chose; or one of the shared
/// predicates, as `scripts/lib/stage_artifact_identity.sh` calls them.
pub fn run(repository: &Path, arguments: &[String]) -> i32 {
    let mut arguments: Vec<&str> = arguments.iter().map(String::as_str).collect();
    // The library passes the directory it was called from, so a relative path resolves, and is named, as the
    // caller wrote it.
    if let ["--caller-dir", directory, ..] = arguments.as_slice() {
        if let Err(error) = std::env::set_current_dir(directory) {
            eprintln!(
                "[chain-currency] cannot return to the calling directory {directory}: {error}"
            );
            return 2;
        }
        arguments.drain(..2);
    }
    match arguments.as_slice() {
        [] | ["--check"] => gate(repository),
        ["--self-test"] => self_test(repository),
        ["--compare-stage-artifact", persisted, replay] => predicate_command(
            compare_stage_artifact(Path::new(persisted), Path::new(replay)),
        ),
        ["--compare-emitted-isf", adapter, root] => {
            predicate_command(compare_emitted_isf(Path::new(adapter), Path::new(root)))
        }
        _ => {
            eprintln!(
                "Usage: cargo xtask check chain-currency [--check|--self-test] | --compare-stage-artifact PERSISTED \
                 REPLAY | --compare-emitted-isf ADAPTER ARTIFACT_ROOT"
            );
            2
        }
    }
}

/// Why a predicate could not read its inputs, and the status Perl's `die` exited with: the failed call's `errno`,
/// or 255.
struct Refusal {
    reason: String,
    code: i32,
}

impl From<String> for Refusal {
    fn from(reason: String) -> Self {
        Refusal { reason, code: 255 }
    }
}

/// Perl's `"cannot read $path: $!"`, exiting with that `errno`.
fn unreadable(path: &Path, error: &std::io::Error) -> Refusal {
    let text = error.to_string();
    Refusal {
        reason: format!(
            "cannot read {}: {}",
            path.display(),
            text.split(" (os error").next().unwrap_or(&text)
        ),
        code: error.raw_os_error().unwrap_or(255),
    }
}

/// A predicate's verdict: the same, a difference, or a refusal.
type Verdict = Result<Result<(), String>, Refusal>;

/// A predicate's exit, as the Perl it replaced exited: 0 when the same, 1 with the difference printed, and Perl's
/// `die` status with the reason on standard error when an input cannot be read.
fn predicate_command(verdict: Verdict) -> i32 {
    match verdict {
        Ok(Ok(())) => 0,
        Ok(Err(difference)) => {
            println!("{difference}");
            1
        }
        Err(refusal) => {
            eprintln!("{}", refusal.reason);
            refusal.code
        }
    }
}

// JSON as `JSON::PP->new->utf8` reads it.

fn read_json(path: &Path) -> Result<Py, Refusal> {
    let bytes = fs::read(path).map_err(|error| unreadable(path, &error))?;
    Ok(decode(&bytes, &path.display().to_string())?)
}

fn decode(bytes: &[u8], label: &str) -> Result<Py, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| format!("{label} is not UTF-8"))?;
    pyjson::loads_strict(text).map_err(|error| format!("{label}: malformed JSON: {error}"))
}

/// How `JSON::PP`'s canonical encoder writes a decoded scalar: an integer longer than twenty characters was decoded
/// as a string; one within 64 bits prints exactly; every other number prints as Perl prints a double.
#[derive(PartialEq)]
enum Canonical {
    Null,
    Bool(bool),
    Number(String),
    Text(PyStr),
}

fn canonical_scalar(value: &Py) -> Option<Canonical> {
    Some(match value {
        Py::None => Canonical::Null,
        Py::Bool(flag) => Canonical::Bool(*flag),
        Py::Int(digits) if digits.len() > 20 => Canonical::Text(pyjson::py_str(digits)),
        Py::Int(digits) if digits.parse::<i64>().is_ok() || digits.parse::<u64>().is_ok() => {
            Canonical::Number(digits.clone())
        }
        Py::Int(digits) => Canonical::Number(perl::number(digits.parse().unwrap_or(0.0))),
        Py::Float(number) => Canonical::Number(perl::number(*number)),
        Py::Str(text) => Canonical::Text(text.clone()),
        Py::List(_) | Py::Dict(_) => return None,
    })
}

/// A decoded object's members, sorted by key.
fn sorted_pairs(pairs: &[(PyStr, Py)]) -> Vec<&(PyStr, Py)> {
    let mut pairs: Vec<&(PyStr, Py)> = pairs.iter().collect();
    pairs.sort_by(|left, right| left.0.cmp(&right.0));
    pairs
}

/// The value of the member named `key`.
fn section<'a>(pairs: &'a [(PyStr, Py)], key: &PyStr) -> Option<&'a Py> {
    pairs
        .iter()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value)
}

/// Whether two decoded values encode to the same canonical `JSON::PP` text.
fn canonical_eq(left: &Py, right: &Py) -> bool {
    match (canonical_scalar(left), canonical_scalar(right)) {
        (Some(left), Some(right)) => left == right,
        (None, None) => match (left, right) {
            (Py::List(left), Py::List(right)) => {
                left.len() == right.len()
                    && left
                        .iter()
                        .zip(right)
                        .all(|(left, right)| canonical_eq(left, right))
            }
            (Py::Dict(left), Py::Dict(right)) => {
                left.len() == right.len()
                    && sorted_pairs(left).iter().zip(sorted_pairs(right)).all(
                        |((left_key, left), (right_key, right))| {
                            left_key == right_key && canonical_eq(left, right)
                        },
                    )
            }
            _ => false,
        },
        _ => false,
    }
}

// The comparison core.

/// An artifact's top-level sections without the back-annotated ones, or the reason it cannot be one.
fn artifact_sections(value: Py, label: &str) -> Result<Vec<(PyStr, Py)>, String> {
    let Py::Dict(pairs) = value else {
        return Err(format!("{label} is not a JSON object"));
    };
    Ok(pairs
        .into_iter()
        .filter(|(key, _)| !EXCLUDED_SECTIONS.contains(&pyjson::to_text(key).as_str()))
        .collect())
}

/// A section's cardinality as the difference names it.
fn cardinality(section: Option<&Py>) -> String {
    match section {
        None => "absent".to_string(),
        Some(Py::List(items)) => items.len().to_string(),
        Some(Py::Dict(pairs)) => pairs.len().to_string(),
        Some(_) => "scalar".to_string(),
    }
}

/// The one content-identity predicate: whether two stage artifacts carry the same content, or their differing
/// top-level sections, sorted, as `name(persisted-cardinality->replay-cardinality)`.
fn compare_stage_artifact(persisted: &Path, replay: &Path) -> Verdict {
    let persisted = artifact_sections(read_json(persisted)?, &persisted.display().to_string())?;
    let replay = artifact_sections(read_json(replay)?, &replay.display().to_string())?;
    Ok(compare_sections(&persisted, &replay))
}

fn compare_sections(persisted: &[(PyStr, Py)], replay: &[(PyStr, Py)]) -> Result<(), String> {
    let keys: BTreeSet<&PyStr> = persisted.iter().chain(replay).map(|(key, _)| key).collect();
    let mut differing = Vec::new();
    for key in keys {
        let (left, right) = (section(persisted, key), section(replay, key));
        if let (Some(left), Some(right)) = (left, right)
            && canonical_eq(left, right)
        {
            continue;
        }
        differing.push(format!(
            "{}({}->{})",
            pyjson::to_text(key),
            cardinality(left),
            cardinality(right)
        ));
    }
    if differing.is_empty() {
        Ok(())
    } else {
        Err(differing.join(" "))
    }
}

/// `File::Basename::basename`: trailing separators dropped, then the last component; `/` stays `/`, and the empty
/// path is `./`.
fn basename(path: &str) -> String {
    let stripped = if path.is_empty() {
        path
    } else {
        match path.trim_end_matches('/') {
            "" => &path[..1],
            trimmed => trimmed,
        }
    };
    match stripped.rfind('/') {
        Some(slash) if slash + 1 < stripped.len() => stripped[slash + 1..].to_string(),
        Some(slash) => stripped[..=slash].to_string(),
        None if stripped.is_empty() => "./".to_string(),
        None => stripped.to_string(),
    }
}

/// `AdapterArtifact::write_to_disk` with `reconcile_emitted_isf_files`, mirrored: the persisted root holds exactly
/// the `.isf` the replayed adapter licenses, with exactly its rendered text.
fn compare_emitted_isf(adapter: &Path, artifact_root: &Path) -> Verdict {
    let label = adapter.display().to_string();
    let value = read_json(adapter)?;
    if !matches!(value, Py::Dict(_)) {
        return Err(format!("{label}: Not a HASH reference").into());
    }
    let isf = pyjson::get(&value, "isf");
    let renderable = if perl::truthy(isf) {
        match isf {
            Some(isf @ Py::Dict(_)) => perl::truthy(pyjson::get(isf, "is_renderable")),
            _ => return Err(format!("{label}: isf is not a HASH reference").into()),
        }
    } else {
        false
    };
    let emitted_path = match pyjson::get(&value, "artifact_layout") {
        None | Some(Py::None) => None,
        Some(layout @ Py::Dict(_)) => perl::scalar(pyjson::get(layout, "emitted_target_path")),
        Some(_) => return Err(format!("{label}: artifact_layout is not a HASH reference").into()),
    };
    let expected_name = match (&emitted_path, renderable) {
        (Some(path), true) => Some(basename(path)),
        _ => None,
    };
    let expected_text = if renderable {
        pyjson::get(isf.expect("a renderable isf"), "source_text")
    } else {
        None
    };
    let entries = fs::read_dir(artifact_root).map_err(|error| unreadable(artifact_root, &error))?;
    let mut present: Vec<String> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".isf"))
        .collect();
    present.sort();
    let mut breaches = Vec::new();
    match expected_name {
        None => {
            if !present.is_empty() {
                breaches.push(format!(
                    "stale emitted .isf present but the current adapter renders none: {}",
                    present.join(" ")
                ));
            }
        }
        Some(expected) => {
            let unexpected: Vec<&String> =
                present.iter().filter(|name| **name != expected).collect();
            if !unexpected.is_empty() {
                let names: Vec<&str> = unexpected.iter().map(|name| name.as_str()).collect();
                breaches.push(format!(
                    "unlicensed .isf sibling(s) retained: {}",
                    names.join(" ")
                ));
            }
            let file = artifact_root.join(&expected);
            if !file.is_file() {
                breaches.push(format!(
                    "the current adapter emits {expected} but it is not persisted"
                ));
            } else {
                let persisted = fs::read(&file).map_err(|error| unreadable(&file, &error))?;
                let rendered = perl::scalar(expected_text).unwrap_or_default();
                if persisted != rendered.as_bytes() {
                    breaches.push(format!(
                        "{expected} body differs from the rendered isf.source_text"
                    ));
                }
            }
        }
    }
    if breaches.is_empty() {
        Ok(Ok(()))
    } else {
        Ok(Err(breaches.join("; ")))
    }
}

/// The normalized bundle entry the evidence replay needs: the first line of the SourceIR that is exactly a
/// `"promoted_markdown_path": "<string>"` member, its string decoded, or nothing when the bundle was reclaimed.
fn promoted_markdown_path(source_ir: &Path) -> Option<String> {
    let bytes = fs::read(source_ir).ok()?;
    for line in bytes.split_inclusive(|byte| *byte == b'\n') {
        let Some(literal) = markdown_member(line) else {
            continue;
        };
        let value = std::str::from_utf8(literal)
            .ok()
            .and_then(|text| pyjson::loads_strict(text).ok());
        return match value {
            Some(Py::Str(text)) => Some(pyjson::to_text(&text).trim_end_matches('\n').to_string()),
            _ => None,
        };
    }
    None
}

/// Perl's `\s` on a byte string.
fn is_perl_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}

/// The string literal of a line matching `^\s*"promoted_markdown_path"\s*:\s*("(?:[^"\\]|\\.)*")\s*,?\s*$`.
fn markdown_member(line: &[u8]) -> Option<&[u8]> {
    let skip = |bytes: &[u8], at: usize| {
        at + bytes[at..]
            .iter()
            .take_while(|byte| is_perl_space(**byte))
            .count()
    };
    let mut at = skip(line, 0);
    let key = br#""promoted_markdown_path""#;
    if !line[at..].starts_with(key) {
        return None;
    }
    at = skip(line, at + key.len());
    if line.get(at) != Some(&b':') {
        return None;
    }
    at = skip(line, at + 1);
    if line.get(at) != Some(&b'"') {
        return None;
    }
    let start = at;
    at += 1;
    loop {
        match line.get(at)? {
            b'"' => break,
            b'\\' => {
                if matches!(line.get(at + 1), None | Some(b'\n')) {
                    return None;
                }
                at += 2;
            }
            _ => at += 1,
        }
    }
    let literal = &line[start..=at];
    at = skip(line, at + 1);
    if line.get(at) == Some(&b',') {
        at = skip(line, at + 1);
    }
    // `$` matches at the end, or before a final newline that `\s*` left unconsumed.
    (at == line.len()).then_some(literal)
}

// The retention leg.

/// Perl's `defined $value && !ref $value && $value =~ /\S/`.
fn nonempty_string(value: Option<&Py>) -> bool {
    match value {
        Some(value) if !perl::is_reference(value) => perl::scalar(Some(value))
            .is_some_and(|text| text.chars().any(|character| !character.is_whitespace())),
        _ => false,
    }
}

fn is_iso_date(text: &str) -> bool {
    let bytes = text.as_bytes();
    bytes.len() == 10
        && bytes.iter().enumerate().all(|(index, byte)| match index {
            4 | 7 => *byte == b'-',
            _ => byte.is_ascii_digit(),
        })
}

fn is_document_key(text: &str) -> bool {
    !text.is_empty()
        && text
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
}

/// The retained-bundle declaration, schema-closed, against the measured retained and corpus key sets: every breach,
/// or the one reason the declaration cannot be read at all.
fn compare_retention(
    declaration_path: &Path,
    retained: &BTreeSet<String>,
    corpus: &BTreeSet<String>,
) -> Vec<String> {
    let Ok(bytes) = fs::read(declaration_path) else {
        return vec![format!(
            "the retained-bundle declaration {} cannot be read",
            declaration_path.display()
        )];
    };
    let declaration = match decode(&bytes, "declaration") {
        Ok(value @ Py::Dict(_)) => value,
        _ => return vec!["the retained-bundle declaration is not a JSON object".to_string()],
    };
    let Py::Dict(pairs) = &declaration else {
        unreachable!("matched as an object")
    };
    let field = |name: &str| pyjson::get(&declaration, name);
    let required = [
        "schema_version",
        "contract_id",
        "owner_leaf",
        "authority",
        "declared_on",
        "retained",
        "reclamations",
    ];
    let mut breaches = Vec::new();
    let mut keys: Vec<String> = pairs.iter().map(|(key, _)| pyjson::to_text(key)).collect();
    keys.sort();
    for key in &keys {
        if !required.contains(&key.as_str()) {
            breaches.push(format!("unknown declaration field: {key}"));
        }
    }
    for key in required {
        if field(key).is_none() {
            breaches.push(format!("missing declaration field: {key}"));
        }
    }
    let scalar_is = |name: &str, wanted: &str| {
        field(name).is_some_and(|value| {
            !perl::is_reference(value) && perl::scalar(Some(value)).as_deref() == Some(wanted)
        })
    };
    if !scalar_is("schema_version", "1") {
        breaches.push("schema_version must be 1".to_string());
    }
    if !scalar_is("contract_id", "chain-currency-retained-bundles") {
        breaches.push("contract_id must be chain-currency-retained-bundles".to_string());
    }
    for key in ["owner_leaf", "authority"] {
        if field(key).is_some() && !nonempty_string(field(key)) {
            breaches.push(format!("{key} must be a non-empty string"));
        }
    }
    if let Some(declared_on) = field("declared_on")
        && (!nonempty_string(Some(declared_on))
            || !is_iso_date(&perl::scalar(Some(declared_on)).unwrap_or_default()))
    {
        breaches.push("declared_on must be an ISO date (YYYY-MM-DD)".to_string());
    }
    let mut declared: Vec<String> = Vec::new();
    match field("retained") {
        None => {}
        Some(Py::List(values)) => {
            let mut previous: Option<String> = None;
            for value in values {
                let key = perl::scalar(Some(value)).unwrap_or_default();
                declared.push(key.clone());
                if !nonempty_string(Some(value)) || !is_document_key(&key) {
                    breaches.push("retained holds a value that is not a document key".to_string());
                    continue;
                }
                if previous.as_ref().is_some_and(|previous| key <= *previous) {
                    breaches.push(format!("retained is not sorted and unique at: {key}"));
                }
                previous = Some(key);
            }
        }
        Some(_) => breaches.push("retained must be an array of document keys".to_string()),
    }
    let mut reclaimed: BTreeSet<String> = BTreeSet::new();
    match field("reclamations") {
        None => {}
        Some(Py::List(records)) => {
            let fields = ["document_key", "owning_leaf", "date", "reason"];
            let mut previous: Option<String> = None;
            for record in records {
                let Py::Dict(record_pairs) = record else {
                    breaches.push("reclamations holds a value that is not a record".to_string());
                    continue;
                };
                let mut names: Vec<String> = record_pairs
                    .iter()
                    .map(|(key, _)| pyjson::to_text(key))
                    .collect();
                names.sort();
                for name in &names {
                    if !fields.contains(&name.as_str()) {
                        breaches.push(format!("unknown reclamation field: {name}"));
                    }
                }
                for name in fields {
                    let value = pyjson::get(record, name);
                    if value.is_none() {
                        breaches.push(format!("reclamation is missing {name}"));
                    }
                    if value.is_some() && !nonempty_string(value) {
                        breaches.push(format!("reclamation {name} must be a non-empty string"));
                    }
                }
                let date = pyjson::get(record, "date");
                if nonempty_string(date) && !is_iso_date(&perl::scalar(date).unwrap_or_default()) {
                    breaches.push("reclamation date must be an ISO date (YYYY-MM-DD)".to_string());
                }
                let key = pyjson::get(record, "document_key");
                if !nonempty_string(key) {
                    continue;
                }
                let key = perl::scalar(key).unwrap_or_default();
                if previous.as_ref().is_some_and(|previous| key <= *previous) {
                    breaches.push(format!("reclamations is not sorted and unique at: {key}"));
                }
                previous = Some(key.clone());
                reclaimed.insert(key);
            }
        }
        Some(_) => breaches.push("reclamations must be an array of records".to_string()),
    }
    for key in &declared {
        if reclaimed.contains(key) {
            breaches.push(format!("{key} is declared both retained and reclaimed"));
        }
    }
    for key in &declared {
        if retained.contains(key) {
            continue;
        }
        breaches.push(if corpus.contains(key) {
            format!(
                "{key} is declared retained but its normalized bundle is absent — an unauthorised reclamation"
            )
        } else {
            format!("{key} is declared retained but is no longer a corpus document")
        });
    }
    let declared: BTreeSet<&String> = declared.iter().collect();
    for key in retained {
        if !declared.contains(key) {
            breaches.push(format!(
                "{key} retains a normalized bundle that no leaf declared — record it as retained"
            ));
        }
    }
    breaches
}

// Small predicates the gate's arithmetic and classification rest on.

/// A deliberately quarantined legacy proof (the product's closed compatibility diagnostic), not a replay failure:
/// `schema version <digits> is legacy/proofless and inspection-only; rebuild it from verified `.
fn replay_is_legacy_proof_blocked(stderr: &[u8]) -> bool {
    let marker = b"schema version ";
    let tail = b" is legacy/proofless and inspection-only; rebuild it from verified ";
    (0..stderr.len()).any(|start| {
        stderr[start..].starts_with(marker) && {
            let digits_at = start + marker.len();
            let digits = stderr[digits_at..]
                .iter()
                .take_while(|byte| byte.is_ascii_digit())
                .count();
            digits > 0 && stderr[digits_at + digits..].starts_with(tail)
        }
    })
}

/// Content-current artifacts: the compared ones less the comparison-stale ones, never negative.
fn current_from_counts(compared: usize, compared_stale: usize) -> Option<usize> {
    compared.checked_sub(compared_stale)
}

/// A blocked adapter's proven absence is an output state, not an emitted file.
fn adapter_emission_summary(states: usize, files: usize) -> Option<String> {
    (files <= states).then(|| {
        format!(
            "{states} adapter output state(s) checked ({files} emitted .isf file(s), {} blocked/no-file state(s))",
            states - files
        )
    })
}

/// The lines after the first one ending `_json:`, as `awk 'flag { print } /_json:$/ { flag = 1 }'` prints them.
fn replay_json(raw: &[u8]) -> Vec<u8> {
    let mut records: Vec<&[u8]> = raw.split(|byte| *byte == b'\n').collect();
    if raw.ends_with(b"\n") {
        records.pop();
    }
    let mut out = Vec::new();
    let mut flag = false;
    for record in records {
        if flag {
            out.extend_from_slice(record);
            out.push(b'\n');
        }
        if record.ends_with(b"_json:") {
            flag = true;
        }
    }
    out
}

// The gate.

/// The corpus documents with a persisted SourceIR, in the order the script's glob gave them: each visible
/// directory's `source_ir.json`, sorted by its whole path.
fn source_irs(generated: &str) -> Vec<(String, String)> {
    let Ok(entries) = fs::read_dir(format!("{generated}/source_ir")) else {
        return Vec::new();
    };
    let mut paths: Vec<(String, String)> = entries
        .flatten()
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|key| !key.starts_with('.'))
        .map(|key| (format!("{generated}/source_ir/{key}/source_ir.json"), key))
        .filter(|(path, _)| Path::new(path).is_file())
        .collect();
    paths.sort();
    paths
}

/// A scratch path under the project-data temporary root, as `mktemp` would choose.
fn scratch(environment: &Environment, kind: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_nanos());
    project_data::path(environment, "TMPDIR").join(format!(
        "chain-currency-{kind}.{}.{nanos}",
        std::process::id()
    ))
}

/// A command with the activated environment and the script's `LC_ALL=C`.
fn command(
    program: impl AsRef<std::ffi::OsStr>,
    root: &Path,
    environment: &Environment,
) -> Command {
    let mut command = Command::new(program);
    command.current_dir(root).env("LC_ALL", "C");
    for (name, value) in environment {
        command.env(name, value);
    }
    command
}

/// Removes the gate's scratch files however it returns, as the script's `trap … EXIT` did.
struct Scratch(Vec<PathBuf>);

impl Drop for Scratch {
    fn drop(&mut self) {
        for path in &self.0 {
            let _ = fs::remove_dir_all(path).or_else(|_| fs::remove_file(path));
        }
    }
}

/// Ask `scripts/lib/corpus_replay_binary.sh` for the build that answers, as every corpus-replay entry point does.
fn corpus_replay_build(root: &Path, environment: &Environment, log: &Path) -> Option<String> {
    let output = command("bash", root, environment)
        .args([
            "-c",
            "ROOT=\"$1\"; . \"$ROOT/scripts/lib/corpus_replay_binary.sh\"; corpus_replay_build \"$2\"",
            "corpus-replay-build",
        ])
        .arg(root)
        .arg(log)
        .stderr(Stdio::inherit())
        .output()
        .ok()?;
    output.status.success().then(|| {
        String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .to_string()
    })
}

fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path).is_ok_and(|metadata| metadata.permissions().mode() & 0o111 != 0)
}

fn gate(root: &Path) -> i32 {
    let environment = match project_data::activate(root) {
        Ok(environment) => environment,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    if let Err(error) = std::env::set_current_dir(root) {
        fail_note(&format!("cannot enter {}: {error}", root.display()));
        return 1;
    }
    let generated = std::env::var("SPECFORGE_CHAIN_CURRENCY_GENERATED_ROOT")
        .unwrap_or_else(|_| DEFAULT_GENERATED_ROOT.to_string());
    let retention_contract = std::env::var("SPECFORGE_CHAIN_CURRENCY_RETENTION_CONTRACT")
        .unwrap_or_else(|_| DEFAULT_RETENTION_CONTRACT.to_string());
    if !Path::new(&format!("{generated}/source_ir")).is_dir() {
        note(&format!(
            "SKIP: no corpus at {generated}/source_ir — a fresh clone and hosted CI have none, so"
        ));
        note(
            "SKIP: this doctrine does not govern this tree. Nothing was measured; nothing is claimed.",
        );
        return 0;
    }
    let log = scratch(&environment, "build");
    let mut cleanup = Scratch(vec![log.clone()]);
    let Some(binary) = corpus_replay_build(root, &environment, &log) else {
        fail_note("cargo could not build the specforge binary, so no replay is possible:");
        eprint!(
            "{}",
            String::from_utf8_lossy(&fs::read(&log).unwrap_or_default())
        );
        return 1;
    };
    if !is_executable(Path::new(&binary)) {
        fail_note(&format!(
            "cargo reported success but {binary} is not an executable"
        ));
        return 1;
    }
    let work = scratch(&environment, "work");
    if fs::create_dir_all(&work).is_err() {
        fail_note("cannot create a repository-local replay workspace");
        return 1;
    }
    cleanup.0.push(work);
    let documents = source_irs(&generated);
    let mut fail = false;
    note(&format!(
        "corpus root: {generated} — {} document(s) with a persisted SourceIR",
        documents.len()
    ));
    for stage in STAGES {
        if replay_stage(stage, &generated, &documents, &binary, root, &environment) {
            fail = true;
        }
    }
    if fail {
        fail_note("the persisted corpus chain is NOT current with this binary.");
        fail_note(
            "Rebuild every drifted document under its owning CORPUS-COVERAGE leaf and attribute each",
        );
        fail_note(
            "delta to this change or to a named earlier leaf (ADR 0025 decision 1). Do not bypass.",
        );
    }
    if !Path::new(&retention_contract).is_file() {
        fail_note(&format!(
            "the retained-bundle declaration is missing: {retention_contract}"
        ));
        fail_note(
            "It is this doctrine data, not a convention, that makes bundle retention checkable.",
        );
        return 1;
    }
    let mut corpus = BTreeSet::new();
    let mut retained = BTreeSet::new();
    for (source_ir, key) in &documents {
        corpus.insert(key.clone());
        if promoted_markdown_path(Path::new(source_ir))
            .is_some_and(|markdown| !markdown.is_empty() && Path::new(&markdown).is_file())
        {
            retained.insert(key.clone());
        }
    }
    let breaches = compare_retention(Path::new(&retention_contract), &retained, &corpus);
    if breaches.is_empty() {
        note(&format!(
            "retention: {} normalized bundle(s) on disk — exactly the declared retained set.",
            retained.len()
        ));
    } else {
        for breach in &breaches {
            fail_note(&format!("retention — {breach}"));
        }
        fail_note(
            "Retention is deliberate and task-owned (ADR 0025 decision 3): a refresh keeps its bundle",
        );
        fail_note(&format!(
            "and records it in {retention_contract}; a reclamation is recorded there with its owning"
        ));
        fail_note(
            "leaf and reason. Do not reconcile by deleting evidence or by widening the declaration.",
        );
        fail = true;
    }
    if fail {
        return 1;
    }
    note("every measurable persisted artifact is exactly what the current binary produces.");
    0
}

/// One stage across the corpus: replay each persisted artifact from its persisted input and compare. Prints the
/// breaches and the stage's summary; true when the stage failed.
fn replay_stage(
    stage: &str,
    generated: &str,
    documents: &[(String, String)],
    binary: &str,
    root: &Path,
    environment: &Environment,
) -> bool {
    let (mut compared, mut not_persisted, mut unmeasurable, mut proof_blocked) = (0, 0, 0, 0);
    let (mut orphaned, mut stale, mut compared_stale) = (0, 0, 0);
    let (mut emission_states_checked, mut emitted_files_checked) = (0, 0);
    let mut fail = false;
    for (source_ir, key) in documents {
        let (input, persisted) = match stage {
            "evidence" => (
                source_ir.clone(),
                format!("{generated}/evidence_ir/{key}/evidence_ir.json"),
            ),
            "semantic" => (
                format!("{generated}/evidence_ir/{key}/evidence_ir.json"),
                format!("{generated}/semantic_ir/{key}/semantic_ir.json"),
            ),
            "intent" => (
                format!("{generated}/semantic_ir/{key}/semantic_ir.json"),
                format!("{generated}/intent_ir/{key}/intent_ir.json"),
            ),
            _ => (
                format!("{generated}/intent_ir/{key}/intent_ir.json"),
                format!("{generated}/adapters/isf/{key}/adapter.json"),
            ),
        };
        if !Path::new(&persisted).is_file() {
            not_persisted += 1;
            continue;
        }
        if !Path::new(&input).is_file() {
            fail_note(&format!(
                "{key} {stage} — persisted artifact has no input to replay from: {input}"
            ));
            orphaned += 1;
            fail = true;
            continue;
        }
        if stage == "evidence"
            && !promoted_markdown_path(Path::new(source_ir))
                .is_some_and(|markdown| !markdown.is_empty() && Path::new(&markdown).is_file())
        {
            unmeasurable += 1;
            continue;
        }
        let mut replay = command(binary, root, environment);
        match stage {
            "evidence" => replay.args(["evidence", &input, "--dry-run"]),
            "semantic" => replay.args(["semantic", &input, "--dry-run"]),
            "intent" => replay.args(["intent", &input, "--dry-run"]),
            _ => replay.args(["adapt", &input, "--target", "isf", "--dry-run"]),
        };
        let output = replay.stdin(Stdio::inherit()).output();
        let (status_ok, raw, err) = match output {
            Ok(output) => (output.status.success(), output.stdout, output.stderr),
            Err(error) => (false, Vec::new(), format!("{error}\n").into_bytes()),
        };
        if !status_ok {
            if replay_is_legacy_proof_blocked(&err) {
                proof_blocked += 1;
                continue;
            }
            let flattened: Vec<u8> = err
                .iter()
                .map(|byte| if *byte == b'\n' { b' ' } else { *byte })
                .collect();
            fail_note(&format!(
                "{key} {stage} — the current binary cannot replay the persisted input: {}",
                String::from_utf8_lossy(&flattened)
            ));
            stale += 1;
            fail = true;
            continue;
        }
        let replay_bytes = replay_json(&raw);
        if replay_bytes.is_empty() {
            fail_note(&format!(
                "{key} {stage} — the replay emitted no artifact JSON (expected a '*_json:' section)"
            ));
            stale += 1;
            fail = true;
            continue;
        }
        compared += 1;
        let verdict = (|| -> Verdict {
            let persisted = artifact_sections(read_json(Path::new(&persisted))?, &persisted)?;
            let replay = artifact_sections(decode(&replay_bytes, "replay.json")?, "replay.json")?;
            Ok(compare_sections(&persisted, &replay))
        })();
        if let Some(differing) = difference(verdict) {
            fail_note(&format!(
                "{key} {stage} — the persisted artifact is NOT what the current binary produces: {differing}"
            ));
            stale += 1;
            compared_stale += 1;
            fail = true;
            continue;
        }
        if stage == "isf-adapter" {
            emission_states_checked += 1;
            // `jq -e '.artifact_layout.emitted_target_path != null'`: any value but null counts, and a layout that
            // is not an object does not.
            let emitted = decode(&replay_bytes, "replay.json").is_ok_and(|value| {
                pyjson::get(&value, "artifact_layout")
                    .and_then(|layout| pyjson::get(layout, "emitted_target_path"))
                    .is_some_and(|path| !matches!(path, Py::None))
            });
            if emitted {
                emitted_files_checked += 1;
            }
            let verdict = write_replay(environment, &replay_bytes).and_then(|path| {
                let verdict = compare_emitted_isf(
                    &path,
                    Path::new(&format!("{generated}/adapters/isf/{key}")),
                );
                let _ = fs::remove_file(&path);
                verdict
            });
            if let Some(breach) = difference(verdict) {
                fail_note(&format!("{key} isf-emit — {breach}"));
                stale += 1;
                compared_stale += 1;
                fail = true;
            }
        }
    }
    let current = match current_from_counts(compared, compared_stale) {
        Some(current) => current,
        None => {
            fail_note(&format!(
                "{stage} — internal currency counts are inconsistent: {compared} compared / {compared_stale} comparison-stale"
            ));
            fail = true;
            0
        }
    };
    let mut summary = format!(
        "{stage}: {compared} replayed, {current} current, {stale} stale, {not_persisted} not persisted"
    );
    if stage == "evidence" {
        summary.push_str(&format!(
            ", {unmeasurable} UNMEASURABLE (normalized bundle reclaimed — needs re-ingest)"
        ));
    }
    if proof_blocked != 0 {
        summary.push_str(&format!(
            ", {proof_blocked} UNMEASURABLE (legacy/proofless upstream is inspection-only)"
        ));
    }
    if orphaned != 0 {
        summary.push_str(&format!(", {orphaned} orphaned"));
    }
    if stage == "isf-adapter" {
        let emission = match adapter_emission_summary(
            emission_states_checked,
            emitted_files_checked,
        ) {
            Some(emission) => emission,
            None => {
                fail_note(&format!(
                    "{stage} — internal emission counts are inconsistent: {emission_states_checked} states / {emitted_files_checked} files"
                ));
                fail = true;
                "invalid adapter emission counts".to_string()
            }
        };
        summary.push_str(&format!("; {emission}"));
    }
    note(&summary);
    fail
}

/// A predicate's verdict as the script read it: `None` when the same; the difference otherwise, empty when the
/// predicate could not read its input (its reason printed, as Perl's `die` printed it).
fn difference(verdict: Verdict) -> Option<String> {
    match verdict {
        Ok(Ok(())) => None,
        Ok(Err(difference)) => Some(difference),
        Err(refusal) => {
            eprintln!("{}", refusal.reason);
            Some(String::new())
        }
    }
}

/// The replayed adapter, written where the emitted-`.isf` predicate can read it.
fn write_replay(environment: &Environment, bytes: &[u8]) -> Result<PathBuf, Refusal> {
    let path = scratch(environment, "replay").with_extension("json");
    fs::write(&path, bytes).map_err(|error| format!("cannot write {}: {error}", path.display()))?;
    Ok(path)
}

// The self-test.

/// Prove the comparison core fail-closed before trusting its pass: 22 cases, the absent-corpus skip proven end to
/// end through the registered script.
fn self_test(root: &Path) -> i32 {
    let environment = match project_data::activate(root) {
        Ok(environment) => environment,
        Err(error) => {
            eprintln!("{error}");
            return 1;
        }
    };
    let work = scratch(&environment, "self-test");
    if fs::create_dir_all(work.join("isf")).is_err() {
        fail_note("cannot create a repository-local self-test workspace");
        return 1;
    }
    let passed = self_test_cases(root, &environment, &work);
    let _ = fs::remove_dir_all(&work);
    if passed != EXPECTED_SELF_TEST_CASES {
        fail_note(&format!(
            "self-test {passed}/{EXPECTED_SELF_TEST_CASES} passed"
        ));
        return 1;
    }
    note(&format!(
        "self-test {passed}/{EXPECTED_SELF_TEST_CASES} passed."
    ));
    0
}

fn self_test_cases(root: &Path, environment: &Environment, work: &Path) -> usize {
    let write = |name: &str, text: &str| {
        let path = work.join(name);
        let _ = fs::write(&path, text);
        path
    };
    let same = |persisted: &Path, replay: &Path| {
        matches!(compare_stage_artifact(persisted, replay), Ok(Ok(())))
    };
    let difference =
        |persisted: &Path, replay: &Path| match compare_stage_artifact(persisted, replay) {
            Ok(Err(difference)) => Some(difference),
            _ => None,
        };
    let mut passed = 0;
    let mut check = |ok: bool, failure: String| {
        if ok {
            passed += 1;
        } else {
            fail_note(&failure);
        }
    };
    let base = write(
        "base.json",
        r#"{"stage":"evidence_ir","timing_constraints":[1,2],"actors":{"a":1},"validation_reports":[],"proof_context":{"mutations":[]},"proof_ledger":{"digest":"base"}}"#,
    );
    let reported = write(
        "reported.json",
        r#"{"stage":"evidence_ir","timing_constraints":[1,2],"actors":{"a":1},"validation_reports":[{"report_id":"r"}],"proof_context":{"mutations":["validation"]},"proof_ledger":{"digest":"validated"}}"#,
    );
    let shrunk = write(
        "shrunk.json",
        r#"{"stage":"evidence_ir","timing_constraints":[1],"actors":{"a":1},"validation_reports":[]}"#,
    );
    let missing = write(
        "missing.json",
        r#"{"stage":"evidence_ir","actors":{"a":1},"validation_reports":[]}"#,
    );
    let reordered = write(
        "reordered.json",
        r#"{"actors":{"a":1},"stage":"evidence_ir","validation_reports":[],"timing_constraints":[1,2]}"#,
    );
    check(
        same(&base, &base),
        "self-test 1: an artifact did not match itself".to_string(),
    );
    check(
        same(&reported, &base),
        "self-test 2: a validation_reports-only difference was reported as staleness".to_string(),
    );
    check(
        same(&reordered, &base),
        "self-test 3: re-ordered keys were reported as staleness".to_string(),
    );
    let output = difference(&base, &shrunk).unwrap_or_default();
    check(
        output == "timing_constraints(2->1)",
        format!(
            "self-test 4: a shrunken section was not caught as 'timing_constraints(2->1)' (got '{output}')"
        ),
    );
    let output = difference(&base, &missing).unwrap_or_default();
    check(
        output == "timing_constraints(2->absent)",
        format!("self-test 5: a vanished section was not caught (got '{output}')"),
    );

    let renderable = write(
        "renderable.json",
        r#"{"artifact_layout":{"emitted_target_path":"generated/adapters/isf/d/wiring_patterns.isf"},"isf":{"is_renderable":true,"source_text":"actor d {}\n"}}"#,
    );
    let blocked = write(
        "blocked.json",
        r#"{"artifact_layout":{"emitted_target_path":null},"isf":{"is_renderable":false,"source_text":""}}"#,
    );
    let isf_root = work.join("isf");
    let emitted = isf_root.join("wiring_patterns.isf");
    let isf_ok = |adapter: &Path| matches!(compare_emitted_isf(adapter, &isf_root), Ok(Ok(())));
    let _ = fs::write(&emitted, "actor d {}\n");
    check(
        isf_ok(&renderable),
        "self-test 6: a faithfully emitted .isf was reported as drifted".to_string(),
    );
    check(
        !isf_ok(&blocked),
        "self-test 7: a stale .isf under a non-renderable adapter was not caught".to_string(),
    );
    let _ = fs::write(&emitted, "actor d { drifted }\n");
    check(
        !isf_ok(&renderable),
        "self-test 8: a drifted .isf body was not caught".to_string(),
    );
    let _ = fs::remove_file(&emitted);
    check(
        !isf_ok(&renderable),
        "self-test 9: a missing emission was not caught".to_string(),
    );

    let skip = command(
        root.join("scripts/check_chain_currency.sh"),
        root,
        environment,
    )
    .arg("--check")
    .env(
        "SPECFORGE_CHAIN_CURRENCY_GENERATED_ROOT",
        work.join("absent-corpus"),
    )
    .output();
    let (status, output) = match skip {
        Ok(output) => {
            let mut text = String::from_utf8_lossy(&output.stdout).into_owned();
            text.push_str(&String::from_utf8_lossy(&output.stderr));
            (
                output.status.code().unwrap_or(1),
                text.trim_end_matches('\n').to_string(),
            )
        }
        Err(error) => (1, error.to_string()),
    };
    let status = if output.contains("SKIP") { status } else { 99 };
    check(
        status == 0,
        format!(
            "self-test 10: an absent corpus root did not skip loudly (status {status}, output '{output}')"
        ),
    );

    check(
        replay_is_legacy_proof_blocked(
            b"error: EvidenceIR schema version 2 is legacy/proofless and inspection-only; rebuild it from verified SourceIR\n",
        ),
        "self-test 11: a deliberately quarantined legacy proof was not classified as unmeasurable".to_string(),
    );
    check(
        !replay_is_legacy_proof_blocked(
            b"error: EvidenceIR proof verification failed: proof ledger ruleset hash is stale\n",
        ),
        "self-test 12: a stale current proof was incorrectly downgraded to unmeasurable"
            .to_string(),
    );
    check(
        current_from_counts(0, 0) == Some(0),
        "self-test 13: an empty replay population did not report zero current".to_string(),
    );
    check(
        current_from_counts(0, 24).is_none(),
        "self-test 14: stale replay failures were allowed to produce a negative current count"
            .to_string(),
    );
    let output = adapter_emission_summary(24, 0).unwrap_or_default();
    check(
        output
            == "24 adapter output state(s) checked (0 emitted .isf file(s), 24 blocked/no-file state(s))",
        format!(
            "self-test 15: blocked adapter states were reported as emitted files (got '{output}')"
        ),
    );
    check(
        adapter_emission_summary(1, 2).is_none(),
        "self-test 16: more emitted files than checked adapter states was accepted".to_string(),
    );

    let keys = |names: &[&str]| {
        names
            .iter()
            .map(|name| (*name).to_string())
            .collect::<BTreeSet<_>>()
    };
    let corpus = keys(&["alpha", "beta", "gamma"]);
    let retention = write(
        "retention.json",
        r#"{"schema_version":1,"contract_id":"chain-currency-retained-bundles","owner_leaf":"T.1","authority":"a","declared_on":"2026-08-10","retained":["alpha","beta"],"reclamations":[{"document_key":"gamma","owning_leaf":"T.2","date":"2026-08-10","reason":"r"}]}"#,
    );
    check(
        compare_retention(&retention, &keys(&["alpha", "beta"]), &corpus).is_empty(),
        "self-test 17: an exactly-declared retained set was reported as a breach".to_string(),
    );
    let breach_case = |declaration: &Path, retained: &[&str], wanted: &str| {
        let breaches = compare_retention(declaration, &keys(retained), &corpus).join("\n");
        (breaches.contains(wanted), breaches)
    };
    let (ok, output) = breach_case(
        &retention,
        &["alpha"],
        "beta is declared retained but its normalized bundle is absent",
    );
    check(
        ok,
        format!("self-test 18: a reclaimed declared bundle was not caught (got '{output}')"),
    );
    let (ok, output) = breach_case(
        &retention,
        &["alpha", "beta", "gamma"],
        "gamma retains a normalized bundle that no leaf declared",
    );
    check(
        ok,
        format!("self-test 19: an undeclared retained bundle was not caught (got '{output}')"),
    );
    let unknown = write(
        "retention-unknown.json",
        r#"{"schema_version":1,"contract_id":"chain-currency-retained-bundles","owner_leaf":"T.1","authority":"a","declared_on":"2026-08-10","retained":["alpha","beta"],"reclamations":[],"note":"free-form"}"#,
    );
    let (ok, output) = breach_case(
        &unknown,
        &["alpha", "beta"],
        "unknown declaration field: note",
    );
    check(
        ok,
        format!("self-test 20: an unknown declaration field was not caught (got '{output}')"),
    );
    let unsorted = write(
        "retention-unsorted.json",
        r#"{"schema_version":1,"contract_id":"chain-currency-retained-bundles","owner_leaf":"T.1","authority":"a","declared_on":"2026-08-10","retained":["beta","alpha"],"reclamations":[]}"#,
    );
    let (ok, output) = breach_case(
        &unsorted,
        &["alpha", "beta"],
        "retained is not sorted and unique at: alpha",
    );
    check(
        ok,
        format!("self-test 21: an unsorted retained list was not caught (got '{output}')"),
    );
    let contradictory = write(
        "retention-contradictory.json",
        r#"{"schema_version":1,"contract_id":"chain-currency-retained-bundles","owner_leaf":"T.1","authority":"a","declared_on":"2026-08-10","retained":["alpha","beta"],"reclamations":[{"document_key":"beta","owning_leaf":"T.2","date":"2026-08-10","reason":"r"}]}"#,
    );
    let (ok, output) = breach_case(
        &contradictory,
        &["alpha", "beta"],
        "beta is declared both retained and reclaimed",
    );
    check(
        ok,
        format!(
            "self-test 22: a key declared both retained and reclaimed was not caught (got '{output}')"
        ),
    );
    passed
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sections(text: &str) -> Vec<(PyStr, Py)> {
        artifact_sections(pyjson::loads_strict(text).expect("JSON"), "fixture").expect("an object")
    }

    #[test]
    fn values_compare_as_their_json_pp_canonical_encodings() {
        for (left, right, same) in [
            (r#"{"a":1.0}"#, r#"{"a":1}"#, true),
            (r#"{"a":1e2}"#, r#"{"a":100}"#, true),
            (r#"{"a":0.10}"#, r#"{"a":0.1}"#, true),
            (
                r#"{"a":123456789012345678901}"#,
                r#"{"a":"123456789012345678901"}"#,
                true,
            ),
            (
                r#"{"a":12345678901234567890}"#,
                r#"{"a":12345678901234567891}"#,
                false,
            ),
            (
                r#"{"a":99999999999999999999}"#,
                r#"{"a":99999999999999999998}"#,
                true,
            ),
            (r#"{"a":"1"}"#, r#"{"a":1}"#, false),
            (r#"{"a":true}"#, r#"{"a":1}"#, false),
            (r#"{"a":{"x":1,"y":2}}"#, r#"{"a":{"y":2,"x":1}}"#, true),
            (r#"{"a":[1,2]}"#, r#"{"a":[2,1]}"#, false),
            (
                r#"{"a":1,"validation_reports":[1],"proof_ledger":{}}"#,
                r#"{"a":1}"#,
                true,
            ),
        ] {
            assert_eq!(
                compare_sections(&sections(left), &sections(right)).is_ok(),
                same,
                "{left} {right}"
            );
        }
        assert_eq!(
            compare_sections(
                &sections(r#"{"l":[1,2,3],"d":{"x":1},"s":"t","n":null}"#),
                &sections(r#"{"l":[1],"d":{},"s":"u"}"#)
            ),
            Err("d(1->0) l(3->1) n(scalar->absent) s(scalar->scalar)".to_string())
        );
    }

    #[test]
    fn the_line_readers_follow_the_scripts_patterns() {
        for (path, name) in [
            ("g/x/w.isf/", "w.isf"),
            ("/", "/"),
            ("", "./"),
            ("a", "a"),
            ("a//b", "b"),
        ] {
            assert_eq!(basename(path), name, "{path:?}");
        }
        let member = |line: &str| {
            markdown_member(line.as_bytes())
                .map(|literal| String::from_utf8_lossy(literal).into_owned())
        };
        assert_eq!(
            member("  \"promoted_markdown_path\" : \"a/b.md\",\n").as_deref(),
            Some("\"a/b.md\"")
        );
        assert_eq!(
            member("\"promoted_markdown_path\": \"a\\\"b\"\n").as_deref(),
            Some("\"a\\\"b\"")
        );
        assert_eq!(member("\"promoted_markdown_path\": \"a\" x\n"), None);
        assert_eq!(member("\"promoted_markdown_path\": null\n"), None);
        assert_eq!(replay_json(b"log\nevidence_ir_json:\n{\n}"), b"{\n}\n");
        assert_eq!(replay_json(b"no marker\n"), b"");
        assert_eq!(
            (current_from_counts(3, 1), current_from_counts(0, 24)),
            (Some(2), None)
        );
        assert_eq!(adapter_emission_summary(1, 2), None);
        assert!(replay_is_legacy_proof_blocked(
            b"x\nerror: schema version 12 is legacy/proofless and inspection-only; rebuild it from verified SourceIR\n"
        ));
        assert!(!replay_is_legacy_proof_blocked(
            b"schema version  is legacy/proofless and inspection-only; rebuild it from verified "
        ));
    }

    #[test]
    fn the_shared_library_reaches_each_predicate_by_its_own_mode() {
        let library = fs::read_to_string(
            crate::repository_root().join("scripts/lib/stage_artifact_identity.sh"),
        )
        .expect("the shared library");
        let body = |function: &str| {
            let start = library
                .find(&format!("{function}() {{"))
                .expect("the function");
            library[start..]
                .split("\n}")
                .next()
                .expect("its body")
                .to_string()
        };
        assert!(body("compare_stage_artifact").contains("--compare-stage-artifact \"$1\" \"$2\""));
        assert!(body("compare_emitted_isf").contains("--compare-emitted-isf \"$1\" \"$2\""));
    }

    #[test]
    fn retention_and_emission_refuse_what_the_declaration_and_the_adapter_do_not_license() {
        let work = crate::repository_root().join(".project-data/tmp/xtask-chain-currency-test");
        let _ = fs::remove_dir_all(&work);
        fs::create_dir_all(work.join("isf")).expect("the fixture's directories");
        let declaration = work.join("retention.json");
        fs::write(
            &declaration,
            r#"{"schema_version":1,"contract_id":"chain-currency-retained-bundles","owner_leaf":"T.1","authority":"a","declared_on":"2026-08-10","retained":["alpha","alpha"],"reclamations":[]}"#,
        )
        .expect("a declaration");
        let keys: BTreeSet<String> = ["alpha".to_string()].into();
        let breaches = compare_retention(&declaration, &keys, &keys);
        let adapter = work.join("adapter.json");
        fs::write(
            &adapter,
            r#"{"artifact_layout":{"emitted_target_path":"g/w.isf"},"isf":{"is_renderable":true,"source_text":"a"}}"#,
        )
        .expect("an adapter");
        fs::write(work.join("isf/w.isf"), "a").expect("the emission");
        fs::write(work.join("isf/old.isf"), "b").expect("a stale sibling");
        let emission = compare_emitted_isf(&adapter, &work.join("isf"));
        fs::remove_dir_all(&work).expect("cleanup");
        assert_eq!(
            breaches,
            vec!["retained is not sorted and unique at: alpha".to_string()]
        );
        assert!(
            matches!(emission, Ok(Err(breach)) if breach == "unlicensed .isf sibling(s) retained: old.isf")
        );
    }
}
