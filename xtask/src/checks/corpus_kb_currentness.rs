//! Corpus knowledge-base currentness, native since `RUSTDOC-AND-XTASK.15.q`: reviewed validation,
//! Git-indexed KG fixtures, managed Markdown/JSON, producer regions and the 17-case suite.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "corpus-kb-currentness";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Check,
    Report,
    SelfTest,
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

fn scalar(value: Option<&Py>) -> String {
    perl::scalar(value).unwrap_or_default()
}

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn positive_integer(value: Option<&Py>) -> bool {
    let text = scalar(value);
    !text.is_empty()
        && text.chars().all(|character| character.is_ascii_digit())
        && perl::numeric(value) > 0.0
}

fn sha_value(value: Option<&Py>) -> bool {
    let value = scalar(value);
    value.len() == 64
        && value
            .chars()
            .all(|character| character.is_ascii_digit() || ('a'..='f').contains(&character))
}

fn safe_relative_path(path: &str, max_bytes: usize) -> bool {
    !path.is_empty()
        && path.len() <= max_bytes
        && !path.starts_with('/')
        && !path.contains("//")
        && !path.contains('\0')
        && !path.split('/').any(|part| part == "..")
}

fn regular_within(base: &Path, path: &Path) -> bool {
    if !path.is_file()
        || fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        return false;
    }
    fs::canonicalize(path)
        .ok()
        .and_then(|path| {
            fs::canonicalize(base)
                .ok()
                .map(|base| path.starts_with(base))
        })
        .unwrap_or(false)
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
        .unwrap_or_else(|_| "scripts/check_corpus_kb_currentness.pl".to_string())
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
                "{LABEL}: malformed JSON at {}: , or }} expected while parsing object/hash, at character offset {offset} (before \"(end of string)\") at {} line 695.\n",
                path.display(),
                shim_path()
            )
        } else {
            format!("{LABEL}: malformed JSON at {}: {error}", path.display())
        }
    })
}

fn line_count(raw: &[u8]) -> usize {
    raw.iter().filter(|byte| **byte == b'\n').count()
        + usize::from(!raw.is_empty() && !raw.ends_with(b"\n"))
}

type ManagedParts = (Vec<u8>, Vec<u8>, Vec<u8>);

fn extract_managed(
    raw: &[u8],
    start_marker: &str,
    end_marker: &str,
) -> Result<ManagedParts, String> {
    if start_marker.is_empty() || end_marker.is_empty() {
        return Err("markers must be non-empty".to_string());
    }
    let raw = String::from_utf8_lossy(raw);
    let Some(start) = raw.find(start_marker) else {
        return Err("start marker is missing".to_string());
    };
    if raw[start + 1..].contains(start_marker) {
        return Err("start marker is duplicated".to_string());
    }
    let search_from = start + start_marker.len();
    let Some(end_start) = raw[search_from..]
        .find(end_marker)
        .map(|index| index + search_from)
    else {
        return Err("end marker is missing or precedes the start marker".to_string());
    };
    if raw[end_start + 1..].contains(end_marker) {
        return Err("end marker is duplicated".to_string());
    }
    let end = end_start + end_marker.len();
    Ok((
        raw.as_bytes()[start..end].to_vec(),
        raw.as_bytes()[..start].to_vec(),
        raw.as_bytes()[end..].to_vec(),
    ))
}

fn git_tracked_paths(base: &Path, root: &str) -> Result<Vec<String>, String> {
    let process = Command::new("git")
        .arg("-C")
        .arg(base)
        .arg("ls-files")
        .arg("-z")
        .arg("--")
        .arg(root)
        .output()
        .map_err(|_| "cannot run git ls-files for KG fixture membership".to_string())?;
    if !process.status.success() {
        return Err("git ls-files failed for KG fixture membership".to_string());
    }
    let paths = process
        .stdout
        .split(|byte| *byte == 0)
        .filter(|path| !path.is_empty())
        .map(|path| String::from_utf8_lossy(path).into_owned())
        .collect();
    Ok(paths)
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

fn render_reviewed_validation_block(raw: &[u8], base: &Path) -> Result<Vec<u8>, String> {
    let text = String::from_utf8(raw.to_vec())
        .map_err(|_| "reviewed validation snapshot is not valid UTF-8".to_string())?;
    let mut part_paths = Vec::new();
    let mut remainder = text.as_str();
    while let Some(index) = remainder.find("](docs/validation-snapshot/") {
        let after = &remainder[index + 2..];
        let Some(end) = after.find(')') else {
            break;
        };
        part_paths.push(after[..end].to_string());
        remainder = &after[end + 1..];
    }
    if part_paths.is_empty() {
        return Err("reviewed validation snapshot routes to no per-document part".to_string());
    }
    let mut chunks = Vec::new();
    for relative in part_paths {
        let absolute = base.join(&relative);
        if !regular_within(base, &absolute) {
            return Err(format!(
                "reviewed validation snapshot part `{relative}` is missing or unsafe"
            ));
        }
        let part = read_raw(&absolute)?;
        let part = String::from_utf8(part).map_err(|_| {
            format!("reviewed validation snapshot part `{relative}` is not valid UTF-8")
        })?;
        let Some((_, projected)) = part.split_once("## Projected Artifacts\n") else {
            return Err(format!(
                "reviewed validation snapshot part `{relative}` has no projected artifact section"
            ));
        };
        let Some(projected) = projected.strip_prefix("### ") else {
            return Err(format!(
                "reviewed validation snapshot part `{relative}` has no projected artifact section"
            ));
        };
        chunks.extend(projected.split("\n### ").map(|chunk| chunk.to_string()));
    }
    if chunks.is_empty() {
        return Err("reviewed validation snapshot has no artifact records".to_string());
    }
    let mut output = "<!-- corpus_kb_validation_findings:start -->\n".to_string();
    output.push_str("<!-- This reviewed block is refreshed from `VALIDATION_SNAPSHOT.md` by `specforge corpus-kb --validation-snapshot`. -->\n\n");
    let mut keys = BTreeSet::new();
    for chunk in chunks {
        let heading = chunk.lines().next().unwrap_or("");
        let Some((_, stage)) = heading.rsplit_once(" (") else {
            return Err(format!(
                "reviewed validation snapshot has malformed heading `{heading}`"
            ));
        };
        let Some(stage) = stage.strip_suffix(')') else {
            return Err(format!(
                "reviewed validation snapshot has malformed heading `{heading}`"
            ));
        };
        let field = |name: &str| {
            let prefix = format!("- {name}: `");
            chunk.lines().find_map(|line| {
                line.strip_prefix(&prefix)
                    .and_then(|line| line.strip_suffix('`'))
                    .map(|value| value.to_string())
            })
        };
        let key = field("document_key");
        let artifact = field("artifact_path");
        let fingerprint = field("artifact_fingerprint");
        let score = field("score");
        let summary = chunk
            .lines()
            .find_map(|line| line.strip_prefix("- summary: "));
        let (Some(key), Some(artifact), Some(fingerprint), Some(score), Some(summary)) =
            (key, artifact, fingerprint, score, summary)
        else {
            return Err("reviewed validation snapshot artifact field is missing".to_string());
        };
        if !keys.insert(key.clone()) {
            return Err(format!(
                "reviewed validation snapshot repeats document_key `{key}`"
            ));
        }
        let Some((_, findings)) = chunk.split_once("- findings:\n") else {
            return Err(format!(
                "reviewed validation snapshot artifact `{key}` has no findings"
            ));
        };
        let findings: Vec<&str> = findings
            .lines()
            .take_while(|line| line.starts_with("  - "))
            .map(|line| &line[4..])
            .collect();
        if findings.is_empty() {
            return Err(format!(
                "reviewed validation snapshot artifact `{key}` has no findings"
            ));
        }
        output.push_str(&format!("### {key}\n"));
        output.push_str(&format!("- artifact_path: `{artifact}`\n"));
        output.push_str(&format!("- stage: `{stage}`\n"));
        output.push_str(&format!("- artifact_fingerprint: `{fingerprint}`\n"));
        output.push_str(&format!("- score: `{score}`\n"));
        output.push_str(&format!("- summary: {}\n", summary.replace('|', "\\|")));
        output.push_str("- findings:\n");
        for finding in findings {
            output.push_str(&format!("  - {}\n", finding.replace('|', "\\|")));
        }
        output.push('\n');
    }
    output.push_str("<!-- corpus_kb_validation_findings:end -->");
    Ok(output.into_bytes())
}

fn validate_validation_input(
    base: &Path,
    validation: &Py,
    outputs: &Py,
    controls: &Py,
    errors: &mut Vec<String>,
) {
    if !matches!(validation, Py::Dict(_)) {
        errors.push("inputs.validation must be an object".to_string());
        return;
    }
    for field in [
        "snapshot_path",
        "authority_contract_path",
        "authority_contract_sha256",
    ] {
        let item = pyjson::get(validation, field);
        if item.is_none_or(perl::is_reference) || scalar(item).is_empty() {
            errors.push(format!(
                "inputs.validation.{field} must be a non-empty scalar"
            ));
        }
    }
    if !matches!(
        pyjson::get(validation, "reviewed_snapshot_is_authoritative"),
        Some(Py::Bool(true))
    ) {
        errors.push("reviewed validation snapshot must be authoritative".to_string());
    }
    if !matches!(
        pyjson::get(validation, "ambient_reports_are_authoritative"),
        Some(Py::Bool(false))
    ) {
        errors.push("ambient validation reports must not be authoritative".to_string());
    }
    if !errors.is_empty() {
        return;
    }
    let max_path = perl::numeric(pyjson::get(controls, "max_path_bytes")) as usize;
    for field in ["snapshot_path", "authority_contract_path"] {
        if !safe_relative_path(&scalar(pyjson::get(validation, field)), max_path) {
            errors.push(format!("unsafe inputs.validation.{field}"));
        }
    }
    if !sha_value(pyjson::get(validation, "authority_contract_sha256")) {
        errors.push("invalid validation authority contract hash".to_string());
    }
    if !errors.is_empty() {
        return;
    }
    let authority_path = base.join(scalar(pyjson::get(validation, "authority_contract_path")));
    if !regular_within(base, &authority_path) {
        errors.push("validation authority contract is missing, unsafe, or a symlink".to_string());
        return;
    }
    let authority_raw = match read_raw(&authority_path) {
        Ok(raw) => raw,
        Err(error) => {
            errors.push(error);
            return;
        }
    };
    if sha256_hex(&authority_raw) != scalar(pyjson::get(validation, "authority_contract_sha256")) {
        errors.push("validation authority contract identity differs".to_string());
    }
    let authority = match String::from_utf8(authority_raw)
        .ok()
        .and_then(|text| pyjson::loads_strict(&text).ok())
    {
        Some(authority @ Py::Dict(_)) => authority,
        _ => {
            errors.push("validation authority contract is malformed JSON".to_string());
            return;
        }
    };
    let review = pyjson::get(&authority, "review_boundary").unwrap_or(&Py::None);
    if scalar(pyjson::get(review, "status")) != "reviewed" {
        errors.push("validation authority does not declare reviewed status".to_string());
    }
    if !matches!(
        pyjson::get(review, "local_artifacts_authoritative"),
        Some(Py::Bool(false))
    ) {
        errors.push("validation authority permits ambient local artifacts".to_string());
    }
    let snapshot = pyjson::get(&authority, "snapshot").unwrap_or(&Py::None);
    if scalar(pyjson::get(snapshot, "path")) != scalar(pyjson::get(validation, "snapshot_path")) {
        errors.push("validation snapshot path differs from its authority contract".to_string());
    }
    let snapshot_path = base.join(scalar(pyjson::get(validation, "snapshot_path")));
    if !regular_within(base, &snapshot_path) {
        errors.push("reviewed validation snapshot is missing, unsafe, or a symlink".to_string());
        return;
    }
    let snapshot_raw = match read_raw(&snapshot_path) {
        Ok(raw) => raw,
        Err(error) => {
            errors.push(error);
            return;
        }
    };
    if sha256_hex(&snapshot_raw) != scalar(pyjson::get(snapshot, "sha256")) {
        errors.push(
            "reviewed validation snapshot identity differs from its authority contract".to_string(),
        );
    }
    let markdown = pyjson::get(outputs, "markdown");
    let validation_output = match markdown {
        Some(Py::List(items)) => items
            .iter()
            .find(|entry| scalar(pyjson::get(entry, "kind")) == "reviewed_validation"),
        _ => None,
    };
    let Some(validation_output) = validation_output else {
        errors.push("one reviewed_validation managed output is required".to_string());
        return;
    };
    let page_path = base.join(scalar(pyjson::get(validation_output, "path")));
    if !regular_within(base, &page_path) {
        return;
    }
    let raw = match read_raw(&page_path) {
        Ok(raw) => raw,
        Err(_) => return,
    };
    let actual = extract_managed(
        &raw,
        &scalar(pyjson::get(validation_output, "start_marker")),
        &scalar(pyjson::get(validation_output, "end_marker")),
    );
    let expected = render_reviewed_validation_block(&snapshot_raw, base);
    if let Err(error) = &expected {
        errors.push(error.clone());
    }
    if let (Ok((actual, _, _)), Ok(expected)) = (actual, expected)
        && actual != expected
    {
        errors.push(
            "reviewed validation managed block differs from VALIDATION_SNAPSHOT.md".to_string(),
        );
    }
}

fn validate_kg_input(
    base: &Path,
    kg: &Py,
    controls: &Py,
    tracked_override: Option<&[String]>,
    errors: &mut Vec<String>,
) {
    if !matches!(kg, Py::Dict(_)) {
        errors.push("inputs.kg_fixtures must be an object".to_string());
        return;
    }
    for field in ["file_count", "fixture_count", "bytes"] {
        if !positive_integer(pyjson::get(kg, field)) {
            errors.push(format!(
                "inputs.kg_fixtures.{field} must be a positive integer"
            ));
        }
    }
    if !sha_value(pyjson::get(kg, "aggregate_sha256")) {
        errors.push("inputs.kg_fixtures.aggregate_sha256 is invalid".to_string());
    }
    if !matches!(
        pyjson::get(kg, "git_index_is_authoritative"),
        Some(Py::Bool(true))
    ) {
        errors.push("KG fixture Git index must be authoritative".to_string());
    }
    let max_path = perl::numeric(pyjson::get(controls, "max_path_bytes")) as usize;
    let root = scalar(pyjson::get(kg, "root"));
    if !safe_relative_path(&root, max_path) {
        errors.push("unsafe KG fixture root".to_string());
    }
    if !errors.is_empty() {
        return;
    }
    if perl::numeric(pyjson::get(kg, "file_count")) as usize
        > perl::numeric(pyjson::get(controls, "max_fixture_files")) as usize
    {
        errors.push("KG fixture file count exceeds control bound".to_string());
    }
    if perl::numeric(pyjson::get(kg, "bytes")) as usize
        > perl::numeric(pyjson::get(controls, "max_fixture_bytes")) as usize
    {
        errors.push("KG fixture bytes exceed control bound".to_string());
    }
    let mut paths = if let Some(paths) = tracked_override {
        paths.to_vec()
    } else {
        match git_tracked_paths(base, &root) {
            Ok(paths) => paths,
            Err(error) => {
                errors.push(error);
                Vec::new()
            }
        }
    };
    paths.sort();
    if paths.len() != perl::numeric(pyjson::get(kg, "file_count")) as usize {
        errors.push("tracked KG fixture file count differs".to_string());
    }
    let mut aggregate = String::new();
    let mut bytes = 0;
    let mut fixtures = 0;
    let mut seen = BTreeSet::new();
    for path in paths {
        if !safe_relative_path(&path, max_path) {
            errors.push(format!("unsafe tracked KG fixture path: {path}"));
        }
        if !path.starts_with(&format!("{root}/")) {
            errors.push(format!(
                "tracked KG fixture path is outside the declared root: {path}"
            ));
        }
        if !seen.insert(path.clone()) {
            errors.push(format!("duplicate tracked KG fixture path: {path}"));
        }
        let absolute = base.join(&path);
        if !regular_within(base, &absolute) {
            errors.push(format!(
                "tracked KG fixture file is missing, unsafe, or a symlink: {path}"
            ));
            continue;
        }
        let raw = match read_raw(&absolute) {
            Ok(raw) => raw,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        aggregate.push_str(&format!("{path}\0{}\n", sha256_hex(&raw)));
        bytes += raw.len();
        if path.ends_with("/fixture.json") {
            fixtures += 1;
        }
    }
    if fixtures != perl::numeric(pyjson::get(kg, "fixture_count")) as usize {
        errors.push("tracked KG fixture count differs".to_string());
    }
    if bytes != perl::numeric(pyjson::get(kg, "bytes")) as usize {
        errors.push("tracked KG fixture byte total differs".to_string());
    }
    if sha256_hex(aggregate.as_bytes()) != scalar(pyjson::get(kg, "aggregate_sha256")) {
        errors.push("tracked KG fixture aggregate identity differs".to_string());
    }
}

fn validate_outputs(
    base: &Path,
    outputs: &Py,
    controls: &Py,
    errors: &mut Vec<String>,
    fixture_count: usize,
) {
    if !matches!(outputs, Py::Dict(_)) {
        errors.push("outputs must be an object".to_string());
        return;
    }
    let markdown = pyjson::get(outputs, "markdown");
    let Some(Py::List(entries)) = markdown else {
        errors.push("outputs.markdown must be a non-empty array".to_string());
        return;
    };
    if entries.is_empty() {
        errors.push("outputs.markdown must be a non-empty array".to_string());
    }
    if entries.len() != 10 {
        errors.push("exactly ten managed Markdown outputs are required".to_string());
    }
    if entries.len() > perl::numeric(pyjson::get(controls, "max_markdown_outputs")) as usize {
        errors.push("managed Markdown output count exceeds control bound".to_string());
    }
    let max_path = perl::numeric(pyjson::get(controls, "max_path_bytes")) as usize;
    let max_marker = perl::numeric(pyjson::get(controls, "max_marker_bytes")) as usize;
    let mut ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for entry in entries {
        let errors_before = errors.len();
        if !matches!(entry, Py::Dict(_)) {
            errors.push("managed Markdown output entry must be an object".to_string());
            continue;
        }
        for field in [
            "id",
            "path",
            "kind",
            "start_marker",
            "end_marker",
            "managed_sha256",
            "human_prefix_sha256",
            "human_suffix_sha256",
        ] {
            let item = pyjson::get(entry, field);
            if item.is_none_or(perl::is_reference) || scalar(item).is_empty() {
                errors.push(format!(
                    "managed Markdown {field} must be a non-empty scalar"
                ));
            }
        }
        for field in ["managed_lines", "managed_bytes"] {
            if !positive_integer(pyjson::get(entry, field)) {
                errors.push(format!(
                    "managed Markdown {field} must be a positive integer"
                ));
            }
        }
        if errors.len() > errors_before {
            continue;
        }
        let id = scalar(pyjson::get(entry, "id"));
        let path = scalar(pyjson::get(entry, "path"));
        if !ids.insert(id.clone()) {
            errors.push(format!("duplicate managed Markdown id: {id}"));
        }
        if !paths.insert(path.clone()) {
            errors.push(format!("duplicate managed Markdown path: {path}"));
        }
        if !safe_relative_path(&path, max_path) {
            errors.push(format!("unsafe managed Markdown path: {path}"));
        }
        for field in ["start_marker", "end_marker"] {
            if scalar(pyjson::get(entry, field)).len() > max_marker {
                errors.push(format!("managed Markdown {field} is overlong: {path}"));
            }
        }
        for field in [
            "managed_sha256",
            "human_prefix_sha256",
            "human_suffix_sha256",
        ] {
            if !sha_value(pyjson::get(entry, field)) {
                errors.push(format!("managed Markdown {field} is invalid: {path}"));
            }
        }
        let absolute = base.join(&path);
        if !regular_within(base, &absolute) {
            errors.push(format!(
                "managed Markdown output is missing, unsafe, or a symlink: {path}"
            ));
            continue;
        }
        let raw = match read_raw(&absolute) {
            Ok(raw) => raw,
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        let managed = extract_managed(
            &raw,
            &scalar(pyjson::get(entry, "start_marker")),
            &scalar(pyjson::get(entry, "end_marker")),
        );
        let (managed, prefix, suffix) = match managed {
            Ok(parts) => parts,
            Err(error) => {
                errors.push(format!("{path}: {error}"));
                continue;
            }
        };
        if sha256_hex(&managed) != scalar(pyjson::get(entry, "managed_sha256")) {
            errors.push(format!("{path}: managed output identity differs"));
        }
        if line_count(&managed) != perl::numeric(pyjson::get(entry, "managed_lines")) as usize {
            errors.push(format!("{path}: managed output line count differs"));
        }
        if managed.len() != perl::numeric(pyjson::get(entry, "managed_bytes")) as usize {
            errors.push(format!("{path}: managed output byte count differs"));
        }
        if sha256_hex(&prefix) != scalar(pyjson::get(entry, "human_prefix_sha256")) {
            errors.push(format!("{path}: human prefix identity differs"));
        }
        if sha256_hex(&suffix) != scalar(pyjson::get(entry, "human_suffix_sha256")) {
            errors.push(format!("{path}: human suffix identity differs"));
        }
        if scalar(pyjson::get(entry, "kind")) == "kg_aggregate" {
            let text = String::from_utf8_lossy(&managed);
            let expected = format!("- fixtures_total: `{fixture_count}`");
            if !text.lines().any(|line| line == expected) {
                errors.push(format!(
                    "aggregate KG output does not declare the {fixture_count} tracked fixtures"
                ));
            }
            let expected = format!("- fixtures_passed: `{fixture_count}`");
            if !text.lines().any(|line| line == expected) {
                errors.push(format!(
                    "aggregate KG output does not declare {fixture_count} passed fixtures"
                ));
            }
            if !text.lines().any(|line| line == "- fixtures_failed: `0`") {
                errors.push("aggregate KG output does not declare zero failures".to_string());
            }
        }
    }

    let json = pyjson::get(outputs, "json").unwrap_or(&Py::None);
    if !matches!(json, Py::Dict(_)) {
        errors.push("outputs.json must be an object".to_string());
        return;
    }
    let json_errors_before = errors.len();
    let path = scalar(pyjson::get(json, "path"));
    if !safe_relative_path(&path, max_path) {
        errors.push("unsafe managed JSON path".to_string());
    }
    for field in ["lines", "bytes", "candidate_count"] {
        if !positive_integer(pyjson::get(json, field)) {
            errors.push(format!("outputs.json.{field} must be a positive integer"));
        }
    }
    if !sha_value(pyjson::get(json, "sha256")) {
        errors.push("outputs.json.sha256 is invalid".to_string());
    }
    if errors.len() > json_errors_before {
        return;
    }
    let absolute = base.join(&path);
    if !regular_within(base, &absolute) {
        errors.push("managed JSON output is missing, unsafe, or a symlink".to_string());
        return;
    }
    let raw = match read_raw(&absolute) {
        Ok(raw) => raw,
        Err(error) => {
            errors.push(error);
            return;
        }
    };
    if sha256_hex(&raw) != scalar(pyjson::get(json, "sha256")) {
        errors.push("managed JSON identity differs".to_string());
    }
    if line_count(&raw) != perl::numeric(pyjson::get(json, "lines")) as usize {
        errors.push("managed JSON line count differs".to_string());
    }
    if raw.len() != perl::numeric(pyjson::get(json, "bytes")) as usize {
        errors.push("managed JSON byte count differs".to_string());
    }
    let decoded = String::from_utf8(raw)
        .ok()
        .and_then(|text| pyjson::loads_strict(&text).ok());
    let Some(decoded @ Py::Dict(_)) = decoded else {
        errors.push("managed JSON is malformed".to_string());
        return;
    };
    let candidates = pyjson::get(&decoded, "candidates");
    if !matches!(candidates, Some(Py::List(items)) if items.len() == perl::numeric(pyjson::get(json, "candidate_count")) as usize)
    {
        errors.push("managed JSON candidate count differs".to_string());
    }
    if scalar(pyjson::get(&decoded, "promotion_status"))
        != scalar(pyjson::get(json, "promotion_status"))
    {
        errors.push("managed JSON promotion status differs".to_string());
    }
}

fn validate_producer(
    base: &Path,
    producer: &Py,
    controls: &Py,
    errors: &mut Vec<String>,
    fixture_count: usize,
) {
    if !matches!(producer, Py::Dict(_)) {
        errors.push("producer must be an object".to_string());
        return;
    }
    let regions = pyjson::get(producer, "regions");
    let Some(Py::List(items)) = regions else {
        errors.push("producer.regions must be a non-empty array".to_string());
        return;
    };
    if items.is_empty() {
        errors.push("producer.regions must be a non-empty array".to_string());
    }
    if items.len() > perl::numeric(pyjson::get(controls, "max_producer_regions")) as usize {
        errors.push("producer region count exceeds control bound".to_string());
    }
    if scalar(pyjson::get(producer, "verification_command"))
        != "cargo run --manifest-path Cargo.toml -- kg-bench"
    {
        errors.push("producer verification command differs".to_string());
    }
    if perl::numeric(pyjson::get(producer, "expected_fixture_count")) as usize != fixture_count
        || perl::numeric(pyjson::get(producer, "expected_passed_count")) as usize != fixture_count
    {
        errors.push(
            "producer expected fixture/pass counts differ from the tracked fixture inventory"
                .to_string(),
        );
    }
    if !matches!(
        pyjson::get(producer, "canonical_ir_mutation_allowed"),
        Some(Py::Bool(false))
    ) {
        errors.push("producer must forbid canonical IR mutation".to_string());
    }
    if !matches!(
        pyjson::get(producer, "prior_memory_mutation_allowed"),
        Some(Py::Bool(false))
    ) {
        errors.push("producer must forbid prior-memory mutation".to_string());
    }
    let max_path = perl::numeric(pyjson::get(controls, "max_path_bytes")) as usize;
    let mut ids = BTreeSet::new();
    for region in items {
        let errors_before = errors.len();
        for field in [
            "id",
            "path",
            "start_literal",
            "end_before_literal",
            "sha256",
        ] {
            let item = pyjson::get(region, field);
            if item.is_none_or(perl::is_reference) || scalar(item).is_empty() {
                errors.push(format!(
                    "producer region {field} must be a non-empty scalar"
                ));
            }
        }
        for field in ["lines", "bytes"] {
            if !positive_integer(pyjson::get(region, field)) {
                errors.push(format!(
                    "producer region {field} must be a positive integer"
                ));
            }
        }
        if errors.len() > errors_before {
            continue;
        }
        let id = scalar(pyjson::get(region, "id"));
        if !ids.insert(id.clone()) {
            errors.push(format!("duplicate producer region id: {id}"));
        }
        let path = scalar(pyjson::get(region, "path"));
        if !safe_relative_path(&path, max_path) {
            errors.push(format!("unsafe producer path: {path}"));
        }
        if !sha_value(pyjson::get(region, "sha256")) {
            errors.push(format!("invalid producer region hash: {id}"));
        }
        let absolute = base.join(&path);
        if !regular_within(base, &absolute) {
            errors.push(format!(
                "producer source is missing, unsafe, or a symlink: {path}"
            ));
            continue;
        }
        let raw = match read_raw(&absolute) {
            Ok(raw) => String::from_utf8_lossy(&raw).into_owned(),
            Err(error) => {
                errors.push(error);
                continue;
            }
        };
        let start_literal = scalar(pyjson::get(region, "start_literal"));
        let end_literal = scalar(pyjson::get(region, "end_before_literal"));
        let start = raw
            .find(&start_literal)
            .map(|index| index as isize)
            .unwrap_or(-1);
        let end = if start < 0 {
            -1
        } else {
            raw[start as usize..]
                .find(&end_literal)
                .map(|index| (index + start as usize) as isize)
                .unwrap_or(-1)
        };
        if start < 0 || end < 0 || end <= start {
            errors.push(format!("producer region boundaries differ: {id}"));
            continue;
        }
        if raw[start as usize + 1..].contains(&start_literal) {
            errors.push(format!("producer region start is ambiguous: {id}"));
        }
        let body = &raw.as_bytes()[start as usize..end as usize];
        if sha256_hex(body) != scalar(pyjson::get(region, "sha256")) {
            errors.push(format!("producer region identity differs: {id}"));
        }
        if line_count(body) != perl::numeric(pyjson::get(region, "lines")) as usize {
            errors.push(format!("producer region line count differs: {id}"));
        }
        if body.len() != perl::numeric(pyjson::get(region, "bytes")) as usize {
            errors.push(format!("producer region byte count differs: {id}"));
        }
    }
}

fn validate_contract(
    base: &Path,
    contract: &Py,
    relative_contract: &str,
    tracked_override: Option<&[String]>,
) -> Vec<String> {
    let mut errors = Vec::new();
    if !matches!(contract, Py::Dict(_)) {
        return vec!["contract root must be an object".to_string()];
    }
    if perl::numeric(pyjson::get(contract, "schema_version")) != 1.0 {
        errors.push("schema_version must be 1".to_string());
    }
    if scalar(pyjson::get(contract, "owner")) != "LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5i" {
        errors.push("owner differs from the task-owned authority".to_string());
    }
    for field in ["inputs", "outputs", "producer", "controls"] {
        if !matches!(pyjson::get(contract, field), Some(Py::Dict(_))) {
            errors.push(format!("{field} must be an object"));
        }
    }
    if !errors.is_empty() {
        return errors;
    }
    let controls = pyjson::get(contract, "controls").unwrap();
    for field in [
        "max_contract_bytes",
        "max_markdown_outputs",
        "max_producer_regions",
        "max_path_bytes",
        "max_marker_bytes",
        "max_fixture_files",
        "max_fixture_bytes",
    ] {
        if !positive_integer(pyjson::get(controls, field)) {
            errors.push(format!("controls.{field} must be a positive integer"));
        }
    }
    if !errors.is_empty() {
        return errors;
    }
    if let Ok(raw) = read_raw(&base.join(relative_contract))
        && raw.len() > perl::numeric(pyjson::get(controls, "max_contract_bytes")) as usize
    {
        errors.push("contract exceeds controls.max_contract_bytes".to_string());
    }
    let inputs = pyjson::get(contract, "inputs").unwrap();
    validate_validation_input(
        base,
        pyjson::get(inputs, "validation").unwrap_or(&Py::None),
        pyjson::get(contract, "outputs").unwrap(),
        controls,
        &mut errors,
    );
    validate_kg_input(
        base,
        pyjson::get(inputs, "kg_fixtures").unwrap_or(&Py::None),
        controls,
        tracked_override,
        &mut errors,
    );
    if !errors.is_empty() {
        return errors;
    }
    let fixture_count = perl::numeric(pyjson::get(
        pyjson::get(inputs, "kg_fixtures").unwrap(),
        "fixture_count",
    )) as usize;
    validate_outputs(
        base,
        pyjson::get(contract, "outputs").unwrap(),
        controls,
        &mut errors,
        fixture_count,
    );
    validate_producer(
        base,
        pyjson::get(contract, "producer").unwrap(),
        controls,
        &mut errors,
        fixture_count,
    );
    errors
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mutation {
    Baseline,
    Schema,
    UnsafePath,
    MissingMarker,
    DuplicateMarker,
    ManagedOutputDrift,
    HumanPrefixDrift,
    ManagedJsonDrift,
    MissingTracked,
    ExtraTracked,
    FixtureContentDrift,
    ValidationAuthorityDrift,
    ProducerRegionDrift,
    FixtureCountContractDrift,
    AggregateStaleCount,
    ProducerExpectedCountDrift,
    CanonicalMutationPermission,
}

const CASES: &[(&str, Mutation)] = &[
    ("baseline", Mutation::Baseline),
    ("schema", Mutation::Schema),
    ("unsafe path", Mutation::UnsafePath),
    ("missing marker", Mutation::MissingMarker),
    ("duplicate marker", Mutation::DuplicateMarker),
    ("managed output drift", Mutation::ManagedOutputDrift),
    ("human prefix drift", Mutation::HumanPrefixDrift),
    ("managed JSON drift", Mutation::ManagedJsonDrift),
    ("missing tracked fixture", Mutation::MissingTracked),
    ("extra tracked fixture", Mutation::ExtraTracked),
    ("fixture content drift", Mutation::FixtureContentDrift),
    (
        "validation authority drift",
        Mutation::ValidationAuthorityDrift,
    ),
    ("producer region drift", Mutation::ProducerRegionDrift),
    (
        "fixture count contract drift",
        Mutation::FixtureCountContractDrift,
    ),
    (
        "aggregate projection declares a stale fixture count",
        Mutation::AggregateStaleCount,
    ),
    (
        "producer expected count drifts from the inventory",
        Mutation::ProducerExpectedCountDrift,
    ),
    (
        "canonical mutation permission",
        Mutation::CanonicalMutationPermission,
    ),
];

fn seed_fixture(
    base: &Path,
    fixture: &Path,
    relative_contract: &str,
    contract: &Py,
    tracked: &[String],
) -> Result<(), String> {
    let inputs = pyjson::get(contract, "inputs").unwrap_or(&Py::None);
    let validation = pyjson::get(inputs, "validation").unwrap_or(&Py::None);
    let outputs = pyjson::get(contract, "outputs").unwrap_or(&Py::None);
    let producer = pyjson::get(contract, "producer").unwrap_or(&Py::None);
    let mut paths = tracked.to_vec();
    paths.push(relative_contract.to_string());
    paths.push(scalar(pyjson::get(validation, "snapshot_path")));
    paths.push(scalar(pyjson::get(validation, "authority_contract_path")));
    let snapshot = read_raw(&base.join(scalar(pyjson::get(validation, "snapshot_path"))))?;
    let snapshot = String::from_utf8_lossy(&snapshot);
    let mut remainder = snapshot.as_ref();
    while let Some(index) = remainder.find("](docs/validation-snapshot/") {
        let after = &remainder[index + 2..];
        let Some(end) = after.find(')') else {
            break;
        };
        paths.push(after[..end].to_string());
        remainder = &after[end + 1..];
    }
    if let Some(Py::List(items)) = pyjson::get(outputs, "markdown") {
        paths.extend(items.iter().map(|entry| scalar(pyjson::get(entry, "path"))));
    }
    paths.push(scalar(pyjson::get(
        pyjson::get(outputs, "json").unwrap_or(&Py::None),
        "path",
    )));
    if let Some(Py::List(items)) = pyjson::get(producer, "regions") {
        paths.extend(items.iter().map(|entry| scalar(pyjson::get(entry, "path"))));
    }
    paths.sort();
    paths.dedup();
    for path in paths {
        let source = base.join(&path);
        let destination = fixture.join(&path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::copy(&source, &destination).map_err(|error| error.to_string())?;
    }
    Ok(())
}

fn mutate_case(
    fixture: &Path,
    contract: &mut Py,
    tracked: &mut Vec<String>,
    mutation: Mutation,
) -> Result<(), String> {
    let outputs = pyjson::get(contract, "outputs")
        .unwrap_or(&Py::None)
        .clone();
    let markdown = pyjson::get(&outputs, "markdown")
        .unwrap_or(&Py::None)
        .clone();
    let first = match &markdown {
        Py::List(items) => &items[0],
        _ => return Err("fixture Markdown outputs are missing".to_string()),
    };
    let page_path = scalar(pyjson::get(first, "path"));
    let start = scalar(pyjson::get(first, "start_marker"));
    match mutation {
        Mutation::Baseline => {}
        Mutation::Schema => set_field(contract, "schema_version", integer(99)),
        Mutation::UnsafePath => {
            *nested_mut(contract, &["outputs", "markdown"]) = {
                let mut items = markdown.clone();
                if let Py::List(entries) = &mut items {
                    set_field(
                        &mut entries[0],
                        "path",
                        crate::pyjson::Py::Str(pyjson::py_str("../escape.md")),
                    );
                }
                items
            };
        }
        Mutation::MissingMarker => {
            let path = fixture.join(&page_path);
            let raw = fs::read_to_string(&path).map_err(|error| error.to_string())?;
            write_raw(&path, raw.replacen(&start, "", 1).as_bytes())?;
        }
        Mutation::DuplicateMarker => {
            let path = fixture.join(&page_path);
            let mut raw = fs::read(&path).map_err(|error| error.to_string())?;
            raw.extend_from_slice(start.as_bytes());
            write_raw(&path, &raw)?;
        }
        Mutation::ManagedOutputDrift => {
            let path = fixture.join(&page_path);
            let raw = fs::read_to_string(&path).map_err(|error| error.to_string())?;
            write_raw(&path, raw.replacen("score:", "score-drift:", 1).as_bytes())?;
        }
        Mutation::HumanPrefixDrift => {
            let path = fixture.join(&page_path);
            let raw = fs::read(&path).map_err(|error| error.to_string())?;
            let mut changed = b"human drift\n".to_vec();
            changed.extend_from_slice(&raw);
            write_raw(&path, &changed)?;
        }
        Mutation::ManagedJsonDrift => {
            let path = scalar(pyjson::get(
                pyjson::get(&outputs, "json").unwrap_or(&Py::None),
                "path",
            ));
            let raw = fs::read(fixture.join(&path)).map_err(|error| error.to_string())?;
            let mut changed = raw;
            changed.push(b' ');
            write_raw(&fixture.join(path), &changed)?;
        }
        Mutation::MissingTracked => {
            tracked.pop();
        }
        Mutation::ExtraTracked => {
            let root = scalar(pyjson::get(
                pyjson::get(contract, "inputs").unwrap_or(&Py::None),
                "root",
            ));
            let root = if root.is_empty() {
                scalar(pyjson::get(
                    pyjson::get(
                        pyjson::get(contract, "inputs").unwrap_or(&Py::None),
                        "kg_fixtures",
                    )
                    .unwrap_or(&Py::None),
                    "root",
                ))
            } else {
                root
            };
            let path = format!("{root}/self_test_extra/fixture.json");
            write_raw(&fixture.join(&path), b"{}\n")?;
            tracked.push(path);
        }
        Mutation::FixtureContentDrift => {
            let path = fixture.join(&tracked[0]);
            let mut raw = fs::read(&path).map_err(|error| error.to_string())?;
            raw.push(b' ');
            write_raw(&path, &raw)?;
        }
        Mutation::ValidationAuthorityDrift => {
            let inputs = pyjson::get(contract, "inputs").unwrap_or(&Py::None);
            let validation = pyjson::get(inputs, "validation").unwrap_or(&Py::None);
            let path = scalar(pyjson::get(validation, "authority_contract_path"));
            let mut raw = fs::read(fixture.join(&path)).map_err(|error| error.to_string())?;
            raw.push(b' ');
            write_raw(&fixture.join(path), &raw)?;
        }
        Mutation::ProducerRegionDrift => {
            let producer = pyjson::get(contract, "producer").unwrap_or(&Py::None);
            let regions = pyjson::get(producer, "regions").unwrap_or(&Py::None);
            let first = match regions {
                Py::List(items) => &items[0],
                _ => return Err("fixture producer regions are missing".to_string()),
            };
            let path = scalar(pyjson::get(first, "path"));
            let start = scalar(pyjson::get(first, "start_literal"));
            let raw = fs::read_to_string(fixture.join(&path)).map_err(|error| error.to_string())?;
            write_raw(
                &fixture.join(path),
                raw.replacen(&start, &format!("{start}// drift"), 1)
                    .as_bytes(),
            )?;
        }
        Mutation::FixtureCountContractDrift => {
            let value = nested_mut(contract, &["inputs", "kg_fixtures", "fixture_count"]);
            *value = integer(perl::numeric(Some(value)) as usize + 1);
        }
        Mutation::AggregateStaleCount => {
            let aggregate = match &markdown {
                Py::List(items) => items
                    .iter()
                    .find(|entry| scalar(pyjson::get(entry, "kind")) == "kg_aggregate"),
                _ => None,
            }
            .ok_or_else(|| "aggregate fixture is missing".to_string())?;
            let path = fixture.join(scalar(pyjson::get(aggregate, "path")));
            let raw = fs::read_to_string(&path).map_err(|error| error.to_string())?;
            let changed = raw
                .lines()
                .map(|line| {
                    if line.starts_with("- fixtures_total: `") {
                        "- fixtures_total: `1`".to_string()
                    } else {
                        format!("{line}\n")
                    }
                })
                .collect::<String>();
            write_raw(&path, changed.as_bytes())?;
        }
        Mutation::ProducerExpectedCountDrift => {
            let value = nested_mut(contract, &["producer", "expected_fixture_count"]);
            *value = integer(perl::numeric(Some(value)) as usize + 1);
        }
        Mutation::CanonicalMutationPermission => {
            *nested_mut(contract, &["producer", "canonical_ir_mutation_allowed"]) = Py::Bool(true);
        }
    }
    Ok(())
}

struct Fixture(PathBuf);

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run_self_test(base: &Path, relative_contract: &str) -> Result<usize, String> {
    let source_contract = load_json(&base.join(relative_contract))?;
    let inputs = pyjson::get(&source_contract, "inputs").unwrap_or(&Py::None);
    let kg = pyjson::get(inputs, "kg_fixtures").unwrap_or(&Py::None);
    let tracked = git_tracked_paths(base, &scalar(pyjson::get(kg, "root")))?;
    let fixture_root = base.join(format!(
        "generated/.corpus-kb-currentness-self-test-{}",
        std::process::id()
    ));
    if fixture_root.exists() {
        fs::remove_dir_all(&fixture_root).map_err(|error| error.to_string())?;
    }
    let _guard = Fixture(fixture_root.clone());
    fs::create_dir_all(&fixture_root).map_err(|error| error.to_string())?;
    let mut passed = 0;
    for (index, (name, mutation)) in CASES.iter().enumerate() {
        let fixture = fixture_root.join(format!("case-{index:02}"));
        seed_fixture(
            base,
            &fixture,
            relative_contract,
            &source_contract,
            &tracked,
        )?;
        let mut contract = source_contract.clone();
        let mut case_tracked = tracked.clone();
        mutate_case(&fixture, &mut contract, &mut case_tracked, *mutation)?;
        let errors = validate_contract(&fixture, &contract, relative_contract, Some(&case_tracked));
        if *mutation == Mutation::Baseline {
            if !errors.is_empty() {
                return Err(format!(
                    "{LABEL} self-test `{name}` unexpectedly failed: {}",
                    errors.join("; ")
                ));
            }
        } else if errors.is_empty() {
            return Err(format!("{LABEL} self-test `{name}` unexpectedly passed"));
        }
        passed += 1;
    }
    if passed != 17 {
        return Err(format!(
            "{LABEL} self-test ran {passed} assertions, declaration expects 17 — re-derive the declaration beside the suite"
        ));
    }
    Ok(passed)
}

fn report(contract: &Py) -> Result<String, String> {
    let inputs = pyjson::get(contract, "inputs").unwrap_or(&Py::None);
    let validation = pyjson::get(inputs, "validation").unwrap_or(&Py::None);
    let kg = pyjson::get(inputs, "kg_fixtures").unwrap_or(&Py::None);
    let outputs = pyjson::get(contract, "outputs").unwrap_or(&Py::None);
    let producer = pyjson::get(contract, "producer").unwrap_or(&Py::None);
    let markdown_count = match pyjson::get(outputs, "markdown") {
        Some(Py::List(items)) => items.len(),
        _ => 0,
    };
    let region_count = match pyjson::get(producer, "regions") {
        Some(Py::List(items)) => items.len(),
        _ => 0,
    };
    encode(
        &object(vec![
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
                "reviewed_validation_snapshot",
                pyjson::get(validation, "snapshot_path")
                    .unwrap_or(&Py::None)
                    .clone(),
            ),
            (
                "fixture_root",
                pyjson::get(kg, "root").unwrap_or(&Py::None).clone(),
            ),
            (
                "fixture_files",
                pyjson::get(kg, "file_count").unwrap_or(&Py::None).clone(),
            ),
            (
                "fixtures",
                pyjson::get(kg, "fixture_count")
                    .unwrap_or(&Py::None)
                    .clone(),
            ),
            (
                "passed",
                pyjson::get(producer, "expected_passed_count")
                    .unwrap_or(&Py::None)
                    .clone(),
            ),
            ("managed_markdown_outputs", integer(markdown_count)),
            ("managed_json_outputs", integer(1)),
            ("producer_regions", integer(region_count)),
            ("human_regions_preserved", Py::Bool(true)),
            ("canonical_ir_mutation_allowed", Py::Bool(false)),
            ("prior_memory_mutation_allowed", Py::Bool(false)),
        ]),
        None,
    )
}

/// Run corpus KB currentness with its original mode and path arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut root = root.to_path_buf();
    let mut relative_contract = "doctrine/live_document_size/corpus_kb.json".to_string();
    let mut mode = Mode::Check;
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" => mode = Mode::Check,
            "--report" => mode = Mode::Report,
            "--self-test" => mode = Mode::SelfTest,
            "--root" => match arguments.next() {
                Some(directory) => root = PathBuf::from(directory),
                None => {
                    output.err("--root requires a value\n");
                    return 1;
                }
            },
            "--contract" => match arguments.next() {
                Some(path) => relative_contract = path.to_string(),
                None => {
                    output.err("--contract requires a value\n");
                    return 1;
                }
            },
            _ => {
                output.err("Usage: cargo xtask check corpus-kb-currentness [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n");
                return 2;
            }
        }
    }
    let root = match fs::canonicalize(&root) {
        Ok(root) if root.is_dir() => root,
        _ => {
            output.err(&format!("{LABEL}: repository root does not exist\n"));
            return 1;
        }
    };
    if !safe_relative_path(&relative_contract, 512) {
        output.err(&format!("{LABEL}: unsafe contract path\n"));
        return 1;
    }
    if mode == Mode::SelfTest {
        return match run_self_test(&root, &relative_contract) {
            Ok(passed) => {
                output.out(&format!("{LABEL}: self-test {passed}/17 passed.\n"));
                0
            }
            Err(error) => {
                output.err(&format!("{error}\n"));
                1
            }
        };
    }
    let contract = match load_json(&root.join(&relative_contract)) {
        Ok(contract) => contract,
        Err(error) => {
            output.err(&format!("{error}\n"));
            return if error.starts_with("corpus-kb-currentness: cannot read ") {
                2
            } else {
                255
            };
        }
    };
    let errors = validate_contract(&root, &contract, &relative_contract, None);
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
        let inputs = pyjson::get(&contract, "inputs").unwrap_or(&Py::None);
        let fixtures = scalar(pyjson::get(
            pyjson::get(inputs, "kg_fixtures").unwrap_or(&Py::None),
            "fixture_count",
        ));
        output.out(&format!(
            "{LABEL}: reviewed validation, {fixtures}/{fixtures} KG fixtures, ten managed Markdown blocks, paired JSON, human regions, and producer seams are current.\n"
        ));
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_seventeen_cases_pass_without_residue() {
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
            "corpus-kb-currentness: self-test 17/17 passed.\n"
        );
        assert!(
            !root
                .join(format!(
                    "generated/.corpus-kb-currentness-self-test-{}",
                    std::process::id()
                ))
                .exists()
        );
    }

    #[test]
    fn the_real_inventory_and_report_hold() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--report".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("\"fixtures\":158"));
        assert!(output.text.contains("\"passed\":158"));
        assert!(output.text.contains("\"managed_markdown_outputs\":10"));
    }
}
