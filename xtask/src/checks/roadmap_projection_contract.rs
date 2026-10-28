//! Roadmap projection contract, native since `RUSTDOC-AND-XTASK.15.s`: exact source reconstruction,
//! bounded current roots, archive/rollover identity, section limits and the 49-case suite.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "roadmap-projection";

#[derive(Clone, Copy, Default)]
struct Metrics {
    lines: usize,
    bytes: usize,
    line_bytes: usize,
}

struct SectionCount {
    heading: String,
    lines: usize,
}

struct ReportData {
    state: String,
    source_metrics: Metrics,
    source_sha: String,
    regions: Py,
    workstreams: usize,
    done_blocks: Metrics,
    drift_findings: usize,
    current_root_limits: Py,
    archive: Py,
    rollovers: Py,
    current_root: Option<Metrics>,
    sections: Option<Vec<SectionCount>>,
    warnings: Vec<String>,
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

fn encode(value: &Py) -> Result<String, String> {
    pyjson::dumps(
        value,
        &pyjson::Encoder {
            indent: None,
            sort_keys: true,
            ensure_ascii: false,
            item_separator: ",",
            key_separator: ":",
        },
    )
    .ok_or_else(|| "cannot encode report".to_string())
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

fn safe_relative(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && !path.contains("//")
        && !path.split('/').any(|part| matches!(part, "." | ".."))
        && !path
            .chars()
            .any(|character| character <= '\u{1f}' || character == '\u{7f}')
}

fn safe_markdown(path: &str, label: &str, errors: &mut Vec<String>) {
    if !safe_relative(path) || !path.ends_with(".md") {
        errors.push(format!(
            "{label} '{path}' is not a safe repository-relative Markdown path"
        ));
    }
}

fn read_raw(path: &Path, label: &str, errors: &mut Vec<String>) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(raw) => Some(raw),
        Err(error) => {
            errors.push(format!("cannot read {label}: {}", os_error(&error)));
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

fn metrics_json(metrics: Metrics) -> Py {
    object(vec![
        ("lines", integer(metrics.lines)),
        ("bytes", integer(metrics.bytes)),
        ("line_bytes", integer(metrics.line_bytes)),
    ])
}

fn physical_lines(raw: &str) -> Vec<&str> {
    raw.split_inclusive('\n').collect()
}

fn reject_unknown(value: &Py, label: &str, allowed: &[&str], errors: &mut Vec<String>) {
    let Py::Dict(pairs) = value else {
        errors.push(format!("{label} must be an object"));
        return;
    };
    for (key, _) in pairs {
        let key = pyjson::to_text(key);
        if !allowed.contains(&key.as_str()) {
            errors.push(format!("{label} has unknown field '{key}'"));
        }
    }
}

fn required_string(
    value: &Py,
    field: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    let item = pyjson::get(value, field);
    if item.is_none_or(perl::is_reference) || scalar(item).is_empty() {
        errors.push(format!("{label} lacks nonempty string '{field}'"));
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

fn require_array<'a>(
    value: &'a Py,
    field: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> &'a [Py] {
    let value = pyjson::get(value, field).unwrap_or(&Py::None);
    if let Py::List(items) = value {
        items
    } else {
        errors.push(format!("{label} '{field}' must be an array"));
        &[]
    }
}

fn validate_hash(value: Option<&Py>, label: &str, errors: &mut Vec<String>) {
    let value = scalar(value);
    if value.len() != 64
        || !value
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
    {
        errors.push(format!("{label} must be a lowercase SHA-256"));
    }
}

fn validate_unique_strings(items: &[Py], label: &str, errors: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    for item in items {
        let item = scalar(Some(item));
        if item.is_empty() {
            errors.push(format!("{label} contains a non-string value"));
        }
        if !seen.insert(item.clone()) {
            errors.push(format!("{label} contains duplicate '{item}'"));
        }
    }
}

fn valid_workstream_id(id: &str) -> bool {
    id.strip_prefix('R').is_some_and(|suffix| {
        (suffix.len() == 3
            && suffix.starts_with("15")
            && suffix
                .chars()
                .last()
                .is_some_and(|character| ('b'..='g').contains(&character)))
            || (!suffix.is_empty() && suffix.chars().all(|character| character.is_ascii_digit()))
    })
}

fn workstream_records(raw: &str) -> Vec<(String, String)> {
    let mut headings = Vec::new();
    let mut offset = 0;
    for line in physical_lines(raw) {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        let level = if trimmed.starts_with("### ") {
            3
        } else if trimmed.starts_with("## ") {
            2
        } else {
            0
        };
        if level > 0 {
            headings.push((level, &trimmed[level + 1..], offset));
        }
        offset += line.len();
    }
    let mut records = Vec::new();
    for (index, (level, title, start)) in headings.iter().enumerate() {
        if *level != 3 {
            continue;
        }
        let id = title.split_whitespace().next().unwrap_or("");
        if !valid_workstream_id(id) {
            continue;
        }
        let end = headings
            .get(index + 1)
            .map(|heading| heading.2)
            .unwrap_or(raw.len());
        records.push((id.to_string(), raw[*start..end].to_string()));
    }
    records
}

fn source_layout_errors(raw: &str, layout: &Py) -> Vec<String> {
    let mut errors = Vec::new();
    if !matches!(layout, Py::Dict(_)) {
        return vec!["layout must be an object".to_string()];
    }
    if raw.split('\n').next().unwrap_or("") != scalar(pyjson::get(layout, "h1")) {
        errors.push("source H1 mismatch".to_string());
    }
    let h2: Vec<&str> = raw
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .collect();
    let expected_h2 = require_array(layout, "h2_order", "layout", &mut errors);
    let expected_h2: Vec<String> = expected_h2.iter().map(|item| scalar(Some(item))).collect();
    if h2.join("\0") != expected_h2.join("\0") {
        errors.push("source H2 order/membership mismatch".to_string());
    }
    let records = workstream_records(raw);
    let ids: Vec<String> = records.iter().map(|(id, _)| id.clone()).collect();
    let expected_ids = require_array(layout, "workstream_ids", "layout", &mut errors);
    let expected_ids: Vec<String> = expected_ids.iter().map(|item| scalar(Some(item))).collect();
    if ids.join("\0") != expected_ids.join("\0") {
        errors.push("source workstream order/membership mismatch".to_string());
    }
    for (id, body) in records {
        let count = body
            .lines()
            .filter(|line| line.starts_with("- status:"))
            .count();
        if count != 1 {
            errors.push(format!(
                "workstream '{id}' must contain exactly one top-level status"
            ));
        }
    }
    errors
}

fn live_root_errors(raw: &str, current: &Py, layout: &Py, owners: &[Py]) -> Vec<String> {
    let mut errors = Vec::new();
    if !matches!(current, Py::Dict(_)) {
        return vec!["current_root must be an object".to_string()];
    }
    let actual = metrics(raw.as_bytes());
    let ceilings = pyjson::get(current, "enforcement_ceilings").unwrap_or(&Py::None);
    if matches!(ceilings, Py::Dict(_)) {
        for (axis, value) in [
            ("lines", actual.lines),
            ("bytes", actual.bytes),
            ("line_bytes", actual.line_bytes),
        ] {
            if let Some(limit) = pyjson::get(ceilings, axis) {
                let limit = perl::numeric(Some(limit)) as usize;
                if value > limit {
                    errors.push(format!("current roadmap exceeds {axis} ceiling"));
                }
            }
        }
    }
    if raw.split('\n').next().unwrap_or("") != scalar(pyjson::get(layout, "h1")) {
        errors.push("current roadmap H1 mismatch".to_string());
    }
    let h2: Vec<&str> = raw
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .collect();
    let expected_h2 = require_array(current, "required_h2_order", "current_root", &mut errors);
    let expected_h2: Vec<String> = expected_h2.iter().map(|item| scalar(Some(item))).collect();
    if h2.join("\0") != expected_h2.join("\0") {
        errors.push("current roadmap H2 order/membership mismatch".to_string());
    }
    for literal in require_array(current, "required_links", "current_root", &mut errors) {
        let literal = scalar(Some(literal));
        if !raw.contains(&literal) {
            errors.push(format!("current roadmap lacks required route '{literal}'"));
        }
    }
    for literal in require_array(current, "forbidden_literals", "current_root", &mut errors) {
        let literal = scalar(Some(literal));
        if raw.contains(&literal) {
            errors.push(format!(
                "current roadmap contains forbidden chronology literal '{literal}'"
            ));
        }
    }
    let start = scalar(pyjson::get(current, "workstream_start_marker"));
    let end = scalar(pyjson::get(current, "workstream_end_marker"));
    let start_at = raw.find(&start).map(|index| index as isize).unwrap_or(-1);
    let end_at = raw.find(&end).map(|index| index as isize).unwrap_or(-1);
    if start.is_empty() || end.is_empty() || start_at < 0 || end_at <= start_at {
        errors.push("current roadmap workstream markers are missing or out of order".to_string());
    }
    if start_at >= 0 && raw[start_at as usize + start.len()..].contains(&start) {
        errors.push("current roadmap workstream start marker is not unique".to_string());
    }
    if end_at >= 0 && raw[end_at as usize + end.len()..].contains(&end) {
        errors.push("current roadmap workstream end marker is not unique".to_string());
    }
    if start_at >= 0 && end_at > start_at {
        let start_at = start_at as usize;
        let end_at = end_at as usize;
        let region = &raw[start_at + start.len()..end_at];
        let mut rows = Vec::new();
        let mut row_text = BTreeMap::new();
        for line in region.lines() {
            let Some(rest) = line.strip_prefix("|") else {
                continue;
            };
            let Some((id, rest)) = rest.split_once('|') else {
                continue;
            };
            let id = id.trim();
            if valid_workstream_id(id) {
                rows.push(id.to_string());
                row_text.entry(id.to_string()).or_insert(rest);
            }
        }
        let expected = require_array(layout, "workstream_ids", "layout", &mut errors);
        let expected: Vec<String> = expected.iter().map(|item| scalar(Some(item))).collect();
        if rows.join("\0") != expected.join("\0") {
            errors.push("current roadmap workstream table order/membership mismatch".to_string());
        }
        for owner in owners {
            if !matches!(owner, Py::Dict(_)) {
                continue;
            }
            let id = scalar(pyjson::get(owner, "id"));
            let status = scalar(pyjson::get(owner, "initial_status"));
            let path = scalar(pyjson::get(owner, "path"));
            if !row_text
                .get(&id)
                .is_some_and(|text| text.trim_start().starts_with(&format!("{status} |")))
            {
                errors.push(format!(
                    "current roadmap workstream '{id}' lacks initial status '{status}'"
                ));
            }
            if !row_text
                .get(&id)
                .is_some_and(|text| text.contains(&format!("({path})")))
            {
                errors.push(format!(
                    "current roadmap workstream '{id}' lacks its owning task-tree route"
                ));
            }
        }
    }
    errors
}

fn section_line_counts(raw: &str) -> Vec<SectionCount> {
    let mut counts = Vec::new();
    let mut heading: Option<String> = None;
    let mut body = Vec::new();
    let mut flush = |heading: &mut Option<String>, body: &mut Vec<&str>| {
        let Some(current) = heading.take() else {
            return;
        };
        while body.last().is_some_and(|line| line.trim().is_empty()) {
            body.pop();
        }
        counts.push(SectionCount {
            heading: current,
            lines: body.len() + 1,
        });
        body.clear();
    };
    for line in physical_lines(raw) {
        let trimmed = line.trim_end_matches(['\n', '\r']);
        if let Some(next) = trimmed.strip_prefix("## ") {
            flush(&mut heading, &mut body);
            heading = Some(next.to_string());
        } else if heading.is_some() {
            body.push(line);
        }
    }
    flush(&mut heading, &mut body);
    counts
}

fn section_limit_schema_errors(current: &Py) -> Vec<String> {
    let mut found = Vec::new();
    if !matches!(current, Py::Dict(_)) {
        return found;
    }
    let limits = pyjson::get(current, "section_limits").unwrap_or(&Py::None);
    if !matches!(limits, Py::Dict(_)) {
        found.push("current_root section_limits must be an object".to_string());
        return found;
    }
    reject_unknown(
        limits,
        "current_root section_limits",
        &["warning_pct", "sections"],
        &mut found,
    );
    let warning = pyjson::get(limits, "warning_pct");
    let warning_text = scalar(warning);
    if warning.is_none_or(perl::is_reference)
        || warning_text.is_empty()
        || !warning_text
            .chars()
            .all(|character| character.is_ascii_digit())
        || perl::numeric(warning) < 1.0
        || perl::numeric(warning) >= 100.0
    {
        found.push(
            "current_root section_limits warning_pct must be an integer percentage below 100"
                .to_string(),
        );
    }
    let sections = pyjson::get(limits, "sections");
    let Some(Py::List(items)) = sections else {
        found.push("current_root section_limits sections must be a nonempty array".to_string());
        return found;
    };
    if items.is_empty() {
        found.push("current_root section_limits sections must be a nonempty array".to_string());
        return found;
    }
    let expected = require_array(current, "required_h2_order", "current_root", &mut found);
    let expected: Vec<String> = expected.iter().map(|item| scalar(Some(item))).collect();
    let mut total = 0;
    for (index, section) in items.iter().enumerate() {
        if !matches!(section, Py::Dict(_)) {
            found.push(format!("current_root section {index} must be an object"));
            continue;
        }
        reject_unknown(
            section,
            &format!("current_root section {index}"),
            &["heading", "max_lines", "remedy"],
            &mut found,
        );
        let heading = scalar(pyjson::get(section, "heading"));
        if heading.is_empty() {
            found.push(format!("current_root section {index} lacks a heading"));
        }
        let heading = if heading.is_empty() {
            format!("index {index}")
        } else {
            heading
        };
        let max = pyjson::get(section, "max_lines");
        let digits = scalar(max);
        if max.is_none_or(perl::is_reference)
            || digits.is_empty()
            || !digits.chars().all(|character| character.is_ascii_digit())
            || perl::numeric(max) < 1.0
        {
            found.push(format!(
                "current_root section '{heading}' lacks a positive 'max_lines'"
            ));
        } else {
            total += perl::numeric(max) as usize;
        }
        if scalar(pyjson::get(section, "remedy")).is_empty() {
            found.push(format!(
                "current_root section '{heading}' lacks a nonempty 'remedy'"
            ));
        }
    }
    let declared: Vec<String> = items
        .iter()
        .map(|section| scalar(pyjson::get(section, "heading")))
        .collect();
    if declared.join("\0") != expected.join("\0") {
        found.push(
            "current_root section_limits must declare exactly the required H2 sections in order"
                .to_string(),
        );
    }
    let health =
        pyjson::get(current, "health_targets").and_then(|targets| pyjson::get(targets, "lines"));
    if let Some(health) = health {
        let health_value = perl::numeric(Some(health)) as usize;
        if total + items.len() > health_value {
            found.push(format!(
                "current_root sections sum to {} legal lines above the {health_value}-line health target",
                total + items.len()
            ));
        }
    }
    found
}

fn section_limit_errors(raw: &str, current: &Py) -> Vec<String> {
    let mut found = Vec::new();
    let limits = pyjson::get(current, "section_limits").unwrap_or(&Py::None);
    let Some(Py::List(declarations)) = pyjson::get(limits, "sections") else {
        return found;
    };
    let bounds: BTreeMap<String, &Py> = declarations
        .iter()
        .filter_map(|section| {
            if matches!(section, Py::Dict(_)) {
                Some((scalar(pyjson::get(section, "heading")), section))
            } else {
                None
            }
        })
        .collect();
    for section in section_line_counts(raw) {
        let Some(declared) = bounds.get(&section.heading) else {
            found.push(format!(
                "current roadmap section '{}' has no declared line bound",
                section.heading
            ));
            continue;
        };
        let max = pyjson::get(declared, "max_lines");
        let max_text = scalar(max);
        if max_text.is_empty() || !max_text.chars().all(|character| character.is_ascii_digit()) {
            continue;
        }
        let max = perl::numeric(max) as usize;
        if section.lines > max {
            found.push(format!(
                "current roadmap section '{}' is {} lines over its {max}-line bound; remedy: {}",
                section.heading,
                section.lines - max,
                scalar(pyjson::get(declared, "remedy"))
            ));
        }
    }
    found
}

fn section_pressure_warnings(raw: &str, current: &Py) -> Vec<String> {
    let mut found = Vec::new();
    let limits = pyjson::get(current, "section_limits").unwrap_or(&Py::None);
    let Some(Py::List(declarations)) = pyjson::get(limits, "sections") else {
        return found;
    };
    let warning = pyjson::get(limits, "warning_pct");
    let warning_text = scalar(warning);
    if warning_text.is_empty()
        || !warning_text
            .chars()
            .all(|character| character.is_ascii_digit())
    {
        return found;
    }
    let warning = perl::numeric(warning);
    let bounds: BTreeMap<String, &Py> = declarations
        .iter()
        .filter_map(|section| {
            if matches!(section, Py::Dict(_)) {
                Some((scalar(pyjson::get(section, "heading")), section))
            } else {
                None
            }
        })
        .collect();
    for section in section_line_counts(raw) {
        let Some(declared) = bounds.get(&section.heading) else {
            continue;
        };
        let max = perl::numeric(pyjson::get(declared, "max_lines"));
        if max == 0.0 {
            continue;
        }
        let percent = 100.0 * section.lines as f64 / max;
        if percent >= warning && section.lines as f64 <= max {
            found.push(format!(
                "section '{}' is at {percent:.1}% of its {}-line bound; remedy: {}",
                section.heading,
                max as usize,
                scalar(pyjson::get(declared, "remedy"))
            ));
        }
    }
    found
}

fn rollover_schema_errors(
    rollovers: &Py,
    state: &str,
    policy: &Py,
    archive: &Py,
    current: &Py,
) -> Vec<String> {
    let mut found = Vec::new();
    let Some(items) = (match rollovers {
        Py::List(items) => Some(items),
        _ => None,
    }) else {
        found.push("rollovers must be an array".to_string());
        return found;
    };
    let max = pyjson::get(policy, "max_capsules")
        .map(|value| perl::numeric(Some(value)) as usize)
        .unwrap_or(16);
    let pre_migration = scalar(pyjson::get(archive, "source_capsule"));
    let ceilings = pyjson::get(current, "enforcement_ceilings").unwrap_or(&Py::None);
    if state != "migrated" && !items.is_empty() {
        found.push("planned migration_state may not declare a sealed rollover".to_string());
    }
    if items.len() > max {
        found.push(format!(
            "rollovers exceed the declared maximum of {max} capsules"
        ));
    }
    let mut seen_id = BTreeSet::new();
    let mut seen_capsule = BTreeSet::new();
    let mut previous_date = String::new();
    for (index, record) in items.iter().enumerate() {
        if !matches!(record, Py::Dict(_)) {
            found.push(format!("rollover {index} must be an object"));
            continue;
        }
        reject_unknown(
            record,
            &format!("rollover {index}"),
            &[
                "id",
                "capsule",
                "sha256",
                "lines",
                "bytes",
                "line_bytes",
                "sealed_date",
                "reason",
            ],
            &mut found,
        );
        let id = scalar(pyjson::get(record, "id"));
        let id = if valid_kebab(&id) {
            if !seen_id.insert(id.clone()) {
                found.push(format!("duplicate rollover id '{id}'"));
            }
            id
        } else {
            found.push(format!("rollover {index} lacks a lowercase identifier"));
            format!("index {index}")
        };
        let capsule = scalar(pyjson::get(record, "capsule"));
        if !safe_relative(&capsule) || !capsule.ends_with(".md") {
            found.push(format!(
                "rollover '{id}' capsule is not a safe repository-relative Markdown path"
            ));
        } else {
            if !seen_capsule.insert(capsule.clone()) {
                found.push(format!("duplicate rollover capsule '{capsule}'"));
            }
            if !pre_migration.is_empty() && capsule == pre_migration {
                found.push(format!(
                    "rollover '{id}' may not reuse the pre-migration source capsule"
                ));
            }
        }
        validate_hash(
            pyjson::get(record, "sha256"),
            &format!("rollover '{id}' sha256"),
            &mut found,
        );
        for axis in ["lines", "bytes", "line_bytes"] {
            let value = pyjson::get(record, axis);
            let digits = scalar(value);
            if value.is_none_or(perl::is_reference)
                || digits.is_empty()
                || !digits.chars().all(|character| character.is_ascii_digit())
                || perl::numeric(value) < 1.0
            {
                found.push(format!("rollover '{id}' lacks positive integer '{axis}'"));
                continue;
            }
            if let Some(ceiling) = pyjson::get(ceilings, axis) {
                let ceiling = perl::numeric(Some(ceiling)) as usize;
                if perl::numeric(value) as usize > ceiling {
                    found.push(format!(
                        "rollover '{id}' {axis} exceeds the current-root enforcement ceiling"
                    ));
                }
            }
        }
        let date = scalar(pyjson::get(record, "sealed_date"));
        if date.len() != 10
            || !date.chars().enumerate().all(|(index, character)| {
                if matches!(index, 4 | 7) {
                    character == '-'
                } else {
                    character.is_ascii_digit()
                }
            })
        {
            found.push(format!("rollover '{id}' lacks an ISO sealed_date"));
        } else {
            if !previous_date.is_empty() && date < previous_date {
                found.push(format!("rollover '{id}' seals before its predecessor"));
            }
            previous_date = date;
        }
        if scalar(pyjson::get(record, "reason")).is_empty() {
            found.push(format!("rollover '{id}' lacks a nonempty 'reason'"));
        }
    }
    found
}

fn valid_kebab(id: &str) -> bool {
    !id.is_empty()
        && id
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        && id.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}

fn rollover_content_errors(
    rollovers: &Py,
    reader: &dyn Fn(&str) -> Option<Vec<u8>>,
    index_bytes: Option<&[u8]>,
) -> Vec<String> {
    let mut found = Vec::new();
    let Some(items) = (match rollovers {
        Py::List(items) => Some(items),
        _ => None,
    }) else {
        return found;
    };
    for record in items {
        if !matches!(record, Py::Dict(_)) {
            continue;
        }
        let id = scalar(pyjson::get(record, "id"));
        let id = if id.is_empty() { "?".to_string() } else { id };
        let capsule = scalar(pyjson::get(record, "capsule"));
        if !safe_relative(&capsule) {
            continue;
        }
        let Some(bytes) = reader(&capsule) else {
            found.push(format!("rollover '{id}' capsule '{capsule}' is missing"));
            continue;
        };
        let actual = metrics(&bytes);
        for (axis, value) in [
            ("lines", actual.lines),
            ("bytes", actual.bytes),
            ("line_bytes", actual.line_bytes),
        ] {
            if let Some(expected) = pyjson::get(record, axis) {
                let expected = perl::numeric(Some(expected)) as usize;
                if value != expected {
                    found.push(format!(
                        "rollover '{id}' {axis} mismatch: expected {expected}, found {value}"
                    ));
                }
            }
        }
        if sha256_hex(&bytes) != scalar(pyjson::get(record, "sha256")) {
            found.push(format!(
                "rollover '{id}' capsule bytes do not match its sealed SHA-256"
            ));
        }
        if let Some(index) = index_bytes {
            let name = Path::new(&capsule)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy();
            if !String::from_utf8_lossy(index).contains(name.as_ref()) {
                found.push(format!(
                    "rollover '{id}' capsule is not reachable from the roadmap archive index"
                ));
            }
        }
    }
    found
}

fn rollover_summary(rollovers: &Py) -> Py {
    let Some(items) = (match rollovers {
        Py::List(items) => Some(items),
        _ => None,
    }) else {
        return Py::List(Vec::new());
    };
    Py::List(
        items
            .iter()
            .map(|record| {
                if matches!(record, Py::Dict(_)) {
                    object(vec![
                        ("id", text(&scalar(pyjson::get(record, "id")))),
                        (
                            "sealed_date",
                            text(&scalar(pyjson::get(record, "sealed_date"))),
                        ),
                        (
                            "lines",
                            pyjson::get(record, "lines").unwrap_or(&Py::None).clone(),
                        ),
                        (
                            "bytes",
                            pyjson::get(record, "bytes").unwrap_or(&Py::None).clone(),
                        ),
                    ])
                } else {
                    object(Vec::new())
                }
            })
            .collect(),
    )
}

fn validate_workstream_ids(ids: &[Py], errors: &mut Vec<String>) {
    validate_unique_strings(ids, "layout workstream_ids", errors);
    if ids.len() != 23 {
        errors.push("layout must contain exactly 23 roadmap workstream ids".to_string());
    }
    for id in ids {
        let id = scalar(Some(id));
        if !valid_workstream_id(&id) {
            errors.push(format!("invalid roadmap workstream id '{id}'"));
        }
    }
}

fn validate_owners(root: &Path, ids: &[Py], owners: &[Py], errors: &mut Vec<String>) {
    if owners.len() != ids.len() {
        errors.push("workstream_owners count must equal workstream_ids count".to_string());
    }
    let mut catalog = read_raw(&root.join("docs/TASK_TREE.md"), "task catalog", errors);
    if let Some(catalog) = &mut catalog
        && root.join("docs/task-catalog").is_dir()
    {
        let mut parts: Vec<PathBuf> = fs::read_dir(root.join("docs/task-catalog"))
            .map(|entries| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| {
                        path.file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .starts_with("catalog-")
                            && path.extension().is_some_and(|extension| extension == "md")
                    })
                    .collect()
            })
            .unwrap_or_default();
        parts.sort();
        for part in parts {
            if let Some(raw) = read_raw(&part, "task catalog part", errors) {
                let text = String::from_utf8_lossy(&raw).replace("(../tasks/", "(tasks/");
                catalog.extend_from_slice(text.as_bytes());
            }
        }
    }
    for (index, owner) in owners.iter().enumerate() {
        reject_unknown(
            owner,
            &format!("workstream owner {index}"),
            &["id", "initial_status", "path"],
            errors,
        );
        if !matches!(owner, Py::Dict(_)) {
            continue;
        }
        let id = required_string(owner, "id", &format!("workstream owner {index}"), errors)
            .unwrap_or_default();
        required_string(
            owner,
            "initial_status",
            &format!("workstream owner {index}"),
            errors,
        );
        let path = required_string(owner, "path", &format!("workstream owner {index}"), errors)
            .unwrap_or_default();
        if let Some(expected) = ids.get(index) {
            let expected = scalar(Some(expected));
            if id != expected {
                errors.push(format!(
                    "workstream owner order mismatch at {index}: expected '{expected}', found '{id}'"
                ));
            }
        }
        safe_markdown(&path, &format!("owner path for '{id}'"), errors);
        if safe_relative(&path) && !root.join(&path).is_file() {
            errors.push(format!("owner path '{path}' is missing"));
        }
        if let Some(catalog) = &catalog
            && let Some(relative) = path.strip_prefix("docs/tasks/")
            && !String::from_utf8_lossy(catalog).contains(&format!("(tasks/{relative})"))
        {
            errors.push(format!(
                "owner path '{path}' is absent from the complete task catalog"
            ));
        }
    }
}

fn validate_regions_schema(regions: &[Py], source_lines: Option<&Py>, errors: &mut Vec<String>) {
    let mut next_line = 1;
    let mut seen = BTreeSet::new();
    for region in regions {
        reject_unknown(
            region,
            "source region",
            &[
                "id",
                "start_line",
                "end_line",
                "lines",
                "bytes",
                "line_bytes",
                "sha256",
            ],
            errors,
        );
        if !matches!(region, Py::Dict(_)) {
            continue;
        }
        let id = required_string(region, "id", "source region", errors).unwrap_or_default();
        if !seen.insert(id.clone()) {
            errors.push(format!("duplicate source region '{id}'"));
        }
        let start = required_positive_integer(
            region,
            "start_line",
            &format!("source region '{id}'"),
            errors,
        );
        let end =
            required_positive_integer(region, "end_line", &format!("source region '{id}'"), errors);
        let lines =
            required_positive_integer(region, "lines", &format!("source region '{id}'"), errors);
        for field in ["bytes", "line_bytes"] {
            required_positive_integer(region, field, &format!("source region '{id}'"), errors);
        }
        validate_hash(
            pyjson::get(region, "sha256"),
            &format!("source region '{id}' sha256"),
            errors,
        );
        let (Some(start), Some(end), Some(lines)) = (start, end, lines) else {
            continue;
        };
        if start != next_line {
            errors.push(format!(
                "source region '{id}' must start at line {next_line}"
            ));
        }
        if end < start || lines != end - start + 1 {
            errors.push(format!(
                "source region '{id}' line count does not match its inclusive bounds"
            ));
        }
        next_line = end + 1;
    }
    if let Some(source_lines) = source_lines
        && perl::numeric(Some(source_lines)) as usize + 1 != next_line
    {
        errors.push("source regions must partition every source line exactly once".to_string());
    }
}

fn validate_drift_findings(findings: &Py, errors: &mut Vec<String>) {
    let Some(items) = (match findings {
        Py::List(items) => Some(items),
        _ => None,
    }) else {
        errors.push("drift_findings must be an array".to_string());
        return;
    };
    let mut seen = BTreeSet::new();
    for finding in items {
        reject_unknown(
            finding,
            "drift finding",
            &["id", "source_literal", "evidence_path", "evidence_literal"],
            errors,
        );
        if !matches!(finding, Py::Dict(_)) {
            continue;
        }
        let id = required_string(finding, "id", "drift finding", errors).unwrap_or_default();
        if !seen.insert(id.clone()) {
            errors.push(format!("duplicate drift finding '{id}'"));
        }
        for field in ["source_literal", "evidence_path", "evidence_literal"] {
            required_string(finding, field, &format!("drift finding '{id}'"), errors);
        }
        safe_markdown(
            &scalar(pyjson::get(finding, "evidence_path")),
            &format!("drift evidence path for '{id}'"),
            errors,
        );
    }
}

fn validate_current_root_schema(current: &Py, errors: &mut Vec<String>) {
    reject_unknown(
        current,
        "current_root",
        &[
            "required_h2_order",
            "workstream_start_marker",
            "workstream_end_marker",
            "required_links",
            "forbidden_literals",
            "health_targets",
            "enforcement_ceilings",
            "section_limits",
        ],
        errors,
    );
    if !matches!(current, Py::Dict(_)) {
        return;
    }
    validate_unique_strings(
        require_array(current, "required_h2_order", "current_root", errors),
        "current_root h2 order",
        errors,
    );
    validate_unique_strings(
        require_array(current, "required_links", "current_root", errors),
        "current_root required links",
        errors,
    );
    validate_unique_strings(
        require_array(current, "forbidden_literals", "current_root", errors),
        "current_root forbidden literals",
        errors,
    );
    for field in ["workstream_start_marker", "workstream_end_marker"] {
        required_string(current, field, "current_root", errors);
    }
    for kind in ["health_targets", "enforcement_ceilings"] {
        let limits = pyjson::get(current, kind).unwrap_or(&Py::None);
        reject_unknown(
            limits,
            &format!("current_root {kind}"),
            &["lines", "bytes", "line_bytes"],
            errors,
        );
        for field in ["lines", "bytes", "line_bytes"] {
            required_positive_integer(limits, field, &format!("current_root {kind}"), errors);
        }
    }
    if let (Some(health), Some(ceilings)) = (
        pyjson::get(current, "health_targets"),
        pyjson::get(current, "enforcement_ceilings"),
    ) && matches!(health, Py::Dict(_))
        && matches!(ceilings, Py::Dict(_))
    {
        for axis in ["lines", "bytes", "line_bytes"] {
            if let (Some(health_value), Some(ceiling)) =
                (pyjson::get(health, axis), pyjson::get(ceilings, axis))
                && perl::numeric(Some(health_value)) > perl::numeric(Some(ceiling))
            {
                errors.push(format!(
                    "current_root health {axis} exceeds enforcement ceiling"
                ));
            }
        }
    }
    errors.extend(section_limit_schema_errors(current));
}

fn validate_archive_schema(archive: &Py, errors: &mut Vec<String>) {
    reject_unknown(
        archive,
        "archive",
        &[
            "index",
            "manifest",
            "source_capsule",
            "sealed_date",
            "reason",
            "verifier",
            "index_limits",
        ],
        errors,
    );
    if !matches!(archive, Py::Dict(_)) {
        return;
    }
    for field in ["index", "source_capsule"] {
        let path = required_string(archive, field, "archive", errors).unwrap_or_default();
        safe_markdown(&path, &format!("archive {field}"), errors);
    }
    let manifest = required_string(archive, "manifest", "archive", errors).unwrap_or_default();
    if !safe_relative(&manifest) || !manifest.ends_with(".json") {
        errors.push(format!(
            "archive manifest '{manifest}' must be a safe repository-relative JSON path"
        ));
    }
    for field in ["sealed_date", "reason", "verifier"] {
        required_string(archive, field, "archive", errors);
    }
    let limits = pyjson::get(archive, "index_limits").unwrap_or(&Py::None);
    reject_unknown(
        limits,
        "archive index_limits",
        &["lines", "bytes", "line_bytes"],
        errors,
    );
    for field in ["lines", "bytes", "line_bytes"] {
        required_positive_integer(limits, field, "archive index_limits", errors);
    }
}

fn validate_rollover_policy_schema(policy: &Py, errors: &mut Vec<String>) {
    reject_unknown(
        policy,
        "rollover_policy",
        &["max_capsules", "warning_capsules", "remedy"],
        errors,
    );
    if !matches!(policy, Py::Dict(_)) {
        return;
    }
    let max = required_positive_integer(policy, "max_capsules", "rollover_policy", errors);
    let warning = required_positive_integer(policy, "warning_capsules", "rollover_policy", errors);
    required_string(policy, "remedy", "rollover_policy", errors);
    if let (Some(max), Some(warning)) = (max, warning)
        && warning >= max
    {
        errors.push("rollover_policy warning_capsules must stay below max_capsules".to_string());
    }
}

fn validate_consumers(root: &Path, consumers: &Py, errors: &mut Vec<String>) {
    reject_unknown(
        consumers,
        "consumers",
        &["readers", "writers", "required_literals"],
        errors,
    );
    if !matches!(consumers, Py::Dict(_)) {
        return;
    }
    for field in ["readers", "writers"] {
        let paths = require_array(consumers, field, "consumers", errors);
        validate_unique_strings(paths, &format!("consumers {field}"), errors);
        for path in paths {
            let path = scalar(Some(path));
            if !safe_relative(&path) {
                errors.push(format!("consumer path '{path}' is unsafe"));
            } else if !root.join(&path).is_file() {
                errors.push(format!("consumer path '{path}' is missing"));
            }
        }
    }
    for entry in require_array(consumers, "required_literals", "consumers", errors) {
        reject_unknown(entry, "consumer literal", &["path", "literal"], errors);
        if !matches!(entry, Py::Dict(_)) {
            continue;
        }
        let path = required_string(entry, "path", "consumer literal", errors).unwrap_or_default();
        let literal = required_string(
            entry,
            "literal",
            &format!("consumer literal '{path}'"),
            errors,
        )
        .unwrap_or_default();
        if !safe_relative(&path) || !root.join(&path).is_file() {
            errors.push(format!(
                "consumer literal path '{path}' is missing or unsafe"
            ));
            continue;
        }
        if let Some(raw) = read_raw(&root.join(&path), &format!("consumer '{path}'"), errors)
            && !literal.is_empty()
            && !String::from_utf8_lossy(&raw).contains(&literal)
        {
            errors.push(format!(
                "consumer '{path}' lacks required literal '{literal}'"
            ));
        }
    }
}

fn validate_exact_source(raw: &[u8], source: &Py, errors: &mut Vec<String>) {
    let actual = metrics(raw);
    for (axis, value) in [
        ("lines", actual.lines),
        ("bytes", actual.bytes),
        ("line_bytes", actual.line_bytes),
    ] {
        if let Some(expected) = pyjson::get(source, axis) {
            let expected = perl::numeric(Some(expected)) as usize;
            if value != expected {
                errors.push(format!(
                    "source {axis} mismatch: expected {expected}, found {value}"
                ));
            }
        }
    }
    if sha256_hex(raw) != scalar(pyjson::get(source, "sha256")) {
        errors.push("source SHA-256 does not match the pinned pre-migration identity".to_string());
    }
}

fn validate_regions_content(raw: &str, regions: &[Py], errors: &mut Vec<String>) {
    let lines = physical_lines(raw);
    for region in regions {
        if !matches!(region, Py::Dict(_)) {
            continue;
        }
        let start = pyjson::get(region, "start_line");
        let end = pyjson::get(region, "end_line");
        let start_number = perl::numeric(start) as usize;
        let end_number = perl::numeric(end) as usize;
        if start.is_none()
            || end.is_none()
            || start_number < 1
            || end_number > lines.len()
            || end_number < start_number
        {
            continue;
        }
        let slice = lines[start_number - 1..end_number].join("");
        let actual = metrics(slice.as_bytes());
        let id = scalar(pyjson::get(region, "id"));
        for (axis, value) in [
            ("lines", actual.lines),
            ("bytes", actual.bytes),
            ("line_bytes", actual.line_bytes),
        ] {
            if let Some(expected) = pyjson::get(region, axis) {
                let expected = perl::numeric(Some(expected)) as usize;
                if value != expected {
                    errors.push(format!("source region '{id}' {axis} mismatch"));
                }
            }
        }
        if sha256_hex(slice.as_bytes()) != scalar(pyjson::get(region, "sha256")) {
            errors.push(format!("source region '{id}' SHA-256 mismatch"));
        }
    }
}

fn validate_drift_evidence(root: &Path, raw: &str, findings: &Py, errors: &mut Vec<String>) {
    let Some(items) = (match findings {
        Py::List(items) => Some(items),
        _ => None,
    }) else {
        return;
    };
    for finding in items {
        if !matches!(finding, Py::Dict(_)) {
            continue;
        }
        let id = scalar(pyjson::get(finding, "id"));
        let literal = scalar(pyjson::get(finding, "source_literal"));
        if !literal.is_empty() && !raw.contains(&literal) {
            errors.push(format!(
                "drift finding '{id}' is no longer present in the pinned source"
            ));
        }
        let path = scalar(pyjson::get(finding, "evidence_path"));
        if !safe_relative(&path) || !root.join(&path).is_file() {
            continue;
        }
        let expected = scalar(pyjson::get(finding, "evidence_literal"));
        if let Some(evidence) = read_raw(
            &root.join(&path),
            &format!("drift evidence '{path}'"),
            errors,
        ) && !expected.is_empty()
            && !String::from_utf8_lossy(&evidence).contains(&expected)
        {
            errors.push(format!(
                "drift finding '{id}' lacks its task-tree evidence literal"
            ));
        }
    }
}

fn validate_archive(root: &Path, contract: &Py, frozen: &[u8], errors: &mut Vec<String>) {
    let archive = pyjson::get(contract, "archive").unwrap_or(&Py::None);
    if !matches!(archive, Py::Dict(_)) {
        return;
    }
    let index_rel = scalar(pyjson::get(archive, "index"));
    let manifest_rel = scalar(pyjson::get(archive, "manifest"));
    for path in [&index_rel, &manifest_rel] {
        if !safe_relative(path) || !root.join(path).is_file() {
            errors.push(format!(
                "migrated archive path '{path}' is missing or unsafe"
            ));
        }
    }
    if !safe_relative(&index_rel)
        || !root.join(&index_rel).is_file()
        || !safe_relative(&manifest_rel)
        || !root.join(&manifest_rel).is_file()
    {
        return;
    }
    if let Some(index) = read_raw(
        &root.join(&index_rel),
        &format!("roadmap archive index '{index_rel}'"),
        errors,
    ) {
        let actual = metrics(&index);
        let limits = pyjson::get(archive, "index_limits").unwrap_or(&Py::None);
        if matches!(limits, Py::Dict(_)) {
            for (axis, value) in [
                ("lines", actual.lines),
                ("bytes", actual.bytes),
                ("line_bytes", actual.line_bytes),
            ] {
                if let Some(limit) = pyjson::get(limits, axis) {
                    let limit = perl::numeric(Some(limit)) as usize;
                    if value > limit {
                        errors.push(format!("roadmap archive index exceeds {axis} limit"));
                    }
                }
            }
        }
        let capsule_path = scalar(pyjson::get(archive, "source_capsule"));
        let capsule_name = Path::new(&capsule_path)
            .file_name()
            .unwrap_or_default()
            .to_string_lossy();
        let text = String::from_utf8_lossy(&index);
        if capsule_name.is_empty() || !text.contains(capsule_name.as_ref()) {
            errors.push("roadmap archive index lacks the exact source capsule route".to_string());
        }
        if !text.contains("ROADMAP.md") {
            errors.push("roadmap archive index lacks the current ROADMAP.md route".to_string());
        }
    }
    let Some(manifest_bytes) = read_raw(
        &root.join(&manifest_rel),
        &format!("roadmap archive manifest '{manifest_rel}'"),
        errors,
    ) else {
        return;
    };
    if manifest_bytes.len() > 8_192 {
        errors.push("roadmap archive manifest exceeds 8192 bytes".to_string());
    }
    let manifest = String::from_utf8_lossy(&manifest_bytes);
    let manifest = pyjson::loads_strict(&manifest);
    let Ok(manifest @ Py::Dict(_)) = manifest else {
        errors.push("roadmap archive manifest is not one JSON object".to_string());
        return;
    };
    reject_unknown(
        &manifest,
        "roadmap archive manifest",
        &[
            "schema_version",
            "current_path",
            "source_capsule",
            "source_sha256",
            "source_metrics",
            "sealed_date",
            "reason",
            "verifier",
        ],
        errors,
    );
    let source = pyjson::get(contract, "source").unwrap_or(&Py::None);
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
    for (field, expected_value) in &expected {
        let actual = pyjson::get(&manifest, field);
        if actual.is_none_or(perl::is_reference) || scalar(actual) != scalar(Some(expected_value)) {
            errors.push(format!("roadmap archive manifest '{field}' mismatch"));
        }
    }
    let source_metrics = pyjson::get(&manifest, "source_metrics").unwrap_or(&Py::None);
    if !matches!(source_metrics, Py::Dict(_)) {
        errors.push("roadmap archive manifest source_metrics must be an object".to_string());
    } else {
        reject_unknown(
            source_metrics,
            "roadmap archive source_metrics",
            &["lines", "bytes", "line_bytes"],
            errors,
        );
        for axis in ["lines", "bytes", "line_bytes"] {
            if pyjson::get(source_metrics, axis).is_none_or(perl::is_reference)
                || perl::numeric(pyjson::get(source_metrics, axis))
                    != perl::numeric(pyjson::get(source, axis))
            {
                errors.push(format!(
                    "roadmap archive manifest source_metrics.{axis} mismatch"
                ));
            }
        }
    }
    if sha256_hex(frozen) != scalar(pyjson::get(&manifest, "source_sha256")) {
        errors.push("roadmap source capsule bytes do not match the manifest identity".to_string());
    }
}

fn done_block_metrics(raw: &str) -> Metrics {
    let mut inside_workstreams = false;
    let mut block = "";
    let mut done_bytes = String::new();
    for line in physical_lines(raw) {
        if line.starts_with("### R0 ") || line.starts_with("### R0\n") {
            inside_workstreams = true;
        }
        if line.starts_with("### Applied task trees since") {
            break;
        }
        if !inside_workstreams {
            continue;
        }
        if line.starts_with("### ") {
            block = "";
        }
        if let Some((key, _)) = line.trim_start().split_once(':')
            && line.starts_with("- ")
        {
            block = key.trim_start_matches("- ");
        }
        if block == "done" {
            done_bytes.push_str(line);
        }
    }
    metrics(done_bytes.as_bytes())
}

fn validate_contract(root: &Path, contract: &Py) -> (Vec<String>, Option<ReportData>) {
    let mut errors = Vec::new();
    reject_unknown(
        contract,
        "contract",
        &[
            "schema_version",
            "owner",
            "migration_state",
            "source",
            "layout",
            "drift_findings",
            "current_root",
            "archive",
            "consumers",
            "rollover_policy",
            "rollovers",
        ],
        &mut errors,
    );
    if perl::numeric(pyjson::get(contract, "schema_version")) != 1.0 {
        errors.push("contract schema_version must be 1".to_string());
    }
    required_string(contract, "owner", "contract", &mut errors);
    let state =
        required_string(contract, "migration_state", "contract", &mut errors).unwrap_or_default();
    if !matches!(state.as_str(), "planned" | "migrated") {
        errors.push(format!("unknown migration_state '{state}'"));
    }
    let source = pyjson::get(contract, "source").unwrap_or(&Py::None);
    reject_unknown(
        source,
        "source",
        &["path", "sha256", "lines", "bytes", "line_bytes", "regions"],
        &mut errors,
    );
    let source_path = required_string(source, "path", "source", &mut errors).unwrap_or_default();
    safe_markdown(&source_path, "source path", &mut errors);
    required_string(source, "sha256", "source", &mut errors);
    validate_hash(pyjson::get(source, "sha256"), "source sha256", &mut errors);
    for field in ["lines", "bytes", "line_bytes"] {
        required_positive_integer(source, field, "source", &mut errors);
    }
    let layout = pyjson::get(contract, "layout").unwrap_or(&Py::None);
    reject_unknown(
        layout,
        "layout",
        &["h1", "h2_order", "workstream_ids", "workstream_owners"],
        &mut errors,
    );
    required_string(layout, "h1", "layout", &mut errors);
    let h2_order = require_array(layout, "h2_order", "layout", &mut errors);
    let ids = require_array(layout, "workstream_ids", "layout", &mut errors);
    let owners = require_array(layout, "workstream_owners", "layout", &mut errors);
    validate_unique_strings(h2_order, "layout h2_order", &mut errors);
    validate_workstream_ids(ids, &mut errors);
    validate_owners(root, ids, owners, &mut errors);
    let regions = require_array(source, "regions", "source", &mut errors);
    validate_regions_schema(regions, pyjson::get(source, "lines"), &mut errors);
    validate_drift_findings(
        pyjson::get(contract, "drift_findings").unwrap_or(&Py::None),
        &mut errors,
    );
    validate_current_root_schema(
        pyjson::get(contract, "current_root").unwrap_or(&Py::None),
        &mut errors,
    );
    validate_archive_schema(
        pyjson::get(contract, "archive").unwrap_or(&Py::None),
        &mut errors,
    );
    validate_rollover_policy_schema(
        pyjson::get(contract, "rollover_policy").unwrap_or(&Py::None),
        &mut errors,
    );
    errors.extend(rollover_schema_errors(
        pyjson::get(contract, "rollovers").unwrap_or(&Py::None),
        &state,
        pyjson::get(contract, "rollover_policy").unwrap_or(&Py::None),
        pyjson::get(contract, "archive").unwrap_or(&Py::None),
        pyjson::get(contract, "current_root").unwrap_or(&Py::None),
    ));
    validate_consumers(
        root,
        pyjson::get(contract, "consumers").unwrap_or(&Py::None),
        &mut errors,
    );

    if safe_relative(&source_path) && !root.join(&source_path).is_file() {
        errors.push(format!("source path '{source_path}' is missing"));
        return (errors, None);
    }
    if !safe_relative(&source_path) {
        return (errors, None);
    }
    let archive = pyjson::get(contract, "archive").unwrap_or(&Py::None);
    let frozen_path = if state == "migrated" {
        scalar(pyjson::get(archive, "source_capsule"))
    } else {
        source_path.clone()
    };
    if !safe_relative(&frozen_path) || !root.join(&frozen_path).is_file() {
        errors.push(format!(
            "{state} identity source '{frozen_path}' is missing or unsafe"
        ));
        return (errors, None);
    }
    let Some(frozen) = read_raw(
        &root.join(&frozen_path),
        &format!("roadmap identity source '{frozen_path}'"),
        &mut errors,
    ) else {
        return (errors, None);
    };
    validate_exact_source(&frozen, source, &mut errors);
    let frozen_text = String::from_utf8_lossy(&frozen);
    errors.extend(source_layout_errors(&frozen_text, layout));
    validate_regions_content(&frozen_text, regions, &mut errors);
    validate_drift_evidence(
        root,
        &frozen_text,
        pyjson::get(contract, "drift_findings").unwrap_or(&Py::None),
        &mut errors,
    );
    let mut report = ReportData {
        state: state.clone(),
        source_metrics: metrics(&frozen),
        source_sha: sha256_hex(&frozen),
        regions: Py::List(regions.to_vec()),
        workstreams: ids.len(),
        done_blocks: done_block_metrics(&frozen_text),
        drift_findings: match pyjson::get(contract, "drift_findings") {
            Some(Py::List(items)) => items.len(),
            _ => 0,
        },
        current_root_limits: pyjson::get(contract, "current_root")
            .and_then(|current| pyjson::get(current, "enforcement_ceilings"))
            .unwrap_or(&Py::None)
            .clone(),
        archive: archive.clone(),
        rollovers: rollover_summary(pyjson::get(contract, "rollovers").unwrap_or(&Py::None)),
        current_root: None,
        sections: None,
        warnings: Vec::new(),
    };
    if state == "migrated" {
        if let Some(live) = read_raw(
            &root.join(&source_path),
            &format!("current roadmap '{source_path}'"),
            &mut errors,
        ) {
            let live_text = String::from_utf8_lossy(&live);
            errors.extend(live_root_errors(
                &live_text,
                pyjson::get(contract, "current_root").unwrap_or(&Py::None),
                layout,
                owners,
            ));
            errors.extend(section_limit_errors(
                &live_text,
                pyjson::get(contract, "current_root").unwrap_or(&Py::None),
            ));
            report.warnings.extend(section_pressure_warnings(
                &live_text,
                pyjson::get(contract, "current_root").unwrap_or(&Py::None),
            ));
            report.current_root = Some(metrics(&live));
            report.sections = Some(section_line_counts(&live_text));
        }
        validate_archive(root, contract, &frozen, &mut errors);
        let rollovers = pyjson::get(contract, "rollovers").unwrap_or(&Py::None);
        let index_rel = scalar(pyjson::get(archive, "index"));
        let index_bytes = if safe_relative(&index_rel) && root.join(&index_rel).is_file() {
            read_raw(&root.join(&index_rel), "roadmap archive index", &mut errors)
        } else {
            None
        };
        let reader = |relative: &str| {
            let path = root.join(relative);
            if path.is_file() {
                fs::read(path).ok()
            } else {
                None
            }
        };
        errors.extend(rollover_content_errors(
            rollovers,
            &reader,
            index_bytes.as_deref(),
        ));
        let policy = pyjson::get(contract, "rollover_policy").unwrap_or(&Py::None);
        if matches!(policy, Py::Dict(_)) {
            let warning = pyjson::get(policy, "warning_capsules");
            let warning_text = scalar(warning);
            if !warning_text.is_empty()
                && warning_text
                    .chars()
                    .all(|character| character.is_ascii_digit())
            {
                let warning = perl::numeric(warning) as usize;
                let count = match rollovers {
                    Py::List(items) => items.len(),
                    _ => 0,
                };
                if count >= warning {
                    report.warnings.push(format!(
                        "sealed rollovers are at {count} of {} capsules; remedy: {}",
                        scalar(pyjson::get(policy, "max_capsules")),
                        scalar(pyjson::get(policy, "remedy"))
                    ));
                }
            }
        }
    }
    (errors, Some(report))
}

fn report_json(report: &ReportData) -> Py {
    let mut fields = vec![
        ("migration_state", text(&report.state)),
        (
            "source",
            object(vec![
                ("lines", integer(report.source_metrics.lines)),
                ("bytes", integer(report.source_metrics.bytes)),
                ("line_bytes", integer(report.source_metrics.line_bytes)),
                ("sha256", text(&report.source_sha)),
            ]),
        ),
        ("regions", report.regions.clone()),
        ("workstreams", integer(report.workstreams)),
        ("done_blocks", metrics_json(report.done_blocks)),
        ("drift_findings", integer(report.drift_findings)),
        ("current_root_limits", report.current_root_limits.clone()),
        ("archive", report.archive.clone()),
        ("rollovers", report.rollovers.clone()),
    ];
    if let Some(current) = report.current_root {
        fields.push(("current_root", metrics_json(current)));
    }
    if let Some(sections) = &report.sections {
        fields.push((
            "sections",
            Py::List(
                sections
                    .iter()
                    .map(|section| {
                        object(vec![
                            ("heading", text(&section.heading)),
                            ("lines", integer(section.lines)),
                        ])
                    })
                    .collect(),
            ),
        ));
    }
    object(fields)
}

fn shim_path() -> String {
    std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "scripts/check_roadmap_projection_contract.pl".to_string())
}

fn read_contract(root: &Path, relative: &str, errors: &mut Vec<String>) -> Option<Py> {
    if !safe_relative(relative) {
        errors.push(format!("contract path '{relative}' is unsafe"));
        return None;
    }
    let path = root.join(relative);
    if !path.is_file() {
        errors.push(format!("contract is missing: {relative}"));
        return None;
    }
    if fs::metadata(&path)
        .map(|metadata| metadata.len())
        .unwrap_or(0)
        > 65_536
    {
        errors.push("contract exceeds the 65536-byte control-plane cap".to_string());
    }
    let raw = read_raw(&path, &format!("contract '{relative}'"), errors)?;
    let text = String::from_utf8_lossy(&raw);
    let value = pyjson::loads_strict(&text);
    if !matches!(value, Ok(Py::Dict(_))) {
        let error = match &value {
            Err(error) => error.clone(),
            Ok(_) => String::new(),
        };
        let error = if error.starts_with("Expecting property name enclosed in double quotes:")
            && error.ends_with(&format!("(char {})", text.chars().count()))
        {
            format!(
                ", or }} expected while parsing object/hash, at character offset {} (before \"(end of string)\") at {} line 115.\n",
                text.chars().count(),
                shim_path()
            )
        } else {
            error
        };
        errors.push(format!("contract is not one JSON object: {error}"));
        return None;
    }
    Some(value.unwrap())
}

/// Run the roadmap projection contract with its original mode and path arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut requested_root = None;
    let mut contract = "doctrine/live_document_size/roadmap_projection.json".to_string();
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
        return match run_self_test() {
            Ok(passed) => {
                output.out(&format!("{LABEL}: self-test {passed}/49 passed.\n"));
                0
            }
            Err(error) => {
                output.err(&format!("{error}\n"));
                1
            }
        };
    }
    let root = match requested_root {
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
                    output.err(&format!("{LABEL}: repository root does not exist\n"));
                    return 1;
                }
            }
        }
        None => root.to_path_buf(),
    };
    let mut errors = Vec::new();
    let contract = read_contract(&root, &contract, &mut errors);
    let mut report = None;
    if let Some(contract) = contract {
        let (validation, data) = validate_contract(&root, &contract);
        errors.extend(validation);
        report = data;
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
    let Some(report) = report else {
        output.err(&format!("{LABEL}: contract is not one JSON object\n"));
        return 1;
    };
    for warning in &report.warnings {
        output.err(&format!("{LABEL}: WARNING {warning}\n"));
    }
    if mode == Mode::Report {
        match encode(&report_json(&report)) {
            Ok(encoded) => output.out(&format!("{encoded}\n")),
            Err(error) => {
                output.err(&format!("{LABEL}: {error}\n"));
                return 1;
            }
        }
    } else {
        output.out(&format!(
            "{LABEL}: source identity, 23-workstream coverage, reader contract, and {} lifecycle satisfy the bounded current/history design.\n",
            report.state
        ));
    }
    0
}

fn usage(output: &mut dyn super::Output) -> i32 {
    output.err("Usage: cargo xtask check roadmap-projection-contract [--root DIR] [--contract PATH] [--check|--report|--self-test]\n");
    2
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

fn self_test_layout() -> Py {
    object(vec![
        ("h1", text("# ROADMAP")),
        (
            "h2_order",
            Py::List(vec![text("Objective"), text("Major workstreams")]),
        ),
        ("workstream_ids", Py::List(vec![text("R0"), text("R1")])),
    ])
}

fn self_test_current() -> Py {
    object(vec![
        (
            "required_h2_order",
            Py::List(vec![
                text("Objective"),
                text("Workstream status"),
                text("History and execution"),
            ]),
        ),
        (
            "workstream_start_marker",
            text("<!-- roadmap_workstreams:start -->"),
        ),
        (
            "workstream_end_marker",
            text("<!-- roadmap_workstreams:end -->"),
        ),
        (
            "required_links",
            Py::List(vec![
                text("docs/TASK_TREE.md"),
                text("docs/archive/roadmap/INDEX.md"),
                text("CHANGES.md"),
            ]),
        ),
        ("forbidden_literals", Py::List(vec![text("- done:")])),
        (
            "enforcement_ceilings",
            metrics_json(Metrics {
                lines: 30,
                bytes: 4_096,
                line_bytes: 256,
            }),
        ),
    ])
}

fn self_test_owners() -> Vec<Py> {
    vec![
        object(vec![
            ("id", text("R0")),
            ("initial_status", text("Done")),
            ("path", text("docs/tasks/R0.md")),
        ]),
        object(vec![
            ("id", text("R1")),
            ("initial_status", text("Active")),
            ("path", text("docs/tasks/R1.md")),
        ]),
    ]
}

fn sealed_record(digest: &str, metrics: Metrics, overrides: Vec<(&str, Py)>) -> Py {
    let mut record = object(vec![
        ("id", text("root-through-2026-08-11")),
        (
            "capsule",
            text("docs/archive/roadmap/root-through-2026-08-11.md"),
        ),
        ("sha256", text(digest)),
        ("lines", integer(metrics.lines)),
        ("bytes", integer(metrics.bytes)),
        ("line_bytes", integer(metrics.line_bytes)),
        ("sealed_date", text("2026-08-11")),
        ("reason", text("chronology accretion reached the ceiling")),
    ]);
    for (key, value) in overrides {
        set_field(&mut record, key, value);
    }
    record
}

fn bounded(current: &Py, sections: Vec<Py>) -> Py {
    let mut bounded = current.clone();
    set_field(
        &mut bounded,
        "health_targets",
        metrics_json(Metrics {
            lines: 30,
            bytes: 4_096,
            line_bytes: 256,
        }),
    );
    set_field(
        &mut bounded,
        "section_limits",
        object(vec![
            ("warning_pct", integer(80)),
            ("sections", Py::List(sections)),
        ]),
    );
    bounded
}

fn section(heading: &str, max_lines: usize, remedy: &str) -> Py {
    object(vec![
        ("heading", text(heading)),
        ("max_lines", integer(max_lines)),
        ("remedy", text(remedy)),
    ])
}

fn run_self_test() -> Result<usize, String> {
    let layout = self_test_layout();
    let valid_source = "# ROADMAP\n## Objective\n- current\n## Major workstreams\n### R0 Bootstrap\n- status: Done\n### R1 Pipeline\n- status: Active\n";
    let owners = self_test_owners();
    let current = self_test_current();
    let valid_live = "# ROADMAP\n## Objective\n- current\n## Workstream status\n<!-- roadmap_workstreams:start -->\n| Workstream | Status | Owner |\n| --- | --- | --- |\n| R0 | Done | [owner](docs/tasks/R0.md) |\n| R1 | Active | [owner](docs/tasks/R1.md) |\n<!-- roadmap_workstreams:end -->\n## History and execution\n- docs/TASK_TREE.md\n- docs/archive/roadmap/INDEX.md\n- CHANGES.md\n";
    let mut passed = 0;
    let mut check = |name: &str, condition: bool| {
        if condition {
            passed += 1;
            Ok(())
        } else {
            Err(format!("{LABEL}: self-test failed: {name}"))
        }
    };
    check(
        "valid source layout",
        source_layout_errors(valid_source, &layout).is_empty(),
    )?;
    let duplicate = valid_source.replace("### R1 Pipeline", "### R0 Pipeline");
    check(
        "duplicate source workstream rejected",
        !source_layout_errors(&duplicate, &layout).is_empty(),
    )?;
    let reordered = valid_source.replace(
        "### R0 Bootstrap\n- status: Done\n### R1 Pipeline\n- status: Active",
        "### R1 Pipeline\n- status: Active\n### R0 Bootstrap\n- status: Done",
    );
    check(
        "reordered source workstream rejected",
        !source_layout_errors(&reordered, &layout).is_empty(),
    )?;
    let missing_status = valid_source.replace("- status: Active\n", "");
    check(
        "missing source status rejected",
        !source_layout_errors(&missing_status, &layout).is_empty(),
    )?;
    check(
        "valid bounded current root",
        live_root_errors(valid_live, &current, &layout, &owners).is_empty(),
    )?;
    let missing_row = valid_live.replace("| R1 | Active | [owner](docs/tasks/R1.md) |\n", "");
    check(
        "missing current workstream rejected",
        !live_root_errors(&missing_row, &current, &layout, &owners).is_empty(),
    )?;
    let duplicate_row = valid_live.replacen(
        "| R1 | Active | [owner](docs/tasks/R1.md) |\n",
        "| R1 | Active | [owner](docs/tasks/R1.md) |\n| R1 | Active | [owner](docs/tasks/R1.md) |\n",
        1,
    );
    check(
        "duplicate current workstream rejected",
        !live_root_errors(&duplicate_row, &current, &layout, &owners).is_empty(),
    )?;
    let chronology = valid_live.replace(
        "## History and execution",
        "- done: old delivery\n## History and execution",
    );
    check(
        "current chronology rejected",
        !live_root_errors(&chronology, &current, &layout, &owners).is_empty(),
    )?;
    check(
        "unsafe archive path rejected",
        !safe_relative("../ROADMAP.md"),
    )?;

    let policy = object(vec![
        ("max_capsules", integer(3)),
        ("warning_capsules", integer(2)),
        ("remedy", text("seal the series")),
    ]);
    let archive = object(vec![(
        "source_capsule",
        text("docs/archive/roadmap/source-through-2026-08-08.md"),
    )]);
    let capsule_bytes = "# ROADMAP\n## Objective\n- retired\n";
    let digest = sha256_hex(capsule_bytes.as_bytes());
    let capsule_metrics = metrics(capsule_bytes.as_bytes());
    let sealed = || sealed_record(&digest, capsule_metrics, Vec::new());
    let schema_errors = |rollovers: &Py, state: &str| {
        rollover_schema_errors(rollovers, state, &policy, &archive, &current)
    };
    check(
        "valid sealed rollover accepted",
        schema_errors(&Py::List(vec![sealed()]), "migrated").is_empty(),
    )?;
    check(
        "empty rollover series accepted",
        schema_errors(&Py::List(Vec::new()), "migrated").is_empty(),
    )?;
    check(
        "non-array rollovers rejected",
        !schema_errors(&object(Vec::new()), "migrated").is_empty(),
    )?;
    check(
        "unknown rollover field rejected",
        !schema_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("note", text("x"))],
            )]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "non-identifier rollover id rejected",
        !schema_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("id", text("Root 1"))],
            )]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "duplicate rollover id rejected",
        !schema_errors(
            &Py::List(vec![
                sealed(),
                sealed_record(
                    &digest,
                    capsule_metrics,
                    vec![(
                        "capsule",
                        text("docs/archive/roadmap/root-through-2026-08-12.md"),
                    )],
                ),
            ]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "duplicate rollover capsule rejected",
        !schema_errors(
            &Py::List(vec![
                sealed(),
                sealed_record(
                    &digest,
                    capsule_metrics,
                    vec![("id", text("root-through-2026-08-12"))],
                ),
            ]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "pre-migration capsule reuse rejected",
        !schema_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![(
                    "capsule",
                    text("docs/archive/roadmap/source-through-2026-08-08.md"),
                )],
            )]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "unsafe rollover capsule rejected",
        !schema_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("capsule", text("../ROADMAP.md"))],
            )]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "non-Markdown rollover capsule rejected",
        !schema_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("capsule", text("docs/archive/roadmap/root.txt"))],
            )]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "out-of-order rollover dates rejected",
        !schema_errors(
            &Py::List(vec![
                sealed(),
                sealed_record(
                    &digest,
                    capsule_metrics,
                    vec![
                        ("id", text("root-through-2026-08-09")),
                        (
                            "capsule",
                            text("docs/archive/roadmap/root-through-2026-08-09.md"),
                        ),
                        ("sealed_date", text("2026-08-09")),
                    ],
                ),
            ]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "malformed rollover date rejected",
        !schema_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("sealed_date", text("11-08-2026"))],
            )]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "missing rollover reason rejected",
        !schema_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("reason", text(""))],
            )]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "rollover above the current-root ceiling rejected",
        !schema_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("lines", integer(31))],
            )]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "rollover count above the declared maximum rejected",
        !schema_errors(
            &Py::List(vec![
                sealed_record(
                    &digest,
                    capsule_metrics,
                    vec![
                        ("id", text("root-through-2026-08-12")),
                        (
                            "capsule",
                            text("docs/archive/roadmap/root-through-2026-08-12.md"),
                        ),
                        ("sealed_date", text("2026-08-12")),
                    ],
                ),
                sealed_record(
                    &digest,
                    capsule_metrics,
                    vec![
                        ("id", text("root-through-2026-08-13")),
                        (
                            "capsule",
                            text("docs/archive/roadmap/root-through-2026-08-13.md"),
                        ),
                        ("sealed_date", text("2026-08-13")),
                    ],
                ),
                sealed_record(
                    &digest,
                    capsule_metrics,
                    vec![
                        ("id", text("root-through-2026-08-14")),
                        (
                            "capsule",
                            text("docs/archive/roadmap/root-through-2026-08-14.md"),
                        ),
                        ("sealed_date", text("2026-08-14")),
                    ],
                ),
                sealed(),
            ]),
            "migrated",
        )
        .is_empty(),
    )?;
    check(
        "rollover declared in planned state rejected",
        !schema_errors(&Py::List(vec![sealed()]), "planned").is_empty(),
    )?;

    let reader = |path: &str| {
        (path == "docs/archive/roadmap/root-through-2026-08-11.md")
            .then(|| capsule_bytes.as_bytes().to_vec())
    };
    let sealed_index = "# Roadmap archive\n[current](../../../ROADMAP.md)\n[retired](root-through-2026-08-11.md)\n";
    let content_errors = |rollovers: &Py, index: Option<&str>| {
        rollover_content_errors(rollovers, &reader, index.map(str::as_bytes))
    };
    check(
        "sealed capsule identity accepted",
        content_errors(&Py::List(vec![sealed()]), Some(sealed_index)).is_empty(),
    )?;
    check(
        "missing capsule rejected",
        !content_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![(
                    "capsule",
                    text("docs/archive/roadmap/root-through-2026-08-12.md"),
                )],
            )]),
            Some(sealed_index),
        )
        .is_empty(),
    )?;
    check(
        "capsule digest mismatch rejected",
        !content_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("sha256", text(&"0".repeat(64)))],
            )]),
            Some(sealed_index),
        )
        .is_empty(),
    )?;
    check(
        "capsule line-count mismatch rejected",
        !content_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("lines", integer(capsule_metrics.lines + 1))],
            )]),
            Some(sealed_index),
        )
        .is_empty(),
    )?;
    check(
        "capsule byte-count mismatch rejected",
        !content_errors(
            &Py::List(vec![sealed_record(
                &digest,
                capsule_metrics,
                vec![("bytes", integer(capsule_metrics.bytes + 1))],
            )]),
            Some(sealed_index),
        )
        .is_empty(),
    )?;
    check(
        "capsule absent from the archive index rejected",
        !content_errors(
            &Py::List(vec![sealed()]),
            Some("# Roadmap archive\n[current](../../../ROADMAP.md)\n"),
        )
        .is_empty(),
    )?;

    let remedy = "route the detail to its owning task tree";
    let declared = vec![
        section("Objective", 4, remedy),
        section("Workstream status", 10, remedy),
        section("History and execution", 8, remedy),
    ];
    check(
        "valid section bounds accepted",
        section_limit_schema_errors(&bounded(&current, declared.clone())).is_empty(),
    )?;
    check(
        "missing section_limits rejected",
        !section_limit_schema_errors(&current).is_empty(),
    )?;
    check(
        "section declaration missing a required H2 rejected",
        !section_limit_schema_errors(&bounded(&current, declared[..2].to_vec())).is_empty(),
    )?;
    check(
        "section declaration in the wrong order rejected",
        !section_limit_schema_errors(&bounded(
            &current,
            vec![
                declared[1].clone(),
                declared[0].clone(),
                declared[2].clone(),
            ],
        ))
        .is_empty(),
    )?;
    check(
        "undeclared extra section rejected",
        !section_limit_schema_errors(&bounded(&current, {
            let mut sections = declared.clone();
            sections.push(section("Appendix", 4, remedy));
            sections
        }))
        .is_empty(),
    )?;
    check(
        "section without max_lines rejected",
        !section_limit_schema_errors(&bounded(
            &current,
            vec![
                object(vec![
                    ("heading", text("Objective")),
                    ("remedy", text(remedy)),
                ]),
                declared[1].clone(),
                declared[2].clone(),
            ],
        ))
        .is_empty(),
    )?;
    check(
        "section without a remedy rejected",
        !section_limit_schema_errors(&bounded(
            &current,
            vec![
                object(vec![
                    ("heading", text("Objective")),
                    ("max_lines", integer(4)),
                ]),
                declared[1].clone(),
                declared[2].clone(),
            ],
        ))
        .is_empty(),
    )?;
    check(
        "section with an unknown field rejected",
        !section_limit_schema_errors(&bounded(
            &current,
            vec![
                object(vec![
                    ("heading", text("Objective")),
                    ("max_lines", integer(4)),
                    ("remedy", text(remedy)),
                    ("owner", text("x")),
                ]),
                declared[1].clone(),
                declared[2].clone(),
            ],
        ))
        .is_empty(),
    )?;
    check(
        "section bounds summing above the health target rejected",
        !section_limit_schema_errors(&bounded(
            &current,
            vec![
                section("Objective", 20, remedy),
                declared[1].clone(),
                declared[2].clone(),
            ],
        ))
        .is_empty(),
    )?;
    let measurement = section_line_counts(valid_live)
        .into_iter()
        .map(|section| format!("{}={}", section.heading, section.lines))
        .collect::<Vec<_>>()
        .join("|");
    check(
        "section measurement counts the heading and trims trailing blanks",
        measurement == "Objective=2|Workstream status=7|History and execution=4",
    )?;
    check(
        "section within its bound accepted",
        section_limit_errors(valid_live, &bounded(&current, declared.clone())).is_empty(),
    )?;
    check(
        "section exactly at its bound accepted",
        section_limit_errors(
            valid_live,
            &bounded(
                &current,
                vec![
                    section("Objective", 2, remedy),
                    declared[1].clone(),
                    declared[2].clone(),
                ],
            ),
        )
        .is_empty(),
    )?;
    check(
        "section one line over its bound rejected",
        !section_limit_errors(
            valid_live,
            &bounded(
                &current,
                vec![
                    section("Objective", 1, remedy),
                    declared[1].clone(),
                    declared[2].clone(),
                ],
            ),
        )
        .is_empty(),
    )?;
    let accreted = valid_live.replace("- current\n", "- current\n- and then this leaf closed\n");
    check(
        "prose accretion inside one section rejected",
        !section_limit_errors(
            &accreted,
            &bounded(
                &current,
                vec![
                    section("Objective", 2, remedy),
                    declared[1].clone(),
                    declared[2].clone(),
                ],
            ),
        )
        .is_empty(),
    )?;
    check(
        "section present in the root but undeclared rejected",
        !section_limit_errors(valid_live, &bounded(&current, declared[..2].to_vec())).is_empty(),
    )?;
    check(
        "section below the warning percentage stays quiet",
        section_pressure_warnings(valid_live, &bounded(&current, declared.clone())).is_empty(),
    )?;
    check(
        "section at the warning percentage reports its remedy",
        section_pressure_warnings(
            valid_live,
            &bounded(
                &current,
                vec![
                    section("Objective", 2, remedy),
                    declared[1].clone(),
                    declared[2].clone(),
                ],
            ),
        )
        .len()
            == 1,
    )?;
    check(
        "an over-bound section warns as an error, not a warning",
        section_pressure_warnings(
            valid_live,
            &bounded(
                &current,
                vec![
                    section("Objective", 1, remedy),
                    declared[1].clone(),
                    declared[2].clone(),
                ],
            ),
        )
        .is_empty(),
    )?;
    Ok(passed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_forty_nine_cases_pass() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--self-test".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert_eq!(output.text, "roadmap-projection: self-test 49/49 passed.\n");
    }

    #[test]
    fn the_real_migrated_roadmap_and_report_hold() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--report".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("\"migration_state\":\"migrated\""));
        assert!(output.text.contains("\"workstreams\":23"));
        assert!(output.text.contains("\"sections\":"));
    }
}
