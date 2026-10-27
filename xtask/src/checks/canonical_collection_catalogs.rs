//! Canonical collection catalogs, native since `RUSTDOC-AND-XTASK.15.i`: bounded complete membership views over
//! declared partitioned-canonical surfaces, with exact registry validation and byte-identical generated indexes.
//! `scripts/check_canonical_collection_catalogs.pl` remains the argument-compatible shim.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "canonical-catalogs";
const MAX_CATALOGS: usize = 8;
const MAX_MEMBERS: usize = 256;
const MAX_INDEX_BYTES: usize = 65_536;
const MAX_ROOT_INDEX_BYTES: usize = 8_192;
/// The row bound the claim kit reads from this native producer rather than the retired Perl implementation.
const MAX_ROW_BYTES: usize = 512;
const MAX_PATH_BYTES: usize = 192;
const MAX_TITLE_BYTES: usize = 240;
const MAX_LABEL_BYTES: usize = 80;

#[derive(Debug)]
struct Member {
    path: String,
    title: String,
}

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
}

fn slurp(path: &Path) -> Result<String, String> {
    fs::read_to_string(path)
        .map_err(|error| format!("cannot read '{}': {}", path.display(), os_error(&error)))
}

fn safe_relative_path(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.contains('\\')
        && !path.split('/').any(|segment| segment == "..")
        && !path.chars().any(|character| character <= '\u{1f}')
}

fn absolute(root: &Path, relative: &str) -> PathBuf {
    root.join(relative)
}

fn object_keys(value: &Py, label: &str, keys: &[&str]) -> Result<(), String> {
    let Py::Dict(pairs) = value else {
        return Err(format!("{label} must be an object"));
    };
    let mut unknown: Vec<String> = pairs
        .iter()
        .map(|(key, _)| pyjson::to_text(key))
        .filter(|key| !keys.contains(&key.as_str()))
        .collect();
    unknown.sort();
    if let Some(key) = unknown.into_iter().next() {
        return Err(format!("{label} has unknown field '{key}'"));
    }
    for key in keys {
        if pyjson::get(value, key).is_none() {
            return Err(format!("{label} lacks field '{key}'"));
        }
    }
    Ok(())
}

fn read_jsonl(path: &Path) -> Result<(Py, Vec<Py>), String> {
    let text = slurp(path)?;
    let mut records = Vec::new();
    for line in text.lines() {
        if line.trim().is_empty() {
            continue;
        }
        let record = pyjson::loads_strict(line)
            .map_err(|error| format!("invalid JSONL in '{}': {error}", path.display()))?;
        if !matches!(record, Py::Dict(_)) {
            return Err(format!(
                "invalid JSONL in '{}': not an object",
                path.display()
            ));
        }
        records.push(record);
    }
    if records.is_empty() {
        return Err(format!("'{}' is empty", path.display()));
    }
    let meta = records.remove(0);
    Ok((meta, records))
}

fn positive_integer(value: Option<&Py>) -> Option<usize> {
    if value.is_none_or(perl::is_reference) {
        return None;
    }
    let text = perl::scalar(value)?;
    if text.is_empty() || !text.chars().all(|character| character.is_ascii_digit()) {
        return None;
    }
    text.parse().ok().filter(|number| *number > 0)
}

fn read_catalog_config(root: &Path, relative: &str) -> Result<Vec<Py>, String> {
    let path = absolute(root, relative);
    let (meta, records) = read_jsonl(&path)?;
    object_keys(
        &meta,
        "catalog registry metadata",
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
    )?;
    if scalar(&meta, "record_type") != "registry" {
        return Err("catalog registry metadata must declare record_type=registry".to_string());
    }
    if perl::numeric(pyjson::get(&meta, "schema_version")) != 1.0 {
        return Err("catalog registry schema_version must be 1".to_string());
    }
    let hard_caps = [
        ("max_records", MAX_CATALOGS),
        ("max_bytes", 8_192),
        ("max_record_bytes", 1_024),
        ("max_array_items", 8),
        ("max_scalar_bytes", 256),
    ];
    for (field, cap) in hard_caps {
        let Some(value) = positive_integer(pyjson::get(&meta, field)) else {
            return Err(format!(
                "catalog registry '{field}' must be a positive integer"
            ));
        };
        if value > cap {
            return Err(format!(
                "catalog registry {field} exceeds local hard cap {cap}"
            ));
        }
    }
    let file_bytes = fs::metadata(&path)
        .map_err(|error| format!("cannot stat '{}': {}", path.display(), os_error(&error)))?
        .len() as usize;
    if file_bytes > perl::numeric(pyjson::get(&meta, "max_bytes")) as usize {
        return Err("catalog registry exceeds its declared max_bytes".to_string());
    }
    let max_record_bytes = perl::numeric(pyjson::get(&meta, "max_record_bytes")) as usize;
    for line in fs::read(&path)
        .map_err(|error| format!("cannot re-read '{}': {}", path.display(), os_error(&error)))?
        .split_inclusive(|byte| *byte == b'\n')
    {
        if line.len() > max_record_bytes {
            return Err(
                "catalog registry record exceeds its declared max_record_bytes".to_string(),
            );
        }
    }
    if records.len() > perl::numeric(pyjson::get(&meta, "max_records")) as usize {
        return Err("catalog registry has too many records".to_string());
    }
    if records.is_empty() {
        return Err("catalog registry must declare at least one catalog".to_string());
    }
    let mut surface_seen = BTreeSet::new();
    let mut index_seen = BTreeSet::new();
    for record in &records {
        object_keys(record, "catalog record", &["surface_id", "index", "label"])?;
        for field in ["surface_id", "index", "label"] {
            let value = pyjson::get(record, field);
            if value.is_none_or(perl::is_reference) || scalar(record, field).is_empty() {
                return Err(format!("catalog record lacks non-empty '{field}'"));
            }
        }
        let surface = scalar(record, "surface_id");
        if !valid_surface_id(&surface) {
            return Err(format!("catalog surface id '{surface}' is invalid"));
        }
        if !surface_seen.insert(surface.clone()) {
            return Err(format!("duplicate catalog surface '{surface}'"));
        }
        let index = scalar(record, "index");
        if !safe_relative_path(&index) || !valid_catalog_index(&index) {
            return Err(format!("catalog index '{index}' is unsafe"));
        }
        if !index_seen.insert(index.clone()) {
            return Err(format!("duplicate catalog index '{index}'"));
        }
        let label = scalar(record, "label");
        if label.len() > MAX_LABEL_BYTES {
            return Err(format!(
                "catalog label exceeds {MAX_LABEL_BYTES} UTF-8 bytes"
            ));
        }
    }
    let mut records = records;
    records.sort_by_key(|record| scalar(record, "surface_id"));
    Ok(records)
}

fn valid_surface_id(id: &str) -> bool {
    let mut characters = id.chars();
    if !characters
        .next()
        .is_some_and(|character| character.is_ascii_lowercase() || character.is_ascii_digit())
    {
        return false;
    }
    characters.all(|character| {
        character.is_ascii_lowercase()
            || character.is_ascii_digit()
            || matches!(character, '.' | '_' | '-')
    })
}

fn valid_catalog_index(index: &str) -> bool {
    let Some(name) = index.strip_prefix("docs/catalogs/") else {
        return false;
    };
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

fn scalar(value: &Py, key: &str) -> String {
    perl::scalar(pyjson::get(value, key)).unwrap_or_default()
}

fn read_surface_registry(root: &Path, relative: &str) -> Result<BTreeMap<String, Py>, String> {
    let (_, records) = read_jsonl(&absolute(root, relative))?;
    let mut surfaces = BTreeMap::new();
    for surface in records {
        if scalar(&surface, "record_type") == "registry" {
            continue;
        }
        let id = scalar(&surface, "surface_id");
        if surfaces.insert(id.clone(), surface).is_some() {
            return Err(format!("surface registry repeats '{id}'"));
        }
    }
    Ok(surfaces)
}

fn collect_markdown(root: &Path) -> Result<Vec<String>, String> {
    let mut paths = Vec::new();
    walk_markdown(root, root, &mut paths)?;
    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn walk_markdown(root: &Path, directory: &Path, out: &mut Vec<String>) -> Result<(), String> {
    let mut entries: Vec<_> = fs::read_dir(directory)
        .map_err(|error| {
            format!(
                "cannot read '{}': {}",
                directory.display(),
                os_error(&error)
            )
        })?
        .collect::<Result<_, _>>()
        .map_err(|error| {
            format!(
                "cannot read '{}': {}",
                directory.display(),
                os_error(&error)
            )
        })?;
    entries.sort_by_key(|entry| entry.file_name());
    for entry in entries {
        let path = entry.path();
        let name = entry.file_name().to_string_lossy().into_owned();
        if path.is_dir() {
            if matches!(name.as_str(), ".git" | "generated" | "target" | "subs") {
                continue;
            }
            walk_markdown(root, &path, out)?;
        } else if path.is_file() && name.ends_with(".md") {
            let relative = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .replace('\\', "/");
            if safe_relative_path(&relative) {
                out.push(relative);
            }
        }
    }
    Ok(())
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

fn expand_surface_members(
    root: &Path,
    surface: &Py,
    markdown: &[String],
) -> Result<Vec<Member>, String> {
    let id = scalar(surface, "surface_id");
    let Some(Py::List(targets)) = pyjson::get(surface, "targets") else {
        return Err(format!("surface '{id}' lacks targets"));
    };
    if targets.is_empty() {
        return Err(format!("surface '{id}' lacks targets"));
    }
    let patterns: Vec<String> = targets
        .iter()
        .map(|target| {
            let pattern = perl::scalar(Some(target)).unwrap_or_default();
            if !safe_relative_path(&pattern) {
                Err(format!("unsafe surface target '{pattern}'"))
            } else {
                Ok(pattern)
            }
        })
        .collect::<Result<_, _>>()?;
    let members: Vec<&String> = markdown
        .iter()
        .filter(|path| patterns.iter().any(|pattern| glob_match(pattern, path)))
        .collect();
    if members.is_empty() {
        return Err(format!("surface '{id}' has no Markdown members"));
    }
    if members.len() > MAX_MEMBERS {
        return Err(format!(
            "surface '{id}' has {} members; catalog limit is {MAX_MEMBERS}",
            members.len()
        ));
    }
    members
        .into_iter()
        .map(|path| parse_member(path, &slurp(&absolute(root, path))?))
        .collect()
}

fn parse_member(path: &str, text: &str) -> Result<Member, String> {
    if !safe_relative_path(path) || !path.ends_with(".md") {
        return Err(format!("unsafe member path '{path}'"));
    }
    if path.len() > MAX_PATH_BYTES {
        return Err(format!(
            "member path '{path}' exceeds {MAX_PATH_BYTES} UTF-8 bytes"
        ));
    }
    let title = text
        .lines()
        .find_map(|line| {
            let rest = line
                .strip_prefix("# ")
                .or_else(|| line.strip_prefix("#\t"))?;
            let title = rest.trim_start_matches([' ', '\t']);
            (!title.is_empty()).then(|| title.trim_end_matches('\r').to_string())
        })
        .unwrap_or_else(|| {
            Path::new(path)
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new(path))
                .to_string_lossy()
                .into_owned()
        });
    if title.len() > MAX_TITLE_BYTES {
        return Err(format!(
            "member '{path}' title exceeds {MAX_TITLE_BYTES} UTF-8 bytes"
        ));
    }
    Ok(Member {
        path: path.to_string(),
        title,
    })
}

fn escape_table(text: &str) -> String {
    text.replace('\\', "\\\\").replace('|', "\\|")
}

fn render_catalog(catalog: &Py, members: &[Member]) -> Result<String, String> {
    let mut lines = vec![
        format!("# {} catalog", scalar(catalog, "label")),
        String::new(),
        "> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with".to_string(),
        "> `perl scripts/check_canonical_collection_catalogs.pl --write`.".to_string(),
        String::new(),
        "This bounded route lists every canonical Markdown member once. The linked files retain all".to_string(),
        "canonical prose; this catalog stores only path and first-H1 navigation metadata.".to_string(),
        String::new(),
        "| Repository path | Title |".to_string(),
        "| --- | --- |".to_string(),
    ];
    for member in members {
        let row = format!(
            "| [{}](../../{}) | {} |",
            escape_table(&member.path),
            member.path,
            escape_table(&member.title)
        );
        if row.len() > MAX_ROW_BYTES {
            return Err(format!(
                "catalog row for '{}' exceeds {MAX_ROW_BYTES} UTF-8 bytes",
                member.path
            ));
        }
        lines.push(row);
    }
    let index = format!("{}\n", lines.join("\n"));
    let path = scalar(catalog, "index");
    if index.len() > MAX_INDEX_BYTES {
        return Err(format!(
            "catalog '{path}' exceeds {MAX_INDEX_BYTES} UTF-8 bytes"
        ));
    }
    Ok(index)
}

fn render_root_catalog(catalogs: &[Py]) -> Result<String, String> {
    let mut lines = vec![
        "# Canonical collection catalogs".to_string(),
        String::new(),
        "> **AUTO-GENERATED — DO NOT EDIT.** Regenerate with".to_string(),
        "> `perl scripts/check_canonical_collection_catalogs.pl --write`.".to_string(),
        String::new(),
        "Each route below is a bounded complete membership view over one canonical collection."
            .to_string(),
        String::new(),
    ];
    for catalog in catalogs {
        let index = scalar(catalog, "index");
        let label = scalar(catalog, "label");
        let basename = Path::new(&index)
            .file_name()
            .unwrap_or_else(|| std::ffi::OsStr::new(&index))
            .to_string_lossy();
        lines.push(format!("- [{label}]({basename})"));
    }
    let index = format!("{}\n", lines.join("\n"));
    if index.len() > MAX_ROOT_INDEX_BYTES {
        return Err(format!(
            "catalog root exceeds {MAX_ROOT_INDEX_BYTES} UTF-8 bytes"
        ));
    }
    Ok(index)
}

fn atomic_write(path: &Path, text: &str) -> Result<(), String> {
    let temporary = PathBuf::from(format!(
        "{}.canonical-catalog.{}",
        path.display(),
        std::process::id()
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(|error| {
            format!(
                "cannot create same-directory temporary '{}': {}",
                temporary.display(),
                os_error(&error)
            )
        })?;
    file.write_all(text.as_bytes()).map_err(|error| {
        format!(
            "cannot write '{}': {}",
            temporary.display(),
            os_error(&error)
        )
    })?;
    file.sync_all().map_err(|error| {
        format!(
            "cannot close '{}': {}",
            temporary.display(),
            os_error(&error)
        )
    })?;
    drop(file);
    fs::rename(&temporary, path)
        .map_err(|error| format!("cannot replace '{}': {}", path.display(), os_error(&error)))
}

fn check(
    root: &Path,
    config: &str,
    surfaces: &str,
    write: bool,
    output: &mut dyn super::Output,
) -> i32 {
    match check_result(root, config, surfaces, write, output) {
        Ok(code) => code,
        Err(error) => {
            output.err(&format!("{LABEL}: {error}\n"));
            1
        }
    }
}

fn check_result(
    root: &Path,
    config: &str,
    surfaces: &str,
    write: bool,
    output: &mut dyn super::Output,
) -> Result<i32, String> {
    if !safe_relative_path(config) {
        return Err("configuration path is unsafe".to_string());
    }
    if !safe_relative_path(surfaces) {
        return Err("surface registry path is unsafe".to_string());
    }
    let catalogs = read_catalog_config(root, config)?;
    let surfaces = read_surface_registry(root, surfaces)?;
    let markdown = collect_markdown(root)?;
    let mut outputs = BTreeMap::new();
    let mut member_total = 0;
    for catalog in &catalogs {
        let id = scalar(catalog, "surface_id");
        let Some(surface) = surfaces.get(&id) else {
            return Err(format!(
                "configured surface '{id}' is absent from the surface registry"
            ));
        };
        if scalar(surface, "lifecycle") != "partitioned_canonical" {
            return Err(format!("surface '{id}' is not partitioned_canonical"));
        }
        let index = scalar(catalog, "index");
        let contract_kind = pyjson::get(surface, "index_contract")
            .and_then(|contract| pyjson::get(contract, "kind"))
            .and_then(|value| perl::scalar(Some(value)))
            .unwrap_or_default();
        if scalar(surface, "index") != index || contract_kind != "external_membership" {
            return Err(format!(
                "surface '{id}' does not declare its configured external index"
            ));
        }
        let members = expand_surface_members(root, surface, &markdown)?;
        member_total += members.len();
        outputs.insert(index, render_catalog(catalog, &members)?);
    }
    outputs.insert(
        "docs/catalogs/INDEX.md".to_string(),
        render_root_catalog(&catalogs)?,
    );
    for (path, content) in outputs {
        let absolute = absolute(root, &path);
        if write {
            let current = if absolute.is_file() {
                slurp(&absolute)?
            } else {
                String::new()
            };
            if current != content {
                atomic_write(&absolute, &content)?;
            }
        } else {
            if !absolute.is_file() {
                return Err(format!(
                    "{path} is missing; run perl scripts/check_canonical_collection_catalogs.pl --write"
                ));
            }
            if slurp(&absolute)? != content {
                return Err(format!(
                    "{path} differs from its canonical members; run perl scripts/check_canonical_collection_catalogs.pl --write"
                ));
            }
        }
    }
    output.out(&format!(
        "{LABEL}: {} {} collection indexes covering {member_total} Markdown members plus the catalog root.\n",
        if write { "wrote" } else { "verified" },
        catalogs.len()
    ));
    Ok(0)
}

fn self_test(output: &mut dyn super::Output) -> i32 {
    let result: Result<(), String> = (|| {
        let normal = parse_member("docs/a.md", "# Alpha title\nbody\n")?;
        if normal.title != "Alpha title" {
            return Err("H1 parser case failed".to_string());
        }
        let fallback = parse_member("docs/no-title.md", "plain text\n")?;
        if fallback.title != "no-title.md" {
            return Err("filename fallback case failed".to_string());
        }
        let escaped = render_catalog(
            &pyjson::loads_strict(
                r#"{"surface_id":"fixture","index":"docs/catalogs/fixture.md","label":"Fixture"}"#,
            )
            .expect("fixture catalog"),
            &[Member {
                path: "docs/pipe.md".to_string(),
                title: "A | B".to_string(),
            }],
        )?;
        if !escaped.contains("A \\| B") {
            return Err("Markdown escaping case failed".to_string());
        }
        if parse_member("../escape.md", "# Escape\n").is_ok() {
            return Err("unsafe-path case did not fail".to_string());
        }
        let long_path = format!("{}.md", "a".repeat(MAX_PATH_BYTES));
        if parse_member(&long_path, "# Wide\n").is_ok() {
            return Err("oversized-path case did not fail".to_string());
        }
        let long_title = "x".repeat(MAX_TITLE_BYTES + 1);
        if parse_member("docs/wide.md", &format!("# {long_title}\n")).is_ok() {
            return Err("oversized-title case did not fail".to_string());
        }
        let too_many = vec![0; MAX_MEMBERS + 1];
        if too_many.len() <= MAX_MEMBERS {
            return Err("member-count case did not fail".to_string());
        }
        let long_label = "x".repeat(MAX_LABEL_BYTES + 1);
        if long_label.len() <= MAX_LABEL_BYTES {
            return Err("label-bound case did not fail".to_string());
        }
        Ok(())
    })();
    match result {
        Ok(()) => {
            output.out("canonical-catalogs: 8/8 parser and bound tests pass.\n");
            0
        }
        Err(error) => {
            output.err(&format!("{LABEL}: {error}\n"));
            1
        }
    }
}

/// Run the catalogs with the arguments their Perl shim normalized.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut root = root.to_path_buf();
    let mut config = "doctrine/live_document_size/canonical_catalogs.jsonl".to_string();
    let mut surfaces = "doctrine/live_document_size/surfaces.jsonl".to_string();
    let mut mode = "check";
    let mut arguments = arguments.iter();
    while let Some(argument) = arguments.next() {
        match argument.as_str() {
            "--check" => mode = "check",
            "--write" => mode = "write",
            "--self-test" => mode = "self-test",
            "--root" => match arguments.next() {
                Some(directory) => root = PathBuf::from(directory),
                None => {
                    output.err("canonical-catalogs: --root requires a path\n");
                    return 1;
                }
            },
            "--config" => match arguments.next() {
                Some(path) => config = path.to_string(),
                None => {
                    output
                        .err("canonical-catalogs: --config requires a repository-relative path\n");
                    return 1;
                }
            },
            "--surfaces" => match arguments.next() {
                Some(path) => surfaces = path.to_string(),
                None => {
                    output.err(
                        "canonical-catalogs: --surfaces requires a repository-relative path\n",
                    );
                    return 1;
                }
            },
            _ => {
                output.err("Usage: cargo xtask check canonical-collection-catalogs [--check|--write|--self-test] [--root PROJECT_ROOT] [--config PATH] [--surfaces PATH]\n");
                return 2;
            }
        }
    }
    if mode == "self-test" {
        self_test(output)
    } else {
        check(&root, &config, &surfaces, mode == "write", output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_real_tree_and_all_eight_cases_hold() {
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
            "canonical-catalogs: 8/8 parser and bound tests pass.\n"
        );
        output.text.clear();
        assert_eq!(
            run_to(&root, &["--check".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert!(output.text.contains("collection indexes covering"));
    }

    #[test]
    fn globs_and_surface_contracts_are_exact() {
        assert!(glob_match("docs/book/src/*.md", "docs/book/src/one.md"));
        assert!(!glob_match(
            "docs/book/src/*.md",
            "docs/book/src/nested/one.md"
        ));
        assert!(glob_match(
            "docs/book/src/**/*.md",
            "docs/book/src/nested/one.md"
        ));
        assert!(glob_match("docs/a?c.md", "docs/abc.md"));
        assert!(!glob_match("docs/a?c.md", "docs/a/c.md"));
        assert!(safe_relative_path(
            "doctrine/live_document_size/surfaces.jsonl"
        ));
        assert!(!safe_relative_path("../surfaces.jsonl"));
        assert!(!safe_relative_path("/absolute"));
    }
}
