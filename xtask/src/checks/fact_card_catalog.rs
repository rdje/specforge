//! Fact-card catalog, native since `RUSTDOC-AND-XTASK.15.v`: canonical cards, bounded deterministic
//! projections, Git-bound legacy identity and the complete 60-case source/plan/route/residue suite.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const DEFAULT_CONTRACT: &str = "doctrine/live_document_size/fact_card_catalog.json";
const VERIFIER: &str = "perl scripts/check_fact_card_catalog.pl --check";
const PLANNED_OUTPUT_LIMIT: usize = 10;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Metrics {
    lines: usize,
    bytes: usize,
    line_bytes: usize,
}

#[derive(Clone, Copy, Debug)]
struct CollectionLimits {
    files: usize,
    lines_each: usize,
    bytes_each: usize,
    lines_total: usize,
    bytes_total: usize,
    line_bytes_each: usize,
}

#[derive(Clone, Copy, Debug)]
struct Limits {
    max_cards: usize,
    cards_per_part: usize,
    max_parts: usize,
    max_id_bytes: usize,
    max_source_title_bytes: usize,
    max_title_cell_bytes: usize,
    max_answers: usize,
    max_answer_bytes: usize,
    max_legacy_row_bytes: usize,
    max_migrated_row_bytes: usize,
    landing_health: Metrics,
    landing_ceilings: Metrics,
    title_health: CollectionLimits,
    title_ceilings: CollectionLimits,
    projection_files: usize,
    projection_lines: usize,
    projection_bytes: usize,
    projection_line_bytes: usize,
}

impl Limits {
    fn fixed() -> Self {
        Self {
            max_cards: 504,
            cards_per_part: 56,
            max_parts: 9,
            max_id_bytes: 64,
            max_source_title_bytes: 1_024,
            max_title_cell_bytes: 112,
            max_answers: 64,
            max_answer_bytes: 2_048,
            max_legacy_row_bytes: 320,
            max_migrated_row_bytes: 333,
            landing_health: Metrics {
                lines: 224,
                bytes: 32_768,
                line_bytes: 256,
            },
            landing_ceilings: Metrics {
                lines: 256,
                bytes: 32_768,
                line_bytes: 320,
            },
            title_health: CollectionLimits {
                files: 9,
                lines_each: 80,
                bytes_each: 24_576,
                lines_total: 720,
                bytes_total: 221_184,
                line_bytes_each: 384,
            },
            title_ceilings: CollectionLimits {
                files: 9,
                lines_each: 96,
                bytes_each: 32_768,
                lines_total: 864,
                bytes_total: 294_912,
                line_bytes_each: 512,
            },
            projection_files: 10,
            projection_lines: 1_120,
            projection_bytes: 327_680,
            projection_line_bytes: 512,
        }
    }

    fn json(self) -> Py {
        object(vec![
            ("max_cards", integer(self.max_cards)),
            ("cards_per_part", integer(self.cards_per_part)),
            ("max_parts", integer(self.max_parts)),
            ("max_id_bytes", integer(self.max_id_bytes)),
            (
                "max_source_title_bytes",
                integer(self.max_source_title_bytes),
            ),
            ("max_title_cell_bytes", integer(self.max_title_cell_bytes)),
            ("max_answers", integer(self.max_answers)),
            ("max_answer_bytes", integer(self.max_answer_bytes)),
            ("max_legacy_row_bytes", integer(self.max_legacy_row_bytes)),
            (
                "max_migrated_row_bytes",
                integer(self.max_migrated_row_bytes),
            ),
            (
                "landing",
                object(vec![
                    ("health_targets", metrics_json(self.landing_health)),
                    ("enforcement_ceilings", metrics_json(self.landing_ceilings)),
                ]),
            ),
            (
                "title_parts",
                object(vec![
                    ("health_targets", collection_limits_json(self.title_health)),
                    (
                        "enforcement_ceilings",
                        collection_limits_json(self.title_ceilings),
                    ),
                ]),
            ),
            (
                "projection_ceiling",
                object(vec![
                    ("files", integer(self.projection_files)),
                    ("lines", integer(self.projection_lines)),
                    ("bytes", integer(self.projection_bytes)),
                    ("line_bytes", integer(self.projection_line_bytes)),
                ]),
            ),
        ])
    }
}

#[derive(Clone, Debug)]
struct Card {
    id: String,
    title: pyjson::PyStr,
    date: String,
    status: String,
    name: String,
    path: String,
}

#[derive(Clone, Debug)]
struct Rendered {
    path: String,
    role: String,
    raw: Vec<u8>,
}

#[derive(Clone, Copy, Debug, Default)]
struct Aggregate {
    files: usize,
    lines: usize,
    bytes: usize,
    line_bytes: usize,
    lines_each: usize,
    bytes_each: usize,
    line_bytes_each: usize,
    lines_total: usize,
    bytes_total: usize,
}

struct Context {
    contract: Py,
    paths: BTreeMap<String, String>,
    limits: Limits,
    cards: Vec<Card>,
    legacy: Vec<u8>,
    projection: Vec<Rendered>,
    planned: Py,
}

struct Validation {
    errors: Vec<String>,
    state: String,
    card_count: usize,
    planned: Py,
    warnings: Vec<String>,
    context: Option<Context>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Check,
    Write,
    Report,
    PrintPlan,
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

fn metrics_json(metrics: Metrics) -> Py {
    object(vec![
        ("lines", integer(metrics.lines)),
        ("bytes", integer(metrics.bytes)),
        ("line_bytes", integer(metrics.line_bytes)),
    ])
}

fn collection_limits_json(limits: CollectionLimits) -> Py {
    object(vec![
        ("files", integer(limits.files)),
        ("lines_each", integer(limits.lines_each)),
        ("bytes_each", integer(limits.bytes_each)),
        ("lines_total", integer(limits.lines_total)),
        ("bytes_total", integer(limits.bytes_total)),
        ("line_bytes_each", integer(limits.line_bytes_each)),
    ])
}

fn canonical_json(value: &Py) -> String {
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
    .unwrap_or_default()
}

fn pretty_json(value: &Py) -> String {
    pyjson::dumps(
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
}

fn sha256_hex(raw: &[u8]) -> String {
    Sha256::digest(raw)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn scalar(value: Option<&Py>) -> String {
    perl::scalar(value).unwrap_or_default()
}

fn field<'a>(value: &'a Py, name: &str) -> Option<&'a Py> {
    pyjson::get(value, name)
}

fn path<'a>(paths: &'a BTreeMap<String, String>, name: &str) -> &'a str {
    paths.get(name).map(String::as_str).unwrap_or("")
}

fn absolute(base: &Path, relative: &str) -> PathBuf {
    base.join(relative)
}

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn safe_relative_path(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('/')
        && !value.contains('\\')
        && !value.contains("//")
        && !value.split('/').any(|part| matches!(part, "." | ".."))
        && !value
            .chars()
            .any(|character| character <= '\u{1f}' || character == '\u{7f}')
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
    let root_device = fs::metadata(base).ok().map(|metadata| metadata.dev());
    let path_device = metadata.map(|metadata| metadata.dev());
    if root_device.is_none() || path_device.is_none() || root_device != path_device {
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

fn read_json(base: &Path, relative: &str, label: &str, errors: &mut Vec<String>) -> Option<Py> {
    let raw = read_regular(base, relative, label, errors)?;
    let value = std::str::from_utf8(&raw)
        .ok()
        .and_then(|text| pyjson::loads_strict(text).ok());
    match value {
        Some(value @ Py::Dict(_)) => Some(value),
        _ => {
            errors.push(format!("{label} is not one valid JSON object"));
            None
        }
    }
}

fn reject_unknown(value: &Py, label: &str, errors: &mut Vec<String>, allowed: &[&str]) {
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
    name: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    let item = field(value, name);
    if item.is_none_or(perl::is_reference) || scalar(item).is_empty() {
        errors.push(format!("{label} lacks non-empty scalar '{name}'"));
        return None;
    }
    Some(scalar(item))
}

fn required_positive_integer(
    value: &Py,
    name: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<usize> {
    let item = field(value, name);
    let digits = scalar(item);
    if item.is_none_or(perl::is_reference)
        || digits.is_empty()
        || !digits.chars().all(|character| character.is_ascii_digit())
        || perl::numeric(item) < 1.0
    {
        errors.push(format!("{label} lacks positive integer '{name}'"));
        return None;
    }
    Some(perl::numeric(item) as usize)
}

fn validate_metrics_object(value: &Py, label: &str, errors: &mut Vec<String>) {
    reject_unknown(value, label, errors, &["lines", "bytes", "line_bytes"]);
    for name in ["lines", "bytes", "line_bytes"] {
        required_positive_integer(value, name, label, errors);
    }
}

fn valid_digest(value: &str, object_id: bool) -> bool {
    let valid_length = value.len() == 64 || (object_id && value.len() == 40);
    valid_length
        && value
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
}

fn validate_planned_schema(records: &Py, errors: &mut Vec<String>) {
    let Py::List(records) = records else {
        errors.push(format!(
            "planned_outputs must contain one to {PLANNED_OUTPUT_LIMIT} output records"
        ));
        return;
    };
    if records.is_empty() || records.len() > PLANNED_OUTPUT_LIMIT {
        errors.push(format!(
            "planned_outputs must contain one to {PLANNED_OUTPUT_LIMIT} output records"
        ));
        return;
    }
    let mut seen = BTreeSet::new();
    for (index, record) in records.iter().enumerate() {
        let label = format!("planned_outputs[{index}]");
        reject_unknown(
            record,
            &label,
            errors,
            &["path", "role", "sha256", "metrics"],
        );
        let output_path = required_scalar(record, "path", &label, errors);
        let role = required_scalar(record, "role", &label, errors);
        let sha = required_scalar(record, "sha256", &label, errors);
        if let Some(output_path) = &output_path {
            if !safe_relative_path(output_path) {
                errors.push(format!("{label} path is unsafe"));
            }
            if !seen.insert(output_path.clone()) {
                errors.push(format!("{label} has duplicate path '{output_path}'"));
            }
        }
        if role
            .as_ref()
            .is_some_and(|role| !matches!(role.as_str(), "landing" | "title_part"))
        {
            errors.push(format!("{label} role must be landing or title_part"));
        }
        if sha.as_ref().is_some_and(|sha| !valid_digest(sha, false)) {
            errors.push(format!("{label} sha256 is invalid"));
        }
        validate_metrics_object(
            field(record, "metrics").unwrap_or(&Py::None),
            &format!("{label}.metrics"),
            errors,
        );
    }
}

fn validate_contract(
    contract: &Py,
    errors: &mut Vec<String>,
    skip_planned: bool,
) -> BTreeMap<String, String> {
    reject_unknown(
        contract,
        "contract",
        errors,
        &[
            "schema_version",
            "migration_state",
            "surface_id",
            "part_surface_id",
            "paths",
            "legacy",
            "limits",
            "planned_outputs",
            "verifier",
        ],
    );
    let schema = field(contract, "schema_version");
    if schema.is_none_or(perl::is_reference) || perl::numeric(schema) != 1.0 {
        errors.push("schema_version must be 1".to_string());
    }
    let state =
        required_scalar(contract, "migration_state", "contract", errors).unwrap_or_default();
    if !matches!(state.as_str(), "legacy_locked" | "migrated") {
        errors.push("migration_state must be legacy_locked or migrated".to_string());
    }
    required_scalar(contract, "surface_id", "contract", errors);
    required_scalar(contract, "part_surface_id", "contract", errors);
    let verifier = required_scalar(contract, "verifier", "contract", errors).unwrap_or_default();
    if verifier != VERIFIER {
        errors.push("verifier must name the stable fact-card checker command".to_string());
    }

    let paths = field(contract, "paths").unwrap_or(&Py::None);
    reject_unknown(
        paths,
        "paths",
        errors,
        &[
            "card_directory",
            "collection_readme",
            "landing",
            "part_directory",
            "part_prefix",
            "surface_registry",
            "question_contract",
        ],
    );
    let names = [
        "card_directory",
        "collection_readme",
        "landing",
        "part_directory",
        "part_prefix",
        "surface_registry",
        "question_contract",
    ];
    let mut result = BTreeMap::new();
    for name in names {
        if let Some(value) = required_scalar(paths, name, "paths", errors) {
            result.insert(name.to_string(), value);
        }
    }
    for name in [
        "card_directory",
        "collection_readme",
        "landing",
        "part_directory",
        "surface_registry",
        "question_contract",
    ] {
        if result.contains_key(name) && !safe_relative_path(path(&result, name)) {
            errors.push(format!("paths.{name} is unsafe"));
        }
    }
    if result.contains_key("part_prefix") {
        let prefix = path(&result, "part_prefix");
        let valid = prefix.ends_with('-')
            && prefix.len() >= 2
            && prefix.chars().next().is_some_and(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit()
            })
            && prefix.chars().all(|character| {
                character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
            });
        if !valid {
            errors.push(
                "paths.part_prefix must be a lowercase safe filename prefix ending in hyphen"
                    .to_string(),
            );
        }
    }
    if result.contains_key("card_directory") && result.contains_key("collection_readme") {
        let expected = format!("{}/README.md", path(&result, "card_directory"));
        if path(&result, "collection_readme") != expected {
            errors.push(
                "collection_readme must be the README directly inside card_directory".to_string(),
            );
        }
    }
    if result.contains_key("card_directory") && result.contains_key("landing") {
        let expected = format!("{}/INDEX.md", path(&result, "card_directory"));
        if path(&result, "landing") != expected {
            errors.push("landing must be INDEX.md directly inside card_directory".to_string());
        }
    }
    if result.contains_key("card_directory")
        && result.contains_key("part_directory")
        && path(&result, "card_directory") == path(&result, "part_directory")
    {
        errors.push("card_directory and part_directory must be distinct".to_string());
    }

    let legacy = field(contract, "legacy").unwrap_or(&Py::None);
    reject_unknown(
        legacy,
        "legacy",
        errors,
        &[
            "boundary_commit",
            "git_blob",
            "sha256",
            "metrics",
            "card_count",
            "row_sha256",
        ],
    );
    for name in ["boundary_commit", "git_blob", "sha256", "row_sha256"] {
        let value = required_scalar(legacy, name, "legacy", errors);
        if let Some(value) = value {
            let object_id = matches!(name, "boundary_commit" | "git_blob");
            if !valid_digest(&value, object_id) {
                errors.push(format!("legacy.{name} is not a lowercase object/digest id"));
            }
        }
    }
    required_positive_integer(legacy, "card_count", "legacy", errors);
    validate_metrics_object(
        field(legacy, "metrics").unwrap_or(&Py::None),
        "legacy.metrics",
        errors,
    );
    if canonical_json(field(contract, "limits").unwrap_or(&Py::None))
        != canonical_json(&Limits::fixed().json())
    {
        errors.push("limits differ from the fixed ADR 0020/0021/0022 profile".to_string());
    }
    if !skip_planned {
        validate_planned_schema(
            field(contract, "planned_outputs").unwrap_or(&Py::None),
            errors,
        );
    }
    result
}

fn numeric_or(value: Option<&Py>, default: f64) -> f64 {
    if value.is_none() {
        default
    } else {
        perl::numeric(value)
    }
}

fn validate_part_surface(
    surface: &Py,
    contract: &Py,
    paths: &BTreeMap<String, String>,
    limits: Limits,
    errors: &mut Vec<String>,
) {
    let target = format!(
        "{}/{}*.md",
        path(paths, "part_directory"),
        path(paths, "part_prefix")
    );
    if canonical_json(field(surface, "targets").unwrap_or(&Py::None))
        != canonical_json(&Py::List(vec![text(&target)]))
    {
        errors.push("title-part surface targets disagree with generated namespace".to_string());
    }
    if scalar(field(surface, "locator")) != "collection"
        || scalar(field(surface, "lifecycle")) != "generated_projection"
        || scalar(field(surface, "state")) != "normal"
    {
        errors
            .push("title-part surface must be collection/generated_projection/normal".to_string());
    }
    if scalar(field(surface, "owner")) != "knowledge-maintainers" {
        errors.push("title-part surface owner must remain knowledge-maintainers".to_string());
    }
    if canonical_json(field(surface, "health_targets").unwrap_or(&Py::None))
        != canonical_json(&collection_limits_json(limits.title_health))
    {
        errors.push("title-part surface health targets differ from the fixed profile".to_string());
    }
    if canonical_json(field(surface, "enforcement_ceilings").unwrap_or(&Py::None))
        != canonical_json(&collection_limits_json(limits.title_ceilings))
    {
        errors.push("title-part surface ceilings differ from the fixed profile".to_string());
    }
    let milestones = field(surface, "milestones").unwrap_or(&Py::None);
    if !matches!(milestones, Py::Dict(_))
        || numeric_or(field(milestones, "warning_pct"), -1.0) != 80.0
        || numeric_or(field(milestones, "rollover_pct"), -1.0) != 90.0
    {
        errors
            .push("title-part surface milestones must remain warning 80 / rollover 90".to_string());
    }
    if scalar(field(surface, "verifier")) != "builtin:budget" {
        errors.push("title-part surface verifier must remain builtin:budget".to_string());
    }
    let cards = format!("{}/*.md", path(paths, "card_directory"));
    if canonical_json(field(surface, "canonical_inputs").unwrap_or(&Py::None))
        != canonical_json(&Py::List(vec![text(&cards)]))
    {
        errors
            .push("title-part surface canonical inputs disagree with canonical cards".to_string());
    }
    if scalar(field(surface, "freshness_verifier")) != "scripts/check_fact_card_catalog.pl" {
        errors.push(
            "title-part surface freshness verifier disagrees with catalog checker".to_string(),
        );
    }
    if scalar(field(surface, "index")) != path(paths, "landing") {
        errors.push("title-part surface index disagrees with stable landing".to_string());
    }
    let index = field(surface, "index_contract").unwrap_or(&Py::None);
    if !matches!(index, Py::Dict(_))
        || scalar(field(index, "kind")) != "external_membership"
        || scalar(field(index, "verifier")) != "builtin:markdown_links"
    {
        errors.push("title-part surface must use external Markdown membership".to_string());
    }
    let _ = contract;
}

fn derived_question_keys(max_facts: usize) -> usize {
    (max_facts * 10).div_ceil(512) * 512
}

fn validate_external_authorities(
    base: &Path,
    contract: &Py,
    paths: &BTreeMap<String, String>,
    limits: Limits,
    errors: &mut Vec<String>,
) {
    let mut record_slots = None;
    let registry = read_regular(
        base,
        path(paths, "surface_registry"),
        "surface registry",
        errors,
    );
    if let Some(registry) = registry {
        let mut records = Vec::new();
        let registry_text = String::from_utf8_lossy(&registry);
        for (index, line) in registry_text.split('\n').enumerate() {
            if line.trim().is_empty() {
                continue;
            }
            match pyjson::loads_strict(line) {
                Ok(record @ Py::Dict(_)) => records.push(record),
                _ => {
                    errors.push(format!(
                        "surface registry line {} is not one JSON object",
                        index + 1
                    ));
                }
            }
        }
        let surface_id = scalar(field(contract, "surface_id"));
        let matches: Vec<&Py> = records
            .iter()
            .filter(|record| scalar(field(record, "surface_id")) == surface_id)
            .collect();
        if matches.len() != 1 {
            errors.push(format!(
                "surface registry must contain surface_id '{surface_id}' exactly once"
            ));
        } else {
            let surface = matches[0];
            let target = format!("{}/*.md", path(paths, "card_directory"));
            if canonical_json(field(surface, "targets").unwrap_or(&Py::None))
                != canonical_json(&Py::List(vec![text(&target)]))
            {
                errors.push(
                    "knowledge-card surface targets disagree with card_directory".to_string(),
                );
            }
            if scalar(field(surface, "lifecycle")) != "partitioned_canonical"
                || scalar(field(surface, "state")) != "normal"
            {
                errors.push(
                    "knowledge-card surface must remain partitioned_canonical/normal".to_string(),
                );
            }
            if scalar(field(surface, "index")) != path(paths, "landing") {
                errors.push("knowledge-card surface index disagrees with landing".to_string());
            }
            if scalar(field(surface, "verifier")) != "scripts/check_fact_card_catalog.pl" {
                errors.push(
                    "knowledge-card surface verifier disagrees with catalog checker".to_string(),
                );
            }
            let empty = object(Vec::new());
            let index = match field(surface, "index_contract") {
                Some(value @ Py::Dict(_)) => value,
                _ => &empty,
            };
            if scalar(field(index, "verifier")) != "builtin:markdown_links" {
                errors.push(
                    "knowledge-card surface index verifier must remain builtin:markdown_links"
                        .to_string(),
                );
            }
            if scalar(field(contract, "migration_state")) == "migrated" {
                if scalar(field(index, "kind")) != "routed_membership"
                    || scalar(field(index, "route_surface"))
                        != scalar(field(contract, "part_surface_id"))
                {
                    errors.push(
                        "knowledge-card surface must route membership through the title parts"
                            .to_string(),
                    );
                }
            } else if scalar(field(index, "kind")) != "membership"
                || field(index, "route_surface").is_some()
            {
                errors
                    .push("knowledge-card surface must retain direct membership links".to_string());
            }
            let health_files = field(surface, "health_targets")
                .filter(|value| matches!(value, Py::Dict(_)))
                .and_then(|value| field(value, "files"));
            let ceiling_files = field(surface, "enforcement_ceilings")
                .filter(|value| matches!(value, Py::Dict(_)))
                .and_then(|value| field(value, "files"));
            if health_files.is_none()
                || ceiling_files.is_none()
                || perl::numeric(health_files) != 506.0
                || perl::numeric(ceiling_files) != 506.0
            {
                errors
                    .push("knowledge-card surface file health/ceiling must remain 506".to_string());
            }
            let milestones = field(surface, "milestones").unwrap_or(&Py::None);
            if !matches!(milestones, Py::Dict(_))
                || numeric_or(field(milestones, "warning_pct"), -1.0) != 80.0
                || numeric_or(field(milestones, "rollover_pct"), -1.0) != 90.0
            {
                errors.push(
                    "knowledge-card surface milestones must remain warning 80 / rollover 90"
                        .to_string(),
                );
            }
            if ceiling_files.is_some()
                && limits.max_cards as f64 != perl::numeric(ceiling_files) - 2.0
            {
                errors.push(
                    "max_cards is not derived from surface files minus README and INDEX"
                        .to_string(),
                );
            }
        }

        let part_id = scalar(field(contract, "part_surface_id"));
        let parts: Vec<&Py> = records
            .iter()
            .filter(|record| scalar(field(record, "surface_id")) == part_id)
            .collect();
        if scalar(field(contract, "migration_state")) == "legacy_locked" {
            if !parts.is_empty() {
                errors.push(format!(
                    "title-part surface '{part_id}' must be absent while legacy_locked"
                ));
            }
        } else if parts.len() != 1 {
            errors.push(format!(
                "surface registry must contain title-part surface '{part_id}' exactly once"
            ));
        } else {
            validate_part_surface(parts[0], contract, paths, limits, errors);
        }

        let decisions: Vec<&Py> = records
            .iter()
            .filter(|record| scalar(field(record, "surface_id")) == "decision_records")
            .collect();
        if decisions.len() != 1 {
            errors.push(
                "surface registry must contain surface 'decision_records' exactly once".to_string(),
            );
        } else {
            let files = field(decisions[0], "enforcement_ceilings")
                .filter(|value| matches!(value, Py::Dict(_)))
                .and_then(|value| field(value, "files"));
            let digits = scalar(files);
            if files.is_none_or(perl::is_reference)
                || digits.is_empty()
                || !digits.chars().all(|character| character.is_ascii_digit())
                || digits.starts_with('0')
                || perl::numeric(files) < 2.0
            {
                errors.push(
                    "decision-record surface must declare a file ceiling above its index"
                        .to_string(),
                );
            } else {
                record_slots = Some(perl::numeric(files) as usize - 1);
            }
        }
    }

    let question = read_json(
        base,
        path(paths, "question_contract"),
        "question projection contract",
        errors,
    );
    if let Some(question) = question {
        if scalar(field(&question, "fact_catalog")) != path(paths, "landing") {
            errors.push("question projection fact_catalog disagrees with landing".to_string());
        }
        let limits_value = field(&question, "limits").unwrap_or(&Py::None);
        let max_facts = if matches!(limits_value, Py::Dict(_)) {
            field(limits_value, "max_facts")
        } else {
            None
        };
        let max_question_keys = if matches!(limits_value, Py::Dict(_)) {
            field(limits_value, "max_question_keys")
        } else {
            None
        };
        let derived_facts = record_slots.map(|slots| limits.max_cards + slots);
        if max_facts.is_none_or(perl::is_reference)
            || derived_facts.is_none()
            || perl::numeric(max_facts) != derived_facts.unwrap_or(0) as f64
        {
            errors.push("question projection max_facts must fund every card slot plus every answers-bearing decision record".to_string());
        }
        let digits = scalar(max_facts);
        let derived_keys = if !max_facts.is_none_or(perl::is_reference)
            && !digits.is_empty()
            && !digits.starts_with('0')
            && digits.chars().all(|character| character.is_ascii_digit())
        {
            Some(derived_question_keys(perl::numeric(max_facts) as usize))
        } else {
            None
        };
        if max_question_keys.is_none_or(perl::is_reference)
            || derived_keys.is_none()
            || perl::numeric(max_question_keys) != derived_keys.unwrap_or(0) as f64
        {
            errors.push("question projection max_question_keys must equal max_facts times 10 rounded up to the 512-key registry quantum".to_string());
        }
    }
}

fn valid_card_filename(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".md") else {
        return false;
    };
    !stem.is_empty()
        && stem
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
        && stem.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}

fn codepoint_bytes(codepoint: u32) -> usize {
    if codepoint <= 0x7f {
        1
    } else if codepoint <= 0x7ff {
        2
    } else if codepoint <= 0xffff {
        3
    } else {
        4
    }
}

fn codepoints_bytes(text: &[u32]) -> Vec<u8> {
    let mut raw = Vec::new();
    for codepoint in text {
        if (0xd800..0xe000).contains(codepoint) {
            raw.push(0xe0 | (*codepoint >> 12) as u8);
            raw.push(0x80 | ((*codepoint >> 6) & 0x3f) as u8);
            raw.push(0x80 | (*codepoint & 0x3f) as u8);
        } else {
            let character = char::from_u32(*codepoint).expect("a decoded codepoint");
            let mut encoded = [0; 4];
            raw.extend_from_slice(character.encode_utf8(&mut encoded).as_bytes());
        }
    }
    raw
}

fn codepoints_len(text: &[u32]) -> usize {
    text.iter()
        .map(|codepoint| codepoint_bytes(*codepoint))
        .sum()
}

fn codepoint_whitespace(codepoint: u32) -> bool {
    char::from_u32(codepoint).is_some_and(char::is_whitespace)
}

fn front_matter(text: &[u32]) -> Option<&[u32]> {
    let body = if text.starts_with(&[45, 45, 45, 13, 10]) {
        &text[5..]
    } else if text.starts_with(&[45, 45, 45, 10]) {
        &text[4..]
    } else {
        return None;
    };
    let mut offset = 0;
    while offset + 4 <= body.len() {
        let Some(found) = body[offset..]
            .windows(4)
            .position(|window| window == [10, 45, 45, 45])
        else {
            break;
        };
        let start = offset + found;
        let after = start + 4;
        let tail = &body[after..];
        if tail.is_empty() || tail[0] == 10 || tail.starts_with(&[13, 10]) {
            let content = &body[..start];
            return Some(if content.last() == Some(&13) {
                &content[..content.len() - 1]
            } else {
                content
            });
        }
        offset = after;
    }
    None
}

fn front_lines(text: &[u32]) -> Vec<&[u32]> {
    let mut lines = Vec::new();
    let mut start = 0;
    for (index, codepoint) in text.iter().enumerate() {
        if *codepoint == 10 {
            let end = if index > start && text[index - 1] == 13 {
                index - 1
            } else {
                index
            };
            lines.push(&text[start..end]);
            start = index + 1;
        }
    }
    lines.push(&text[start..]);
    lines
}

fn trim_horizontal(text: &[u32]) -> &[u32] {
    let mut start = 0;
    let mut end = text.len();
    while start < end && matches!(text[start], 32 | 9) {
        start += 1;
    }
    while end > start && matches!(text[end - 1], 32 | 9) {
        end -= 1;
    }
    &text[start..end]
}

fn clean_scalar(value: &[u32]) -> pyjson::PyStr {
    let value = trim_horizontal(value);
    if value.len() >= 2
        && ((value[0] == 34 && value[value.len() - 1] == 34)
            || (value[0] == 39 && value[value.len() - 1] == 39))
    {
        value[1..value.len() - 1].to_vec()
    } else {
        value.to_vec()
    }
}

fn split_commas(text: &[u32]) -> Vec<&[u32]> {
    let mut result = Vec::new();
    let mut start = 0;
    for (index, codepoint) in text.iter().enumerate() {
        if *codepoint == 44 {
            result.push(&text[start..index]);
            start = index + 1;
        }
    }
    result.push(&text[start..]);
    result
}

fn parse_card(name: &str, text: &[u32], limits: Limits) -> Result<Card, String> {
    let front =
        front_matter(text).ok_or_else(|| format!("{name} lacks leading YAML front matter\n"))?;
    let mut scalars = BTreeMap::<String, pyjson::PyStr>::new();
    let mut answers = Vec::new();
    let mut seen = BTreeSet::new();
    let mut current_key = String::new();
    for line in front_lines(front) {
        let mut indent = 0;
        while indent < line.len() && matches!(line[indent], 32 | 9) {
            indent += 1;
        }
        if indent > 0 && line.get(indent) == Some(&45) {
            let mut value_start = indent + 1;
            while value_start < line.len() && matches!(line[value_start], 32 | 9) {
                value_start += 1;
            }
            if value_start > indent + 1 && value_start < line.len() {
                let value = clean_scalar(&line[value_start..]);
                if current_key == "answers" {
                    if value.is_empty() {
                        return Err(format!("{name} has an empty answer\n"));
                    }
                    answers.push(value);
                }
                continue;
            }
        }
        if line
            .iter()
            .all(|codepoint| codepoint_whitespace(*codepoint))
        {
            continue;
        }
        let Some(colon) = line.iter().position(|codepoint| *codepoint == 58) else {
            return Err(format!(
                "{name} has unsupported front-matter syntax: {}\n",
                pyjson::to_text(line)
            ));
        };
        let key = &line[..colon];
        let valid_key = key.first().is_some_and(|codepoint| {
            (65..=90).contains(codepoint) || (97..=122).contains(codepoint) || *codepoint == 95
        }) && key[1..].iter().all(|codepoint| {
            (48..=57).contains(codepoint)
                || (65..=90).contains(codepoint)
                || (97..=122).contains(codepoint)
                || *codepoint == 95
        });
        if !valid_key {
            return Err(format!(
                "{name} has unsupported front-matter syntax: {}\n",
                pyjson::to_text(line)
            ));
        }
        let key = pyjson::to_text(key);
        if !seen.insert(key.clone()) {
            return Err(format!("{name} repeats front-matter key '{key}'\n"));
        }
        current_key = key.clone();
        let rest = &line[colon + 1..];
        let rest = &rest[rest
            .iter()
            .take_while(|codepoint| matches!(codepoint, 32 | 9))
            .count()..];
        if rest.len() >= 2 && rest[0] == 91 && rest[rest.len() - 1] == 93 {
            if key == "answers" {
                answers.extend(
                    split_commas(&rest[1..rest.len() - 1])
                        .into_iter()
                        .map(clean_scalar),
                );
            }
            current_key.clear();
        } else if !rest.is_empty() {
            scalars.insert(key, clean_scalar(rest));
            current_key.clear();
        }
    }

    let expected_id = name.strip_suffix(".md").unwrap_or(name);
    let id_value = scalars.get("id").cloned().unwrap_or_default();
    if id_value.is_empty() {
        return Err(format!("{name} lacks non-empty id\n"));
    }
    if id_value != pyjson::py_str(expected_id) {
        return Err(format!(
            "{name} id '{}' does not match filename '{expected_id}'\n",
            pyjson::to_text(&id_value)
        ));
    }
    if codepoints_len(&id_value) > limits.max_id_bytes {
        return Err(format!(
            "{name} id exceeds {} UTF-8 bytes\n",
            limits.max_id_bytes
        ));
    }
    let id = expected_id.to_string();
    let title = scalars.get("title").cloned().unwrap_or_default();
    if title.is_empty() {
        return Err(format!("{name} lacks non-empty title\n"));
    }
    if codepoints_len(&title) > limits.max_source_title_bytes {
        return Err(format!(
            "{name} title exceeds {} UTF-8 bytes\n",
            limits.max_source_title_bytes
        ));
    }
    let date = pyjson::to_text(&scalars.get("date").cloned().unwrap_or_default());
    let valid_date = date.len() == 10
        && date.as_bytes()[4] == b'-'
        && date.as_bytes()[7] == b'-'
        && date
            .chars()
            .enumerate()
            .all(|(index, character)| matches!(index, 4 | 7) || character.is_ascii_digit())
        && date[5..7]
            .parse::<usize>()
            .is_ok_and(|month| (1..=12).contains(&month))
        && date[8..10]
            .parse::<usize>()
            .is_ok_and(|day| (1..=31).contains(&day));
    if !valid_date {
        return Err(format!("{name} date must be YYYY-MM-DD\n"));
    }
    let status_value = scalars
        .get("status")
        .cloned()
        .unwrap_or_else(|| pyjson::py_str("current"));
    if ![
        pyjson::py_str("current"),
        pyjson::py_str("superseded"),
        pyjson::py_str("deprecated"),
    ]
    .contains(&status_value)
    {
        return Err(format!(
            "{name} has unknown status '{}'\n",
            pyjson::to_text(&status_value)
        ));
    }
    let status = pyjson::to_text(&status_value);
    answers.retain(|answer| !answer.is_empty());
    if answers.is_empty() {
        return Err(format!("{name} lacks a non-empty answers list\n"));
    }
    if answers.len() > limits.max_answers {
        return Err(format!(
            "{name} has {} answers; limit is {}\n",
            answers.len(),
            limits.max_answers
        ));
    }
    if answers
        .iter()
        .any(|answer| codepoints_len(answer) > limits.max_answer_bytes)
    {
        return Err(format!(
            "{name} answer exceeds {} UTF-8 bytes\n",
            limits.max_answer_bytes
        ));
    }
    if scalars.get("evidence").is_none_or(|value| value.is_empty())
        && scalars.get("reverify").is_none_or(|value| value.is_empty())
    {
        return Err(format!("{name} lacks evidence or reverify\n"));
    }
    Ok(Card {
        id,
        title,
        date,
        status,
        name: name.to_string(),
        path: String::new(),
    })
}

fn collect_cards(
    base: &Path,
    paths: &BTreeMap<String, String>,
    limits: Limits,
    errors: &mut Vec<String>,
) -> Vec<Card> {
    let relative = path(paths, "card_directory");
    if relative.is_empty() || !safe_relative_path(relative) {
        return Vec::new();
    }
    let directory = absolute(base, relative);
    let metadata = fs::symlink_metadata(&directory).ok();
    if metadata
        .as_ref()
        .is_none_or(|metadata| !metadata.is_dir() || metadata.file_type().is_symlink())
    {
        errors.push(format!(
            "card_directory is missing or not a regular directory: {relative}"
        ));
        return Vec::new();
    }
    let root_device = fs::metadata(base).ok().map(|metadata| metadata.dev());
    let directory_device = metadata.map(|metadata| metadata.dev());
    if root_device.is_none() || directory_device.is_none() || root_device != directory_device {
        errors.push(format!(
            "card_directory is off the repository volume: {relative}"
        ));
    }
    let entries = match fs::read_dir(&directory) {
        Ok(entries) => entries,
        Err(error) => {
            errors.push(format!(
                "cannot open card_directory '{relative}': {}",
                os_error(&error)
            ));
            return Vec::new();
        }
    };
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .filter(|name| name.ends_with(".md") && name != "README.md" && name != "INDEX.md")
        .collect();
    names.sort();
    if names.is_empty() {
        errors.push("catalog has no canonical fact cards".to_string());
    }
    if names.len() > limits.max_cards {
        errors.push(format!(
            "catalog has {} cards; limit is {}",
            names.len(),
            limits.max_cards
        ));
    }
    let mut cards = Vec::new();
    for name in names {
        if !valid_card_filename(&name) {
            errors.push(format!("unsafe or reserved fact-card filename '{name}'"));
            continue;
        }
        let card_path = format!("{relative}/{name}");
        let Some(raw) = read_regular(base, &card_path, &format!("fact card '{name}'"), errors)
        else {
            continue;
        };
        match decode_card(&name, &raw, limits) {
            Ok(mut card) => {
                card.path = card_path;
                cards.push(card);
            }
            Err(error) => errors.push(error.trim_end().to_string()),
        }
    }
    cards
}

fn decode_card(name: &str, raw: &[u8], limits: Limits) -> Result<Card, String> {
    let text = decode_wtf8(raw)?;
    parse_card(name, &text, limits)
}

fn surrogate_at(raw: &[u8], offset: usize) -> Option<u32> {
    let bytes = raw.get(offset..offset + 3)?;
    if bytes[0] == 0xed && (0xa0..=0xbf).contains(&bytes[1]) && (0x80..=0xbf).contains(&bytes[2]) {
        Some(
            (u32::from(bytes[0] & 0x0f) << 12)
                | (u32::from(bytes[1] & 0x3f) << 6)
                | u32::from(bytes[2] & 0x3f),
        )
    } else {
        None
    }
}

fn decode_wtf8(raw: &[u8]) -> Result<pyjson::PyStr, String> {
    let mut result = Vec::new();
    let mut offset = 0;
    while offset < raw.len() {
        if let Some(codepoint) = surrogate_at(raw, offset) {
            result.push(codepoint);
            offset += 3;
            continue;
        }
        match std::str::from_utf8(&raw[offset..]) {
            Ok(text) => {
                result.extend(pyjson::py_str(text));
                return Ok(result);
            }
            Err(error) => {
                let valid = error.valid_up_to();
                let prefix =
                    std::str::from_utf8(&raw[offset..offset + valid]).expect("valid prefix");
                result.extend(pyjson::py_str(prefix));
                let start = offset + valid;
                if surrogate_at(raw, start).is_some() {
                    offset = start;
                    continue;
                }
                return Err(invalid_utf8_message(raw, start, error.error_len()));
            }
        }
    }
    Ok(result)
}

fn invalid_utf8_message(raw: &[u8], start: usize, error_len: Option<usize>) -> String {
    let mut length = error_len.unwrap_or(raw.len() - start);
    if matches!(raw.get(start), Some(0xc0 | 0xc1))
        && raw
            .get(start + 1)
            .is_some_and(|byte| (0x80..=0xbf).contains(byte))
    {
        length = 2;
    }
    let bytes = raw[start..start + length]
        .iter()
        .map(|byte| format!("\\x{byte:02X}"))
        .collect::<String>();
    let program = std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "scripts/check_fact_card_catalog.pl".to_string());
    format!("utf8 \"{bytes}\" does not map to Unicode at {program} line 535.\n")
}

fn compact_title(title: &[u32], limit: usize) -> pyjson::PyStr {
    let mut escaped = Vec::new();
    for codepoint in title {
        if *codepoint == 92 {
            escaped.extend_from_slice(&[92, 92]);
        } else if *codepoint == 124 {
            escaped.extend_from_slice(&[92, 124]);
        } else {
            escaped.push(*codepoint);
        }
    }
    if codepoints_len(&escaped) <= limit {
        return escaped;
    }
    let content_limit = limit - codepoint_bytes(0x2026);
    let mut short = Vec::new();
    let mut length = 0;
    for codepoint in escaped {
        let bytes = codepoint_bytes(codepoint);
        if length + bytes > content_limit {
            break;
        }
        short.push(codepoint);
        length += bytes;
    }
    while short
        .last()
        .is_some_and(|codepoint| codepoint_whitespace(*codepoint))
    {
        short.pop();
    }
    short.push(0x2026);
    short
}

fn legacy_row(card: &Card, limits: Limits) -> Vec<u8> {
    let mut row = format!(
        "| [{}]({}) | {} | `{}` | ",
        card.id, card.name, card.date, card.status
    )
    .into_bytes();
    row.extend(codepoints_bytes(&compact_title(
        &card.title,
        limits.max_title_cell_bytes,
    )));
    row.extend_from_slice(b" |");
    row
}

fn migrated_row(card: &Card, limits: Limits, part_path: &str) -> Vec<u8> {
    let mut row = format!(
        "| [{}]({}) | {} | `{}` | ",
        card.id,
        relative_link(part_path, &card.path),
        card.date,
        card.status
    )
    .into_bytes();
    row.extend(codepoints_bytes(&compact_title(
        &card.title,
        limits.max_title_cell_bytes,
    )));
    row.extend_from_slice(b" |");
    row
}

fn join_raw_lines(lines: &[Vec<u8>]) -> Vec<u8> {
    let mut raw = Vec::new();
    for line in lines {
        raw.extend_from_slice(line);
        raw.push(b'\n');
    }
    raw
}

fn render_legacy(cards: &[Card], limits: Limits, errors: &mut Vec<String>) -> Vec<u8> {
    let mut lines: Vec<Vec<u8>> = [
        "# Knowledge fact-card catalog",
        "",
        "> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with",
        "> `perl scripts/check_fact_card_catalog.pl --write`.",
        "",
        "This bounded human route lists every immediate fact card once. Search by question in the",
        "[generated Knowledge Map](../../KNOWLEDGE_MAP.md); browse durable rationale in the",
        "[decision index](../decisions/INDEX.md). Full titles, questions, evidence, and prose remain",
        "canonical in the linked cards.",
        "",
        "Collection guidance: [authoring and lifecycle README](README.md).",
        "",
        "| Fact | Established | Status | Title |",
        "| --- | --- | --- | --- |",
    ]
    .into_iter()
    .map(|line| line.as_bytes().to_vec())
    .collect();
    for card in cards {
        let row = legacy_row(card, limits);
        if row.len() > limits.max_legacy_row_bytes {
            errors.push(format!(
                "{} legacy row exceeds {} UTF-8 bytes",
                card.name, limits.max_legacy_row_bytes
            ));
        }
        lines.push(row);
    }
    join_raw_lines(&lines)
}

fn part_path(paths: &BTreeMap<String, String>, number: usize) -> String {
    format!(
        "{}/{}{number:04}.md",
        path(paths, "part_directory"),
        path(paths, "part_prefix")
    )
}

fn dirname(path: &str) -> &str {
    path.rsplit_once('/')
        .map(|(parent, _)| parent)
        .unwrap_or(".")
}

fn relative_link(source_path: &str, target_path: &str) -> String {
    let mut source: Vec<&str> = dirname(source_path).split('/').collect();
    let mut target: Vec<&str> = target_path.split('/').collect();
    while !source.is_empty() && !target.is_empty() && source[0] == target[0] {
        source.remove(0);
        target.remove(0);
    }
    source
        .iter()
        .map(|_| "..")
        .chain(target)
        .collect::<Vec<_>>()
        .join("/")
}

fn aggregate_metrics(outputs: &[Rendered]) -> Aggregate {
    let mut result = Aggregate {
        files: outputs.len(),
        ..Aggregate::default()
    };
    for output in outputs {
        let item = metrics(output.raw.as_slice());
        result.lines += item.lines;
        result.bytes += item.bytes;
        result.line_bytes = result.line_bytes.max(item.line_bytes);
        result.lines_each = result.lines_each.max(item.lines);
        result.bytes_each = result.bytes_each.max(item.bytes);
        result.line_bytes_each = result.line_bytes_each.max(item.line_bytes);
        result.lines_total += item.lines;
        result.bytes_total += item.bytes;
    }
    result
}

fn enforce_metrics(actual: Metrics, limits: Metrics, label: &str, errors: &mut Vec<String>) {
    for (dimension, actual, ceiling) in [
        ("lines", actual.lines, limits.lines),
        ("bytes", actual.bytes, limits.bytes),
        ("line_bytes", actual.line_bytes, limits.line_bytes),
    ] {
        if actual > ceiling {
            errors.push(format!("{label} exceeds {dimension} ceiling {ceiling}"));
        }
    }
}

fn collection_values(actual: Aggregate) -> [(&'static str, usize); 6] {
    [
        ("files", actual.files),
        ("lines_each", actual.lines_each),
        ("bytes_each", actual.bytes_each),
        ("lines_total", actual.lines_total),
        ("bytes_total", actual.bytes_total),
        ("line_bytes_each", actual.line_bytes_each),
    ]
}

fn collection_limit_values(limits: CollectionLimits) -> [usize; 6] {
    [
        limits.files,
        limits.lines_each,
        limits.bytes_each,
        limits.lines_total,
        limits.bytes_total,
        limits.line_bytes_each,
    ]
}

fn enforce_collection_metrics(
    actual: Aggregate,
    limits: CollectionLimits,
    label: &str,
    errors: &mut Vec<String>,
) {
    for ((dimension, actual), ceiling) in collection_values(actual)
        .into_iter()
        .zip(collection_limit_values(limits))
    {
        if actual > ceiling {
            errors.push(format!("{label} exceeds {dimension} ceiling {ceiling}"));
        }
    }
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
        let mut characters = prefix.chars();
        characters
            .next()
            .is_some_and(|character| character.is_ascii_alphabetic())
            && characters.all(|character| {
                character.is_ascii_alphanumeric() || matches!(character, '+' | '.' | '-')
            })
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

fn resolved_links(
    raw: &[u8],
    source_path: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
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
        let label_text = &raw[open + 1..close];
        if label_text
            .iter()
            .any(|byte| matches!(byte, b'\r' | b'\n' | b']'))
        {
            offset = open + 1;
            continue;
        }
        let target_start = close + 2;
        let Some(end) = raw[target_start..].iter().position(|byte| *byte == b')') else {
            break;
        };
        let end = target_start + end;
        let target = &raw[target_start..end];
        offset = end + 1;
        if target.is_empty() || target.iter().any(|byte| matches!(byte, b'\r' | b'\n')) {
            continue;
        }
        let Ok(target) = std::str::from_utf8(target) else {
            errors.push(format!(
                "{label} has unsafe local link with a non-UTF-8 target"
            ));
            continue;
        };
        let resolved = normalize_link(source_path, target);
        if resolved
            .as_deref()
            .is_none_or(|path| !safe_relative_path(path))
        {
            errors.push(format!("{label} has unsafe local link '{target}'"));
        } else if let Some(resolved) = resolved {
            *counts.entry(resolved).or_insert(0) += 1;
        }
    }
    counts
}

fn validate_expected_links(
    outputs: &[Rendered],
    cards: &[Card],
    paths: &BTreeMap<String, String>,
    errors: &mut Vec<String>,
) {
    if outputs.is_empty() {
        return;
    }
    let root_links = resolved_links(&outputs[0].raw, &outputs[0].path, "planned landing", errors);
    if root_links
        .get(path(paths, "collection_readme"))
        .copied()
        .unwrap_or(0)
        != 1
    {
        errors.push("planned landing must link collection README exactly once".to_string());
    }
    for card in cards {
        if root_links.contains_key(&card.path) {
            errors.push(format!(
                "planned landing must route card '{}' through a title part, not link it",
                card.path
            ));
        }
    }
    for output in &outputs[1..] {
        if root_links.get(&output.path).copied().unwrap_or(0) != 1 {
            errors.push(format!(
                "planned landing must link title part '{}' exactly once",
                output.path
            ));
        }
    }
    let mut routed = BTreeMap::<String, usize>::new();
    for (part_index, output) in outputs[1..].iter().enumerate() {
        let links = resolved_links(&output.raw, &output.path, "planned title part", errors);
        let start = part_index * 56;
        let end = (start + 56).min(cards.len());
        for card in &cards[start..end] {
            if links.get(&card.path).copied().unwrap_or(0) != 1 {
                errors.push(format!(
                    "planned title part '{}' must resolve card '{}' exactly once",
                    output.path, card.path
                ));
            }
        }
        for card in cards {
            *routed.entry(card.path.clone()).or_insert(0) +=
                links.get(&card.path).copied().unwrap_or(0);
        }
    }
    for card in cards {
        if routed.get(&card.path).copied().unwrap_or(0) != 1 {
            errors.push(format!(
                "card '{}' is not reachable exactly once through the landing's title parts",
                card.path
            ));
        }
    }
}

fn validate_landing_ranges(
    raw: &[u8],
    cards: &[Card],
    limits: Limits,
    part_count: usize,
    errors: &mut Vec<String>,
) {
    let raw = std::str::from_utf8(raw).expect("landing paths and ids are valid UTF-8");
    let mut rows = Vec::new();
    for line in raw.split('\n') {
        let Some(rest) = line.strip_prefix("| [") else {
            continue;
        };
        if rest.len() < 4
            || !rest[..4]
                .chars()
                .all(|character| character.is_ascii_digit())
        {
            continue;
        }
        let number = &rest[..4];
        let Some(rest) = rest[4..].strip_prefix("](") else {
            continue;
        };
        let Some(target_end) = rest.find(") | ") else {
            continue;
        };
        if rest[..target_end].contains(['\r', ')']) {
            continue;
        }
        let rest = &rest[target_end + 4..];
        let Some((count, rest)) = rest.split_once(" | `") else {
            continue;
        };
        if count.is_empty() || !count.chars().all(|character| character.is_ascii_digit()) {
            continue;
        }
        let Some((first, last)) = rest.split_once("` | `") else {
            continue;
        };
        let Some(last) = last.strip_suffix("` |") else {
            continue;
        };
        if first.contains(['`', '\r']) || last.contains(['`', '\r']) {
            continue;
        }
        rows.push((
            number.parse::<usize>().unwrap_or(0),
            count.parse::<usize>().unwrap_or(0),
            first.to_string(),
            last.to_string(),
        ));
    }
    if rows.len() != part_count {
        errors.push("planned landing must carry exactly one range row per title part".to_string());
        return;
    }
    let mut covered = 0;
    for (index, (number, count, first, last)) in rows.into_iter().enumerate() {
        let expected_number = index + 1;
        let start = index * limits.cards_per_part;
        let end = (start + limits.cards_per_part).min(cards.len());
        if number != expected_number {
            errors.push(format!(
                "planned landing range row {expected_number} is out of part order"
            ));
        }
        if count != end - start {
            errors.push(format!(
                "planned landing range row {expected_number} miscounts its title part"
            ));
        }
        if cards.get(start).is_some_and(|card| card.id != first) {
            errors.push(format!(
                "planned landing range row {expected_number} names the wrong first id"
            ));
        }
        if end > start && cards[end - 1].id != last {
            errors.push(format!(
                "planned landing range row {expected_number} names the wrong last id"
            ));
        }
        covered += count;
    }
    if covered != cards.len() {
        errors.push("planned landing ranges do not cover every card exactly once".to_string());
    }
}

fn validate_projection_metrics(outputs: &[Rendered], limits: Limits, errors: &mut Vec<String>) {
    if outputs.is_empty() {
        return;
    }
    enforce_metrics(
        metrics(outputs[0].raw.as_slice()),
        limits.landing_ceilings,
        "planned landing",
        errors,
    );
    enforce_collection_metrics(
        aggregate_metrics(&outputs[1..]),
        limits.title_ceilings,
        "planned title parts",
        errors,
    );
    let projection = aggregate_metrics(outputs);
    for (dimension, actual, ceiling) in [
        ("files", projection.files, limits.projection_files),
        ("lines", projection.lines, limits.projection_lines),
        ("bytes", projection.bytes, limits.projection_bytes),
        (
            "line_bytes",
            projection.line_bytes,
            limits.projection_line_bytes,
        ),
    ] {
        if actual > ceiling {
            errors.push(format!(
                "planned projection exceeds {dimension} ceiling {ceiling}"
            ));
        }
    }
}

fn render_projection(
    cards: &[Card],
    paths: &BTreeMap<String, String>,
    limits: Limits,
    errors: &mut Vec<String>,
) -> Vec<Rendered> {
    if cards.len() > limits.max_cards {
        errors.push(format!(
            "card count exceeds migrated maximum {}",
            limits.max_cards
        ));
    }
    let part_count = cards.len().div_ceil(limits.cards_per_part);
    if part_count > limits.max_parts {
        errors.push(format!(
            "projection requires {part_count} parts; limit is {}",
            limits.max_parts
        ));
    }
    let mut range_rows = Vec::new();
    for index in 0..part_count {
        let start = index * limits.cards_per_part;
        let end = (start + limits.cards_per_part).min(cards.len());
        let output_path = part_path(paths, index + 1);
        range_rows.push(format!(
            "| [{:04}]({}) | {} | `{}` | `{}` |",
            index + 1,
            relative_link(path(paths, "landing"), &output_path),
            end - start,
            cards[start].id,
            cards[end - 1].id
        ));
    }
    let mut root = vec![
        "# Knowledge fact-card catalog".to_string(),
        "> **AUTO-GENERATED — DO NOT EDIT.** Run `perl scripts/check_fact_card_catalog.pl --write`. [README](README.md) · [questions](../../KNOWLEDGE_MAP.md) · [decisions](../decisions/INDEX.md).".to_string(),
        format!(
            "> {} fact card{} route through {} title part{}; find an id inside a range, then open that part.",
            cards.len(),
            if cards.len() == 1 { "" } else { "s" },
            part_count,
            if part_count == 1 { "" } else { "s" }
        ),
        String::new(),
        "| Part | Cards | First id | Last id |".to_string(),
        "| --- | ---: | --- | --- |".to_string(),
    ];
    root.extend(range_rows);
    let mut outputs = vec![Rendered {
        path: path(paths, "landing").to_string(),
        role: "landing".to_string(),
        raw: format!("{}\n", root.join("\n")).into_bytes(),
    }];
    for index in 0..part_count {
        let start = index * limits.cards_per_part;
        let end = (start + limits.cards_per_part).min(cards.len());
        let output_path = part_path(paths, index + 1);
        let mut part: Vec<Vec<u8>> = vec![
            format!("# Knowledge fact-card titles — part {:04}", index + 1),
            String::new(),
            "> **AUTO-GENERATED — DO NOT EDIT.** Canonical facts remain in `docs/knowledge/`."
                .to_string(),
            format!(
                "> Return to the [bounded fact-card landing]({}).",
                relative_link(&output_path, path(paths, "landing"))
            ),
            String::new(),
            "| Fact | Established | Status | Title |".to_string(),
            "| --- | --- | --- | --- |".to_string(),
        ]
        .into_iter()
        .map(String::into_bytes)
        .collect();
        for card in &cards[start..end] {
            let row = migrated_row(card, limits, &output_path);
            if row.len() > limits.max_migrated_row_bytes {
                errors.push(format!(
                    "{} migrated row exceeds {} UTF-8 bytes",
                    card.name, limits.max_migrated_row_bytes
                ));
            }
            part.push(row);
        }
        outputs.push(Rendered {
            path: output_path,
            role: "title_part".to_string(),
            raw: join_raw_lines(&part),
        });
    }
    validate_projection_metrics(&outputs, limits, errors);
    validate_expected_links(&outputs, cards, paths, errors);
    validate_landing_ranges(&outputs[0].raw, cards, limits, part_count, errors);
    outputs
}

fn planned_records(outputs: &[Rendered]) -> Py {
    Py::List(
        outputs
            .iter()
            .map(|output| {
                object(vec![
                    ("path", text(&output.path)),
                    ("role", text(&output.role)),
                    ("sha256", text(&sha256_hex(output.raw.as_slice()))),
                    ("metrics", metrics_json(metrics(output.raw.as_slice()))),
                ])
            })
            .collect(),
    )
}

fn compare_planned_records(actual: &Py, expected: &Py, errors: &mut Vec<String>) {
    if !matches!(expected, Py::List(records) if !records.is_empty()) {
        return;
    }
    if canonical_json(actual) != canonical_json(expected) {
        errors.push(
            "planned_outputs membership/order differs from deterministic rendering".to_string(),
        );
    }
}

fn legacy_rows(raw: &[u8]) -> Vec<Vec<u8>> {
    let text = String::from_utf8_lossy(raw);
    text.split('\n')
        .filter(|line| line.starts_with("| [") && line.len() > 3 && !line.contains('\r'))
        .map(|line| line.as_bytes().to_vec())
        .collect()
}

fn row_digest(rows: &[Vec<u8>]) -> String {
    let mut raw = Vec::new();
    for row in rows {
        raw.extend_from_slice(row);
        raw.push(b'\n');
    }
    sha256_hex(&raw)
}

fn command_capture(program: &str, arguments: &[&str]) -> (Vec<u8>, i32, Vec<u8>) {
    match Command::new(program).args(arguments).output() {
        Ok(output) => (
            output.stdout,
            output.status.code().unwrap_or(0),
            output.stderr,
        ),
        Err(error) => (Vec::new(), 127, error.to_string().into_bytes()),
    }
}

fn git_capture(base: &Path, arguments: &[&str]) -> (Vec<u8>, i32, Vec<u8>) {
    let base = base.to_string_lossy().into_owned();
    let mut command = vec!["-C", base.as_str()];
    command.extend_from_slice(arguments);
    command_capture("git", &command)
}

fn trim_newlines(raw: &[u8]) -> String {
    String::from_utf8_lossy(raw)
        .trim_end_matches(['\r', '\n'])
        .to_string()
}

fn compare_metrics(actual: Metrics, expected: &Py, label: &str, errors: &mut Vec<String>) {
    if !matches!(expected, Py::Dict(_)) {
        return;
    }
    for (dimension, actual) in [
        ("lines", actual.lines),
        ("bytes", actual.bytes),
        ("line_bytes", actual.line_bytes),
    ] {
        if let Some(expected) = field(expected, dimension)
            && actual as f64 != perl::numeric(Some(expected))
        {
            errors.push(format!(
                "{label} {dimension} is {actual}, expected {}",
                scalar(Some(expected))
            ));
        }
    }
}

fn validate_git_boundary(
    base: &Path,
    contract: &Py,
    paths: &BTreeMap<String, String>,
    errors: &mut Vec<String>,
) -> Option<Vec<u8>> {
    let legacy = field(contract, "legacy").unwrap_or(&Py::None);
    let (top, top_exit, top_error) = git_capture(base, &["rev-parse", "--show-toplevel"]);
    if top_exit != 0 {
        errors.push(format!(
            "cannot resolve Git root: {}",
            String::from_utf8_lossy(&top_error)
        ));
        return None;
    }
    let top = trim_newlines(&top);
    let resolved_top = fs::canonicalize(&top)
        .ok()
        .and_then(|path| path.to_str().map(str::to_string))
        .unwrap_or(top);
    if Path::new(&resolved_top) != base {
        errors.push("validation root is not the Git repository root".to_string());
    }
    if field(legacy, "boundary_commit").is_none() || !paths.contains_key("landing") {
        return None;
    }
    let boundary = scalar(field(legacy, "boundary_commit"));
    let commit = format!("{boundary}^{{commit}}");
    let (_, commit_exit, _) = git_capture(base, &["cat-file", "-e", &commit]);
    if commit_exit != 0 {
        errors.push("legacy boundary commit lookup failed".to_string());
        return None;
    }
    let (_, ancestor_exit, _) =
        git_capture(base, &["merge-base", "--is-ancestor", &boundary, "HEAD"]);
    if ancestor_exit != 0 {
        errors.push("legacy boundary commit is not an ancestor of HEAD".to_string());
    }
    let object = format!("{boundary}:{}", path(paths, "landing"));
    let (blob, blob_exit, blob_error) = git_capture(base, &["rev-parse", &object]);
    if blob_exit != 0 {
        errors.push(format!(
            "legacy boundary blob lookup failed: {}",
            String::from_utf8_lossy(&blob_error)
        ));
        return None;
    }
    let blob = trim_newlines(&blob);
    if field(legacy, "git_blob").is_some() && blob != scalar(field(legacy, "git_blob")) {
        errors.push(format!(
            "legacy boundary blob '{}' differs from declared '{}'",
            blob,
            scalar(field(legacy, "git_blob"))
        ));
    }
    let (raw, raw_exit, raw_error) = git_capture(base, &["cat-file", "blob", &blob]);
    if raw_exit != 0 {
        errors.push(format!(
            "cannot read legacy boundary blob: {}",
            String::from_utf8_lossy(&raw_error)
        ));
        return None;
    }
    if field(legacy, "sha256").is_some() && sha256_hex(&raw) != scalar(field(legacy, "sha256")) {
        errors.push("legacy boundary SHA-256 differs from contract".to_string());
    }
    compare_metrics(
        metrics(&raw),
        field(legacy, "metrics").unwrap_or(&Py::None),
        "legacy boundary",
        errors,
    );
    let rows = legacy_rows(&raw);
    if field(legacy, "card_count").is_some()
        && rows.len() as f64 != perl::numeric(field(legacy, "card_count"))
    {
        errors.push(format!(
            "legacy boundary row count is {}, expected {}",
            rows.len(),
            scalar(field(legacy, "card_count"))
        ));
    }
    if field(legacy, "row_sha256").is_some()
        && row_digest(&rows) != scalar(field(legacy, "row_sha256"))
    {
        errors.push("legacy boundary row digest differs from contract".to_string());
    }
    Some(raw)
}

fn validate_stage_zero(base: &Path, relative: &str, expected_blob: &str, errors: &mut Vec<String>) {
    let (output, exit, stderr) = git_capture(base, &["ls-files", "--stage", "--", relative]);
    if exit != 0 {
        errors.push(format!(
            "cannot inspect stage-zero landing: {}",
            String::from_utf8_lossy(&stderr)
        ));
        return;
    }
    let output = String::from_utf8_lossy(&output);
    let rows: Vec<&str> = output
        .split(['\r', '\n'])
        .filter(|row| !row.is_empty())
        .collect();
    let parsed = rows.first().and_then(|row| {
        let (metadata, path) = row.split_once('\t')?;
        let mut fields = metadata.split(' ');
        let mode = fields.next()?;
        let blob = fields.next()?;
        let stage = fields.next()?;
        if fields.next().is_some()
            || mode.is_empty()
            || !mode.chars().all(|character| character.is_ascii_digit())
            || stage != "0"
            || path != relative
            || !valid_digest(blob, true)
        {
            return None;
        }
        Some(blob.to_string())
    });
    let Some(blob) = parsed.filter(|_| rows.len() == 1) else {
        errors.push("landing must have exactly one stage-zero Git index entry".to_string());
        return;
    };
    if !expected_blob.is_empty() && blob != expected_blob {
        errors.push(format!(
            "stage-zero landing blob '{blob}' differs from legacy '{expected_blob}'"
        ));
    }
}

fn prepare_context(
    base: &Path,
    contract_path: &str,
    skip_planned: bool,
) -> (Vec<String>, Option<Context>) {
    let mut errors = Vec::new();
    if !safe_relative_path(contract_path) {
        errors.push("contract path is unsafe".to_string());
        return (errors, None);
    }
    let Some(contract) = read_json(base, contract_path, "contract", &mut errors) else {
        return (errors, None);
    };
    let paths = validate_contract(&contract, &mut errors, skip_planned);
    let limits = Limits::fixed();
    validate_external_authorities(base, &contract, &paths, limits, &mut errors);
    if paths.contains_key("collection_readme") {
        read_regular(
            base,
            path(&paths, "collection_readme"),
            "collection README",
            &mut errors,
        );
    }
    let cards = collect_cards(base, &paths, limits, &mut errors);
    let legacy = render_legacy(&cards, limits, &mut errors);
    let projection = render_projection(&cards, &paths, limits, &mut errors);
    let planned = planned_records(&projection);
    if !skip_planned {
        compare_planned_records(
            &planned,
            field(&contract, "planned_outputs").unwrap_or(&Py::None),
            &mut errors,
        );
    }
    (
        errors,
        Some(Context {
            contract,
            paths,
            limits,
            cards,
            legacy,
            projection,
            planned,
        }),
    )
}

fn valid_part_name(name: &str, prefix: &str) -> bool {
    let Some(digits) = name
        .strip_prefix(prefix)
        .and_then(|rest| rest.strip_suffix(".md"))
    else {
        return false;
    };
    digits.len() == 4 && digits.chars().all(|character| character.is_ascii_digit())
}

fn validate_migrated_outputs(
    base: &Path,
    context: &Context,
    errors: &mut Vec<String>,
    skip_outputs: bool,
) {
    let expected: BTreeMap<&str, &Rendered> = context
        .projection
        .iter()
        .map(|output| (output.path.as_str(), output))
        .collect();
    let directory = absolute(base, path(&context.paths, "part_directory"));
    if !directory.exists() {
        if !skip_outputs {
            errors.push(format!(
                "title-part directory is missing: {}",
                path(&context.paths, "part_directory")
            ));
        }
    } else {
        let metadata = fs::symlink_metadata(&directory).ok();
        if metadata
            .as_ref()
            .is_none_or(|metadata| !metadata.is_dir() || metadata.file_type().is_symlink())
        {
            errors.push(format!(
                "title-part directory is not a regular directory: {}",
                path(&context.paths, "part_directory")
            ));
        } else {
            match fs::read_dir(&directory) {
                Ok(entries) => {
                    let mut names: Vec<String> = entries
                        .filter_map(Result::ok)
                        .map(|entry| entry.file_name().to_string_lossy().into_owned())
                        .collect();
                    names.sort();
                    for name in names {
                        let relative = format!("{}/{name}", path(&context.paths, "part_directory"));
                        let generated = valid_part_name(&name, path(&context.paths, "part_prefix"));
                        if !generated {
                            errors.push(format!("unexpected title-part residue '{relative}'"));
                        }
                        if !skip_outputs && generated && !expected.contains_key(relative.as_str()) {
                            errors.push(format!("stale title-part output '{relative}'"));
                        }
                    }
                }
                Err(error) => errors.push(format!(
                    "cannot open title-part directory: {}",
                    os_error(&error)
                )),
            }
        }
    }
    if skip_outputs {
        return;
    }
    for output in &context.projection {
        if let Some(actual) = read_regular(
            base,
            &output.path,
            &format!("migrated {}", output.role),
            errors,
        ) && actual != output.raw.as_slice()
        {
            errors.push(format!(
                "migrated output differs from deterministic rendering: {}",
                output.path
            ));
        }
    }
    let current_rows: Vec<Vec<u8>> = context
        .cards
        .iter()
        .map(|card| legacy_row(card, context.limits))
        .collect();
    let legacy = field(&context.contract, "legacy").unwrap_or(&Py::None);
    if current_rows.len() as f64 == perl::numeric(field(legacy, "card_count"))
        && row_digest(&current_rows) == scalar(field(legacy, "row_sha256"))
    {
        compare_planned_records(
            &planned_records(&context.projection),
            field(&context.contract, "planned_outputs").unwrap_or(&Py::None),
            errors,
        );
    }
}

fn pressure_findings(actual: Metrics, targets: Metrics, label: &str) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    for (dimension, actual, target) in [
        ("lines", actual.lines, targets.lines),
        ("bytes", actual.bytes, targets.bytes),
        ("line_bytes", actual.line_bytes, targets.line_bytes),
    ] {
        let percent = 100.0 * actual as f64 / target as f64;
        if percent >= 90.0 {
            errors.push(format!(
                "{label} {dimension} is at {percent:.1}% of health; mandatory rollover is required"
            ));
        } else if percent >= 80.0 {
            warnings.push(format!("{label} {dimension} is at {percent:.1}% of health"));
        }
    }
    (errors, warnings)
}

fn collection_pressure_findings(
    actual: Aggregate,
    targets: CollectionLimits,
    label: &str,
) -> (Vec<String>, Vec<String>) {
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    for ((dimension, actual), target) in collection_values(actual)
        .into_iter()
        .zip(collection_limit_values(targets))
    {
        if dimension == "files" && actual == target {
            continue;
        }
        let percent = 100.0 * actual as f64 / target as f64;
        if percent >= 90.0 {
            errors.push(format!(
                "{label} {dimension} is at {percent:.1}% of health; mandatory rollover is required"
            ));
        } else if percent >= 80.0 {
            warnings.push(format!("{label} {dimension} is at {percent:.1}% of health"));
        }
    }
    (errors, warnings)
}

fn validate_tree(base: &Path, contract_path: &str, skip_outputs: bool) -> Validation {
    let (mut errors, context) = prepare_context(base, contract_path, skip_outputs);
    let Some(context) = context else {
        return Validation {
            errors,
            state: String::new(),
            card_count: 0,
            planned: Py::List(Vec::new()),
            warnings: Vec::new(),
            context: None,
        };
    };
    let boundary = validate_git_boundary(base, &context.contract, &context.paths, &mut errors);
    let state = scalar(field(&context.contract, "migration_state"));
    let mut warnings = Vec::new();
    if state == "legacy_locked" {
        if context.paths.contains_key("landing") {
            let legacy = field(&context.contract, "legacy").unwrap_or(&Py::None);
            validate_stage_zero(
                base,
                path(&context.paths, "landing"),
                &scalar(field(legacy, "git_blob")),
                &mut errors,
            );
        }
        if let Some(current) = read_regular(
            base,
            path(&context.paths, "landing"),
            "legacy landing",
            &mut errors,
        ) {
            if boundary
                .as_ref()
                .is_some_and(|boundary| current != *boundary)
            {
                errors.push("working legacy landing differs from committed boundary".to_string());
            }
            if current != context.legacy.as_slice() {
                errors.push(
                    "working legacy landing differs from canonical card rendering".to_string(),
                );
            }
        }
        if context.paths.contains_key("part_directory")
            && absolute(base, path(&context.paths, "part_directory")).exists()
        {
            errors.push(format!(
                "premature title-part directory exists: {}",
                path(&context.paths, "part_directory")
            ));
        }
    } else if state == "migrated" {
        validate_migrated_outputs(base, &context, &mut errors, skip_outputs);
    }
    let (pressure_errors, pressure_warnings) = pressure_findings(
        metrics(context.projection[0].raw.as_slice()),
        context.limits.landing_health,
        "planned landing",
    );
    errors.extend(pressure_errors);
    warnings.extend(pressure_warnings);
    let (pressure_errors, pressure_warnings) = collection_pressure_findings(
        aggregate_metrics(&context.projection[1..]),
        context.limits.title_health,
        "planned title parts",
    );
    errors.extend(pressure_errors);
    warnings.extend(pressure_warnings);
    Validation {
        errors,
        state,
        card_count: context.cards.len(),
        planned: context.planned.clone(),
        warnings,
        context: Some(context),
    }
}

fn atomic_write(path: &Path, raw: &[u8]) -> Result<(), String> {
    if let Some(parent) = path.parent()
        && !parent.is_dir()
    {
        fs::create_dir_all(parent).map_err(|error| os_error(&error))?;
    }
    let temporary = PathBuf::from(format!(
        "{}.fact-catalog.{}",
        path.display(),
        std::process::id()
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|error| {
            format!(
                "fact-card-catalog: cannot create same-directory temporary '{}': {}",
                temporary.display(),
                os_error(&error)
            )
        })?;
    file.write_all(raw).map_err(|error| {
        format!(
            "fact-card-catalog: cannot write '{}': {}",
            temporary.display(),
            os_error(&error)
        )
    })?;
    drop(file);
    fs::rename(&temporary, path).map_err(|error| {
        format!(
            "fact-card-catalog: cannot replace '{}': {}",
            path.display(),
            os_error(&error)
        )
    })
}

struct FixtureCleanup(PathBuf);

impl Drop for FixtureCleanup {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn write_projection(
    base: &Path,
    context: &Context,
    quiet: bool,
    output: &mut dyn super::Output,
) -> Result<(), String> {
    let state = scalar(field(&context.contract, "migration_state"));
    if state == "legacy_locked" {
        atomic_write(
            &absolute(base, path(&context.paths, "landing")),
            context.legacy.as_slice(),
        )?;
        if !quiet {
            output.out(&format!(
                "fact-card-catalog: wrote legacy monolithic catalog for {} cards.\n",
                context.cards.len()
            ));
        }
        return Ok(());
    }
    let workspace_relative = format!("generated/.fact-card-catalog-write.{}", std::process::id());
    let workspace = absolute(base, &workspace_relative);
    if workspace.exists() {
        return Err("fact-card-catalog: unsafe existing write workspace".to_string());
    }
    fs::create_dir_all(&workspace)
        .map_err(|error| format!("fact-card-catalog: {}", os_error(&error)))?;
    let cleanup = FixtureCleanup(workspace.clone());
    let root_device = fs::metadata(base).ok().map(|metadata| metadata.dev());
    let workspace_device = fs::metadata(&workspace).ok().map(|metadata| metadata.dev());
    if root_device.is_none() || workspace_device.is_none() || root_device != workspace_device {
        return Err("fact-card-catalog: write workspace is off the repository volume".to_string());
    }
    for rendered in &context.projection {
        let staged = absolute(&workspace, &rendered.path);
        if let Some(parent) = staged.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("fact-card-catalog: {}", os_error(&error)))?;
        }
        atomic_write(&staged, rendered.raw.as_slice())?;
    }
    let mut workspace_errors = Vec::new();
    for rendered in &context.projection {
        let staged = read_regular(
            &workspace,
            &rendered.path,
            &format!("staged {}", rendered.role),
            &mut workspace_errors,
        );
        if staged
            .as_deref()
            .is_some_and(|raw| raw != rendered.raw.as_slice())
        {
            workspace_errors.push(format!(
                "staged output differs from deterministic rendering: {}",
                rendered.path
            ));
        }
    }
    if !workspace_errors.is_empty() {
        return Err(workspace_errors.join("\n"));
    }
    fs::create_dir_all(absolute(base, path(&context.paths, "part_directory")))
        .map_err(|error| format!("fact-card-catalog: {}", os_error(&error)))?;
    for rendered in &context.projection[1..] {
        atomic_write(&absolute(base, &rendered.path), rendered.raw.as_slice())?;
    }
    let expected: BTreeSet<&str> = context
        .projection
        .iter()
        .map(|rendered| rendered.path.as_str())
        .collect();
    let directory = absolute(base, path(&context.paths, "part_directory"));
    let entries = fs::read_dir(&directory)
        .map_err(|error| format!("cannot open title-part directory: {}", os_error(&error)))?;
    let mut names: Vec<String> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.file_name().to_string_lossy().into_owned())
        .collect();
    names.sort();
    for name in names {
        if !valid_part_name(&name, path(&context.paths, "part_prefix")) {
            continue;
        }
        let relative = format!("{}/{name}", path(&context.paths, "part_directory"));
        if expected.contains(relative.as_str()) {
            continue;
        }
        fs::remove_file(absolute(base, &relative)).map_err(|error| {
            format!(
                "cannot remove stale generated title part '{relative}': {}",
                os_error(&error)
            )
        })?;
    }
    atomic_write(
        &absolute(base, &context.projection[0].path),
        context.projection[0].raw.as_slice(),
    )?;
    drop(cleanup);
    if !quiet {
        output.out(&format!(
            "fact-card-catalog: wrote bounded landing plus {} title parts for {} cards.\n",
            context.projection.len() - 1,
            context.cards.len()
        ));
    }
    Ok(())
}

fn fail(errors: &[String], output: &mut dyn super::Output) -> i32 {
    for error in errors {
        output.err(&format!("fact-card-catalog: {error}\n"));
    }
    output.err(&format!(
        "fact-card-catalog: FAILED with {} violation(s).\n",
        errors.len()
    ));
    1
}

fn usage(output: &mut dyn super::Output) -> i32 {
    let program = std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "cargo xtask check fact-card-catalog".to_string());
    output.err(&format!(
        "Usage: {program} [--root DIR] [--contract PATH] [--check|--write|--report|--print-plan|--self-test]\n"
    ));
    2
}

/// Run the fact-card catalog with its original mode and path arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut requested_root = None;
    let mut contract = DEFAULT_CONTRACT.to_string();
    let mut mode = Mode::Check;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" => mode = Mode::Check,
            "--write" => mode = Mode::Write,
            "--report" => mode = Mode::Report,
            "--print-plan" => mode = Mode::PrintPlan,
            "--self-test" => mode = Mode::SelfTest,
            "--root" => match arguments.next() {
                Some(directory) => requested_root = Some(PathBuf::from(directory)),
                None => return usage(output),
            },
            "--contract" => match arguments.next() {
                Some(path) => contract = path.to_string(),
                None => return usage(output),
            },
            _ => return usage(output),
        }
    }
    if mode == Mode::SelfTest {
        return match run_self_test(root) {
            Ok(()) => {
                output.out("fact-card-catalog: self-test 60/60 source/plan/route/residue/bound cases pass.\n");
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
            let resolved = fs::canonicalize(&directory).ok().or_else(|| {
                let name = directory.file_name()?;
                let parent = directory
                    .parent()
                    .filter(|parent| !parent.as_os_str().is_empty())
                    .unwrap_or_else(|| Path::new("."));
                Some(fs::canonicalize(parent).ok()?.join(name))
            });
            match resolved {
                Some(root) => root,
                None => {
                    output.err("fact-card-catalog: root does not exist\n");
                    return 2;
                }
            }
        }
        None => root.to_path_buf(),
    };
    if mode == Mode::PrintPlan {
        let (errors, context) = prepare_context(&root, &contract, true);
        if !errors.is_empty() {
            return fail(&errors, output);
        }
        let Some(context) = context else {
            return 1;
        };
        output.out(&format!("{}\n", pretty_json(&context.planned)));
        return 0;
    }
    if mode == Mode::Write {
        let validation = validate_tree(&root, &contract, true);
        if !validation.errors.is_empty() {
            return fail(&validation.errors, output);
        }
        let Some(context) = validation.context else {
            return 1;
        };
        if let Err(error) = write_projection(&root, &context, false, output) {
            output.err(&format!("{error}\n"));
            return 1;
        }
    }
    let validation = validate_tree(&root, &contract, false);
    if !validation.errors.is_empty() {
        return fail(&validation.errors, output);
    }
    for warning in &validation.warnings {
        output.err(&format!("fact-card-catalog: warning: {warning}\n"));
    }
    if mode == Mode::Report {
        let result = object(vec![
            ("migration_state", text(&validation.state)),
            ("card_count", integer(validation.card_count)),
            ("planned_outputs", validation.planned),
            (
                "warnings",
                Py::List(
                    validation
                        .warnings
                        .iter()
                        .map(|warning| text(warning))
                        .collect(),
                ),
            ),
        ]);
        output.out(&format!("{}\n", canonical_json(&result)));
    } else {
        output.out(&format!(
            "fact-card-catalog: {} projection is valid for {} canonical cards.\n",
            validation.state, validation.card_count
        ));
    }
    0
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

fn write_raw(base: &Path, relative: &str, raw: &[u8]) -> Result<(), String> {
    let path = absolute(base, relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            format!(
                "fact-card-catalog self-test: cannot write {relative}: {}",
                os_error(&error)
            )
        })?;
    }
    fs::write(&path, raw).map_err(|error| {
        format!(
            "fact-card-catalog self-test: cannot write {relative}: {}",
            os_error(&error)
        )
    })
}

fn write_json_raw(base: &Path, relative: &str, value: &Py, pretty: bool) -> Result<(), String> {
    let encoded = if pretty {
        format!("{}\n", pretty_json(value))
    } else {
        canonical_json(value)
    };
    write_raw(base, relative, encoded.as_bytes())
}

fn fixture_card(id: &str, title: &str) -> String {
    format!(
        "---\nid: {id}\ntitle: {title}\nanswers:\n  - \"where is {id}\"\ndate: 2026-08-09\nevidence: docs/example.md\n---\n\nFixture fact.\n"
    )
}

fn self_test_paths() -> BTreeMap<String, String> {
    [
        ("card_directory", "docs/knowledge"),
        ("collection_readme", "docs/knowledge/README.md"),
        ("landing", "docs/knowledge/INDEX.md"),
        ("part_directory", "docs/knowledge-catalog"),
        ("part_prefix", "titles-"),
    ]
    .into_iter()
    .map(|(key, value)| (key.to_string(), value.to_string()))
    .collect()
}

fn fixture_question_limits() -> Py {
    let max_facts = 504 + 58 - 1;
    object(vec![
        ("max_facts", integer(max_facts)),
        (
            "max_question_keys",
            integer(derived_question_keys(max_facts)),
        ),
    ])
}

fn fixture_contract(state: &str) -> Py {
    object(vec![
        ("schema_version", integer(1)),
        ("migration_state", text(state)),
        ("surface_id", text("knowledge_cards")),
        ("part_surface_id", text("fact_card_titles")),
        (
            "paths",
            object(vec![
                ("card_directory", text("docs/knowledge")),
                ("collection_readme", text("docs/knowledge/README.md")),
                ("landing", text("docs/knowledge/INDEX.md")),
                ("part_directory", text("docs/knowledge-catalog")),
                ("part_prefix", text("titles-")),
                (
                    "surface_registry",
                    text("doctrine/live_document_size/surfaces.jsonl"),
                ),
                (
                    "question_contract",
                    text("doctrine/knowledge_map/shard_contract.json"),
                ),
            ]),
        ),
        (
            "legacy",
            object(vec![
                ("boundary_commit", text(&"0".repeat(40))),
                ("git_blob", text(&"0".repeat(40))),
                ("sha256", text(&"0".repeat(64))),
                (
                    "metrics",
                    metrics_json(Metrics {
                        lines: 1,
                        bytes: 1,
                        line_bytes: 1,
                    }),
                ),
                ("card_count", integer(3)),
                ("row_sha256", text(&"0".repeat(64))),
            ]),
        ),
        ("limits", Limits::fixed().json()),
        ("planned_outputs", Py::List(Vec::new())),
        ("verifier", text(VERIFIER)),
    ])
}

fn fixture_surface(files: usize, state: &str) -> Py {
    let index_contract = if state == "migrated" {
        object(vec![
            ("kind", text("routed_membership")),
            ("verifier", text("builtin:markdown_links")),
            ("route_surface", text("fact_card_titles")),
        ])
    } else {
        object(vec![
            ("kind", text("membership")),
            ("verifier", text("builtin:markdown_links")),
        ])
    };
    object(vec![
        ("surface_id", text("knowledge_cards")),
        ("targets", Py::List(vec![text("docs/knowledge/*.md")])),
        ("locator", text("file")),
        ("lifecycle", text("partitioned_canonical")),
        ("state", text("normal")),
        ("owner", text("fixture")),
        ("health_targets", object(vec![("files", integer(files))])),
        (
            "enforcement_ceilings",
            object(vec![("files", integer(files))]),
        ),
        (
            "milestones",
            object(vec![
                ("warning_pct", integer(80)),
                ("rollover_pct", integer(90)),
            ]),
        ),
        ("verifier", text("scripts/check_fact_card_catalog.pl")),
        ("index", text("docs/knowledge/INDEX.md")),
        ("index_contract", index_contract),
    ])
}

fn fixture_record_surface(files: usize) -> Py {
    object(vec![
        ("surface_id", text("decision_records")),
        ("targets", Py::List(vec![text("docs/decisions/*.md")])),
        ("locator", text("collection")),
        ("lifecycle", text("partitioned_canonical")),
        ("state", text("normal")),
        ("owner", text("fixture")),
        ("health_targets", object(vec![("files", integer(files))])),
        (
            "enforcement_ceilings",
            object(vec![("files", integer(files))]),
        ),
        (
            "milestones",
            object(vec![
                ("warning_pct", integer(80)),
                ("rollover_pct", integer(90)),
            ]),
        ),
        ("verifier", text("builtin:budget")),
        ("index", text("docs/decisions/INDEX.md")),
        (
            "index_contract",
            object(vec![
                ("kind", text("membership")),
                ("verifier", text("builtin:markdown_links")),
            ]),
        ),
    ])
}

fn fixture_part_surface() -> Py {
    let limits = Limits::fixed();
    object(vec![
        ("surface_id", text("fact_card_titles")),
        (
            "targets",
            Py::List(vec![text("docs/knowledge-catalog/titles-*.md")]),
        ),
        ("locator", text("collection")),
        ("lifecycle", text("generated_projection")),
        ("state", text("normal")),
        ("owner", text("knowledge-maintainers")),
        (
            "health_targets",
            collection_limits_json(limits.title_health),
        ),
        (
            "enforcement_ceilings",
            collection_limits_json(limits.title_ceilings),
        ),
        (
            "milestones",
            object(vec![
                ("warning_pct", integer(80)),
                ("rollover_pct", integer(90)),
            ]),
        ),
        ("verifier", text("builtin:budget")),
        (
            "canonical_inputs",
            Py::List(vec![text("docs/knowledge/*.md")]),
        ),
        (
            "freshness_verifier",
            text("scripts/check_fact_card_catalog.pl"),
        ),
        ("index", text("docs/knowledge/INDEX.md")),
        (
            "index_contract",
            object(vec![
                ("kind", text("external_membership")),
                ("verifier", text("builtin:markdown_links")),
            ]),
        ),
    ])
}

fn git_status(base: &Path, arguments: &[&str]) -> Result<(), String> {
    let base_text = base.to_string_lossy().into_owned();
    let status = Command::new("git")
        .arg("-C")
        .arg(&base_text)
        .args(arguments)
        .status()
        .map_err(|error| {
            format!(
                "fact-card-catalog self-test: git failed: {}",
                os_error(&error)
            )
        })?;
    if status.success() {
        Ok(())
    } else {
        Err("fact-card-catalog self-test: git command failed".to_string())
    }
}

#[derive(Clone, Copy)]
enum Mutation {
    UnknownContractField,
    UnsafeLanding,
    LimitInflation,
    SurfaceCapacity,
    SurfaceMilestone,
    PrematureSurface,
    QuestionCapacity,
    QuestionKeys,
    DecisionCapacity,
    BoundaryCommit,
    BoundaryBlob,
    BoundaryDigest,
    BoundaryMetric,
    BoundaryRows,
    WorkingLanding,
    StageZero,
    CanonicalRow,
    PrematureDirectory,
    PlannedHash,
    PlannedDuplicate,
    PlannedAboveLimit,
    MissingTitleSurface,
    TitleSurfaceDrift,
    MembershipKind,
    RouteSurface,
    MissingPart,
    StalePart,
    UnexpectedResidue,
    LandingDrift,
    LinkDrift,
    CardAddition,
}

fn write_surfaces(base: &Path, surfaces: &[Py]) -> Result<(), String> {
    let mut raw = String::new();
    for surface in surfaces {
        raw.push_str(&canonical_json(surface));
        raw.push('\n');
    }
    write_raw(
        base,
        "doctrine/live_document_size/surfaces.jsonl",
        raw.as_bytes(),
    )
}

fn apply_mutation(
    base: &Path,
    contract: &mut Py,
    legacy: &[u8],
    projection: &[Rendered],
    mutation: Mutation,
) -> Result<(), String> {
    match mutation {
        Mutation::UnknownContractField => set_field(contract, "unknown", integer(1)),
        Mutation::UnsafeLanding => {
            if let Some(value) = nested_field_mut(contract, &["paths", "landing"]) {
                *value = text("../INDEX.md");
            }
        }
        Mutation::LimitInflation => {
            if let Some(value) = nested_field_mut(contract, &["limits", "max_cards"]) {
                *value = integer(505);
            }
        }
        Mutation::SurfaceCapacity => {
            write_surfaces(base, &[fixture_surface(507, "legacy_locked")])?
        }
        Mutation::SurfaceMilestone => {
            let mut surface = fixture_surface(506, "legacy_locked");
            if let Some(value) = nested_field_mut(&mut surface, &["milestones", "rollover_pct"]) {
                *value = integer(91);
            }
            write_surfaces(base, &[surface])?;
        }
        Mutation::PrematureSurface => write_surfaces(
            base,
            &[
                fixture_surface(506, "legacy_locked"),
                fixture_part_surface(),
            ],
        )?,
        Mutation::QuestionCapacity | Mutation::QuestionKeys => {
            let mut limits = fixture_question_limits();
            if matches!(mutation, Mutation::QuestionCapacity) {
                set_field(&mut limits, "max_facts", integer(562));
            } else {
                set_field(&mut limits, "max_question_keys", integer(5120));
            }
            write_json_raw(
                base,
                "doctrine/knowledge_map/shard_contract.json",
                &object(vec![
                    ("fact_catalog", text("docs/knowledge/INDEX.md")),
                    ("limits", limits),
                ]),
                false,
            )?;
        }
        Mutation::DecisionCapacity => write_surfaces(
            base,
            &[
                fixture_surface(506, "legacy_locked"),
                fixture_record_surface(59),
            ],
        )?,
        Mutation::BoundaryCommit
        | Mutation::BoundaryBlob
        | Mutation::BoundaryDigest
        | Mutation::BoundaryRows => {
            let (field, length) = match mutation {
                Mutation::BoundaryCommit => ("boundary_commit", 40),
                Mutation::BoundaryBlob => ("git_blob", 40),
                Mutation::BoundaryDigest => ("sha256", 64),
                _ => ("row_sha256", 64),
            };
            if let Some(value) = nested_field_mut(contract, &["legacy", field]) {
                *value = text(&"f".repeat(length));
            }
        }
        Mutation::BoundaryMetric => {
            if let Some(value) = nested_field_mut(contract, &["legacy", "metrics", "bytes"]) {
                *value = integer(metrics(legacy).bytes + 1);
            }
        }
        Mutation::WorkingLanding => write_raw(
            base,
            "docs/knowledge/INDEX.md",
            &[legacy, b"changed\n"].concat(),
        )?,
        Mutation::StageZero => {
            write_raw(
                base,
                "docs/knowledge/INDEX.md",
                &[legacy, b"stage\n"].concat(),
            )?;
            git_status(base, &["add", "--", "docs/knowledge/INDEX.md"])?;
            write_raw(base, "docs/knowledge/INDEX.md", legacy)?;
        }
        Mutation::CanonicalRow => write_raw(
            base,
            "docs/knowledge/alpha.md",
            fixture_card("alpha", "Changed title").as_bytes(),
        )?,
        Mutation::PrematureDirectory => fs::create_dir_all(base.join("docs/knowledge-catalog"))
            .map_err(|error| os_error(&error))?,
        Mutation::PlannedHash => {
            if let Some(Py::List(records)) = nested_field_mut(contract, &["planned_outputs"])
                && let Some(record) = records.first_mut()
            {
                set_field(record, "sha256", text(&"f".repeat(64)));
            }
        }
        Mutation::PlannedDuplicate => {
            if let Some(Py::List(records)) = nested_field_mut(contract, &["planned_outputs"])
                && records.len() > 1
            {
                let first = scalar(field(&records[0], "path"));
                set_field(&mut records[1], "path", text(&first));
            }
        }
        Mutation::PlannedAboveLimit => {
            if let Some(Py::List(records)) = nested_field_mut(contract, &["planned_outputs"])
                && let Some(first) = records.first().cloned()
            {
                while records.len() <= PLANNED_OUTPUT_LIMIT {
                    records.push(first.clone());
                }
            }
        }
        Mutation::MissingTitleSurface => write_surfaces(base, &[fixture_surface(338, "migrated")])?,
        Mutation::TitleSurfaceDrift => {
            let mut part = fixture_part_surface();
            if let Some(value) = nested_field_mut(&mut part, &["health_targets", "lines_each"]) {
                *value = integer(81);
            }
            write_surfaces(base, &[fixture_surface(338, "migrated"), part])?;
        }
        Mutation::MembershipKind => write_surfaces(
            base,
            &[
                fixture_surface(338, "legacy_locked"),
                fixture_part_surface(),
            ],
        )?,
        Mutation::RouteSurface => {
            let mut surface = fixture_surface(338, "migrated");
            if let Some(value) =
                nested_field_mut(&mut surface, &["index_contract", "route_surface"])
            {
                *value = text("other_titles");
            }
            write_surfaces(base, &[surface, fixture_part_surface()])?;
        }
        Mutation::MissingPart => {
            fs::remove_file(base.join(&projection[1].path)).map_err(|error| os_error(&error))?
        }
        Mutation::StalePart => {
            write_raw(base, "docs/knowledge-catalog/titles-9999.md", b"stale\n")?
        }
        Mutation::UnexpectedResidue => {
            write_raw(base, "docs/knowledge-catalog/notes.md", b"residue\n")?
        }
        Mutation::LandingDrift => write_raw(
            base,
            &projection[0].path,
            &[projection[0].raw.as_slice(), b"changed\n"].concat(),
        )?,
        Mutation::LinkDrift => write_raw(
            base,
            &projection[1].path,
            &String::from_utf8(projection[1].raw.clone())
                .expect("fixture projection is UTF-8")
                .replace("../knowledge/alpha.md", "alpha.md")
                .into_bytes(),
        )?,
        Mutation::CardAddition => write_raw(
            base,
            "docs/knowledge/delta.md",
            fixture_card("delta", "Delta title").as_bytes(),
        )?,
    }
    Ok(())
}

fn init_fixture(base: &Path, state: &str, mutation: Option<Mutation>) -> Result<(), String> {
    let mut contract = fixture_contract(state);
    let mut surfaces = vec![fixture_surface(506, state), fixture_record_surface(58)];
    if state == "migrated" {
        surfaces.push(fixture_part_surface());
    }
    write_surfaces(base, &surfaces)?;
    write_json_raw(
        base,
        "doctrine/knowledge_map/shard_contract.json",
        &object(vec![
            ("fact_catalog", text("docs/knowledge/INDEX.md")),
            ("limits", fixture_question_limits()),
        ]),
        true,
    )?;
    write_raw(base, "docs/knowledge/README.md", b"# Cards\n")?;
    write_raw(
        base,
        "docs/knowledge/alpha.md",
        fixture_card("alpha", "Alpha title").as_bytes(),
    )?;
    write_raw(
        base,
        "docs/knowledge/beta.md",
        fixture_card("beta", "Beta | title").as_bytes(),
    )?;
    write_raw(
        base,
        "docs/knowledge/gamma.md",
        fixture_card("gamma", "Gamma — title").as_bytes(),
    )?;
    let mut errors = Vec::new();
    let cards = collect_cards(base, &self_test_paths(), Limits::fixed(), &mut errors);
    if !errors.is_empty() {
        return Err(format!("fixture card parse failed: {}", errors.join("; ")));
    }
    let legacy = render_legacy(&cards, Limits::fixed(), &mut errors);
    if !errors.is_empty() {
        return Err(format!(
            "fixture legacy render failed: {}",
            errors.join("; ")
        ));
    }
    write_raw(base, "docs/knowledge/INDEX.md", legacy.as_slice())?;
    git_status(base, &["init", "-q"])?;
    git_status(base, &["add", "--", "docs/knowledge"])?;
    git_status(
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
            "fixture fact catalog boundary",
        ],
    )?;
    let (commit, commit_exit, _) = git_capture(base, &["rev-parse", "HEAD"]);
    if commit_exit != 0 {
        return Err("fixture commit lookup failed".to_string());
    }
    let commit = trim_newlines(&commit);
    let (blob, blob_exit, _) = git_capture(base, &["rev-parse", "HEAD:docs/knowledge/INDEX.md"]);
    if blob_exit != 0 {
        return Err("fixture blob lookup failed".to_string());
    }
    let blob = trim_newlines(&blob);
    let rows = legacy_rows(legacy.as_slice());
    set_field(
        &mut contract,
        "legacy",
        object(vec![
            ("boundary_commit", text(&commit)),
            ("git_blob", text(&blob)),
            ("sha256", text(&sha256_hex(legacy.as_slice()))),
            ("metrics", metrics_json(metrics(legacy.as_slice()))),
            ("card_count", integer(cards.len())),
            ("row_sha256", text(&row_digest(&rows))),
        ]),
    );
    let projection = render_projection(&cards, &self_test_paths(), Limits::fixed(), &mut errors);
    if !errors.is_empty() {
        return Err(format!(
            "fixture projection render failed: {}",
            errors.join("; ")
        ));
    }
    set_field(
        &mut contract,
        "planned_outputs",
        planned_records(&projection),
    );
    if state == "migrated" {
        for output in &projection {
            write_raw(base, &output.path, output.raw.as_slice())?;
        }
    }
    if let Some(mutation) = mutation {
        apply_mutation(base, &mut contract, &legacy, &projection, mutation)?;
    }
    write_json_raw(base, DEFAULT_CONTRACT, &contract, true)
}

fn require(condition: bool, message: impl FnOnce() -> String) -> Result<(), String> {
    if condition { Ok(()) } else { Err(message()) }
}

fn synthetic_cards(count: usize, capacity: bool) -> Vec<Card> {
    (1..=count)
        .map(|index| {
            let (id, title, status) = if capacity {
                let prefix = format!("fact-{index:03}-");
                (
                    format!("{prefix}{}", "x".repeat(64 - prefix.len())),
                    "T".repeat(112),
                    "superseded".to_string(),
                )
            } else {
                (
                    format!("fact-{index:03}"),
                    format!("Fact {index}"),
                    "current".to_string(),
                )
            };
            Card {
                name: format!("{id}.md"),
                path: format!("docs/knowledge/{id}.md"),
                id,
                title: pyjson::py_str(&title),
                date: "2026-08-09".to_string(),
                status,
            }
        })
        .collect()
}

fn run_self_test(base: &Path) -> Result<(), String> {
    let generated = base.join("generated");
    fs::create_dir_all(&generated).map_err(|error| os_error(&error))?;
    let limits = Limits::fixed();
    require(
        limits.max_cards == limits.cards_per_part * limits.max_parts,
        || {
            "fact-card-catalog parser self-test: max_cards is not the derived part capacity"
                .to_string()
        },
    )?;
    for (name, part) in [
        ("health_targets", limits.title_health),
        ("enforcement_ceilings", limits.title_ceilings),
    ] {
        require(part.files == limits.max_parts, || {
            format!(
                "fact-card-catalog parser self-test: title-part {name} files disagree with max_parts"
            )
        })?;
        require(part.lines_total == part.files * part.lines_each, || {
            format!(
                "fact-card-catalog parser self-test: title-part {name} lines_total is not files x lines_each"
            )
        })?;
        require(part.bytes_total == part.files * part.bytes_each, || {
            format!(
                "fact-card-catalog parser self-test: title-part {name} bytes_total is not files x bytes_each"
            )
        })?;
    }
    require(
        limits.projection_files == PLANNED_OUTPUT_LIMIT
            && limits.projection_lines
                == limits.landing_ceilings.lines + limits.title_ceilings.lines_total
            && limits.projection_bytes
                == limits.landing_ceilings.bytes + limits.title_ceilings.bytes_total,
        || {
            "fact-card-catalog parser self-test: projection ceiling does not cover landing plus parts".to_string()
        },
    )?;

    let normal = fixture_card("alpha", "Alpha title");
    let parsed = parse_card("alpha.md", &pyjson::py_str(&normal), limits)?;
    require(parsed.status == "current", || {
        "fact-card-catalog parser self-test: default status failed".to_string()
    })?;
    let pipe = parse_card(
        "beta.md",
        &pyjson::py_str(&fixture_card("beta", "Beta | title")),
        limits,
    )?;
    require(
        String::from_utf8(legacy_row(&pipe, limits))
            .expect("fixture row is UTF-8")
            .contains("Beta \\| title"),
        || "fact-card-catalog parser self-test: Markdown pipe escaping failed".to_string(),
    )?;
    let parser_failures = [
        (
            "missing answers",
            normal.replace("answers:\n  - \"where is alpha\"\n", ""),
            "alpha.md",
            "answers",
        ),
        (
            "filename mismatch",
            normal.clone(),
            "other.md",
            "does not match filename",
        ),
        (
            "bad date",
            normal.replace("2026-08-09", "2026-13-09"),
            "alpha.md",
            "YYYY-MM-DD",
        ),
        (
            "missing evidence",
            normal.replace("evidence: docs/example.md\n", ""),
            "alpha.md",
            "evidence or reverify",
        ),
        (
            "duplicate id",
            normal.replace("title:", "id: duplicate\ntitle:"),
            "alpha.md",
            "repeats front-matter key",
        ),
    ];
    for (name, raw, filename, expected) in parser_failures {
        let result = parse_card(filename, &pyjson::py_str(&raw), limits);
        require(result.is_err(), || {
            format!("fact-card-catalog parser self-test '{name}' unexpectedly passed")
        })?;
        require(
            result.err().is_some_and(|error| error.contains(expected)),
            || format!("fact-card-catalog parser self-test '{name}' missed {expected}"),
        )?;
    }
    let long_title = "x".repeat(limits.max_source_title_bytes + 1);
    require(
        parse_card(
            "alpha.md",
            &pyjson::py_str(&normal.replace("Alpha title", &long_title)),
            limits,
        )
        .is_err(),
        || "fact-card-catalog parser self-test: oversized title passed".to_string(),
    )?;

    let packing_cards = synthetic_cards(57, false);
    let mut packing_errors = Vec::new();
    let packing = render_projection(
        &packing_cards,
        &self_test_paths(),
        limits,
        &mut packing_errors,
    );
    require(packing_errors.is_empty() && packing.len() == 3, || {
        format!(
            "fact-card-catalog parser self-test: 57-card packing failed: {}",
            packing_errors.join("; ")
        )
    })?;
    let overflow_cards = synthetic_cards(limits.max_cards + 1, false);
    let mut overflow_errors = Vec::new();
    render_projection(
        &overflow_cards,
        &self_test_paths(),
        limits,
        &mut overflow_errors,
    );
    require(
        overflow_errors
            .iter()
            .any(|error| error.contains("card count exceeds migrated maximum 504")),
        || "fact-card-catalog parser self-test: 505-card ceiling did not fail closed".to_string(),
    )?;

    let capacity_cards = synthetic_cards(limits.max_cards, true);
    let mut capacity_errors = Vec::new();
    let capacity_projection = render_projection(
        &capacity_cards,
        &self_test_paths(),
        limits,
        &mut capacity_errors,
    );
    let (capacity_landing_errors, _) = pressure_findings(
        metrics(capacity_projection[0].raw.as_slice()),
        limits.landing_health,
        "capacity landing",
    );
    let (capacity_part_errors, _) = collection_pressure_findings(
        aggregate_metrics(&capacity_projection[1..]),
        limits.title_health,
        "capacity title parts",
    );
    require(
        capacity_errors.is_empty()
            && capacity_landing_errors.is_empty()
            && capacity_part_errors.is_empty(),
        || {
            format!(
                "fact-card-catalog parser self-test: exact 504-card capacity crosses mandatory pressure: {}",
                [
                    capacity_errors,
                    capacity_landing_errors,
                    capacity_part_errors
                ]
                .concat()
                .join("; ")
            )
        },
    )?;
    let mut single_errors = Vec::new();
    let single = render_projection(
        &capacity_cards[..1],
        &self_test_paths(),
        limits,
        &mut single_errors,
    );
    require(single_errors.is_empty(), || {
        format!(
            "fact-card-catalog parser self-test: single-card projection failed: {}",
            single_errors.join("; ")
        )
    })?;
    for (projection, parts) in [
        (&single, 1),
        (&packing, 2),
        (&capacity_projection, limits.max_parts),
    ] {
        let lines = metrics(projection[0].raw.as_slice()).lines;
        require(lines == parts + 6 && projection.len() == parts + 1, || {
            format!(
                "fact-card-catalog parser self-test: landing is {lines} lines for {parts} parts, expected {}",
                parts + 6
            )
        })?;
    }
    let mut landing_link_errors = Vec::new();
    let landing_links = resolved_links(
        &capacity_projection[0].raw,
        "docs/knowledge/INDEX.md",
        "capacity landing",
        &mut landing_link_errors,
    );
    require(landing_link_errors.is_empty(), || {
        format!(
            "fact-card-catalog parser self-test: capacity landing has unsafe links: {}",
            landing_link_errors.join("; ")
        )
    })?;
    require(
        !capacity_cards
            .iter()
            .any(|card| landing_links.contains_key(&card.path)),
        || {
            "fact-card-catalog parser self-test: the router landing still links a card directly"
                .to_string()
        },
    )?;
    let landing =
        String::from_utf8(capacity_projection[0].raw.clone()).expect("capacity landing is UTF-8");
    let miscounted = landing.replacen("| 56 | ", "| 55 | ", 1);
    let renamed = landing.replacen(
        &format!("| `{}`", capacity_cards[0].id),
        "| `fact-000-renamed`",
        1,
    );
    let dropped = landing
        .split_inclusive('\n')
        .filter(|line| !line.starts_with("| [0002]"))
        .collect::<String>();
    for (name, raw, expected) in [
        ("miscounted part", miscounted, "miscounts its title part"),
        ("renamed first id", renamed, "names the wrong first id"),
        (
            "dropped range row",
            dropped,
            "exactly one range row per title part",
        ),
    ] {
        let mut range_errors = Vec::new();
        validate_landing_ranges(
            raw.as_bytes(),
            &capacity_cards,
            limits,
            limits.max_parts,
            &mut range_errors,
        );
        require(range_errors.join("\n").contains(expected), || {
            format!(
                "fact-card-catalog parser self-test: range rule missed '{name}': {}",
                range_errors.join("; ")
            )
        })?;
    }
    let (rollover_errors, rollover_warnings) = pressure_findings(
        Metrics {
            lines: 90,
            bytes: 80,
            line_bytes: 1,
        },
        Metrics {
            lines: 100,
            bytes: 100,
            line_bytes: 100,
        },
        "synthetic projection",
    );
    require(
        rollover_errors.len() == 1
            && rollover_warnings.len() == 1
            && rollover_errors[0].contains("mandatory rollover")
            && rollover_warnings[0].contains("bytes is at 80.0%"),
        || "fact-card-catalog parser self-test: rollover/warning pressure split failed".to_string(),
    )?;

    let cases = [
        ("legacy positive", "legacy_locked", None, ""),
        (
            "unknown contract field",
            "legacy_locked",
            Some(Mutation::UnknownContractField),
            "unknown field",
        ),
        (
            "unsafe landing path",
            "legacy_locked",
            Some(Mutation::UnsafeLanding),
            "landing is unsafe",
        ),
        (
            "fixed limit inflation",
            "legacy_locked",
            Some(Mutation::LimitInflation),
            "limits differ",
        ),
        (
            "surface capacity drift",
            "legacy_locked",
            Some(Mutation::SurfaceCapacity),
            "file health/ceiling must remain 506",
        ),
        (
            "surface milestone drift",
            "legacy_locked",
            Some(Mutation::SurfaceMilestone),
            "milestones must remain warning 80",
        ),
        (
            "premature title surface",
            "legacy_locked",
            Some(Mutation::PrematureSurface),
            "must be absent while legacy_locked",
        ),
        (
            "question capacity drift",
            "legacy_locked",
            Some(Mutation::QuestionCapacity),
            "max_facts must fund",
        ),
        (
            "question-key capacity drift",
            "legacy_locked",
            Some(Mutation::QuestionKeys),
            "max_question_keys must equal",
        ),
        (
            "decision-record capacity drift",
            "legacy_locked",
            Some(Mutation::DecisionCapacity),
            "max_facts must fund",
        ),
        (
            "boundary commit drift",
            "legacy_locked",
            Some(Mutation::BoundaryCommit),
            "boundary commit lookup",
        ),
        (
            "boundary blob drift",
            "legacy_locked",
            Some(Mutation::BoundaryBlob),
            "boundary blob",
        ),
        (
            "boundary digest drift",
            "legacy_locked",
            Some(Mutation::BoundaryDigest),
            "SHA-256",
        ),
        (
            "boundary metric drift",
            "legacy_locked",
            Some(Mutation::BoundaryMetric),
            "legacy boundary bytes",
        ),
        (
            "boundary row drift",
            "legacy_locked",
            Some(Mutation::BoundaryRows),
            "row digest",
        ),
        (
            "working landing drift",
            "legacy_locked",
            Some(Mutation::WorkingLanding),
            "working legacy landing differs",
        ),
        (
            "stage-zero drift",
            "legacy_locked",
            Some(Mutation::StageZero),
            "stage-zero landing blob",
        ),
        (
            "canonical row drift",
            "legacy_locked",
            Some(Mutation::CanonicalRow),
            "canonical card rendering",
        ),
        (
            "premature title directory",
            "legacy_locked",
            Some(Mutation::PrematureDirectory),
            "premature title-part directory",
        ),
        (
            "planned hash drift",
            "legacy_locked",
            Some(Mutation::PlannedHash),
            "planned_outputs membership",
        ),
        (
            "planned duplicate path",
            "legacy_locked",
            Some(Mutation::PlannedDuplicate),
            "duplicate path|membership/order",
        ),
        (
            "planned output above derived limit",
            "legacy_locked",
            Some(Mutation::PlannedAboveLimit),
            "planned_outputs must contain one to 10 output records",
        ),
        ("migrated positive", "migrated", None, ""),
        (
            "missing migrated title surface",
            "migrated",
            Some(Mutation::MissingTitleSurface),
            "must contain title-part surface",
        ),
        (
            "migrated title surface drift",
            "migrated",
            Some(Mutation::TitleSurfaceDrift),
            "title-part surface health targets differ",
        ),
        (
            "migrated membership kind drift",
            "migrated",
            Some(Mutation::MembershipKind),
            "must route membership through the title parts",
        ),
        (
            "migrated route surface drift",
            "migrated",
            Some(Mutation::RouteSurface),
            "must route membership through the title parts",
        ),
        (
            "missing migrated part",
            "migrated",
            Some(Mutation::MissingPart),
            "migrated title_part is missing",
        ),
        (
            "stale migrated part",
            "migrated",
            Some(Mutation::StalePart),
            "stale title-part output",
        ),
        (
            "unexpected part residue",
            "migrated",
            Some(Mutation::UnexpectedResidue),
            "unexpected title-part residue",
        ),
        (
            "migrated landing drift",
            "migrated",
            Some(Mutation::LandingDrift),
            "migrated output differs",
        ),
        (
            "migrated link drift",
            "migrated",
            Some(Mutation::LinkDrift),
            "migrated output differs",
        ),
        (
            "migrated card addition without regeneration",
            "migrated",
            Some(Mutation::CardAddition),
            "migrated output differs",
        ),
    ];
    for (index, (name, state, mutation, expected)) in cases.into_iter().enumerate() {
        let fixture = generated.join(format!(
            ".fact-card-catalog-self-test.{}.{index}",
            std::process::id()
        ));
        if fixture.exists() {
            fs::remove_dir_all(&fixture).map_err(|error| os_error(&error))?;
        }
        fs::create_dir_all(&fixture).map_err(|error| os_error(&error))?;
        let _cleanup = FixtureCleanup(fixture.clone());
        init_fixture(&fixture, state, mutation)?;
        let validation = validate_tree(&fixture, DEFAULT_CONTRACT, false);
        let joined = validation.errors.join("\n");
        if expected.is_empty() {
            require(validation.errors.is_empty(), || {
                format!("fact-card-catalog self-test '{name}' unexpectedly failed:\n{joined}")
            })?;
        } else {
            require(!validation.errors.is_empty(), || {
                format!("fact-card-catalog self-test '{name}' unexpectedly passed")
            })?;
            require(
                expected.split('|').any(|needle| joined.contains(needle)),
                || {
                    format!(
                        "fact-card-catalog self-test '{name}' missed expected diagnostic {expected}:\n{joined}"
                    )
                },
            )?;
        }
    }

    let write_fixture = generated.join(format!(
        ".fact-card-catalog-write-self-test.{}",
        std::process::id()
    ));
    if write_fixture.exists() {
        fs::remove_dir_all(&write_fixture).map_err(|error| os_error(&error))?;
    }
    fs::create_dir_all(&write_fixture).map_err(|error| os_error(&error))?;
    let _cleanup = FixtureCleanup(write_fixture.clone());
    init_fixture(&write_fixture, "legacy_locked", None)?;
    let mut errors = Vec::new();
    let mut contract = read_json(&write_fixture, DEFAULT_CONTRACT, "contract", &mut errors)
        .ok_or_else(|| {
            format!(
                "fact-card-catalog write self-test: cannot read contract: {}",
                errors.join("; ")
            )
        })?;
    set_field(&mut contract, "migration_state", text("migrated"));
    write_json_raw(&write_fixture, DEFAULT_CONTRACT, &contract, true)?;
    write_surfaces(
        &write_fixture,
        &[
            fixture_surface(506, "migrated"),
            fixture_record_surface(58),
            fixture_part_surface(),
        ],
    )?;
    write_raw(
        &write_fixture,
        "docs/knowledge-catalog/titles-9999.md",
        b"stale\n",
    )?;
    let (prepare_errors, context) = prepare_context(&write_fixture, DEFAULT_CONTRACT, false);
    if !prepare_errors.is_empty() {
        return Err(format!(
            "fact-card-catalog write self-test: preparation failed: {}",
            prepare_errors.join("; ")
        ));
    }
    let context = context
        .ok_or_else(|| "fact-card-catalog write self-test: preparation failed".to_string())?;
    let mut sink = super::Captured::default();
    write_projection(&write_fixture, &context, true, &mut sink)?;
    let validation = validate_tree(&write_fixture, DEFAULT_CONTRACT, false);
    if !validation.errors.is_empty() {
        return Err(format!(
            "fact-card-catalog write self-test: migrated output failed: {}",
            validation.errors.join("; ")
        ));
    }
    require(
        !write_fixture
            .join("docs/knowledge-catalog/titles-9999.md")
            .exists(),
        || "fact-card-catalog write self-test: stale generated part survived".to_string(),
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_sixty_cases_and_the_real_projection_hold() {
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
            "fact-card-catalog: self-test 60/60 source/plan/route/residue/bound cases pass.\n"
        );
        output.text.clear();
        assert_eq!(
            run_to(&root, &["--report".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("\"migration_state\":\"migrated\""));
        assert!(output.text.contains("\"planned_outputs\":"));
    }

    #[test]
    fn lone_surrogate_before_card_matches_the_front_matter_refusal() {
        let root = crate::repository_root();
        let fixture = root.join(format!(
            "generated/.fact-card-catalog-surrogate-test.{}",
            std::process::id()
        ));
        if fixture.exists() {
            fs::remove_dir_all(&fixture).expect("remove stale surrogate fixture");
        }
        fs::create_dir_all(&fixture).expect("create surrogate fixture");
        let _cleanup = FixtureCleanup(fixture.clone());
        init_fixture(&fixture, "migrated", None).expect("initialize surrogate fixture");
        let card = fixture.join("docs/knowledge/alpha.md");
        let mut raw = vec![0xed, 0xa0, 0x80];
        raw.extend(fs::read(&card).expect("read fixture card"));
        fs::write(&card, raw).expect("write leading surrogate");

        let mut output = super::super::Captured::default();
        assert_eq!(run_to(&fixture, &[], &mut output), 1);
        assert_eq!(
            output.text,
            concat!(
                "fact-card-catalog: alpha.md lacks leading YAML front matter\n",
                "fact-card-catalog: planned_outputs membership/order differs from deterministic rendering\n",
                "fact-card-catalog: migrated output differs from deterministic rendering: docs/knowledge/INDEX.md\n",
                "fact-card-catalog: migrated output differs from deterministic rendering: docs/knowledge-catalog/titles-0001.md\n",
                "fact-card-catalog: FAILED with 4 violation(s).\n",
            )
        );
    }

    #[test]
    fn valid_surrogate_body_round_trips_and_truncates_by_original_bytes() {
        let root = crate::repository_root();
        let fixture = root.join(format!(
            "generated/.fact-card-catalog-surrogate-body-test.{}",
            std::process::id()
        ));
        if fixture.exists() {
            fs::remove_dir_all(&fixture).expect("remove stale surrogate-body fixture");
        }
        fs::create_dir_all(&fixture).expect("create surrogate-body fixture");
        let _cleanup = FixtureCleanup(fixture.clone());
        init_fixture(&fixture, "migrated", None).expect("initialize surrogate-body fixture");
        let card = fixture.join("docs/knowledge/alpha.md");
        let original = fs::read(&card).expect("read fixture card");
        let insertion = original
            .windows(b"Alpha title".len())
            .position(|window| window == b"Alpha title")
            .expect("fixture title")
            + "Alpha ".len();
        let mut raw = original[..insertion].to_vec();
        raw.extend_from_slice(&[0xed, 0xa0, 0x80]);
        raw.push(b' ');
        raw.extend_from_slice(&original[insertion..]);
        fs::write(&card, raw).expect("write surrogate title");

        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&fixture, &["--print-plan".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert_eq!(
            output.text,
            concat!(
                "[\n",
                "   {\n",
                "      \"metrics\" : {\n",
                "         \"bytes\" : 450,\n",
                "         \"line_bytes\" : 190,\n",
                "         \"lines\" : 7\n",
                "      },\n",
                "      \"path\" : \"docs/knowledge/INDEX.md\",\n",
                "      \"role\" : \"landing\",\n",
                "      \"sha256\" : \"c4f52766e65a50d08ace9396739f94447d77f70b10571babf9f9396b0d8cf4c7\"\n",
                "   },\n",
                "   {\n",
                "      \"metrics\" : {\n",
                "         \"bytes\" : 492,\n",
                "         \"line_bytes\" : 82,\n",
                "         \"lines\" : 10\n",
                "      },\n",
                "      \"path\" : \"docs/knowledge-catalog/titles-0001.md\",\n",
                "      \"role\" : \"title_part\",\n",
                "      \"sha256\" : \"6673ec78d0812e401c671eeffc12e23b73ba676bd01e4a60c31a8287b9048a8f\"\n",
                "   }\n",
                "]\n",
            )
        );

        let mut title = pyjson::py_str("Alpha ");
        title.push(0xd800);
        title.extend(pyjson::py_str(&format!(" {}", "x".repeat(110))));
        let actual = codepoints_bytes(&compact_title(&title, 112));
        let mut expected = b"Alpha ".to_vec();
        expected.extend_from_slice(&[0xed, 0xa0, 0x80]);
        expected.push(b' ');
        expected.extend_from_slice(&"x".repeat(99).into_bytes());
        expected.extend_from_slice("…".as_bytes());
        assert_eq!(actual, expected);
    }
}
