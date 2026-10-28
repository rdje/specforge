//! Active-task evidence contracts, native since `RUSTDOC-AND-XTASK.15.x`.

use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const DEFAULT_CONTRACT: &str = "doctrine/live_document_size/active_task_evidence.json";
const TASK_EVIDENCE_LINE_BYTES_CAP: u64 = 6_400;
const ROUTE_PART_FIXED_LINES: u64 = 10;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Metrics {
    lines: u64,
    bytes: u64,
    line_bytes: u64,
}

#[derive(Default)]
struct Validation {
    errors: Vec<String>,
    result: BTreeMap<String, Py>,
    warnings: Vec<String>,
}

type SourceInputs = (Vec<u8>, BTreeMap<String, Vec<u8>>);

fn integer(value: u64) -> Py {
    Py::Int(value.to_string())
}

fn text(value: &str) -> Py {
    Py::Str(pyjson::py_str(value))
}

fn object(pairs: Vec<(&str, Py)>) -> Py {
    Py::Dict(
        pairs
            .into_iter()
            .map(|(key, value)| (pyjson::py_str(key), value))
            .collect(),
    )
}

fn field<'a>(value: Option<&'a Py>, key: &str) -> Option<&'a Py> {
    value.and_then(|value| pyjson::get(value, key))
}

fn has_field(value: Option<&Py>, key: &str) -> bool {
    matches!(value, Some(Py::Dict(pairs)) if pairs.iter().any(|(candidate, _)| *candidate == pyjson::py_str(key)))
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

fn remove_field(value: &mut Py, key: &str) {
    if let Py::Dict(pairs) = value {
        pairs.retain(|(candidate, _)| *candidate != pyjson::py_str(key));
    }
}

fn nested_field_mut<'a>(value: &'a mut Py, path: &[&str]) -> Option<&'a mut Py> {
    let mut current = value;
    for name in path {
        let Py::Dict(pairs) = current else {
            return None;
        };
        let key = pyjson::py_str(name);
        let position = pairs.iter().position(|(candidate, _)| *candidate == key)?;
        current = &mut pairs[position].1;
    }
    Some(current)
}

fn scalar(value: Option<&Py>) -> String {
    perl::scalar(value).unwrap_or_default()
}

fn defined_scalar(value: Option<&Py>) -> Option<String> {
    let value = value?;
    if matches!(value, Py::None) || perl::is_reference(value) {
        None
    } else {
        perl::scalar(Some(value))
    }
}

fn array(value: Option<&Py>) -> &[Py] {
    match value {
        Some(Py::List(items)) => items,
        _ => &[],
    }
}

fn sha256_hex(raw: &[u8]) -> String {
    Sha256::digest(raw)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn canonical_json_with_ascii(value: &Py, ascii: bool) -> String {
    pyjson::dumps(
        value,
        &pyjson::Encoder {
            indent: None,
            sort_keys: true,
            ensure_ascii: ascii,
            item_separator: ",",
            key_separator: ":",
        },
    )
    .unwrap_or_default()
}

fn canonical_json(value: &Py) -> String {
    canonical_json_with_ascii(value, true)
}

fn report_json(value: &Py) -> String {
    canonical_json_with_ascii(value, false)
}

fn pretty_json(value: &Py) -> Vec<u8> {
    let mut raw = pyjson::dumps(
        value,
        &pyjson::Encoder {
            indent: Some(3),
            sort_keys: true,
            ensure_ascii: false,
            item_separator: ",",
            key_separator: " : ",
        },
    )
    .unwrap_or_default()
    .into_bytes();
    raw.push(b'\n');
    raw
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

fn absolute(base: &Path, relative: &str) -> PathBuf {
    base.join(relative)
}

fn dirname(path: &str) -> &str {
    match path.rfind('/') {
        None => ".",
        Some(0) => "/",
        Some(index) => &path[..index],
    }
}

fn path_exists(path: &Path) -> bool {
    fs::symlink_metadata(path).is_ok()
}

fn metrics(raw: &[u8]) -> Metrics {
    Metrics {
        lines: raw.iter().filter(|byte| **byte == b'\n').count() as u64
            + u64::from(!raw.is_empty() && !raw.ends_with(b"\n")),
        bytes: raw.len() as u64,
        line_bytes: raw
            .split(|byte| *byte == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line).len() as u64)
            .max()
            .unwrap_or(0),
    }
}

fn metric_values(metrics: Metrics) -> BTreeMap<String, u64> {
    BTreeMap::from([
        ("lines".to_string(), metrics.lines),
        ("bytes".to_string(), metrics.bytes),
        ("line_bytes".to_string(), metrics.line_bytes),
    ])
}

fn values_json(values: &BTreeMap<String, u64>) -> Py {
    Py::Dict(
        values
            .iter()
            .map(|(key, value)| (pyjson::py_str(key), integer(*value)))
            .collect(),
    )
}

fn metrics_json(metrics: Metrics) -> Py {
    values_json(&metric_values(metrics))
}

fn split_source_lines(raw: &[u8]) -> Vec<&[u8]> {
    let mut lines = Vec::new();
    let mut start = 0;
    for (index, byte) in raw.iter().enumerate() {
        if *byte == b'\n' {
            lines.push(&raw[start..=index]);
            start = index + 1;
        }
    }
    if start < raw.len() {
        lines.push(&raw[start..]);
    }
    lines
}

fn find_bytes(raw: &[u8], literal: &[u8], offset: usize) -> Option<usize> {
    if literal.is_empty() {
        return Some(offset.min(raw.len()));
    }
    raw.get(offset..)?
        .windows(literal.len())
        .position(|window| window == literal)
        .map(|position| offset + position)
}

fn occurrences(raw: &[u8], literal: &[u8]) -> usize {
    if literal.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut offset = 0;
    while let Some(position) = find_bytes(raw, literal, offset) {
        count += 1;
        offset = position + literal.len();
    }
    count
}

fn task_token_occurrences(raw: &[u8], literal: &[u8]) -> usize {
    if literal.is_empty() {
        return 0;
    }
    let boundary = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'.';
    let mut count = 0;
    let mut offset = 0;
    while let Some(position) = find_bytes(raw, literal, offset) {
        let before_ok = position == 0 || !boundary(raw[position - 1]);
        let after = position + literal.len();
        let after_ok = after == raw.len() || !boundary(raw[after]);
        if before_ok && after_ok {
            count += 1;
        }
        offset = after;
    }
    count
}

#[cfg(unix)]
fn device(path: &Path) -> Option<u64> {
    use std::os::unix::fs::MetadataExt;
    fs::metadata(path).ok().map(|metadata| metadata.dev())
}

#[cfg(not(unix))]
fn device(path: &Path) -> Option<u64> {
    fs::metadata(path).ok().map(|_| 0)
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
    let path = absolute(base, relative);
    let metadata = fs::symlink_metadata(&path).ok();
    if metadata
        .as_ref()
        .is_none_or(|metadata| !metadata.is_file() || metadata.file_type().is_symlink())
    {
        errors.push(format!(
            "{label} is missing or not a regular non-symlink file: {relative}"
        ));
        return None;
    }
    match fs::canonicalize(&path) {
        Ok(resolved) if resolved == base || resolved.starts_with(base) => {}
        _ => errors.push(format!(
            "{label} resolves outside the repository: {relative}"
        )),
    }
    let root_device = device(base);
    let file_device = device(&path);
    if root_device.is_none() || file_device.is_none() || root_device != file_device {
        errors.push(format!("{label} is off the repository volume: {relative}"));
    }
    match fs::read(&path) {
        Ok(raw) => Some(raw),
        Err(error) => {
            errors.push(format!(
                "cannot read {label} '{relative}': {}",
                os_error(&error)
            ));
            None
        }
    }
}

fn read_json_object(
    base: &Path,
    relative: &str,
    label: &str,
    max_bytes: u64,
    errors: &mut Vec<String>,
) -> Option<(Py, Vec<u8>)> {
    let raw = read_regular(base, relative, label, errors)?;
    if raw.len() as u64 > max_bytes {
        errors.push(format!("{label} exceeds {max_bytes} bytes"));
    }
    let decoded = std::str::from_utf8(&raw)
        .ok()
        .and_then(|text| pyjson::loads_strict(text).ok());
    match decoded {
        Some(value @ Py::Dict(_)) => Some((value, raw)),
        _ => {
            errors.push(format!("{label} is not one valid JSON object"));
            None
        }
    }
}

fn reject_unknown(value: Option<&Py>, label: &str, errors: &mut Vec<String>, allowed: &[&str]) {
    let Some(Py::Dict(pairs)) = value else {
        errors.push(format!("{label} must be an object"));
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
    value: Option<&Py>,
    key: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    match defined_scalar(field(value, key)) {
        Some(text) if !text.is_empty() => Some(text),
        _ => {
            errors.push(format!("{label} lacks non-empty scalar '{key}'"));
            None
        }
    }
}

fn positive_integer(
    value: Option<&Py>,
    key: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<u64> {
    let item = field(value, key);
    let text = defined_scalar(item).unwrap_or_default();
    if item.is_none_or(|value| matches!(value, Py::None) || perl::is_reference(value))
        || text.is_empty()
        || !text.bytes().all(|byte| byte.is_ascii_digit())
        || perl::numeric(item) < 1.0
    {
        errors.push(format!("{label} lacks positive integer '{key}'"));
        return None;
    }
    Some(perl::numeric(item) as u64)
}

fn scalar_array(value: Option<&Py>, label: &str, errors: &mut Vec<String>, allow_empty: bool) {
    let Some(Py::List(items)) = value else {
        errors.push(format!(
            "{label} must be {}",
            if allow_empty {
                "an array"
            } else {
                "a non-empty array"
            }
        ));
        return;
    };
    if items.is_empty() && !allow_empty {
        errors.push(format!("{label} must be a non-empty array"));
        return;
    }
    let mut seen = BTreeSet::new();
    for item in items {
        let Some(text) = defined_scalar(Some(item)).filter(|text| !text.is_empty()) else {
            errors.push(format!("{label} contains an empty or non-scalar item"));
            continue;
        };
        if !seen.insert(text.clone()) {
            errors.push(format!("{label} contains a duplicate '{text}'"));
        }
    }
}

fn metric_object_with_exemptions(
    value: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
    exempt: &BTreeSet<String>,
    fields: &[&str],
) -> BTreeMap<String, u64> {
    reject_unknown(value, label, errors, fields);
    let mut values = BTreeMap::new();
    for name in fields {
        if exempt.contains(*name) {
            if field(value, name).is_some_and(|value| !matches!(value, Py::None)) {
                errors.push(format!(
                    "{label} exempt dimension '{name}' must be null, not a value"
                ));
            }
            continue;
        }
        if let Some(value) = positive_integer(value, name, label, errors) {
            values.insert((*name).to_string(), value);
        }
    }
    values
}

fn metric_object(
    value: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
    fields: &[&str],
) -> BTreeMap<String, u64> {
    metric_object_with_exemptions(value, label, errors, &BTreeSet::new(), fields)
}

fn compare_metrics(
    actual: &BTreeMap<String, u64>,
    expected: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
    fields: &[&str],
) {
    for name in fields {
        let Some(expected) = field(expected, name).filter(|value| !matches!(value, Py::None))
        else {
            continue;
        };
        let actual_value = actual.get(*name).copied().unwrap_or(0);
        if actual_value as f64 != perl::numeric(Some(expected)) {
            errors.push(format!(
                "{label} {name} is {actual_value}, expected {}",
                scalar(Some(expected))
            ));
        }
    }
}

fn validate_limit_pair(
    spec: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
    fields: &[&str],
    extra: &[&str],
) {
    let mut allowed = vec![
        "health_targets",
        "enforcement_ceilings",
        "milestones",
        "cardinality_exemption",
    ];
    allowed.extend_from_slice(extra);
    reject_unknown(spec, label, errors, &allowed);
    let mut exempt = BTreeSet::new();
    if field(spec, "cardinality_exemption").is_some_and(|value| !matches!(value, Py::None)) {
        let exemption = field(spec, "cardinality_exemption");
        if !matches!(exemption, Some(Py::Dict(_))) {
            errors.push(format!("{label} cardinality_exemption must be an object"));
        } else {
            reject_unknown(
                exemption,
                &format!("{label} cardinality_exemption"),
                errors,
                &["authority", "work_unit", "rationale"],
            );
            for required in ["authority", "work_unit", "rationale"] {
                if defined_scalar(field(exemption, required)).is_none_or(|value| value.is_empty()) {
                    errors.push(format!("{label} cardinality_exemption lacks '{required}'"));
                }
            }
            exempt.insert("files".to_string());
        }
    }
    let health = metric_object_with_exemptions(
        field(spec, "health_targets"),
        &format!("{label} health_targets"),
        errors,
        &exempt,
        fields,
    );
    let ceilings = metric_object_with_exemptions(
        field(spec, "enforcement_ceilings"),
        &format!("{label} enforcement_ceilings"),
        errors,
        &exempt,
        fields,
    );
    for name in fields {
        if let (Some(health), Some(ceiling)) = (health.get(*name), ceilings.get(*name))
            && health > ceiling
        {
            errors.push(format!(
                "{label} health {name} exceeds its enforcement ceiling"
            ));
        }
    }
    let milestones = field(spec, "milestones");
    reject_unknown(
        milestones,
        &format!("{label} milestones"),
        errors,
        &["warning_pct", "rollover_pct"],
    );
    let warning = positive_integer(
        milestones,
        "warning_pct",
        &format!("{label} milestones"),
        errors,
    );
    let rollover = positive_integer(
        milestones,
        "rollover_pct",
        &format!("{label} milestones"),
        errors,
    );
    if let (Some(warning), Some(rollover)) = (warning, rollover)
        && (warning >= rollover || rollover >= 100)
    {
        errors.push(format!(
            "{label} milestones must satisfy warning_pct < rollover_pct < 100"
        ));
    }
}

fn enforce_ceilings(
    actual: &BTreeMap<String, u64>,
    ceilings: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
    mapping: &[(&str, &str)],
) {
    let mut mapping = mapping.to_vec();
    mapping.sort_by(|left, right| left.0.cmp(right.0));
    for (actual_field, ceiling_field) in mapping {
        let Some(ceiling) =
            field(ceilings, ceiling_field).filter(|value| !matches!(value, Py::None))
        else {
            continue;
        };
        if actual.get(actual_field).copied().unwrap_or(0) as f64 > perl::numeric(Some(ceiling)) {
            errors.push(format!(
                "{label} exceeds {ceiling_field} ceiling {}",
                scalar(Some(ceiling))
            ));
        }
    }
}

fn warning_messages(
    actual: &BTreeMap<String, u64>,
    spec: Option<&Py>,
    label: &str,
    mapping: &[(&str, &str)],
) -> Vec<String> {
    let mut warnings = Vec::new();
    let targets = field(spec, "health_targets");
    let milestones = field(spec, "milestones");
    if !matches!(targets, Some(Py::Dict(_))) || !matches!(milestones, Some(Py::Dict(_))) {
        return warnings;
    }
    let mut mapping = mapping.to_vec();
    mapping.sort_by(|left, right| left.0.cmp(right.0));
    for (actual_field, target_field) in mapping {
        let target = field(targets, target_field);
        if !perl::truthy(target) {
            continue;
        }
        let percent =
            100.0 * actual.get(actual_field).copied().unwrap_or(0) as f64 / perl::numeric(target);
        let rollover = field(milestones, "rollover_pct");
        let warning = field(milestones, "warning_pct");
        if percent >= rollover.map_or(101.0, |value| perl::numeric(Some(value))) {
            warnings.push(format!(
                "{label} {target_field} is at or above rollover ({percent:.1}%)"
            ));
        } else if percent >= warning.map_or(101.0, |value| perl::numeric(Some(value))) {
            warnings.push(format!(
                "{label} {target_field} is at or above warning ({percent:.1}%)"
            ));
        }
    }
    warnings
}

fn apply_live_pressure(
    actual: &BTreeMap<String, u64>,
    spec: Option<&Py>,
    label: &str,
    mapping: &[(&str, &str)],
    errors: &mut Vec<String>,
    warnings: &mut Vec<String>,
) {
    for message in warning_messages(actual, spec, label, mapping) {
        if message.contains(" rollover ") {
            errors.push(message.replace(" rollover ", " mandatory rollover "));
        } else {
            warnings.push(message);
        }
    }
}

fn enforce_portable_caps(
    values: Option<&Py>,
    caps: &[(&str, u64)],
    label: &str,
    errors: &mut Vec<String>,
) {
    let mut caps = caps.to_vec();
    caps.sort_by(|left, right| left.0.cmp(right.0));
    for (name, cap) in caps {
        let Some(value) = field(values, name).filter(|value| !perl::is_reference(value)) else {
            continue;
        };
        if !matches!(value, Py::None) && perl::numeric(Some(value)) > cap as f64 {
            errors.push(format!("{label} {name} exceeds portable cap {cap}"));
        }
    }
}

fn verifier_command_for(relative_contract: &str) -> String {
    if relative_contract == DEFAULT_CONTRACT {
        "perl scripts/check_active_task_evidence.pl --check".to_string()
    } else {
        format!("perl scripts/check_active_task_evidence.pl --contract {relative_contract} --check")
    }
}

fn writer_command_for(contract: &Py) -> String {
    let command = scalar(field(Some(contract), "verifier"));
    command
        .strip_suffix("--check")
        .map(|prefix| format!("{prefix}--write"))
        .unwrap_or(command)
}

fn root_owner_ids(contract: &Py) -> BTreeSet<String> {
    let mut ids = BTreeSet::new();
    for literal in array(field(
        field(Some(contract), "migrated_requirements"),
        "root_required_literals",
    )) {
        let literal = scalar(Some(literal));
        if let Some(id) = literal
            .strip_prefix("- ID: `")
            .and_then(|rest| rest.strip_suffix('`'))
            .filter(|id| !id.is_empty() && !id.contains('`'))
        {
            ids.insert(id.to_string());
        }
    }
    ids
}

fn validate_leaf_route_exemption(
    project_root: &Path,
    exemption: Option<&Py>,
    errors: &mut Vec<String>,
) {
    if !matches!(exemption, Some(Py::Dict(_))) {
        errors.push("sharded contract must declare manifest leaf_route_exemption".to_string());
        return;
    }
    reject_unknown(
        exemption,
        "contract leaf_route_exemption",
        errors,
        &["authority", "work_unit", "rationale"],
    );
    for name in ["authority", "work_unit", "rationale"] {
        required_scalar(exemption, name, "contract leaf_route_exemption", errors);
    }
    let authority = scalar(field(exemption, "authority"));
    if authority.is_empty() {
        return;
    }
    let valid_shape = authority
        .strip_prefix("docs/decisions/")
        .and_then(|name| name.strip_suffix(".md"))
        .is_some_and(|name| {
            let bytes = name.as_bytes();
            bytes.len() >= 6
                && bytes[..4].iter().all(u8::is_ascii_digit)
                && bytes[4] == b'-'
                && !bytes[5..].is_empty()
                && bytes[5..]
                    .iter()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
        });
    if !valid_shape || !absolute(project_root, &authority).is_file() {
        errors.push(format!(
            "contract leaf_route_exemption authority '{authority}' is not a tracked decision record"
        ));
    }
}

fn valid_object_id(value: &str, sha256: bool) -> bool {
    (value.len() == 64 || (!sha256 && value.len() == 40))
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_date(value: &str) -> bool {
    value.len() == 10
        && value.bytes().enumerate().all(|(index, byte)| {
            if matches!(index, 4 | 7) {
                byte == b'-'
            } else {
                byte.is_ascii_digit()
            }
        })
}

fn validate_contract_schema(
    project_root: &Path,
    contract: &Py,
    relative_contract: &str,
    errors: &mut Vec<String>,
) {
    reject_unknown(
        Some(contract),
        "contract",
        errors,
        &[
            "schema_version",
            "contract_id",
            "migration_state",
            "input_state",
            "current_path",
            "identity",
            "current_frontier",
            "migration_metadata",
            "source",
            "source_requirements",
            "destinations",
            "regions",
            "leaf_routes",
            "route_basis",
            "route_catalog_state",
            "marker_prefix",
            "limits",
            "migrated_requirements",
            "verifier",
        ],
    );
    let schema = field(Some(contract), "schema_version");
    if schema.is_none_or(|value| matches!(value, Py::None) || perl::is_reference(value))
        || perl::numeric(schema) != 1.0
    {
        errors.push("contract schema_version must be 1".to_string());
    }
    required_scalar(Some(contract), "contract_id", "contract", errors);
    let migration_state = required_scalar(Some(contract), "migration_state", "contract", errors);
    if migration_state
        .as_ref()
        .is_some_and(|state| !matches!(state.as_str(), "source_locked" | "migrated"))
    {
        errors.push(format!(
            "contract has invalid migration_state '{}'",
            migration_state.clone().unwrap_or_default()
        ));
    }
    let input_state = required_scalar(Some(contract), "input_state", "contract", errors);
    if input_state
        .as_ref()
        .is_some_and(|state| !matches!(state.as_str(), "topology_declared" | "complete"))
    {
        errors.push(format!(
            "contract has invalid input_state '{}'",
            input_state.clone().unwrap_or_default()
        ));
    }
    if migration_state.as_deref() == Some("migrated") && input_state.as_deref() != Some("complete")
    {
        errors.push("migrated contract requires complete migration inputs".to_string());
    }

    let current = required_scalar(Some(contract), "current_path", "contract", errors);
    if current
        .as_ref()
        .is_some_and(|path| !safe_relative_path(path))
    {
        errors.push("contract current_path is unsafe".to_string());
    }
    let verifier = required_scalar(Some(contract), "verifier", "contract", errors);
    let expected_verifier = verifier_command_for(relative_contract);
    if verifier
        .as_ref()
        .is_some_and(|value| *value != expected_verifier)
    {
        errors.push(format!("contract verifier must be '{expected_verifier}'"));
    }
    let route_basis = required_scalar(Some(contract), "route_basis", "contract", errors);
    if route_basis
        .as_deref()
        .is_some_and(|value| value != "boundary_path_commit_subject_ids")
    {
        errors.push("contract route_basis must be 'boundary_path_commit_subject_ids'".to_string());
    }
    let catalog_state = required_scalar(Some(contract), "route_catalog_state", "contract", errors);
    if catalog_state
        .as_ref()
        .is_some_and(|state| !matches!(state.as_str(), "inline" | "sharded"))
    {
        errors.push(format!(
            "contract has invalid route_catalog_state '{}'",
            catalog_state.clone().unwrap_or_default()
        ));
    }
    let sharded = catalog_state.as_deref() == Some("sharded");
    let marker_prefix = required_scalar(Some(contract), "marker_prefix", "contract", errors);
    if marker_prefix.as_ref().is_some_and(|prefix| {
        prefix.is_empty()
            || !prefix
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
            || prefix.starts_with('-')
            || prefix.ends_with('-')
            || prefix.contains("--")
    }) {
        errors.push(
            "contract marker_prefix must use lowercase letters, digits, and hyphens only"
                .to_string(),
        );
    }

    let identity = field(Some(contract), "identity");
    reject_unknown(
        identity,
        "contract identity",
        errors,
        &["tree_id", "heading", "active_status_literal"],
    );
    for name in ["tree_id", "heading", "active_status_literal"] {
        required_scalar(identity, name, "contract identity", errors);
    }
    let frontier = field(Some(contract), "current_frontier");
    reject_unknown(
        frontier,
        "contract current_frontier",
        errors,
        &["mode", "literal", "leaf_id", "part_id"],
    );
    let frontier_mode = required_scalar(frontier, "mode", "contract current_frontier", errors);
    if frontier_mode
        .as_ref()
        .is_some_and(|mode| !matches!(mode.as_str(), "none" | "eligible"))
    {
        errors.push(format!(
            "contract current_frontier has invalid mode '{}'",
            frontier_mode.clone().unwrap_or_default()
        ));
    }
    required_scalar(frontier, "literal", "contract current_frontier", errors);
    if frontier_mode.as_deref() == Some("eligible") {
        required_scalar(frontier, "leaf_id", "contract current_frontier", errors);
        required_scalar(frontier, "part_id", "contract current_frontier", errors);
    } else if has_field(frontier, "leaf_id") || has_field(frontier, "part_id") {
        errors.push("contract no-frontier object must not declare leaf_id or part_id".to_string());
    }

    let migration_metadata = field(Some(contract), "migration_metadata");
    if migration_metadata.is_some_and(|value| !matches!(value, Py::None))
        || migration_state.as_deref() == Some("migrated")
    {
        reject_unknown(
            migration_metadata,
            "contract migration_metadata",
            errors,
            &["migrated_on", "reason"],
        );
        let migrated_on = required_scalar(
            migration_metadata,
            "migrated_on",
            "contract migration_metadata",
            errors,
        );
        if migrated_on.as_ref().is_some_and(|date| !valid_date(date)) {
            errors.push("contract migration_metadata migrated_on is not an ISO date".to_string());
        }
        required_scalar(
            migration_metadata,
            "reason",
            "contract migration_metadata",
            errors,
        );
    }

    let source = field(Some(contract), "source");
    reject_unknown(
        source,
        "contract source",
        errors,
        &["boundary_commit", "git_blob", "sha256", "metrics"],
    );
    for name in ["boundary_commit", "git_blob", "sha256"] {
        let value = required_scalar(source, name, "contract source", errors);
        if value
            .as_ref()
            .is_some_and(|value| !valid_object_id(value, name == "sha256"))
        {
            errors.push(format!(
                "contract source {name} has invalid object identity"
            ));
        }
    }
    metric_object(
        field(source, "metrics"),
        "contract source metrics",
        errors,
        &["lines", "bytes", "line_bytes"],
    );
    scalar_array(
        field(Some(contract), "source_requirements"),
        "contract source_requirements",
        errors,
        false,
    );

    let destinations = field(Some(contract), "destinations");
    reject_unknown(
        destinations,
        "contract destinations",
        errors,
        &[
            "collection_directory",
            "index",
            "manifest",
            "parts",
            "archive_directory",
            "source_capsule",
            "route_part_prefix",
        ],
    );
    let mut paths = BTreeSet::new();
    if let Some(current) = &current {
        paths.insert(current.clone());
    }
    for name in [
        "collection_directory",
        "index",
        "manifest",
        "archive_directory",
        "source_capsule",
    ] {
        let path = required_scalar(destinations, name, "contract destinations", errors);
        if let Some(path) = path {
            if !safe_relative_path(&path) {
                errors.push(format!("contract destination {name} path is unsafe"));
            }
            if !paths.insert(path.clone()) {
                errors.push(format!("contract path '{path}' is declared more than once"));
            }
        }
    }
    let collection_directory = scalar(field(destinations, "collection_directory"));
    let archive_directory = scalar(field(destinations, "archive_directory"));
    for name in ["index", "manifest"] {
        let path = scalar(field(destinations, name));
        if !collection_directory.is_empty()
            && !path.starts_with(&format!("{collection_directory}/"))
        {
            errors.push(format!(
                "contract destination {name} is not below collection_directory"
            ));
        }
    }
    if !archive_directory.is_empty()
        && !scalar(field(destinations, "source_capsule"))
            .starts_with(&format!("{archive_directory}/"))
    {
        errors.push("contract source_capsule is not below archive_directory".to_string());
    }
    let mut route_prefix = None;
    if !sharded {
        if has_field(destinations, "route_part_prefix") {
            errors.push("inline route catalog must not declare a route_part_prefix".to_string());
        }
    } else {
        route_prefix = required_scalar(
            destinations,
            "route_part_prefix",
            "contract destinations",
            errors,
        );
    }
    if let Some(prefix) = route_prefix {
        if !safe_relative_path(&format!("{prefix}0001.md")) {
            errors.push("contract destination route_part_prefix is unsafe".to_string());
        }
        if !collection_directory.is_empty()
            && !prefix.starts_with(&format!("{collection_directory}/"))
        {
            errors.push(
                "contract destination route_part_prefix is not below collection_directory"
                    .to_string(),
            );
        }
        let stem = prefix.rsplit('/').next().unwrap_or(prefix.as_str());
        let valid_stem = stem.ends_with('-')
            && stem.len() > 1
            && stem[..stem.len() - 1]
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
            && !stem.starts_with('-')
            && !stem[..stem.len() - 1].ends_with('-')
            && !stem.contains("--");
        if prefix.rfind('/').is_none() || !valid_stem {
            errors.push(
                "contract destination route_part_prefix must end with a hyphenated stem"
                    .to_string(),
            );
        }
    }

    let parts_value = field(destinations, "parts");
    let parts = if matches!(parts_value, Some(Py::List(items)) if !items.is_empty()) {
        array(parts_value)
    } else {
        errors.push("contract destinations parts must be a non-empty array".to_string());
        &[]
    };
    let mut part_ids = BTreeSet::new();
    let mut part_paths = BTreeSet::new();
    let mut part_states = BTreeMap::new();
    for part in parts {
        reject_unknown(
            Some(part),
            "contract part",
            errors,
            &[
                "part_id",
                "path",
                "heading",
                "state",
                "sha256",
                "metrics",
                "sealed_commit",
                "git_blob",
                "archived_from",
            ],
        );
        let part_id = required_scalar(Some(part), "part_id", "contract part", errors);
        let path = required_scalar(Some(part), "path", "contract part", errors);
        required_scalar(Some(part), "heading", "contract part", errors);
        let state = required_scalar(Some(part), "state", "contract part", errors);
        if state.as_ref().is_some_and(|state| {
            !matches!(state.as_str(), "legacy" | "active" | "sealed" | "archived")
        }) {
            errors.push(format!(
                "contract part has invalid state '{}'",
                state.clone().unwrap_or_default()
            ));
        }
        let archived = state.as_deref() == Some("archived");
        if let Some(id) = &part_id {
            if !part_ids.insert(id.clone()) {
                errors.push(format!("contract duplicate part_id '{id}'"));
            }
            if let Some(state) = &state {
                part_states.insert(id.clone(), state.clone());
            }
        }
        if let Some(path) = &path {
            if !part_paths.insert(path.clone()) {
                errors.push(format!("contract duplicate part path '{path}'"));
            }
            let home = if archived {
                &archive_directory
            } else {
                &collection_directory
            };
            let home_field = if archived {
                "archive_directory"
            } else {
                "collection_directory"
            };
            if !safe_relative_path(path) || !path.starts_with(&format!("{home}/")) {
                errors.push(format!(
                    "contract part path '{path}' is unsafe or outside {home_field}"
                ));
            }
            if !paths.insert(path.clone()) {
                errors.push(format!("contract path '{path}' is declared more than once"));
            }
        }
        if archived {
            let from = required_scalar(
                Some(part),
                "archived_from",
                "archived contract part",
                errors,
            );
            if let Some(from) = from {
                if !safe_relative_path(&from)
                    || !from.starts_with(&format!("{collection_directory}/"))
                {
                    errors.push(format!(
                        "archived contract part archived_from '{from}' is unsafe or outside collection_directory"
                    ));
                }
                if !paths.insert(from.clone()) {
                    errors.push(format!("contract path '{from}' is declared more than once"));
                }
            }
        } else if has_field(Some(part), "archived_from") {
            errors.push("non-archived contract part must not declare archived_from".to_string());
        }
        if migration_state.as_deref() == Some("migrated") {
            let sha = required_scalar(Some(part), "sha256", "migrated contract part", errors);
            if sha.as_ref().is_some_and(|sha| !valid_object_id(sha, true)) {
                errors.push("migrated contract part sha256 is invalid".to_string());
            }
            metric_object(
                field(Some(part), "metrics"),
                "migrated contract part metrics",
                errors,
                &["lines", "bytes", "line_bytes"],
            );
        } else if has_field(Some(part), "sha256") || has_field(Some(part), "metrics") {
            errors.push("source-locked part must not pin result sha256 or metrics".to_string());
        }
        if matches!(state.as_deref(), Some("sealed" | "archived")) {
            let commit =
                required_scalar(Some(part), "sealed_commit", "sealed contract part", errors);
            let blob = required_scalar(Some(part), "git_blob", "sealed contract part", errors);
            if commit
                .as_ref()
                .is_some_and(|value| !valid_object_id(value, false))
            {
                errors.push("sealed contract part has invalid sealed_commit".to_string());
            }
            if blob
                .as_ref()
                .is_some_and(|value| !valid_object_id(value, false))
            {
                errors.push("sealed contract part has invalid git_blob".to_string());
            }
        } else if has_field(Some(part), "sealed_commit") || has_field(Some(part), "git_blob") {
            errors.push(
                "non-sealed, non-archived contract part must not declare sealed_commit or git_blob"
                    .to_string(),
            );
        }
    }

    let limits = field(Some(contract), "limits");
    reject_unknown(
        limits,
        "contract limits",
        errors,
        &[
            "root",
            "index",
            "parts",
            "route_parts",
            "capsule_exact",
            "manifest",
        ],
    );
    validate_limit_pair(
        field(limits, "root"),
        "contract root limits",
        errors,
        &["lines", "bytes", "line_bytes"],
        &[],
    );
    validate_limit_pair(
        field(limits, "index"),
        "contract index limits",
        errors,
        &["lines", "bytes", "line_bytes"],
        &[],
    );
    validate_limit_pair(
        field(limits, "parts"),
        "contract parts limits",
        errors,
        &[
            "files",
            "lines_each",
            "bytes_each",
            "line_bytes_each",
            "lines_total",
            "bytes_total",
        ],
        &[],
    );
    enforce_portable_caps(
        field(field(limits, "root"), "enforcement_ceilings"),
        &[("lines", 384), ("bytes", 36_864), ("line_bytes", 1_024)],
        "contract root enforcement ceiling",
        errors,
    );
    enforce_portable_caps(
        field(field(limits, "index"), "enforcement_ceilings"),
        &[("lines", 224), ("bytes", 24_576), ("line_bytes", 768)],
        "contract index enforcement ceiling",
        errors,
    );
    enforce_portable_caps(
        field(field(limits, "parts"), "enforcement_ceilings"),
        &[
            ("files", 24),
            ("lines_each", 896),
            ("bytes_each", 98_304),
            ("line_bytes_each", TASK_EVIDENCE_LINE_BYTES_CAP),
            ("lines_total", 9_600),
            ("bytes_total", 1_179_648),
        ],
        "contract parts enforcement ceiling",
        errors,
    );
    let route_parts = field(limits, "route_parts");
    if !sharded {
        if has_field(limits, "route_parts") {
            errors.push("inline route catalog must not declare route_parts limits".to_string());
        }
    } else {
        validate_limit_pair(
            route_parts,
            "contract route_parts limits",
            errors,
            &[
                "files",
                "lines_each",
                "bytes_each",
                "line_bytes_each",
                "lines_total",
                "bytes_total",
            ],
            &["routes_per_part", "max_parts", "max_unverified_routes"],
        );
        let routes_per_part = positive_integer(
            route_parts,
            "routes_per_part",
            "contract route_parts limits",
            errors,
        );
        let max_route_parts = positive_integer(
            route_parts,
            "max_parts",
            "contract route_parts limits",
            errors,
        );
        let max_unverified = field(route_parts, "max_unverified_routes");
        if defined_scalar(max_unverified).is_none_or(|value| {
            value.is_empty() || !value.bytes().all(|byte| byte.is_ascii_digit())
        }) {
            errors.push(
                "contract route_parts limits lacks a non-negative max_unverified_routes"
                    .to_string(),
            );
        }
        enforce_portable_caps(
            field(route_parts, "enforcement_ceilings"),
            &[
                ("files", 16),
                ("lines_each", 256),
                ("bytes_each", 32_768),
                ("line_bytes_each", 768),
                ("lines_total", 4_096),
                ("bytes_total", 524_288),
            ],
            "contract route_parts enforcement ceiling",
            errors,
        );
        enforce_portable_caps(
            Some(&object(vec![
                (
                    "routes_per_part",
                    routes_per_part.map(integer).unwrap_or(Py::None),
                ),
                (
                    "max_parts",
                    max_route_parts.map(integer).unwrap_or(Py::None),
                ),
            ])),
            &[("routes_per_part", 128), ("max_parts", 16)],
            "contract route_parts limit",
            errors,
        );
        if let Some(per_part) = routes_per_part {
            let target = field(field(route_parts, "health_targets"), "lines_each");
            let warning = field(field(route_parts, "milestones"), "warning_pct");
            if target.is_some_and(|value| perl::numeric(Some(value)) > 0.0)
                && warning.is_some()
                && 100.0 * (per_part + ROUTE_PART_FIXED_LINES) as f64 / perl::numeric(target)
                    >= perl::numeric(warning)
            {
                errors.push(
                    "contract route_parts health lines_each admits no full part below its warning band"
                        .to_string(),
                );
            }
        }
        if let (Some(max_parts), Some(per_part)) = (max_route_parts, routes_per_part)
            && matches!(field(Some(contract), "leaf_routes"), Some(Py::List(_)))
            && array(field(Some(contract), "leaf_routes")).len() as u64 > max_parts * per_part
        {
            errors.push(
                "contract route catalog capacity is below the declared leaf route count"
                    .to_string(),
            );
        }
    }
    let capsule_exact = metric_object(
        field(limits, "capsule_exact"),
        "contract capsule_exact",
        errors,
        &["lines", "bytes", "line_bytes"],
    );
    if matches!(field(source, "metrics"), Some(Py::Dict(_))) {
        let expected = field(source, "metrics");
        for name in ["lines", "bytes", "line_bytes"] {
            if let Some(actual) = capsule_exact.get(name) {
                let expected_value = field(expected, name);
                if expected_value.is_some_and(|value| !matches!(value, Py::None))
                    && *actual as f64 != perl::numeric(expected_value)
                {
                    errors.push(format!(
                        "contract capsule/source metric agreement {name} is {actual}, expected {}",
                        scalar(expected_value)
                    ));
                }
            }
        }
    }
    let manifest_limits = field(limits, "manifest");
    reject_unknown(
        manifest_limits,
        "contract manifest limits",
        errors,
        &[
            "bytes",
            "line_bytes",
            "scalar_bytes",
            "max_parts",
            "max_regions",
            "max_leaf_routes",
            "leaf_route_exemption",
        ],
    );
    for name in [
        "bytes",
        "line_bytes",
        "scalar_bytes",
        "max_parts",
        "max_regions",
    ] {
        positive_integer(manifest_limits, name, "contract manifest limits", errors);
    }
    if sharded {
        if has_field(manifest_limits, "max_leaf_routes") {
            errors.push(
                "sharded contract must not declare manifest max_leaf_routes: its route catalog parts bound the route count"
                    .to_string(),
            );
        }
        validate_leaf_route_exemption(
            project_root,
            field(manifest_limits, "leaf_route_exemption"),
            errors,
        );
    } else {
        positive_integer(
            manifest_limits,
            "max_leaf_routes",
            "contract manifest limits",
            errors,
        );
        if has_field(manifest_limits, "leaf_route_exemption") {
            errors.push(
                "inline contract must not declare leaf_route_exemption: it has no route catalog to bound its routes"
                    .to_string(),
            );
        }
    }
    enforce_portable_caps(
        manifest_limits,
        &[
            ("bytes", 65_536),
            ("line_bytes", 1_024),
            ("scalar_bytes", 1_024),
            ("max_parts", 24),
            ("max_regions", 32),
            ("max_leaf_routes", 128),
        ],
        "contract manifest limit",
        errors,
    );
    if parts.len() as f64 > perl::numeric(field(manifest_limits, "max_parts")) {
        errors.push("contract part count exceeds max_parts".to_string());
    }

    let regions_value = field(Some(contract), "regions");
    let regions = if matches!(regions_value, Some(Py::List(items)) if !items.is_empty()) {
        array(regions_value)
    } else {
        errors.push("contract regions must be a non-empty array".to_string());
        &[]
    };
    if regions.len() as f64 > perl::numeric(field(manifest_limits, "max_regions")) {
        errors.push("contract region count exceeds max_regions".to_string());
    }
    let mut region_ids = BTreeSet::new();
    let mut part_region_count = BTreeMap::<String, usize>::new();
    let mut expected_start = 1_u64;
    for region in regions {
        reject_unknown(
            Some(region),
            "contract region",
            errors,
            &[
                "region_id",
                "start_line",
                "end_line",
                "part_id",
                "sha256",
                "metrics",
            ],
        );
        let region_id = required_scalar(Some(region), "region_id", "contract region", errors);
        let start = positive_integer(Some(region), "start_line", "contract region", errors);
        let end = positive_integer(Some(region), "end_line", "contract region", errors);
        let part_id = required_scalar(Some(region), "part_id", "contract region", errors);
        if let Some(id) = &region_id
            && !region_ids.insert(id.clone())
        {
            errors.push(format!("contract duplicate region_id '{id}'"));
        }
        if let Some(part_id) = &part_id {
            if !part_ids.contains(part_id) {
                errors.push(format!(
                    "contract region '{}' references unknown part '{part_id}'",
                    region_id.clone().unwrap_or_default()
                ));
            }
            *part_region_count.entry(part_id.clone()).or_insert(0) += 1;
        }
        if let Some(start) = start {
            if start != expected_start {
                errors.push(format!(
                    "contract regions are not contiguous at line {expected_start}"
                ));
            }
            if let Some(end) = end {
                if end < start {
                    errors.push(format!(
                        "contract region '{}' ends before it starts",
                        region_id.clone().unwrap_or_default()
                    ));
                } else {
                    expected_start = end + 1;
                }
            }
        }
        if input_state.as_deref() == Some("complete") {
            let sha = required_scalar(Some(region), "sha256", "complete contract region", errors);
            if sha.as_ref().is_some_and(|sha| !valid_object_id(sha, true)) {
                errors.push("complete contract region sha256 is invalid".to_string());
            }
            let region_metrics = metric_object(
                field(Some(region), "metrics"),
                "complete contract region metrics",
                errors,
                &["lines", "bytes", "line_bytes"],
            );
            if let (Some(start), Some(end), Some(lines)) = (start, end, region_metrics.get("lines"))
                && i128::from(*lines) != i128::from(end) - i128::from(start) + 1
            {
                errors.push(format!(
                    "complete contract region '{}' line metric disagrees with span",
                    region_id.clone().unwrap_or_default()
                ));
            }
        } else if has_field(Some(region), "sha256") || has_field(Some(region), "metrics") {
            errors.push("topology-declared region must not pin sha256 or metrics".to_string());
        }
    }
    let source_lines = field(field(Some(contract), "source"), "metrics");
    if field(source_lines, "lines").is_some()
        && expected_start as f64 != perl::numeric(field(source_lines, "lines")) + 1.0
    {
        errors.push("contract regions do not cover the complete source line range".to_string());
    }
    for part in parts {
        if scalar(field(Some(part), "state")) == "legacy"
            && part_region_count
                .get(&scalar(field(Some(part), "part_id")))
                .copied()
                .unwrap_or(0)
                == 0
        {
            errors.push(format!(
                "legacy part '{}' owns no source region",
                scalar(field(Some(part), "part_id"))
            ));
        }
    }

    let routes_value = field(Some(contract), "leaf_routes");
    let routes = if matches!(routes_value, Some(Py::List(_))) {
        array(routes_value)
    } else {
        errors.push("contract leaf_routes must be an array".to_string());
        &[]
    };
    if field(manifest_limits, "max_leaf_routes")
        .is_some_and(|value| !matches!(value, Py::None) && !perl::is_reference(value))
        && routes.len() as f64 > perl::numeric(field(manifest_limits, "max_leaf_routes"))
    {
        errors.push("contract leaf route count exceeds max_leaf_routes".to_string());
    }
    if input_state.as_deref() == Some("topology_declared") && !routes.is_empty() {
        errors.push("topology-declared contract must not contain leaf routes".to_string());
    }
    if input_state.as_deref() == Some("complete") && routes.is_empty() {
        errors.push("complete contract must contain leaf routes".to_string());
    }
    let mut route_ids = BTreeSet::new();
    let mut route_parts = BTreeMap::new();
    for route in routes {
        reject_unknown(
            Some(route),
            "contract leaf route",
            errors,
            &[
                "leaf_id",
                "part_id",
                "origin",
                "source_literal",
                "lifecycle",
            ],
        );
        let leaf_id = required_scalar(Some(route), "leaf_id", "contract leaf route", errors);
        let part_id = required_scalar(Some(route), "part_id", "contract leaf route", errors);
        let origin = required_scalar(Some(route), "origin", "contract leaf route", errors);
        if let Some(id) = &leaf_id {
            if !route_ids.insert(id.clone()) {
                errors.push(format!("contract duplicate leaf route '{id}'"));
            }
            if let Some(part_id) = &part_id {
                route_parts.insert(id.clone(), part_id.clone());
            }
        }
        if let Some(part_id) = &part_id
            && !part_ids.contains(part_id)
        {
            errors.push(format!(
                "contract leaf route '{}' references unknown part '{part_id}'",
                leaf_id.clone().unwrap_or_default()
            ));
        }
        if origin.as_ref().is_some_and(|origin| {
            !matches!(origin.as_str(), "legacy" | "structural" | "post_migration")
        }) {
            errors.push(format!(
                "contract leaf route '{}' has invalid origin '{}'",
                leaf_id.clone().unwrap_or_default(),
                origin.clone().unwrap_or_default()
            ));
        }
        if !sharded {
            if has_field(Some(route), "lifecycle") {
                errors.push(format!(
                    "inline contract leaf route '{}' must not declare a lifecycle",
                    leaf_id.clone().unwrap_or_default()
                ));
            }
        } else {
            let lifecycle =
                required_scalar(Some(route), "lifecycle", "contract leaf route", errors);
            if lifecycle
                .as_ref()
                .is_some_and(|state| !matches!(state.as_str(), "open" | "closed"))
            {
                errors.push(format!(
                    "contract leaf route '{}' has invalid lifecycle '{}'",
                    leaf_id.clone().unwrap_or_default(),
                    lifecycle.clone().unwrap_or_default()
                ));
            }
        }
        if migration_state.as_deref() == Some("source_locked")
            && origin.as_deref() == Some("post_migration")
        {
            errors.push(format!(
                "source-locked contract cannot declare post-migration leaf route '{}'",
                leaf_id.clone().unwrap_or_default()
            ));
        }
        if matches!(origin.as_deref(), Some("legacy" | "structural")) {
            let literal = required_scalar(
                Some(route),
                "source_literal",
                &format!("{} contract leaf route", origin.clone().unwrap_or_default()),
                errors,
            );
            if let (Some(literal), Some(leaf_id)) = (literal, leaf_id.clone()) {
                let tree_id = scalar(field(identity, "tree_id"));
                let relative = leaf_id.strip_prefix(&tree_id).unwrap_or(&leaf_id);
                if literal != leaf_id && (relative.is_empty() || literal != relative) {
                    errors.push(format!(
                        "{} leaf route '{leaf_id}' source_literal is not its full or tree-relative id",
                        origin.clone().unwrap_or_default()
                    ));
                }
            }
        } else if has_field(Some(route), "source_literal") {
            errors.push(format!(
                "post-migration leaf route '{}' must not declare source_literal",
                leaf_id.clone().unwrap_or_default()
            ));
        }
    }
    if frontier_mode.as_deref() == Some("eligible") {
        let frontier_leaf = scalar(field(frontier, "leaf_id"));
        let frontier_part = scalar(field(frontier, "part_id"));
        if !route_ids.contains(&frontier_leaf) {
            errors.push("eligible frontier leaf_id lacks a primary leaf route".to_string());
        }
        if route_ids.contains(&frontier_leaf)
            && frontier_part != route_parts.get(&frontier_leaf).cloned().unwrap_or_default()
        {
            errors.push(
                "eligible frontier part_id disagrees with its primary leaf route".to_string(),
            );
        }
        if part_states.get(&frontier_part).map(String::as_str) != Some("active") {
            errors.push("eligible frontier must route to an active semantic part".to_string());
        }
    }

    let migrated = field(Some(contract), "migrated_requirements");
    reject_unknown(
        migrated,
        "contract migrated_requirements",
        errors,
        &[
            "root_required_literals",
            "root_forbidden_literals",
            "index_required_literals",
        ],
    );
    scalar_array(
        field(migrated, "root_required_literals"),
        "contract root_required_literals",
        errors,
        false,
    );
    scalar_array(
        field(migrated, "root_forbidden_literals"),
        "contract root_forbidden_literals",
        errors,
        true,
    );
    scalar_array(
        field(migrated, "index_required_literals"),
        "contract index_required_literals",
        errors,
        false,
    );
    let owners = root_owner_ids(contract);
    let tree_id = scalar(field(identity, "tree_id"));
    for owner in &owners {
        if owner != &tree_id && !owner.starts_with(&format!("{tree_id}.")) {
            errors.push(format!(
                "contract root owner '{owner}' is outside tree '{tree_id}'"
            ));
        }
    }
    if input_state.as_deref() == Some("complete") && !owners.is_empty() {
        for owner in owners.iter().filter(|owner| !route_ids.contains(*owner)) {
            errors.push(format!(
                "complete leaf routes omit declared root owner '{owner}'"
            ));
        }
        let required: BTreeSet<String> = if sharded {
            routes
                .iter()
                .filter(|route| scalar(field(Some(route), "lifecycle")) != "closed")
                .filter_map(|route| defined_scalar(field(Some(route), "leaf_id")))
                .filter(|id| route_ids.contains(id))
                .collect()
        } else {
            route_ids.clone()
        };
        for id in required.iter().filter(|id| !owners.contains(*id)) {
            errors.push(format!(
                "complete leaf route '{id}' is absent from the declared root owner registry"
            ));
        }
    }
}

fn command_capture(program: &str, arguments: &[String]) -> (Vec<u8>, i32, Vec<u8>) {
    match Command::new(program).args(arguments).output() {
        Ok(output) => (
            output.stdout,
            output.status.code().unwrap_or(255),
            output.stderr,
        ),
        Err(error) => (Vec::new(), 255, error.to_string().into_bytes()),
    }
}

fn git_capture(
    base: &Path,
    label: &str,
    errors: &mut Vec<String>,
    arguments: &[String],
) -> Option<Vec<u8>> {
    let mut command = vec!["-C".to_string(), base.to_string_lossy().into_owned()];
    command.extend_from_slice(arguments);
    let (raw, exit, _) = command_capture("git", &command);
    if exit != 0 {
        errors.push(format!("{label} failed (git exit {exit})"));
        None
    } else {
        Some(raw)
    }
}

fn trim_newlines(raw: &[u8]) -> String {
    String::from_utf8_lossy(raw)
        .trim_end_matches(['\r', '\n'])
        .to_string()
}

fn validate_git_boundary(base: &Path, contract: &Py, source_raw: &[u8], errors: &mut Vec<String>) {
    let source = field(Some(contract), "source");
    let Some(commit) = defined_scalar(field(source, "boundary_commit")) else {
        return;
    };
    let path = scalar(field(Some(contract), "current_path"));
    if !safe_relative_path(&path) {
        return;
    }
    if let Some(resolved) = git_capture(
        base,
        "source boundary commit lookup",
        errors,
        &[
            "rev-parse".to_string(),
            "--verify".to_string(),
            format!("{commit}^{{commit}}"),
        ],
    ) {
        let resolved = trim_newlines(&resolved);
        if resolved != commit {
            errors.push(format!(
                "source boundary commit resolves to '{resolved}', expected '{commit}'"
            ));
        }
    }
    if let Some(blob) = git_capture(
        base,
        "source boundary blob lookup",
        errors,
        &["rev-parse".to_string(), format!("{commit}:{path}")],
    ) {
        let blob = trim_newlines(&blob);
        let expected = scalar(field(source, "git_blob"));
        if blob != expected {
            errors.push(format!(
                "source boundary blob is '{blob}', expected '{expected}'"
            ));
        }
    }
    if let Some(committed) = git_capture(
        base,
        "source boundary content read",
        errors,
        &["show".to_string(), format!("{commit}:{path}")],
    ) && committed != source_raw
    {
        errors.push("source boundary Git content differs from the pinned source bytes".to_string());
    }
    let mut command = vec!["-C".to_string(), base.to_string_lossy().into_owned()];
    command.extend([
        "merge-base".to_string(),
        "--is-ancestor".to_string(),
        commit,
        "HEAD".to_string(),
    ]);
    let (_, exit, _) = command_capture("git", &command);
    if exit != 0 {
        errors.push("source boundary commit is not an ancestor of HEAD".to_string());
    }
}

fn boundary_route_ids(base: &Path, contract: &Py, errors: &mut Vec<String>) -> Vec<String> {
    let source = field(Some(contract), "source");
    let commit = defined_scalar(field(source, "boundary_commit"));
    let tree_id = defined_scalar(field(field(Some(contract), "identity"), "tree_id"));
    let path = scalar(field(Some(contract), "current_path"));
    let (Some(commit), Some(tree_id)) = (commit, tree_id) else {
        return Vec::new();
    };
    if !safe_relative_path(&path) {
        return Vec::new();
    }
    let Some(subjects) = git_capture(
        base,
        "source boundary path history read",
        errors,
        &[
            "log".to_string(),
            "--format=%s".to_string(),
            commit,
            "--".to_string(),
            path,
        ],
    ) else {
        return Vec::new();
    };
    let mut ids = BTreeSet::new();
    let needle = tree_id.as_bytes();
    let word = |byte: u8| byte.is_ascii_alphanumeric() || byte == b'_';
    let mut offset = 0;
    while let Some(start) = find_bytes(&subjects, needle, offset) {
        offset = start + needle.len();
        let before_ok = start == 0 || !word(subjects[start - 1]);
        let mut end = offset;
        while subjects.get(end) == Some(&b'.') {
            let suffix_start = end + 1;
            let mut suffix_end = suffix_start;
            while subjects
                .get(suffix_end)
                .is_some_and(|byte| byte.is_ascii_alphanumeric())
            {
                suffix_end += 1;
            }
            if suffix_end == suffix_start {
                break;
            }
            end = suffix_end;
        }
        let after_ok = end == subjects.len() || !word(subjects[end]);
        if before_ok && after_ok {
            ids.insert(String::from_utf8_lossy(&subjects[start..end]).into_owned());
        }
    }
    ids.into_iter().collect()
}

fn normalize_link(source_path: &str, target: &str) -> Option<String> {
    if target.is_empty()
        || target
            .chars()
            .any(|character| character <= '\u{1f}' || character == '\u{7f}' || character == '\\')
    {
        return None;
    }
    let scheme = target.split_once(':').is_some_and(|(prefix, _)| {
        let mut chars = prefix.chars();
        chars.next().is_some_and(|c| c.is_ascii_alphabetic())
            && chars.all(|c| c.is_ascii_alphanumeric() || matches!(c, '+' | '.' | '-'))
    });
    if scheme || target.starts_with('/') || target.starts_with('#') {
        return None;
    }
    let target = target.split(['?', '#']).next().unwrap_or(target);
    let mut normalized = Vec::new();
    for part in dirname(source_path).split('/').chain(target.split('/')) {
        if part.is_empty() || part == "." {
            continue;
        }
        if part == ".." {
            normalized.pop()?;
        } else {
            normalized.push(part);
        }
    }
    Some(normalized.join("/"))
}

fn markdown_links(raw: &[u8]) -> Vec<String> {
    let mut links = Vec::new();
    let mut offset = 0;
    while offset < raw.len() {
        let Some(open) = raw[offset..].iter().position(|byte| *byte == b'[') else {
            break;
        };
        let open = offset + open;
        let Some(close) = raw[open + 1..]
            .windows(2)
            .position(|window| window == b"](")
        else {
            break;
        };
        let close = open + 1 + close;
        if raw[open + 1..close]
            .iter()
            .any(|byte| matches!(byte, b']' | b'\r' | b'\n'))
        {
            offset = open + 1;
            continue;
        }
        let target_start = close + 2;
        let Some(end) = raw[target_start..].iter().position(|byte| *byte == b')') else {
            break;
        };
        let end = target_start + end;
        if !raw[target_start..end]
            .iter()
            .any(|byte| matches!(byte, b'\r' | b'\n'))
        {
            links.push(String::from_utf8_lossy(&raw[target_start..end]).into_owned());
        }
        offset = end + 1;
    }
    links
}

fn link_counts(
    raw: &[u8],
    source_path: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for target in markdown_links(raw) {
        let resolved = normalize_link(source_path, &target);
        match resolved.filter(|path| safe_relative_path(path)) {
            Some(path) => *counts.entry(path).or_insert(0) += 1,
            None => errors.push(format!("{label} contains unsafe local link '{target}'")),
        }
    }
    counts
}

fn validate_required_literals(
    raw: &[u8],
    literals: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
) {
    for literal in array(literals) {
        let Some(literal) = defined_scalar(Some(literal)).filter(|literal| !literal.is_empty())
        else {
            continue;
        };
        if occurrences(raw, literal.as_bytes()) == 0 {
            errors.push(format!("{label} lacks required literal '{literal}'"));
        }
    }
}

fn validate_forbidden_literals(
    raw: &[u8],
    literals: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
) {
    for literal in array(literals) {
        let Some(literal) = defined_scalar(Some(literal)).filter(|literal| !literal.is_empty())
        else {
            continue;
        };
        if occurrences(raw, literal.as_bytes()) != 0 {
            errors.push(format!("{label} contains forbidden literal '{literal}'"));
        }
    }
}

fn validate_scalar_bounds(value: &Py, limit: u64, label: &str, errors: &mut Vec<String>) {
    match value {
        Py::List(items) => {
            for item in items {
                validate_scalar_bounds(item, limit, label, errors);
            }
        }
        Py::Dict(pairs) => {
            for (key, item) in pairs {
                if pyjson::to_text(key).len() as u64 > limit {
                    errors.push(format!("{label} contains a key above {limit} bytes"));
                }
                validate_scalar_bounds(item, limit, label, errors);
            }
        }
        Py::Bool(_) | Py::None => {}
        _ => {
            if scalar(Some(value)).len() as u64 > limit {
                errors.push(format!("{label} contains a scalar above {limit} bytes"));
            }
        }
    }
}

fn validate_source_and_inputs(
    base: &Path,
    contract: &Py,
    source_path: &str,
    locked: bool,
    validation: &mut Validation,
) -> Option<SourceInputs> {
    let errors = &mut validation.errors;
    let source_raw = read_regular(base, source_path, "source authority", errors)?;
    let source_metrics = metrics(&source_raw);
    compare_metrics(
        &metric_values(source_metrics),
        field(field(Some(contract), "source"), "metrics"),
        "source authority",
        errors,
        &["lines", "bytes", "line_bytes"],
    );
    let sha = sha256_hex(&source_raw);
    let expected_sha = scalar(field(field(Some(contract), "source"), "sha256"));
    if sha != expected_sha {
        errors.push(format!(
            "source authority sha256 is {sha}, expected {expected_sha}"
        ));
    }
    validate_required_literals(
        &source_raw,
        field(Some(contract), "source_requirements"),
        "source authority",
        errors,
    );
    let identity = field(Some(contract), "identity");
    let heading = scalar(field(identity, "heading"));
    if !source_raw.starts_with(heading.as_bytes()) {
        errors.push("source authority heading does not match contract identity".to_string());
    }
    let tree_literal = format!("- Tree ID: `{}`", scalar(field(identity, "tree_id")));
    if occurrences(&source_raw, tree_literal.as_bytes()) != 1 {
        errors
            .push("source authority does not carry exactly one declared tree identity".to_string());
    }
    if occurrences(
        &source_raw,
        scalar(field(identity, "active_status_literal")).as_bytes(),
    ) == 0
    {
        errors.push("source authority lacks the declared active-status literal".to_string());
    }
    validate_git_boundary(base, contract, &source_raw, errors);
    if locked
        && let Some(staged) = git_capture(
            base,
            "source Git-index lookup",
            errors,
            &[
                "ls-files".to_string(),
                "--stage".to_string(),
                "--".to_string(),
                scalar(field(Some(contract), "current_path")),
            ],
        )
    {
        let entries: Vec<&[u8]> = staged
            .split(|byte| *byte == b'\n')
            .filter(|entry| !entry.is_empty())
            .collect();
        let expected = scalar(field(field(Some(contract), "source"), "git_blob"));
        let parsed = entries.first().and_then(|entry| {
            let tab = entry.iter().position(|byte| *byte == b'\t')?;
            let mut fields = entry[..tab].split(|byte| *byte == b' ');
            let mode = fields.next()?;
            let blob = fields.next()?;
            let stage = fields.next()?;
            if fields.next().is_some()
                || mode.is_empty()
                || !mode.iter().all(u8::is_ascii_digit)
                || stage != b"0"
                || !valid_object_id(&String::from_utf8_lossy(blob), false)
            {
                return None;
            }
            Some(String::from_utf8_lossy(blob).into_owned())
        });
        if entries.len() != 1 || parsed.is_none() {
            errors.push(
                "source current path does not have exactly one stage-zero Git-index entry"
                    .to_string(),
            );
        } else if parsed.as_deref() != Some(expected.as_str()) {
            errors.push(format!(
                "source Git-index blob is '{}', expected '{expected}'",
                parsed.unwrap_or_default()
            ));
        }
    }

    let lines = split_source_lines(&source_raw);
    let mut region_raw = BTreeMap::new();
    let mut part_payload = BTreeMap::<String, Vec<u8>>::new();
    for region in array(field(Some(contract), "regions")) {
        let start =
            field(Some(region), "start_line").map(|value| perl::numeric(Some(value)) as usize);
        let end = field(Some(region), "end_line").map(|value| perl::numeric(Some(value)) as usize);
        let (Some(start), Some(end)) = (start, end) else {
            continue;
        };
        if start < 1 || end > lines.len() || end < start {
            continue;
        }
        let raw = lines[start - 1..end].concat();
        let id = scalar(field(Some(region), "region_id"));
        let part_id = scalar(field(Some(region), "part_id"));
        region_raw.insert(id.clone(), raw.clone());
        part_payload
            .entry(part_id)
            .or_default()
            .extend_from_slice(&raw);
        if scalar(field(Some(contract), "input_state")) == "complete" {
            compare_metrics(
                &metric_values(metrics(&raw)),
                field(Some(region), "metrics"),
                &format!("source region '{id}'"),
                errors,
                &["lines", "bytes", "line_bytes"],
            );
            if sha256_hex(&raw) != scalar(field(Some(region), "sha256")) {
                errors.push(format!("source region '{id}' sha256 mismatch"));
            }
        }
    }
    let mut planned = BTreeMap::new();
    let part_limits = field(field(Some(contract), "limits"), "parts");
    for part in array(field(field(Some(contract), "destinations"), "parts")) {
        let id = scalar(field(Some(part), "part_id"));
        if id.is_empty() || !part_payload.contains_key(&id) {
            continue;
        }
        let actual = metric_values(metrics(&part_payload[&id]));
        planned.insert(id.clone(), actual.clone());
        let mapping = [
            ("lines", "lines_each"),
            ("bytes", "bytes_each"),
            ("line_bytes", "line_bytes_each"),
        ];
        for warning in warning_messages(
            &actual,
            part_limits,
            &format!("planned part '{id}' payload"),
            &mapping,
        ) {
            validation.errors.push(format!(
                "planned part '{id}' payload starts at or above its warning boundary: {warning}"
            ));
        }
        enforce_ceilings(
            &actual,
            field(part_limits, "enforcement_ceilings"),
            &format!("planned part '{id}' payload"),
            &mut validation.errors,
            &mapping,
        );
    }

    if scalar(field(Some(contract), "input_state")) == "complete" {
        let basis = boundary_route_ids(base, contract, &mut validation.errors);
        let expected: BTreeSet<String> = basis.into_iter().collect();
        let actual: BTreeSet<String> = array(field(Some(contract), "leaf_routes"))
            .iter()
            .filter(|route| scalar(field(Some(route), "origin")) == "legacy")
            .map(|route| scalar(field(Some(route), "leaf_id")))
            .collect();
        for id in expected.difference(&actual) {
            validation.errors.push(format!(
                "complete leaf routes omit boundary-history id '{id}'"
            ));
        }
        for id in actual.difference(&expected) {
            validation.errors.push(format!(
                "complete legacy leaf route '{id}' is absent from boundary path commit subjects"
            ));
        }
        let part_ids: BTreeSet<String> =
            array(field(field(Some(contract), "destinations"), "parts"))
                .iter()
                .map(|part| scalar(field(Some(part), "part_id")))
                .collect();
        for route in array(field(Some(contract), "leaf_routes")) {
            let id = scalar(field(Some(route), "leaf_id"));
            let part_id = scalar(field(Some(route), "part_id"));
            if !part_ids.contains(&part_id) {
                continue;
            }
            let origin = scalar(field(Some(route), "origin"));
            if matches!(origin.as_str(), "legacy" | "structural") {
                let literal = scalar(field(Some(route), "source_literal"));
                if task_token_occurrences(
                    part_payload.get(&part_id).map(Vec::as_slice).unwrap_or(&[]),
                    literal.as_bytes(),
                ) == 0
                {
                    validation.errors.push(format!(
                        "{origin} leaf route '{id}' source literal '{literal}' is absent from its primary part payload"
                    ));
                }
            }
        }
    }
    validation
        .result
        .insert("source_path".to_string(), text(source_path));
    validation
        .result
        .insert("source_sha256".to_string(), text(&sha));
    validation
        .result
        .insert("source_metrics".to_string(), metrics_json(source_metrics));
    validation.result.insert(
        "planned_part_payload_metrics".to_string(),
        Py::Dict(
            planned
                .iter()
                .map(|(id, values)| (pyjson::py_str(id), values_json(values)))
                .collect(),
        ),
    );
    Some((source_raw, region_raw))
}

fn validate_destinations_absent(base: &Path, contract: &Py, errors: &mut Vec<String>) {
    let destinations = field(Some(contract), "destinations");
    let mut declared = vec![
        (
            "collection_directory".to_string(),
            scalar(field(destinations, "collection_directory")),
        ),
        (
            "archive_directory".to_string(),
            scalar(field(destinations, "archive_directory")),
        ),
        ("index".to_string(), scalar(field(destinations, "index"))),
        (
            "manifest".to_string(),
            scalar(field(destinations, "manifest")),
        ),
        (
            "source_capsule".to_string(),
            scalar(field(destinations, "source_capsule")),
        ),
    ];
    for part in array(field(destinations, "parts")) {
        declared.push((
            format!("part '{}'", scalar(field(Some(part), "part_id"))),
            scalar(field(Some(part), "path")),
        ));
    }
    for (label, path) in declared {
        if safe_relative_path(&path) && path_exists(&absolute(base, &path)) {
            errors.push(format!(
                "source_locked state has premature {label} destination at '{path}'"
            ));
        }
    }
}

fn manifest_expected_subset(contract: &Py) -> Py {
    let mut routes = field(Some(contract), "leaf_routes")
        .cloned()
        .unwrap_or_else(|| Py::List(Vec::new()));
    if scalar(field(Some(contract), "route_catalog_state")) == "sharded" {
        routes = Py::List(
            array(Some(&routes))
                .iter()
                .filter(|route| scalar(field(Some(route), "lifecycle")) != "closed")
                .cloned()
                .collect(),
        );
    }
    object(vec![
        ("schema_version", integer(1)),
        (
            "contract_id",
            field(Some(contract), "contract_id")
                .cloned()
                .unwrap_or(Py::None),
        ),
        ("migration_state", text("migrated")),
        (
            "current_path",
            field(Some(contract), "current_path")
                .cloned()
                .unwrap_or(Py::None),
        ),
        (
            "index_path",
            field(field(Some(contract), "destinations"), "index")
                .cloned()
                .unwrap_or(Py::None),
        ),
        (
            "source_capsule",
            field(field(Some(contract), "destinations"), "source_capsule")
                .cloned()
                .unwrap_or(Py::None),
        ),
        (
            "source",
            field(Some(contract), "source").cloned().unwrap_or(Py::None),
        ),
        (
            "current_frontier",
            field(Some(contract), "current_frontier")
                .cloned()
                .unwrap_or(Py::None),
        ),
        (
            "marker_prefix",
            field(Some(contract), "marker_prefix")
                .cloned()
                .unwrap_or(Py::None),
        ),
        (
            "parts",
            field(field(Some(contract), "destinations"), "parts")
                .cloned()
                .unwrap_or(Py::None),
        ),
        (
            "regions",
            field(Some(contract), "regions")
                .cloned()
                .unwrap_or(Py::None),
        ),
        ("leaf_routes", routes),
        (
            "migrated_on",
            field(field(Some(contract), "migration_metadata"), "migrated_on")
                .cloned()
                .unwrap_or(Py::None),
        ),
        (
            "reason",
            field(field(Some(contract), "migration_metadata"), "reason")
                .cloned()
                .unwrap_or(Py::None),
        ),
        (
            "verifier",
            field(Some(contract), "verifier")
                .cloned()
                .unwrap_or(Py::None),
        ),
    ])
}

fn validate_manifest(base: &Path, contract: &Py, validation: &mut Validation) {
    let path = scalar(field(field(Some(contract), "destinations"), "manifest"));
    let limits = field(field(Some(contract), "limits"), "manifest");
    let max_bytes = perl::numeric(field(limits, "bytes")) as u64;
    let Some((manifest, raw)) = read_json_object(
        base,
        &path,
        "active task manifest",
        max_bytes,
        &mut validation.errors,
    ) else {
        return;
    };
    let manifest_metrics = metrics(&raw);
    let line_limit = perl::numeric(field(limits, "line_bytes")) as u64;
    if manifest_metrics.line_bytes > line_limit {
        validation.errors.push(format!(
            "active task manifest exceeds line_bytes ceiling {line_limit}"
        ));
    }
    validate_scalar_bounds(
        &manifest,
        perl::numeric(field(limits, "scalar_bytes")) as u64,
        "active task manifest",
        &mut validation.errors,
    );
    reject_unknown(
        Some(&manifest),
        "active task manifest",
        &mut validation.errors,
        &[
            "schema_version",
            "contract_id",
            "migration_state",
            "current_path",
            "index_path",
            "source_capsule",
            "source",
            "current_frontier",
            "marker_prefix",
            "parts",
            "regions",
            "leaf_routes",
            "migrated_on",
            "reason",
            "verifier",
        ],
    );
    let migrated_on = required_scalar(
        Some(&manifest),
        "migrated_on",
        "active task manifest",
        &mut validation.errors,
    );
    if migrated_on.as_ref().is_some_and(|date| !valid_date(date)) {
        validation
            .errors
            .push("active task manifest migrated_on is not an ISO date".to_string());
    }
    required_scalar(
        Some(&manifest),
        "reason",
        "active task manifest",
        &mut validation.errors,
    );
    let expected = manifest_expected_subset(contract);
    if let Py::Dict(fields) = &expected {
        let mut fields: Vec<&(Vec<u32>, Py)> = fields.iter().collect();
        fields.sort_by(|left, right| left.0.cmp(&right.0));
        for (name, expected) in fields {
            let name = pyjson::to_text(name);
            if canonical_json(pyjson::get(&manifest, &name).unwrap_or(&Py::None))
                != canonical_json(expected)
            {
                validation.errors.push(format!(
                    "active task manifest field '{name}' disagrees with the contract"
                ));
            }
        }
    }
    validation.result.insert(
        "manifest_metrics".to_string(),
        metrics_json(manifest_metrics),
    );
}

fn validate_archived_part(base: &Path, contract: &Py, part: &Py, errors: &mut Vec<String>) {
    let part_id = scalar(field(Some(part), "part_id"));
    let from = scalar(field(Some(part), "archived_from"));
    if !from.is_empty() && absolute(base, &from).exists() {
        errors.push(format!(
            "archived part '{part_id}' still exists at its live path '{from}'"
        ));
    }
    for route in array(field(Some(contract), "leaf_routes")) {
        if scalar(field(Some(route), "part_id")) == part_id
            && scalar(field(Some(route), "lifecycle")) != "closed"
        {
            errors.push(format!(
                "archived part '{part_id}' routes open leaf '{}'",
                scalar(field(Some(route), "leaf_id"))
            ));
        }
    }
    if scalar(field(field(Some(contract), "current_frontier"), "part_id")) == part_id {
        errors.push(format!(
            "archived part '{part_id}' is the current frontier's part"
        ));
    }
}

fn validate_part_and_regions(
    base: &Path,
    contract: &Py,
    part: &Py,
    region_raw: &BTreeMap<String, Vec<u8>>,
    seen_regions: &mut BTreeMap<String, usize>,
    part_raw: &mut BTreeMap<String, Vec<u8>>,
    errors: &mut Vec<String>,
) -> Option<Metrics> {
    let path = scalar(field(Some(part), "path"));
    let part_id = scalar(field(Some(part), "part_id"));
    let raw = read_regular(base, &path, &format!("semantic part '{part_id}'"), errors)?;
    part_raw.insert(path.clone(), raw.clone());
    let actual = metrics(&raw);
    compare_metrics(
        &metric_values(actual),
        field(Some(part), "metrics"),
        &format!("semantic part '{part_id}'"),
        errors,
        &["lines", "bytes", "line_bytes"],
    );
    if sha256_hex(&raw) != scalar(field(Some(part), "sha256")) {
        errors.push(format!("semantic part '{part_id}' sha256 mismatch"));
    }
    if !raw.starts_with(scalar(field(Some(part), "heading")).as_bytes()) {
        errors.push(format!("semantic part '{part_id}' lacks its exact heading"));
    }
    let state = scalar(field(Some(part), "state"));
    let archived = state == "archived";
    if archived {
        let live_states = ["active", "legacy", "sealed"]
            .iter()
            .filter(|state| occurrences(&raw, format!("- State: `{state}`").as_bytes()) == 1)
            .count();
        if live_states != 1 {
            errors.push(format!(
                "archived part '{part_id}' lacks exactly one live state literal"
            ));
        }
        validate_archived_part(base, contract, part, errors);
    } else {
        let state_literal = format!("- State: `{state}`");
        if occurrences(&raw, state_literal.as_bytes()) != 1 {
            errors.push(format!(
                "semantic part '{part_id}' lacks state literal '{state_literal}'"
            ));
        }
    }
    if matches!(state.as_str(), "sealed" | "archived") {
        let commit = scalar(field(Some(part), "sealed_commit"));
        let sealed_path = if archived {
            scalar(field(Some(part), "archived_from"))
        } else {
            path.clone()
        };
        if let Some(blob) = git_capture(
            base,
            &format!("sealed part '{part_id}' blob lookup"),
            errors,
            &["rev-parse".to_string(), format!("{commit}:{sealed_path}")],
        ) {
            let blob = trim_newlines(&blob);
            let expected = scalar(field(Some(part), "git_blob"));
            if blob != expected {
                errors.push(format!(
                    "sealed part '{part_id}' Git blob is '{blob}', expected '{expected}'"
                ));
            }
        }
        if let Some(sealed_raw) = git_capture(
            base,
            &format!("sealed part '{part_id}' content read"),
            errors,
            &["show".to_string(), format!("{commit}:{sealed_path}")],
        ) && sealed_raw != raw
        {
            errors.push(format!(
                "sealed part '{part_id}' differs from its sealing commit"
            ));
        }
        let mut command = vec!["-C".to_string(), base.to_string_lossy().into_owned()];
        command.extend([
            "merge-base".to_string(),
            "--is-ancestor".to_string(),
            commit,
            "HEAD".to_string(),
        ]);
        let (_, exit, _) = command_capture("git", &command);
        if exit != 0 {
            errors.push(format!(
                "sealed part '{part_id}' commit is not an ancestor of HEAD"
            ));
        }
    }
    for route in array(field(Some(contract), "leaf_routes")) {
        if scalar(field(Some(route), "part_id")) == part_id
            && scalar(field(Some(route), "origin")) == "post_migration"
        {
            let id = scalar(field(Some(route), "leaf_id"));
            if find_bytes(&raw, id.as_bytes(), 0).is_none() {
                errors.push(format!(
                    "post-migration leaf route '{id}' is absent from its primary part"
                ));
            }
        }
    }

    let prefix = scalar(field(Some(contract), "marker_prefix"));
    for region in array(field(Some(contract), "regions")) {
        if scalar(field(Some(region), "part_id")) != part_id {
            continue;
        }
        let id = scalar(field(Some(region), "region_id"));
        let start = format!("<!-- {prefix}:{id}:start -->\n");
        let end = format!("<!-- {prefix}:{id}:end -->");
        let start_count = occurrences(&raw, start.as_bytes());
        let end_count = occurrences(&raw, end.as_bytes());
        if start_count != 1 {
            errors.push(format!(
                "semantic part '{part_id}' must contain one start marker for region '{id}'"
            ));
        }
        if end_count != 1 {
            errors.push(format!(
                "semantic part '{part_id}' must contain one end marker for region '{id}'"
            ));
        }
        if start_count != 1 || end_count != 1 {
            continue;
        }
        let start_at = find_bytes(&raw, start.as_bytes(), 0).unwrap_or(0) + start.len();
        let end_at = find_bytes(&raw, end.as_bytes(), start_at);
        let Some(end_at) = end_at else {
            errors.push(format!(
                "semantic part '{part_id}' region '{id}' markers are reversed"
            ));
            continue;
        };
        if raw[start_at..end_at] != *region_raw.get(&id).map(Vec::as_slice).unwrap_or(&[]) {
            errors.push(format!(
                "semantic part '{part_id}' region '{id}' payload differs from source"
            ));
        }
        *seen_regions.entry(id).or_insert(0) += 1;
    }
    Some(actual)
}

fn route_catalog_is_sharded(contract: &Py) -> bool {
    scalar(field(Some(contract), "route_catalog_state")) == "sharded"
}

fn route_part_paths(contract: &Py) -> Vec<String> {
    if !route_catalog_is_sharded(contract) {
        return Vec::new();
    }
    let prefix = defined_scalar(field(
        field(Some(contract), "destinations"),
        "route_part_prefix",
    ));
    let per_part = field(field(Some(contract), "limits"), "route_parts");
    let per_part = field(per_part, "routes_per_part");
    let (Some(prefix), Some(per_part)) = (prefix, per_part) else {
        return Vec::new();
    };
    if perl::is_reference(per_part) || perl::numeric(Some(per_part)) < 1.0 {
        return Vec::new();
    }
    let per_part = perl::numeric(Some(per_part)) as usize;
    let routes = array(field(Some(contract), "leaf_routes")).len();
    let count = routes.div_ceil(per_part).max(1);
    (1..=count)
        .map(|position| format!("{prefix}{position:04}.md"))
        .collect()
}

fn landing_leaf_routes(contract: &Py) -> Vec<Py> {
    array(field(Some(contract), "leaf_routes"))
        .iter()
        .filter(|route| {
            !route_catalog_is_sharded(contract) || scalar(field(Some(route), "lifecycle")) == "open"
        })
        .cloned()
        .collect()
}

fn relative_markdown_link(from_path: &str, to_path: &str) -> String {
    let mut source: Vec<&str> = dirname(from_path)
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect();
    let mut target: Vec<&str> = to_path
        .split('/')
        .filter(|part| !part.is_empty() && *part != ".")
        .collect();
    let mut common = 0;
    while common < source.len() && common < target.len() && source[common] == target[common] {
        common += 1;
    }
    source.drain(..common);
    target.drain(..common);
    let parts: Vec<&str> = std::iter::repeat_n("..", source.len())
        .chain(target)
        .collect();
    if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    }
}

fn part_display_label(part_id: &str) -> String {
    let mut words: Vec<&str> = part_id.split('-').collect();
    while words.last() == Some(&"") {
        words.pop();
    }
    let mut label = words.join(" ");
    if let Some(first) = label.as_bytes().first().copied()
        && first.is_ascii_lowercase()
    {
        label.replace_range(0..1, &(first as char).to_uppercase().to_string());
    }
    label
}

fn render_index(contract: &Py) -> Vec<u8> {
    let path = scalar(field(field(Some(contract), "destinations"), "index"));
    let part_by_id: BTreeMap<String, &Py> =
        array(field(field(Some(contract), "destinations"), "parts"))
            .iter()
            .map(|part| (scalar(field(Some(part), "part_id")), part))
            .collect();
    let tree_id = scalar(field(field(Some(contract), "identity"), "tree_id"));
    let mut raw = format!("# {tree_id} task-evidence index\n\n");
    raw.push_str(&format!(
        "- [Current root]({})\n",
        relative_markdown_link(&path, &scalar(field(Some(contract), "current_path")))
    ));
    raw.push_str(&format!(
        "- [Manifest]({})\n\n",
        relative_markdown_link(
            &path,
            &scalar(field(field(Some(contract), "destinations"), "manifest"))
        )
    ));
    raw.push_str("## Semantic parts\n\n");
    for part in array(field(field(Some(contract), "destinations"), "parts")) {
        let part_id = scalar(field(Some(part), "part_id"));
        let archived = if scalar(field(Some(part), "state")) == "archived" {
            " — archived"
        } else {
            ""
        };
        raw.push_str(&format!(
            "- [{}]({}){archived}\n",
            part_display_label(&part_id),
            relative_markdown_link(&path, &scalar(field(Some(part), "path")))
        ));
    }
    if route_catalog_is_sharded(contract) {
        raw.push_str("\n## Open leaf routes\n\n");
        raw.push_str(
            "Every leaf a session can still act on — one this tree does not record as `done` or\n",
        );
        raw.push_str("`superseded`. The complete route set for every leaf the tree has ever declared is in the\n");
        raw.push_str("route catalog below, so this landing measures work in flight rather than project age.\n\n");
    } else {
        raw.push_str("\n## Primary leaf routes\n\n");
    }
    raw.push_str("| Leaf | Primary detail |\n| --- | --- |\n");
    for route in landing_leaf_routes(contract) {
        let part_id = scalar(field(Some(&route), "part_id"));
        let Some(part) = part_by_id.get(&part_id) else {
            continue;
        };
        raw.push_str(&format!(
            "| `{}` | [{}]({}) |\n",
            scalar(field(Some(&route), "leaf_id")),
            part_display_label(&part_id),
            relative_markdown_link(&path, &scalar(field(Some(part), "path")))
        ));
    }
    let route_paths = route_part_paths(contract);
    if !route_paths.is_empty() {
        raw.push_str("\n## Complete route catalog\n\n");
        for (position, route_path) in route_paths.iter().enumerate() {
            raw.push_str(&format!(
                "- [Route catalog part {:04}]({})\n",
                position + 1,
                relative_markdown_link(&path, route_path)
            ));
        }
    }
    raw.push_str("\n## Exact provenance\n\n");
    raw.push_str(&format!(
        "- [Source]({})\n\n",
        relative_markdown_link(
            &path,
            &scalar(field(
                field(Some(contract), "destinations"),
                "source_capsule"
            ))
        )
    ));
    raw.push_str(&format!(
        "## Verification\n\nRun `{}` from the repository root.\n",
        scalar(field(Some(contract), "verifier"))
    ));
    raw.into_bytes()
}

fn render_route_part(contract: &Py, position: usize, route_paths: &[String]) -> Vec<u8> {
    let path = &route_paths[position];
    let part_by_id: BTreeMap<String, &Py> =
        array(field(field(Some(contract), "destinations"), "parts"))
            .iter()
            .map(|part| (scalar(field(Some(part), "part_id")), part))
            .collect();
    let per_part = perl::numeric(field(
        field(field(Some(contract), "limits"), "route_parts"),
        "routes_per_part",
    )) as usize;
    let routes = array(field(Some(contract), "leaf_routes"));
    let first = position * per_part;
    let last = (first + per_part).min(routes.len());
    let tree_id = scalar(field(field(Some(contract), "identity"), "tree_id"));
    let mut raw = format!("# {tree_id} route catalog part {:04}\n\n", position + 1);
    raw.push_str("> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with\n");
    raw.push_str(&format!("> `{}`.\n\n", writer_command_for(contract)));
    raw.push_str(
        "Complete membership for this range, closed leaves included. The bounded landing is\n",
    );
    raw.push_str(&format!(
        "[the task-evidence index]({}); it carries the open leaves only.\n\n",
        relative_markdown_link(
            path,
            &scalar(field(field(Some(contract), "destinations"), "index"))
        )
    ));
    raw.push_str("| Leaf | Lifecycle | Primary detail |\n| --- | --- | --- |\n");
    if first < routes.len() {
        for route in &routes[first..last] {
            let part_id = scalar(field(Some(route), "part_id"));
            let Some(part) = part_by_id.get(&part_id) else {
                continue;
            };
            raw.push_str(&format!(
                "| `{}` | `{}` | [{}]({}) |\n",
                scalar(field(Some(route), "leaf_id")),
                scalar(field(Some(route), "lifecycle")),
                part_display_label(&part_id),
                relative_markdown_link(path, &scalar(field(Some(part), "path")))
            ));
        }
    }
    raw.into_bytes()
}

fn unplanned_route_part_paths(
    base: &Path,
    contract: &Py,
    route_paths: &[String],
    errors: &mut Vec<String>,
) -> Vec<String> {
    let prefix = scalar(field(
        field(Some(contract), "destinations"),
        "route_part_prefix",
    ));
    if prefix.is_empty() {
        return Vec::new();
    }
    let directory = dirname(&prefix);
    let stem = prefix
        .strip_prefix(&format!("{directory}/"))
        .unwrap_or(&prefix);
    let planned: BTreeSet<&str> = route_paths.iter().map(String::as_str).collect();
    let entries = match fs::read_dir(absolute(base, directory)) {
        Ok(entries) => entries,
        Err(_) => {
            errors.push(format!("cannot read route catalog directory '{directory}'"));
            return Vec::new();
        }
    };
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| {
            name.strip_prefix(stem)
                .and_then(|rest| rest.strip_suffix(".md"))
                .is_some_and(|rest| !rest.contains('/'))
        })
        .collect();
    names.sort();
    names
        .into_iter()
        .map(|name| format!("{directory}/{name}"))
        .filter(|candidate| !planned.contains(candidate.as_str()))
        .collect()
}

fn validate_route_parts(base: &Path, contract: &Py, validation: &mut Validation) {
    if !route_catalog_is_sharded(contract) {
        return;
    }
    let route_paths = route_part_paths(contract);
    let spec = field(field(Some(contract), "limits"), "route_parts");
    if route_paths.is_empty() {
        validation
            .errors
            .push("contract declares no derivable route catalog part".to_string());
        return;
    }
    if route_paths.len() as f64 > perl::numeric(field(spec, "max_parts")) {
        validation
            .errors
            .push("route catalog part count exceeds its declared max_parts".to_string());
    }
    let mut declared_paths = BTreeMap::from([
        (
            scalar(field(field(Some(contract), "destinations"), "index")),
            "index".to_string(),
        ),
        (
            scalar(field(field(Some(contract), "destinations"), "manifest")),
            "manifest".to_string(),
        ),
    ]);
    for part in array(field(field(Some(contract), "destinations"), "parts")) {
        declared_paths.insert(
            scalar(field(Some(part), "path")),
            format!("semantic part '{}'", scalar(field(Some(part), "part_id"))),
        );
    }
    let mut actual = BTreeMap::from([
        ("files".to_string(), 0),
        ("lines_each".to_string(), 0),
        ("bytes_each".to_string(), 0),
        ("line_bytes_each".to_string(), 0),
        ("lines_total".to_string(), 0),
        ("bytes_total".to_string(), 0),
    ]);
    for (position, path) in route_paths.iter().enumerate() {
        if let Some(declared) = declared_paths.get(path) {
            validation.errors.push(format!(
                "route catalog part '{path}' collides with the declared {declared}"
            ));
            continue;
        }
        let Some(raw) = read_regular(
            base,
            path,
            &format!("route catalog part '{path}'"),
            &mut validation.errors,
        ) else {
            continue;
        };
        if raw != render_route_part(contract, position, &route_paths) {
            validation.errors.push(format!(
                "route catalog part '{path}' differs from its derived form; regenerate with {}",
                writer_command_for(contract)
            ));
        }
        let current = metrics(&raw);
        *actual.get_mut("files").unwrap() += 1;
        *actual.get_mut("lines_total").unwrap() += current.lines;
        *actual.get_mut("bytes_total").unwrap() += current.bytes;
        for (name, value) in [
            ("lines_each", current.lines),
            ("bytes_each", current.bytes),
            ("line_bytes_each", current.line_bytes),
        ] {
            let entry = actual.get_mut(name).unwrap();
            *entry = (*entry).max(value);
        }
    }
    for stale in unplanned_route_part_paths(base, contract, &route_paths, &mut validation.errors) {
        validation.errors.push(format!(
            "unplanned route catalog file '{stale}' is not derived by the contract"
        ));
    }
    let mapping: Vec<(&str, &str)> = actual
        .keys()
        .map(|key| (key.as_str(), key.as_str()))
        .collect();
    enforce_ceilings(
        &actual,
        field(spec, "enforcement_ceilings"),
        "route catalog collection",
        &mut validation.errors,
        &mapping,
    );
    apply_live_pressure(
        &actual,
        spec,
        "route catalog collection",
        &mapping,
        &mut validation.errors,
        &mut validation.warnings,
    );
    validation.result.insert(
        "route_part_collection_metrics".to_string(),
        values_json(&actual),
    );
}

fn declared_node_statuses(raw: Option<&[u8]>, leaf_id: &str) -> Vec<Option<String>> {
    let Some(raw) = raw else {
        return Vec::new();
    };
    if leaf_id.is_empty() {
        return Vec::new();
    }
    let needle = format!("- ID: `{leaf_id}`").into_bytes();
    let mut statuses = Vec::new();
    let mut cursor = 0;
    while cursor < raw.len() {
        let Some(found) = find_bytes(raw, &needle, cursor) else {
            break;
        };
        if found != 0 && raw[found - 1] != b'\n' {
            cursor = found + needle.len();
            continue;
        }
        let Some(newline_relative) = raw[found + needle.len()..]
            .iter()
            .position(|byte| *byte == b'\n')
        else {
            break;
        };
        let newline = found + needle.len() + newline_relative;
        let inline_end = if newline > found + needle.len() && raw[newline - 1] == b'\r' {
            newline - 1
        } else {
            newline
        };
        let inline = &raw[found + needle.len()..inline_end];
        let tail_start = newline + 1;
        let mut tail_end = tail_start;
        for _ in 0..2 {
            if tail_end >= raw.len() {
                break;
            }
            let Some(next) = raw[tail_end..].iter().position(|byte| *byte == b'\n') else {
                break;
            };
            tail_end += next + 1;
        }
        let tail = &raw[tail_start..tail_end];
        let mut status = inline_status(inline);
        if status.is_none() {
            for line in tail.split(|byte| *byte == b'\n') {
                let line = line.strip_suffix(b"\r").unwrap_or(line);
                let indentation = line
                    .iter()
                    .take_while(|byte| matches!(byte, b' ' | b'\t'))
                    .count();
                if indentation > 0 {
                    status = inline_status(&line[indentation..]);
                    if status.is_some() {
                        break;
                    }
                }
            }
        }
        statuses.push(status);
        cursor = tail_end;
    }
    statuses
}

fn inline_status(raw: &[u8]) -> Option<String> {
    let mut candidates = Vec::new();
    for name in ["State", "Status"] {
        let needle = format!("{name}: `").into_bytes();
        let mut offset = 0;
        while let Some(found) = find_bytes(raw, &needle, offset) {
            if found == 0 || !(raw[found - 1].is_ascii_alphanumeric() || raw[found - 1] == b'_') {
                candidates.push(found);
            }
            offset = found + needle.len();
        }
    }
    candidates.sort_unstable();
    for start in candidates {
        let Some(open) = raw[start..].iter().position(|byte| *byte == b'`') else {
            continue;
        };
        let open = open + start + 1;
        let Some(close) = raw[open..].iter().position(|byte| *byte == b'`') else {
            continue;
        };
        let close = close + open;
        if close > open
            && !raw[open..close]
                .iter()
                .any(|byte| matches!(byte, b'\r' | b'\n'))
        {
            return Some(String::from_utf8_lossy(&raw[open..close]).into_owned());
        }
    }
    None
}

fn strip_marked_regions(raw: Option<&[u8]>, marker_prefix: &str) -> Vec<u8> {
    let Some(raw) = raw else {
        return Vec::new();
    };
    if marker_prefix.is_empty() {
        return raw.to_vec();
    }
    let start_prefix = format!("<!-- {marker_prefix}:").into_bytes();
    let mut output = Vec::new();
    let mut cursor = 0;
    while let Some(start) = find_bytes(raw, &start_prefix, cursor) {
        let id_start = start + start_prefix.len();
        let Some(id_end_relative) = raw[id_start..]
            .windows(10)
            .position(|window| window == b":start -->")
        else {
            break;
        };
        let id_end = id_start + id_end_relative;
        if id_end == id_start
            || !raw[id_start..id_end]
                .iter()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
        {
            cursor = id_start;
            continue;
        }
        let start_line_end = id_end + b":start -->".len();
        if raw.get(start_line_end) != Some(&b'\n') {
            cursor = start_line_end;
            continue;
        }
        let end_prefix = format!("<!-- {marker_prefix}:").into_bytes();
        let mut search = start_line_end + 1;
        let mut found_end = None;
        while let Some(end) = find_bytes(raw, &end_prefix, search) {
            let end_id_start = end + end_prefix.len();
            let Some(end_id_relative) = raw[end_id_start..]
                .windows(8)
                .position(|window| window == b":end -->")
            else {
                break;
            };
            let end_id = end_id_start + end_id_relative;
            if end_id > end_id_start
                && raw[end_id_start..end_id]
                    .iter()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
            {
                found_end = Some(end_id + b":end -->".len());
                break;
            }
            search = end_id_start;
        }
        let Some(end) = found_end else {
            break;
        };
        output.extend_from_slice(&raw[cursor..start]);
        cursor = end;
    }
    output.extend_from_slice(&raw[cursor..]);
    output
}

fn validate_route_lifecycles(
    contract: &Py,
    part_raw: &BTreeMap<String, Vec<u8>>,
    validation: &mut Validation,
) {
    if !route_catalog_is_sharded(contract) {
        return;
    }
    let part_path: BTreeMap<String, String> =
        array(field(field(Some(contract), "destinations"), "parts"))
            .iter()
            .map(|part| {
                (
                    scalar(field(Some(part), "part_id")),
                    scalar(field(Some(part), "path")),
                )
            })
            .collect();
    let mut unverified = 0_u64;
    let mut corroborated = BTreeSet::new();
    for route in array(field(Some(contract), "leaf_routes")) {
        let part_id = scalar(field(Some(route), "part_id"));
        let leaf_id = scalar(field(Some(route), "leaf_id"));
        let path = part_path.get(&part_id).cloned().unwrap_or_default();
        let raw = part_raw.get(&path).map(Vec::as_slice);
        let sealed_and_free = declared_node_statuses(raw, &leaf_id);
        let stripped = strip_marked_regions(raw, &scalar(field(Some(contract), "marker_prefix")));
        let free = declared_node_statuses(Some(&stripped), &leaf_id);
        let stratum = if free.is_empty() {
            "sealed"
        } else {
            "post-migration"
        };
        let statuses = if free.is_empty() {
            &sealed_and_free
        } else {
            &free
        };
        if statuses.len() > 1 {
            validation.errors.push(format!(
                "leaf route '{leaf_id}' is declared {} times in the {stratum} stratum of its primary part '{part_id}'",
                statuses.len()
            ));
            corroborated.insert(leaf_id);
            continue;
        }
        let Some(status) = statuses.first().and_then(|status| status.clone()) else {
            unverified += 1;
            continue;
        };
        corroborated.insert(leaf_id.clone());
        let observed = if matches!(status.as_str(), "done" | "superseded") {
            "closed"
        } else {
            "open"
        };
        let lifecycle = scalar(field(Some(route), "lifecycle"));
        if observed != lifecycle {
            validation.errors.push(format!(
                "leaf route '{leaf_id}' declares lifecycle '{lifecycle}' but its primary part records status '{status}'"
            ));
        }
    }
    let declared = root_owner_ids(contract);
    if scalar(field(Some(contract), "input_state")) == "complete" && !declared.is_empty() {
        for route in array(field(Some(contract), "leaf_routes"))
            .iter()
            .filter(|route| scalar(field(Some(route), "lifecycle")) == "closed")
        {
            let leaf_id = scalar(field(Some(route), "leaf_id"));
            if corroborated.contains(&leaf_id) && declared.contains(&leaf_id) {
                validation.errors.push(format!(
                    "closed leaf route '{leaf_id}' is still declared in the root owner registry; a closed leaf whose part records it belongs to the route catalog"
                ));
            } else if !corroborated.contains(&leaf_id) && !declared.contains(&leaf_id) {
                validation.errors.push(format!(
                    "closed leaf route '{leaf_id}' has no status record in its primary part, so the root owner registry must keep declaring it"
                ));
            }
        }
    }
    let allowed = field(
        field(field(Some(contract), "limits"), "route_parts"),
        "max_unverified_routes",
    );
    if allowed.is_some_and(|value| !perl::is_reference(value))
        && unverified as f64 > perl::numeric(allowed)
    {
        validation.errors.push(format!(
            "{unverified} leaf routes declare a lifecycle their primary part does not corroborate, above the declared max_unverified_routes {}",
            scalar(allowed)
        ));
    }
    validation.result.insert(
        "routes_without_declared_status".to_string(),
        integer(unverified),
    );
}

fn index_route_rows(
    raw: &[u8],
    source_path: &str,
    errors: &mut Vec<String>,
) -> BTreeMap<String, Vec<String>> {
    let mut rows = BTreeMap::<String, Vec<String>>::new();
    for physical in raw.split_inclusive(|byte| *byte == b'\n') {
        if !physical.starts_with(b"| `") {
            continue;
        }
        let line = physical
            .strip_suffix(b"\n")
            .unwrap_or(physical)
            .strip_suffix(b"\r")
            .unwrap_or(physical.strip_suffix(b"\n").unwrap_or(physical));
        let Ok(line) = std::str::from_utf8(line) else {
            continue;
        };
        let line = line.trim_end();
        let Some(rest) = line.strip_prefix("| `") else {
            continue;
        };
        let Some((leaf_id, rest)) = rest.split_once("` | [") else {
            continue;
        };
        if leaf_id.is_empty() || leaf_id.contains('`') {
            continue;
        }
        let Some((label, target)) = rest.split_once("](") else {
            continue;
        };
        if label.is_empty() || label.contains([']', '\r', '\n']) {
            continue;
        }
        let Some(target) = target.strip_suffix(") |") else {
            continue;
        };
        if target.is_empty() || target.contains([')', '\r', '\n']) {
            continue;
        }
        let resolved = normalize_link(source_path, target);
        if resolved
            .as_deref()
            .is_none_or(|path| !safe_relative_path(path))
        {
            errors.push(format!(
                "active task index route row for '{leaf_id}' has unsafe target"
            ));
        }
        rows.entry(leaf_id.to_string())
            .or_default()
            .push(resolved.unwrap_or_default());
    }
    rows
}

fn validate_index(base: &Path, contract: &Py, validation: &mut Validation) {
    let path = scalar(field(field(Some(contract), "destinations"), "index"));
    let Some(raw) = read_regular(base, &path, "active task index", &mut validation.errors) else {
        return;
    };
    let actual = metric_values(metrics(&raw));
    let spec = field(field(Some(contract), "limits"), "index");
    let mapping = [
        ("lines", "lines"),
        ("bytes", "bytes"),
        ("line_bytes", "line_bytes"),
    ];
    enforce_ceilings(
        &actual,
        field(spec, "enforcement_ceilings"),
        "active task index",
        &mut validation.errors,
        &mapping,
    );
    apply_live_pressure(
        &actual,
        spec,
        "active task index",
        &mapping,
        &mut validation.errors,
        &mut validation.warnings,
    );
    validate_required_literals(
        &raw,
        field(
            field(Some(contract), "migrated_requirements"),
            "index_required_literals",
        ),
        "active task index",
        &mut validation.errors,
    );
    if raw != render_index(contract) {
        validation.errors.push(format!(
            "active task index differs from its derived form; regenerate with {}",
            writer_command_for(contract)
        ));
    }
    let links = link_counts(&raw, &path, "active task index", &mut validation.errors);
    let mut expected = BTreeMap::from([
        (scalar(field(Some(contract), "current_path")), 1_usize),
        (
            scalar(field(field(Some(contract), "destinations"), "manifest")),
            1,
        ),
        (
            scalar(field(
                field(Some(contract), "destinations"),
                "source_capsule",
            )),
            1,
        ),
    ]);
    for part in array(field(field(Some(contract), "destinations"), "parts")) {
        *expected
            .entry(scalar(field(Some(part), "path")))
            .or_insert(0) += 1;
    }
    for route_path in route_part_paths(contract) {
        *expected.entry(route_path).or_insert(0) += 1;
    }
    let part_path: BTreeMap<String, String> =
        array(field(field(Some(contract), "destinations"), "parts"))
            .iter()
            .map(|part| {
                (
                    scalar(field(Some(part), "part_id")),
                    scalar(field(Some(part), "path")),
                )
            })
            .collect();
    let route_rows = index_route_rows(&raw, &path, &mut validation.errors);
    for route in array(field(Some(contract), "leaf_routes")) {
        let leaf_id = scalar(field(Some(route), "leaf_id"));
        let expected_path = part_path
            .get(&scalar(field(Some(route), "part_id")))
            .cloned()
            .unwrap_or_default();
        let rows = route_rows.get(&leaf_id).cloned().unwrap_or_default();
        if route_catalog_is_sharded(contract) && scalar(field(Some(route), "lifecycle")) != "open" {
            if !rows.is_empty() {
                validation.errors.push(format!(
                    "active task index must not route closed leaf '{leaf_id}'; a closed leaf belongs to the route catalog"
                ));
            }
            continue;
        }
        if rows.len() != 1 {
            validation.errors.push(format!(
                "active task index must route leaf '{leaf_id}' exactly once"
            ));
        }
        if rows.len() == 1 && rows[0] != expected_path {
            validation.errors.push(format!(
                "active task index leaf '{leaf_id}' routes to '{}', expected '{expected_path}'",
                rows[0]
            ));
        }
        if !expected_path.is_empty() {
            *expected.entry(expected_path).or_insert(0) += 1;
        }
    }
    let declared_routes: BTreeSet<String> = array(field(Some(contract), "leaf_routes"))
        .iter()
        .map(|route| scalar(field(Some(route), "leaf_id")))
        .collect();
    for id in route_rows
        .keys()
        .filter(|id| !declared_routes.contains(*id))
    {
        validation.errors.push(format!(
            "active task index contains undeclared leaf route '{id}'"
        ));
    }
    for (target, count) in &expected {
        let actual = links.get(target).copied().unwrap_or(0);
        if actual != *count {
            validation.errors.push(format!(
                "active task index links '{target}' {actual} times, expected {count}"
            ));
        }
    }
    for target in links
        .keys()
        .filter(|target| !expected.contains_key(*target))
    {
        validation.errors.push(format!(
            "active task index contains unexpected local link '{target}'"
        ));
    }
    validation
        .result
        .insert("index_metrics".to_string(), values_json(&actual));
}

fn validate_migrated(
    base: &Path,
    contract: &Py,
    region_raw: &BTreeMap<String, Vec<u8>>,
    validation: &mut Validation,
    skip_derived: bool,
) {
    let current_path = scalar(field(Some(contract), "current_path"));
    if let Some(root_raw) = read_regular(
        base,
        &current_path,
        "bounded active root",
        &mut validation.errors,
    ) {
        let actual = metric_values(metrics(&root_raw));
        let spec = field(field(Some(contract), "limits"), "root");
        let mapping = [
            ("lines", "lines"),
            ("bytes", "bytes"),
            ("line_bytes", "line_bytes"),
        ];
        enforce_ceilings(
            &actual,
            field(spec, "enforcement_ceilings"),
            "bounded active root",
            &mut validation.errors,
            &mapping,
        );
        apply_live_pressure(
            &actual,
            spec,
            "bounded active root",
            &mapping,
            &mut validation.errors,
            &mut validation.warnings,
        );
        validate_required_literals(
            &root_raw,
            field(
                field(Some(contract), "migrated_requirements"),
                "root_required_literals",
            ),
            "bounded active root",
            &mut validation.errors,
        );
        validate_forbidden_literals(
            &root_raw,
            field(
                field(Some(contract), "migrated_requirements"),
                "root_forbidden_literals",
            ),
            "bounded active root",
            &mut validation.errors,
        );
        let identity = field(Some(contract), "identity");
        if !root_raw.starts_with(scalar(field(identity, "heading")).as_bytes()) {
            validation
                .errors
                .push("bounded active root heading does not match contract identity".to_string());
        }
        let tree_literal = format!("- Tree ID: `{}`", scalar(field(identity, "tree_id")));
        if occurrences(&root_raw, tree_literal.as_bytes()) != 1 {
            validation.errors.push(
                "bounded active root does not carry exactly one declared tree identity".to_string(),
            );
        }
        if occurrences(
            &root_raw,
            scalar(field(identity, "active_status_literal")).as_bytes(),
        ) == 0
        {
            validation
                .errors
                .push("bounded active root lacks the declared active-status literal".to_string());
        }
        if occurrences(
            &root_raw,
            scalar(field(field(Some(contract), "current_frontier"), "literal")).as_bytes(),
        ) != 1
        {
            validation.errors.push(
                "bounded active root does not carry exactly one current-frontier literal"
                    .to_string(),
            );
        }
        let links = link_counts(
            &root_raw,
            &current_path,
            "bounded active root",
            &mut validation.errors,
        );
        let index = scalar(field(field(Some(contract), "destinations"), "index"));
        if links.get(&index).copied().unwrap_or(0) != 1 {
            validation.errors.push(format!(
                "bounded active root must link index '{index}' exactly once"
            ));
        }
        validation
            .result
            .insert("root_metrics".to_string(), values_json(&actual));
    }
    if !skip_derived {
        validate_index(base, contract, validation);
        validate_route_parts(base, contract, validation);
    }
    let mut seen_regions = BTreeMap::new();
    let mut part_raw = BTreeMap::new();
    let mut actual = BTreeMap::from([
        ("files".to_string(), 0),
        ("lines_each".to_string(), 0),
        ("bytes_each".to_string(), 0),
        ("line_bytes_each".to_string(), 0),
        ("lines_total".to_string(), 0),
        ("bytes_total".to_string(), 0),
    ]);
    let mut archived_actual = BTreeMap::from([
        ("files".to_string(), 0),
        ("lines_total".to_string(), 0),
        ("bytes_total".to_string(), 0),
    ]);
    for part in array(field(field(Some(contract), "destinations"), "parts")) {
        let Some(part_metrics) = validate_part_and_regions(
            base,
            contract,
            part,
            region_raw,
            &mut seen_regions,
            &mut part_raw,
            &mut validation.errors,
        ) else {
            continue;
        };
        if scalar(field(Some(part), "state")) == "archived" {
            *archived_actual.get_mut("files").unwrap() += 1;
            *archived_actual.get_mut("lines_total").unwrap() += part_metrics.lines;
            *archived_actual.get_mut("bytes_total").unwrap() += part_metrics.bytes;
            continue;
        }
        *actual.get_mut("files").unwrap() += 1;
        *actual.get_mut("lines_total").unwrap() += part_metrics.lines;
        *actual.get_mut("bytes_total").unwrap() += part_metrics.bytes;
        for (name, value) in [
            ("lines_each", part_metrics.lines),
            ("bytes_each", part_metrics.bytes),
            ("line_bytes_each", part_metrics.line_bytes),
        ] {
            let entry = actual.get_mut(name).unwrap();
            *entry = (*entry).max(value);
        }
    }
    for region in array(field(Some(contract), "regions")) {
        let id = scalar(field(Some(region), "region_id"));
        let count = seen_regions.get(&id).copied().unwrap_or(0);
        if count != 1 {
            validation.errors.push(format!(
                "source region '{id}' occurs in {count} semantic parts, expected exactly one"
            ));
        }
    }
    let part_spec = field(field(Some(contract), "limits"), "parts");
    let mapping: Vec<(&str, &str)> = actual
        .keys()
        .map(|key| (key.as_str(), key.as_str()))
        .collect();
    enforce_ceilings(
        &actual,
        field(part_spec, "enforcement_ceilings"),
        "semantic part collection",
        &mut validation.errors,
        &mapping,
    );
    apply_live_pressure(
        &actual,
        part_spec,
        "semantic part collection",
        &mapping,
        &mut validation.errors,
        &mut validation.warnings,
    );
    validation
        .result
        .insert("part_collection_metrics".to_string(), values_json(&actual));
    validation.result.insert(
        "archived_part_metrics".to_string(),
        values_json(&archived_actual),
    );
    validate_route_lifecycles(contract, &part_raw, validation);
    if !skip_derived {
        validate_manifest(base, contract, validation);
    }
}

fn validate_tree(
    project_root: &Path,
    base: &Path,
    relative_contract: &str,
    skip_derived: bool,
) -> Validation {
    let mut validation = Validation::default();
    if !safe_relative_path(relative_contract) {
        validation
            .errors
            .push("contract path is absolute, escaping, or malformed".to_string());
        return validation;
    }
    let Some((contract, _)) = read_json_object(
        base,
        relative_contract,
        "contract",
        131_072,
        &mut validation.errors,
    ) else {
        return validation;
    };
    validate_contract_schema(
        project_root,
        &contract,
        relative_contract,
        &mut validation.errors,
    );
    let migration_state = scalar(field(Some(&contract), "migration_state"));
    let input_state = scalar(field(Some(&contract), "input_state"));
    validation
        .result
        .insert("migration_state".to_string(), text(&migration_state));
    validation
        .result
        .insert("input_state".to_string(), text(&input_state));
    let locked = migration_state == "source_locked";
    let source_path = if locked {
        scalar(field(Some(&contract), "current_path"))
    } else {
        scalar(field(
            field(Some(&contract), "destinations"),
            "source_capsule",
        ))
    };
    let source = validate_source_and_inputs(base, &contract, &source_path, locked, &mut validation);
    if locked {
        validate_destinations_absent(base, &contract, &mut validation.errors);
    } else if migration_state == "migrated"
        && let Some((_, region_raw)) = source
    {
        validate_migrated(base, &contract, &region_raw, &mut validation, skip_derived);
    }
    validation.result.insert(
        "warnings".to_string(),
        Py::List(
            validation
                .warnings
                .iter()
                .map(|warning| text(warning))
                .collect(),
        ),
    );
    validation
}

fn render_semantic_part(contract: &Py, source_raw: &[u8], part: &Py) -> Vec<u8> {
    let lines = split_source_lines(source_raw);
    let part_id = scalar(field(Some(part), "part_id"));
    let mut raw = format!(
        "{}\n\n- Part ID: `{part_id}`\n- State: `{}`\n\n",
        scalar(field(Some(part), "heading")),
        scalar(field(Some(part), "state"))
    )
    .into_bytes();
    for region in array(field(Some(contract), "regions")) {
        if scalar(field(Some(region), "part_id")) != part_id {
            continue;
        }
        let id = scalar(field(Some(region), "region_id"));
        let start = perl::numeric(field(Some(region), "start_line")) as usize;
        let end = perl::numeric(field(Some(region), "end_line")) as usize;
        if start < 1 || end < start || end > lines.len() {
            continue;
        }
        raw.extend_from_slice(
            format!(
                "<!-- {}:{id}:start -->\n",
                scalar(field(Some(contract), "marker_prefix"))
            )
            .as_bytes(),
        );
        raw.extend_from_slice(&lines[start - 1..end].concat());
        raw.extend_from_slice(
            format!(
                "<!-- {}:{id}:end -->\n\n",
                scalar(field(Some(contract), "marker_prefix"))
            )
            .as_bytes(),
        );
    }
    if raw.ends_with(b"\n\n") {
        raw.truncate(raw.len() - 1);
    }
    raw
}

fn validate_migration_root_template(
    contract: &Py,
    root_raw: &[u8],
    errors: &mut Vec<String>,
) -> Metrics {
    let actual = metric_values(metrics(root_raw));
    let spec = field(field(Some(contract), "limits"), "root");
    let mapping = [
        ("lines", "lines"),
        ("bytes", "bytes"),
        ("line_bytes", "line_bytes"),
    ];
    enforce_ceilings(
        &actual,
        field(spec, "enforcement_ceilings"),
        "migration root template",
        errors,
        &mapping,
    );
    let mut warnings = Vec::new();
    apply_live_pressure(
        &actual,
        spec,
        "migration root template",
        &mapping,
        errors,
        &mut warnings,
    );
    for warning in warnings {
        errors.push(format!(
            "migration root template begins under warning pressure: {warning}"
        ));
    }
    validate_required_literals(
        root_raw,
        field(
            field(Some(contract), "migrated_requirements"),
            "root_required_literals",
        ),
        "migration root template",
        errors,
    );
    validate_forbidden_literals(
        root_raw,
        field(
            field(Some(contract), "migrated_requirements"),
            "root_forbidden_literals",
        ),
        "migration root template",
        errors,
    );
    let identity = field(Some(contract), "identity");
    if !root_raw.starts_with(scalar(field(identity, "heading")).as_bytes()) {
        errors.push("migration root template heading does not match contract identity".to_string());
    }
    let tree_literal = format!("- Tree ID: `{}`", scalar(field(identity, "tree_id")));
    if occurrences(root_raw, tree_literal.as_bytes()) != 1 {
        errors.push(
            "migration root template does not carry exactly one declared tree identity".to_string(),
        );
    }
    if occurrences(
        root_raw,
        scalar(field(identity, "active_status_literal")).as_bytes(),
    ) == 0
    {
        errors.push("migration root template lacks the declared active-status literal".to_string());
    }
    if occurrences(
        root_raw,
        scalar(field(field(Some(contract), "current_frontier"), "literal")).as_bytes(),
    ) != 1
    {
        errors.push(
            "migration root template does not carry exactly one current-frontier literal"
                .to_string(),
        );
    }
    let current_path = scalar(field(Some(contract), "current_path"));
    let links = link_counts(root_raw, &current_path, "migration root template", errors);
    let index = scalar(field(field(Some(contract), "destinations"), "index"));
    if links.get(&index).copied().unwrap_or(0) != 1 {
        errors.push(format!(
            "migration root template must link index '{index}' exactly once"
        ));
    }
    for target in links.keys().filter(|target| **target != index) {
        errors.push(format!(
            "migration root template contains unexpected local link '{target}'"
        ));
    }
    metrics(root_raw)
}

fn build_migration_outputs(
    contract: &Py,
    contract_rel: &str,
    source_raw: &[u8],
    root_raw: &[u8],
) -> (Py, BTreeMap<String, Vec<u8>>) {
    let mut migrated = contract.clone();
    set_field(&mut migrated, "migration_state", text("migrated"));
    let mut outputs = BTreeMap::new();
    outputs.insert(
        scalar(field(
            field(Some(&migrated), "destinations"),
            "source_capsule",
        )),
        source_raw.to_vec(),
    );
    let parts_snapshot = array(field(field(Some(&migrated), "destinations"), "parts")).to_vec();
    for (index, part) in parts_snapshot.iter().enumerate() {
        let raw = render_semantic_part(&migrated, source_raw, part);
        let path = scalar(field(Some(part), "path"));
        outputs.insert(path, raw.clone());
        if let Some(Py::List(parts)) = nested_field_mut(&mut migrated, &["destinations", "parts"])
            && let Some(actual) = parts.get_mut(index)
        {
            set_field(actual, "sha256", text(&sha256_hex(&raw)));
            set_field(actual, "metrics", metrics_json(metrics(&raw)));
        }
    }
    outputs.insert(
        scalar(field(field(Some(&migrated), "destinations"), "index")),
        render_index(&migrated),
    );
    let route_paths = route_part_paths(&migrated);
    for (position, path) in route_paths.iter().enumerate() {
        outputs.insert(
            path.clone(),
            render_route_part(&migrated, position, &route_paths),
        );
    }
    outputs.insert(
        scalar(field(field(Some(&migrated), "destinations"), "manifest")),
        pretty_json(&manifest_expected_subset(&migrated)),
    );
    outputs.insert(contract_rel.to_string(), pretty_json(&migrated));
    outputs.insert(
        scalar(field(Some(&migrated), "current_path")),
        root_raw.to_vec(),
    );
    (migrated, outputs)
}

fn write_raw_atomic(base: &Path, relative: &str, raw: &[u8]) -> Result<(), String> {
    if !safe_relative_path(relative) {
        return Err(format!(
            "active-task-evidence migration: unsafe output path '{relative}'\n"
        ));
    }
    let path = absolute(base, relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "active-task-evidence migration: {relative}: {}\n",
                os_error(&error)
            )
        })?;
    }
    let temporary = PathBuf::from(format!(
        "{}.active-task-evidence.{}",
        path.display(),
        std::process::id()
    ));
    let result = (|| -> Result<(), String> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temporary)
            .map_err(|error| {
                format!(
                    "active-task-evidence migration: cannot create temporary for '{relative}': {}\n",
                    os_error(&error)
                )
            })?;
        file.write_all(raw).map_err(|error| {
            format!(
                "active-task-evidence migration: {relative}: cannot write: {}\n",
                os_error(&error)
            )
        })?;
        file.sync_all().map_err(|error| {
            format!(
                "active-task-evidence migration: {relative}: cannot close: {}\n",
                os_error(&error)
            )
        })?;
        drop(file);
        let mut permissions = fs::metadata(&temporary)
            .map_err(|error| os_error(&error))?
            .permissions();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            permissions.set_mode(0o644);
        }
        fs::set_permissions(&temporary, permissions).map_err(|error| {
            format!(
                "active-task-evidence migration: {relative}: cannot chmod: {}\n",
                os_error(&error)
            )
        })?;
        fs::rename(&temporary, &path).map_err(|error| {
            format!(
                "active-task-evidence migration: {relative}: cannot rename: {}\n",
                os_error(&error)
            )
        })?;
        Ok(())
    })();
    if result.is_err() && path_exists(&temporary) {
        let _ = fs::remove_file(&temporary);
    }
    result
}

fn materialize_migration(
    project_root: &Path,
    base: &Path,
    relative_contract: &str,
    template_rel: &str,
    quiet: bool,
    output: &mut dyn super::Output,
) -> Result<(), String> {
    if !safe_relative_path(template_rel) {
        return Err("active-task-evidence migration: root template path is unsafe\n".to_string());
    }
    let existing = validate_tree(project_root, base, relative_contract, false);
    if !existing.errors.is_empty() {
        return Err(format!(
            "active-task-evidence migration: source preflight failed:\n{}\n",
            existing
                .errors
                .iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    let mut read_errors = Vec::new();
    let contract_read = read_json_object(
        base,
        relative_contract,
        "migration contract",
        131_072,
        &mut read_errors,
    );
    let source_raw = contract_read.as_ref().and_then(|(contract, _)| {
        read_regular(
            base,
            &scalar(field(Some(contract), "current_path")),
            "migration source",
            &mut read_errors,
        )
    });
    let root_raw = read_regular(
        base,
        template_rel,
        "migration root template",
        &mut read_errors,
    );
    let (Some((contract, contract_raw)), Some(source_raw), Some(root_raw)) =
        (contract_read, source_raw, root_raw)
    else {
        return Err(format!(
            "active-task-evidence migration: input read failed:\n{}\n",
            read_errors
                .iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    };
    if !read_errors.is_empty() {
        return Err(format!(
            "active-task-evidence migration: input read failed:\n{}\n",
            read_errors
                .iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    if scalar(field(Some(&contract), "migration_state")) != "source_locked"
        || scalar(field(Some(&contract), "input_state")) != "complete"
    {
        return Err(
            "active-task-evidence migration: contract must be source_locked/complete\n".to_string(),
        );
    }
    if !matches!(
        field(Some(&contract), "migration_metadata"),
        Some(Py::Dict(_))
    ) {
        return Err(
            "active-task-evidence migration: contract lacks migration_metadata\n".to_string(),
        );
    }
    let current_path = scalar(field(Some(&contract), "current_path"));
    let destinations = field(Some(&contract), "destinations");
    let authority_conflict = template_rel == relative_contract
        || template_rel == current_path
        || [
            scalar(field(destinations, "index")),
            scalar(field(destinations, "manifest")),
            scalar(field(destinations, "source_capsule")),
        ]
        .contains(&template_rel.to_string())
        || array(field(destinations, "parts"))
            .iter()
            .any(|part| scalar(field(Some(part), "path")) == template_rel);
    if authority_conflict {
        return Err(
            "active-task-evidence migration: root template must not be a declared authority path\n"
                .to_string(),
        );
    }
    let mut template_errors = Vec::new();
    let root_metrics = validate_migration_root_template(&contract, &root_raw, &mut template_errors);
    if !template_errors.is_empty() {
        return Err(format!(
            "active-task-evidence migration: root template preflight failed:\n{}\n",
            template_errors
                .iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    let (migrated, outputs) =
        build_migration_outputs(&contract, relative_contract, &source_raw, &root_raw);
    let mut write_order = vec![scalar(field(
        field(Some(&migrated), "destinations"),
        "source_capsule",
    ))];
    write_order.extend(
        array(field(field(Some(&migrated), "destinations"), "parts"))
            .iter()
            .map(|part| scalar(field(Some(part), "path"))),
    );
    write_order.extend(route_part_paths(&migrated));
    write_order.extend([
        scalar(field(field(Some(&migrated), "destinations"), "index")),
        scalar(field(field(Some(&migrated), "destinations"), "manifest")),
        relative_contract.to_string(),
        scalar(field(Some(&migrated), "current_path")),
    ]);
    let mut write_error = None;
    for relative in &write_order {
        if let Err(error) = write_raw_atomic(base, relative, &outputs[relative]) {
            write_error = Some(error);
            break;
        }
    }
    let migrated_errors = if write_error.is_none() {
        validate_tree(project_root, base, relative_contract, false).errors
    } else {
        Vec::new()
    };
    if write_error.is_some() || !migrated_errors.is_empty() {
        let mut reason = write_error.unwrap_or_else(|| {
            migrated_errors
                .iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n")
        });
        let rollback = (|| -> Result<(), String> {
            write_raw_atomic(base, &current_path, &source_raw)?;
            write_raw_atomic(base, relative_contract, &contract_raw)?;
            for directory in ["collection_directory", "archive_directory"] {
                let path = absolute(base, &scalar(field(destinations, directory)));
                if path_exists(&path) {
                    fs::remove_dir_all(&path).map_err(|error| os_error(&error))?;
                }
            }
            Ok(())
        })();
        if let Err(error) = rollback {
            reason.push_str(&format!("\nrollback failed: {error}"));
        }
        return Err(format!(
            "active-task-evidence migration: transaction failed and was rolled back:\n{reason}\n"
        ));
    }
    if !quiet {
        let part_count = array(field(field(Some(&migrated), "destinations"), "parts")).len();
        output.out(&format!(
            "active-task-evidence migration: wrote bounded root ({} lines / {} bytes), {part_count} semantic parts, index, manifest, and exact source capsule.\n",
            root_metrics.lines, root_metrics.bytes
        ));
    }
    Ok(())
}

fn materialize_derived(
    project_root: &Path,
    base: &Path,
    relative_contract: &str,
    quiet: bool,
    output: &mut dyn super::Output,
) -> Result<(), String> {
    let preflight = validate_tree(project_root, base, relative_contract, true);
    if !preflight.errors.is_empty() {
        return Err(format!(
            "active-task-evidence writer: preflight failed:\n{}\n",
            preflight
                .errors
                .iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    let mut read_errors = Vec::new();
    let contract = read_json_object(
        base,
        relative_contract,
        "writer contract",
        131_072,
        &mut read_errors,
    )
    .map(|(contract, _)| contract);
    let Some(contract) = contract else {
        return Err(format!(
            "active-task-evidence writer: contract read failed:\n{}\n",
            read_errors.join("\n")
        ));
    };
    if !read_errors.is_empty() {
        return Err(format!(
            "active-task-evidence writer: contract read failed:\n{}\n",
            read_errors.join("\n")
        ));
    }
    if scalar(field(Some(&contract), "migration_state")) != "migrated"
        || scalar(field(Some(&contract), "input_state")) != "complete"
    {
        return Err(
            "active-task-evidence writer: contract must be migrated/complete\n".to_string(),
        );
    }
    let route_paths = route_part_paths(&contract);
    if route_catalog_is_sharded(&contract) && route_paths.is_empty() {
        return Err(
            "active-task-evidence writer: sharded contract derives no route catalog part\n"
                .to_string(),
        );
    }
    let mut outputs = BTreeMap::new();
    outputs.insert(
        scalar(field(field(Some(&contract), "destinations"), "index")),
        render_index(&contract),
    );
    for (position, path) in route_paths.iter().enumerate() {
        outputs.insert(
            path.clone(),
            render_route_part(&contract, position, &route_paths),
        );
    }
    outputs.insert(
        scalar(field(field(Some(&contract), "destinations"), "manifest")),
        pretty_json(&manifest_expected_subset(&contract)),
    );
    let mut scan_errors = Vec::new();
    let stale = unplanned_route_part_paths(base, &contract, &route_paths, &mut scan_errors);
    if !scan_errors.is_empty() {
        return Err(format!(
            "active-task-evidence writer: cannot enumerate route catalog parts:\n{}\n",
            scan_errors
                .iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n")
        ));
    }
    let mut previous = BTreeMap::new();
    for relative in outputs.keys().chain(stale.iter()) {
        let mut ignored = Vec::new();
        previous.insert(
            relative.clone(),
            read_regular(base, relative, "writer preimage", &mut ignored),
        );
    }
    let mut write_error = None;
    for (relative, raw) in &outputs {
        if let Err(error) = write_raw_atomic(base, relative, raw) {
            write_error = Some(error);
            break;
        }
    }
    if write_error.is_none() {
        for relative in &stale {
            if let Err(error) = fs::remove_file(absolute(base, relative)) {
                write_error = Some(format!(
                    "cannot remove stale route catalog part '{relative}': {}\n",
                    os_error(&error)
                ));
                break;
            }
        }
    }
    let post_errors = if write_error.is_none() {
        validate_tree(project_root, base, relative_contract, false).errors
    } else {
        Vec::new()
    };
    if write_error.is_some() || !post_errors.is_empty() {
        let mut reason = write_error.unwrap_or_else(|| {
            post_errors
                .iter()
                .map(|error| format!("- {error}"))
                .collect::<Vec<_>>()
                .join("\n")
        });
        let rollback = (|| -> Result<(), String> {
            for (relative, preimage) in &previous {
                if let Some(raw) = preimage {
                    write_raw_atomic(base, relative, raw)?;
                } else if absolute(base, relative).exists() {
                    fs::remove_file(absolute(base, relative)).map_err(|error| os_error(&error))?;
                }
            }
            Ok(())
        })();
        if let Err(error) = rollback {
            reason.push_str(&format!("\nrollback failed: {error}"));
        }
        return Err(format!(
            "active-task-evidence writer: transaction failed and was rolled back:\n{reason}\n"
        ));
    }
    if !quiet {
        output.out(&format!(
            "active-task-evidence writer: wrote the bounded index ({} routed of {} leaf routes), {} route catalog part(s), and the manifest; removed {} stale part(s).\n",
            landing_leaf_routes(&contract).len(),
            array(field(Some(&contract), "leaf_routes")).len(),
            route_paths.len(),
            stale.len()
        ));
    }
    Ok(())
}

fn write_raw(base: &Path, relative: &str, raw: &[u8]) -> Result<(), String> {
    let path = absolute(base, relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "active-task-evidence self-test: cannot write {relative}: {}\n",
                os_error(&error)
            )
        })?;
    }
    fs::write(&path, raw).map_err(|error| {
        format!(
            "active-task-evidence self-test: cannot write {relative}: {}\n",
            os_error(&error)
        )
    })
}

fn write_json(base: &Path, relative: &str, value: &Py) -> Result<(), String> {
    write_raw(base, relative, &pretty_json(value))
}

fn fixture_source() -> &'static [u8] {
    b"# PROGRAM: fixture\n## Metadata\n- Tree ID: `PROGRAM`\n- Status: `active`\n## Task tree \xe2\x80\x94 legacy\n- ID: `PROGRAM`\n- ID: `.1`\n## Verification Log\n| Date | Leaf | Checks | Result |\n| 2026-08-09 | PROGRAM.1 | fixture | green |\n"
}

fn fixture_root() -> &'static str {
    "# PROGRAM: fixture\n\n## Metadata\n\n- Tree ID: `PROGRAM`\n- Status: `active`\n\n## Task owners\n\n- ID: `PROGRAM`\n- ID: `PROGRAM.1`\n\n## Current Frontier\n\nNo eligible frontier.\n\n## Detailed task evidence\n\n[Complete task evidence](program/INDEX.md)\n\n## Verification Log\n\nVerified.\n\n## Commit Log\n\nCommitted.\n"
}

fn fixture_limit_pair(health: &[(&str, u64)], ceilings: &[(&str, u64)]) -> Py {
    object(vec![
        (
            "health_targets",
            Py::Dict(
                health
                    .iter()
                    .map(|(name, value)| (pyjson::py_str(name), integer(*value)))
                    .collect(),
            ),
        ),
        (
            "enforcement_ceilings",
            Py::Dict(
                ceilings
                    .iter()
                    .map(|(name, value)| (pyjson::py_str(name), integer(*value)))
                    .collect(),
            ),
        ),
        (
            "milestones",
            object(vec![
                ("warning_pct", integer(80)),
                ("rollover_pct", integer(90)),
            ]),
        ),
    ])
}

fn region_bytes(source_raw: &[u8], region: &Py) -> Vec<u8> {
    let lines = split_source_lines(source_raw);
    let start = perl::numeric(field(Some(region), "start_line")) as usize;
    let end = perl::numeric(field(Some(region), "end_line")) as usize;
    lines[start - 1..end].concat()
}

fn fixture_contract(input_state: &str, source_raw: &[u8]) -> Py {
    let source_metrics = metrics(source_raw);
    object(vec![
        ("schema_version", integer(1)),
        ("contract_id", text("fixture-active-task")),
        ("migration_state", text("source_locked")),
        ("input_state", text(input_state)),
        ("current_path", text("docs/tasks/PROGRAM.md")),
        (
            "identity",
            object(vec![
                ("tree_id", text("PROGRAM")),
                ("heading", text("# PROGRAM: fixture")),
                ("active_status_literal", text("- Status: `active`")),
            ]),
        ),
        (
            "current_frontier",
            object(vec![
                ("mode", text("none")),
                ("literal", text("No eligible frontier.")),
            ]),
        ),
        (
            "migration_metadata",
            object(vec![
                ("migrated_on", text("2026-08-09")),
                ("reason", text("fixture migration")),
            ]),
        ),
        (
            "source",
            object(vec![
                ("boundary_commit", text(&"0".repeat(40))),
                ("git_blob", text(&"0".repeat(40))),
                ("sha256", text(&sha256_hex(source_raw))),
                ("metrics", metrics_json(source_metrics)),
            ]),
        ),
        (
            "source_requirements",
            Py::List(vec![
                text("# PROGRAM: fixture"),
                text("- Tree ID: `PROGRAM`"),
                text("## Verification Log"),
            ]),
        ),
        (
            "destinations",
            object(vec![
                ("collection_directory", text("docs/tasks/program")),
                ("index", text("docs/tasks/program/INDEX.md")),
                ("manifest", text("docs/tasks/program/manifest.json")),
                (
                    "parts",
                    Py::List(vec![
                        object(vec![
                            ("part_id", text("foundation")),
                            ("path", text("docs/tasks/program/foundation.md")),
                            ("heading", text("# PROGRAM — foundation")),
                            ("state", text("legacy")),
                        ]),
                        object(vec![
                            ("part_id", text("activity")),
                            ("path", text("docs/tasks/program/activity.md")),
                            ("heading", text("# PROGRAM — activity")),
                            ("state", text("legacy")),
                        ]),
                    ]),
                ),
                ("route_part_prefix", text("docs/tasks/program/routes-")),
                ("archive_directory", text("docs/archive/tasks/program")),
                (
                    "source_capsule",
                    text("docs/archive/tasks/program/source.md"),
                ),
            ]),
        ),
        (
            "regions",
            Py::List(vec![
                object(vec![
                    ("region_id", text("identity")),
                    ("start_line", integer(1)),
                    ("end_line", integer(5)),
                    ("part_id", text("foundation")),
                ]),
                object(vec![
                    ("region_id", text("activity")),
                    ("start_line", integer(6)),
                    ("end_line", integer(7)),
                    ("part_id", text("activity")),
                ]),
                object(vec![
                    ("region_id", text("verification")),
                    ("start_line", integer(8)),
                    ("end_line", integer(10)),
                    ("part_id", text("foundation")),
                ]),
            ]),
        ),
        ("leaf_routes", Py::List(Vec::new())),
        ("route_basis", text("boundary_path_commit_subject_ids")),
        ("route_catalog_state", text("sharded")),
        ("marker_prefix", text("active-task-source-region")),
        (
            "limits",
            object(vec![
                (
                    "root",
                    fixture_limit_pair(
                        &[("lines", 64), ("bytes", 4096), ("line_bytes", 256)],
                        &[("lines", 96), ("bytes", 6144), ("line_bytes", 512)],
                    ),
                ),
                (
                    "index",
                    fixture_limit_pair(
                        &[("lines", 64), ("bytes", 4096), ("line_bytes", 256)],
                        &[("lines", 96), ("bytes", 6144), ("line_bytes", 512)],
                    ),
                ),
                (
                    "parts",
                    fixture_limit_pair(
                        &[
                            ("files", 8),
                            ("lines_each", 64),
                            ("bytes_each", 4096),
                            ("line_bytes_each", 256),
                            ("lines_total", 256),
                            ("bytes_total", 16384),
                        ],
                        &[
                            ("files", 12),
                            ("lines_each", 96),
                            ("bytes_each", 6144),
                            ("line_bytes_each", 512),
                            ("lines_total", 512),
                            ("bytes_total", 32768),
                        ],
                    ),
                ),
                ("route_parts", {
                    let mut limits = fixture_limit_pair(
                        &[
                            ("files", 4),
                            ("lines_each", 32),
                            ("bytes_each", 4096),
                            ("line_bytes_each", 256),
                            ("lines_total", 128),
                            ("bytes_total", 16384),
                        ],
                        &[
                            ("files", 6),
                            ("lines_each", 48),
                            ("bytes_each", 6144),
                            ("line_bytes_each", 384),
                            ("lines_total", 192),
                            ("bytes_total", 24576),
                        ],
                    );
                    set_field(&mut limits, "routes_per_part", integer(8));
                    set_field(&mut limits, "max_parts", integer(4));
                    set_field(&mut limits, "max_unverified_routes", integer(2));
                    limits
                }),
                ("capsule_exact", metrics_json(source_metrics)),
                (
                    "manifest",
                    object(vec![
                        ("bytes", integer(32768)),
                        ("line_bytes", integer(1024)),
                        ("scalar_bytes", integer(512)),
                        ("max_parts", integer(8)),
                        ("max_regions", integer(8)),
                        (
                            "leaf_route_exemption",
                            object(vec![
                                (
                                    "authority",
                                    text(
                                        "docs/decisions/0056-a-sharded-task-trees-leaf-count-is-bounded-by-its-route-catalog.md",
                                    ),
                                ),
                                ("work_unit", text("LIVE-DOCUMENT-PRESSURE-HEADROOM.17")),
                                (
                                    "rationale",
                                    text("The route catalog parts bound the route count."),
                                ),
                            ]),
                        ),
                    ]),
                ),
            ]),
        ),
        (
            "migrated_requirements",
            object(vec![
                (
                    "root_required_literals",
                    Py::List(
                        [
                            "# PROGRAM: fixture",
                            "- Tree ID: `PROGRAM`",
                            "- Status: `active`",
                            "- ID: `PROGRAM`",
                            "- ID: `PROGRAM.1`",
                            "## Current Frontier",
                            "No eligible frontier.",
                            "## Detailed task evidence",
                            "## Verification Log",
                            "## Commit Log",
                        ]
                        .into_iter()
                        .map(text)
                        .collect(),
                    ),
                ),
                (
                    "root_forbidden_literals",
                    Py::List(vec![text("- Status: `done`")]),
                ),
                (
                    "index_required_literals",
                    Py::List(
                        [
                            "# PROGRAM task-evidence index",
                            "## Semantic parts",
                            "## Open leaf routes",
                            "## Complete route catalog",
                            "## Exact provenance",
                            "## Verification",
                        ]
                        .into_iter()
                        .map(text)
                        .collect(),
                    ),
                ),
            ]),
        ),
        (
            "verifier",
            text("perl scripts/check_active_task_evidence.pl --check"),
        ),
    ])
}

fn complete_fixture_inputs(contract: &mut Py, source_raw: &[u8]) {
    set_field(contract, "input_state", text("complete"));
    if let Some(regions) = nested_field_mut(contract, &["regions"])
        && let Py::List(regions) = regions
    {
        for region in regions {
            let raw = region_bytes(source_raw, region);
            set_field(region, "sha256", text(&sha256_hex(&raw)));
            set_field(region, "metrics", metrics_json(metrics(&raw)));
        }
    }
    set_field(
        contract,
        "leaf_routes",
        Py::List(vec![
            object(vec![
                ("leaf_id", text("PROGRAM.1")),
                ("part_id", text("activity")),
                ("origin", text("legacy")),
                ("source_literal", text(".1")),
                ("lifecycle", text("closed")),
            ]),
            object(vec![
                ("leaf_id", text("PROGRAM")),
                ("part_id", text("foundation")),
                ("origin", text("structural")),
                ("source_literal", text("PROGRAM")),
                ("lifecycle", text("open")),
            ]),
        ]),
    );
}

fn fixture_part_raw(contract: &Py, source_raw: &[u8], part: &Py) -> Vec<u8> {
    let part_id = scalar(field(Some(part), "part_id"));
    let mut raw = format!(
        "{}\n\n- Part ID: `{part_id}`\n- State: `{}`\n\n",
        scalar(field(Some(part), "heading")),
        scalar(field(Some(part), "state"))
    )
    .into_bytes();
    for region in array(field(Some(contract), "regions")) {
        if scalar(field(Some(region), "part_id")) != part_id {
            continue;
        }
        let id = scalar(field(Some(region), "region_id"));
        raw.extend_from_slice(
            format!(
                "<!-- {}:{id}:start -->\n",
                scalar(field(Some(contract), "marker_prefix"))
            )
            .as_bytes(),
        );
        raw.extend_from_slice(&region_bytes(source_raw, region));
        raw.extend_from_slice(
            format!(
                "<!-- {}:{id}:end -->\n\n",
                scalar(field(Some(contract), "marker_prefix"))
            )
            .as_bytes(),
        );
    }
    raw
}

fn write_fixture_derived(base: &Path, contract: &Py) -> Result<(), String> {
    write_raw(
        base,
        &scalar(field(field(Some(contract), "destinations"), "index")),
        &render_index(contract),
    )?;
    let route_paths = route_part_paths(contract);
    for (position, path) in route_paths.iter().enumerate() {
        write_raw(
            base,
            path,
            &render_route_part(contract, position, &route_paths),
        )?;
    }
    write_raw(
        base,
        &scalar(field(field(Some(contract), "destinations"), "manifest")),
        &pretty_json(&manifest_expected_subset(contract)),
    )
}

fn convert_fixture_to_inline(base: &Path, contract: &mut Py) -> Result<(), String> {
    set_field(contract, "route_catalog_state", text("inline"));
    if let Some(value) = nested_field_mut(contract, &["destinations"]) {
        remove_field(value, "route_part_prefix");
    }
    if let Some(value) = nested_field_mut(contract, &["limits"]) {
        remove_field(value, "route_parts");
    }
    if let Some(value) = nested_field_mut(contract, &["limits", "manifest"]) {
        remove_field(value, "leaf_route_exemption");
        set_field(value, "max_leaf_routes", integer(16));
    }
    if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
        for route in routes {
            remove_field(route, "lifecycle");
        }
    }
    set_field(contract, "migrated_requirements", {
        let mut requirements = field(Some(contract), "migrated_requirements")
            .cloned()
            .unwrap_or_else(|| object(Vec::new()));
        set_field(
            &mut requirements,
            "index_required_literals",
            Py::List(
                [
                    "# PROGRAM task-evidence index",
                    "## Semantic parts",
                    "## Primary leaf routes",
                    "## Exact provenance",
                    "## Verification",
                ]
                .into_iter()
                .map(text)
                .collect(),
            ),
        );
        requirements
    });
    let stale = absolute(base, "docs/tasks/program/routes-0001.md");
    if stale.exists() {
        fs::remove_file(stale).map_err(|error| os_error(&error))?;
    }
    write_fixture_derived(base, contract)
}

fn git_fixture(base: &Path, arguments: &[&str], failure: &str) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(base)
        .args(arguments)
        .status()
        .map_err(|error| os_error(&error))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("active-task-evidence self-test: {failure}\n"))
    }
}

fn git_fixture_output(base: &Path, arguments: &[&str], failure: &str) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(base)
        .args(arguments)
        .output()
        .map_err(|error| os_error(&error))?;
    if !output.status.success() {
        return Err(format!("active-task-evidence self-test: {failure}\n"));
    }
    Ok(trim_newlines(&output.stdout))
}

fn init_fixture_git(base: &Path, contract: &mut Py, source_raw: &[u8]) -> Result<(), String> {
    let current = scalar(field(Some(contract), "current_path"));
    write_raw(base, &current, source_raw)?;
    git_fixture(base, &["init", "-q"], "git init failed")?;
    git_fixture(base, &["add", "--", &current], "git add failed")?;
    git_fixture(
        base,
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-qm",
            "PROGRAM.1 — fixture source",
        ],
        "git commit failed",
    )?;
    let commit = git_fixture_output(base, &["rev-parse", "HEAD"], "git rev-parse failed")?;
    let blob = git_fixture_output(
        base,
        &["rev-parse", &format!("HEAD:{current}")],
        "git blob lookup failed",
    )?;
    if let Some(source) = nested_field_mut(contract, &["source"]) {
        set_field(source, "boundary_commit", text(&commit));
        set_field(source, "git_blob", text(&blob));
    }
    Ok(())
}

type Mutator = fn(&Path, &mut Py, &[u8]) -> Result<(), String>;

fn seed_fixture(
    base: &Path,
    state: &str,
    input_state: &str,
    mutator: Option<Mutator>,
    contract_rel: &str,
) -> Result<Py, String> {
    let source_raw = fixture_source();
    let mut contract = fixture_contract(input_state, source_raw);
    set_field(
        &mut contract,
        "verifier",
        text(&verifier_command_for(contract_rel)),
    );
    init_fixture_git(base, &mut contract, source_raw)?;
    if input_state == "complete" {
        complete_fixture_inputs(&mut contract, source_raw);
    }
    if state == "migrated" {
        set_field(&mut contract, "migration_state", text("migrated"));
        write_raw(
            base,
            &scalar(field(
                field(Some(&contract), "destinations"),
                "source_capsule",
            )),
            source_raw,
        )?;
        write_raw(
            base,
            &scalar(field(Some(&contract), "current_path")),
            fixture_root().as_bytes(),
        )?;
        let parts = array(field(field(Some(&contract), "destinations"), "parts")).to_vec();
        for part in parts {
            let raw = fixture_part_raw(&contract, source_raw, &part);
            write_raw(base, &scalar(field(Some(&part), "path")), &raw)?;
            if let Some(Py::List(parts)) =
                nested_field_mut(&mut contract, &["destinations", "parts"])
            {
                for candidate in parts.iter_mut().filter(|candidate| {
                    scalar(field(Some(candidate), "part_id"))
                        == scalar(field(Some(&part), "part_id"))
                }) {
                    set_field(candidate, "sha256", text(&sha256_hex(&raw)));
                    set_field(candidate, "metrics", metrics_json(metrics(&raw)));
                }
            }
        }
        write_fixture_derived(base, &contract)?;
    }
    if let Some(mutator) = mutator {
        mutator(base, &mut contract, source_raw)?;
    }
    write_json(base, contract_rel, &contract)?;
    Ok(contract)
}

fn append_fixture_activity(
    base: &Path,
    contract: &mut Py,
    node_status: &str,
    inline_declaration: bool,
) -> Result<(), String> {
    let part = object(vec![
        ("part_id", text("activity-02")),
        ("path", text("docs/tasks/program/activity-02.md")),
        ("heading", text("# PROGRAM — activity 02")),
        ("state", text("active")),
    ]);
    let node = if inline_declaration {
        format!("- ID: `PROGRAM.2` · Status: `{node_status}` · Goal: new bounded activity\n")
    } else {
        format!("- ID: `PROGRAM.2`\n  State: `{node_status}`\n  Goal: new bounded activity\n")
    };
    let part_raw = format!(
        "{}\n\n- Part ID: `activity-02`\n- State: `active`\n\n## PROGRAM.2\n\n{node}",
        scalar(field(Some(&part), "heading"))
    )
    .into_bytes();
    if let Some(Py::List(parts)) = nested_field_mut(contract, &["destinations", "parts"]) {
        let mut pinned = part.clone();
        set_field(&mut pinned, "sha256", text(&sha256_hex(&part_raw)));
        set_field(&mut pinned, "metrics", metrics_json(metrics(&part_raw)));
        parts.push(pinned);
    }
    let closed = matches!(node_status, "done" | "superseded");
    if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
        routes.push(object(vec![
            ("leaf_id", text("PROGRAM.2")),
            ("part_id", text("activity-02")),
            ("origin", text("post_migration")),
            ("lifecycle", text(if closed { "closed" } else { "open" })),
        ]));
    }
    set_field(
        contract,
        "current_frontier",
        object(vec![
            ("mode", text("eligible")),
            ("literal", text("Eligible frontier: PROGRAM.2.")),
            ("leaf_id", text("PROGRAM.2")),
            ("part_id", text("activity-02")),
        ]),
    );
    if let Some(Py::List(literals)) = nested_field_mut(
        contract,
        &["migrated_requirements", "root_required_literals"],
    ) {
        for literal in literals.iter_mut() {
            if scalar(Some(literal)) == "No eligible frontier." {
                *literal = text("Eligible frontier: PROGRAM.2.");
            }
        }
        if !closed {
            literals.push(text("- ID: `PROGRAM.2`"));
        }
    }
    let mut root = fixture_root().to_string();
    if !closed {
        root = root.replace(
            "- ID: `PROGRAM.1`\n",
            "- ID: `PROGRAM.1`\n- ID: `PROGRAM.2`\n",
        );
    }
    root = root.replace("No eligible frontier.", "Eligible frontier: PROGRAM.2.");
    write_raw(
        base,
        &scalar(field(Some(contract), "current_path")),
        root.as_bytes(),
    )?;
    write_raw(base, "docs/tasks/program/activity-02.md", &part_raw)?;
    write_fixture_derived(base, contract)
}

fn set_fixture_root_owners(base: &Path, contract: &mut Py, owners: &[&str]) -> Result<(), String> {
    if let Some(Py::List(literals)) = nested_field_mut(
        contract,
        &["migrated_requirements", "root_required_literals"],
    ) {
        literals.retain(|literal| !scalar(Some(literal)).starts_with("- ID: `"));
        literals.extend(owners.iter().map(|owner| text(&format!("- ID: `{owner}`"))));
    }
    let registry = owners
        .iter()
        .map(|owner| format!("- ID: `{owner}`\n"))
        .collect::<String>();
    let mut root = fixture_root().to_string();
    root = root.replace("- ID: `PROGRAM`\n- ID: `PROGRAM.1`\n", &registry);
    root = root.replace(
        "No eligible frontier.",
        &scalar(field(field(Some(contract), "current_frontier"), "literal")),
    );
    write_raw(
        base,
        &scalar(field(Some(contract), "current_path")),
        root.as_bytes(),
    )
}

fn seal_fixture_activity_part(
    base: &Path,
    contract: &mut Py,
    source_raw: &[u8],
) -> Result<(), String> {
    let part = array(field(field(Some(contract), "destinations"), "parts"))[1].clone();
    let mut sealed = part.clone();
    set_field(&mut sealed, "state", text("sealed"));
    let raw = fixture_part_raw(contract, source_raw, &sealed);
    let path = scalar(field(Some(&sealed), "path"));
    write_raw(base, &path, &raw)?;
    if let Some(Py::List(parts)) = nested_field_mut(contract, &["destinations", "parts"]) {
        set_field(&mut parts[1], "state", text("sealed"));
        set_field(&mut parts[1], "sha256", text(&sha256_hex(&raw)));
        set_field(&mut parts[1], "metrics", metrics_json(metrics(&raw)));
    }
    git_fixture(base, &["add", "--", &path], "sealed part git add failed")?;
    git_fixture(
        base,
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-qm",
            "PROGRAM.1 — seal fixture part",
        ],
        "sealed part git commit failed",
    )?;
    let commit = git_fixture_output(base, &["rev-parse", "HEAD"], "sealed commit lookup failed")?;
    let blob = git_fixture_output(
        base,
        &["rev-parse", &format!("HEAD:{path}")],
        "sealed blob lookup failed",
    )?;
    if let Some(Py::List(parts)) = nested_field_mut(contract, &["destinations", "parts"]) {
        set_field(&mut parts[1], "sealed_commit", text(&commit));
        set_field(&mut parts[1], "git_blob", text(&blob));
    }
    write_raw(
        base,
        &scalar(field(field(Some(contract), "destinations"), "manifest")),
        &pretty_json(&manifest_expected_subset(contract)),
    )
}

fn archive_fixture_activity_part(base: &Path, contract: &mut Py) -> Result<(), String> {
    let part = array(field(field(Some(contract), "destinations"), "parts"))[1].clone();
    let live = scalar(field(Some(&part), "path"));
    git_fixture(base, &["add", "--", &live], "archived part git add failed")?;
    git_fixture(
        base,
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "-c",
            "commit.gpgsign=false",
            "commit",
            "-qm",
            "PROGRAM.1 — the live part before archiving",
        ],
        "archived part git commit failed",
    )?;
    let commit = git_fixture_output(
        base,
        &["rev-parse", "HEAD"],
        "archived commit lookup failed",
    )?;
    let blob = git_fixture_output(
        base,
        &["rev-parse", &format!("HEAD:{live}")],
        "archived blob lookup failed",
    )?;
    let raw = fs::read(absolute(base, &live)).map_err(|error| os_error(&error))?;
    let archived = format!(
        "{}/activity.md",
        scalar(field(
            field(Some(contract), "destinations"),
            "archive_directory"
        ))
    );
    write_raw(base, &archived, &raw)?;
    fs::remove_file(absolute(base, &live)).map_err(|error| os_error(&error))?;
    if let Some(Py::List(parts)) = nested_field_mut(contract, &["destinations", "parts"]) {
        set_field(&mut parts[1], "state", text("archived"));
        set_field(&mut parts[1], "path", text(&archived));
        set_field(&mut parts[1], "archived_from", text(&live));
        set_field(&mut parts[1], "sealed_commit", text(&commit));
        set_field(&mut parts[1], "git_blob", text(&blob));
    }
    write_fixture_derived(base, contract)
}

fn replace_file(base: &Path, relative: &str, from: &str, to: &str) -> Result<(), String> {
    let path = absolute(base, relative);
    let raw = fs::read(&path).map_err(|error| os_error(&error))?;
    let text = String::from_utf8(raw).map_err(|error| error.to_string())?;
    write_raw(base, relative, text.replace(from, to).as_bytes())
}

struct FixtureCleanup(PathBuf);

impl Drop for FixtureCleanup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn prepare_fixture(path: &Path) -> Result<FixtureCleanup, String> {
    if path.exists() {
        fs::remove_dir_all(path).map_err(|error| os_error(&error))?;
    }
    fs::create_dir_all(path).map_err(|error| os_error(&error))?;
    Ok(FixtureCleanup(path.to_path_buf()))
}

fn set_nested(contract: &mut Py, path: &[&str], value: Py) {
    let Some((key, parent)) = path.split_last() else {
        return;
    };
    if let Some(parent) = nested_field_mut(contract, parent) {
        set_field(parent, key, value);
    }
}

fn remove_nested(contract: &mut Py, path: &[&str], key: &str) {
    if let Some(target) = nested_field_mut(contract, path) {
        remove_field(target, key);
    }
}

struct Case {
    name: &'static str,
    state: &'static str,
    input_state: &'static str,
    mutator: Option<Mutator>,
    expected: &'static str,
    contract_rel: &'static str,
}

fn cases() -> Vec<Case> {
    let default = DEFAULT_CONTRACT;
    let alternate = "doctrine/live_document_size/fixture_task_evidence.json";
    vec![
        Case {
            name: "source-locked topology positive",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: None,
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "non-default contract verifier positive",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: None,
            expected: "",
            contract_rel: alternate,
        },
        Case {
            name: "non-default contract verifier mismatch",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_field(
                    contract,
                    "verifier",
                    text("perl scripts/check_active_task_evidence.pl --check"),
                );
                Ok(())
            }),
            expected: "contract verifier must be",
            contract_rel: alternate,
        },
        Case {
            name: "source-locked complete positive",
            state: "source_locked",
            input_state: "complete",
            mutator: None,
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "unknown contract field",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_field(contract, "unknown", integer(1));
                Ok(())
            }),
            expected: "unknown field",
            contract_rel: default,
        },
        Case {
            name: "unsafe current path",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_field(contract, "current_path", text("../escape.md"));
                Ok(())
            }),
            expected: "current_path is unsafe",
            contract_rel: default,
        },
        Case {
            name: "working source drift",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|base, contract, source| {
                let path = scalar(field(Some(contract), "current_path"));
                write_raw(base, &path, &[source, b"changed\n"].concat())
            }),
            expected: "source authority",
            contract_rel: default,
        },
        Case {
            name: "boundary commit drift",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["source", "boundary_commit"],
                    text(&"f".repeat(40)),
                );
                Ok(())
            }),
            expected: "boundary commit lookup",
            contract_rel: default,
        },
        Case {
            name: "Git blob drift",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_nested(contract, &["source", "git_blob"], text(&"f".repeat(40)));
                Ok(())
            }),
            expected: "boundary blob",
            contract_rel: default,
        },
        Case {
            name: "region gap",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(regions)) = nested_field_mut(contract, &["regions"]) {
                    let start = perl::numeric(field(Some(&regions[1]), "start_line")) as u64;
                    set_field(&mut regions[1], "start_line", integer(start + 1));
                }
                Ok(())
            }),
            expected: "not contiguous",
            contract_rel: default,
        },
        Case {
            name: "region unknown part",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(regions)) = nested_field_mut(contract, &["regions"]) {
                    set_field(&mut regions[1], "part_id", text("missing"));
                }
                Ok(())
            }),
            expected: "unknown part",
            contract_rel: default,
        },
        Case {
            name: "duplicate part path",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                let path = scalar(field(
                    Some(&array(field(field(Some(contract), "destinations"), "parts"))[0]),
                    "path",
                ));
                if let Some(Py::List(parts)) =
                    nested_field_mut(contract, &["destinations", "parts"])
                {
                    set_field(&mut parts[1], "path", text(&path));
                }
                Ok(())
            }),
            expected: "duplicate part path",
            contract_rel: default,
        },
        Case {
            name: "invalid limit pair",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "root", "health_targets", "bytes"],
                    integer(7000),
                );
                Ok(())
            }),
            expected: "health bytes exceeds",
            contract_rel: default,
        },
        Case {
            name: "portable ceiling inflation",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "parts", "enforcement_ceilings", "files"],
                    integer(25),
                );
                Ok(())
            }),
            expected: "exceeds portable cap 24",
            contract_rel: default,
        },
        Case {
            name: "exact wide part ceiling positive",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "parts", "enforcement_ceilings", "line_bytes_each"],
                    integer(6400),
                );
                Ok(())
            }),
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "exact wide part ceiling inflation",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "parts", "enforcement_ceilings", "line_bytes_each"],
                    integer(6401),
                );
                Ok(())
            }),
            expected: "exceeds portable cap 6400",
            contract_rel: default,
        },
        Case {
            name: "premature collection directory",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|base, contract, _| {
                fs::create_dir_all(absolute(
                    base,
                    &scalar(field(
                        field(Some(contract), "destinations"),
                        "collection_directory",
                    )),
                ))
                .map_err(|error| os_error(&error))
            }),
            expected: "premature collection_directory",
            contract_rel: default,
        },
        Case {
            name: "premature archive directory",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|base, contract, _| {
                fs::create_dir_all(absolute(
                    base,
                    &scalar(field(
                        field(Some(contract), "destinations"),
                        "archive_directory",
                    )),
                ))
                .map_err(|error| os_error(&error))
            }),
            expected: "premature archive_directory",
            contract_rel: default,
        },
        Case {
            name: "topology route leakage",
            state: "source_locked",
            input_state: "topology_declared",
            mutator: Some(|_, contract, _| {
                set_field(
                    contract,
                    "leaf_routes",
                    Py::List(vec![object(vec![
                        ("leaf_id", text("PROGRAM.1")),
                        ("part_id", text("activity")),
                        ("origin", text("legacy")),
                    ])]),
                );
                Ok(())
            }),
            expected: "must not contain leaf routes",
            contract_rel: default,
        },
        Case {
            name: "complete region digest missing",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(regions)) = nested_field_mut(contract, &["regions"]) {
                    remove_field(&mut regions[0], "sha256");
                }
                Ok(())
            }),
            expected: "lacks non-empty scalar 'sha256'",
            contract_rel: default,
        },
        Case {
            name: "complete route omission",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                set_field(contract, "leaf_routes", Py::List(Vec::new()));
                Ok(())
            }),
            expected: "must contain leaf routes|omit boundary-history",
            contract_rel: default,
        },
        Case {
            name: "complete route source literal missing",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    remove_field(&mut routes[0], "source_literal");
                }
                Ok(())
            }),
            expected: "lacks non-empty scalar 'source_literal'",
            contract_rel: default,
        },
        Case {
            name: "complete route source literal invalid",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    set_field(&mut routes[0], "source_literal", text("PROGRAM"));
                }
                Ok(())
            }),
            expected: "not its full or tree-relative id",
            contract_rel: default,
        },
        Case {
            name: "complete route source literal absent",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    set_field(&mut routes[0], "source_literal", text("PROGRAM.1"));
                }
                Ok(())
            }),
            expected: "source literal 'PROGRAM.1' is absent",
            contract_rel: default,
        },
        Case {
            name: "complete structural owner route omitted",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    routes.pop();
                }
                Ok(())
            }),
            expected: "omit declared root owner 'PROGRAM'",
            contract_rel: default,
        },
        Case {
            name: "complete undeclared structural route",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    routes.push(object(vec![
                        ("leaf_id", text("PROGRAM.9")),
                        ("part_id", text("foundation")),
                        ("origin", text("structural")),
                        ("source_literal", text("PROGRAM")),
                        ("lifecycle", text("open")),
                    ]));
                }
                Ok(())
            }),
            expected: "leaf route 'PROGRAM.9' is absent from the declared root owner registry",
            contract_rel: default,
        },
        Case {
            name: "structural route source literal missing",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    remove_field(&mut routes[1], "source_literal");
                }
                Ok(())
            }),
            expected: "structural contract leaf route lacks non-empty scalar 'source_literal'",
            contract_rel: default,
        },
        Case {
            name: "structural route source literal absent",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    set_field(&mut routes[1], "leaf_id", text("PROGRAM.9"));
                    set_field(&mut routes[1], "source_literal", text(".9"));
                }
                Ok(())
            }),
            expected: "structural leaf route 'PROGRAM.9' source literal '.9' is absent",
            contract_rel: default,
        },
        Case {
            name: "premature post-migration route",
            state: "source_locked",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    set_field(&mut routes[0], "origin", text("post_migration"));
                }
                Ok(())
            }),
            expected: "cannot declare post-migration",
            contract_rel: default,
        },
        Case {
            name: "migrated positive",
            state: "migrated",
            input_state: "complete",
            mutator: None,
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "capsule mutation",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, source| {
                let path = scalar(field(
                    field(Some(contract), "destinations"),
                    "source_capsule",
                ));
                write_raw(base, &path, &[source, b"changed\n"].concat())
            }),
            expected: "source authority",
            contract_rel: default,
        },
        Case {
            name: "root owner declaration missing",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                replace_file(
                    base,
                    &scalar(field(Some(contract), "current_path")),
                    "- ID: `PROGRAM.1`\n",
                    "",
                )
            }),
            expected: "lacks required literal '- ID: `PROGRAM.1`'",
            contract_rel: default,
        },
        Case {
            name: "root frontier missing",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                replace_file(
                    base,
                    &scalar(field(Some(contract), "current_path")),
                    "No eligible frontier.",
                    "Frontier unknown.",
                )
            }),
            expected: "frontier",
            contract_rel: default,
        },
        Case {
            name: "root mandatory rollover",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "root", "health_targets", "lines"],
                    integer(20),
                );
                Ok(())
            }),
            expected: "mandatory rollover",
            contract_rel: default,
        },
        Case {
            name: "index part route missing",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                replace_file(
                    base,
                    &scalar(field(field(Some(contract), "destinations"), "index")),
                    "- [Foundation](foundation.md)\n",
                    "",
                )
            }),
            expected: "links 'docs/tasks/program/foundation.md'",
            contract_rel: default,
        },
        Case {
            name: "index leaf misroute",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                replace_file(
                    base,
                    &scalar(field(field(Some(contract), "destinations"), "index")),
                    "[Foundation](foundation.md) |",
                    "[Activity](activity.md) |",
                )
            }),
            expected: "leaf 'PROGRAM' routes",
            contract_rel: default,
        },
        Case {
            name: "index derived drift",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                replace_file(
                    base,
                    &scalar(field(field(Some(contract), "destinations"), "index")),
                    "measures work in flight",
                    "measures nothing in particular",
                )
            }),
            expected: "index differs from its derived form",
            contract_rel: default,
        },
        Case {
            name: "index carries a closed leaf",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                replace_file(
                    base,
                    &scalar(field(field(Some(contract), "destinations"), "index")),
                    "| `PROGRAM` | [Foundation](foundation.md) |\n",
                    "| `PROGRAM` | [Foundation](foundation.md) |\n| `PROGRAM.1` | [Activity](activity.md) |\n",
                )
            }),
            expected: "must not route closed leaf 'PROGRAM.1'",
            contract_rel: default,
        },
        Case {
            name: "route catalog drift",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                let path = route_part_paths(contract)[0].clone();
                replace_file(base, &path, "`closed`", "`open`")
            }),
            expected: "differs from its derived form",
            contract_rel: default,
        },
        Case {
            name: "unplanned route catalog part",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, _, _| {
                write_raw(base, "docs/tasks/program/routes-0002.md", b"# stray\n")
            }),
            expected: "unplanned route catalog file 'docs/tasks/program/routes-0002.md'",
            contract_rel: default,
        },
        Case {
            name: "route catalog capacity below route count",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "route_parts", "max_parts"],
                    integer(1),
                );
                set_nested(
                    contract,
                    &["limits", "route_parts", "routes_per_part"],
                    integer(1),
                );
                Ok(())
            }),
            expected: "route catalog capacity is below the declared leaf route count",
            contract_rel: default,
        },
        Case {
            name: "route part health admits no full part",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "route_parts", "health_targets", "lines_each"],
                    integer(20),
                );
                Ok(())
            }),
            expected: "admits no full part below its warning band",
            contract_rel: default,
        },
        Case {
            name: "leaf route lifecycle missing",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    remove_field(&mut routes[0], "lifecycle");
                }
                Ok(())
            }),
            expected: "lacks non-empty scalar 'lifecycle'",
            contract_rel: default,
        },
        Case {
            name: "leaf route lifecycle invalid",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    set_field(&mut routes[0], "lifecycle", text("retired"));
                }
                Ok(())
            }),
            expected: "has invalid lifecycle 'retired'",
            contract_rel: default,
        },
        Case {
            name: "uncorroborated routes exceed their ratchet",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "route_parts", "max_unverified_routes"],
                    integer(1),
                );
                Ok(())
            }),
            expected: "above the declared max_unverified_routes 1",
            contract_rel: default,
        },
        Case {
            name: "inline route catalog positive",
            state: "migrated",
            input_state: "complete",
            mutator: Some(convert_inline_mutator),
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "inline route catalog with a lifecycle",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, source| {
                convert_fixture_to_inline(base, contract)?;
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    set_field(&mut routes[0], "lifecycle", text("closed"));
                }
                let _ = source;
                Ok(())
            }),
            expected: "must not declare a lifecycle",
            contract_rel: default,
        },
        Case {
            name: "inline route catalog with a route prefix",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, source| {
                convert_fixture_to_inline(base, contract)?;
                set_nested(
                    contract,
                    &["destinations", "route_part_prefix"],
                    text("docs/tasks/program/routes-"),
                );
                let _ = source;
                Ok(())
            }),
            expected: "must not declare a route_part_prefix",
            contract_rel: default,
        },
        Case {
            name: "sharded manifest carries a closed route",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                let mut manifest = manifest_expected_subset(contract);
                set_field(
                    &mut manifest,
                    "leaf_routes",
                    field(Some(contract), "leaf_routes")
                        .cloned()
                        .unwrap_or(Py::None),
                );
                write_raw(
                    base,
                    &scalar(field(field(Some(contract), "destinations"), "manifest")),
                    &pretty_json(&manifest),
                )
            }),
            expected: "manifest field 'leaf_routes' disagrees",
            contract_rel: default,
        },
        Case {
            name: "sharded manifest route count",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "manifest", "max_leaf_routes"],
                    integer(16),
                );
                Ok(())
            }),
            expected: "must not declare manifest max_leaf_routes",
            contract_rel: default,
        },
        Case {
            name: "sharded route exemption missing",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                remove_nested(contract, &["limits", "manifest"], "leaf_route_exemption");
                Ok(())
            }),
            expected: "must declare manifest leaf_route_exemption",
            contract_rel: default,
        },
        Case {
            name: "sharded route exemption untracked authority",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                set_nested(
                    contract,
                    &["limits", "manifest", "leaf_route_exemption", "authority"],
                    text("docs/decisions/9999-missing.md"),
                );
                Ok(())
            }),
            expected: "is not a tracked decision record",
            contract_rel: default,
        },
        Case {
            name: "sharded route exemption without rationale",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                remove_nested(
                    contract,
                    &["limits", "manifest", "leaf_route_exemption"],
                    "rationale",
                );
                Ok(())
            }),
            expected: "leaf_route_exemption lacks non-empty scalar 'rationale'",
            contract_rel: default,
        },
        Case {
            name: "inline route exemption",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, source| {
                convert_fixture_to_inline(base, contract)?;
                set_nested(
                    contract,
                    &["limits", "manifest", "leaf_route_exemption"],
                    object(vec![
                        (
                            "authority",
                            text(
                                "docs/decisions/0045-task-plane-cardinality-is-removed-behind-a-declared-exemption.md",
                            ),
                        ),
                        ("work_unit", text("X")),
                        ("rationale", text("Y")),
                    ]),
                );
                let _ = source;
                Ok(())
            }),
            expected: "inline contract must not declare leaf_route_exemption",
            contract_rel: default,
        },
        Case {
            name: "inline route count missing",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, source| {
                convert_fixture_to_inline(base, contract)?;
                remove_nested(contract, &["limits", "manifest"], "max_leaf_routes");
                let _ = source;
                Ok(())
            }),
            expected: "max_leaf_routes",
            contract_rel: default,
        },
        Case {
            name: "inline route count exceeded",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, source| {
                convert_fixture_to_inline(base, contract)?;
                set_nested(
                    contract,
                    &["limits", "manifest", "max_leaf_routes"],
                    integer(1),
                );
                let _ = source;
                Ok(())
            }),
            expected: "leaf route count exceeds max_leaf_routes",
            contract_rel: default,
        },
        Case {
            name: "unknown route catalog state",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                set_field(contract, "route_catalog_state", text("partitioned"));
                Ok(())
            }),
            expected: "invalid route_catalog_state 'partitioned'",
            contract_rel: default,
        },
        Case {
            name: "post-migration closed append positive",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                append_fixture_activity(base, contract, "done", false)
            }),
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "leaf route lifecycle disagrees with its primary part",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                append_fixture_activity(base, contract, "done", false)?;
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    let last = routes.len() - 1;
                    set_field(&mut routes[last], "lifecycle", text("open"));
                }
                write_fixture_derived(base, contract)
            }),
            expected: "declares lifecycle 'open' but its primary part records status 'done'",
            contract_rel: default,
        },
        Case {
            name: "post-migration inline-declared append positive",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                append_fixture_activity(base, contract, "done", true)
            }),
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "inline leaf route lifecycle disagrees with its primary part",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                append_fixture_activity(base, contract, "done", true)?;
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    let last = routes.len() - 1;
                    set_field(&mut routes[last], "lifecycle", text("open"));
                }
                write_fixture_derived(base, contract)
            }),
            expected: "declares lifecycle 'open' but its primary part records status 'done'",
            contract_rel: default,
        },
        Case {
            name: "part payload mutation",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                let path = scalar(field(
                    Some(&array(field(field(Some(contract), "destinations"), "parts"))[1]),
                    "path",
                ));
                replace_file(base, &path, "- ID: `.1`", "- ID: `.2`")
            }),
            expected: "semantic part|payload differs",
            contract_rel: default,
        },
        Case {
            name: "manifest identity drift",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                let mut manifest = manifest_expected_subset(contract);
                set_nested(&mut manifest, &["source", "sha256"], text(&"f".repeat(64)));
                write_raw(
                    base,
                    &scalar(field(field(Some(contract), "destinations"), "manifest")),
                    &pretty_json(&manifest),
                )
            }),
            expected: "manifest field 'source' disagrees",
            contract_rel: default,
        },
        Case {
            name: "manifest scalar overflow",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                let mut manifest = manifest_expected_subset(contract);
                set_field(&mut manifest, "reason", text(&"x".repeat(600)));
                write_raw(
                    base,
                    &scalar(field(field(Some(contract), "destinations"), "manifest")),
                    &pretty_json(&manifest),
                )
            }),
            expected: "scalar above 512 bytes",
            contract_rel: default,
        },
        Case {
            name: "post-migration active append positive",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                append_fixture_activity(base, contract, "pending", false)
            }),
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "closed recorded leaf left in the root owner registry",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                append_fixture_activity(base, contract, "done", false)?;
                set_fixture_root_owners(base, contract, &["PROGRAM", "PROGRAM.1", "PROGRAM.2"])
            }),
            expected: "closed leaf route 'PROGRAM.2' is still declared in the root owner registry",
            contract_rel: default,
        },
        Case {
            name: "closed unrecorded leaf dropped from the root owner registry",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                set_fixture_root_owners(base, contract, &["PROGRAM"])
            }),
            expected: "closed leaf route 'PROGRAM.1' has no status record in its primary part",
            contract_rel: default,
        },
        Case {
            name: "open leaf absent from the root owner registry",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                append_fixture_activity(base, contract, "pending", false)?;
                set_fixture_root_owners(base, contract, &["PROGRAM", "PROGRAM.1"])
            }),
            expected: "complete leaf route 'PROGRAM.2' is absent from the declared root owner registry",
            contract_rel: default,
        },
        Case {
            name: "sealed part positive",
            state: "migrated",
            input_state: "complete",
            mutator: Some(seal_fixture_mutator),
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "sealed part mutation",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, source| {
                seal_fixture_activity_part(base, contract, source)?;
                let path = scalar(field(
                    Some(&array(field(field(Some(contract), "destinations"), "parts"))[1]),
                    "path",
                ));
                let mut raw = fs::read(absolute(base, &path)).map_err(|error| os_error(&error))?;
                raw.extend_from_slice(b"changed\n");
                write_raw(base, &path, &raw)
            }),
            expected: "differs from its sealing commit",
            contract_rel: default,
        },
        Case {
            name: "archived part positive",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| archive_fixture_activity_part(base, contract)),
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "an archived part leaves the live total",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                archive_fixture_activity_part(base, contract)?;
                set_nested(
                    contract,
                    &["limits", "parts", "health_targets", "files"],
                    integer(2),
                );
                set_nested(
                    contract,
                    &["limits", "parts", "enforcement_ceilings", "files"],
                    integer(2),
                );
                Ok(())
            }),
            expected: "",
            contract_rel: default,
        },
        Case {
            name: "archived part routes an open leaf",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                archive_fixture_activity_part(base, contract)?;
                if let Some(Py::List(routes)) = nested_field_mut(contract, &["leaf_routes"]) {
                    set_field(&mut routes[0], "lifecycle", text("open"));
                }
                write_fixture_derived(base, contract)
            }),
            expected: "archived part 'activity' routes open leaf 'PROGRAM.1'",
            contract_rel: default,
        },
        Case {
            name: "archived part left at its live path",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                archive_fixture_activity_part(base, contract)?;
                let part = array(field(field(Some(contract), "destinations"), "parts"))[1].clone();
                let raw = fs::read(absolute(base, &scalar(field(Some(&part), "path"))))
                    .map_err(|error| os_error(&error))?;
                write_raw(base, &scalar(field(Some(&part), "archived_from")), &raw)
            }),
            expected: "still exists at its live path",
            contract_rel: default,
        },
        Case {
            name: "archived part is the frontier part",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                archive_fixture_activity_part(base, contract)?;
                set_field(
                    contract,
                    "current_frontier",
                    object(vec![
                        ("mode", text("eligible")),
                        ("literal", text("No eligible frontier.")),
                        ("leaf_id", text("PROGRAM")),
                        ("part_id", text("activity")),
                    ]),
                );
                write_fixture_derived(base, contract)
            }),
            expected: "archived part 'activity' is the current frontier's part",
            contract_rel: default,
        },
        Case {
            name: "archived part byte drift",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                archive_fixture_activity_part(base, contract)?;
                let path = scalar(field(
                    Some(&array(field(field(Some(contract), "destinations"), "parts"))[1]),
                    "path",
                ));
                let mut raw = fs::read(absolute(base, &path)).map_err(|error| os_error(&error))?;
                raw.extend_from_slice(b"changed\n");
                write_raw(base, &path, &raw)
            }),
            expected: "differs from its sealing commit",
            contract_rel: default,
        },
        Case {
            name: "archived part outside the archive directory",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|base, contract, _| {
                archive_fixture_activity_part(base, contract)?;
                if let Some(Py::List(parts)) =
                    nested_field_mut(contract, &["destinations", "parts"])
                {
                    set_field(
                        &mut parts[1],
                        "path",
                        text("docs/tasks/program/archived-activity.md"),
                    );
                }
                Ok(())
            }),
            expected: "outside archive_directory",
            contract_rel: default,
        },
        Case {
            name: "live part declares archived_from",
            state: "migrated",
            input_state: "complete",
            mutator: Some(|_, contract, _| {
                if let Some(Py::List(parts)) =
                    nested_field_mut(contract, &["destinations", "parts"])
                {
                    set_field(
                        &mut parts[1],
                        "archived_from",
                        text("docs/tasks/program/activity.md"),
                    );
                }
                Ok(())
            }),
            expected: "non-archived contract part must not declare archived_from",
            contract_rel: default,
        },
    ]
}

fn convert_inline_mutator(base: &Path, contract: &mut Py, _: &[u8]) -> Result<(), String> {
    convert_fixture_to_inline(base, contract)
}

fn seal_fixture_mutator(base: &Path, contract: &mut Py, source: &[u8]) -> Result<(), String> {
    seal_fixture_activity_part(base, contract, source)
}

fn require_case(condition: bool, message: impl FnOnce() -> String) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message()) }
}

fn run_self_test(project_root: &Path) -> Result<(), String> {
    let generated = project_root.join("generated");
    fs::create_dir_all(&generated).map_err(|error| os_error(&error))?;
    let cases = cases();
    let mut passed = 0;
    for (index, case) in cases.iter().enumerate() {
        let fixture = generated.join(format!(
            ".active-task-evidence-self-test.{}.{index}",
            std::process::id()
        ));
        let _cleanup = prepare_fixture(&fixture)?;
        seed_fixture(
            &fixture,
            case.state,
            case.input_state,
            case.mutator,
            case.contract_rel,
        )?;
        let validation = validate_tree(project_root, &fixture, case.contract_rel, false);
        let joined = validation.errors.join("\n");
        if case.expected.is_empty() {
            require_case(validation.errors.is_empty(), || {
                format!(
                    "active-task-evidence self-test '{}' unexpectedly failed:\n{joined}\n",
                    case.name
                )
            })?;
        } else {
            require_case(!validation.errors.is_empty(), || {
                format!(
                    "active-task-evidence self-test '{}' unexpectedly passed\n",
                    case.name
                )
            })?;
            require_case(
                case.expected
                    .split('|')
                    .any(|expected| joined.contains(expected)),
                || {
                    format!(
                        "active-task-evidence self-test '{}' missed expected diagnostic {}:\n{joined}\n",
                        case.name, case.expected
                    )
                },
            )?;
        }
        passed += 1;
    }

    let writer_fixture = generated.join(format!(
        ".active-task-evidence-writer-self-test.{}",
        std::process::id()
    ));
    {
        let _cleanup = prepare_fixture(&writer_fixture)?;
        seed_fixture(
            &writer_fixture,
            "source_locked",
            "complete",
            None,
            DEFAULT_CONTRACT,
        )?;
        write_raw(
            &writer_fixture,
            "generated/root-template.md",
            fixture_root().as_bytes(),
        )?;
        let mut sink = super::Captured::default();
        materialize_migration(
            project_root,
            &writer_fixture,
            DEFAULT_CONTRACT,
            "generated/root-template.md",
            true,
            &mut sink,
        )?;
        let mut hygiene_errors = Vec::new();
        let (writer_contract, _) = read_json_object(
            &writer_fixture,
            DEFAULT_CONTRACT,
            "migration writer contract",
            131_072,
            &mut hygiene_errors,
        )
        .ok_or_else(|| {
            format!(
                "active-task-evidence self-test 'migration writer positive' could not read its contract:\n{}\n",
                hygiene_errors.join("\n")
            )
        })?;
        for part in array(field(
            field(Some(&writer_contract), "destinations"),
            "parts",
        )) {
            let path = scalar(field(Some(part), "path"));
            let raw = read_regular(
                &writer_fixture,
                &path,
                &format!("migration writer part '{}'", scalar(field(Some(part), "part_id"))),
                &mut hygiene_errors,
            )
            .ok_or_else(|| {
                format!(
                    "active-task-evidence self-test 'migration writer positive' could not read semantic parts:\n{}\n",
                    hygiene_errors.join("\n")
                )
            })?;
            require_case(!raw.ends_with(b"\n\n"), || {
                format!(
                    "active-task-evidence self-test 'migration writer positive' left a blank line at the end of '{path}'\n"
                )
            })?;
        }
        let validation = validate_tree(project_root, &writer_fixture, DEFAULT_CONTRACT, false);
        require_case(
            validation.errors.is_empty()
                && scalar(validation.result.get("migration_state")) == "migrated",
            || {
                format!(
                    "active-task-evidence self-test 'migration writer positive' failed:\n{}\n",
                    validation.errors.join("\n")
                )
            },
        )?;
        passed += 1;
    }
    {
        let _cleanup = prepare_fixture(&writer_fixture)?;
        seed_fixture(
            &writer_fixture,
            "source_locked",
            "complete",
            None,
            DEFAULT_CONTRACT,
        )?;
        let invalid_root = fixture_root().replace("No eligible frontier.", "Unknown frontier.");
        write_raw(
            &writer_fixture,
            "generated/root-template.md",
            invalid_root.as_bytes(),
        )?;
        let mut sink = super::Captured::default();
        let failure = materialize_migration(
            project_root,
            &writer_fixture,
            DEFAULT_CONTRACT,
            "generated/root-template.md",
            true,
            &mut sink,
        )
        .err()
        .ok_or_else(|| {
            "active-task-evidence self-test 'migration writer preflight refusal' unexpectedly passed\n"
                .to_string()
        })?;
        require_case(failure.contains("root template preflight failed"), || {
            "active-task-evidence self-test 'migration writer preflight refusal' missed diagnostic\n"
                .to_string()
        })?;
        let validation = validate_tree(project_root, &writer_fixture, DEFAULT_CONTRACT, false);
        require_case(
            validation.errors.is_empty()
                && scalar(validation.result.get("migration_state")) == "source_locked",
            || {
                "active-task-evidence self-test 'migration writer preflight refusal' changed source state\n"
                    .to_string()
            },
        )?;
        passed += 1;
    }
    {
        let _cleanup = prepare_fixture(&writer_fixture)?;
        seed_fixture(
            &writer_fixture,
            "migrated",
            "complete",
            None,
            DEFAULT_CONTRACT,
        )?;
        write_raw(
            &writer_fixture,
            "docs/tasks/program/INDEX.md",
            b"# PROGRAM task-evidence index\n",
        )?;
        write_raw(
            &writer_fixture,
            "docs/tasks/program/routes-0007.md",
            b"# stray\n",
        )?;
        let mut sink = super::Captured::default();
        materialize_derived(
            project_root,
            &writer_fixture,
            DEFAULT_CONTRACT,
            true,
            &mut sink,
        )?;
        require_case(
            !writer_fixture
                .join("docs/tasks/program/routes-0007.md")
                .exists(),
            || {
                "active-task-evidence self-test 'derived writer positive' left a stray route catalog part\n"
                    .to_string()
            },
        )?;
        let validation = validate_tree(project_root, &writer_fixture, DEFAULT_CONTRACT, false);
        require_case(validation.errors.is_empty(), || {
            format!(
                "active-task-evidence self-test 'derived writer positive' failed:\n{}\n",
                validation.errors.join("\n")
            )
        })?;
        passed += 1;
        let preimage = fs::read(writer_fixture.join("docs/tasks/program/routes-0001.md"))
            .map_err(|error| os_error(&error))?;
        write_raw(
            &writer_fixture,
            "docs/tasks/program/foundation.md",
            b"corrupted\n",
        )?;
        let refused = materialize_derived(
            project_root,
            &writer_fixture,
            DEFAULT_CONTRACT,
            true,
            &mut sink,
        )
        .err()
        .ok_or_else(|| {
            "active-task-evidence self-test 'derived writer refusal' unexpectedly passed\n"
                .to_string()
        })?;
        require_case(refused.contains("writer: preflight failed"), || {
            "active-task-evidence self-test 'derived writer refusal' missed diagnostic\n"
                .to_string()
        })?;
        let after = fs::read(writer_fixture.join("docs/tasks/program/routes-0001.md"))
            .map_err(|error| os_error(&error))?;
        require_case(after == preimage, || {
            "active-task-evidence self-test 'derived writer refusal' changed a derived surface\n"
                .to_string()
        })?;
        passed += 1;
    }

    let prefix = "fx-source-region";
    let sealed = format!(
        "<!-- {prefix}:r1:start -->\n- ID: `T.1`\n  Status: `pending`\n<!-- {prefix}:r1:end -->\n"
    );
    let free = "\n## Post-migration work\n\n- ID: `T.1`\n  Status: `done`\n";
    let mixed = format!("{sealed}{free}");
    let both = declared_node_statuses(Some(mixed.as_bytes()), "T.1");
    let stripped = strip_marked_regions(Some(mixed.as_bytes()), prefix);
    let only = declared_node_statuses(Some(&stripped), "T.1");
    require_case(both.len() == 2, || {
        format!(
            "active-task-evidence self-test 'stratum split' saw {} declarations\n",
            both.len()
        )
    })?;
    require_case(
        only.len() == 1 && only[0].as_deref() == Some("done"),
        || {
            "active-task-evidence self-test 'stratum split' did not isolate the post-migration one\n"
            .to_string()
        },
    )?;
    passed += 1;
    let sealed_only_raw = strip_marked_regions(Some(sealed.as_bytes()), prefix);
    let sealed_only = declared_node_statuses(Some(&sealed_only_raw), "T.1");
    let sealed_all = declared_node_statuses(Some(sealed.as_bytes()), "T.1");
    require_case(
        sealed_only.is_empty()
            && sealed_all.len() == 1
            && sealed_all[0].as_deref() == Some("pending"),
        || {
            "active-task-evidence self-test 'sealed-only fallback' stripped the wrong stratum\n"
                .to_string()
        },
    )?;
    passed += 1;
    let duplicated = format!("{mixed}\n- ID: `T.1`\n  Status: `pending`\n");
    let duplicated_raw = strip_marked_regions(Some(duplicated.as_bytes()), prefix);
    let dup_free = declared_node_statuses(Some(&duplicated_raw), "T.1");
    require_case(dup_free.len() == 2, || {
        "active-task-evidence self-test 'same-stratum duplicate' was not preserved as a breach\n"
            .to_string()
    })?;
    passed += 1;

    let inline = "- ID: `T.2` · Status: `done` · Goal: stated on the owner's own line\n";
    let read = declared_node_statuses(Some(inline.as_bytes()), "T.2");
    require_case(
        read.len() == 1 && read[0].as_deref() == Some("done"),
        || {
            "active-task-evidence self-test 'inline node status' did not read the owner's own line\n"
            .to_string()
        },
    )?;
    passed += 1;
    let continued = "- ID: `T.3` · Goal: stated below\n  Status: `pending`\n";
    let below = declared_node_statuses(Some(continued.as_bytes()), "T.3");
    require_case(
        below.len() == 1 && below[0].as_deref() == Some("pending"),
        || {
            "active-task-evidence self-test 'annotated id with an indented status' lost the status\n"
            .to_string()
        },
    )?;
    passed += 1;
    let statusless = "- ID: `T.4` · Goal: a node that declares no status\n  Acceptance: none\n";
    let none = declared_node_statuses(Some(statusless.as_bytes()), "T.4");
    require_case(none.len() == 1 && none[0].is_none(), || {
        "active-task-evidence self-test 'statusless node' invented a status\n".to_string()
    })?;
    passed += 1;

    let exemption_base = || {
        let mut value = fixture_limit_pair(
            &[
                ("lines_each", 10),
                ("bytes_each", 10),
                ("line_bytes_each", 10),
                ("lines_total", 10),
                ("bytes_total", 10),
            ],
            &[
                ("lines_each", 20),
                ("bytes_each", 20),
                ("line_bytes_each", 20),
                ("lines_total", 20),
                ("bytes_total", 20),
            ],
        );
        if let Some(health) = nested_field_mut(&mut value, &["health_targets"]) {
            set_field(health, "files", Py::None);
        }
        if let Some(ceilings) = nested_field_mut(&mut value, &["enforcement_ceilings"]) {
            set_field(ceilings, "files", Py::None);
        }
        set_field(
            &mut value,
            "cardinality_exemption",
            object(vec![
                ("authority", text("docs/decisions/0045.md")),
                ("work_unit", text("T.1")),
                ("rationale", text("measured")),
            ]),
        );
        value
    };
    let fields = [
        "files",
        "lines_each",
        "bytes_each",
        "line_bytes_each",
        "lines_total",
        "bytes_total",
    ];
    let mut ok = Vec::new();
    validate_limit_pair(
        Some(&exemption_base()),
        "fixture parts limits",
        &mut ok,
        &fields,
        &[],
    );
    require_case(ok.is_empty(), || {
        format!(
            "active-task-evidence self-test 'declared exemption' was refused: {}\n",
            ok.join(" ")
        )
    })?;
    passed += 1;
    let mut valued = exemption_base();
    set_nested(&mut valued, &["enforcement_ceilings", "files"], integer(24));
    let mut red_valued = Vec::new();
    validate_limit_pair(
        Some(&valued),
        "fixture parts limits",
        &mut red_valued,
        &fields,
        &[],
    );
    require_case(
        red_valued
            .iter()
            .any(|error| error.contains("exempt dimension 'files' must be null")),
        || {
            "active-task-evidence self-test 'exempt dimension still valued' was accepted\n"
                .to_string()
        },
    )?;
    passed += 1;
    let mut unauthored = exemption_base();
    remove_nested(&mut unauthored, &["cardinality_exemption"], "authority");
    let mut red_unauthored = Vec::new();
    validate_limit_pair(
        Some(&unauthored),
        "fixture parts limits",
        &mut red_unauthored,
        &fields,
        &[],
    );
    require_case(
        red_unauthored
            .iter()
            .any(|error| error.contains("cardinality_exemption lacks 'authority'")),
        || {
            "active-task-evidence self-test 'exemption without authority' was accepted\n"
                .to_string()
        },
    )?;
    passed += 1;
    let mut bare = exemption_base();
    remove_field(&mut bare, "cardinality_exemption");
    let mut red_bare = Vec::new();
    validate_limit_pair(
        Some(&bare),
        "fixture parts limits",
        &mut red_bare,
        &fields,
        &[],
    );
    require_case(
        red_bare
            .iter()
            .any(|error| error.contains("lacks positive integer 'files'")),
        || {
            "active-task-evidence self-test 'null files without an exemption' was accepted\n"
                .to_string()
        },
    )?;
    passed += 1;

    let expected = 92;
    require_case(passed == expected, || {
        format!(
            "active-task-evidence self-test ran {passed} assertions, declaration expects {expected} — re-derive the declaration beside the suite\n"
        )
    })?;
    Ok(())
}

fn usage(output: &mut dyn super::Output) -> i32 {
    let program = std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "cargo xtask check active-task-evidence".to_string());
    output.err(&format!(
        "Usage: {program} [--root DIR] [--contract PATH] [--check|--report|--write|--self-test|--migrate ROOT_TEMPLATE]\n"
    ));
    2
}

/// Run the active-task evidence checker with its original arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut requested_root = None;
    let mut contract_rel = DEFAULT_CONTRACT.to_string();
    let mut report = false;
    let mut self_test = false;
    let mut write = false;
    let mut migrate_template_rel = None;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => match arguments.next() {
                Some(directory) => requested_root = Some(PathBuf::from(directory)),
                None => return usage(output),
            },
            "--contract" => match arguments.next() {
                Some(path) => contract_rel = path.to_string(),
                None => return usage(output),
            },
            "--check" => {}
            "--report" => report = true,
            "--self-test" => self_test = true,
            "--write" => write = true,
            "--migrate" => match arguments.next() {
                Some(path) => migrate_template_rel = Some(path.to_string()),
                None => return usage(output),
            },
            _ => return usage(output),
        }
    }
    if migrate_template_rel.is_some() && (self_test || report || write)
        || write && (self_test || report)
    {
        return usage(output);
    }
    if self_test {
        return match run_self_test(root) {
            Ok(()) => {
                output.out(
                    "active-task-evidence self-test: 92/92 source/topology/route/payload/bound cases pass.\n",
                );
                0
            }
            Err(error) => {
                output.err(&error);
                25
            }
        };
    }
    let base = match requested_root {
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
                    output.err("active-task-evidence: root does not exist\n");
                    return 2;
                }
            }
        }
        None => root.to_path_buf(),
    };
    if let Some(template) = migrate_template_rel {
        return match materialize_migration(root, &base, &contract_rel, &template, false, output) {
            Ok(()) => 0,
            Err(error) => {
                output.err(&error);
                25
            }
        };
    }
    if write {
        return match materialize_derived(root, &base, &contract_rel, false, output) {
            Ok(()) => 0,
            Err(error) => {
                output.err(&error);
                25
            }
        };
    }
    let validation = validate_tree(root, &base, &contract_rel, false);
    if !validation.errors.is_empty() {
        for error in &validation.errors {
            output.err(&format!("active-task-evidence: {error}\n"));
        }
        output.err(&format!(
            "active-task-evidence: FAILED with {} violation(s).\n",
            validation.errors.len()
        ));
        return 1;
    }
    for warning in &validation.warnings {
        output.err(&format!("active-task-evidence: warning: {warning}\n"));
    }
    if report {
        let result = Py::Dict(
            validation
                .result
                .iter()
                .map(|(key, value)| (pyjson::py_str(key), value.clone()))
                .collect(),
        );
        output.out(&format!("{}\n", report_json(&result)));
    } else {
        output.out(&format!(
            "active-task-evidence: {}/{} contract is valid.\n",
            scalar(validation.result.get("migration_state")),
            scalar(validation.result.get("input_state"))
        ));
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_ninety_two_source_topology_route_payload_and_bound_cases_pass() {
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
            "active-task-evidence self-test: 92/92 source/topology/route/payload/bound cases pass.\n"
        );
    }
}
