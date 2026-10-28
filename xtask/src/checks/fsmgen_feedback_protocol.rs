//! FSMGen feedback protocol, native since `RUSTDOC-AND-XTASK.15.r`: exact source regions, evidence,
//! current correspondence, archive identity and the 13-case fixture suite.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "feedback-protocol";

#[derive(Clone, Default)]
struct Metrics {
    lines: usize,
    bytes: usize,
    line_bytes: usize,
    sha256: String,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Check,
    Report,
    SelfTest,
}

fn integer(value: usize) -> Py {
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

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains(['\\', '\0'])
        && !path.contains("//")
        && !path.split('/').any(|part| matches!(part, "." | ".."))
}

fn read_raw(path: &Path) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|error| {
        format!(
            "{LABEL}: cannot read {}: {}",
            path.display(),
            os_error(&error)
        )
    })
}

fn decoded(raw: &[u8], label: &str) -> Result<String, String> {
    String::from_utf8(raw.to_vec()).map_err(|_| format!("{LABEL}: {label} is not valid UTF-8"))
}

fn shim_path() -> String {
    std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "scripts/check_fsmgen_feedback_protocol.pl".to_string())
}

fn load_json(path: &Path) -> Result<Py, String> {
    let raw = read_raw(path)?;
    let text = String::from_utf8(raw).map_err(|error| error.to_string())?;
    pyjson::loads_strict(&text).map_err(|error| {
        let offset = text.chars().count();
        if error.starts_with("Expecting property name enclosed in double quotes:")
            && error.ends_with(&format!("(char {offset})"))
        {
            format!(
                "{LABEL}: invalid JSON in {}: , or }} expected while parsing object/hash, at character offset {offset} (before \"(end of string)\") at {} line 157.\n",
                path.display(),
                shim_path()
            )
        } else {
            format!("{LABEL}: invalid JSON in {}: {error}", path.display())
        }
    })
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
        sha256: sha256_hex(raw),
    }
}

fn metrics_json(metrics: Metrics) -> Py {
    object(vec![
        ("lines", integer(metrics.lines)),
        ("bytes", integer(metrics.bytes)),
        ("line_bytes", integer(metrics.line_bytes)),
        ("sha256", text(&metrics.sha256)),
    ])
}

fn line_span(raw: &str, start: Option<&Py>, end: Option<&Py>) -> Option<String> {
    let start = perl::numeric(start) as usize;
    let end = perl::numeric(end) as usize;
    if start < 1 || end < start {
        return None;
    }
    let lines: Vec<&str> = raw.split_inclusive('\n').collect();
    if end > lines.len() {
        return None;
    }
    Some(lines[start - 1..end].join(""))
}

fn count_literal(text: &str, literal: &str) -> usize {
    if literal.is_empty() {
        0
    } else {
        text.matches(literal).count()
    }
}

fn required_hash<'a>(value: Option<&'a Py>, label: &str, errors: &mut Vec<String>) -> &'a Py {
    if matches!(value, Some(Py::Dict(_))) {
        value.unwrap()
    } else {
        errors.push(format!("{label} must be an object"));
        &Py::None
    }
}

fn required_array<'a>(value: Option<&'a Py>, label: &str, errors: &mut Vec<String>) -> &'a [Py] {
    if let Some(Py::List(items)) = value {
        items
    } else {
        errors.push(format!("{label} must be an array"));
        &[]
    }
}

fn validate_safe_path(path: &str, label: &str, errors: &mut Vec<String>) {
    if !safe_relative_path(path) {
        errors.push(format!("{label} must be one safe repository-relative path"));
    }
}

fn validate_metric_contract(
    actual: Metrics,
    expected: &Py,
    label: &str,
    errors: &mut Vec<String>,
    with_sha: bool,
) {
    for (key, value) in [
        ("lines", actual.lines),
        ("bytes", actual.bytes),
        ("line_bytes", actual.line_bytes),
    ] {
        let expected_value = pyjson::get(expected, key);
        if expected_value.is_none_or(perl::is_reference)
            || !scalar(expected_value)
                .chars()
                .all(|character| character.is_ascii_digit())
            || scalar(expected_value).is_empty()
        {
            errors.push(format!("{label} has invalid {key}"));
            continue;
        }
        let expected_value = perl::numeric(expected_value) as usize;
        if value != expected_value {
            errors.push(format!(
                "{label} {key} differs: {value} != {expected_value}"
            ));
        }
    }
    if with_sha {
        let expected_sha = pyjson::get(expected, "sha256");
        let valid = scalar(expected_sha);
        if expected_sha.is_none_or(perl::is_reference)
            || valid.len() != 64
            || !valid
                .chars()
                .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
        {
            errors.push(format!("{label} has invalid sha256"));
        } else if actual.sha256 != valid {
            errors.push(format!(
                "{label} sha256 differs: {} != {valid}",
                actual.sha256
            ));
        }
    }
}

fn validate_paths(contract: &Py, errors: &mut Vec<String>) {
    let source = required_hash(pyjson::get(contract, "source"), "source", errors);
    validate_safe_path(&scalar(pyjson::get(source, "path")), "source.path", errors);
    let archive = required_hash(pyjson::get(contract, "archive"), "archive", errors);
    for field in ["index", "manifest", "source_capsule"] {
        validate_safe_path(
            &scalar(pyjson::get(archive, field)),
            &format!("archive.{field}"),
            errors,
        );
    }
    let mut seen = BTreeSet::new();
    for record in required_array(pyjson::get(contract, "records"), "records", errors) {
        if !matches!(record, Py::Dict(_)) {
            continue;
        }
        let id = scalar(pyjson::get(record, "id"));
        if !valid_kebab(&id) {
            errors.push(format!("record has invalid id '{id}'"));
        }
        if !seen.insert(id.clone()) {
            errors.push(format!("record id '{id}' is duplicated"));
        }
        for evidence in required_array(
            pyjson::get(record, "evidence"),
            &format!("record '{id}' evidence"),
            errors,
        ) {
            if matches!(evidence, Py::Dict(_)) {
                validate_safe_path(
                    &scalar(pyjson::get(evidence, "path")),
                    &format!("record '{id}' evidence path"),
                    errors,
                );
            }
        }
    }
    let consumers = required_hash(pyjson::get(contract, "consumers"), "consumers", errors);
    for path in required_array(
        pyjson::get(consumers, "known_paths"),
        "consumers.known_paths",
        errors,
    ) {
        validate_safe_path(&scalar(Some(path)), "consumer path", errors);
    }
    for item in required_array(
        pyjson::get(consumers, "required_literals"),
        "consumers.required_literals",
        errors,
    ) {
        if matches!(item, Py::Dict(_)) {
            validate_safe_path(
                &scalar(pyjson::get(item, "path")),
                "consumer literal path",
                errors,
            );
        }
    }
    for finding in required_array(
        pyjson::get(contract, "current_truth_findings"),
        "current_truth_findings",
        errors,
    ) {
        if matches!(finding, Py::Dict(_)) {
            validate_safe_path(
                &scalar(pyjson::get(finding, "evidence_path")),
                "finding evidence path",
                errors,
            );
        }
    }
}

fn valid_kebab(id: &str) -> bool {
    !id.is_empty()
        && id.split('-').all(|part| {
            !part.is_empty()
                && part
                    .chars()
                    .all(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        })
}

fn direction_display(direction: &str) -> String {
    match direction {
        "specforge_to_fsmgen" => "SpecForge → FSMGen".to_string(),
        "fsmgen_to_specforge" => "FSMGen → SpecForge".to_string(),
        _ => direction.to_string(),
    }
}

fn validate_source(base: &Path, contract: &Py, errors: &mut Vec<String>) {
    let source = required_hash(pyjson::get(contract, "source"), "source", errors);
    let archive = required_hash(pyjson::get(contract, "archive"), "archive", errors);
    let state = scalar(pyjson::get(contract, "migration_state"));
    let authority = if state == "migrated" {
        scalar(pyjson::get(archive, "source_capsule"))
    } else {
        scalar(pyjson::get(source, "path"))
    };
    if !safe_relative_path(&authority) {
        return;
    }
    let absolute = base.join(&authority);
    if !absolute.is_file() {
        errors.push(format!("source authority '{authority}' is missing"));
        return;
    }
    let raw = match read_raw(&absolute) {
        Ok(raw) => raw,
        Err(error) => {
            errors.push(error);
            return;
        }
    };
    let text = match decoded(&raw, &authority) {
        Ok(text) => text,
        Err(_) => {
            errors.push(format!("source authority '{authority}' is not valid UTF-8"));
            return;
        }
    };
    let actual = metrics(&raw);
    validate_metric_contract(
        actual,
        source,
        &format!("source authority '{authority}'"),
        errors,
        true,
    );

    let regions = required_array(pyjson::get(source, "regions"), "source.regions", errors);
    let mut expected_start = 1;
    let mut reconstructed = String::new();
    let mut region_ids = BTreeSet::new();
    for region in regions {
        if !matches!(region, Py::Dict(_)) {
            errors.push("source region must be an object".to_string());
            continue;
        }
        let id = scalar(pyjson::get(region, "id"));
        if id.is_empty()
            || !id.chars().all(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
            })
        {
            errors.push(format!("source region id '{id}' is invalid"));
        }
        if !region_ids.insert(id.clone()) {
            errors.push(format!("source region id '{id}' is duplicated"));
        }
        let start = pyjson::get(region, "start_line");
        let end = pyjson::get(region, "end_line");
        if perl::numeric(start) as usize != expected_start {
            errors.push(format!(
                "source region '{id}' must start at line {expected_start}"
            ));
        }
        let Some(chunk) = line_span(&text, start, end) else {
            errors.push(format!("source region '{id}' has an invalid line span"));
            continue;
        };
        validate_metric_contract(
            metrics(chunk.as_bytes()),
            region,
            &format!("source region '{id}'"),
            errors,
            true,
        );
        reconstructed.push_str(&chunk);
        expected_start = perl::numeric(end) as usize + 1;
    }
    if reconstructed != text {
        errors.push("source regions do not reconstruct the exact source".to_string());
    }

    let mut record_ids = BTreeSet::new();
    for record in required_array(pyjson::get(contract, "records"), "records", errors) {
        if !matches!(record, Py::Dict(_)) {
            continue;
        }
        let id = scalar(pyjson::get(record, "id"));
        let id = if id.is_empty() {
            "<unknown>".to_string()
        } else {
            id
        };
        record_ids.insert(id.clone());
        let Some(chunk) = line_span(
            &text,
            pyjson::get(record, "start_line"),
            pyjson::get(record, "end_line"),
        ) else {
            errors.push(format!("record '{id}' has an invalid line span"));
            continue;
        };
        validate_metric_contract(
            metrics(chunk.as_bytes()),
            record,
            &format!("record '{id}'"),
            errors,
            true,
        );
        let heading = scalar(pyjson::get(record, "heading"));
        if !chunk.starts_with(&format!("{heading}\n")) && chunk != heading {
            errors.push(format!(
                "record '{id}' heading does not begin its source span"
            ));
        }
        let status_literal = scalar(pyjson::get(record, "source_status_literal"));
        if status_literal.is_empty() || !chunk.contains(&status_literal) {
            errors.push(format!("record '{id}' source status literal is missing"));
        }
        for field in ["kind", "direction", "status"] {
            let value = scalar(pyjson::get(record, field));
            if value.is_empty()
                || !value
                    .chars()
                    .next()
                    .is_some_and(|character| character.is_ascii_lowercase())
                || !value.chars().all(|character| {
                    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
                })
            {
                errors.push(format!("record '{id}' has invalid {field}"));
            }
        }
        for evidence in required_array(
            pyjson::get(record, "evidence"),
            &format!("record '{id}' evidence"),
            errors,
        ) {
            if !matches!(evidence, Py::Dict(_)) {
                continue;
            }
            let path = scalar(pyjson::get(evidence, "path"));
            if !safe_relative_path(&path) {
                continue;
            }
            let absolute = base.join(&path);
            if !absolute.is_file() {
                errors.push(format!("record '{id}' evidence '{path}' is missing"));
                continue;
            }
            let evidence_text = read_raw(&absolute)
                .ok()
                .and_then(|raw| decoded(&raw, &path).ok())
                .unwrap_or_default();
            let literal = scalar(pyjson::get(evidence, "literal"));
            if literal.is_empty() || !evidence_text.contains(&literal) {
                errors.push(format!(
                    "record '{id}' evidence literal is missing from '{path}'"
                ));
            }
        }
    }

    for finding in required_array(
        pyjson::get(contract, "current_truth_findings"),
        "current_truth_findings",
        errors,
    ) {
        if !matches!(finding, Py::Dict(_)) {
            continue;
        }
        let id = scalar(pyjson::get(finding, "id"));
        let id = if id.is_empty() {
            "<unknown>".to_string()
        } else {
            id
        };
        let source_literal = scalar(pyjson::get(finding, "source_literal"));
        if source_literal.is_empty() || !text.contains(&source_literal) {
            errors.push(format!("finding '{id}' source literal is absent"));
        }
        let path = scalar(pyjson::get(finding, "evidence_path"));
        if !safe_relative_path(&path) {
            continue;
        }
        let absolute = base.join(&path);
        if !absolute.is_file() {
            errors.push(format!("finding '{id}' evidence path is missing"));
            continue;
        }
        let evidence_text = read_raw(&absolute)
            .ok()
            .and_then(|raw| decoded(&raw, &path).ok())
            .unwrap_or_default();
        let literal = scalar(pyjson::get(finding, "evidence_literal"));
        if literal.is_empty() || !evidence_text.contains(&literal) {
            errors.push(format!("finding '{id}' evidence literal is absent"));
        }
    }
}

fn validate_consumers(base: &Path, contract: &Py, errors: &mut Vec<String>) {
    let consumers = required_hash(pyjson::get(contract, "consumers"), "consumers", errors);
    let mut seen = BTreeSet::new();
    for path in required_array(
        pyjson::get(consumers, "known_paths"),
        "consumers.known_paths",
        errors,
    ) {
        let path = scalar(Some(path));
        if !safe_relative_path(&path) {
            continue;
        }
        if !seen.insert(path.clone()) {
            errors.push(format!("consumer '{path}' is duplicated"));
        }
        let absolute = base.join(&path);
        if !absolute.is_file() {
            errors.push(format!("consumer '{path}' is missing"));
            continue;
        }
        let text = read_raw(&absolute)
            .ok()
            .and_then(|raw| decoded(&raw, &path).ok())
            .unwrap_or_default();
        if !text.contains("docs/FSMGEN_FEEDBACK.md") {
            errors.push(format!(
                "consumer '{path}' no longer names docs/FSMGEN_FEEDBACK.md"
            ));
        }
    }
    for item in required_array(
        pyjson::get(consumers, "required_literals"),
        "consumers.required_literals",
        errors,
    ) {
        if !matches!(item, Py::Dict(_)) {
            continue;
        }
        let path = scalar(pyjson::get(item, "path"));
        if !safe_relative_path(&path) {
            continue;
        }
        let absolute = base.join(&path);
        if !absolute.is_file() {
            errors.push(format!("required consumer '{path}' is missing"));
            continue;
        }
        let text = read_raw(&absolute)
            .ok()
            .and_then(|raw| decoded(&raw, &path).ok())
            .unwrap_or_default();
        let literal = scalar(pyjson::get(item, "literal"));
        if literal.is_empty() || !text.contains(&literal) {
            errors.push(format!(
                "required consumer literal is missing from '{path}'"
            ));
        }
    }
}

fn between_markers<'a>(
    text: &'a str,
    start: &str,
    end: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> &'a str {
    let start_count = count_literal(text, start);
    let end_count = count_literal(text, end);
    if start_count != 1 {
        errors.push(format!("{label} start marker must occur exactly once"));
    }
    if end_count != 1 {
        errors.push(format!("{label} end marker must occur exactly once"));
    }
    if start_count != 1 || end_count != 1 {
        return "";
    }
    let start_at = text.find(start).unwrap() + start.len();
    let end_at = text.find(end).unwrap();
    if end_at < start_at {
        errors.push(format!("{label} markers are reversed"));
        return "";
    }
    &text[start_at..end_at]
}

fn h3_chunks(text: &str) -> Vec<&str> {
    let mut starts = Vec::new();
    if text.starts_with("### ") {
        starts.push(0);
    }
    for (index, _) in text.match_indices("\n### ") {
        starts.push(index + 1);
    }
    starts
        .iter()
        .enumerate()
        .map(|(index, start)| {
            let end = starts.get(index + 1).copied().unwrap_or(text.len());
            &text[*start..end]
        })
        .collect()
}

fn open_id(line: &str) -> Option<&str> {
    let id = line.strip_prefix("### ")?.split_whitespace().next()?;
    valid_kebab(id).then_some(id)
}

fn validate_current_root(base: &Path, contract: &Py, errors: &mut Vec<String>) {
    let source = required_hash(pyjson::get(contract, "source"), "source", errors);
    let current = required_hash(
        pyjson::get(contract, "current_root"),
        "current_root",
        errors,
    );
    let source_path = scalar(pyjson::get(source, "path"));
    if !safe_relative_path(&source_path) {
        return;
    }
    let path = base.join(&source_path);
    if !path.is_file() {
        errors.push(format!("current feedback root '{source_path}' is missing"));
        return;
    }
    let raw = match read_raw(&path) {
        Ok(raw) => raw,
        Err(error) => {
            errors.push(error);
            return;
        }
    };
    let text = decoded(&raw, &source_path).unwrap_or_default();
    let actual = metrics(&raw);
    let limits = required_hash(
        pyjson::get(current, "enforcement_ceilings"),
        "current_root.enforcement_ceilings",
        errors,
    );
    for (key, value) in [
        ("lines", actual.lines),
        ("bytes", actual.bytes),
        ("line_bytes", actual.line_bytes),
    ] {
        if let Some(limit) = pyjson::get(limits, key) {
            let limit = perl::numeric(Some(limit)) as usize;
            if value > limit {
                errors.push(format!(
                    "current feedback root exceeds {key} ceiling: {value} > {limit}"
                ));
            }
        }
    }
    if text.split('\n').next().unwrap_or("") != scalar(pyjson::get(current, "h1")) {
        errors.push("current feedback root H1 differs".to_string());
    }
    let h2: Vec<&str> = text
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .collect();
    let expected_h2 = required_array(
        pyjson::get(current, "required_h2_order"),
        "current_root.required_h2_order",
        errors,
    );
    let expected_h2: Vec<String> = expected_h2.iter().map(|item| scalar(Some(item))).collect();
    if h2.join("\n") != expected_h2.join("\n") {
        errors.push("current feedback root H2 order differs".to_string());
    }
    for field in ["required_links", "required_literals"] {
        for literal in required_array(
            pyjson::get(current, field),
            &format!("current_root.{field}"),
            errors,
        ) {
            let literal = scalar(Some(literal));
            if count_literal(&text, &literal) < 1 {
                errors.push(format!(
                    "current feedback root lacks required literal '{literal}'"
                ));
            }
        }
    }
    for literal in required_array(
        pyjson::get(current, "forbidden_literals"),
        "current_root.forbidden_literals",
        errors,
    ) {
        let literal = scalar(Some(literal));
        if count_literal(&text, &literal) > 0 {
            errors.push(format!(
                "current feedback root retains forbidden historical literal '{literal}'"
            ));
        }
    }

    let open = between_markers(
        &text,
        &scalar(pyjson::get(current, "open_start_marker")),
        &scalar(pyjson::get(current, "open_end_marker")),
        "open correspondence",
        errors,
    );
    let open_ids: Vec<&str> = open.lines().filter_map(open_id).collect();
    let expected_open = required_array(
        pyjson::get(current, "open_record_ids"),
        "current_root.open_record_ids",
        errors,
    );
    let expected_open: Vec<String> = expected_open
        .iter()
        .map(|item| scalar(Some(item)))
        .collect();
    if open_ids.join("\n") != expected_open.join("\n") {
        errors.push("current feedback open-record ids differ".to_string());
    }
    let open_limits = required_hash(
        pyjson::get(current, "open_record_limits"),
        "current_root.open_record_limits",
        errors,
    );
    if let Some(limit) = pyjson::get(open_limits, "records") {
        let limit = perl::numeric(Some(limit)) as usize;
        if open_ids.len() > limit {
            errors.push("current feedback has too many open records".to_string());
        }
    }
    let allowed = |field, errors: &mut Vec<String>| {
        required_array(
            pyjson::get(current, field),
            &format!("current_root.{field}"),
            errors,
        )
        .iter()
        .map(|item| scalar(Some(item)))
        .collect::<BTreeSet<String>>()
    };
    let allowed_direction = allowed("open_direction_values", errors);
    let allowed_kind = allowed("open_kind_values", errors);
    let allowed_status = allowed("open_status_values", errors);
    for chunk in h3_chunks(open) {
        let id = open_id(chunk.lines().next().unwrap_or("")).unwrap_or("<unknown>");
        let actual = metrics(chunk.as_bytes());
        for (actual_key, limit_key) in [
            ("lines", "lines_each"),
            ("bytes", "bytes_each"),
            ("line_bytes", "line_bytes_each"),
        ] {
            let value = match actual_key {
                "lines" => actual.lines,
                "bytes" => actual.bytes,
                _ => actual.line_bytes,
            };
            if let Some(limit) = pyjson::get(open_limits, limit_key) {
                let limit = perl::numeric(Some(limit)) as usize;
                if value > limit {
                    errors.push(format!("open record '{id}' exceeds {limit_key}"));
                }
            }
        }
        let mut fields = BTreeMap::new();
        for line in chunk.lines().filter_map(|line| line.strip_prefix("- ")) {
            for pair in line.split(" · ") {
                if let Some((key, value)) = pair.split_once(':') {
                    fields.insert(
                        key.to_string(),
                        value
                            .trim()
                            .trim_start_matches('`')
                            .trim_end_matches('`')
                            .to_string(),
                    );
                }
            }
        }
        for field in required_array(
            pyjson::get(current, "open_required_fields"),
            "current_root.open_required_fields",
            errors,
        ) {
            let field = scalar(Some(field));
            if fields.get(&field).is_none_or(|value| value.is_empty()) {
                errors.push(format!("open record '{id}' lacks {field}"));
            }
        }
        for (field, allowed) in [
            ("Direction", &allowed_direction),
            ("Kind", &allowed_kind),
            ("Status", &allowed_status),
        ] {
            if let Some(value) = fields.get(field)
                && !allowed.contains(value)
            {
                errors.push(format!("open record '{id}' has unknown {field}"));
            }
        }
    }

    let register = between_markers(
        &text,
        &scalar(pyjson::get(current, "register_start_marker")),
        &scalar(pyjson::get(current, "register_end_marker")),
        "closed correspondence register",
        errors,
    );
    let mut actual_rows = Vec::new();
    for line in register.lines().filter(|line| line.starts_with("| ")) {
        let fields: Vec<&str> = line.split(" | ").collect();
        if fields.len() >= 3 && valid_kebab(fields[0].trim_start_matches("| ")) {
            let status = fields[1];
            if !status.is_empty()
                && status
                    .chars()
                    .next()
                    .is_some_and(|character| character.is_ascii_lowercase())
                && status.chars().all(|character| {
                    character.is_ascii_lowercase() || character.is_ascii_digit() || character == '_'
                })
            {
                actual_rows.push((
                    fields[0].trim_start_matches("| ").to_string(),
                    status.to_string(),
                    fields[2].to_string(),
                ));
            }
        }
    }
    let records = required_array(pyjson::get(contract, "records"), "records", errors);
    let expected_rows: Vec<(String, String, String)> = records
        .iter()
        .map(|record| {
            (
                scalar(pyjson::get(record, "id")),
                scalar(pyjson::get(record, "status")),
                direction_display(&scalar(pyjson::get(record, "direction"))),
            )
        })
        .collect();
    if actual_rows.len() != expected_rows.len() {
        errors.push("current feedback closed-register row count differs".to_string());
    }
    for (index, expected) in expected_rows.iter().enumerate() {
        let Some(actual) = actual_rows.get(index) else {
            break;
        };
        if actual != expected {
            errors.push(format!(
                "current feedback closed-register row {index} differs"
            ));
        }
    }
}

fn validate_archive(base: &Path, contract: &Py, errors: &mut Vec<String>) {
    let source = required_hash(pyjson::get(contract, "source"), "source", errors);
    let archive = required_hash(pyjson::get(contract, "archive"), "archive", errors);
    let index_path = scalar(pyjson::get(archive, "index"));
    let manifest_path = scalar(pyjson::get(archive, "manifest"));
    if !safe_relative_path(&index_path) || !safe_relative_path(&manifest_path) {
        return;
    }
    let index_absolute = base.join(&index_path);
    if !index_absolute.is_file() {
        errors.push(format!("feedback archive index '{index_path}' is missing"));
    } else if let Ok(raw) = read_raw(&index_absolute) {
        let text = decoded(&raw, &index_path).unwrap_or_default();
        let actual = metrics(&raw);
        let limits = required_hash(
            pyjson::get(archive, "index_limits"),
            "archive.index_limits",
            errors,
        );
        for (key, value) in [
            ("lines", actual.lines),
            ("bytes", actual.bytes),
            ("line_bytes", actual.line_bytes),
        ] {
            if let Some(limit) = pyjson::get(limits, key) {
                let limit = perl::numeric(Some(limit)) as usize;
                if value > limit {
                    errors.push(format!("feedback archive index exceeds {key} ceiling"));
                }
            }
        }
        for literal in [
            scalar(pyjson::get(source, "path")),
            Path::new(&scalar(pyjson::get(archive, "source_capsule")))
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
            Path::new(&manifest_path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned(),
        ] {
            if !text.contains(&literal) {
                errors.push(format!("feedback archive index lacks '{literal}'"));
            }
        }
    }
    let manifest_absolute = base.join(&manifest_path);
    if !manifest_absolute.is_file() {
        errors.push(format!(
            "feedback archive manifest '{manifest_path}' is missing"
        ));
        return;
    }
    let manifest = match load_json(&manifest_absolute) {
        Ok(manifest @ Py::Dict(_)) => manifest,
        _ => {
            errors.push("feedback archive manifest is invalid JSON".to_string());
            return;
        }
    };
    let expected = [
        ("schema_version", integer(1)),
        ("current_path", text(&scalar(pyjson::get(source, "path")))),
        (
            "source_capsule",
            text(&scalar(pyjson::get(archive, "source_capsule"))),
        ),
        (
            "source_sha256",
            text(&scalar(pyjson::get(source, "sha256"))),
        ),
        (
            "sealed_date",
            text(&scalar(pyjson::get(archive, "sealed_date"))),
        ),
        ("reason", text(&scalar(pyjson::get(archive, "reason")))),
        ("verifier", text(&scalar(pyjson::get(archive, "verifier")))),
    ];
    for (key, expected_value) in &expected {
        let actual = pyjson::get(&manifest, key);
        if actual.is_none_or(perl::is_reference) || scalar(actual) != scalar(Some(expected_value)) {
            errors.push(format!("feedback archive manifest {key} differs"));
        }
    }
    let source_metrics = required_hash(
        pyjson::get(&manifest, "source_metrics"),
        "manifest.source_metrics",
        errors,
    );
    for key in ["lines", "bytes", "line_bytes"] {
        if pyjson::get(source_metrics, key).is_none()
            || perl::numeric(pyjson::get(source_metrics, key))
                != perl::numeric(pyjson::get(source, key))
        {
            errors.push(format!(
                "feedback archive manifest source_metrics.{key} differs"
            ));
        }
    }
    if let Py::Dict(pairs) = &manifest {
        for (key, _) in pairs {
            let key = pyjson::to_text(key);
            if key != "source_metrics"
                && !expected
                    .iter()
                    .any(|(expected_key, _)| *expected_key == key)
            {
                errors.push(format!(
                    "feedback archive manifest has unknown field '{key}'"
                ));
            }
        }
    }
}

fn validate_contract(base: &Path, contract: &Py) -> Vec<String> {
    let mut errors = Vec::new();
    if !matches!(contract, Py::Dict(_)) {
        return vec!["contract root must be an object".to_string()];
    }
    if perl::numeric(pyjson::get(contract, "schema_version")) != 1.0 {
        errors.push("schema_version must be 1".to_string());
    }
    let state = scalar(pyjson::get(contract, "migration_state"));
    if !matches!(state.as_str(), "planned" | "migrated") {
        errors.push(format!("migration_state '{state}' is invalid"));
    }
    validate_paths(contract, &mut errors);
    validate_source(base, contract, &mut errors);
    validate_consumers(base, contract, &mut errors);
    if state == "migrated" {
        validate_current_root(base, contract, &mut errors);
        validate_archive(base, contract, &mut errors);
    }
    errors
}

fn write_raw(path: &Path, raw: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    fs::write(path, raw).map_err(|error| {
        format!(
            "{LABEL} self-test: cannot write {}: {error}",
            path.display()
        )
    })
}

fn write_json(path: &Path, value: &Py) -> Result<(), String> {
    write_raw(path, encode(value, Some(2))?.as_bytes())
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

fn nested_mut<'a>(mut value: &'a mut Py, path: &[&str]) -> &'a mut Py {
    for key in path {
        if let Py::Dict(pairs) = value {
            value = pairs
                .iter_mut()
                .find(|(candidate, _)| *candidate == pyjson::py_str(key))
                .map(|(_, item)| item)
                .expect("fixture field");
        } else {
            panic!("fixture parent is not an object");
        }
    }
    value
}

fn render_current_root(contract: &Py) -> String {
    let current = pyjson::get(contract, "current_root").unwrap_or(&Py::None);
    let required = required_array(
        pyjson::get(current, "required_literals"),
        "current_root.required_literals",
        &mut Vec::new(),
    );
    let mut out = vec![scalar(pyjson::get(current, "h1")), String::new()];
    out.extend([
        "## Current channel".to_string(),
        String::new(),
        "The stable channel is `docs/FSMGEN_FEEDBACK.md`; detailed history is in docs/archive/fsmgen-feedback/INDEX.md.".to_string(),
        String::new(),
        "## Current downstream boundary".to_string(),
        String::new(),
        "SPECFORGE's single adapter target is now `.isf`.".to_string(),
    ]);
    out.extend(required.iter().map(|item| scalar(Some(item))));
    out.push(String::new());
    out.extend([
        "## Open correspondence".to_string(),
        String::new(),
        scalar(pyjson::get(current, "open_start_marker")),
    ]);
    let open_ids = required_array(
        pyjson::get(current, "open_record_ids"),
        "current_root.open_record_ids",
        &mut Vec::new(),
    );
    if open_ids.is_empty() {
        out.push("- None at the sealed source boundary.".to_string());
    } else {
        let directions = required_array(
            pyjson::get(current, "open_direction_values"),
            "current_root.open_direction_values",
            &mut Vec::new(),
        );
        let kinds = required_array(
            pyjson::get(current, "open_kind_values"),
            "current_root.open_kind_values",
            &mut Vec::new(),
        );
        let statuses = required_array(
            pyjson::get(current, "open_status_values"),
            "current_root.open_status_values",
            &mut Vec::new(),
        );
        for id in open_ids {
            let id = scalar(Some(id));
            out.push(format!("### {id} — fixture record"));
            out.push(String::new());
            out.push(format!(
                "- Direction: `{}` · Kind: `{}` · Status: `{}` · Owner: `FIXTURE-OWNER` · Evidence: `docs/FSMGEN_FEEDBACK.md`",
                scalar(directions.first()),
                scalar(kinds.first()),
                scalar(statuses.first())
            ));
            out.push(String::new());
        }
    }
    out.extend([
        scalar(pyjson::get(current, "open_end_marker")),
        String::new(),
        "## Closed correspondence register".to_string(),
        String::new(),
        scalar(pyjson::get(current, "register_start_marker")),
        "| Record | Status | Direction | History |".to_string(),
        "| --- | --- | --- | --- |".to_string(),
    ]);
    for record in required_array(pyjson::get(contract, "records"), "records", &mut Vec::new()) {
        out.push(format!(
            "| {} | {} | {} | history |",
            scalar(pyjson::get(record, "id")),
            scalar(pyjson::get(record, "status")),
            direction_display(&scalar(pyjson::get(record, "direction")))
        ));
    }
    out.extend([
        scalar(pyjson::get(current, "register_end_marker")),
        String::new(),
        "## Feedback record format".to_string(),
        String::new(),
        "New records are task-owned, evidence-linked, explicitly directed, and status-bearing. See TOOLBOX.md.".to_string(),
        String::new(),
        "## History and retrieval".to_string(),
        String::new(),
        "- docs/archive/fsmgen-feedback/INDEX.md".to_string(),
        "- subs/fsmgen/docs/SPECFORGE_FEEDBACK_RESPONSE.md".to_string(),
        "- docs/catalogs/fsmgen-issue-packets.md".to_string(),
        "- TOOLBOX.md".to_string(),
        String::new(),
    ]);
    out.join("\n")
}

fn seed_fixture(fixture: &Path, contract: &Py, source_raw: &[u8]) -> Result<(), String> {
    let mut content: BTreeMap<String, String> = BTreeMap::new();
    for record in required_array(pyjson::get(contract, "records"), "records", &mut Vec::new()) {
        for evidence in required_array(
            pyjson::get(record, "evidence"),
            "record evidence",
            &mut Vec::new(),
        ) {
            let path = scalar(pyjson::get(evidence, "path"));
            let literal = scalar(pyjson::get(evidence, "literal"));
            content
                .entry(path)
                .or_default()
                .push_str(&format!("{literal}\n"));
        }
    }
    for finding in required_array(
        pyjson::get(contract, "current_truth_findings"),
        "current_truth_findings",
        &mut Vec::new(),
    ) {
        let path = scalar(pyjson::get(finding, "evidence_path"));
        if path
            == scalar(
                pyjson::get(contract, "source").and_then(|source| pyjson::get(source, "path")),
            )
        {
            continue;
        }
        let literal = scalar(pyjson::get(finding, "evidence_literal"));
        content
            .entry(path)
            .or_default()
            .push_str(&format!("{literal}\n"));
    }
    let consumers = pyjson::get(contract, "consumers").unwrap_or(&Py::None);
    for path in required_array(
        pyjson::get(consumers, "known_paths"),
        "consumers.known_paths",
        &mut Vec::new(),
    ) {
        content
            .entry(scalar(Some(path)))
            .or_default()
            .push_str("docs/FSMGEN_FEEDBACK.md\n");
    }
    for item in required_array(
        pyjson::get(consumers, "required_literals"),
        "consumers.required_literals",
        &mut Vec::new(),
    ) {
        let path = scalar(pyjson::get(item, "path"));
        let literal = scalar(pyjson::get(item, "literal"));
        content
            .entry(path)
            .or_default()
            .push_str(&format!("{literal}\n"));
    }
    for (path, content) in content {
        write_raw(&fixture.join(path), content.as_bytes())?;
    }
    write_raw(
        &fixture.join(scalar(
            pyjson::get(contract, "source").and_then(|source| pyjson::get(source, "path")),
        )),
        source_raw,
    )
}

fn seed_migrated_files(fixture: &Path, contract: &Py, source_raw: &[u8]) -> Result<(), String> {
    let archive = pyjson::get(contract, "archive").unwrap_or(&Py::None);
    let source = pyjson::get(contract, "source").unwrap_or(&Py::None);
    write_raw(
        &fixture.join(scalar(pyjson::get(archive, "source_capsule"))),
        source_raw,
    )?;
    write_raw(
        &fixture.join(scalar(pyjson::get(source, "path"))),
        render_current_root(contract).as_bytes(),
    )?;
    let index = format!(
        "# Archive\n\n- {}\n- {}\n- {}\n",
        scalar(pyjson::get(source, "path")),
        Path::new(&scalar(pyjson::get(archive, "source_capsule")))
            .file_name()
            .unwrap_or_default()
            .to_string_lossy(),
        Path::new(&scalar(pyjson::get(archive, "manifest")))
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
    );
    write_raw(
        &fixture.join(scalar(pyjson::get(archive, "index"))),
        index.as_bytes(),
    )?;
    let manifest = object(vec![
        ("schema_version", integer(1)),
        ("current_path", text(&scalar(pyjson::get(source, "path")))),
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
            object(vec![
                (
                    "lines",
                    pyjson::get(source, "lines").unwrap_or(&Py::None).clone(),
                ),
                (
                    "bytes",
                    pyjson::get(source, "bytes").unwrap_or(&Py::None).clone(),
                ),
                (
                    "line_bytes",
                    pyjson::get(source, "line_bytes")
                        .unwrap_or(&Py::None)
                        .clone(),
                ),
            ]),
        ),
        (
            "sealed_date",
            text(&scalar(pyjson::get(archive, "sealed_date"))),
        ),
        ("reason", text(&scalar(pyjson::get(archive, "reason")))),
        ("verifier", text(&scalar(pyjson::get(archive, "verifier")))),
    ]);
    write_json(
        &fixture.join(scalar(pyjson::get(archive, "manifest"))),
        &manifest,
    )
}

#[derive(Clone, Copy)]
enum Mutation {
    ValidPlanned,
    SourceIdentityDrift,
    MissingResponseEvidence,
    RecordHeadingDrift,
    UnsafeArchivePath,
    ValidMigrated,
    DuplicateOpenMarker,
    MissingClosedRegisterRow,
    ForbiddenHistoricalLeakage,
    InlineOwnerMissing,
    InlineUnknownStatus,
    ManifestIdentityDrift,
}

const PLANNED_CASES: &[(&str, bool, Mutation)] = &[
    ("valid planned contract", true, Mutation::ValidPlanned),
    (
        "source identity drift",
        false,
        Mutation::SourceIdentityDrift,
    ),
    (
        "missing response evidence",
        false,
        Mutation::MissingResponseEvidence,
    ),
    ("record heading drift", false, Mutation::RecordHeadingDrift),
    ("unsafe archive path", false, Mutation::UnsafeArchivePath),
];

const MIGRATED_CASES: &[(&str, bool, Mutation)] = &[
    ("valid migrated contract", true, Mutation::ValidMigrated),
    (
        "duplicate open marker",
        false,
        Mutation::DuplicateOpenMarker,
    ),
    (
        "missing closed-register row",
        false,
        Mutation::MissingClosedRegisterRow,
    ),
    (
        "forbidden historical leakage",
        false,
        Mutation::ForbiddenHistoricalLeakage,
    ),
    (
        "inline field line lacking its owner",
        false,
        Mutation::InlineOwnerMissing,
    ),
    (
        "inline field line with an unknown status",
        false,
        Mutation::InlineUnknownStatus,
    ),
    (
        "manifest identity drift",
        false,
        Mutation::ManifestIdentityDrift,
    ),
];

struct Fixture(PathBuf);

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run_self_test(base: &Path, contract: &Py) -> Result<usize, String> {
    let archive = pyjson::get(contract, "archive").unwrap_or(&Py::None);
    let source = pyjson::get(contract, "source").unwrap_or(&Py::None);
    let state = scalar(pyjson::get(contract, "migration_state"));
    let authority = if state == "migrated" {
        scalar(pyjson::get(archive, "source_capsule"))
    } else {
        scalar(pyjson::get(source, "path"))
    };
    let source_raw = read_raw(&base.join(&authority))?;
    let fixture_root = base.join(format!(
        "generated/.fsmgen-feedback-protocol-test.{}",
        std::process::id()
    ));
    if fixture_root.exists() {
        fs::remove_dir_all(&fixture_root).map_err(|error| error.to_string())?;
    }
    let _guard = Fixture(fixture_root.clone());
    fs::create_dir_all(&fixture_root).map_err(|error| error.to_string())?;
    let mut passed = 0;

    for (index, (name, expected_ok, mutation)) in PLANNED_CASES.iter().enumerate() {
        let mut contract = contract.clone();
        set_field(&mut contract, "migration_state", text("planned"));
        let fixture = fixture_root.join(format!("planned-{index}"));
        fs::create_dir_all(&fixture).map_err(|error| error.to_string())?;
        seed_fixture(&fixture, &contract, &source_raw)?;
        let records = pyjson::get(&contract, "records")
            .unwrap_or(&Py::None)
            .clone();
        match mutation {
            Mutation::ValidPlanned => {}
            Mutation::SourceIdentityDrift => {
                let path = scalar(
                    pyjson::get(&contract, "source").and_then(|source| pyjson::get(source, "path")),
                );
                let mut raw = source_raw.clone();
                raw.extend_from_slice(b"drift\n");
                write_raw(&fixture.join(path), &raw)?;
            }
            Mutation::MissingResponseEvidence => {
                let first = match &records {
                    Py::List(items) => &items[0],
                    _ => return Err("fixture records are missing".to_string()),
                };
                let evidence = match pyjson::get(first, "evidence") {
                    Some(Py::List(items)) => &items[0],
                    _ => return Err("fixture evidence is missing".to_string()),
                };
                write_raw(
                    &fixture.join(scalar(pyjson::get(evidence, "path"))),
                    b"unrelated\n",
                )?;
            }
            Mutation::RecordHeadingDrift => {
                *nested_mut(&mut contract, &["records"]) = {
                    let mut records = records.clone();
                    if let Py::List(items) = &mut records {
                        set_field(&mut items[0], "heading", text("## Missing heading"));
                    }
                    records
                };
            }
            Mutation::UnsafeArchivePath => {
                *nested_mut(&mut contract, &["archive", "index"]) = text("../outside.md");
            }
            _ => return Err("invalid planned fixture mutation".to_string()),
        }
        let errors = validate_contract(&fixture, &contract);
        if *expected_ok != errors.is_empty() {
            return Err(format!(
                "feedback-protocol self-test '{name}' failed: {}",
                errors.join("; ")
            ));
        }
        passed += 1;
    }

    let mut changed = contract.clone();
    set_field(&mut changed, "migration_state", text("planned"));
    let required = Py::List(vec![
        text("fixture-declared adapter route"),
        text("fixture-declared downstream boundary"),
    ]);
    *nested_mut(&mut changed, &["current_root", "required_literals"]) = required;
    let fixture = fixture_root.join("changed-literals");
    fs::create_dir_all(&fixture).map_err(|error| error.to_string())?;
    seed_fixture(&fixture, &changed, &source_raw)?;
    let errors = validate_contract(&fixture, &changed);
    if !errors.is_empty() {
        return Err(format!(
            "feedback-protocol self-test 'declared current literals drive fixture rendering' failed: {}",
            errors.join("; ")
        ));
    }
    passed += 1;

    for (index, (name, expected_ok, mutation)) in MIGRATED_CASES.iter().enumerate() {
        let mut contract = contract.clone();
        set_field(&mut contract, "migration_state", text("migrated"));
        let fixture = fixture_root.join(format!("migrated-{index}"));
        fs::create_dir_all(&fixture).map_err(|error| error.to_string())?;
        seed_fixture(&fixture, &contract, &source_raw)?;
        seed_migrated_files(&fixture, &contract, &source_raw)?;
        match mutation {
            Mutation::ValidMigrated => {}
            Mutation::DuplicateOpenMarker => {
                let source = pyjson::get(&contract, "source").unwrap_or(&Py::None);
                let current = pyjson::get(&contract, "current_root").unwrap_or(&Py::None);
                let path = fixture.join(scalar(pyjson::get(source, "path")));
                let mut raw = fs::read(&path).map_err(|error| error.to_string())?;
                raw.extend_from_slice(scalar(pyjson::get(current, "open_start_marker")).as_bytes());
                raw.push(b'\n');
                write_raw(&path, &raw)?;
            }
            Mutation::MissingClosedRegisterRow => {
                let source = pyjson::get(&contract, "source").unwrap_or(&Py::None);
                let path = fixture.join(scalar(pyjson::get(source, "path")));
                let raw = fs::read_to_string(&path).map_err(|error| error.to_string())?;
                let changed = raw
                    .lines()
                    .filter(|line| !line.starts_with("| enum-type-clarity |"))
                    .map(|line| format!("{line}\n"))
                    .collect::<String>();
                write_raw(&path, changed.as_bytes())?;
            }
            Mutation::ForbiddenHistoricalLeakage => {
                let source = pyjson::get(&contract, "source").unwrap_or(&Py::None);
                let current = pyjson::get(&contract, "current_root").unwrap_or(&Py::None);
                let forbidden = required_array(
                    pyjson::get(current, "forbidden_literals"),
                    "current_root.forbidden_literals",
                    &mut Vec::new(),
                );
                let path = fixture.join(scalar(pyjson::get(source, "path")));
                let mut raw = fs::read(&path).map_err(|error| error.to_string())?;
                raw.extend_from_slice(scalar(forbidden.first()).as_bytes());
                raw.push(b'\n');
                write_raw(&path, &raw)?;
            }
            Mutation::InlineOwnerMissing => {
                let source = pyjson::get(&contract, "source").unwrap_or(&Py::None);
                let path = fixture.join(scalar(pyjson::get(source, "path")));
                let raw = fs::read_to_string(&path).map_err(|error| error.to_string())?;
                let changed = raw.replace(" · Owner: `FIXTURE-OWNER`", "");
                if changed == raw {
                    return Err("fixture has no inline owner".to_string());
                }
                write_raw(&path, changed.as_bytes())?;
            }
            Mutation::InlineUnknownStatus => {
                let source = pyjson::get(&contract, "source").unwrap_or(&Py::None);
                let path = fixture.join(scalar(pyjson::get(source, "path")));
                let raw = fs::read_to_string(&path).map_err(|error| error.to_string())?;
                let start = raw
                    .find(" · Status: `")
                    .ok_or_else(|| "fixture has no inline status".to_string())?;
                let end = raw[start + 11..]
                    .find('`')
                    .map(|index| index + start + 11)
                    .ok_or_else(|| "fixture has no inline status".to_string())?;
                let changed = format!("{} · Status: `bogus`{}", &raw[..start], &raw[end + 1..]);
                write_raw(&path, changed.as_bytes())?;
            }
            Mutation::ManifestIdentityDrift => {
                let archive = pyjson::get(&contract, "archive").unwrap_or(&Py::None);
                let path = fixture.join(scalar(pyjson::get(archive, "manifest")));
                let mut manifest = load_json(&path)?;
                set_field(&mut manifest, "source_sha256", text(&"0".repeat(64)));
                write_json(&path, &manifest)?;
            }
            _ => return Err("invalid migrated fixture mutation".to_string()),
        }
        let errors = validate_contract(&fixture, &contract);
        if *expected_ok != errors.is_empty() {
            return Err(format!(
                "feedback-protocol self-test '{name}' failed: {}",
                errors.join("; ")
            ));
        }
        passed += 1;
    }
    if passed != 13 {
        return Err(format!(
            "feedback-protocol self-test ran {passed} assertions, declaration expects 13"
        ));
    }
    Ok(passed)
}

fn report(base: &Path, contract: &Py) -> Result<String, String> {
    let source = pyjson::get(contract, "source").unwrap_or(&Py::None);
    let records = required_array(pyjson::get(contract, "records"), "records", &mut Vec::new());
    let mut status_counts = BTreeMap::new();
    for record in records {
        *status_counts
            .entry(scalar(pyjson::get(record, "status")))
            .or_insert(0) += 1;
    }
    let current_root = pyjson::get(contract, "current_root").unwrap_or(&Py::None);
    let open_count = match pyjson::get(current_root, "open_record_ids") {
        Some(Py::List(items)) => items.len(),
        _ => 0,
    };
    let finding_count = match pyjson::get(contract, "current_truth_findings") {
        Some(Py::List(items)) => items.len(),
        _ => 0,
    };
    let consumer_count = match pyjson::get(contract, "consumers")
        .and_then(|consumers| pyjson::get(consumers, "known_paths"))
    {
        Some(Py::List(items)) => items.len(),
        _ => 0,
    };
    let authority = if scalar(pyjson::get(contract, "migration_state")) == "migrated" {
        scalar(
            pyjson::get(contract, "archive")
                .and_then(|archive| pyjson::get(archive, "source_capsule")),
        )
    } else {
        scalar(pyjson::get(source, "path"))
    };
    let source_raw = read_raw(&base.join(authority))?;
    let mut fields = vec![
        (
            "migration_state",
            pyjson::get(contract, "migration_state")
                .unwrap_or(&Py::None)
                .clone(),
        ),
        ("source", metrics_json(metrics(&source_raw))),
        (
            "regions",
            pyjson::get(source, "regions").unwrap_or(&Py::None).clone(),
        ),
        ("records", integer(records.len())),
        ("open_records", integer(open_count)),
        (
            "status_counts",
            Py::Dict(
                status_counts
                    .into_iter()
                    .map(|(key, value)| (pyjson::py_str(&key), integer(value)))
                    .collect(),
            ),
        ),
        ("current_truth_findings", integer(finding_count)),
        ("consumers", integer(consumer_count)),
        (
            "archive",
            pyjson::get(contract, "archive")
                .unwrap_or(&Py::None)
                .clone(),
        ),
        (
            "current_root_limits",
            pyjson::get(current_root, "enforcement_ceilings")
                .unwrap_or(&Py::None)
                .clone(),
        ),
    ];
    if scalar(pyjson::get(contract, "migration_state")) == "migrated" {
        let path = base.join(scalar(pyjson::get(source, "path")));
        fields.push(("current_root", metrics_json(metrics(&read_raw(&path)?))));
    }
    encode(&object(fields), None)
}

/// Run the FSMGen feedback protocol with its original mode and path arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut root = root.to_path_buf();
    let mut contract_path = "doctrine/live_document_size/fsmgen_feedback.json".to_string();
    let mut mode = Mode::Check;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--root" => match arguments.next() {
                Some(directory) => root = PathBuf::from(directory),
                None => {
                    output.err("Usage: cargo xtask check fsmgen-feedback-protocol [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n");
                    return 2;
                }
            },
            "--contract" => match arguments.next() {
                Some(path) => contract_path = path.to_string(),
                None => {
                    output.err("Usage: cargo xtask check fsmgen-feedback-protocol [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n");
                    return 2;
                }
            },
            "--check" => mode = Mode::Check,
            "--report" => mode = Mode::Report,
            "--self-test" => mode = Mode::SelfTest,
            _ => {
                output.err("Usage: cargo xtask check fsmgen-feedback-protocol [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n");
                return 2;
            }
        }
    }
    let root = match fs::canonicalize(&root) {
        Ok(root) => root,
        Err(_) => {
            output.err(&format!("{LABEL}: invalid --root\n"));
            return 1;
        }
    };
    let contract_absolute = root.join(&contract_path);
    if !contract_absolute.is_file() {
        output.err(&format!("{LABEL}: contract '{contract_path}' is missing\n"));
        return 2;
    }
    let contract = match load_json(&contract_absolute) {
        Ok(contract) => contract,
        Err(error) => {
            output.err(&format!("{error}\n"));
            return 255;
        }
    };
    if mode == Mode::SelfTest {
        return match run_self_test(&root, &contract) {
            Ok(passed) => {
                output.out(&format!("{LABEL}: self-test {passed}/13 passed.\n"));
                0
            }
            Err(error) => {
                output.err(&format!("{error}\n"));
                1
            }
        };
    }
    let errors = validate_contract(&root, &contract);
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
    if mode == Mode::Report {
        match report(&root, &contract) {
            Ok(report) => output.out(&format!("{report}\n")),
            Err(error) => {
                output.err(&format!("{error}\n"));
                return 1;
            }
        }
    } else {
        output.out(&format!(
            "{LABEL}: exact source, six closed exchanges, evidence, consumers, and {} lifecycle satisfy the bounded channel design.\n",
            scalar(pyjson::get(&contract, "migration_state"))
        ));
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_thirteen_cases_pass_without_residue() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--self-test".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert_eq!(output.text, "feedback-protocol: self-test 13/13 passed.\n");
        assert!(
            !root
                .join(format!(
                    "generated/.fsmgen-feedback-protocol-test.{}",
                    std::process::id()
                ))
                .exists()
        );
    }

    #[test]
    fn the_real_migrated_channel_and_report_hold() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--report".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("\"migration_state\":\"migrated\""));
        assert!(output.text.contains("\"records\":6"));
        assert!(output.text.contains("\"current_root\":"));
    }
}
