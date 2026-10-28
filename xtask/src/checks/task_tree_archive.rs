//! Task-tree archive contracts, native since `RUSTDOC-AND-XTASK.15.o`: source identity, migration
//! manifests, bounded current/index surfaces, exact local routes and the 15-case fixture suite.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "task-tree-archive";

#[derive(Clone, Copy, Default)]
struct Metrics {
    lines: usize,
    bytes: usize,
    line_bytes: usize,
}

struct ResultData {
    state: String,
    source_path: String,
    source_sha: String,
    source_metrics: Metrics,
    warnings: Vec<String>,
    current_metrics: Option<Metrics>,
    index_metrics: Option<Metrics>,
    manifest_metrics: Option<Metrics>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Check,
    Report,
    SelfTest,
}

fn text(value: &str) -> Py {
    Py::Str(pyjson::py_str(value))
}

fn integer(value: usize) -> Py {
    Py::Int(value.to_string())
}

fn object(pairs: Vec<(&str, Py)>) -> Py {
    Py::Dict(
        pairs
            .into_iter()
            .map(|(key, value)| (pyjson::py_str(key), value))
            .collect(),
    )
}

fn metrics_json(metrics: Metrics) -> Py {
    object(vec![
        ("lines", integer(metrics.lines)),
        ("bytes", integer(metrics.bytes)),
        ("line_bytes", integer(metrics.line_bytes)),
    ])
}

fn encode(value: &Py, indent: Option<usize>) -> Result<String, String> {
    pyjson::dumps(
        value,
        &pyjson::Encoder {
            indent,
            sort_keys: true,
            ensure_ascii: false,
            item_separator: ",",
            key_separator: ":",
        },
    )
    .ok_or_else(|| "cannot encode JSON".to_string())
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn scalar(value: Option<&Py>) -> String {
    perl::scalar(value).unwrap_or_default()
}

fn safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && !path.contains("//")
        && !path.split('/').any(|part| matches!(part, "." | ".."))
        && !path
            .chars()
            .any(|character| character <= '\u{1f}' || character == '\u{7f}')
}

#[cfg(unix)]
fn device(path: &Path) -> u64 {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path)
        .map(|metadata| metadata.dev())
        .unwrap_or(0)
}

#[cfg(not(unix))]
fn device(_path: &Path) -> u64 {
    0
}

fn read_regular(
    base: &Path,
    relative: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<Vec<u8>> {
    if !safe_relative_path(relative) {
        errors.push(format!("{label} path is absolute, escaping, or malformed"));
        return None;
    }
    let path = base.join(relative);
    if !path.is_file()
        || fs::symlink_metadata(&path).is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        errors.push(format!(
            "{label} is missing or not a regular non-symlink file: {relative}"
        ));
        return None;
    }
    if device(base) != device(&path) {
        errors.push(format!("{label} is off the repository volume: {relative}"));
    }
    match fs::read(&path) {
        Ok(raw) => Some(raw),
        Err(error) => {
            errors.push(format!("cannot read {label} '{relative}': {error}"));
            None
        }
    }
}

fn metrics(raw: &[u8]) -> Metrics {
    Metrics {
        lines: raw.iter().filter(|byte| **byte == b'\n').count()
            + usize::from(!raw.is_empty() && !raw.ends_with(b"\n")),
        bytes: raw.len(),
        line_bytes: raw
            .split(|byte| *byte == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line).len())
            .max()
            .unwrap_or(0),
    }
}

fn occurrences(raw: &str, literal: &str) -> usize {
    if literal.is_empty() {
        0
    } else {
        raw.matches(literal).count()
    }
}

fn reject_unknown(value: &Py, label: &str, allowed: &[&str], errors: &mut Vec<String>) {
    let Py::Dict(pairs) = value else {
        errors.push(format!("{label} must be an object"));
        return;
    };
    let mut keys: Vec<String> = pairs
        .iter()
        .map(|(key, _)| pyjson::to_text(key))
        .filter(|key| !allowed.contains(&key.as_str()))
        .collect();
    keys.sort();
    for key in keys {
        errors.push(format!("{label} has unknown field '{key}'"));
    }
}

fn required_scalar(
    value: &Py,
    field: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    let item = pyjson::get(value, field);
    if item.is_none_or(perl::is_reference) || scalar(item).is_empty() {
        errors.push(format!("{label} lacks non-empty scalar '{field}'"));
        return None;
    }
    Some(scalar(item))
}

fn required_positive_integer(
    value: &Py,
    field: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<usize> {
    let item = pyjson::get(value, field);
    let digits = scalar(item);
    if item.is_none_or(perl::is_reference)
        || digits.is_empty()
        || !digits.chars().all(|character| character.is_ascii_digit())
        || perl::numeric(item) < 1.0
    {
        errors.push(format!("{label} lacks positive integer '{field}'"));
        return None;
    }
    Some(perl::numeric(item) as usize)
}

fn required_literals(value: Option<&Py>, label: &str, errors: &mut Vec<String>) -> Vec<String> {
    let Some(Py::List(items)) = value else {
        errors.push(format!("{label} must be a non-empty array"));
        return Vec::new();
    };
    if items.is_empty() {
        errors.push(format!("{label} must be a non-empty array"));
    }
    if items.len() > 32 {
        errors.push(format!("{label} exceeds 32 items"));
    }
    let mut seen = BTreeSet::new();
    let mut result = Vec::new();
    for item in items {
        if perl::is_reference(item) || scalar(Some(item)).is_empty() {
            errors.push(format!("{label} contains an empty or non-scalar item"));
            continue;
        }
        let item = scalar(Some(item));
        if item.len() > 512 {
            errors.push(format!("{label} contains a scalar longer than 512 bytes"));
        }
        if !seen.insert(item.clone()) {
            errors.push(format!("{label} contains a duplicate literal '{item}'"));
        }
        result.push(item);
    }
    result
}

fn validate_metric_object(
    value: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
) -> BTreeMap<String, usize> {
    let value = value.unwrap_or(&Py::None);
    reject_unknown(value, label, &["lines", "bytes", "line_bytes"], errors);
    let mut result = BTreeMap::new();
    for field in ["lines", "bytes", "line_bytes"] {
        if let Some(number) = required_positive_integer(value, field, label, errors) {
            result.insert(field.to_string(), number);
        }
    }
    result
}

fn compare_metrics(actual: Metrics, expected: Option<&Py>, label: &str, errors: &mut Vec<String>) {
    let Some(Py::Dict(_)) = expected else {
        return;
    };
    for (field, value) in [
        ("lines", actual.lines),
        ("bytes", actual.bytes),
        ("line_bytes", actual.line_bytes),
    ] {
        if let Some(expected_value) = pyjson::get(expected.unwrap(), field) {
            let expected_value = perl::numeric(Some(expected_value)) as usize;
            if value != expected_value {
                errors.push(format!(
                    "{label} {field} is {value}, expected {expected_value}"
                ));
            }
        }
    }
}

fn enforce_limits(actual: Metrics, limits: Option<&Py>, label: &str, errors: &mut Vec<String>) {
    let Some(limits) = limits else {
        return;
    };
    for (field, value) in [
        ("lines", actual.lines),
        ("bytes", actual.bytes),
        ("line_bytes", actual.line_bytes),
    ] {
        if let Some(limit) = pyjson::get(limits, field) {
            let limit = perl::numeric(Some(limit)) as usize;
            if value > limit {
                errors.push(format!("{label} exceeds {field} ceiling {limit}"));
            }
        }
    }
}

fn section_body(raw: &str, heading: &str, label: &str, errors: &mut Vec<String>) -> String {
    let marker = format!("## {heading}");
    let count = occurrences(raw, &marker);
    if count != 1 {
        errors.push(format!(
            "{label} must contain exactly one '{marker}' heading (found {count})"
        ));
        return String::new();
    }
    let Some((_, remainder)) = raw.split_once(&format!("{marker}\n")) else {
        return String::new();
    };
    remainder
        .split_once("\n## ")
        .map(|(body, _)| format!("{body}\n"))
        .unwrap_or_else(|| remainder.to_string())
}

fn normalize_link(index_path: &str, target: &str) -> Option<String> {
    if target.is_empty()
        || target
            .chars()
            .any(|character| character <= '\u{1f}' || character == '\u{7f}' || character == '\\')
    {
        return None;
    }
    let mut chars = target.chars();
    if chars
        .next()
        .is_some_and(|character| character.is_ascii_alphabetic())
        && target.contains(':')
        && target.split_once(':').is_some_and(|(scheme, _)| {
            scheme.chars().all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '+' | '.' | '-')
            })
        })
        || target.starts_with(['/', '#'])
    {
        return None;
    }
    let target = target.split(['?', '#']).next().unwrap_or(target);
    let mut parts: Vec<&str> = Path::new(index_path)
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .components()
        .filter_map(|component| component.as_os_str().to_str())
        .collect();
    parts.extend(target.split('/'));
    let mut normalized = Vec::new();
    for part in parts {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            if normalized.is_empty() {
                return None;
            }
            normalized.pop();
        } else {
            normalized.push(part);
        }
    }
    Some(normalized.join("/"))
}

fn resolved_link_counts(
    raw: &str,
    source_path: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    let mut remainder = raw;
    while let Some(open) = remainder.find('[') {
        let after_open = &remainder[open + 1..];
        let Some(close) = after_open.find("](") else {
            break;
        };
        let after_close = &after_open[close + 2..];
        let Some(end) = after_close.find(')') else {
            break;
        };
        let target = &after_close[..end];
        match normalize_link(source_path, target) {
            Some(resolved) if safe_relative_path(&resolved) => {
                *counts.entry(resolved).or_insert(0) += 1;
            }
            _ => errors.push(format!(
                "{label} contains an unsafe local link target '{target}'"
            )),
        }
        remainder = &after_close[end + 1..];
    }
    counts
}

fn read_json_object(
    base: &Path,
    relative: &str,
    label: &str,
    max_bytes: usize,
    errors: &mut Vec<String>,
) -> Option<(Py, Vec<u8>)> {
    let raw = read_regular(base, relative, label, errors)?;
    if raw.len() > max_bytes {
        errors.push(format!("{label} exceeds {max_bytes} bytes"));
    }
    let text = String::from_utf8_lossy(&raw);
    let value = pyjson::loads_strict(&text);
    if !matches!(value, Ok(Py::Dict(_))) {
        errors.push(format!("{label} is not one valid JSON object"));
        return None;
    }
    Some((value.unwrap(), raw))
}

fn validate_contract_schema(contract: &Py, errors: &mut Vec<String>) {
    reject_unknown(
        contract,
        "contract",
        &[
            "schema_version",
            "migration_state",
            "current_path",
            "archive",
            "source",
            "source_requirements",
            "current_root",
            "archive_index",
            "manifest_limits",
            "verifier",
        ],
        errors,
    );
    let schema = pyjson::get(contract, "schema_version");
    let digits = scalar(schema);
    if schema.is_none_or(perl::is_reference)
        || digits.is_empty()
        || !digits.chars().all(|character| character.is_ascii_digit())
        || perl::numeric(schema) != 1.0
    {
        errors.push("contract schema_version must be 1".to_string());
    }
    let state = required_scalar(contract, "migration_state", "contract", errors);
    if state
        .as_ref()
        .is_some_and(|state| !matches!(state.as_str(), "source_locked" | "migrated"))
    {
        errors.push(format!(
            "contract has invalid migration_state '{}'",
            state.unwrap()
        ));
    }
    let current = required_scalar(contract, "current_path", "contract", errors);
    if current
        .as_ref()
        .is_some_and(|path| !safe_relative_path(path))
    {
        errors.push("contract current_path is unsafe".to_string());
    }
    let verifier = required_scalar(contract, "verifier", "contract", errors);
    if verifier
        .as_ref()
        .is_some_and(|value| value != "perl scripts/check_task_tree_archive.pl --check")
    {
        errors.push(
            "contract verifier must be 'perl scripts/check_task_tree_archive.pl --check'"
                .to_string(),
        );
    }

    let archive = pyjson::get(contract, "archive").unwrap_or(&Py::None);
    reject_unknown(
        archive,
        "contract archive",
        &["index", "manifest", "source_capsule"],
        errors,
    );
    let mut paths = BTreeSet::new();
    for field in ["index", "manifest", "source_capsule"] {
        let path = required_scalar(archive, field, "contract archive", errors);
        if let Some(path) = path {
            if !safe_relative_path(&path) {
                errors.push(format!("contract archive {field} path is unsafe"));
            }
            if !paths.insert(path.clone()) {
                errors.push(format!("contract path '{path}' is declared more than once"));
            }
        }
    }
    if let Some(current) = current
        && !paths.insert(current.clone())
    {
        errors.push(format!(
            "contract path '{current}' is declared more than once"
        ));
    }

    let source = pyjson::get(contract, "source").unwrap_or(&Py::None);
    reject_unknown(source, "contract source", &["sha256", "metrics"], errors);
    let sha = required_scalar(source, "sha256", "contract source", errors);
    if sha.as_ref().is_some_and(|sha| {
        sha.len() != 64
            || !sha
                .chars()
                .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
    }) {
        errors
            .push("contract source sha256 must be 64 lowercase hexadecimal characters".to_string());
    }
    validate_metric_object(
        pyjson::get(source, "metrics"),
        "contract source metrics",
        errors,
    );
    required_literals(
        pyjson::get(contract, "source_requirements"),
        "contract source_requirements",
        errors,
    );

    for field in ["current_root", "archive_index"] {
        let spec = pyjson::get(contract, field).unwrap_or(&Py::None);
        reject_unknown(
            spec,
            &format!("contract {field}"),
            &[
                "health_targets",
                "enforcement_ceilings",
                "milestones",
                "required_literals",
                "forbidden_literals",
            ],
            errors,
        );
        let health = validate_metric_object(
            pyjson::get(spec, "health_targets"),
            &format!("contract {field} health_targets"),
            errors,
        );
        let ceilings = validate_metric_object(
            pyjson::get(spec, "enforcement_ceilings"),
            &format!("contract {field} enforcement_ceilings"),
            errors,
        );
        for dimension in ["lines", "bytes", "line_bytes"] {
            if let (Some(health), Some(ceiling)) = (health.get(dimension), ceilings.get(dimension))
                && health > ceiling
            {
                errors.push(format!(
                    "contract {field} health {dimension} exceeds its ceiling"
                ));
            }
        }
        let milestones = pyjson::get(spec, "milestones").unwrap_or(&Py::None);
        reject_unknown(
            milestones,
            &format!("contract {field} milestones"),
            &["warning_pct", "rollover_pct"],
            errors,
        );
        let warning = required_positive_integer(
            milestones,
            "warning_pct",
            &format!("contract {field} milestones"),
            errors,
        );
        let rollover = required_positive_integer(
            milestones,
            "rollover_pct",
            &format!("contract {field} milestones"),
            errors,
        );
        if let (Some(warning), Some(rollover)) = (warning, rollover)
            && (warning >= rollover || rollover >= 100)
        {
            errors.push(format!(
                "contract {field} milestones must satisfy warning_pct < rollover_pct < 100"
            ));
        }
        required_literals(
            pyjson::get(spec, "required_literals"),
            &format!("contract {field} required_literals"),
            errors,
        );
        required_literals(
            pyjson::get(spec, "forbidden_literals"),
            &format!("contract {field} forbidden_literals"),
            errors,
        );
    }
    let limits = pyjson::get(contract, "manifest_limits").unwrap_or(&Py::None);
    reject_unknown(
        limits,
        "contract manifest_limits",
        &["bytes", "line_bytes", "scalar_bytes"],
        errors,
    );
    for field in ["bytes", "line_bytes", "scalar_bytes"] {
        required_positive_integer(limits, field, "contract manifest_limits", errors);
    }
}

fn validate_required_presence(
    raw: &str,
    literals: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
) {
    let Some(Py::List(items)) = literals else {
        return;
    };
    for literal in items {
        let literal = scalar(Some(literal));
        if !literal.is_empty() && occurrences(raw, &literal) == 0 {
            errors.push(format!("{label} lacks required literal '{literal}'"));
        }
    }
}

fn validate_forbidden_absence(
    raw: &str,
    literals: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
) {
    let Some(Py::List(items)) = literals else {
        return;
    };
    for literal in items {
        let literal = scalar(Some(literal));
        if !literal.is_empty() && occurrences(raw, &literal) > 0 {
            errors.push(format!("{label} contains forbidden literal '{literal}'"));
        }
    }
}

fn pressure_warnings(actual: Metrics, spec: Option<&Py>, label: &str) -> Vec<String> {
    let mut warnings = Vec::new();
    let Some(spec) = spec else {
        return warnings;
    };
    let targets = pyjson::get(spec, "health_targets");
    let milestones = pyjson::get(spec, "milestones");
    let (Some(targets), Some(milestones)) = (targets, milestones) else {
        return warnings;
    };
    let warning = perl::numeric(pyjson::get(milestones, "warning_pct"));
    let rollover = perl::numeric(pyjson::get(milestones, "rollover_pct"));
    for (dimension, value) in [
        ("lines", actual.lines),
        ("bytes", actual.bytes),
        ("line_bytes", actual.line_bytes),
    ] {
        let target = perl::numeric(pyjson::get(targets, dimension));
        if target == 0.0 {
            continue;
        }
        let percent = 100.0 * value as f64 / target;
        if percent >= rollover {
            warnings.push(format!(
                "{label} {dimension} is at or above rollover ({percent:.1}%)"
            ));
        } else if percent >= warning {
            warnings.push(format!(
                "{label} {dimension} is at or above warning ({percent:.1}%)"
            ));
        }
    }
    warnings
}

fn validate_source(
    base: &Path,
    path: &str,
    contract: &Py,
    errors: &mut Vec<String>,
) -> Option<(Vec<u8>, Metrics, String)> {
    let raw = read_regular(base, path, "source authority", errors)?;
    let actual = metrics(&raw);
    let source = pyjson::get(contract, "source").unwrap_or(&Py::None);
    compare_metrics(
        actual,
        pyjson::get(source, "metrics"),
        "source authority",
        errors,
    );
    let sha = sha256_hex(&raw);
    let expected = scalar(pyjson::get(source, "sha256"));
    if !expected.is_empty() && sha != expected {
        errors.push(format!(
            "source authority sha256 is {sha}, expected {expected}"
        ));
    }
    let text = String::from_utf8_lossy(&raw);
    validate_required_presence(
        &text,
        pyjson::get(contract, "source_requirements"),
        "source authority",
        errors,
    );
    Some((raw, actual, sha))
}

fn validate_migrated_manifest(
    base: &Path,
    contract: &Py,
    errors: &mut Vec<String>,
) -> Option<(Py, Metrics)> {
    let limits = pyjson::get(contract, "manifest_limits").unwrap_or(&Py::None);
    let archive = pyjson::get(contract, "archive").unwrap_or(&Py::None);
    let path = scalar(pyjson::get(archive, "manifest"));
    let max_bytes = perl::numeric(pyjson::get(limits, "bytes")) as usize;
    let (manifest, raw) = read_json_object(base, &path, "archive manifest", max_bytes, errors)?;
    let actual = metrics(&raw);
    let line_limit = perl::numeric(pyjson::get(limits, "line_bytes")) as usize;
    if actual.line_bytes > line_limit {
        errors.push(format!(
            "archive manifest exceeds line_bytes ceiling {line_limit}"
        ));
    }
    reject_unknown(
        &manifest,
        "archive manifest",
        &[
            "schema_version",
            "current_path",
            "source_capsule",
            "source_sha256",
            "source_metrics",
            "source_boundary_commit",
            "sealed_date",
            "reason",
            "verifier",
        ],
        errors,
    );
    if perl::numeric(pyjson::get(&manifest, "schema_version")) != 1.0 {
        errors.push("archive manifest schema_version must be 1".to_string());
    }
    let scalar_limit = perl::numeric(pyjson::get(limits, "scalar_bytes")) as usize;
    for field in [
        "current_path",
        "source_capsule",
        "source_sha256",
        "source_boundary_commit",
        "sealed_date",
        "reason",
        "verifier",
    ] {
        if let Some(value) = required_scalar(&manifest, field, "archive manifest", errors)
            && value.len() > scalar_limit
        {
            errors.push(format!(
                "archive manifest scalar '{field}' exceeds {scalar_limit} bytes"
            ));
        }
    }
    if scalar(pyjson::get(&manifest, "current_path"))
        != scalar(pyjson::get(contract, "current_path"))
    {
        errors.push("archive manifest current_path disagrees with contract".to_string());
    }
    if scalar(pyjson::get(&manifest, "source_capsule"))
        != scalar(pyjson::get(archive, "source_capsule"))
    {
        errors.push("archive manifest source_capsule disagrees with contract".to_string());
    }
    let source = pyjson::get(contract, "source").unwrap_or(&Py::None);
    if scalar(pyjson::get(&manifest, "source_sha256")) != scalar(pyjson::get(source, "sha256")) {
        errors.push("archive manifest source_sha256 disagrees with contract".to_string());
    }
    if scalar(pyjson::get(&manifest, "verifier")) != scalar(pyjson::get(contract, "verifier")) {
        errors.push("archive manifest verifier disagrees with contract".to_string());
    }
    let boundary = scalar(pyjson::get(&manifest, "source_boundary_commit"));
    if boundary.len() != 40
        || !boundary
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
    {
        errors.push(
            "archive manifest source_boundary_commit must be a full lowercase Git object id"
                .to_string(),
        );
    }
    let declared_metrics = validate_metric_object(
        pyjson::get(&manifest, "source_metrics"),
        "archive manifest source_metrics",
        errors,
    );
    let expected_metrics = pyjson::get(source, "metrics");
    if matches!(expected_metrics, Some(Py::Dict(_))) {
        let actual_metrics = Metrics {
            lines: declared_metrics.get("lines").copied().unwrap_or(0),
            bytes: declared_metrics.get("bytes").copied().unwrap_or(0),
            line_bytes: declared_metrics.get("line_bytes").copied().unwrap_or(0),
        };
        compare_metrics(
            actual_metrics,
            expected_metrics,
            "archive manifest source_metrics",
            errors,
        );
    }
    Some((manifest, actual))
}

fn validate_tree(base: &Path, relative_contract: &str) -> (Vec<String>, ResultData) {
    let mut errors = Vec::new();
    let empty = || ResultData {
        state: String::new(),
        source_path: String::new(),
        source_sha: String::new(),
        source_metrics: Metrics::default(),
        warnings: Vec::new(),
        current_metrics: None,
        index_metrics: None,
        manifest_metrics: None,
    };
    if !safe_relative_path(relative_contract) {
        errors.push("contract path is absolute, escaping, or malformed".to_string());
        return (errors, empty());
    }
    let Some((contract, _)) =
        read_json_object(base, relative_contract, "contract", 65_536, &mut errors)
    else {
        return (errors, empty());
    };
    validate_contract_schema(&contract, &mut errors);
    let state = scalar(pyjson::get(&contract, "migration_state"));
    let current = scalar(pyjson::get(&contract, "current_path"));
    let archive = pyjson::get(&contract, "archive").unwrap_or(&Py::None);
    let source_path = if state == "migrated" {
        scalar(pyjson::get(archive, "source_capsule"))
    } else {
        current.clone()
    };
    let source = if safe_relative_path(&source_path) {
        validate_source(base, &source_path, &contract, &mut errors)
    } else {
        None
    };
    let mut result = ResultData {
        state: state.clone(),
        source_path,
        source_sha: source
            .as_ref()
            .map(|(_, _, sha)| sha.clone())
            .unwrap_or_default(),
        source_metrics: source
            .as_ref()
            .map(|(_, metrics, _)| *metrics)
            .unwrap_or_default(),
        warnings: Vec::new(),
        current_metrics: None,
        index_metrics: None,
        manifest_metrics: None,
    };
    if state == "source_locked" {
        for field in ["index", "manifest", "source_capsule"] {
            let path = scalar(pyjson::get(archive, field));
            if !safe_relative_path(&path) {
                continue;
            }
            let absolute = base.join(&path);
            if absolute.exists()
                || fs::symlink_metadata(&absolute)
                    .is_ok_and(|metadata| metadata.file_type().is_symlink())
            {
                errors.push(format!(
                    "source_locked state has premature archive {field} at '{path}'"
                ));
            }
        }
    } else if state == "migrated" {
        if let Some(raw) = read_regular(base, &current, "bounded current root", &mut errors) {
            let actual = metrics(&raw);
            result.current_metrics = Some(actual);
            result.warnings.extend(pressure_warnings(
                actual,
                pyjson::get(&contract, "current_root"),
                "bounded current root",
            ));
            let spec = pyjson::get(&contract, "current_root").unwrap_or(&Py::None);
            enforce_limits(
                actual,
                pyjson::get(spec, "enforcement_ceilings"),
                "bounded current root",
                &mut errors,
            );
            let text = String::from_utf8_lossy(&raw);
            validate_required_presence(
                &text,
                pyjson::get(spec, "required_literals"),
                "bounded current root",
                &mut errors,
            );
            validate_forbidden_absence(
                &text,
                pyjson::get(spec, "forbidden_literals"),
                "bounded current root",
                &mut errors,
            );
            let metadata = section_body(&text, "Metadata", "bounded current root", &mut errors);
            if !metadata
                .lines()
                .any(|line| line.trim_end() == "- Status: `done`")
            {
                errors.push("bounded current root metadata status is not done".to_string());
            }
            let frontier = section_body(
                &text,
                "Current Frontier",
                "bounded current root",
                &mut errors,
            );
            if !frontier.contains("No active frontier.") {
                errors.push("bounded current root does not declare an empty frontier".to_string());
            }
            let verification = section_body(
                &text,
                "Verification Log",
                "bounded current root",
                &mut errors,
            );
            if !verification.contains("| Date | Leaf | Checks | Result |") {
                errors.push(
                    "bounded current root lacks the verification capture-boundary header"
                        .to_string(),
                );
            }
            let links = resolved_link_counts(&text, &current, "bounded current root", &mut errors);
            let index = scalar(pyjson::get(archive, "index"));
            if links.get(&index).copied().unwrap_or(0) != 1 {
                errors.push(format!(
                    "bounded current root must link archive index exactly once (found {})",
                    links.get(&index).copied().unwrap_or(0)
                ));
            }
        }

        let index_path = scalar(pyjson::get(archive, "index"));
        if let Some(raw) = read_regular(base, &index_path, "archive index", &mut errors) {
            let actual = metrics(&raw);
            result.index_metrics = Some(actual);
            result.warnings.extend(pressure_warnings(
                actual,
                pyjson::get(&contract, "archive_index"),
                "archive index",
            ));
            let spec = pyjson::get(&contract, "archive_index").unwrap_or(&Py::None);
            enforce_limits(
                actual,
                pyjson::get(spec, "enforcement_ceilings"),
                "archive index",
                &mut errors,
            );
            let text = String::from_utf8_lossy(&raw);
            validate_required_presence(
                &text,
                pyjson::get(spec, "required_literals"),
                "archive index",
                &mut errors,
            );
            validate_forbidden_absence(
                &text,
                pyjson::get(spec, "forbidden_literals"),
                "archive index",
                &mut errors,
            );
            let links = resolved_link_counts(&text, &index_path, "archive index", &mut errors);
            let expected = [
                current,
                scalar(pyjson::get(archive, "source_capsule")),
                scalar(pyjson::get(archive, "manifest")),
            ];
            for path in &expected {
                if links.get(path).copied().unwrap_or(0) != 1 {
                    errors.push(format!(
                        "archive index must link '{path}' exactly once (found {})",
                        links.get(path).copied().unwrap_or(0)
                    ));
                }
            }
            for path in links.keys() {
                if !expected.contains(path) {
                    errors.push(format!(
                        "archive index contains unexpected local link '{path}'"
                    ));
                }
            }
        }
        if let Some((_, actual)) = validate_migrated_manifest(base, &contract, &mut errors) {
            result.manifest_metrics = Some(actual);
        }
    }
    (errors, result)
}

fn result_json(result: &ResultData) -> Py {
    let mut fields = vec![
        ("migration_state", text(&result.state)),
        ("source_path", text(&result.source_path)),
        ("source_sha256", text(&result.source_sha)),
        ("source_metrics", metrics_json(result.source_metrics)),
        (
            "warnings",
            Py::List(result.warnings.iter().map(|item| text(item)).collect()),
        ),
    ];
    if let Some(metrics) = result.current_metrics {
        fields.push(("current_metrics", metrics_json(metrics)));
    }
    if let Some(metrics) = result.index_metrics {
        fields.push(("index_metrics", metrics_json(metrics)));
    }
    if let Some(metrics) = result.manifest_metrics {
        fields.push(("manifest_metrics", metrics_json(metrics)));
    }
    object(fields)
}

fn write_raw(base: &Path, relative: &str, raw: &[u8]) -> Result<(), String> {
    let path = base.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(&path, raw)
        .map_err(|error| format!("{LABEL} self-test: cannot write {relative}: {error}"))
}

fn write_json(base: &Path, relative: &str, value: &Py) -> Result<(), String> {
    write_raw(base, relative, encode(value, Some(2))?.as_bytes())
}

fn source_fixture() -> &'static str {
    "# PROGRAM: fixture\n\n## Metadata\n\n- Status: `in_progress`\n\n## Verification Log\n\n| Date | Leaf | Checks | Result |\n| --- | --- | --- | --- |\n"
}

fn current_fixture() -> &'static str {
    "# PROGRAM: fixture\n\n## Metadata\n\n- Status: `done`\n\n## Goal and outcome\n\nClosed.\n\n## Current Frontier\n\nNo active frontier.\n\n## Archive and retrieval\n\n[Complete history](../archive/tasks/program/INDEX.md)\n\n## Verification Log\n\n| Date | Leaf | Checks | Result |\n| --- | --- | --- | --- |\n| 2026-08-09 | close | all | green |\n"
}

fn index_fixture() -> &'static str {
    "# PROGRAM archive\n\n- [Bounded current root](../../../tasks/PROGRAM.md)\n- [Exact source](source-through-2026-08-09.md)\n- [Manifest](manifest.json)\n\n## Verification\n\nRun the declared verifier.\n"
}

fn spec(
    health_targets: Metrics,
    enforcement_ceilings: Metrics,
    required: Vec<Py>,
    forbidden: Vec<Py>,
) -> Py {
    object(vec![
        ("health_targets", metrics_json(health_targets)),
        ("enforcement_ceilings", metrics_json(enforcement_ceilings)),
        (
            "milestones",
            object(vec![
                ("warning_pct", integer(80)),
                ("rollover_pct", integer(90)),
            ]),
        ),
        ("required_literals", Py::List(required)),
        ("forbidden_literals", Py::List(forbidden)),
    ])
}

fn fixture_contract(state: &str, source_raw: &str) -> Py {
    let source_metrics = metrics(source_raw.as_bytes());
    object(vec![
        ("schema_version", integer(1)),
        ("migration_state", text(state)),
        ("current_path", text("docs/tasks/PROGRAM.md")),
        (
            "archive",
            object(vec![
                ("index", text("docs/archive/tasks/program/INDEX.md")),
                ("manifest", text("docs/archive/tasks/program/manifest.json")),
                (
                    "source_capsule",
                    text("docs/archive/tasks/program/source-through-2026-08-09.md"),
                ),
            ]),
        ),
        (
            "source",
            object(vec![
                ("sha256", text(&sha256_hex(source_raw.as_bytes()))),
                ("metrics", metrics_json(source_metrics)),
            ]),
        ),
        (
            "source_requirements",
            Py::List(vec![
                text("# PROGRAM: fixture"),
                text("## Verification Log"),
                text("| Date | Leaf | Checks | Result |"),
            ]),
        ),
        (
            "current_root",
            spec(
                Metrics {
                    lines: 64,
                    bytes: 4_096,
                    line_bytes: 256,
                },
                Metrics {
                    lines: 96,
                    bytes: 6_144,
                    line_bytes: 512,
                },
                vec![
                    text("# PROGRAM: fixture"),
                    text("## Verification Log"),
                    text("| Date | Leaf | Checks | Result |"),
                    text("No active frontier."),
                ],
                vec![text("Status: `in_progress`"), text("Status: `pending`")],
            ),
        ),
        (
            "archive_index",
            spec(
                Metrics {
                    lines: 32,
                    bytes: 2_048,
                    line_bytes: 256,
                },
                Metrics {
                    lines: 48,
                    bytes: 4_096,
                    line_bytes: 512,
                },
                vec![text("# PROGRAM archive"), text("## Verification")],
                vec![text("Status: `in_progress`")],
            ),
        ),
        (
            "manifest_limits",
            object(vec![
                ("bytes", integer(8_192)),
                ("line_bytes", integer(512)),
                ("scalar_bytes", integer(512)),
            ]),
        ),
        (
            "verifier",
            text("perl scripts/check_task_tree_archive.pl --check"),
        ),
    ])
}

fn manifest_fixture(contract: &Py) -> Py {
    let archive = pyjson::get(contract, "archive").unwrap_or(&Py::None);
    let source = pyjson::get(contract, "source").unwrap_or(&Py::None);
    object(vec![
        ("schema_version", integer(1)),
        (
            "current_path",
            text(&scalar(pyjson::get(contract, "current_path"))),
        ),
        (
            "source_capsule",
            text(&scalar(pyjson::get(archive, "source_capsule"))),
        ),
        (
            "source_sha256",
            text(&scalar(pyjson::get(source, "sha256"))),
        ),
        (
            "source_metrics",
            pyjson::get(source, "metrics").unwrap_or(&Py::None).clone(),
        ),
        ("source_boundary_commit", text(&"a".repeat(40))),
        ("sealed_date", text("2026-08-09")),
        ("reason", text("fixture migration")),
        ("verifier", text(&scalar(pyjson::get(contract, "verifier")))),
    ])
}

fn field_mut<'a>(value: &'a mut Py, key: &str) -> Option<&'a mut Py> {
    if let Py::Dict(pairs) = value {
        pairs
            .iter_mut()
            .find(|(candidate, _)| *candidate == pyjson::py_str(key))
            .map(|(_, item)| item)
    } else {
        None
    }
}

fn nested_mut<'a>(mut value: &'a mut Py, path: &[&str]) -> &'a mut Py {
    for key in path {
        value = field_mut(value, key).expect("fixture field");
    }
    value
}

fn set_field(value: &mut Py, key: &str, item: Py) {
    if let Py::Dict(pairs) = value {
        let key = pyjson::py_str(key);
        if let Some((_, current)) = pairs.iter_mut().find(|(candidate, _)| *candidate == key) {
            *current = item;
        } else {
            pairs.push((key, item));
        }
    }
}

#[derive(Clone, Copy)]
enum Mutation {
    None,
    SourceHash,
    SourceMetric,
    SourceMarker,
    PrematureArchive,
    UnsafeSourcePath,
    CapsuleDrift,
    ActiveCurrent,
    MissingRootRoute,
    IncompleteIndex,
    ManifestIdentity,
    InvalidProvenance,
    CurrentOverflow,
    IndexOverflow,
}

struct Case {
    name: &'static str,
    migrated: bool,
    mutation: Mutation,
    expected: Option<&'static str>,
}

const CASES: &[Case] = &[
    Case {
        name: "source_locked positive",
        migrated: false,
        mutation: Mutation::None,
        expected: None,
    },
    Case {
        name: "source hash drift",
        migrated: false,
        mutation: Mutation::SourceHash,
        expected: Some("sha256"),
    },
    Case {
        name: "source metric drift",
        migrated: false,
        mutation: Mutation::SourceMetric,
        expected: Some("source authority bytes"),
    },
    Case {
        name: "source marker missing",
        migrated: false,
        mutation: Mutation::SourceMarker,
        expected: Some("lacks required literal"),
    },
    Case {
        name: "premature archive",
        migrated: false,
        mutation: Mutation::PrematureArchive,
        expected: Some("premature archive index"),
    },
    Case {
        name: "unsafe source path",
        migrated: false,
        mutation: Mutation::UnsafeSourcePath,
        expected: Some("current_path is unsafe"),
    },
    Case {
        name: "migrated positive",
        migrated: true,
        mutation: Mutation::None,
        expected: None,
    },
    Case {
        name: "capsule mutation",
        migrated: true,
        mutation: Mutation::CapsuleDrift,
        expected: Some("source authority"),
    },
    Case {
        name: "active current status",
        migrated: true,
        mutation: Mutation::ActiveCurrent,
        expected: Some("metadata status is not done"),
    },
    Case {
        name: "missing root route",
        migrated: true,
        mutation: Mutation::MissingRootRoute,
        expected: Some("must link archive index exactly once"),
    },
    Case {
        name: "incomplete index",
        migrated: true,
        mutation: Mutation::IncompleteIndex,
        expected: Some("must link 'docs/archive/tasks/program/manifest.json' exactly once"),
    },
    Case {
        name: "manifest identity drift",
        migrated: true,
        mutation: Mutation::ManifestIdentity,
        expected: Some("source_sha256 disagrees"),
    },
    Case {
        name: "invalid provenance",
        migrated: true,
        mutation: Mutation::InvalidProvenance,
        expected: Some("source_boundary_commit"),
    },
    Case {
        name: "current overflow",
        migrated: true,
        mutation: Mutation::CurrentOverflow,
        expected: Some("bounded current root exceeds bytes ceiling"),
    },
    Case {
        name: "index overflow",
        migrated: true,
        mutation: Mutation::IndexOverflow,
        expected: Some("archive index exceeds lines ceiling"),
    },
];

fn seed_fixture(base: &Path, migrated: bool, mutation: Mutation) -> Result<(), String> {
    let source = source_fixture();
    let mut contract = fixture_contract(
        if migrated {
            "migrated"
        } else {
            "source_locked"
        },
        source,
    );
    write_raw(
        base,
        "docs/tasks/PROGRAM.md",
        if migrated {
            current_fixture().as_bytes()
        } else {
            source.as_bytes()
        },
    )?;
    if migrated {
        write_raw(
            base,
            "docs/archive/tasks/program/source-through-2026-08-09.md",
            source.as_bytes(),
        )?;
        write_raw(
            base,
            "docs/archive/tasks/program/INDEX.md",
            index_fixture().as_bytes(),
        )?;
        write_json(
            base,
            "docs/archive/tasks/program/manifest.json",
            &manifest_fixture(&contract),
        )?;
    }
    match mutation {
        Mutation::None => {}
        Mutation::SourceHash => {
            *nested_mut(&mut contract, &["source", "sha256"]) = text(&"0".repeat(64));
        }
        Mutation::SourceMetric => {
            let value = nested_mut(&mut contract, &["source", "metrics", "bytes"]);
            *value = integer(perl::numeric(Some(value)) as usize + 1);
        }
        Mutation::SourceMarker => {
            let value = nested_mut(&mut contract, &["source_requirements"]);
            if let Py::List(items) = value {
                items[0] = text("missing marker");
            }
        }
        Mutation::PrematureArchive => {
            write_raw(base, "docs/archive/tasks/program/INDEX.md", b"premature\n")?
        }
        Mutation::UnsafeSourcePath => {
            set_field(&mut contract, "current_path", text("../escape.md"));
        }
        Mutation::CapsuleDrift => write_raw(
            base,
            "docs/archive/tasks/program/source-through-2026-08-09.md",
            format!("{}changed\n", source_fixture()).as_bytes(),
        )?,
        Mutation::ActiveCurrent => write_raw(
            base,
            "docs/tasks/PROGRAM.md",
            current_fixture()
                .replace("`done`", "`in_progress`")
                .as_bytes(),
        )?,
        Mutation::MissingRootRoute => write_raw(
            base,
            "docs/tasks/PROGRAM.md",
            current_fixture()
                .replace(
                    "[Complete history](../archive/tasks/program/INDEX.md)",
                    "history unavailable",
                )
                .as_bytes(),
        )?,
        Mutation::IncompleteIndex => write_raw(
            base,
            "docs/archive/tasks/program/INDEX.md",
            index_fixture()
                .replace("- [Manifest](manifest.json)\n", "")
                .as_bytes(),
        )?,
        Mutation::ManifestIdentity | Mutation::InvalidProvenance => {
            let mut manifest = manifest_fixture(&contract);
            if matches!(mutation, Mutation::ManifestIdentity) {
                set_field(&mut manifest, "source_sha256", text(&"0".repeat(64)));
            } else {
                set_field(&mut manifest, "source_boundary_commit", text("short"));
            }
            write_json(base, "docs/archive/tasks/program/manifest.json", &manifest)?;
        }
        Mutation::CurrentOverflow => {
            *nested_mut(
                &mut contract,
                &["current_root", "enforcement_ceilings", "bytes"],
            ) = integer(10);
        }
        Mutation::IndexOverflow => {
            *nested_mut(
                &mut contract,
                &["archive_index", "enforcement_ceilings", "lines"],
            ) = integer(1);
        }
    }
    write_json(
        base,
        "doctrine/live_document_size/task_tree_archive.json",
        &contract,
    )
}

struct Fixture(PathBuf);

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run_self_test(base: &Path) -> Result<usize, String> {
    fs::create_dir_all(base.join("generated")).map_err(|error| error.to_string())?;
    let mut passed = 0;
    for (index, case) in CASES.iter().enumerate() {
        let root = base.join(format!(
            "generated/.task-tree-archive-self-test.{}.{index}",
            std::process::id()
        ));
        if root.exists() {
            fs::remove_dir_all(&root).map_err(|error| error.to_string())?;
        }
        let _fixture = Fixture(root.clone());
        fs::create_dir_all(&root).map_err(|error| error.to_string())?;
        seed_fixture(&root, case.migrated, case.mutation)?;
        let (errors, _) =
            validate_tree(&root, "doctrine/live_document_size/task_tree_archive.json");
        let joined = errors.join("\n");
        match case.expected {
            None if !errors.is_empty() => {
                return Err(format!(
                    "{LABEL} self-test '{}' unexpectedly failed:\n{joined}",
                    case.name
                ));
            }
            Some(_) if errors.is_empty() => {
                return Err(format!(
                    "{LABEL} self-test '{}' unexpectedly passed",
                    case.name
                ));
            }
            Some(expected) if !joined.contains(expected) => {
                return Err(format!(
                    "{LABEL} self-test '{}' missed expected diagnostic {expected}:\n{joined}",
                    case.name
                ));
            }
            _ => {}
        }
        passed += 1;
    }
    if passed != 15 {
        return Err(format!(
            "{LABEL} self-test ran {passed} assertions, declaration expects 15 — re-derive the declaration beside the suite"
        ));
    }
    Ok(passed)
}

/// Run the task-tree archive contract with its original mode and path arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut requested_root = None;
    let mut contract = "doctrine/live_document_size/task_tree_archive.json".to_string();
    let mut mode = Mode::Check;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => match arguments.next() {
                Some(directory) => requested_root = Some(PathBuf::from(directory)),
                None => return usage(output),
            },
            "--contract" => match arguments.next() {
                Some(path) => contract = path.to_string(),
                None => return usage(output),
            },
            "--check" => {}
            "--report" => mode = Mode::Report,
            "--self-test" => mode = Mode::SelfTest,
            _ => return usage(output),
        }
    }
    if mode == Mode::SelfTest {
        return match run_self_test(root) {
            Ok(passed) => {
                output.out(&format!(
                    "{LABEL} self-test: {passed}/15 source/route/identity/boundary cases pass.\n"
                ));
                0
            }
            Err(error) => {
                output.err(&format!("{error}\n"));
                1
            }
        };
    }
    let root = match requested_root {
        None => root.to_path_buf(),
        Some(directory) => {
            let canonical = fs::canonicalize(&directory).ok().or_else(|| {
                let name = directory.file_name()?;
                let parent = directory
                    .parent()
                    .filter(|parent| !parent.as_os_str().is_empty())
                    .unwrap_or_else(|| Path::new("."));
                Some(fs::canonicalize(parent).ok()?.join(name))
            });
            match canonical {
                Some(root) => root,
                None => {
                    output.err(&format!("{LABEL}: root does not exist\n"));
                    return 1;
                }
            }
        }
    };
    let (errors, result) = validate_tree(&root, &contract);
    if !errors.is_empty() {
        for error in &errors {
            output.err(&format!("{LABEL}: {error}\n"));
        }
        output.err(&format!(
            "{LABEL}: FAILED with {} violation(s).\n",
            errors.len()
        ));
        return 1;
    }
    for warning in &result.warnings {
        output.err(&format!("{LABEL}: warning: {warning}\n"));
    }
    if mode == Mode::Report {
        match encode(&result_json(&result), None) {
            Ok(report) => output.out(&format!("{report}\n")),
            Err(error) => {
                output.err(&format!("{LABEL}: {error}\n"));
                return 1;
            }
        }
    } else {
        output.out(&format!(
            "{LABEL}: {} source identity and bounded route are valid.\n",
            result.state
        ));
    }
    0
}

fn usage(output: &mut dyn super::Output) -> i32 {
    output.err("Usage: cargo xtask check task-tree-archive [--root DIR] [--contract PATH] [--check|--report|--self-test]\n");
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_fifteen_cases_pass_without_residue() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--self-test".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert_eq!(
            output.text,
            "task-tree-archive self-test: 15/15 source/route/identity/boundary cases pass.\n"
        );
        let prefix = format!(".task-tree-archive-self-test.{}.", std::process::id());
        let residue = fs::read_dir(root.join("generated"))
            .expect("generated")
            .filter_map(Result::ok)
            .any(|entry| entry.file_name().to_string_lossy().starts_with(&prefix));
        assert!(!residue);
    }

    #[test]
    fn the_migrated_real_tree_and_report_hold() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--report".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("\"migration_state\":\"migrated\""));
        assert!(output.text.contains("\"source_sha256\":\""));
        assert!(output.text.contains("\"manifest_metrics\":"));
    }
}
