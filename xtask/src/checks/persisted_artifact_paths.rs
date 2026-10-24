//! The persisted-artifact path contract (`ARTIFACT-PATH-PORTABILITY`), native since `RUSTDOC-AND-XTASK.14.g`: no
//! canonical artifact under `generated/` stores an absolute path to something the repository owns, so a moved or
//! re-cloned repository keeps every artifact valid. Every `"…_path"`/`"…_root"`/`"working_directory"` string member of
//! every persisted JSON artifact is read; an absolute value passes only where its artifact labels it an external
//! input (a SourceIR's requested and canonical paths, a markdown source's promoted path, a Docling sidecar's input,
//! an EvidenceIR's source path). The producers' persistence seams must still be in place, and a legacy absolute root
//! can be migrated to repository-relative values, labelling each source's origin as it goes.
//!
//! `PROJECT-DATA-LOCALITY` runs its self-test and its check in-process (`xtask/src/checks/project_data_locality.rs`).
//! `scripts/check_persisted_artifact_paths.pl`, the tool the records cite, is a shim that keeps its argument loop, its
//! output layers (so a refusal exits with the status Perl's `die` leaves) and the root resolution, and passes the
//! native tool a normalized command line: `--mode check|report|self-test|migrate --root DIR [--legacy DIR]
//! [--external DIR]... [--execute]`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::pyjson::{self, Py};

const LABEL: &str = "persisted-artifact-paths";
/// The assertions the self-test runs: twelve scanner cases, three seam cases, the migration rewrite and two origin
/// insertions. Declared apart from the suite so a dropped assertion fails it (`PRODUCTION-GRAPH-CENSUS-PIN.3`).
const EXPECTED_SELF_TEST_ASSERTIONS: usize = 18;
/// The producers' persistence seams, by file.
const PRODUCER_CONTRACT: &[(&str, &[&str])] = &[
    (
        "crates/specforge/src/persisted_path.rs",
        &["pub(crate) fn normalize_for_storage("],
    ),
    (
        "crates/specforge/src/ir/source.rs",
        &[
            "persisted.verify_canonical_proof()?;",
            "serde_json::to_string_pretty(&persisted)?",
            "source_ir.runtime_clone()",
            "persisted.artifact_layout.normalize_for_storage()?;",
            "resolve_reference(&runtime.source.canonical_path, source_origin)?",
        ],
    ),
    (
        "crates/specforge/src/ir/evidence.rs",
        &[
            "let persisted = self.persisted_clone()?;",
            "persisted.verify_canonical_proof()?;",
            "to_string_pretty(&persisted)?",
            "evidence_ir.runtime_clone()",
            "persisted.artifact_layout.normalize_for_storage()?;",
            "anchor.source_path = resolve_reference(&anchor.source_path, source_path_origin)?;",
        ],
    ),
    (
        "crates/specforge/src/ir/figure_region.rs",
        &[
            "serialize_with = \"serialize_repository_image_path\"",
            "normalize_for_storage(path, PersistedPathOrigin::RepositoryOwned)",
            "resolve_reference(&path, PersistedPathOrigin::RepositoryOwned)",
        ],
    ),
    (
        "crates/specforge/src/ir/semantic.rs",
        &[
            "let persisted = self.persisted_clone()?;",
            "persisted.verify_canonical_proof()?;",
            "to_string_pretty(&persisted)?",
            "semantic_ir.runtime_clone()",
            "persisted.artifact_layout.normalize_for_storage()?;",
        ],
    ),
    (
        "crates/specforge/src/ir/intent.rs",
        &[
            "let persisted = self.persisted_clone()?;",
            "persisted.verify_canonical_proof()?;",
            "to_string_pretty(&persisted)?",
            "intent_ir.runtime_clone()",
            "persisted.artifact_layout.normalize_for_storage()?;",
        ],
    ),
    (
        "crates/specforge/src/ir/adapters.rs",
        &[
            "let persisted = self.persisted_clone()?;",
            "persisted.verify_canonical_proof()?;",
            "to_string_pretty(&persisted)?",
            "artifact.runtime_clone()",
            "persisted.artifact_layout.normalize_for_storage()?;",
        ],
    ),
    (
        "crates/specforge/src/ir/prior_memory.rs",
        &[
            "to_string_pretty(&self.persisted_clone()?)",
            "normalize_for_storage(&source.artifact_path, PersistedPathOrigin::RepositoryOwned)",
        ],
    ),
    (
        "crates/specforge/src/commands/learn_priors.rs",
        &["let pretty_json = corpus_memory.to_pretty_json()?;"],
    ),
    (
        "crates/specforge/src/commands/project_validation.rs",
        &[
            "resolve_existing(artifact, PersistedPathOrigin::RepositoryOwned)?",
            "artifact_path: repo_relative_display(&snapshot.artifact_path, repo_root)",
            "path: repo_relative_display(&input.path, repo_root)",
            "working_directory: \".\".to_string()",
        ],
    ),
    (
        "crates/specforge/src/commands/recover_register_bits.rs",
        &["resolve_existing(&args.evidence_ir, PersistedPathOrigin::RepositoryOwned)?"],
    ),
    (
        "crates/specforge/src/ir/source/docling_backend.rs",
        &[
            "normalize_backend_metadata_paths(",
            "normalize_for_storage(source_path, source_path_origin)?",
            "normalize_for_storage(promoted_markdown_path, PersistedPathOrigin::RepositoryOwned)?",
        ],
    ),
];
/// The seams `persisted_path.rs` must expose as callable functions.
const CALLABLE_SEAMS: [&str; 2] = ["resolve_existing", "resolve_repository_output"];

/// The command line the shim forwards.
struct Command {
    mode: String,
    root: PathBuf,
    legacy: Option<String>,
    external: Vec<String>,
    execute: bool,
}

fn parse(repository: &Path, arguments: &[String]) -> Option<Command> {
    let mut command = Command {
        mode: "check".to_string(),
        root: repository.to_path_buf(),
        legacy: None,
        external: Vec::new(),
        execute: false,
    };
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--mode" => command.mode = arguments.next()?.clone(),
            "--root" => command.root = PathBuf::from(arguments.next()?),
            "--legacy" => command.legacy = Some(arguments.next()?.clone()),
            "--external" => command.external.push(arguments.next()?.clone()),
            "--execute" => command.execute = true,
            _ => return None,
        }
    }
    matches!(
        command.mode.as_str(),
        "check" | "report" | "self-test" | "migrate"
    )
    .then_some(command)
}

/// The tool, as the shim forwards it.
pub fn run(repository: &Path, arguments: &[String]) -> i32 {
    let Some(command) = parse(repository, arguments) else {
        eprintln!(
            "Usage: cargo xtask check persisted-artifact-paths [--mode check|report|self-test|migrate] [--root DIR] \
             [--legacy DIR] [--external DIR]... [--execute]"
        );
        return 2;
    };
    match command.mode.as_str() {
        "self-test" => match self_test() {
            Ok(passed) => {
                println!("{LABEL}: self-test {passed}/{EXPECTED_SELF_TEST_ASSERTIONS} passed.");
                0
            }
            Err(failure) => {
                eprintln!("{failure}");
                255
            }
        },
        "migrate" => migrate_command(&command),
        mode => {
            let mut errors = Vec::new();
            validate_producer_contract(&command.root, &mut errors);
            let artifacts = persisted_json_artifacts(&command.root, &mut errors);
            let stats = scan_artifacts(&artifacts, &mut errors);
            if !errors.is_empty() {
                for error in &errors {
                    eprintln!("{LABEL}: {error}");
                }
                return 1;
            }
            if mode == "report" {
                println!(
                    "{{\"absolute_repository_paths\":0,\"artifact_files\":{},\"authorized_external_paths\":{},\
                     \"path_values\":{},\"producer_files\":12,\"schema_version\":1}}",
                    stats.artifact_files, stats.authorized_external_paths, stats.path_values
                );
            } else {
                println!(
                    "{LABEL}: PASS — {} persisted JSON artifacts, {} path values, {} authorized external absolute paths, \
                     zero repository-owned absolute paths",
                    stats.artifact_files, stats.path_values, stats.authorized_external_paths
                );
            }
            0
        }
    }
}

/// `PROJECT-DATA-LOCALITY`'s use of the tool: its self-test, then its check, each reporting a failure on standard
/// error as the tool does; true when both hold.
pub fn locality_verdicts(root: &Path) -> (bool, bool) {
    let self_test_holds = match self_test() {
        Ok(_) => true,
        Err(failure) => {
            eprintln!("{failure}");
            false
        }
    };
    let mut errors = Vec::new();
    validate_producer_contract(root, &mut errors);
    let artifacts = persisted_json_artifacts(root, &mut errors);
    scan_artifacts(&artifacts, &mut errors);
    for error in &errors {
        eprintln!("{LABEL}: {error}");
    }
    (self_test_holds, errors.is_empty())
}

// Reading files as Perl read them.

/// `$!` as Perl prints it: the system message without Rust's `(os error N)`.
fn perl_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn read_raw(path: &Path, label: &str, errors: &mut Vec<String>) -> Option<Vec<u8>> {
    match fs::read(path) {
        Ok(bytes) => Some(bytes),
        Err(error) => {
            errors.push(format!("cannot read '{label}': {}", perl_error(&error)));
            None
        }
    }
}

fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    !needle.is_empty()
        && haystack
            .windows(needle.len())
            .any(|window| window == needle)
}

/// Perl's `\s` on a byte string.
fn is_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

fn is_word(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn skip_spaces(bytes: &[u8], mut at: usize) -> usize {
    while at < bytes.len() && is_space(bytes[at]) {
        at += 1;
    }
    at
}

// The producer contract.

/// `\bpub(?:\s*\(\s*crate\s*\))?\s+fn\s+NAME\s*\(` anywhere in the text.
fn has_callable_rust_function(raw: &[u8], function: &str) -> bool {
    let after_fn = |at: usize| -> bool {
        // `\s+fn\s+NAME\s*\(` from `at`.
        let spaced = skip_spaces(raw, at);
        if spaced == at || !raw[spaced..].starts_with(b"fn") {
            return false;
        }
        let name_at = skip_spaces(raw, spaced + 2);
        if name_at == spaced + 2 || !raw[name_at..].starts_with(function.as_bytes()) {
            return false;
        }
        let paren = skip_spaces(raw, name_at + function.len());
        raw.get(paren) == Some(&b'(')
    };
    (0..raw.len()).any(|start| {
        if !raw[start..].starts_with(b"pub") || (start > 0 && is_word(raw[start - 1])) {
            return false;
        }
        let tail = start + 3;
        let restricted = {
            let open = skip_spaces(raw, tail);
            raw.get(open) == Some(&b'(') && {
                let word = skip_spaces(raw, open + 1);
                raw[word..].starts_with(b"crate") && {
                    let close = skip_spaces(raw, word + 5);
                    raw.get(close) == Some(&b')') && after_fn(close + 1)
                }
            }
        };
        restricted || after_fn(tail)
    })
}

fn validate_producer_contract(base: &Path, errors: &mut Vec<String>) {
    let mut contract: Vec<&(&str, &[&str])> = PRODUCER_CONTRACT.iter().collect();
    contract.sort_by_key(|(relative, _)| *relative);
    for (relative, needles) in contract {
        let path = base.join(relative);
        let is_symlink =
            fs::symlink_metadata(&path).is_ok_and(|metadata| metadata.file_type().is_symlink());
        if !path.is_file() || is_symlink {
            errors.push(format!(
                "producer contract file is missing, unsafe, or a symlink: {relative}"
            ));
            continue;
        }
        let Some(raw) = read_raw(&path, relative, errors) else {
            continue;
        };
        for needle in *needles {
            if !contains(&raw, needle.as_bytes()) {
                errors.push(format!(
                    "producer contract '{relative}' is missing required seam: {needle}"
                ));
            }
        }
        if *relative == "crates/specforge/src/persisted_path.rs" {
            for function in CALLABLE_SEAMS {
                if !has_callable_rust_function(&raw, function) {
                    errors.push(format!(
                        "producer contract '{relative}' is missing callable seam: {function}"
                    ));
                }
            }
        }
    }
}

// The artifacts.

/// One artifact: its repository-relative path, its path on disk, and its kind.
struct Artifact {
    relative: String,
    path: PathBuf,
    kind: &'static str,
}

/// The kind of a canonical artifact by its repository-relative path, or `None` for any other file.
fn artifact_kind(relative: &str) -> Option<&'static str> {
    let parts: Vec<&str> = relative.split('/').collect();
    let component = |part: &str| !part.is_empty();
    match parts.as_slice() {
        ["generated", "source_ir", key, "source_ir.json"] if component(key) => Some("source"),
        ["generated", "source_ir", key, "normalized", file]
            if component(key)
                && file.ends_with(".meta.json")
                && file.len() > ".meta.json".len() =>
        {
            Some("source_metadata")
        }
        ["generated", "evidence_ir", key, "evidence_ir.json"] if component(key) => Some("evidence"),
        ["generated", "semantic_ir", key, "semantic_ir.json"] if component(key) => Some("semantic"),
        ["generated", "intent_ir", key, "intent_ir.json"] if component(key) => Some("intent"),
        ["generated", "adapters", "isf", key, "adapter.json"] if component(key) => Some("adapter"),
        ["generated", "prior_memory", "corpus_memory.json"] => Some("prior_memory"),
        _ => None,
    }
}

/// Every file under `generated/` (symbolic links not followed into), as `File::Find` with `no_chdir` visits them,
/// with its repository-relative path; a directory (or a link to one) is skipped.
fn generated_files(base: &Path, errors: &mut Vec<String>) -> Option<Vec<(String, PathBuf)>> {
    let generated = base.join("generated");
    let Ok(metadata) = fs::symlink_metadata(&generated) else {
        return Some(Vec::new());
    };
    if metadata.file_type().is_symlink() || !metadata.is_dir() {
        errors.push("generated root is not a real in-repository directory".to_string());
        return None;
    }
    fn walk(directory: &Path, relative: &str, out: &mut Vec<(String, PathBuf)>) {
        let Ok(entries) = fs::read_dir(directory) else {
            return;
        };
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            let path = entry.path();
            let relative = format!("{relative}/{name}");
            match entry.file_type() {
                Ok(kind) if kind.is_dir() => walk(&path, &relative, out),
                _ if path.is_dir() => {}
                _ => out.push((relative, path)),
            }
        }
    }
    let mut out = Vec::new();
    walk(&generated, "generated", &mut out);
    Some(out)
}

fn is_real_file(path: &Path) -> bool {
    !fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_symlink())
        && path.is_file()
}

fn canonical_artifacts(base: &Path, errors: &mut Vec<String>) -> Vec<Artifact> {
    let Some(files) = generated_files(base, errors) else {
        return Vec::new();
    };
    let mut artifacts = Vec::new();
    for (relative, path) in files {
        let Some(kind) = artifact_kind(&relative) else {
            continue;
        };
        if !is_real_file(&path) {
            errors.push(format!("canonical artifact is not a real file: {relative}"));
            continue;
        }
        artifacts.push(Artifact {
            relative,
            path,
            kind,
        });
    }
    artifacts.sort_by(|left, right| left.relative.cmp(&right.relative));
    artifacts
}

fn persisted_json_artifacts(base: &Path, errors: &mut Vec<String>) -> Vec<Artifact> {
    let Some(files) = generated_files(base, errors) else {
        return Vec::new();
    };
    let mut artifacts = Vec::new();
    for (relative, path) in files {
        if !relative.ends_with(".json") {
            continue;
        }
        if !is_real_file(&path) {
            errors.push(format!(
                "persisted JSON artifact is not a real file: {relative}"
            ));
            continue;
        }
        let kind = artifact_kind(&relative).unwrap_or("other");
        artifacts.push(Artifact {
            relative,
            path,
            kind,
        });
    }
    artifacts.sort_by(|left, right| left.relative.cmp(&right.relative));
    artifacts
}

// The scan.

/// Every match of `"([A-Za-z][A-Za-z0-9_]*)"\s*:\s*"([^"]*)"` in `text`, left to right, as Perl's `/g` finds them.
fn string_members(text: &[u8]) -> Vec<(&[u8], &[u8])> {
    let mut members = Vec::new();
    let mut at = 0;
    while at < text.len() {
        match member_at(text, at) {
            Some((key, value, end)) => {
                members.push((key, value));
                at = end;
            }
            None => at += 1,
        }
    }
    members
}

fn member_at(text: &[u8], start: usize) -> Option<(&[u8], &[u8], usize)> {
    if text.get(start) != Some(&b'"') || !text.get(start + 1)?.is_ascii_alphabetic() {
        return None;
    }
    let mut at = start + 2;
    while at < text.len() && is_word(text[at]) {
        at += 1;
    }
    let key = &text[start + 1..at];
    if text.get(at) != Some(&b'"') {
        return None;
    }
    at = skip_spaces(text, at + 1);
    if text.get(at) != Some(&b':') {
        return None;
    }
    at = skip_spaces(text, at + 1);
    if text.get(at) != Some(&b'"') {
        return None;
    }
    let value_start = at + 1;
    let value_end = value_start + text[value_start..].iter().position(|byte| *byte == b'"')?;
    Some((key, &text[value_start..value_end], value_end + 1))
}

/// The first match of `"FIELD"\s*:\s*"(one of the words)"` in `text`.
fn first_word_field(text: &[u8], field: &str, word: impl Fn(&[u8]) -> bool) -> Option<String> {
    let needle = format!("\"{field}\"");
    let needle = needle.as_bytes();
    (0..text.len()).find_map(|start| {
        if !text[start..].starts_with(needle) {
            return None;
        }
        let mut at = skip_spaces(text, start + needle.len());
        if text.get(at) != Some(&b':') {
            return None;
        }
        at = skip_spaces(text, at + 1);
        if text.get(at) != Some(&b'"') {
            return None;
        }
        let value_start = at + 1;
        let value_end = value_start + text[value_start..].iter().position(|byte| *byte == b'"')?;
        let value = &text[value_start..value_end];
        word(value).then(|| String::from_utf8_lossy(value).into_owned())
    })
}

fn is_origin(value: &[u8]) -> bool {
    value == b"repository_owned" || value == b"external_input"
}

fn is_source_kind(value: &[u8]) -> bool {
    !value.is_empty()
        && value
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || *byte == b'_')
}

/// What an artifact says about the origin of its paths.
#[derive(Default)]
struct Metadata {
    path_origin: Option<String>,
    source_path_origin: Option<String>,
    source_kind: Option<String>,
}

/// The labels, read line by line until all three are found, each from the first line that carries it.
fn artifact_metadata(raw: &[u8]) -> Metadata {
    let mut meta = Metadata::default();
    for line in raw.split_inclusive(|byte| *byte == b'\n') {
        // Each label's key ends `_origin"` or `_kind"`; a line naming neither cannot set one.
        if !contains(line, b"_origin\"") && !contains(line, b"_kind\"") {
            continue;
        }
        if meta.path_origin.is_none() {
            meta.path_origin = first_word_field(line, "path_origin", is_origin);
        }
        if meta.source_path_origin.is_none() {
            meta.source_path_origin = first_word_field(line, "source_path_origin", is_origin);
        }
        if meta.source_kind.is_none() {
            meta.source_kind = first_word_field(line, "source_kind", is_source_kind);
        }
        if meta.path_origin.is_some()
            && meta.source_path_origin.is_some()
            && meta.source_kind.is_some()
        {
            break;
        }
    }
    meta
}

/// The self-test's reading: each label's first match anywhere in the text.
fn metadata_from_text(raw: &[u8]) -> Metadata {
    Metadata {
        path_origin: first_word_field(raw, "path_origin", is_origin),
        source_path_origin: first_word_field(raw, "source_path_origin", is_origin),
        source_kind: first_word_field(raw, "source_kind", is_source_kind),
    }
}

fn is_path_key(key: &[u8]) -> bool {
    key == b"working_directory"
        || [b"path".as_slice(), b"root".as_slice()]
            .iter()
            .any(|suffix| {
                key == *suffix
                    || (key.ends_with(suffix) && key[key.len() - suffix.len() - 1] == b'_')
            })
}

fn is_absolute_path(value: &[u32]) -> bool {
    let at = |index: usize| value.get(index).copied().and_then(char::from_u32);
    at(0) == Some('/')
        || (at(0) == Some('\\') && at(1) == Some('\\'))
        || (at(0).is_some_and(|first| first.is_ascii_alphabetic())
            && at(1) == Some(':')
            && matches!(at(2), Some('\\' | '/')))
}

fn is_authorized_external(kind: &str, key: &[u8], meta: &Metadata) -> bool {
    let path_origin = meta.path_origin.as_deref().unwrap_or("");
    if kind == "source" && path_origin == "external_input" {
        if key == b"requested_path" || key == b"canonical_path" {
            return true;
        }
        if key == b"promoted_markdown_path" && meta.source_kind.as_deref() == Some("markdown") {
            return true;
        }
    }
    (kind == "source_metadata" && key == b"input_path" && path_origin == "external_input")
        || (kind == "evidence"
            && key == b"source_path"
            && meta.source_path_origin.as_deref() == Some("external_input"))
}

/// `JSON::PP->new->allow_nonref->decode(qq{"$encoded"})` on raw bytes: each byte a character, escapes decoded.
fn decode_json_string(encoded: &[u8]) -> Option<Vec<u32>> {
    // Without an escape or a control character the literal is its own value, one character per byte.
    if encoded.iter().all(|byte| *byte != b'\\' && *byte >= 0x20) {
        return Some(encoded.iter().map(|byte| u32::from(*byte)).collect());
    }
    let mut text = String::with_capacity(encoded.len() + 2);
    text.push('"');
    text.extend(encoded.iter().map(|byte| char::from(*byte)));
    text.push('"');
    match pyjson::loads_strict(&text) {
        Ok(Py::Str(value)) => Some(value),
        _ => None,
    }
}

/// The counts the report publishes.
#[derive(Default)]
struct Stats {
    artifact_files: usize,
    path_values: usize,
    authorized_external_paths: usize,
}

fn scan_artifacts(artifacts: &[Artifact], errors: &mut Vec<String>) -> Stats {
    let mut stats = Stats {
        artifact_files: artifacts.len(),
        ..Stats::default()
    };
    let mut absolute_repository_paths = 0usize;
    for artifact in artifacts {
        let raw = match fs::read(&artifact.path) {
            Ok(raw) => raw,
            Err(error) => {
                errors.push(format!(
                    "cannot read canonical artifact '{}': {}",
                    artifact.relative,
                    perl_error(&error)
                ));
                continue;
            }
        };
        let meta = artifact_metadata(&raw);
        for line in raw.split_inclusive(|byte| *byte == b'\n') {
            for (key, encoded) in string_members(line) {
                if !is_path_key(key) {
                    continue;
                }
                let Some(value) = decode_json_string(encoded) else {
                    errors.push(format!(
                        "cannot decode {} value as a JSON string",
                        artifact.relative
                    ));
                    continue;
                };
                stats.path_values += 1;
                if !is_absolute_path(&value) {
                    continue;
                }
                if is_authorized_external(artifact.kind, key, &meta) {
                    stats.authorized_external_paths += 1;
                    continue;
                }
                absolute_repository_paths += 1;
                if absolute_repository_paths <= 20 {
                    errors.push(format!(
                        "canonical artifact '{}' stores absolute {}: {}",
                        artifact.relative,
                        String::from_utf8_lossy(key),
                        pyjson::to_text(&value)
                    ));
                }
            }
        }
    }
    if absolute_repository_paths > 20 {
        errors.push(format!(
            "{absolute_repository_paths} repository-owned absolute path value(s) remain across canonical artifacts"
        ));
    }
    stats
}

/// The self-test's scan of one text: the label and each absolute, unauthorized path key.
fn scan_text(kind: &str, raw: &[u8], meta: &Metadata, label: &str) -> Vec<String> {
    let mut errors = Vec::new();
    for (key, encoded) in string_members(raw) {
        if !is_path_key(key) {
            continue;
        }
        let Some(value) = decode_json_string(encoded) else {
            errors.push(format!("cannot decode {label} value as a JSON string"));
            continue;
        };
        if !is_absolute_path(&value) || is_authorized_external(kind, key, meta) {
            continue;
        }
        errors.push(format!(
            "{label} stores absolute {}",
            String::from_utf8_lossy(key)
        ));
    }
    errors
}

// The migration.

/// The counts a migration reports.
#[derive(Default)]
struct Migration {
    artifact_files: usize,
    changed_files: usize,
    changed_values: usize,
    origin_labels_added: usize,
    net_removed_bytes: isize,
}

/// Where a source's paths come from, by document key: the source's own origin, and its promoted markdown's.
struct Origins {
    source: String,
    promoted: String,
}

fn count_literal(text: &[u8], literal: &[u8]) -> usize {
    if literal.is_empty() {
        return 0;
    }
    let mut count = 0;
    let mut at = 0;
    while let Some(found) = text[at..]
        .windows(literal.len())
        .position(|window| window == literal)
    {
        count += 1;
        at += found + literal.len();
    }
    count
}

fn replace_all(text: &[u8], literal: &[u8], replacement: &[u8]) -> (Vec<u8>, usize) {
    let mut out = Vec::with_capacity(text.len());
    let mut count = 0;
    let mut at = 0;
    while let Some(found) = text[at..]
        .windows(literal.len())
        .position(|window| window == literal)
    {
        out.extend_from_slice(&text[at..at + found]);
        out.extend_from_slice(replacement);
        at += found + literal.len();
        count += 1;
    }
    out.extend_from_slice(&text[at..]);
    (out, count)
}

fn path_below(path: &str, root: &str) -> bool {
    path == root || path.starts_with(&format!("{root}/"))
}

/// `[\x00-\x1f"\\]` anywhere.
fn is_unsafe_root(root: &str) -> bool {
    root.bytes()
        .any(|byte| byte < 0x20 || byte == b'"' || byte == b'\\')
}

fn document_key(relative: &str, stage: &str, file: &str) -> Option<String> {
    let parts: Vec<&str> = relative.split('/').collect();
    match parts.as_slice() {
        ["generated", directory, key, name]
            if *directory == stage && *name == file && !key.is_empty() =>
        {
            Some((*key).to_string())
        }
        _ => None,
    }
}

/// `"FIELD"\s*:\s*"([^"]*)"` must match exactly once; its value decoded.
fn extract_json_string_field(
    raw: &[u8],
    field: &str,
    label: &str,
    errors: &mut Vec<String>,
) -> Option<String> {
    let values = field_values_anywhere(raw, field);
    if values.len() != 1 {
        errors.push(format!(
            "expected exactly one {field} in '{label}', found {}",
            values.len()
        ));
        return None;
    }
    match decode_json_string(values[0]) {
        Some(value) => Some(pyjson::to_text(&value)),
        None => {
            errors.push(format!("cannot decode {label} {field} as a JSON string"));
            None
        }
    }
}

/// Every match of `"FIELD"\s*:\s*"([^"]*)"`, left to right.
fn field_values_anywhere<'a>(raw: &'a [u8], field: &str) -> Vec<&'a [u8]> {
    let needle = format!("\"{field}\"");
    let needle = needle.as_bytes();
    let mut values = Vec::new();
    let mut at = 0;
    while at < raw.len() {
        if raw[at..].starts_with(needle) {
            let mut cursor = skip_spaces(raw, at + needle.len());
            if raw.get(cursor) == Some(&b':') {
                cursor = skip_spaces(raw, cursor + 1);
                if raw.get(cursor) == Some(&b'"')
                    && let Some(length) = raw[cursor + 1..].iter().position(|byte| *byte == b'"')
                {
                    values.push(&raw[cursor + 1..cursor + 1 + length]);
                    at = cursor + 2 + length;
                    continue;
                }
            }
        }
        at += 1;
    }
    values
}

fn migration_source_origins(
    base: &Path,
    artifacts: &[Artifact],
    legacy: &str,
    external_roots: &[String],
    errors: &mut Vec<String>,
) -> BTreeMap<String, Origins> {
    let base = base.to_string_lossy();
    let mut origins = BTreeMap::new();
    for artifact in artifacts
        .iter()
        .filter(|artifact| artifact.kind == "source")
    {
        let Some(key) = document_key(&artifact.relative, "source_ir", "source_ir.json") else {
            continue;
        };
        let Some(raw) = read_raw(&artifact.path, &artifact.relative, errors) else {
            continue;
        };
        let canonical =
            extract_json_string_field(&raw, "canonical_path", &artifact.relative, errors);
        let source_kind =
            extract_json_string_field(&raw, "source_kind", &artifact.relative, errors);
        let (Some(canonical), Some(source_kind)) = (canonical, source_kind) else {
            continue;
        };
        let origin = match first_word_field(&raw, "path_origin", is_origin) {
            Some(declared) => declared,
            None => {
                let absolute = is_absolute_path(&pyjson::py_str(&canonical));
                if !absolute || path_below(&canonical, legacy) || path_below(&canonical, &base) {
                    "repository_owned".to_string()
                } else if external_roots
                    .iter()
                    .filter(|root| path_below(&canonical, root))
                    .count()
                    == 1
                {
                    "external_input".to_string()
                } else {
                    errors.push(format!(
                        "unlabeled absolute source in '{}' is not below exactly one authorized external root: {canonical}",
                        artifact.relative
                    ));
                    continue;
                }
            }
        };
        let promoted = if source_kind == "markdown" {
            origin.clone()
        } else {
            "repository_owned".to_string()
        };
        origins.insert(
            key,
            Origins {
                source: origin,
                promoted,
            },
        );
    }
    origins
}

/// `s{^(\s*"FIELD"\s*:\s*"[^"]*",\n)}{$1 . INSERT}emx`, the first match only: where the line naming the field ends.
fn insertion_point(raw: &[u8], field: &str) -> Option<usize> {
    let needle = format!("\"{field}\"");
    let needle = needle.as_bytes();
    let mut at = 0;
    while at < raw.len() {
        if raw[at..].starts_with(needle) {
            // `^\s*` before it: a line start within the run of white space that precedes it, or the start.
            let run_start = raw[..at]
                .iter()
                .rposition(|byte| !is_space(*byte))
                .map_or(0, |last| last + 1);
            let starts_a_line = run_start == 0 || raw[run_start..at].contains(&b'\n');
            if starts_a_line {
                let mut cursor = skip_spaces(raw, at + needle.len());
                if raw.get(cursor) == Some(&b':') {
                    cursor = skip_spaces(raw, cursor + 1);
                    if raw.get(cursor) == Some(&b'"')
                        && let Some(length) =
                            raw[cursor + 1..].iter().position(|byte| *byte == b'"')
                    {
                        let end = cursor + 2 + length;
                        if raw.get(end) == Some(&b',') && raw.get(end + 1) == Some(&b'\n') {
                            return Some(end + 2);
                        }
                    }
                }
            }
        }
        at += 1;
    }
    None
}

/// Where an artifact kind records its path origin: the label, the stage directory and file its key comes from, the
/// member the label is inserted after, the insertion's indentation, and whether it is the promoted markdown's origin.
struct OriginLabel {
    key: &'static str,
    stage: &'static str,
    file: &'static str,
    anchor: &'static str,
    indent: &'static str,
    promoted: bool,
}

/// Label a source's or an EvidenceIR's path origin: the number of labels added, or `None` with the reason recorded.
fn add_origin_label(
    relative: &str,
    kind: &str,
    raw: &mut Vec<u8>,
    origins: &BTreeMap<String, Origins>,
    errors: &mut Vec<String>,
) -> Option<usize> {
    let label = match kind {
        "source" => OriginLabel {
            key: "path_origin",
            stage: "source_ir",
            file: "source_ir.json",
            anchor: "canonical_path",
            indent: "    ",
            promoted: false,
        },
        "evidence" => OriginLabel {
            key: "source_path_origin",
            stage: "evidence_ir",
            file: "evidence_ir.json",
            anchor: "source_ir_path",
            indent: "  ",
            promoted: true,
        },
        _ => return Some(0),
    };
    let (label_key, stage, file, anchor, indent) = (
        label.key,
        label.stage,
        label.file,
        label.anchor,
        label.indent,
    );
    let which = |origins: &Origins| -> String {
        if label.promoted {
            origins.promoted.clone()
        } else {
            origins.source.clone()
        }
    };
    if field_labelled(raw, label_key) {
        return Some(0);
    }
    let key = document_key(relative, stage, file);
    let Some(origin) = key.as_ref().and_then(|key| origins.get(key)).map(which) else {
        errors.push(if kind == "source" {
            format!("cannot determine source origin for '{relative}'")
        } else {
            format!("cannot determine evidence source origin for '{relative}'")
        });
        return None;
    };
    let Some(point) = insertion_point(raw, anchor) else {
        errors.push(if kind == "source" {
            format!("cannot insert source path_origin into '{relative}'")
        } else {
            format!("cannot insert source_path_origin into '{relative}'")
        });
        return None;
    };
    let insert = format!("{indent}\"{label_key}\": \"{origin}\",\n");
    raw.splice(point..point, insert.into_bytes());
    Some(1)
}

/// `/"LABEL"\s*:/` anywhere.
fn field_labelled(raw: &[u8], label: &str) -> bool {
    let needle = format!("\"{label}\"");
    let needle = needle.as_bytes();
    (0..raw.len()).any(|at| {
        raw[at..].starts_with(needle) && raw.get(skip_spaces(raw, at + needle.len())) == Some(&b':')
    })
}

/// Replace a file through a sibling temporary, keeping its permissions, as `atomic_write` did.
fn atomic_write(path: &Path, raw: &[u8], relative: &str, errors: &mut Vec<String>) {
    use std::os::unix::fs::PermissionsExt;
    let directory = path.parent().unwrap_or(Path::new("."));
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.subsec_nanos());
    let temporary = directory.join(format!(
        ".persisted-path-migration.{}{nanos}",
        std::process::id()
    ));
    if let Err(error) = fs::write(&temporary, raw) {
        errors.push(format!(
            "cannot write migration temporary for '{relative}': {}",
            perl_error(&error)
        ));
        let _ = fs::remove_file(&temporary);
        return;
    }
    let replaced = fs::metadata(path)
        .and_then(|metadata| {
            fs::set_permissions(
                &temporary,
                fs::Permissions::from_mode(metadata.permissions().mode() & 0o7777),
            )
        })
        .and_then(|()| fs::rename(&temporary, path));
    if let Err(error) = replaced {
        errors.push(format!(
            "cannot atomically replace '{relative}': {}",
            perl_error(&error)
        ));
        let _ = fs::remove_file(&temporary);
    }
}

fn migrate_legacy_root(
    base: &Path,
    artifacts: &[Artifact],
    legacy: Option<&str>,
    external_roots: &[String],
    execute: bool,
    errors: &mut Vec<String>,
) -> Migration {
    let empty = || Migration {
        artifact_files: artifacts.len(),
        ..Migration::default()
    };
    let Some(legacy) = legacy.filter(|legacy| legacy.starts_with('/')) else {
        errors.push("legacy root must be an absolute path".to_string());
        return empty();
    };
    let legacy = legacy.trim_end_matches('/');
    let base_text = base.to_string_lossy();
    if legacy.is_empty() || legacy == base_text || is_unsafe_root(legacy) {
        errors.push("legacy root is unsafe or equals the current repository".to_string());
        return empty();
    }
    let mut authorized = Vec::new();
    for external in external_roots {
        let external = external.trim_end_matches('/');
        if !external.starts_with('/')
            || external.is_empty()
            || external == base_text
            || external == legacy
            || is_unsafe_root(external)
            || !Path::new(external).is_dir()
        {
            errors.push(format!(
                "external source root is unsafe or unavailable: {external}"
            ));
            continue;
        }
        authorized.push(external.to_string());
    }
    let origins = migration_source_origins(base, artifacts, legacy, &authorized, errors);
    let needle = format!("\"{legacy}/");
    let mut migration = empty();
    for artifact in artifacts {
        let Some(raw) = read_raw(&artifact.path, &artifact.relative, errors) else {
            continue;
        };
        let all = count_literal(&raw, legacy.as_bytes());
        let migratable = count_literal(&raw, needle.as_bytes());
        if all != migratable {
            errors.push(format!(
                "legacy root in '{}' is not exclusively a JSON path-value prefix ({all} total, {migratable} migratable)",
                artifact.relative
            ));
            continue;
        }
        let (mut updated, changed) = replace_all(&raw, needle.as_bytes(), b"\"");
        if changed != migratable || contains(&updated, legacy.as_bytes()) {
            errors.push(format!(
                "legacy-root rewrite was incomplete for '{}'",
                artifact.relative
            ));
            continue;
        }
        let Some(labels) = add_origin_label(
            &artifact.relative,
            artifact.kind,
            &mut updated,
            &origins,
            errors,
        ) else {
            continue;
        };
        if changed == 0 && labels == 0 {
            continue;
        }
        migration.changed_files += 1;
        migration.changed_values += changed;
        migration.origin_labels_added += labels;
        migration.net_removed_bytes += raw.len() as isize - updated.len() as isize;
        if execute {
            atomic_write(&artifact.path, &updated, &artifact.relative, errors);
        }
    }
    migration
}

fn migrate_command(command: &Command) -> i32 {
    let mut errors = Vec::new();
    validate_producer_contract(&command.root, &mut errors);
    let artifacts = canonical_artifacts(&command.root, &mut errors);
    let migration = migrate_legacy_root(
        &command.root,
        &artifacts,
        command.legacy.as_deref(),
        &command.external,
        command.execute,
        &mut errors,
    );
    if !errors.is_empty() {
        for error in &errors {
            eprintln!("{LABEL}: {error}");
        }
        return 1;
    }
    println!(
        "{LABEL}: {} artifact_files={} changed_files={} changed_values={} origin_labels_added={} net_removed_bytes={}",
        if command.execute {
            "migration-executed"
        } else {
            "migration-dry-run"
        },
        migration.artifact_files,
        migration.changed_files,
        migration.changed_values,
        migration.origin_labels_added,
        migration.net_removed_bytes
    );
    0
}

// The self-test.

/// The script's assertions, in its order; the first failure ends it, as Perl's `die` did.
fn self_test() -> Result<usize, String> {
    let fail = |name: &str| format!("{LABEL} self-test '{name}' failed");
    let cases: [(&str, &str, &str, bool); 12] = [
        (
            "relative intent paths",
            "intent",
            r#"{"semantic_ir_path":"generated/semantic_ir/doc/semantic_ir.json","artifact_root":"generated/intent_ir/doc"}"#,
            true,
        ),
        (
            "current absolute path rejected",
            "intent",
            r#"{"semantic_ir_path":"/repo/generated/semantic_ir/doc/semantic_ir.json"}"#,
            false,
        ),
        (
            "retired absolute path rejected",
            "adapter",
            r#"{"intent_ir_path":"/retired/specforge/generated/intent_ir/doc/intent_ir.json"}"#,
            false,
        ),
        (
            "labeled external source accepted",
            "source",
            r#"{"source":{"requested_path":"/inputs/spec.md","canonical_path":"/inputs/spec.md","path_origin":"external_input","source_kind":"markdown"},"promoted_markdown_path":"/inputs/spec.md"}"#,
            true,
        ),
        (
            "unlabeled external source rejected",
            "source",
            r#"{"source":{"requested_path":"/inputs/spec.md","canonical_path":"/inputs/spec.md","source_kind":"markdown"}}"#,
            false,
        ),
        (
            "labeled external evidence accepted",
            "evidence",
            r#"{"source_path_origin":"external_input","source_path":"/inputs/spec.md","source_ir_path":"generated/source_ir/doc/source_ir.json"}"#,
            true,
        ),
        (
            "relative docling metadata accepted",
            "source_metadata",
            r#"{"input_path":"corpus/spec.pdf","path_origin":"repository_owned","promoted_markdown_path":"generated/source_ir/doc/normalized/doc.md"}"#,
            true,
        ),
        (
            "labeled external docling input accepted",
            "source_metadata",
            r#"{"input_path":"/inputs/spec.pdf","path_origin":"external_input","promoted_markdown_path":"generated/source_ir/doc/normalized/doc.md"}"#,
            true,
        ),
        (
            "unlabeled external docling input rejected",
            "source_metadata",
            r#"{"input_path":"/inputs/spec.pdf","promoted_markdown_path":"generated/source_ir/doc/normalized/doc.md"}"#,
            false,
        ),
        (
            "unknown path field fails closed",
            "semantic",
            r#"{"raw_image_path":"/retired/specforge/generated/source_ir/doc/image.png"}"#,
            false,
        ),
        (
            "Windows absolute path rejected",
            "prior_memory",
            r#"{"artifact_path":"C:\\work\\specforge\\generated\\intent_ir\\doc\\intent_ir.json"}"#,
            false,
        ),
        (
            "absolute working directory rejected",
            "other",
            r#"{"working_directory":"/retired/specforge"}"#,
            false,
        ),
    ];
    let mut passed = 0;
    for (name, kind, raw, should_pass) in cases {
        let raw = format!("{raw}\n");
        let errors = scan_text(
            kind,
            raw.as_bytes(),
            &metadata_from_text(raw.as_bytes()),
            name,
        );
        if should_pass != errors.is_empty() {
            return Err(fail(name));
        }
        passed += 1;
    }
    for (name, text, holds) in [
        (
            "crate-visible seam",
            "pub(crate) fn resolve_existing(",
            true,
        ),
        ("public seam", "pub fn resolve_existing (", true),
        ("private seam rejection", "fn resolve_existing(", false),
    ] {
        if has_callable_rust_function(text.as_bytes(), "resolve_existing") != holds {
            return Err(fail(name));
        }
        passed += 1;
    }
    let legacy = "/retired/specforge";
    let raw = r#"{"source_ir_path":"/retired/specforge/generated/source_ir/doc/source_ir.json","artifact_root":"/retired/specforge/generated/evidence_ir/doc"}"#;
    let (updated, changed) = replace_all(raw.as_bytes(), format!("\"{legacy}/").as_bytes(), b"\"");
    if changed != 2 || contains(&updated, legacy.as_bytes()) {
        return Err(fail("migration rewrite"));
    }
    passed += 1;
    let mut origins = BTreeMap::new();
    origins.insert(
        "doc".to_string(),
        Origins {
            source: "external_input".to_string(),
            promoted: "repository_owned".to_string(),
        },
    );
    let mut source = b"{\n  \"source\": {\n    \"canonical_path\": \"/inputs/doc.pdf\",\n    \"source_kind\": \"pdf\"\n  }\n}\n".to_vec();
    let mut source_errors = Vec::new();
    let source_added = add_origin_label(
        "generated/source_ir/doc/source_ir.json",
        "source",
        &mut source,
        &origins,
        &mut source_errors,
    );
    if !source_errors.is_empty()
        || source_added != Some(1)
        || !contains(&source, b"\"path_origin\": \"external_input\"")
    {
        return Err(fail("source origin insertion"));
    }
    passed += 1;
    let mut evidence =
        b"{\n  \"source_ir_path\": \"generated/source_ir/doc/source_ir.json\",\n  \"stage\": \"evidence_ir\"\n}\n".to_vec();
    let mut evidence_errors = Vec::new();
    let evidence_added = add_origin_label(
        "generated/evidence_ir/doc/evidence_ir.json",
        "evidence",
        &mut evidence,
        &origins,
        &mut evidence_errors,
    );
    if !evidence_errors.is_empty()
        || evidence_added != Some(1)
        || !contains(&evidence, b"\"source_path_origin\": \"repository_owned\"")
    {
        return Err(fail("evidence origin insertion"));
    }
    passed += 1;
    if passed != EXPECTED_SELF_TEST_ASSERTIONS {
        return Err(format!(
            "{LABEL}: self-test ran {passed} assertions, declaration expects {EXPECTED_SELF_TEST_ASSERTIONS} — \
             re-derive the declaration beside the suite"
        ));
    }
    Ok(passed)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(name: &str) -> PathBuf {
        let root = crate::repository_root()
            .join(format!(".project-data/tmp/xtask-persisted-paths-{name}"));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("a scratch root");
        root
    }

    #[test]
    fn the_self_test_holds_and_the_scanner_reads_values_as_the_script_did() {
        assert_eq!(self_test(), Ok(EXPECTED_SELF_TEST_ASSERTIONS));
        for (key, path_key) in [
            ("path", true),
            ("root", true),
            ("artifact_root", true),
            ("source_ir_path", true),
            ("working_directory", true),
            ("pathway", false),
            ("xroot", false),
        ] {
            assert_eq!(is_path_key(key.as_bytes()), path_key, "{key}");
        }
        for (value, absolute) in [
            ("/a", true),
            ("\\\\server\\s", true),
            ("C:\\w", true),
            ("c:/w", true),
            ("rel/a", false),
        ] {
            assert_eq!(
                is_absolute_path(&pyjson::py_str(value)),
                absolute,
                "{value}"
            );
        }
        assert_eq!(
            decode_json_string(b"caf\\u00e9"),
            decode_json_string("caf\u{e9}".as_bytes()).map(|_| vec![99, 97, 102, 0xe9])
        );
        assert_eq!(decode_json_string(b"a\\/b"), Some(pyjson::py_str("a/b")));
        assert_eq!(decode_json_string(b"bad\\x"), None);
        let members = string_members(br#"{"note":"a \"path\": \"/x\" b","artifact_root":"/r"}"#);
        let keys: Vec<&[u8]> = members.iter().map(|(key, _)| *key).collect();
        // The value ends at the escaped quote, so the text after it is never read as a key, as Perl reads it.
        assert_eq!(keys, vec![b"note".as_slice(), b"artifact_root"]);
    }

    #[test]
    fn labels_are_read_line_by_line_and_the_messages_are_capped_at_twenty() {
        let meta = artifact_metadata(
            b"{\n\"source_path_origin\": \"external_input\",\n\"path_origin\": \"x\"\n}\n",
        );
        assert_eq!(meta.source_path_origin.as_deref(), Some("external_input"));
        assert_eq!(meta.path_origin, None);
        let root = scratch("cap");
        let artifact = root.join("many.json");
        let body: String = (0..21)
            .map(|index| format!("{{\"p{index}_path\":\"/abs/{index}\"}}\n"))
            .collect();
        fs::write(&artifact, body).expect("an artifact");
        let mut errors = Vec::new();
        let stats = scan_artifacts(
            &[Artifact {
                relative: "generated/other/many.json".to_string(),
                path: artifact,
                kind: "other",
            }],
            &mut errors,
        );
        fs::remove_dir_all(&root).expect("cleanup");
        assert_eq!(stats.path_values, 21);
        assert_eq!(errors.len(), 21);
        assert_eq!(
            errors.last().map(String::as_str),
            Some("21 repository-owned absolute path value(s) remain across canonical artifacts")
        );
    }

    #[test]
    fn a_legacy_root_migrates_to_relative_values_and_labels_each_origin() {
        let root = scratch("migrate");
        let external = root.join("inputs");
        fs::create_dir_all(&external).expect("an external root");
        let source = root.join("generated/source_ir/doc/source_ir.json");
        let intent = root.join("generated/intent_ir/doc/intent_ir.json");
        fs::create_dir_all(source.parent().expect("a parent")).expect("directories");
        fs::create_dir_all(intent.parent().expect("a parent")).expect("directories");
        fs::write(
            &source,
            format!(
                "{{\n  \"source\": {{\n    \"canonical_path\": \"{}/a.pdf\",\n    \"source_kind\": \"pdf\"\n  }}\n}}\n",
                external.display()
            ),
        )
        .expect("a source");
        fs::write(
            &intent,
            "{\"semantic_ir_path\":\"/legacy/specforge/generated/x.json\"}\n",
        )
        .expect("an intent");
        let mut errors = Vec::new();
        let artifacts = canonical_artifacts(&root, &mut errors);
        let external = [external.to_string_lossy().into_owned()];
        let migration = migrate_legacy_root(
            &root,
            &artifacts,
            Some("/legacy/specforge/"),
            &external,
            true,
            &mut errors,
        );
        let migrated_intent = fs::read_to_string(&intent).expect("the migrated intent");
        let migrated_source = fs::read_to_string(&source).expect("the migrated source");
        fs::remove_dir_all(&root).expect("cleanup");
        assert!(errors.is_empty(), "{errors:?}");
        assert_eq!(
            migrated_intent,
            "{\"semantic_ir_path\":\"generated/x.json\"}\n"
        );
        // The input lies inside the repository root, which the origin rule checks before the external roots.
        assert!(migrated_source.contains("\n    \"path_origin\": \"repository_owned\",\n"));
        assert_eq!(
            (
                migration.artifact_files,
                migration.changed_files,
                migration.changed_values,
                migration.origin_labels_added
            ),
            (2, 2, 1, 1)
        );
    }
}
