//! Validation snapshot currentness, native since `RUSTDOC-AND-XTASK.15.n`: reviewed source identity,
//! bounded contracts, routed snapshot parts, live projections, producer regions and the 12-case suite.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "validation-snapshot-currentness";

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

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn scalar(value: Option<&Py>) -> String {
    perl::scalar(value).unwrap_or_default()
}

fn problem(errors: &mut Vec<String>, message: impl AsRef<str>) {
    errors.push(message.as_ref().to_string());
}

fn unknown_fields(value: &Py, label: &str, allowed: &[&str], errors: &mut Vec<String>) {
    let Py::Dict(pairs) = value else {
        return;
    };
    for (key, _) in pairs {
        let key = pyjson::to_text(key);
        if !allowed.contains(&key.as_str()) {
            problem(errors, format!("{label} has unknown field '{key}'"));
        }
    }
}

fn object_value<'a>(value: &'a Py, label: &str, errors: &mut Vec<String>) -> &'a Py {
    if matches!(value, Py::Dict(_)) {
        value
    } else {
        problem(errors, format!("{label} must be an object"));
        &Py::None
    }
}

fn object_array<'a>(
    value: Option<&'a Py>,
    label: &str,
    errors: &mut Vec<String>,
    max: usize,
) -> Vec<&'a Py> {
    let Some(Py::List(items)) = value else {
        problem(errors, format!("{label} must be an array"));
        return Vec::new();
    };
    if items.len() > max {
        problem(errors, format!("{label} exceeds {max} entries"));
    }
    for item in items {
        if !matches!(item, Py::Dict(_)) {
            problem(errors, format!("{label} entries must be objects"));
        }
    }
    items
        .iter()
        .filter(|item| matches!(item, Py::Dict(_)))
        .collect()
}

fn scalar_string(value: Option<&Py>, label: &str, errors: &mut Vec<String>) -> Option<String> {
    if value.is_none_or(perl::is_reference) {
        problem(
            errors,
            format!("{label} must be a bounded non-empty string"),
        );
        return None;
    }
    let value = scalar(value);
    if value.is_empty() || value.len() > 1_024 {
        problem(
            errors,
            format!("{label} must be a bounded non-empty string"),
        );
        return None;
    }
    Some(value)
}

fn string_array(
    value: Option<&Py>,
    label: &str,
    errors: &mut Vec<String>,
    max: usize,
) -> Vec<String> {
    let Some(Py::List(items)) = value else {
        problem(errors, format!("{label} must be an array"));
        return Vec::new();
    };
    if items.len() > max {
        problem(errors, format!("{label} exceeds {max} entries"));
    }
    let mut result = Vec::new();
    for item in items {
        if perl::is_reference(item) {
            problem(errors, format!("{label} entries must be non-empty strings"));
            continue;
        }
        let item = perl::scalar(Some(item)).unwrap_or_default();
        if item.is_empty() || item.len() > 512 {
            problem(errors, format!("{label} entries must be non-empty strings"));
        }
        if !item.is_empty() {
            result.push(item);
        }
    }
    result
}

fn safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && path.chars().count() <= 512
        && !path.starts_with('/')
        && !path.contains(['\\', '\0'])
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

fn safe_path_value(value: Option<&Py>, label: &str, errors: &mut Vec<String>) -> Option<String> {
    let path = scalar_string(value, label, errors)?;
    if !safe_relative_path(&path) {
        problem(
            errors,
            format!("{label} must be a safe repository-relative path"),
        );
        return None;
    }
    Some(path)
}

fn positive_integer(value: Option<&Py>, label: &str, errors: &mut Vec<String>) {
    if !valid_digits(value) || perl::numeric(value) < 1.0 {
        problem(errors, format!("{label} must be a positive integer"));
    }
}

fn nonnegative_integer(value: Option<&Py>, label: &str, errors: &mut Vec<String>) {
    if !valid_digits(value) {
        problem(errors, format!("{label} must be a nonnegative integer"));
    }
}

fn bounded_integer(
    value: Option<&Py>,
    label: &str,
    min: usize,
    max: usize,
    errors: &mut Vec<String>,
) {
    let number = perl::numeric(value);
    if !valid_digits(value) || number < min as f64 || number > max as f64 {
        problem(
            errors,
            format!("{label} must be an integer from {min} through {max}"),
        );
    }
}

fn valid_digits(value: Option<&Py>) -> bool {
    value.is_some_and(|value| !perl::is_reference(value))
        && !scalar(value).is_empty()
        && scalar(value)
            .chars()
            .all(|character| character.is_ascii_digit())
}

fn optional_integer(value: Option<&Py>) -> Option<usize> {
    valid_digits(value).then(|| perl::numeric(value) as usize)
}

fn sha_value(value: Option<&Py>, label: &str, errors: &mut Vec<String>) {
    let value = scalar(value);
    if value.len() != 64
        || !value
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
    {
        problem(errors, format!("{label} must be a SHA-256 hex digest"));
    }
}

fn count_literal(text: &str, literal: &str) -> usize {
    if literal.is_empty() {
        0
    } else {
        text.matches(literal).count()
    }
}

fn has_line(text: &str, expected: &str) -> bool {
    text.lines().any(|line| line == expected)
}

fn line_with_count(text: &str, prefix: &str) -> bool {
    text.lines().any(|line| {
        line.strip_prefix(prefix).is_some_and(|rest| {
            rest.is_empty() || rest.chars().next().is_some_and(char::is_whitespace)
        })
    })
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

fn metrics(raw: &[u8]) -> (usize, usize, usize, String) {
    (
        raw.iter().filter(|byte| **byte == b'\n').count(),
        raw.len(),
        raw.split(|byte| *byte == b'\n')
            .map(|line| line.len())
            .max()
            .unwrap_or(0),
        sha256_hex(raw),
    )
}

fn compare_identity(
    raw: &[u8],
    declared: &Py,
    label: &str,
    errors: &mut Vec<String>,
    keys: &[&str],
) {
    let (lines, bytes, line_bytes, sha256) = metrics(raw);
    for key in keys {
        let actual = match *key {
            "lines" => lines.to_string(),
            "bytes" => bytes.to_string(),
            "line_bytes" => line_bytes.to_string(),
            "sha256" => sha256.clone(),
            _ => continue,
        };
        if pyjson::get(declared, key).is_none() || scalar(pyjson::get(declared, key)) != actual {
            problem(errors, format!("{label} {key} differs"));
        }
    }
}

fn decoded(raw: &[u8], label: &str, errors: &mut Vec<String>) -> Option<String> {
    match String::from_utf8(raw.to_vec()) {
        Ok(text) => Some(text),
        Err(_) => {
            problem(errors, format!("{label} is not valid UTF-8"));
            None
        }
    }
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

fn shim_path() -> String {
    std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "scripts/check_validation_snapshot_currentness.pl".to_string())
}

fn load_json(path: &Path) -> Result<Py, String> {
    let raw = read_raw(path)?;
    if raw.len() > 32_768 {
        return Err(format!("{LABEL}: contract exceeds 32,768 bytes"));
    }
    let text = String::from_utf8_lossy(&raw);
    pyjson::loads_strict(&text).map_err(|error| {
        let offset = text.chars().count();
        if error.starts_with("Expecting property name enclosed in double quotes:")
            && error.ends_with(&format!("(char {offset})"))
        {
            format!(
                "{LABEL}: invalid JSON at {}: , or }} expected while parsing object/hash, at character offset {offset} (before \"(end of string)\") at {} line 695.\n",
                path.display(),
                shim_path()
            )
        } else {
            format!("{LABEL}: invalid JSON at {}: {error}", path.display())
        }
    })
}

fn validate_contract(base: &Path, contract: &Py, skip_git_history: bool) -> Vec<String> {
    let mut errors = Vec::new();
    if !matches!(contract, Py::Dict(_)) {
        return vec!["contract root must be an object".to_string()];
    }
    unknown_fields(
        contract,
        "contract",
        &[
            "schema_version",
            "owner",
            "snapshot",
            "review_boundary",
            "producer",
            "artifacts",
            "rescan_projection",
            "live_projection",
        ],
        &mut errors,
    );
    if perl::numeric(pyjson::get(contract, "schema_version")) != 1.0 {
        problem(&mut errors, "schema_version must be 1");
    }
    scalar_string(pyjson::get(contract, "owner"), "owner", &mut errors);

    let snapshot = object_value(
        pyjson::get(contract, "snapshot").unwrap_or(&Py::None),
        "snapshot",
        &mut errors,
    );
    unknown_fields(
        snapshot,
        "snapshot",
        &[
            "path",
            "sha256",
            "lines",
            "bytes",
            "line_bytes",
            "required_h2_order",
            "required_literals",
        ],
        &mut errors,
    );
    let snapshot_path =
        safe_path_value(pyjson::get(snapshot, "path"), "snapshot.path", &mut errors);
    sha_value(
        pyjson::get(snapshot, "sha256"),
        "snapshot.sha256",
        &mut errors,
    );
    positive_integer(
        pyjson::get(snapshot, "lines"),
        "snapshot.lines",
        &mut errors,
    );
    positive_integer(
        pyjson::get(snapshot, "bytes"),
        "snapshot.bytes",
        &mut errors,
    );
    positive_integer(
        pyjson::get(snapshot, "line_bytes"),
        "snapshot.line_bytes",
        &mut errors,
    );
    let required_h2 = string_array(
        pyjson::get(snapshot, "required_h2_order"),
        "snapshot.required_h2_order",
        &mut errors,
        8,
    );
    let required_literals = string_array(
        pyjson::get(snapshot, "required_literals"),
        "snapshot.required_literals",
        &mut errors,
        16,
    );

    let review = object_value(
        pyjson::get(contract, "review_boundary").unwrap_or(&Py::None),
        "review_boundary",
        &mut errors,
    );
    unknown_fields(
        review,
        "review_boundary",
        &[
            "source_commit",
            "source_snapshot_sha256",
            "status",
            "evidence_path",
            "evidence_literal",
            "local_artifacts_authoritative",
        ],
        &mut errors,
    );
    let source_commit = scalar(pyjson::get(review, "source_commit"));
    if source_commit.len() != 40
        || !source_commit
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
    {
        problem(
            &mut errors,
            "review_boundary.source_commit must be a full Git object id",
        );
    }
    sha_value(
        pyjson::get(review, "source_snapshot_sha256"),
        "review_boundary.source_snapshot_sha256",
        &mut errors,
    );
    if scalar(pyjson::get(review, "status")) != "reviewed" {
        problem(&mut errors, "review_boundary.status must be reviewed");
    }
    let evidence_path = safe_path_value(
        pyjson::get(review, "evidence_path"),
        "review_boundary.evidence_path",
        &mut errors,
    );
    scalar_string(
        pyjson::get(review, "evidence_literal"),
        "review_boundary.evidence_literal",
        &mut errors,
    );
    if !matches!(
        pyjson::get(review, "local_artifacts_authoritative"),
        Some(Py::Bool(false))
    ) {
        problem(
            &mut errors,
            "ambient local artifacts must not be validation authority",
        );
    }

    let producer = object_value(
        pyjson::get(contract, "producer").unwrap_or(&Py::None),
        "producer",
        &mut errors,
    );
    unknown_fields(producer, "producer", &["path", "regions"], &mut errors);
    let producer_path =
        safe_path_value(pyjson::get(producer, "path"), "producer.path", &mut errors);
    let regions = object_array(
        pyjson::get(producer, "regions"),
        "producer.regions",
        &mut errors,
        8,
    );
    let mut region_ids = BTreeSet::new();
    for (index, region) in regions.iter().enumerate() {
        let label = format!("producer.regions[{index}]");
        unknown_fields(
            region,
            &label,
            &[
                "id",
                "start_literal",
                "end_before_literal",
                "lines",
                "bytes",
                "sha256",
            ],
            &mut errors,
        );
        let id = scalar_string(
            pyjson::get(region, "id"),
            &format!("{label}.id"),
            &mut errors,
        );
        if let Some(id) = &id
            && !region_ids.insert(id.clone())
        {
            problem(
                &mut errors,
                format!("producer region id '{id}' is duplicated"),
            );
        }
        scalar_string(
            pyjson::get(region, "start_literal"),
            &format!("{label}.start_literal"),
            &mut errors,
        );
        scalar_string(
            pyjson::get(region, "end_before_literal"),
            &format!("{label}.end_before_literal"),
            &mut errors,
        );
        positive_integer(
            pyjson::get(region, "lines"),
            &format!("{label}.lines"),
            &mut errors,
        );
        positive_integer(
            pyjson::get(region, "bytes"),
            &format!("{label}.bytes"),
            &mut errors,
        );
        sha_value(
            pyjson::get(region, "sha256"),
            &format!("{label}.sha256"),
            &mut errors,
        );
    }

    let artifacts = object_array(
        pyjson::get(contract, "artifacts"),
        "artifacts",
        &mut errors,
        16,
    );
    if artifacts.len() != 4 {
        problem(
            &mut errors,
            "exactly four reviewed artifacts must be declared",
        );
    }
    let mut keys = BTreeSet::new();
    let mut paths = BTreeSet::new();
    let mut fingerprints = BTreeSet::new();
    for (index, artifact) in artifacts.iter().enumerate() {
        let label = format!("artifacts[{index}]");
        unknown_fields(
            artifact,
            &label,
            &[
                "document_key",
                "display_name",
                "stage",
                "artifact_path",
                "artifact_fingerprint",
                "score",
                "grade",
                "finding_count",
            ],
            &mut errors,
        );
        let key = scalar_string(
            pyjson::get(artifact, "document_key"),
            &format!("{label}.document_key"),
            &mut errors,
        );
        if let Some(key) = &key {
            if !key.split('_').all(|part| {
                !part.is_empty()
                    && part.chars().all(|character| {
                        character.is_ascii_lowercase() || character.is_ascii_digit()
                    })
            }) {
                problem(&mut errors, format!("{label}.document_key is invalid"));
            }
            if !keys.insert(key.clone()) {
                problem(
                    &mut errors,
                    format!("artifact document_key '{key}' is duplicated"),
                );
            }
        }
        scalar_string(
            pyjson::get(artifact, "display_name"),
            &format!("{label}.display_name"),
            &mut errors,
        );
        if scalar(pyjson::get(artifact, "stage")) != "intent_ir" {
            problem(&mut errors, format!("{label}.stage must be intent_ir"));
        }
        let path = safe_path_value(
            pyjson::get(artifact, "artifact_path"),
            &format!("{label}.artifact_path"),
            &mut errors,
        );
        if let Some(path) = &path
            && !paths.insert(path.clone())
        {
            problem(&mut errors, format!("artifact path '{path}' is duplicated"));
        }
        let fingerprint = scalar(pyjson::get(artifact, "artifact_fingerprint"));
        if fingerprint.len() != 16
            || !fingerprint
                .chars()
                .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
        {
            problem(
                &mut errors,
                format!("{label}.artifact_fingerprint must be 16 lowercase hex digits"),
            );
        }
        if !fingerprint.is_empty() && !fingerprints.insert(fingerprint.clone()) {
            problem(
                &mut errors,
                format!("artifact fingerprint '{fingerprint}' is duplicated"),
            );
        }
        bounded_integer(
            pyjson::get(artifact, "score"),
            &format!("{label}.score"),
            0,
            100,
            &mut errors,
        );
        let grade = scalar(pyjson::get(artifact, "grade"));
        if !matches!(
            grade.as_str(),
            "EXCELLENT" | "GOOD" | "ADEQUATE" | "NEEDS IMPROVEMENT"
        ) {
            problem(&mut errors, format!("{label}.grade is invalid"));
        }
        positive_integer(
            pyjson::get(artifact, "finding_count"),
            &format!("{label}.finding_count"),
            &mut errors,
        );
    }

    let rescan = object_value(
        pyjson::get(contract, "rescan_projection").unwrap_or(&Py::None),
        "rescan_projection",
        &mut errors,
    );
    unknown_fields(
        rescan,
        "rescan_projection",
        &[
            "recommendations",
            "execution_summaries",
            "live_queue_rows",
            "live_queue_remainder",
        ],
        &mut errors,
    );
    positive_integer(
        pyjson::get(rescan, "recommendations"),
        "rescan_projection.recommendations",
        &mut errors,
    );
    nonnegative_integer(
        pyjson::get(rescan, "execution_summaries"),
        "rescan_projection.execution_summaries",
        &mut errors,
    );
    nonnegative_integer(
        pyjson::get(rescan, "live_queue_rows"),
        "rescan_projection.live_queue_rows",
        &mut errors,
    );
    nonnegative_integer(
        pyjson::get(rescan, "live_queue_remainder"),
        "rescan_projection.live_queue_remainder",
        &mut errors,
    );
    if let (Some(recommendations), Some(rows), Some(remainder)) = (
        optional_integer(pyjson::get(rescan, "recommendations")),
        optional_integer(pyjson::get(rescan, "live_queue_rows")),
        optional_integer(pyjson::get(rescan, "live_queue_remainder")),
    ) && rows + remainder != recommendations
    {
        problem(
            &mut errors,
            "live queue rows plus remainder must equal recommendations",
        );
    }

    let live = object_value(
        pyjson::get(contract, "live_projection").unwrap_or(&Py::None),
        "live_projection",
        &mut errors,
    );
    unknown_fields(
        live,
        "live_projection",
        &[
            "path",
            "start_marker",
            "end_marker",
            "sha256",
            "lines",
            "bytes",
            "required_literal",
        ],
        &mut errors,
    );
    let live_path = safe_path_value(
        pyjson::get(live, "path"),
        "live_projection.path",
        &mut errors,
    );
    scalar_string(
        pyjson::get(live, "start_marker"),
        "live_projection.start_marker",
        &mut errors,
    );
    scalar_string(
        pyjson::get(live, "end_marker"),
        "live_projection.end_marker",
        &mut errors,
    );
    sha_value(
        pyjson::get(live, "sha256"),
        "live_projection.sha256",
        &mut errors,
    );
    positive_integer(
        pyjson::get(live, "lines"),
        "live_projection.lines",
        &mut errors,
    );
    positive_integer(
        pyjson::get(live, "bytes"),
        "live_projection.bytes",
        &mut errors,
    );
    scalar_string(
        pyjson::get(live, "required_literal"),
        "live_projection.required_literal",
        &mut errors,
    );

    if let Some(path) = &snapshot_path {
        validate_snapshot(
            base,
            path,
            snapshot,
            SnapshotRequirements {
                artifacts: &artifacts,
                rescan,
                required_h2: &required_h2,
                required_literals: &required_literals,
            },
            &mut errors,
        );
    }
    if let Some(path) = &live_path {
        validate_live_projection(base, path, live, &artifacts, rescan, &mut errors);
    }
    if let Some(path) = &producer_path {
        validate_producer(base, path, &regions, &mut errors);
    }
    if let Some(path) = &evidence_path {
        validate_review_evidence(base, path, review, &mut errors);
    }
    if !skip_git_history && let Some(path) = &snapshot_path {
        validate_review_history(base, path, review, &mut errors);
    }
    errors
}

fn validation_snapshot_part_path(document_key: &str) -> String {
    format!("docs/validation-snapshot/{document_key}.md")
}

struct SnapshotRequirements<'a> {
    artifacts: &'a [&'a Py],
    rescan: &'a Py,
    required_h2: &'a [String],
    required_literals: &'a [String],
}

fn validate_snapshot(
    base: &Path,
    path: &str,
    declared: &Py,
    requirements: SnapshotRequirements<'_>,
    errors: &mut Vec<String>,
) {
    let SnapshotRequirements {
        artifacts,
        rescan,
        required_h2,
        required_literals,
    } = requirements;
    let absolute = base.join(path);
    if !absolute.is_file() {
        problem(errors, format!("snapshot '{path}' is missing"));
        return;
    }
    let raw = match read_raw(&absolute) {
        Ok(raw) => raw,
        Err(error) => {
            problem(errors, error);
            return;
        }
    };
    let Some(text) = decoded(&raw, path, errors) else {
        return;
    };
    compare_identity(
        &raw,
        declared,
        "snapshot",
        errors,
        &["lines", "bytes", "line_bytes", "sha256"],
    );
    let h2: Vec<&str> = text
        .lines()
        .filter_map(|line| line.strip_prefix("## "))
        .collect();
    if h2.join("\n") != required_h2.join("\n") {
        problem(errors, "snapshot H2 order differs");
    }
    for literal in required_literals {
        if !text.contains(literal) {
            problem(
                errors,
                format!("snapshot lacks required literal '{literal}'"),
            );
        }
    }
    if !has_line(
        &text,
        &format!("- Artifacts projected: {}", artifacts.len()),
    ) {
        problem(errors, "snapshot artifact summary count differs");
    }
    let recommendations = scalar(pyjson::get(rescan, "recommendations"));
    if !has_line(
        &text,
        &format!("- Targeted rescan recommendations: {recommendations}"),
    ) {
        problem(errors, "snapshot recommendation summary count differs");
    }
    let summaries = scalar(pyjson::get(rescan, "execution_summaries"));
    if !line_with_count(&text, &format!("- Rescan execution summaries: {summaries}")) {
        problem(errors, "snapshot execution-summary count differs");
    }

    let mut recommendation_text = String::new();
    let mut projected_text = String::new();
    for artifact in artifacts {
        let part = validation_snapshot_part_path(&scalar(pyjson::get(artifact, "document_key")));
        if !text.contains(&part) {
            problem(errors, format!("snapshot does not route to part '{part}'"));
            continue;
        }
        let part_absolute = base.join(&part);
        if !part_absolute.is_file() {
            problem(errors, format!("snapshot part '{part}' is missing"));
            continue;
        }
        let raw = match read_raw(&part_absolute) {
            Ok(raw) => raw,
            Err(error) => {
                problem(errors, error);
                continue;
            }
        };
        let Some(part_text) = decoded(&raw, &part, errors) else {
            continue;
        };
        let Some((before_projected, projected)) = part_text.split_once("## Projected Artifacts\n")
        else {
            problem(
                errors,
                format!("snapshot part '{part}' sections are malformed"),
            );
            continue;
        };
        let Some((_, recommendations)) =
            before_projected.split_once("## Targeted Rescan Recommendations\n")
        else {
            problem(
                errors,
                format!("snapshot part '{part}' sections are malformed"),
            );
            continue;
        };
        recommendation_text.push_str(recommendations);
        projected_text.push_str(projected);
    }
    if text.lines().any(|line| line.starts_with("### ")) {
        problem(errors, "snapshot landing still carries record detail");
    }
    let recommendation_chunks = h3_chunks(&recommendation_text);
    if recommendation_chunks.len()
        != optional_integer(pyjson::get(rescan, "recommendations")).unwrap_or(usize::MAX)
    {
        problem(errors, "snapshot recommendation record count differs");
    }
    let artifact_chunks = h3_chunks(&projected_text);
    if artifact_chunks.len() != artifacts.len() {
        problem(errors, "snapshot projected artifact record count differs");
    }
    for (index, artifact) in artifacts.iter().enumerate() {
        let Some(chunk) = artifact_chunks.get(index) else {
            break;
        };
        let display_name = scalar(pyjson::get(artifact, "display_name"));
        let stage = scalar(pyjson::get(artifact, "stage"));
        if !has_line(chunk, &format!("### {display_name} ({stage})")) {
            problem(
                errors,
                format!("snapshot projected artifact {index} heading differs"),
            );
        }
        for field in ["document_key", "artifact_path", "artifact_fingerprint"] {
            let value = scalar(pyjson::get(artifact, field));
            if !has_line(chunk, &format!("- {field}: `{value}`")) {
                problem(
                    errors,
                    format!("snapshot projected artifact {index} {field} differs"),
                );
            }
        }
        let score = scalar(pyjson::get(artifact, "score"));
        let grade = scalar(pyjson::get(artifact, "grade"));
        if !has_line(chunk, &format!("- score: `{score}/100 {grade}`")) {
            problem(
                errors,
                format!("snapshot projected artifact {index} score differs"),
            );
        }
        let finding_count = scalar(pyjson::get(artifact, "finding_count"));
        let suffix = format!(" with {finding_count} finding(s)");
        if !chunk
            .lines()
            .any(|line| line.starts_with("- summary: ") && line.ends_with(&suffix))
        {
            problem(
                errors,
                format!("snapshot projected artifact {index} summary finding count differs"),
            );
        }
        let finding_lines = chunk
            .lines()
            .filter(|line| line.starts_with("  - ["))
            .count();
        if finding_lines
            != optional_integer(pyjson::get(artifact, "finding_count")).unwrap_or(usize::MAX)
        {
            problem(
                errors,
                format!("snapshot projected artifact {index} finding-line count differs"),
            );
        }
    }
}

fn validate_live_projection(
    base: &Path,
    path: &str,
    declared: &Py,
    artifacts: &[&Py],
    rescan: &Py,
    errors: &mut Vec<String>,
) {
    let absolute = base.join(path);
    if !absolute.is_file() {
        problem(errors, format!("live projection path '{path}' is missing"));
        return;
    }
    let raw = match read_raw(&absolute) {
        Ok(raw) => raw,
        Err(error) => {
            problem(errors, error);
            return;
        }
    };
    let Some(text) = decoded(&raw, path, errors) else {
        return;
    };
    let start = scalar(pyjson::get(declared, "start_marker"));
    let end = scalar(pyjson::get(declared, "end_marker"));
    if count_literal(&text, &start) != 1 {
        problem(
            errors,
            "live projection start marker must occur exactly once",
        );
    }
    if count_literal(&text, &end) != 1 {
        problem(errors, "live projection end marker must occur exactly once");
    }
    let framed_start = format!("{start}\n");
    let framed_end = format!("\n{end}");
    let from = text
        .find(&framed_start)
        .map(|index| index as isize)
        .unwrap_or(-1);
    let to = if from >= 0 {
        text[from as usize + framed_start.len()..]
            .find(&framed_end)
            .map(|index| index + from as usize + framed_start.len())
            .map(|index| index as isize)
            .unwrap_or(-1)
    } else {
        -1
    };
    if from < 0 || to < 0 {
        problem(errors, "live projection marker framing is malformed");
        return;
    }
    let block = &text[from as usize + framed_start.len()..to as usize];
    compare_identity(
        block.as_bytes(),
        declared,
        "live projection",
        errors,
        &["lines", "bytes", "sha256"],
    );
    let required = scalar(pyjson::get(declared, "required_literal"));
    if !block.contains(&required) {
        problem(errors, "live projection required label differs");
    }
    for (index, artifact) in artifacts.iter().enumerate() {
        let display_name = scalar(pyjson::get(artifact, "display_name"));
        let stage = scalar(pyjson::get(artifact, "stage"));
        let score = scalar(pyjson::get(artifact, "score"));
        let grade = scalar(pyjson::get(artifact, "grade"));
        let artifact_path = scalar(pyjson::get(artifact, "artifact_path"));
        let row = format!(
            "  - `{display_name}` (`{stage}`): `{score}/100 {grade}` from `{artifact_path}`"
        );
        if count_literal(block, &row) != 1 {
            problem(
                errors,
                format!("live projection artifact row {index} differs"),
            );
        }
    }
    let summaries = scalar(pyjson::get(rescan, "execution_summaries"));
    if !line_with_count(block, &format!("- Rescan execution summaries: {summaries}")) {
        problem(errors, "live projection execution-summary count differs");
    }
    let Some((_, queue)) = block.split_once("- Targeted rescan queue:\n") else {
        problem(errors, "live projection queue is missing");
        return;
    };
    let rows = queue
        .lines()
        .filter(|line| line.starts_with("  - `") && line.len() > 5)
        .count();
    if rows != optional_integer(pyjson::get(rescan, "live_queue_rows")).unwrap_or(usize::MAX) {
        problem(errors, "live projection queue row count differs");
    }
    let remainder = scalar(pyjson::get(rescan, "live_queue_remainder"));
    let remainder = format!("  - ... and {remainder} more targeted rescan recommendation(s)");
    if count_literal(queue, &remainder) != 1 {
        problem(errors, "live projection queue remainder differs");
    }
}

fn validate_producer(base: &Path, path: &str, regions: &[&Py], errors: &mut Vec<String>) {
    let absolute = base.join(path);
    if !absolute.is_file() {
        problem(errors, format!("producer '{path}' is missing"));
        return;
    }
    let raw = match read_raw(&absolute) {
        Ok(raw) => raw,
        Err(error) => {
            problem(errors, error);
            return;
        }
    };
    let Some(text) = decoded(&raw, path, errors) else {
        return;
    };
    for region in regions {
        let id = scalar(pyjson::get(region, "id"));
        let start = scalar(pyjson::get(region, "start_literal"));
        let end = scalar(pyjson::get(region, "end_before_literal"));
        if count_literal(&text, &start) != 1 || count_literal(&text, &end) != 1 {
            problem(errors, format!("producer region '{id}' boundary differs"));
            continue;
        }
        let Some(from) = text.find(&start) else {
            problem(errors, format!("producer region '{id}' is malformed"));
            continue;
        };
        let to = text[from + start.len()..]
            .find(&end)
            .map(|index| index + from + start.len());
        if to.is_none_or(|to| to <= from) {
            problem(errors, format!("producer region '{id}' is malformed"));
            continue;
        }
        let raw = &text.as_bytes()[from..to.unwrap()];
        compare_identity(
            raw,
            region,
            &format!("producer region '{id}'"),
            errors,
            &["lines", "bytes", "sha256"],
        );
    }
}

fn validate_review_evidence(base: &Path, path: &str, review: &Py, errors: &mut Vec<String>) {
    let absolute = base.join(path);
    if !absolute.is_file() {
        problem(errors, format!("review evidence '{path}' is missing"));
        return;
    }
    let raw = match read_raw(&absolute) {
        Ok(raw) => raw,
        Err(error) => {
            problem(errors, error);
            return;
        }
    };
    let Some(text) = decoded(&raw, path, errors) else {
        return;
    };
    let literal = scalar(pyjson::get(review, "evidence_literal"));
    if count_literal(&text, &literal) != 1 {
        problem(errors, "review-gate evidence literal differs");
    }
}

fn validate_review_history(
    base: &Path,
    snapshot_path: &str,
    review: &Py,
    errors: &mut Vec<String>,
) {
    let commit = scalar(pyjson::get(review, "source_commit"));
    if commit.len() != 40
        || !commit
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
        || !safe_relative_path(snapshot_path)
    {
        return;
    }
    let spec = format!("{commit}:{snapshot_path}");
    let process = match Command::new("git")
        .arg("-C")
        .arg(base)
        .arg("show")
        .arg(spec)
        .output()
    {
        Ok(process) => process,
        Err(_) => {
            problem(errors, "cannot open reviewed snapshot from Git");
            return;
        }
    };
    if !process.status.success() {
        problem(errors, "reviewed snapshot Git object is missing");
        return;
    }
    if sha256_hex(&process.stdout) != scalar(pyjson::get(review, "source_snapshot_sha256")) {
        problem(errors, "reviewed source snapshot identity differs");
    }
}

fn write_contract(fixture: &Path, relative: &str, contract: &Py) -> Result<(), String> {
    let path = fixture.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| error.to_string())?;
    }
    let content = encode(contract, Some(2))?;
    fs::write(&path, content).map_err(|error| {
        format!(
            "{LABEL} self-test: cannot write {}: {error}",
            path.display()
        )
    })
}

fn replace_once(path: &Path, from: &str, to: &str) -> Result<(), String> {
    let raw = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let count = count_literal(&raw, from);
    if count != 1 {
        return Err(format!(
            "{LABEL} self-test: replacement '{from}' count is {count}"
        ));
    }
    fs::write(path, raw.replacen(from, to, 1)).map_err(|error| error.to_string())
}

fn append_raw(path: &Path, suffix: &str) -> Result<(), String> {
    let mut raw = fs::read(path).map_err(|error| error.to_string())?;
    raw.extend_from_slice(suffix.as_bytes());
    fs::write(path, raw).map_err(|error| error.to_string())
}

fn seed_fixture(
    base: &Path,
    fixture: &Path,
    relative_contract: &str,
    contract: &Py,
) -> Result<(), String> {
    let snapshot = pyjson::get(contract, "snapshot").unwrap_or(&Py::None);
    let live = pyjson::get(contract, "live_projection").unwrap_or(&Py::None);
    let producer = pyjson::get(contract, "producer").unwrap_or(&Py::None);
    let review = pyjson::get(contract, "review_boundary").unwrap_or(&Py::None);
    let mut paths = vec![
        relative_contract.to_string(),
        scalar(pyjson::get(snapshot, "path")),
        scalar(pyjson::get(live, "path")),
        scalar(pyjson::get(producer, "path")),
        scalar(pyjson::get(review, "evidence_path")),
    ];
    if let Some(Py::List(artifacts)) = pyjson::get(contract, "artifacts") {
        for artifact in artifacts {
            paths.push(validation_snapshot_part_path(&scalar(pyjson::get(
                artifact,
                "document_key",
            ))));
        }
    }
    for path in paths {
        let source = base.join(&path);
        let destination = fixture.join(&path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::copy(&source, &destination)
            .map_err(|error| format!("cannot seed fixture '{}': {error}", destination.display()))?;
    }
    Ok(())
}

struct FixtureRoot(PathBuf);

impl Drop for FixtureRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

#[derive(Clone, Copy)]
enum Mutation {
    Valid,
    SnapshotIdentity,
    ReportFingerprint,
    RecommendationCount,
    LiveProjection,
    ProducerRegion,
    MissingPart,
    LandingDetail,
    ReviewEvidence,
    UnsafeArtifactPath,
    DuplicateArtifact,
    UnknownSchema,
}

const CASES: &[(&str, bool, Mutation)] = &[
    ("valid reviewed boundary", true, Mutation::Valid),
    ("snapshot identity drift", false, Mutation::SnapshotIdentity),
    (
        "report fingerprint drift",
        false,
        Mutation::ReportFingerprint,
    ),
    (
        "recommendation count drift",
        false,
        Mutation::RecommendationCount,
    ),
    ("live projection drift", false, Mutation::LiveProjection),
    ("producer region drift", false, Mutation::ProducerRegion),
    ("missing routed part", false, Mutation::MissingPart),
    (
        "landing carries record detail",
        false,
        Mutation::LandingDetail,
    ),
    ("review evidence drift", false, Mutation::ReviewEvidence),
    ("unsafe artifact path", false, Mutation::UnsafeArtifactPath),
    (
        "duplicate artifact identity",
        false,
        Mutation::DuplicateArtifact,
    ),
    ("unknown schema", false, Mutation::UnknownSchema),
];

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

fn mutate_case(
    fixture: &Path,
    relative_contract: &str,
    contract: &mut Py,
    mutation: Mutation,
) -> Result<(), String> {
    let snapshot = pyjson::get(contract, "snapshot").unwrap_or(&Py::None);
    let snapshot_path = scalar(pyjson::get(snapshot, "path"));
    let artifacts = pyjson::get(contract, "artifacts").unwrap_or(&Py::None);
    let first = match artifacts {
        Py::List(items) => &items[0],
        _ => return Err("fixture artifacts are missing".to_string()),
    };
    let first_part = validation_snapshot_part_path(&scalar(pyjson::get(first, "document_key")));
    match mutation {
        Mutation::Valid => {}
        Mutation::SnapshotIdentity => append_raw(&fixture.join(&snapshot_path), "\nDRIFT\n")?,
        Mutation::ReportFingerprint => replace_once(
            &fixture.join(&first_part),
            &scalar(pyjson::get(first, "artifact_fingerprint")),
            "0000000000000000",
        )?,
        Mutation::RecommendationCount => replace_once(
            &fixture.join(&snapshot_path),
            "- Targeted rescan recommendations: 29",
            "- Targeted rescan recommendations: 28",
        )?,
        Mutation::LiveProjection => {
            let live = pyjson::get(contract, "live_projection").unwrap_or(&Py::None);
            replace_once(
                &fixture.join(scalar(pyjson::get(live, "path"))),
                "Last reviewed projected validation snapshot",
                "Drifted validation snapshot",
            )?;
        }
        Mutation::ProducerRegion => {
            let producer = pyjson::get(contract, "producer").unwrap_or(&Py::None);
            replace_once(
                &fixture.join(scalar(pyjson::get(producer, "path"))),
                "let mut lines = vec![\"- Last reviewed projected validation snapshot:\".to_string()];",
                "let mut lines = vec![\"- Drifted projected validation snapshot:\".to_string()];",
            )?;
        }
        Mutation::MissingPart => {
            fs::remove_file(fixture.join(&first_part)).map_err(|error| error.to_string())?
        }
        Mutation::LandingDetail => {
            append_raw(&fixture.join(&snapshot_path), "\n### smuggled record\n")?
        }
        Mutation::ReviewEvidence => {
            let review = pyjson::get(contract, "review_boundary").unwrap_or(&Py::None);
            replace_once(
                &fixture.join(scalar(pyjson::get(review, "evidence_path"))),
                &scalar(pyjson::get(review, "evidence_literal")),
                "review evidence removed",
            )?;
        }
        Mutation::UnsafeArtifactPath => {
            if let Some(Py::List(items)) = field_mut(contract, "artifacts") {
                set_field(&mut items[0], "artifact_path", text("../outside.json"));
            }
        }
        Mutation::DuplicateArtifact => {
            if let Some(Py::List(items)) = field_mut(contract, "artifacts") {
                items[1] = items[0].clone();
            }
        }
        Mutation::UnknownSchema => set_field(contract, "schema_version", integer(2)),
    }
    if matches!(
        mutation,
        Mutation::UnsafeArtifactPath | Mutation::DuplicateArtifact | Mutation::UnknownSchema
    ) {
        write_contract(fixture, relative_contract, contract)?;
    }
    Ok(())
}

fn run_self_test(base: &Path, relative_contract: &str) -> Result<(usize, usize), String> {
    let fixture_root = base.join(format!(
        "generated/.validation-snapshot-currentness-test.{}",
        std::process::id()
    ));
    if fixture_root.exists() {
        fs::remove_dir_all(&fixture_root)
            .map_err(|error| format!("{LABEL} self-test: pre-clean failed: {error}"))?;
    }
    let _guard = FixtureRoot(fixture_root.clone());
    fs::create_dir_all(&fixture_root).map_err(|error| error.to_string())?;
    let source_contract = load_json(&base.join(relative_contract))?;
    let mut passed = 0;
    for (index, (name, expected_ok, mutation)) in CASES.iter().enumerate() {
        let fixture = fixture_root.join(format!("case-{:02}", index + 1));
        seed_fixture(base, &fixture, relative_contract, &source_contract)?;
        let mut contract = load_json(&fixture.join(relative_contract))?;
        mutate_case(&fixture, relative_contract, &mut contract, *mutation)?;
        let contract = load_json(&fixture.join(relative_contract))?;
        let errors = validate_contract(&fixture, &contract, true);
        let actual_ok = errors.is_empty();
        if actual_ok != *expected_ok {
            return Err(format!(
                "{LABEL} self-test '{name}' expected {}, got {}: {}",
                *expected_ok as usize,
                actual_ok as usize,
                errors.join("; ")
            ));
        }
        passed += 1;
    }
    let expected = 12;
    if passed != expected {
        return Err(format!(
            "{LABEL} self-test: expected {expected} cases, passed {passed}"
        ));
    }
    Ok((passed, expected))
}

fn report(contract: &Py) -> Result<String, String> {
    let snapshot = pyjson::get(contract, "snapshot").unwrap_or(&Py::None);
    let review = pyjson::get(contract, "review_boundary").unwrap_or(&Py::None);
    let producer = pyjson::get(contract, "producer").unwrap_or(&Py::None);
    let rescan = pyjson::get(contract, "rescan_projection").unwrap_or(&Py::None);
    let artifact_count = match pyjson::get(contract, "artifacts") {
        Some(Py::List(items)) => items.len(),
        _ => 0,
    };
    let region_count = match pyjson::get(producer, "regions") {
        Some(Py::List(items)) => items.len(),
        _ => 0,
    };
    let report = object(vec![
        (
            "schema_version",
            pyjson::get(contract, "schema_version")
                .unwrap_or(&Py::None)
                .clone(),
        ),
        (
            "owner",
            pyjson::get(contract, "owner").unwrap_or(&Py::None).clone(),
        ),
        (
            "reviewed_commit",
            text(&scalar(pyjson::get(review, "source_commit"))),
        ),
        (
            "snapshot",
            object(
                ["path", "sha256", "lines", "bytes", "line_bytes"]
                    .into_iter()
                    .map(|field| {
                        (
                            field,
                            pyjson::get(snapshot, field).unwrap_or(&Py::None).clone(),
                        )
                    })
                    .collect(),
            ),
        ),
        ("artifacts", integer(artifact_count)),
        (
            "recommendations",
            pyjson::get(rescan, "recommendations")
                .unwrap_or(&Py::None)
                .clone(),
        ),
        (
            "execution_summaries",
            pyjson::get(rescan, "execution_summaries")
                .unwrap_or(&Py::None)
                .clone(),
        ),
        ("producer_regions", integer(region_count)),
        ("ambient_local_artifacts_authoritative", Py::Bool(false)),
    ]);
    encode(&report, None)
}

/// Run validation-snapshot currentness with its original mode and path arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut root = root.to_path_buf();
    let mut contract_path = "doctrine/live_document_size/validation_snapshot.json".to_string();
    let mut mode = Mode::Check;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" => mode = Mode::Check,
            "--report" => mode = Mode::Report,
            "--self-test" => mode = Mode::SelfTest,
            "--root" => match arguments.next() {
                Some(directory) => {
                    let canonical = fs::canonicalize(directory).ok().or_else(|| {
                        let path = Path::new(directory);
                        let name = path.file_name()?;
                        let parent = path
                            .parent()
                            .filter(|parent| !parent.as_os_str().is_empty())
                            .unwrap_or_else(|| Path::new("."));
                        Some(fs::canonicalize(parent).ok()?.join(name))
                    });
                    match canonical {
                        Some(canonical) => root = canonical,
                        None => {
                            output.err(&format!("{LABEL}: root does not exist\n"));
                            return 1;
                        }
                    }
                }
                None => {
                    output.err(&format!("{LABEL}: --root requires a path\n"));
                    return 1;
                }
            },
            "--contract" => match arguments.next() {
                Some(path) => contract_path = path.to_string(),
                None => {
                    output.err(&format!("{LABEL}: --contract requires a path\n"));
                    return 1;
                }
            },
            _ => {
                output.err("Usage: cargo xtask check validation-snapshot-currentness [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n");
                return 2;
            }
        }
    }
    if mode == Mode::SelfTest {
        return match run_self_test(&root, &contract_path) {
            Ok((passed, expected)) => {
                output.out(&format!("{LABEL}: self-test {passed}/{expected} passed.\n"));
                0
            }
            Err(error) => {
                output.err(&format!("{error}\n"));
                1
            }
        };
    }
    let contract = match load_json(&root.join(&contract_path)) {
        Ok(contract) => contract,
        Err(error) => {
            output.err(&format!("{error}\n"));
            return if error.starts_with("validation-snapshot-currentness: cannot read ") {
                2
            } else {
                255
            };
        }
    };
    let errors = validate_contract(&root, &contract, false);
    if !errors.is_empty() {
        for error in errors {
            output.err(&format!("{LABEL}: {error}\n"));
        }
        return 1;
    }
    if mode == Mode::Report {
        match report(&contract) {
            Ok(report) => output.out(&format!("{report}\n")),
            Err(error) => {
                output.err(&format!("{LABEL}: {error}\n"));
                return 1;
            }
        }
    } else {
        output.out(&format!(
            "{LABEL}: reviewed boundary, four report identities, producer regions, and live projection are current.\n"
        ));
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_twelve_self_tests_pass_without_residue() {
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
            "validation-snapshot-currentness: self-test 12/12 passed.\n"
        );
        assert!(
            !root
                .join(format!(
                    "generated/.validation-snapshot-currentness-test.{}",
                    std::process::id()
                ))
                .exists()
        );
    }

    #[test]
    fn the_real_reviewed_boundary_and_report_hold() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--report".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("\"artifacts\":4"));
        assert!(output.text.contains("\"schema_version\":1"));
        assert!(
            output
                .text
                .contains("\"ambient_local_artifacts_authoritative\":false")
        );
    }
}
