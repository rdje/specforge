//! Derived-state field contracts, native since `RUSTDOC-AND-XTASK.15.k`: bounded JSONL registries,
//! current-surface ownership, all four field classes, secondary copies and executed verifiers.
//! `scripts/check_derived_state_contracts.pl` remains the argument-compatible shim.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "derived-state";

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

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn absolute(root: &Path, relative: &str) -> PathBuf {
    root.join(relative)
}

fn relative_to_root(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

fn scalar_value(value: Option<&Py>) -> Option<String> {
    if value.is_none_or(perl::is_reference) {
        return None;
    }
    let text = perl::scalar(value).unwrap_or_default();
    (!text.is_empty()).then_some(text)
}

fn problem(errors: &mut Vec<String>, message: impl AsRef<str>) {
    errors.push(message.as_ref().to_string());
}

fn safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && !path.split('/').any(|segment| matches!(segment, "." | ".."))
        && !path.contains("//")
        && !path
            .chars()
            .any(|character| character <= '\u{1f}' || character == '\u{7f}')
}

fn glob_match(pattern: &str, text: &str) -> bool {
    fn recurse(pattern: &[u8], text: &[u8]) -> bool {
        if pattern.is_empty() {
            return text.is_empty();
        }
        match pattern[0] {
            b'*' if pattern.get(1) == Some(&b'*') => {
                (0..=text.len()).any(|index| recurse(&pattern[2..], &text[index..]))
            }
            b'*' => {
                let mut index = 0;
                loop {
                    if recurse(&pattern[1..], &text[index..]) {
                        return true;
                    }
                    if index == text.len() || text[index] == b'/' {
                        return false;
                    }
                    index += 1;
                }
            }
            b'?' => !text.is_empty() && text[0] != b'/' && recurse(&pattern[1..], &text[1..]),
            literal => {
                if text.first() == Some(&literal) {
                    recurse(&pattern[1..], &text[1..])
                } else {
                    false
                }
            }
        }
    }
    recurse(pattern.as_bytes(), text.as_bytes())
}

fn reject_unknown_fields(value: &Py, label: &str, allowed: &[&str], errors: &mut Vec<String>) {
    let Py::Dict(pairs) = value else {
        return;
    };
    let mut unknown: Vec<String> = pairs
        .iter()
        .map(|(key, _)| pyjson::to_text(key))
        .filter(|key| !allowed.contains(&key.as_str()))
        .collect();
    unknown.sort();
    for key in unknown {
        problem(errors, format!("{label} has unknown field '{key}'"));
    }
}

fn validate_value_bounds(
    value: &Py,
    label: &str,
    max_array: usize,
    max_scalar: usize,
    errors: &mut Vec<String>,
) {
    match value {
        Py::Dict(pairs) => {
            let mut pairs: Vec<_> = pairs.iter().collect();
            pairs.sort_by(|left, right| left.0.cmp(&right.0));
            for (key, item) in pairs {
                validate_value_bounds(
                    item,
                    &format!("{label}.{}", pyjson::to_text(key)),
                    max_array,
                    max_scalar,
                    errors,
                );
            }
        }
        Py::List(items) => {
            if items.len() > max_array {
                problem(
                    errors,
                    format!("{label} has more than {max_array} array items"),
                );
            }
            for (index, item) in items.iter().enumerate() {
                validate_value_bounds(
                    item,
                    &format!("{label}[{index}]"),
                    max_array,
                    max_scalar,
                    errors,
                );
            }
        }
        Py::None | Py::Bool(_) | Py::Int(_) | Py::Float(_) | Py::Str(_) => {
            let text = perl::scalar(Some(value)).unwrap_or_default();
            if text.len() > max_scalar {
                problem(
                    errors,
                    format!("{label} exceeds the declared scalar byte limit {max_scalar}"),
                );
            }
        }
    }
}

fn read_jsonl_registry(
    root: &Path,
    relative: &str,
    label: &str,
    hard_records: usize,
    hard_bytes: usize,
    hard_record_bytes: usize,
    errors: &mut Vec<String>,
) -> (Py, Vec<Py>) {
    let path = absolute(root, relative);
    let Ok(metadata) = fs::symlink_metadata(&path) else {
        problem(
            errors,
            format!(
                "{label} is missing or not a regular file: {}",
                relative_to_root(root, &path)
            ),
        );
        return (Py::None, Vec::new());
    };
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        problem(
            errors,
            format!(
                "{label} is missing or not a regular file: {}",
                relative_to_root(root, &path)
            ),
        );
        return (Py::None, Vec::new());
    }
    let file_bytes = metadata.len() as usize;
    if file_bytes > hard_bytes {
        problem(
            errors,
            format!("{label} exceeds portable hard byte cap {hard_bytes}"),
        );
    }
    let Ok(raw) = fs::read(&path) else {
        problem(errors, format!("cannot read {label}"));
        return (Py::None, Vec::new());
    };
    let text = String::from_utf8_lossy(&raw);
    let mut records = Vec::new();
    let mut lengths = Vec::new();
    for (index, line) in text.split_inclusive('\n').enumerate() {
        let number = index + 1;
        let line = line.strip_suffix('\n').unwrap_or(line);
        let line = line.strip_suffix('\r').unwrap_or(line);
        if line.is_empty() {
            problem(errors, format!("{label} line {number} is blank"));
            continue;
        }
        if line.len() > hard_record_bytes {
            problem(
                errors,
                format!(
                    "{label} line {number} exceeds portable raw-record cap {hard_record_bytes}"
                ),
            );
        }
        match pyjson::loads_strict(line) {
            Ok(record @ Py::Dict(_)) => {
                records.push(record);
                lengths.push(line.len());
            }
            _ => problem(
                errors,
                format!("{label} line {number} is not one JSON object"),
            ),
        }
    }
    if records.is_empty() {
        problem(errors, format!("{label} is empty"));
        return (Py::None, Vec::new());
    }
    let meta = records.remove(0);
    reject_unknown_fields(
        &meta,
        &format!("{label} registry record"),
        &[
            "record_type",
            "schema_version",
            "max_records",
            "max_bytes",
            "max_record_bytes",
            "max_array_items",
            "max_scalar_bytes",
            "milestones",
        ],
        errors,
    );
    if scalar(&meta, "record_type") != "registry" {
        problem(
            errors,
            format!("{label} first record must have record_type=registry"),
        );
    }
    for field in [
        "schema_version",
        "max_records",
        "max_bytes",
        "max_record_bytes",
        "max_array_items",
        "max_scalar_bytes",
    ] {
        let value = pyjson::get(&meta, field);
        if value.is_none_or(perl::is_reference)
            || !perl::scalar(value)
                .unwrap_or_default()
                .chars()
                .all(|character| character.is_ascii_digit())
        {
            problem(
                errors,
                format!("{label} registry record lacks numeric '{field}'"),
            );
        }
    }
    if pyjson::get(&meta, "schema_version").is_some_and(|value| perl::numeric(Some(value)) != 1.0) {
        problem(errors, format!("{label} schema_version must be 1"));
    }
    for field in [
        "max_records",
        "max_bytes",
        "max_record_bytes",
        "max_array_items",
        "max_scalar_bytes",
    ] {
        if pyjson::get(&meta, field).is_some_and(|value| perl::numeric(Some(value)) == 0.0) {
            problem(
                errors,
                format!("{label} registry '{field}' must be positive"),
            );
        }
    }
    if let Some(max_records) = pyjson::get(&meta, "max_records") {
        let max_records = perl::numeric(Some(max_records)) as usize;
        if max_records > hard_records {
            problem(
                errors,
                format!("{label} declares max_records above portable hard cap"),
            );
        }
        if records.len() > max_records {
            problem(
                errors,
                format!("{label} has more records than its declared max_records"),
            );
        }
    }
    if let Some(max_bytes) = pyjson::get(&meta, "max_bytes") {
        let max_bytes = perl::numeric(Some(max_bytes)) as usize;
        if max_bytes > hard_bytes {
            problem(
                errors,
                format!("{label} declares max_bytes above portable hard cap"),
            );
        }
        if file_bytes > max_bytes {
            problem(errors, format!("{label} exceeds its declared max_bytes"));
        }
    }
    if let Some(max_record_bytes) = pyjson::get(&meta, "max_record_bytes") {
        let max_record_bytes = perl::numeric(Some(max_record_bytes)) as usize;
        if max_record_bytes > hard_record_bytes {
            problem(
                errors,
                format!("{label} declares max_record_bytes above portable hard cap"),
            );
        }
        if lengths.iter().any(|length| *length > max_record_bytes) {
            problem(
                errors,
                format!("{label} contains a record above its declared max_record_bytes"),
            );
        }
    }
    if pyjson::get(&meta, "max_array_items").is_some_and(|value| perl::numeric(Some(value)) > 64.0)
    {
        problem(
            errors,
            format!("{label} declares max_array_items above portable hard cap"),
        );
    }
    if pyjson::get(&meta, "max_scalar_bytes")
        .is_some_and(|value| perl::numeric(Some(value)) > 1_024.0)
    {
        problem(
            errors,
            format!("{label} declares max_scalar_bytes above portable hard cap"),
        );
    }
    if let (Some(max_array), Some(max_scalar)) = (
        pyjson::get(&meta, "max_array_items"),
        pyjson::get(&meta, "max_scalar_bytes"),
    ) {
        let max_array = perl::numeric(Some(max_array)) as usize;
        let max_scalar = perl::numeric(Some(max_scalar)) as usize;
        for record in &records {
            validate_value_bounds(
                record,
                &format!("{label} data record"),
                max_array,
                max_scalar,
                errors,
            );
        }
    }
    (meta, records)
}

fn scalar(value: &Py, key: &str) -> String {
    perl::scalar(pyjson::get(value, key)).unwrap_or_default()
}

fn literal_occurrences(content: &str, literal: &str) -> usize {
    if literal.is_empty() {
        0
    } else {
        content.matches(literal).count()
    }
}

fn required_class_scalar(
    contract: &Py,
    field: &str,
    id: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    let value = scalar_value(pyjson::get(contract, field));
    if value.is_none() {
        problem(
            errors,
            format!("derived-state contract '{id}' lacks non-empty scalar '{field}'"),
        );
    }
    value
}

#[derive(Default)]
struct ContractState {
    contract_seen: BTreeSet<String>,
    field_seen: BTreeSet<(String, String, String)>,
    marker_seen: BTreeSet<(String, String)>,
    class_count: BTreeMap<String, usize>,
    secondary_copy_count: usize,
    secondary_ownership_count: BTreeMap<String, usize>,
    executed_core: BTreeSet<String>,
}

fn validate_contract(
    root: &Path,
    registry: &str,
    contract: &Py,
    surfaces: &BTreeMap<String, Py>,
    state: &mut ContractState,
    output: &mut dyn super::Output,
    errors: &mut Vec<String>,
) {
    let classification = scalar_value(pyjson::get(contract, "classification"))
        .unwrap_or_else(|| "<invalid>".to_string());
    let mut class_fields: Vec<&str> = Vec::new();
    match classification.as_str() {
        "derive_on_read" => {
            class_fields.extend(["accessor", "forbidden_storage_marker", "verifier"])
        }
        "verified_copy" => class_fields.extend(["accessor", "verifier", "secondary_copies"]),
        "immutable_evidence" => class_fields.push("capture_boundary"),
        _ => {}
    }
    let mut common = vec![
        "record_type",
        "schema_version",
        "contract_id",
        "surface_id",
        "path",
        "field_id",
        "classification",
        "field_marker",
        "authority",
    ];
    common.extend(class_fields);
    reject_unknown_fields(contract, "derived-state contract", &common, errors);
    if scalar(contract, "record_type") != "contract" {
        problem(
            errors,
            "derived-state record must have record_type=contract",
        );
    }
    let schema = pyjson::get(contract, "schema_version");
    if schema.is_none_or(perl::is_reference)
        || !perl::scalar(schema)
            .unwrap_or_default()
            .chars()
            .all(|character| character.is_ascii_digit())
        || perl::numeric(schema) != 1.0
    {
        problem(
            errors,
            "derived-state contract has unsupported schema_version",
        );
    }
    let mut values = BTreeMap::new();
    for field in [
        "contract_id",
        "surface_id",
        "path",
        "field_id",
        "classification",
        "field_marker",
        "authority",
    ] {
        let value = scalar_value(pyjson::get(contract, field));
        if value.is_none() {
            problem(
                errors,
                format!("derived-state contract lacks non-empty scalar '{field}'"),
            );
        }
        values.insert(field, value);
    }
    let Some(id) = values.get("contract_id").cloned().flatten() else {
        return;
    };
    if !valid_identifier(&id) {
        problem(
            errors,
            format!("derived-state contract_id '{id}' has an invalid identifier shape"),
        );
    }
    if !state.contract_seen.insert(id.clone()) {
        problem(
            errors,
            format!("derived-state contract '{id}' is declared more than once"),
        );
    }
    if !matches!(
        classification.as_str(),
        "derive_on_read" | "verified_copy" | "authored_intent" | "immutable_evidence"
    ) {
        problem(
            errors,
            format!("derived-state contract '{id}' has invalid classification '{classification}'"),
        );
        return;
    }
    *state.class_count.entry(classification.clone()).or_insert(0) += 1;
    let surface_id = values
        .get("surface_id")
        .cloned()
        .flatten()
        .unwrap_or_default();
    let Some(surface) = surfaces.get(&surface_id) else {
        problem(
            errors,
            format!("derived-state contract '{id}' names unknown surface '{surface_id}'"),
        );
        return;
    };
    if matches!(
        scalar(surface, "lifecycle").as_str(),
        "archive_terminal" | "external_terminal" | "frozen_legacy"
    ) {
        problem(
            errors,
            format!("derived-state contract '{id}' must govern a current maintained surface"),
        );
        return;
    }
    let path = values.get("path").cloned().flatten().unwrap_or_default();
    if !safe_relative_path(&path) {
        problem(
            errors,
            format!("derived-state contract '{id}' path is absolute, escaping, or malformed"),
        );
        return;
    }
    let Some(Py::List(targets)) = pyjson::get(surface, "targets") else {
        problem(
            errors,
            format!("derived-state contract '{id}' path is outside surface '{surface_id}': {path}"),
        );
        return;
    };
    let target_match = targets.iter().any(|target| {
        let pattern = perl::scalar(Some(target)).unwrap_or_default();
        safe_relative_path(&pattern) && glob_match(&pattern, &path)
    });
    if !target_match {
        problem(
            errors,
            format!("derived-state contract '{id}' path is outside surface '{surface_id}': {path}"),
        );
        return;
    }
    let absolute_path = absolute(root, &path);
    if !absolute_path.is_file()
        || fs::symlink_metadata(&absolute_path)
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        problem(
            errors,
            format!("derived-state contract '{id}' path is missing or not a regular file: {path}"),
        );
        return;
    }
    if device(&absolute_path) != device(root) {
        problem(
            errors,
            format!("derived-state contract '{id}' path is off the repository volume: {path}"),
        );
        return;
    }
    let field_id = values
        .get("field_id")
        .cloned()
        .flatten()
        .unwrap_or_default();
    if !valid_identifier(&field_id) {
        problem(
            errors,
            format!("derived-state contract '{id}' has invalid field_id '{field_id}'"),
        );
    }
    if !state
        .field_seen
        .insert((surface_id.clone(), path.clone(), field_id.clone()))
    {
        problem(
            errors,
            format!(
                "derived-state field is declared more than once: {surface_id} {path} {field_id}"
            ),
        );
    }
    let marker = values
        .get("field_marker")
        .cloned()
        .flatten()
        .unwrap_or_default();
    if !state.marker_seen.insert((path.clone(), marker.clone())) {
        problem(
            errors,
            format!("derived-state marker is declared more than once: {path} {marker}"),
        );
    }
    let content = match fs::read_to_string(&absolute_path) {
        Ok(content) => content,
        Err(error) => {
            problem(
                errors,
                format!(
                    "derived-state contract '{id}' cannot read {path}: {}",
                    os_error(&error)
                ),
            );
            return;
        }
    };
    let count = literal_occurrences(&content, &marker);
    if count != 1 {
        problem(
            errors,
            format!(
                "derived-state contract '{id}' field marker must occur exactly once in {path} (found {count})"
            ),
        );
    }
    match classification.as_str() {
        "derive_on_read" => {
            let accessor = required_class_scalar(contract, "accessor", &id, errors);
            let forbidden =
                required_class_scalar(contract, "forbidden_storage_marker", &id, errors);
            let verifier = required_class_scalar(contract, "verifier", &id, errors);
            if accessor.is_some_and(|value| !content.contains(&value)) {
                problem(
                    errors,
                    format!("derived-state contract '{id}' accessor is absent from {path}"),
                );
            }
            if forbidden.is_some_and(|value| content.contains(&value)) {
                problem(
                    errors,
                    format!(
                        "derived-state contract '{id}' still stores forbidden current-state marker in {path}"
                    ),
                );
            }
            if verifier.is_some_and(|value| value != "builtin:derive_on_read") {
                problem(
                    errors,
                    format!(
                        "derived-state contract '{id}' derive_on_read must use builtin:derive_on_read"
                    ),
                );
            }
        }
        "verified_copy" => {
            required_class_scalar(contract, "accessor", &id, errors);
            if pyjson::get(contract, "secondary_copies").is_some() {
                validate_secondary_copies(root, contract, &id, surfaces, state, errors);
            }
            let verifier = required_class_scalar(contract, "verifier", &id, errors);
            if let Some(verifier) = verifier {
                execute_verifier(
                    root,
                    registry,
                    &id,
                    &verifier,
                    &mut state.executed_core,
                    output,
                    errors,
                );
            }
        }
        "immutable_evidence" => {
            let boundary = required_class_scalar(contract, "capture_boundary", &id, errors);
            let count = boundary
                .as_ref()
                .map(|value| literal_occurrences(&content, value))
                .unwrap_or(0);
            if boundary.is_some() && count != 1 {
                problem(
                    errors,
                    format!(
                        "derived-state contract '{id}' capture boundary must occur exactly once in {path} (found {count})"
                    ),
                );
            }
        }
        _ => {}
    }
}

fn valid_identifier(id: &str) -> bool {
    let mut characters = id.chars();
    if !characters
        .next()
        .is_some_and(|character| character.is_ascii_lowercase())
    {
        return false;
    }
    characters.all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, '.' | '_' | '-')
    })
}

fn validate_secondary_copies(
    root: &Path,
    contract: &Py,
    id: &str,
    surfaces: &BTreeMap<String, Py>,
    state: &mut ContractState,
    errors: &mut Vec<String>,
) {
    let Some(Py::List(copies)) = pyjson::get(contract, "secondary_copies") else {
        problem(
            errors,
            format!("derived-state contract '{id}' secondary_copies must be a non-empty array"),
        );
        return;
    };
    if copies.is_empty() {
        problem(
            errors,
            format!("derived-state contract '{id}' secondary_copies must be a non-empty array"),
        );
        return;
    }
    let mut role_seen = BTreeSet::new();
    for (index, copy) in copies.iter().enumerate() {
        let label = format!("derived-state contract '{id}' secondary_copies[{index}]");
        if !matches!(copy, Py::Dict(_)) {
            problem(errors, format!("{label} must be an object"));
            continue;
        }
        reject_unknown_fields(
            copy,
            &label,
            &["role", "ownership", "surface_id", "path", "field_marker"],
            errors,
        );
        let mut values = BTreeMap::new();
        for field in ["role", "ownership", "path", "field_marker"] {
            let value = scalar_value(pyjson::get(copy, field));
            if value.is_none() {
                problem(errors, format!("{label} lacks non-empty scalar '{field}'"));
            }
            values.insert(field, value);
        }
        let role = values.get("role").cloned().flatten().unwrap_or_default();
        if !valid_identifier(&role) {
            problem(
                errors,
                format!("{label} role '{role}' has an invalid identifier shape"),
            );
        }
        if !role_seen.insert(role.clone()) {
            problem(
                errors,
                format!(
                    "derived-state contract '{id}' secondary role '{role}' is declared more than once"
                ),
            );
        }
        let ownership = values
            .get("ownership")
            .cloned()
            .flatten()
            .unwrap_or_default();
        if !matches!(ownership.as_str(), "surface" | "control") {
            problem(
                errors,
                format!("{label} has invalid ownership '{ownership}'"),
            );
        }
        let path = values.get("path").cloned().flatten().unwrap_or_default();
        if !safe_relative_path(&path) {
            problem(
                errors,
                format!("{label} path is absolute, escaping, or malformed"),
            );
            continue;
        }
        let absolute_path = absolute(root, &path);
        if !absolute_path.is_file()
            || fs::symlink_metadata(&absolute_path)
                .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            problem(
                errors,
                format!("{label} path is missing or not a regular file: {path}"),
            );
            continue;
        }
        if device(&absolute_path) != device(root) {
            problem(
                errors,
                format!("{label} path is off the repository volume: {path}"),
            );
            continue;
        }
        let surface_id = scalar_value(pyjson::get(copy, "surface_id"));
        if ownership == "surface" {
            let Some(surface_id) = surface_id else {
                problem(
                    errors,
                    format!("{label} surface ownership requires surface_id"),
                );
                continue;
            };
            let Some(surface) = surfaces.get(&surface_id) else {
                problem(
                    errors,
                    format!("{label} names unknown surface '{surface_id}'"),
                );
                continue;
            };
            if matches!(
                scalar(surface, "lifecycle").as_str(),
                "archive_terminal" | "external_terminal" | "frozen_legacy"
            ) {
                problem(
                    errors,
                    format!("{label} must govern a current maintained surface"),
                );
            }
            let Some(Py::List(targets)) = pyjson::get(surface, "targets") else {
                problem(
                    errors,
                    format!("{label} path is outside surface '{surface_id}': {path}"),
                );
                continue;
            };
            let matches = targets.iter().any(|target| {
                let pattern = perl::scalar(Some(target)).unwrap_or_default();
                safe_relative_path(&pattern) && glob_match(&pattern, &path)
            });
            if !matches {
                problem(
                    errors,
                    format!("{label} path is outside surface '{surface_id}': {path}"),
                );
            }
        } else if ownership == "control" {
            if surface_id.is_some() {
                problem(
                    errors,
                    format!("{label} control ownership must not declare surface_id"),
                );
            }
            if path.to_ascii_lowercase().ends_with(".md") {
                problem(
                    errors,
                    format!("{label} Markdown path must use surface ownership: {path}"),
                );
            }
        }
        let Ok(content) = fs::read_to_string(&absolute_path) else {
            problem(errors, format!("{label} cannot read {path}"));
            continue;
        };
        if let Some(marker) = values.get("field_marker").cloned().flatten() {
            if !state.marker_seen.insert((path.clone(), marker.clone())) {
                problem(
                    errors,
                    format!("derived-state marker is declared more than once: {path} {marker}"),
                );
            }
            let count = literal_occurrences(&content, &marker);
            if count != 1 {
                problem(
                    errors,
                    format!(
                        "{label} field marker must occur exactly once in {path} (found {count})"
                    ),
                );
            }
        }
        state.secondary_copy_count += 1;
        if !ownership.is_empty() {
            *state
                .secondary_ownership_count
                .entry(ownership)
                .or_insert(0) += 1;
        }
    }
}

fn execute_verifier(
    root: &Path,
    registry: &str,
    id: &str,
    verifier: &str,
    executed_core: &mut BTreeSet<String>,
    output: &mut dyn super::Output,
    errors: &mut Vec<String>,
) {
    let Some((kind, path)) = verifier.split_once(':') else {
        problem(
            errors,
            format!(
                "derived-state contract '{id}' verified_copy must use core: or adapter: execution"
            ),
        );
        return;
    };
    if !matches!(kind, "core" | "adapter") {
        problem(
            errors,
            format!(
                "derived-state contract '{id}' verified_copy must use core: or adapter: execution"
            ),
        );
        return;
    }
    if !safe_relative_path(path) || path.contains(['*', '?']) {
        problem(
            errors,
            format!("derived-state contract '{id}' has unsafe verifier path '{path}'"),
        );
        return;
    }
    if kind == "core" && !executed_core.insert(path.to_string()) {
        return;
    }
    let absolute_path = absolute(root, path);
    if !absolute_path.is_file()
        || fs::symlink_metadata(&absolute_path)
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        problem(
            errors,
            format!("derived-state contract '{id}' verifier is missing: {path}"),
        );
        return;
    }
    if kind == "adapter" && path == "scripts/check_derived_state_authorities.pl" {
        let code = super::derived_state_authorities::run_to(
            root,
            &[
                "--root".to_string(),
                root.to_string_lossy().into_owned(),
                "--registry".to_string(),
                registry.to_string(),
                "--contract".to_string(),
                id.to_string(),
            ],
            output,
        );
        if code != 0 {
            problem(
                errors,
                format!("derived-state contract '{id}' verifier failed (exit {code}): {path}"),
            );
        }
        return;
    }
    if kind == "core" && !path.ends_with(".pl") && !executable(&absolute_path) {
        problem(
            errors,
            format!("derived-state contract '{id}' core verifier is not executable: {path}"),
        );
        return;
    }
    let mut command = if path.ends_with(".pl") {
        let mut command = Command::new("perl");
        command.arg(&absolute_path);
        command
    } else {
        Command::new(&absolute_path)
    };
    if kind == "adapter" {
        command.args([
            "--root".to_string(),
            root.to_string_lossy().into_owned(),
            "--registry".to_string(),
            registry.to_string(),
            "--contract".to_string(),
            id.to_string(),
        ]);
    }
    let result = command.current_dir(root).stdin(Stdio::null()).output();
    match result {
        Ok(process) => {
            output.out(&String::from_utf8_lossy(&process.stdout));
            output.err(&String::from_utf8_lossy(&process.stderr));
            let code = crate::exit_code(process.status);
            if code != 0 {
                problem(
                    errors,
                    format!("derived-state contract '{id}' verifier failed (exit {code}): {path}"),
                );
            }
        }
        Err(error) => problem(
            errors,
            format!("derived-state contract '{id}' cannot execute verifier '{path}': {error}"),
        ),
    }
}

#[cfg(unix)]
fn executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn executable(path: &Path) -> bool {
    path.is_file()
}

/// Run the derived-state contracts with the arguments their Perl shim normalized.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut root = root.to_path_buf();
    let mut registry = "doctrine/live_document_size/derived_state_contracts.jsonl".to_string();
    let mut surfaces = "doctrine/live_document_size/surfaces.jsonl".to_string();
    let mut report = false;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => match arguments.next() {
                Some(directory) => root = PathBuf::from(directory),
                None => return usage(output),
            },
            "--registry" => match arguments.next() {
                Some(path) => registry = path.to_string(),
                None => return usage(output),
            },
            "--surfaces" => match arguments.next() {
                Some(path) => surfaces = path.to_string(),
                None => return usage(output),
            },
            "--report" => report = true,
            _ => return usage(output),
        }
    }
    if !root.is_dir() {
        output.err("derived-state: repository root does not exist\n");
        return 1;
    }
    let root = fs::canonicalize(&root).unwrap_or(root);
    let mut errors = Vec::new();
    let (_, surface_records) = read_jsonl_registry(
        &root,
        &surfaces,
        "surface registry",
        128,
        131_072,
        8_192,
        &mut errors,
    );
    let (_, contracts) = read_jsonl_registry(
        &root,
        &registry,
        "derived-state registry",
        64,
        65_536,
        4_096,
        &mut errors,
    );
    let mut surfaces_by_id = BTreeMap::new();
    for surface in surface_records {
        let id = scalar(&surface, "surface_id");
        if id.is_empty() {
            continue;
        }
        if surfaces_by_id.contains_key(&id) {
            problem(
                &mut errors,
                format!("duplicate surface_id '{id}' prevents derived-state ownership"),
            );
            continue;
        }
        surfaces_by_id.insert(id, surface);
    }
    let mut state = ContractState::default();
    for contract in &contracts {
        validate_contract(
            &root,
            &registry,
            contract,
            &surfaces_by_id,
            &mut state,
            output,
            &mut errors,
        );
    }
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
    if report {
        let value = Py::Dict(vec![
            (
                pyjson::py_str("contracts"),
                Py::Int(contracts.len().to_string()),
            ),
            (
                pyjson::py_str("classifications"),
                Py::Dict(
                    state
                        .class_count
                        .into_iter()
                        .map(|(key, count)| (pyjson::py_str(&key), Py::Int(count.to_string())))
                        .collect(),
                ),
            ),
            (
                pyjson::py_str("executed_core_verifiers"),
                Py::Int(state.executed_core.len().to_string()),
            ),
            (
                pyjson::py_str("secondary_copies"),
                Py::Int(state.secondary_copy_count.to_string()),
            ),
            (
                pyjson::py_str("secondary_ownership"),
                Py::Dict(
                    state
                        .secondary_ownership_count
                        .into_iter()
                        .map(|(key, count)| (pyjson::py_str(&key), Py::Int(count.to_string())))
                        .collect(),
                ),
            ),
        ]);
        let encoded = pyjson::dumps(
            &value,
            &pyjson::Encoder {
                indent: None,
                sort_keys: true,
                ensure_ascii: false,
                item_separator: ",",
                key_separator: ":",
            },
        )
        .expect("report encoding");
        output.out(&format!("{encoded}\n"));
    } else {
        output.out(&format!(
            "{LABEL}: {} explicit field contracts satisfy bounded classification and authority rules.\n",
            contracts.len()
        ));
    }
    0
}

fn usage(output: &mut dyn super::Output) -> i32 {
    output.err("Usage: cargo xtask check derived-state-contracts [--root DIR] [--registry PATH] [--surfaces PATH] [--report]\n");
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_real_contracts_and_report_hold() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(run_to(&root, &[], &mut output), 0, "{}", output.text);
        assert!(output.text.ends_with("derived-state: 14 explicit field contracts satisfy bounded classification and authority rules.\n"));
        output.text.clear();
        assert_eq!(
            run_to(&root, &["--report".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("\"contracts\":14"));
        assert!(output.text.contains("\"executed_core_verifiers\":8"));
        assert!(output.text.contains("\"secondary_copies\":3"));
    }
}
