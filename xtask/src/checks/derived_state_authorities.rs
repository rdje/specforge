//! Derived-state verified-copy authorities, native since `RUSTDOC-AND-XTASK.15.j`: Rust prerequisite copies
//! agree with the workspace manifest, and FSMGen feedback copies agree with the stage-zero Git-index gitlink.
//! Secondary copy paths and markers come only from the declared data plane, never from executable fallbacks.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::perl;
use crate::pyjson::{self, Py};

const LABEL: &str = "derived-state-authority";

struct Copy {
    path: String,
    content: String,
}

fn os_error(error: &std::io::Error) -> String {
    let text = error.to_string();
    text.split(" (os error").next().unwrap_or(&text).to_string()
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

fn absolute(root: &Path, relative: &str) -> PathBuf {
    root.join(relative)
}

fn slurp(root: &Path, relative: &str, label: &str) -> Result<String, String> {
    if !safe_relative_path(relative) {
        return Err(format!("{LABEL}: unsafe {label} path '{relative}'"));
    }
    let path = absolute(root, relative);
    let metadata = fs::symlink_metadata(&path)
        .map_err(|_| format!("{LABEL}: missing or non-regular {label} '{relative}'"))?;
    if !metadata.is_file() || metadata.file_type().is_symlink() {
        return Err(format!(
            "{LABEL}: missing or non-regular {label} '{relative}'"
        ));
    }
    fs::read_to_string(&path).map_err(|error| {
        format!(
            "{LABEL}: cannot read {label} '{relative}': {}",
            os_error(&error)
        )
    })
}

fn scalar(value: &Py, key: &str) -> String {
    perl::scalar(pyjson::get(value, key)).unwrap_or_default()
}

fn load_contract(root: &Path, registry: &str, wanted: &str) -> Result<Py, String> {
    let content = slurp(root, registry, "derived-state registry")?;
    let mut matches = Vec::new();
    for (index, line) in content.lines().enumerate() {
        if line.is_empty() {
            continue;
        }
        let record = pyjson::loads_strict(line)
            .map_err(|_| format!("{LABEL}: invalid registry JSON on line {}", index + 1))?;
        if !matches!(record, Py::Dict(_)) {
            return Err(format!(
                "{LABEL}: invalid registry JSON on line {}",
                index + 1
            ));
        }
        if scalar(&record, "contract_id") == wanted {
            matches.push(record);
        }
    }
    if matches.len() != 1 {
        return Err(format!(
            "{LABEL}: expected one registry contract '{wanted}', found {}",
            matches.len()
        ));
    }
    Ok(matches.remove(0))
}

fn one_capture(content: &str, kind: Capture, label: &str) -> Result<String, String> {
    let mut values = Vec::new();
    match kind {
        Capture::WorkspaceRust => {
            for line in content.lines() {
                if let Some(rest) = line.strip_prefix("rust-version") {
                    let rest = rest.trim_start();
                    if let Some(rest) = rest.strip_prefix('=') {
                        let rest = rest.trim();
                        if rest.starts_with('"') && rest.ends_with('"') && rest.len() >= 2 {
                            values.push(rest[1..rest.len() - 1].to_string());
                        }
                    }
                }
            }
        }
        Capture::RustCopy => {
            for line in content.lines() {
                let rest = line.trim_end();
                if let Some(version) = rest.strip_prefix("- Rust `")
                    && let Some(version) = version.strip_suffix('`')
                {
                    values.push(version.to_string());
                }
            }
        }
        Capture::Ci => {
            for line in content.lines() {
                let rest = line.trim();
                if let Some(version) = rest.strip_prefix("toolchain:") {
                    values.push(version.trim().to_string());
                }
            }
        }
        Capture::Boundary => {
            if let Some(start) = content.find("## Current downstream boundary") {
                let after = &content[start + "## Current downstream boundary".len()..];
                if let Some(end) = after.find("## Open correspondence") {
                    values.push(after[..end].trim().to_string());
                }
            }
        }
        Capture::Hash40 => {
            for word in content.split(|character: char| !character.is_ascii_hexdigit()) {
                if word.len() == 40 && word.chars().all(|character| character.is_ascii_hexdigit()) {
                    values.push(word.to_string());
                }
            }
        }
    }
    if values.len() != 1 {
        return Err(format!(
            "{LABEL}: expected one {label}, found {}",
            values.len()
        ));
    }
    Ok(values.remove(0))
}

#[derive(Clone, Copy)]
enum Capture {
    WorkspaceRust,
    RustCopy,
    Ci,
    Boundary,
    Hash40,
}

fn declared_copy(root: &Path, record: &Py, label: &str) -> Result<Copy, String> {
    if !matches!(record, Py::Dict(_)) {
        return Err(format!("{LABEL}: {label} declaration must be an object"));
    }
    let path = scalar(record, "path");
    let marker = scalar(record, "field_marker");
    if !safe_relative_path(&path) {
        return Err(format!(
            "{LABEL}: {label} has unsafe or missing declared path"
        ));
    }
    if marker.is_empty() {
        return Err(format!("{LABEL}: {label} lacks a declared field marker"));
    }
    let content = slurp(root, &path, label)?;
    let count = content.matches(&marker).count();
    if count != 1 {
        return Err(format!(
            "{LABEL}: {label} marker must occur exactly once in '{path}', found {count}"
        ));
    }
    Ok(Copy { path, content })
}

fn required_secondary_copies(
    root: &Path,
    contract: &Py,
    roles: &[&str],
) -> Result<BTreeMap<String, Copy>, String> {
    let Some(Py::List(copies)) = pyjson::get(contract, "secondary_copies") else {
        return Err(format!(
            "{LABEL}: contract lacks secondary_copies declarations"
        ));
    };
    let mut by_role = BTreeMap::new();
    for copy in copies {
        if !matches!(copy, Py::Dict(_)) {
            return Err(format!(
                "{LABEL}: secondary copy declaration must be an object"
            ));
        }
        let role = scalar(copy, "role");
        if role.is_empty()
            || !role
                .chars()
                .next()
                .is_some_and(|character| character.is_ascii_lowercase())
            || !role.chars().all(|character| {
                character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || matches!(character, '.' | '_' | '-')
            })
        {
            return Err(format!(
                "{LABEL}: secondary copy has invalid or missing role"
            ));
        }
        if !roles.contains(&role.as_str()) {
            return Err(format!("{LABEL}: unknown secondary copy role '{role}'"));
        }
        if by_role.contains_key(&role) {
            return Err(format!(
                "{LABEL}: secondary copy role '{role}' is declared more than once"
            ));
        }
        by_role.insert(
            role.clone(),
            declared_copy(root, copy, &format!("secondary copy role '{role}'"))?,
        );
    }
    for role in roles {
        if !by_role.contains_key(*role) {
            return Err(format!(
                "{LABEL}: required secondary copy role '{role}' is missing"
            ));
        }
    }
    Ok(by_role)
}

fn normalize_rust_version(value: &str, label: &str) -> Result<String, String> {
    let parts: Vec<&str> = value.split('.').collect();
    let valid = (2..=3).contains(&parts.len())
        && parts.iter().all(|part| {
            !part.is_empty()
                && part.chars().all(|character| character.is_ascii_digit())
                && (*part == "0" || !part.starts_with('0'))
        });
    if !valid {
        return Err(format!(
            "{LABEL}: {label} has unsupported Rust version '{value}'"
        ));
    }
    Ok(format!(
        "{}.{}.{}",
        parts[0],
        parts[1],
        parts.get(2).unwrap_or(&"0")
    ))
}

fn check_rust_prerequisite(root: &Path, contract: &Py) -> Result<(), String> {
    let cargo = slurp(root, "Cargo.toml", "workspace manifest")?;
    let primary = declared_copy(root, contract, "primary Rust prerequisite")?;
    let secondary = required_secondary_copies(root, contract, &["rust_book", "rust_ci"])?;
    let authority = one_capture(&cargo, Capture::WorkspaceRust, "workspace rust-version")?;
    let readme = one_capture(
        &primary.content,
        Capture::RustCopy,
        &format!("{} Rust prerequisite", primary.path),
    )?;
    let book = &secondary["rust_book"];
    let book_copy = one_capture(
        &book.content,
        Capture::RustCopy,
        &format!("{} Rust prerequisite", book.path),
    )?;
    let ci = &secondary["rust_ci"];
    let ci_copy = one_capture(
        &ci.content,
        Capture::Ci,
        &format!("{} Rust toolchain", ci.path),
    )?;
    let normalized = normalize_rust_version(&authority, "workspace rust-version")?;
    for (path, actual) in [
        (&primary.path, readme),
        (&book.path, book_copy),
        (&ci.path, ci_copy),
    ] {
        let actual = normalize_rust_version(&actual, &format!("{path} Rust copy"))?;
        if actual != normalized {
            return Err(format!(
                "{LABEL}: {path} Rust copy '{actual}' differs from workspace authority '{normalized}'"
            ));
        }
    }
    Ok(())
}

fn check_fsmgen_gitlink(root: &Path, contract: &Py) -> Result<(), String> {
    let primary = declared_copy(root, contract, "primary FSMGen feedback copy")?;
    let secondary = required_secondary_copies(root, contract, &["feedback_contract_json"])?;
    let contract_copy = &secondary["feedback_contract_json"];
    let contract_text = &contract_copy.content;
    let parsed = pyjson::loads_strict(contract_text)
        .map_err(|_| format!("{LABEL}: FSMGen feedback contract is invalid JSON"))?;
    if !matches!(parsed, Py::Dict(_)) {
        return Err(format!("{LABEL}: FSMGen feedback contract is invalid JSON"));
    }
    let boundary = one_capture(
        &primary.content,
        Capture::Boundary,
        "current downstream boundary",
    )?;
    let feedback_copy = one_capture(&boundary, Capture::Hash40, "current feedback gitlink")?;
    let Some(Py::List(required)) = pyjson::get(&parsed, "current_root")
        .and_then(|current| pyjson::get(current, "required_literals"))
    else {
        return Err(format!(
            "{LABEL}: feedback contract lacks required_literals"
        ));
    };
    let mut contract_copies: Vec<String> = required
        .iter()
        .filter(|value| matches!(value, Py::Str(_)))
        .map(|value| perl::scalar(Some(value)).unwrap_or_default())
        .filter(|value| {
            value.len() == 40 && value.chars().all(|character| character.is_ascii_hexdigit())
        })
        .collect();
    if contract_copies.len() != 1 {
        return Err(format!(
            "{LABEL}: expected one contract gitlink literal, found {}",
            contract_copies.len()
        ));
    }
    // Original interface: git', '-C', $root, 'ls-files', '--stage', '--', 'subs/fsmgen'.
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["ls-files", "--stage", "--", "subs/fsmgen"])
        .output()
        .map_err(|_| format!("{LABEL}: cannot query FSMGen Git-index entry"))?;
    if !output.status.success() {
        return Err(format!("{LABEL}: Git-index query failed"));
    }
    let rows: Vec<&str> = std::str::from_utf8(&output.stdout)
        .unwrap_or("")
        .lines()
        .filter(|line| !line.is_empty())
        .collect();
    if rows.len() != 1 {
        return Err(format!(
            "{LABEL}: expected one FSMGen Git-index entry, found {}",
            rows.len()
        ));
    }
    let row = rows[0];
    let mut fields = row.split_whitespace();
    let mode = fields.next().unwrap_or("");
    let object = fields.next().unwrap_or("");
    let stage = fields.next().unwrap_or("");
    let path = fields.next().unwrap_or("");
    if mode.len() != 6
        || object.len() < 40
        || object.len() > 64
        || !object
            .chars()
            .all(|character| character.is_ascii_hexdigit())
        || stage.len() != 1
        || path != "subs/fsmgen"
    {
        return Err(format!("{LABEL}: malformed FSMGen Git-index entry"));
    }
    if mode != "160000" {
        return Err(format!(
            "{LABEL}: subs/fsmgen index mode is '{mode}', expected '160000'"
        ));
    }
    if stage != "0" {
        return Err(format!(
            "{LABEL}: subs/fsmgen index stage is '{stage}', expected '0'"
        ));
    }
    for (path, actual) in [
        (&primary.path, feedback_copy),
        (&contract_copy.path, contract_copies.remove(0)),
    ] {
        if actual != object {
            return Err(format!(
                "{LABEL}: {path} gitlink '{actual}' differs from Git-index authority '{object}'"
            ));
        }
    }
    Ok(())
}

/// Run an authority adapter with the arguments its Perl shim normalized.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn super::Output) -> i32 {
    let mut root = root.to_path_buf();
    let mut registry = "doctrine/live_document_size/derived_state_contracts.jsonl".to_string();
    let mut contract = None;
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
            "--contract" => match arguments.next() {
                Some(id) => contract = Some(id.to_string()),
                None => return usage(output),
            },
            _ => return usage(output),
        }
    }
    let Some(contract) = contract else {
        return usage(output);
    };
    if contract.is_empty()
        || !contract
            .chars()
            .next()
            .is_some_and(|character| character.is_ascii_lowercase())
        || !contract.chars().all(|character| {
            character.is_ascii_lowercase()
                || character.is_ascii_digit()
                || matches!(character, '.' | '_' | '-')
        })
    {
        return usage(output);
    }
    let outcome = (|| {
        let loaded = load_contract(&root, &registry, &contract)?;
        if scalar(&loaded, "classification") != "verified_copy" {
            return Err(format!(
                "{LABEL}: contract '{contract}' is not a verified copy"
            ));
        }
        if !safe_relative_path(&registry) {
            return Err(format!("{LABEL}: unsafe registry path"));
        }
        match contract.as_str() {
            "rust_prerequisite_copies" => check_rust_prerequisite(&root, &loaded),
            "fsmgen_gitlink_copies" => check_fsmgen_gitlink(&root, &loaded),
            _ => Err(format!(
                "{LABEL}: contract '{contract}' is not a declared project adapter"
            )),
        }
    })();
    match outcome {
        Ok(()) => {
            output.out(&format!(
                "{LABEL}: {contract} agrees with its canonical authority.\n"
            ));
            0
        }
        Err(error) => {
            output.err(&format!("{error}\n"));
            1
        }
    }
}

fn usage(output: &mut dyn super::Output) -> i32 {
    output.err("Usage: cargo xtask check derived-state-authorities [--root DIR] [--registry PATH] --contract ID\n");
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_real_adapters_agree_with_their_authorities() {
        let root = crate::repository_root();
        for contract in ["rust_prerequisite_copies", "fsmgen_gitlink_copies"] {
            let mut output = super::super::Captured::default();
            assert_eq!(
                run_to(
                    &root,
                    &["--contract".to_string(), contract.to_string()],
                    &mut output
                ),
                0,
                "{}",
                output.text
            );
            assert_eq!(
                output.text,
                format!(
                    "derived-state-authority: {contract} agrees with its canonical authority.\n"
                )
            );
        }
    }

    #[test]
    fn the_real_feedback_boundary_captures_the_current_gitlink() {
        let root = crate::repository_root();
        let contract = load_contract(
            &root,
            "doctrine/live_document_size/derived_state_contracts.jsonl",
            "fsmgen_gitlink_copies",
        )
        .expect("contract");
        let primary =
            declared_copy(&root, &contract, "primary FSMGen feedback copy").expect("primary");
        let boundary = one_capture(
            &primary.content,
            Capture::Boundary,
            "current downstream boundary",
        )
        .expect("boundary");
        let hash =
            one_capture(&boundary, Capture::Hash40, "current feedback gitlink").expect("hash");
        assert_eq!(hash.len(), 40);
    }
}
