//! Source PDF registry currentness, native since `RUSTDOC-AND-XTASK.15.p`: Git-indexed corpus authority,
//! exact registry identity, derived keys, PDF signatures, producer regions and the 13-case suite.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use sha2::{Digest, Sha256};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "source-pdf-registry-currentness";

#[derive(Clone)]
struct Row {
    document_key: String,
    class: String,
    path: String,
    directory: String,
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
        && text
            .chars()
            .next()
            .is_some_and(|character| ('1'..='9').contains(&character))
        && text.chars().all(|character| character.is_ascii_digit())
}

fn safe_relative_path(path: &str, max_bytes: usize) -> bool {
    !path.is_empty()
        && path.len() <= max_bytes
        && !path.starts_with('/')
        && !path.contains(['\\', '\r', '\n', '\0', '`', '|'])
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

fn canonical_path_is_within(base: &Path, path: &Path) -> bool {
    let Ok(base) = fs::canonicalize(base) else {
        return false;
    };
    let Ok(path) = fs::canonicalize(path) else {
        return false;
    };
    path.starts_with(base)
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
        .unwrap_or_else(|_| "scripts/check_source_pdf_registry_currentness.pl".to_string())
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
                ", or }} expected while parsing object/hash, at character offset {offset} (before \"(end of string)\") at {} line 688.",
                shim_path()
            )
        } else {
            error
        }
    })
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

fn compare_identity(raw: &[u8], declared: &Py, label: &str, errors: &mut Vec<String>) {
    let (lines, bytes, line_bytes, sha256) = metrics(raw);
    for (field, actual) in [
        ("lines", lines.to_string()),
        ("bytes", bytes.to_string()),
        ("line_bytes", line_bytes.to_string()),
        ("sha256", sha256),
    ] {
        if pyjson::get(declared, field).is_none() || scalar(pyjson::get(declared, field)) != actual
        {
            errors.push(format!("{label} {field} differs"));
        }
    }
}

fn literal_occurrences(raw: &str, literal: &str) -> usize {
    if literal.is_empty() {
        0
    } else {
        raw.matches(literal).count()
    }
}

fn parse_registry(text: &str, registry: &Py) -> (Vec<Row>, Vec<String>) {
    let mut errors = Vec::new();
    let lines: Vec<&str> = text.split('\n').collect();
    let header = scalar(pyjson::get(registry, "table_header"));
    let indices: Vec<usize> = lines
        .iter()
        .enumerate()
        .filter_map(|(index, line)| (*line == header).then_some(index))
        .collect();
    if indices.len() != 1 {
        errors.push("registry must contain exactly one declared table header".to_string());
        return (Vec::new(), errors);
    }
    let index = indices[0];
    if lines.get(index + 1).copied().unwrap_or("")
        != scalar(pyjson::get(registry, "table_separator"))
    {
        errors.push("registry table separator differs".to_string());
        return (Vec::new(), errors);
    }
    let mut rows = Vec::new();
    for (line_index, line) in lines.iter().enumerate().skip(index + 2) {
        if !line.starts_with('|') {
            break;
        }
        let Some(mut remainder) = line.strip_prefix("| `") else {
            errors.push(format!("malformed registry row at line {}", line_index + 1));
            continue;
        };
        let Some((key, rest)) = remainder.split_once("` | ") else {
            errors.push(format!("malformed registry row at line {}", line_index + 1));
            continue;
        };
        remainder = rest;
        let Some((class, rest)) = remainder.split_once(" | `") else {
            errors.push(format!("malformed registry row at line {}", line_index + 1));
            continue;
        };
        remainder = rest;
        let Some((path, rest)) = remainder.split_once("` | `") else {
            errors.push(format!("malformed registry row at line {}", line_index + 1));
            continue;
        };
        let Some(directory) = rest.strip_suffix("` |") else {
            errors.push(format!("malformed registry row at line {}", line_index + 1));
            continue;
        };
        rows.push(Row {
            document_key: key.to_string(),
            class: class.trim().to_string(),
            path: path.to_string(),
            directory: directory.to_string(),
        });
    }
    (rows, errors)
}

fn git_tracked_pdfs(base: &Path, root: &str, extension: &str) -> Result<Vec<String>, String> {
    let process = Command::new("git")
        .arg("-C")
        .arg(base)
        .arg("ls-files")
        .arg("-z")
        .arg("--")
        .arg(root)
        .output()
        .map_err(|error| format!("cannot run git ls-files: {error}"))?;
    if !process.status.success() {
        return Err("git ls-files failed".to_string());
    }
    let mut paths = Vec::new();
    for raw_path in process.stdout.split(|byte| *byte == 0) {
        if raw_path.is_empty() {
            continue;
        }
        let path = std::str::from_utf8(raw_path)
            .map_err(|_| "git ls-files emitted a non-UTF-8 path".to_string())?;
        if path.len() >= extension.len() && path.to_lowercase().ends_with(&extension.to_lowercase())
        {
            paths.push(path.to_string());
        }
    }
    paths.sort();
    Ok(paths)
}

fn derive_document_key(path: &str, extension: &str) -> String {
    let name = Path::new(path)
        .file_name()
        .unwrap_or_default()
        .to_string_lossy();
    if name.len() < extension.len() || !name.to_lowercase().ends_with(&extension.to_lowercase()) {
        return "source".to_string();
    }
    let name = &name[..name.len() - extension.len()];
    let mut key = String::new();
    let mut last_was_separator = false;
    for character in name.chars() {
        if character.is_ascii_alphanumeric() {
            key.push(character.to_ascii_lowercase());
            last_was_separator = false;
        } else if !last_was_separator && !key.is_empty() {
            key.push('_');
            last_was_separator = true;
        }
    }
    let key = key.trim_matches('_');
    if key.is_empty() {
        "source".to_string()
    } else {
        key.to_string()
    }
}

fn corpus_directory(path: &str, root: &str) -> String {
    let relative = path.strip_prefix(&format!("{root}/")).unwrap_or(path);
    relative
        .rsplit_once('/')
        .map(|(directory, _)| format!("{directory}/"))
        .unwrap_or_default()
}

fn extract_braced_region(raw: &str, signature: &str) -> Result<String, String> {
    if signature.is_empty() {
        return Err("signature is empty".to_string());
    }
    let count = literal_occurrences(raw, signature);
    if count != 1 {
        return Err(format!("signature occurrence count is {count}"));
    }
    let start = raw.find(signature).unwrap();
    let Some(open) = raw[start..].find('{').map(|index| index + start) else {
        return Err("opening brace is missing".to_string());
    };
    let mut depth = 0;
    for (offset, character) in raw[open..].char_indices() {
        if character == '{' {
            depth += 1;
        } else if character == '}' {
            depth -= 1;
            if depth == 0 {
                return Ok(raw[start..=open + offset].to_string());
            }
        }
    }
    Err("closing brace is missing".to_string())
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
        errors.push("unsupported schema_version".to_string());
    }
    if scalar(pyjson::get(contract, "owner")) != "LIVE-DOCUMENT-SIZE-CONTAINMENT-ADOPTION.5g.ii" {
        errors.push("owner differs from the task-owned authority".to_string());
    }
    for name in ["registry", "tracked_corpus", "derivation", "controls"] {
        if !matches!(pyjson::get(contract, name), Some(Py::Dict(_))) {
            errors.push(format!("{name} must be an object"));
        }
    }
    if !errors.is_empty() {
        return errors;
    }
    let registry = pyjson::get(contract, "registry").unwrap();
    let tracked = pyjson::get(contract, "tracked_corpus").unwrap();
    let derivation = pyjson::get(contract, "derivation").unwrap();
    let controls = pyjson::get(contract, "controls").unwrap();
    for field in [
        "max_contract_bytes",
        "max_registry_rows",
        "max_required_literals",
        "max_path_bytes",
        "max_class_bytes",
        "max_producer_regions",
        "max_producer_seams",
    ] {
        if !positive_integer(pyjson::get(controls, field)) {
            errors.push(format!("controls.{field} must be a positive integer"));
        }
    }
    if !errors.is_empty() {
        return errors;
    }
    let limit = |field| perl::numeric(pyjson::get(controls, field)) as usize;
    if let Ok(raw) = read_raw(&base.join(relative_contract))
        && raw.len() > limit("max_contract_bytes")
    {
        errors.push("contract exceeds controls.max_contract_bytes".to_string());
    }
    for field in ["path", "h1", "table_header", "table_separator", "sha256"] {
        let item = pyjson::get(registry, field);
        if item.is_none_or(perl::is_reference) || scalar(item).is_empty() {
            errors.push(format!("registry.{field} must be a non-empty scalar"));
        }
    }
    for field in ["lines", "bytes", "line_bytes", "row_count"] {
        if !positive_integer(pyjson::get(registry, field)) {
            errors.push(format!("registry.{field} must be a positive integer"));
        }
    }
    let required_literals = pyjson::get(registry, "required_literals");
    if !matches!(required_literals, Some(Py::List(items)) if !items.is_empty()) {
        errors.push("registry.required_literals must be a non-empty array".to_string());
    }
    if matches!(required_literals, Some(Py::List(items)) if items.len() > limit("max_required_literals"))
    {
        errors.push("registry.required_literals exceeds its control bound".to_string());
    }
    if !errors.is_empty() {
        return errors;
    }

    let registry_relative = scalar(pyjson::get(registry, "path"));
    if !safe_relative_path(&registry_relative, limit("max_path_bytes")) {
        errors.push("unsafe registry path".to_string());
    }
    let registry_path = base.join(&registry_relative);
    if !registry_path.is_file()
        || fs::symlink_metadata(&registry_path)
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        errors.push("registry path is missing, non-regular, or a symlink".to_string());
        return errors;
    }
    if !canonical_path_is_within(base, &registry_path) {
        errors.push("registry path escapes the repository root".to_string());
    }
    let registry_raw = match read_raw(&registry_path) {
        Ok(raw) => raw,
        Err(error) => {
            errors.push(error);
            return errors;
        }
    };
    compare_identity(&registry_raw, registry, "registry", &mut errors);
    let registry_text = match String::from_utf8(registry_raw) {
        Ok(text) => text,
        Err(_) => {
            errors.push("registry is not valid UTF-8".to_string());
            return errors;
        }
    };
    if registry_text.split('\n').next().unwrap_or("") != scalar(pyjson::get(registry, "h1")) {
        errors.push("registry H1 differs".to_string());
    }
    if let Some(Py::List(items)) = required_literals {
        for literal in items {
            if perl::is_reference(literal) || scalar(Some(literal)).is_empty() {
                errors.push("registry required literal is invalid".to_string());
            }
            let literal = scalar(Some(literal));
            if !literal.is_empty() && !registry_text.contains(&literal) {
                errors.push(format!("registry required literal is missing: {literal}"));
            }
        }
    }
    let (rows, parse_errors) = parse_registry(&registry_text, registry);
    errors.extend(parse_errors);
    if rows.len() != perl::numeric(pyjson::get(registry, "row_count")) as usize {
        errors.push("registry row count differs".to_string());
    }
    if rows.len() > limit("max_registry_rows") {
        errors.push("registry row count exceeds its control bound".to_string());
    }

    let errors_before_tracked_shape = errors.len();
    for field in ["root", "extension", "file_signature"] {
        let item = pyjson::get(tracked, field);
        if item.is_none_or(perl::is_reference) || scalar(item).is_empty() {
            errors.push(format!("tracked_corpus.{field} must be a non-empty scalar"));
        }
    }
    if !positive_integer(pyjson::get(tracked, "file_count")) {
        errors.push("tracked_corpus.file_count must be a positive integer".to_string());
    }
    if !matches!(
        pyjson::get(tracked, "git_index_is_authoritative"),
        Some(Py::Bool(true))
    ) {
        errors.push("tracked_corpus.git_index_is_authoritative must be true".to_string());
    }
    if !matches!(
        pyjson::get(tracked, "host_local_or_generated_is_authoritative"),
        Some(Py::Bool(false))
    ) {
        errors.push(
            "tracked_corpus.host_local_or_generated_is_authoritative must be false".to_string(),
        );
    }
    if errors.len() > errors_before_tracked_shape {
        return errors;
    }
    let corpus_root = scalar(pyjson::get(tracked, "root"));
    let extension = scalar(pyjson::get(tracked, "extension"));
    if !safe_relative_path(&corpus_root, limit("max_path_bytes")) {
        errors.push("unsafe tracked corpus root".to_string());
    }
    if extension.to_lowercase() != ".pdf" {
        errors.push("tracked corpus extension must be .pdf".to_string());
    }
    let mut tracked_paths = if let Some(paths) = tracked_override {
        paths.to_vec()
    } else {
        match git_tracked_pdfs(base, &corpus_root, &extension) {
            Ok(paths) => paths,
            Err(error) => {
                errors.push(error);
                Vec::new()
            }
        }
    };
    tracked_paths.sort();
    if tracked_paths.len() != perl::numeric(pyjson::get(tracked, "file_count")) as usize {
        errors.push("tracked PDF count differs".to_string());
    }
    if perl::numeric(pyjson::get(registry, "row_count")) as usize
        != perl::numeric(pyjson::get(tracked, "file_count")) as usize
    {
        errors.push("registry/tracked file count contract differs".to_string());
    }
    let mut tracked_seen = BTreeSet::new();
    for path in &tracked_paths {
        if !safe_relative_path(path, limit("max_path_bytes")) {
            errors.push(format!("unsafe tracked PDF path: {path}"));
        }
        if !path.starts_with(&format!("{corpus_root}/")) {
            errors.push(format!("tracked PDF is outside corpus root: {path}"));
        }
        if !path.to_lowercase().ends_with(&extension.to_lowercase()) {
            errors.push(format!(
                "tracked corpus member does not end in {extension}: {path}"
            ));
        }
        if !tracked_seen.insert(path.clone()) {
            errors.push(format!("duplicate tracked PDF path: {path}"));
        }
    }

    let mut row_paths = BTreeSet::new();
    let mut row_keys = BTreeSet::new();
    for row in &rows {
        if !safe_relative_path(&row.path, limit("max_path_bytes")) {
            errors.push(format!("unsafe registry path: {}", row.path));
        }
        if !row.path.starts_with(&format!("{corpus_root}/")) {
            errors.push(format!(
                "registry path is outside corpus root: {}",
                row.path
            ));
        }
        if row.class.is_empty() || row.class.len() > limit("max_class_bytes") {
            errors.push(format!(
                "registry class is empty or overlong for {}",
                row.path
            ));
        }
        if !row_keys.insert(row.document_key.clone()) {
            errors.push(format!("duplicate document_key: {}", row.document_key));
        }
        if !row_paths.insert(row.path.clone()) {
            errors.push(format!("duplicate registry path: {}", row.path));
        }
        if !tracked_seen.contains(&row.path) {
            errors.push(format!(
                "registry path is not a tracked corpus PDF: {}",
                row.path
            ));
        }
        let derived = derive_document_key(&row.path, &extension);
        if row.document_key != derived {
            errors.push(format!(
                "document_key differs from code-derived key for {}: {} != {}",
                row.path, row.document_key, derived
            ));
        }
        let expected_directory = corpus_directory(&row.path, &corpus_root);
        if row.directory != expected_directory {
            errors.push(format!(
                "corpus directory differs for {}: {} != {}",
                row.path, row.directory, expected_directory
            ));
        }
        let absolute = base.join(&row.path);
        if !absolute.is_file()
            || fs::symlink_metadata(&absolute)
                .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            errors.push(format!(
                "registered PDF is missing, non-regular, or a symlink: {}",
                row.path
            ));
            continue;
        }
        if !canonical_path_is_within(base, &absolute) {
            errors.push(format!(
                "registered PDF escapes the repository root: {}",
                row.path
            ));
            continue;
        }
        let signature = scalar(pyjson::get(tracked, "file_signature"));
        let actual = fs::read(&absolute)
            .map(|raw| raw[..raw.len().min(signature.len())].to_vec())
            .unwrap_or_default();
        if actual != signature.as_bytes() {
            errors.push(format!(
                "registered file does not start with {signature}: {}",
                row.path
            ));
        }
    }
    for path in &tracked_paths {
        if !row_paths.contains(path) {
            errors.push(format!("tracked PDF is missing a registry row: {path}"));
        }
    }

    let source_relative = scalar(pyjson::get(derivation, "source_path"));
    if !safe_relative_path(&source_relative, limit("max_path_bytes")) {
        errors.push("derivation.source_path must be a safe repository-relative path".to_string());
    }
    let regions = pyjson::get(derivation, "regions");
    if !matches!(regions, Some(Py::List(items)) if !items.is_empty()) {
        errors.push("derivation.regions must be a non-empty array".to_string());
    }
    let seams = pyjson::get(derivation, "seams");
    if !matches!(seams, Some(Py::List(items)) if !items.is_empty()) {
        errors.push("derivation.seams must be a non-empty array".to_string());
    }
    if matches!(regions, Some(Py::List(items)) if items.len() > limit("max_producer_regions")) {
        errors.push("derivation.regions exceeds its control bound".to_string());
    }
    if matches!(seams, Some(Py::List(items)) if items.len() > limit("max_producer_seams")) {
        errors.push("derivation.seams exceeds its control bound".to_string());
    }
    if !errors.is_empty() {
        return errors;
    }
    let source_path = base.join(&source_relative);
    if !source_path.is_file()
        || fs::symlink_metadata(&source_path)
            .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        errors.push("derivation source is missing, non-regular, or a symlink".to_string());
        return errors;
    }
    let source_raw = match read_raw(&source_path) {
        Ok(raw) => String::from_utf8_lossy(&raw).into_owned(),
        Err(error) => {
            errors.push(error);
            return errors;
        }
    };
    let mut region_names = BTreeSet::new();
    if let Some(Py::List(items)) = regions {
        for region in items {
            if !matches!(region, Py::Dict(_)) {
                errors.push("derivation region must be an object".to_string());
                continue;
            }
            let name = scalar(pyjson::get(region, "name"));
            if name.is_empty() || !region_names.insert(name.clone()) {
                errors.push(format!("duplicate or empty derivation region name: {name}"));
            }
            let signature = scalar(pyjson::get(region, "signature"));
            match extract_braced_region(&source_raw, &signature) {
                Ok(raw) => compare_identity(
                    raw.as_bytes(),
                    region,
                    &format!("derivation region {name}"),
                    &mut errors,
                ),
                Err(error) => errors.push(format!("derivation region {name}: {error}")),
            }
        }
    }
    if let Some(Py::List(items)) = seams {
        for seam in items {
            if !matches!(seam, Py::Dict(_))
                || pyjson::get(seam, "literal").is_none_or(perl::is_reference)
                || !positive_integer(pyjson::get(seam, "occurrences"))
            {
                errors.push("derivation seam is malformed".to_string());
                continue;
            }
            let literal = scalar(pyjson::get(seam, "literal"));
            let actual = literal_occurrences(&source_raw, &literal);
            if actual != perl::numeric(pyjson::get(seam, "occurrences")) as usize {
                errors.push(format!("derivation seam occurrence differs: {literal}"));
            }
        }
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

fn replace_once(path: &Path, from: &str, to: &str) -> Result<(), String> {
    let raw = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let count = literal_occurrences(&raw, from);
    if count != 1 {
        return Err(format!(
            "{LABEL} self-test: replacement '{from}' count is {count}"
        ));
    }
    write_raw(path, raw.replacen(from, to, 1).as_bytes())
}

fn append_raw(path: &Path, suffix: &str) -> Result<(), String> {
    let mut raw = fs::read(path).map_err(|error| error.to_string())?;
    raw.extend_from_slice(suffix.as_bytes());
    write_raw(path, &raw)
}

fn remove_first_registry_row(path: &Path) -> Result<(), String> {
    let raw = fs::read_to_string(path).map_err(|error| error.to_string())?;
    let mut removed = 0;
    let mut result = String::new();
    for line in raw.split_inclusive('\n') {
        if removed == 0 && line.starts_with("| `") {
            removed += 1;
        } else {
            result.push_str(line);
        }
    }
    if removed != 1 {
        return Err(format!("{LABEL} self-test: row removal count is {removed}"));
    }
    write_raw(path, result.as_bytes())
}

fn write_contract(base: &Path, relative: &str, contract: &Py) -> Result<(), String> {
    write_raw(&base.join(relative), encode(contract, Some(2))?.as_bytes())
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

fn seed_fixture(
    base: &Path,
    fixture: &Path,
    relative_contract: &str,
    contract: &Py,
    rows: &[Row],
) -> Result<(), String> {
    let registry = pyjson::get(contract, "registry").unwrap_or(&Py::None);
    let derivation = pyjson::get(contract, "derivation").unwrap_or(&Py::None);
    for path in [
        relative_contract.to_string(),
        scalar(pyjson::get(registry, "path")),
        scalar(pyjson::get(derivation, "source_path")),
    ] {
        let source = base.join(&path);
        let destination = fixture.join(&path);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::copy(&source, &destination).map_err(|error| error.to_string())?;
    }
    for row in rows {
        write_raw(&fixture.join(&row.path), b"%PDF-1.7\n")?;
    }
    Ok(())
}

fn fixture_rows(fixture: &Path, contract: &Py) -> Result<Vec<Row>, String> {
    let registry = pyjson::get(contract, "registry").unwrap_or(&Py::None);
    let raw = read_raw(&fixture.join(scalar(pyjson::get(registry, "path"))))?;
    let text = String::from_utf8(raw).map_err(|error| error.to_string())?;
    let (rows, errors) = parse_registry(&text, registry);
    if !errors.is_empty() {
        return Err(format!(
            "{LABEL} self-test: fixture parse failed: {}",
            errors.join("; ")
        ));
    }
    Ok(rows)
}

#[derive(Clone, Copy)]
enum Mutation {
    Valid,
    RegistryIdentity,
    RegistryRowOmitted,
    ExtraTracked,
    RegistryPathNotTracked,
    DocumentKeyDrift,
    DuplicateDocumentKey,
    DuplicateRegistryPath,
    CorpusDirectoryDrift,
    UnsafeRegistryPath,
    InvalidPdfSignature,
    ProducerDrift,
    UnknownSchema,
}

const CASES: &[(&str, bool, &str, Mutation)] = &[
    ("valid tracked registry", true, "", Mutation::Valid),
    (
        "registry identity drift",
        false,
        "registry bytes differs",
        Mutation::RegistryIdentity,
    ),
    (
        "registry row omitted",
        false,
        "tracked PDF is missing a registry row",
        Mutation::RegistryRowOmitted,
    ),
    (
        "tracked PDF omitted from registry",
        false,
        "tracked PDF is missing a registry row",
        Mutation::ExtraTracked,
    ),
    (
        "registry path not tracked",
        false,
        "registry path is not a tracked corpus PDF",
        Mutation::RegistryPathNotTracked,
    ),
    (
        "document key drift",
        false,
        "document_key differs from code-derived key",
        Mutation::DocumentKeyDrift,
    ),
    (
        "duplicate document key",
        false,
        "duplicate document_key",
        Mutation::DuplicateDocumentKey,
    ),
    (
        "duplicate registry path",
        false,
        "duplicate registry path",
        Mutation::DuplicateRegistryPath,
    ),
    (
        "corpus directory drift",
        false,
        "corpus directory differs",
        Mutation::CorpusDirectoryDrift,
    ),
    (
        "unsafe registry path",
        false,
        "unsafe registry path",
        Mutation::UnsafeRegistryPath,
    ),
    (
        "invalid PDF signature",
        false,
        "registered file does not start with",
        Mutation::InvalidPdfSignature,
    ),
    (
        "producer derivation drift",
        false,
        "derivation region document_key",
        Mutation::ProducerDrift,
    ),
    (
        "unknown schema",
        false,
        "unsupported schema_version",
        Mutation::UnknownSchema,
    ),
];

struct Fixture(PathBuf);

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn mutate_case(
    fixture: &Path,
    relative_contract: &str,
    contract: &mut Py,
    tracked: &mut Vec<String>,
    mutation: Mutation,
) -> Result<(), String> {
    let registry = pyjson::get(contract, "registry").unwrap_or(&Py::None);
    let registry_path = scalar(pyjson::get(registry, "path"));
    let rows = fixture_rows(fixture, contract)?;
    match mutation {
        Mutation::Valid => {}
        Mutation::RegistryIdentity => append_raw(&fixture.join(&registry_path), "\nDRIFT\n")?,
        Mutation::RegistryRowOmitted => remove_first_registry_row(&fixture.join(&registry_path))?,
        Mutation::ExtraTracked => {
            let path = "corpus/fixture/extra/Extra-Spec.pdf";
            write_raw(&fixture.join(path), b"%PDF-1.7\n")?;
            tracked.push(path.to_string());
        }
        Mutation::RegistryPathNotTracked => {
            tracked.remove(0);
        }
        Mutation::DocumentKeyDrift => replace_once(
            &fixture.join(&registry_path),
            &format!("| `{}` |", rows[0].document_key),
            "| `wrong_key` |",
        )?,
        Mutation::DuplicateDocumentKey => replace_once(
            &fixture.join(&registry_path),
            &format!("| `{}` |", rows[1].document_key),
            &format!("| `{}` |", rows[0].document_key),
        )?,
        Mutation::DuplicateRegistryPath => replace_once(
            &fixture.join(&registry_path),
            &format!("`{}`", rows[1].path),
            &format!("`{}`", rows[0].path),
        )?,
        Mutation::CorpusDirectoryDrift => replace_once(
            &fixture.join(&registry_path),
            &format!("`{}` |", rows[0].directory),
            "`wrong/directory/` |",
        )?,
        Mutation::UnsafeRegistryPath => replace_once(
            &fixture.join(&registry_path),
            &format!("`{}`", rows[0].path),
            "`../outside.pdf`",
        )?,
        Mutation::InvalidPdfSignature => write_raw(&fixture.join(&tracked[0]), b"NOT-A-PDF\n")?,
        Mutation::ProducerDrift => {
            let derivation = pyjson::get(contract, "derivation").unwrap_or(&Py::None);
            replace_once(
                &fixture.join(scalar(pyjson::get(derivation, "source_path"))),
                "if ch.is_ascii_alphanumeric() {",
                "if ch.is_alphanumeric() {",
            )?;
        }
        Mutation::UnknownSchema => {
            set_field(contract, "schema_version", integer(2));
            write_contract(fixture, relative_contract, contract)?;
        }
    }
    Ok(())
}

fn run_self_test(base: &Path, relative_contract: &str) -> Result<usize, String> {
    let fixture_root = base.join(format!(
        "generated/.source-pdf-registry-currentness-test.{}",
        std::process::id()
    ));
    if fixture_root.exists() {
        fs::remove_dir_all(&fixture_root)
            .map_err(|error| format!("{LABEL} self-test: pre-clean failed: {error}"))?;
    }
    let _guard = Fixture(fixture_root.clone());
    fs::create_dir_all(&fixture_root).map_err(|error| error.to_string())?;
    let source_contract = load_json(&base.join(relative_contract))?;
    let registry = pyjson::get(&source_contract, "registry").unwrap_or(&Py::None);
    let registry_raw = read_raw(&base.join(scalar(pyjson::get(registry, "path"))))?;
    let registry_text = String::from_utf8(registry_raw).map_err(|error| error.to_string())?;
    let (source_rows, source_errors) = parse_registry(&registry_text, registry);
    if !source_errors.is_empty() {
        return Err(format!(
            "{LABEL} self-test: cannot parse source registry: {}",
            source_errors.join("; ")
        ));
    }
    let source_tracked: Vec<String> = source_rows.iter().map(|row| row.path.clone()).collect();
    let mut passed = 0;
    for (index, (name, expected_ok, expected_error, mutation)) in CASES.iter().enumerate() {
        let fixture = fixture_root.join(format!("case-{:02}", index + 1));
        seed_fixture(
            base,
            &fixture,
            relative_contract,
            &source_contract,
            &source_rows,
        )?;
        let mut contract = load_json(&fixture.join(relative_contract))?;
        let mut tracked = source_tracked.clone();
        mutate_case(
            &fixture,
            relative_contract,
            &mut contract,
            &mut tracked,
            *mutation,
        )?;
        let contract = load_json(&fixture.join(relative_contract))?;
        let case_errors = validate_contract(&fixture, &contract, relative_contract, Some(&tracked));
        let actual_ok = case_errors.is_empty();
        if actual_ok != *expected_ok {
            return Err(format!(
                "{LABEL} self-test '{name}' expected {}, got {}: {}",
                *expected_ok as usize,
                actual_ok as usize,
                case_errors.join("; ")
            ));
        }
        if !expected_ok && !case_errors.join("; ").contains(expected_error) {
            return Err(format!(
                "{LABEL} self-test '{name}' did not emit '{expected_error}': {}",
                case_errors.join("; ")
            ));
        }
        passed += 1;
    }
    if passed != 13 {
        return Err(format!(
            "{LABEL} self-test: expected 13 cases, passed {passed}"
        ));
    }
    Ok(passed)
}

fn report(contract: &Py) -> Result<String, String> {
    let registry = pyjson::get(contract, "registry").unwrap_or(&Py::None);
    let tracked = pyjson::get(contract, "tracked_corpus").unwrap_or(&Py::None);
    let producer = pyjson::get(contract, "derivation").unwrap_or(&Py::None);
    let count = |value, field| match pyjson::get(value, field) {
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
                "registry",
                object(
                    [
                        "path",
                        "row_count",
                        "lines",
                        "bytes",
                        "line_bytes",
                        "sha256",
                    ]
                    .into_iter()
                    .map(|field| {
                        let name = if field == "row_count" { "rows" } else { field };
                        (
                            name,
                            pyjson::get(registry, field).unwrap_or(&Py::None).clone(),
                        )
                    })
                    .collect(),
                ),
            ),
            (
                "tracked_corpus",
                object(vec![
                    (
                        "root",
                        pyjson::get(tracked, "root").unwrap_or(&Py::None).clone(),
                    ),
                    (
                        "pdfs",
                        pyjson::get(tracked, "file_count")
                            .unwrap_or(&Py::None)
                            .clone(),
                    ),
                    ("git_index_is_authoritative", Py::Bool(true)),
                    ("host_local_or_generated_is_authoritative", Py::Bool(false)),
                ]),
            ),
            ("derivation_regions", integer(count(producer, "regions"))),
            ("derivation_seams", integer(count(producer, "seams"))),
        ]),
        None,
    )
}

/// Run Source PDF registry currentness with its original mode and path arguments.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut root = root.to_path_buf();
    let mut relative_contract = "doctrine/live_document_size/source_pdf_registry.json".to_string();
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
                output.err("Usage: cargo xtask check source-pdf-registry-currentness [--check|--report|--self-test] [--root PROJECT_ROOT] [--contract PATH]\n");
                return 2;
            }
        }
    }
    let root = match fs::canonicalize(&root) {
        Ok(root) if root.is_dir() => root,
        _ => {
            output.err(&format!(
                "{LABEL}: repository root does not exist: {}\n",
                root.display()
            ));
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
                output.out(&format!("{LABEL}: self-test {passed}/13 passed.\n"));
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
            return if error.starts_with("source-pdf-registry-currentness: cannot read ") {
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
        output.out(&format!(
            "{LABEL}: 22 tracked PDFs, registry rows, derived keys, paths, signatures, and producer seams are current.\n"
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
        assert_eq!(
            output.text,
            "source-pdf-registry-currentness: self-test 13/13 passed.\n"
        );
        assert!(
            !root
                .join(format!(
                    "generated/.source-pdf-registry-currentness-test.{}",
                    std::process::id()
                ))
                .exists()
        );
    }

    #[test]
    fn the_real_git_indexed_registry_and_report_hold() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--report".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("\"pdfs\":22"));
        assert!(output.text.contains("\"rows\":22"));
        assert!(output.text.contains("\"git_index_is_authoritative\":true"));
    }
}
