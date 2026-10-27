//! The 25 derived-state authority checks, native since `RUSTDOC-AND-XTASK.15.j`: fresh Git repositories,
//! verified-copy declarations and every fail-closed mutation, plus the source-literal check that forbids
//! secondary-copy fallback paths in the adapter.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::pyjson::{self, Py};

use super::Output;
use super::derived_state_authorities;

struct Fixture {
    root: PathBuf,
    gitlink: String,
    contracts: Vec<Py>,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
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

fn field<'a>(value: &'a Py, key: &str) -> Option<&'a Py> {
    pyjson::get(value, key)
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

fn write_text(root: &Path, relative: &str, content: &str) -> Result<(), String> {
    let path = root.join(relative);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("create {}: {error}", parent.display()))?;
    }
    fs::write(&path, content)
        .map_err(|error| format!("cannot write fixture '{}': {error}", path.display()))
}

fn read_text(root: &Path, relative: &str) -> Result<String, String> {
    let path = root.join(relative);
    fs::read_to_string(&path).map_err(|error| format!("cannot read fixture '{relative}': {error}"))
}

fn git(root: &Path, arguments: &[&str]) -> Result<(), String> {
    let status = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .status()
        .map_err(|error| format!("fixture Git command failed: {error}"))?;
    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "fixture Git command failed: git {}",
            arguments.join(" ")
        ))
    }
}

fn git_output(root: &Path, arguments: &[&str]) -> Result<String, String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(arguments)
        .output()
        .map_err(|error| format!("cannot run fixture Git query: {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "fixture Git query failed: git {}",
            arguments.join(" ")
        ));
    }
    Ok(String::from_utf8_lossy(&output.stdout)
        .trim_end()
        .to_string())
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
    .ok_or_else(|| "cannot encode fixture JSON".to_string())
}

fn save_registry(fixture: &Fixture) -> Result<(), String> {
    let meta = object(vec![
        ("record_type", text("registry")),
        ("schema_version", Py::Int("1".to_string())),
        ("max_records", Py::Int("8".to_string())),
        ("max_bytes", Py::Int("8192".to_string())),
        ("max_record_bytes", Py::Int("2048".to_string())),
        ("max_array_items", Py::Int("8".to_string())),
        ("max_scalar_bytes", Py::Int("512".to_string())),
    ]);
    let mut content = encode(&meta)? + "\n";
    for contract in &fixture.contracts {
        content.push_str(&(encode(contract)? + "\n"));
    }
    write_text(
        &fixture.root,
        "doctrine/live_document_size/derived_state_contracts.jsonl",
        &content,
    )
}

fn contract_mut<'a>(fixture: &'a mut Fixture, id: &str) -> &'a mut Py {
    fixture
        .contracts
        .iter_mut()
        .find(|candidate| {
            field(candidate, "contract_id")
                .and_then(|value| match value {
                    Py::Str(text) => Some(pyjson::to_text(text)),
                    _ => None,
                })
                .is_some_and(|candidate| candidate == id)
        })
        .unwrap_or_else(|| panic!("fixture contract '{id}' does not exist"))
}

fn secondary_copy_mut<'a>(fixture: &'a mut Fixture, contract_id: &str, role: &str) -> &'a mut Py {
    let copies = match field_mut(contract_mut(fixture, contract_id), "secondary_copies") {
        Some(Py::List(copies)) => copies,
        _ => panic!("fixture secondary copies do not exist"),
    };
    copies
        .iter_mut()
        .find(|copy| {
            field(copy, "role")
                .and_then(|value| match value {
                    Py::Str(text) => Some(pyjson::to_text(text)),
                    _ => None,
                })
                .is_some_and(|candidate| candidate == role)
        })
        .unwrap_or_else(|| panic!("fixture secondary copy '{contract_id}/{role}' does not exist"))
}

fn new_fixture(root: &Path) -> Result<Fixture, String> {
    write_text(root, "README.md", "fixture seed\n")?;
    git(root, &["init", "-q"])?;
    git(root, &["config", "user.name", "SpecForge Fixture"])?;
    git(root, &["config", "user.email", "fixture@specforge.invalid"])?;
    git(root, &["config", "core.hooksPath", ".git/no-hooks"])?;
    git(root, &["add", "README.md"])?;
    git(root, &["commit", "-q", "-m", "fixture authority object"])?;
    let gitlink = git_output(root, &["rev-parse", "HEAD"])?;
    write_text(
        root,
        "Cargo.toml",
        "[workspace]\nmembers = []\n\n[workspace.package]\nrust-version = \"1.95\"\n",
    )?;
    write_text(root, "README.md", "# Fixture\n\n- Rust `1.95.0`\n")?;
    write_text(
        root,
        "docs/book/src/getting-started.md",
        "# Getting Started\n\n- Rust `1.95.0`\n",
    )?;
    write_text(
        root,
        ".github/workflows/ci.yml",
        "jobs:\n  rust:\n    steps:\n      - with:\n          toolchain: 1.95.0\n",
    )?;
    write_text(
        root,
        "docs/FSMGEN_FEEDBACK.md",
        &format!(
            "# Feedback\n\n## Current downstream boundary\n\nPinned object `{gitlink}`.\n\n## Open correspondence\n\n- None.\n"
        ),
    )?;
    let feedback_json = object(vec![(
        "current_root",
        object(vec![("required_literals", Py::List(vec![text(&gitlink)]))]),
    )]);
    write_text(
        root,
        "doctrine/live_document_size/fsmgen_feedback.json",
        &(encode(&feedback_json)? + "\n"),
    )?;
    let contracts = vec![
        object(vec![
            ("record_type", text("contract")),
            ("schema_version", Py::Int("1".to_string())),
            ("contract_id", text("rust_prerequisite_copies")),
            ("classification", text("verified_copy")),
            ("path", text("README.md")),
            ("field_marker", text("- Rust `1.95.0`")),
            (
                "secondary_copies",
                Py::List(vec![
                    object(vec![
                        ("role", text("rust_book")),
                        ("ownership", text("surface")),
                        ("surface_id", text("shipped_behavior")),
                        ("path", text("docs/book/src/getting-started.md")),
                        ("field_marker", text("- Rust `1.95.0`")),
                    ]),
                    object(vec![
                        ("role", text("rust_ci")),
                        ("ownership", text("control")),
                        ("path", text(".github/workflows/ci.yml")),
                        ("field_marker", text("toolchain: 1.95.0")),
                    ]),
                ]),
            ),
        ]),
        object(vec![
            ("record_type", text("contract")),
            ("schema_version", Py::Int("1".to_string())),
            ("contract_id", text("fsmgen_gitlink_copies")),
            ("classification", text("verified_copy")),
            ("path", text("docs/FSMGEN_FEEDBACK.md")),
            ("field_marker", text(&gitlink)),
            (
                "secondary_copies",
                Py::List(vec![object(vec![
                    ("role", text("feedback_contract_json")),
                    ("ownership", text("control")),
                    (
                        "path",
                        text("doctrine/live_document_size/fsmgen_feedback.json"),
                    ),
                    ("field_marker", text(&gitlink)),
                ])]),
            ),
        ]),
    ];
    let fixture = Fixture {
        root: root.to_path_buf(),
        gitlink: gitlink.clone(),
        contracts,
    };
    save_registry(&fixture)?;
    git(
        root,
        &[
            "update-index",
            "--add",
            "--cacheinfo",
            &format!("160000,{gitlink},subs/fsmgen"),
        ],
    )?;
    Ok(fixture)
}

fn run_adapter(fixture: &Fixture, contract: &str) -> (i32, String) {
    let mut output = super::Captured::default();
    let code = derived_state_authorities::run_to(
        &fixture.root,
        &[
            "--root".to_string(),
            fixture.root.to_string_lossy().into_owned(),
            "--registry".to_string(),
            "doctrine/live_document_size/derived_state_contracts.jsonl".to_string(),
            "--contract".to_string(),
            contract.to_string(),
        ],
        &mut output,
    );
    (code, output.text)
}

#[derive(Clone, Copy)]
enum Mutation {
    None,
    PatchSemver,
    ReadmeDrift,
    BookDrift,
    CiDrift,
    UnsupportedSemver,
    DuplicateRust,
    FeedbackDrift,
    FeedbackJsonDrift,
    NonGitlink,
    MissingGitlink,
    DuplicateBoundary,
    InvalidFeedbackJson,
    AlternateBook,
    AlternateCi,
    AlternateJson,
    MissingRoles,
    DuplicateRoles,
    UnknownRoles,
    UnsafeSecondary,
    MissingMarker,
    NonVerified,
}

struct Case {
    name: &'static str,
    contract: &'static str,
    success: bool,
    pattern: Option<&'static str>,
    mutation: Mutation,
}

const CASES: &[Case] = &[
    Case {
        name: "workspace short semver normalizes and all Rust copies agree",
        contract: "rust_prerequisite_copies",
        success: true,
        pattern: Some("agrees with its canonical authority"),
        mutation: Mutation::None,
    },
    Case {
        name: "workspace patch semver remains stable",
        contract: "rust_prerequisite_copies",
        success: true,
        pattern: Some("agrees with its canonical authority"),
        mutation: Mutation::PatchSemver,
    },
    Case {
        name: "README Rust drift fails closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("README.md Rust copy "),
        mutation: Mutation::ReadmeDrift,
    },
    Case {
        name: "mdBook Rust drift fails closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("getting-started.md Rust copy "),
        mutation: Mutation::BookDrift,
    },
    Case {
        name: "CI Rust drift fails closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("ci.yml Rust copy "),
        mutation: Mutation::CiDrift,
    },
    Case {
        name: "unsupported workspace semver fails closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("unsupported Rust version"),
        mutation: Mutation::UnsupportedSemver,
    },
    Case {
        name: "duplicate Rust declarations fail closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("primary Rust prerequisite marker must occur exactly once"),
        mutation: Mutation::DuplicateRust,
    },
    Case {
        name: "feedback and JSON copies agree with the mode-160000 index object",
        contract: "fsmgen_gitlink_copies",
        success: true,
        pattern: Some("agrees with its canonical authority"),
        mutation: Mutation::None,
    },
    Case {
        name: "feedback gitlink drift fails closed",
        contract: "fsmgen_gitlink_copies",
        success: false,
        pattern: Some("FSMGEN_FEEDBACK.md gitlink "),
        mutation: Mutation::FeedbackDrift,
    },
    Case {
        name: "feedback JSON gitlink drift fails closed",
        contract: "fsmgen_gitlink_copies",
        success: false,
        pattern: Some("fsmgen_feedback.json gitlink "),
        mutation: Mutation::FeedbackJsonDrift,
    },
    Case {
        name: "non-gitlink index mode fails closed",
        contract: "fsmgen_gitlink_copies",
        success: false,
        pattern: Some("index mode is '100644'"),
        mutation: Mutation::NonGitlink,
    },
    Case {
        name: "missing gitlink index entry fails closed",
        contract: "fsmgen_gitlink_copies",
        success: false,
        pattern: Some("expected one FSMGen Git-index entry, found 0"),
        mutation: Mutation::MissingGitlink,
    },
    Case {
        name: "duplicate current-boundary hashes fail closed",
        contract: "fsmgen_gitlink_copies",
        success: false,
        pattern: Some("primary FSMGen feedback copy marker must occur exactly once"),
        mutation: Mutation::DuplicateBoundary,
    },
    Case {
        name: "invalid feedback JSON fails closed",
        contract: "fsmgen_gitlink_copies",
        success: false,
        pattern: Some("contract is invalid JSON"),
        mutation: Mutation::InvalidFeedbackJson,
    },
    Case {
        name: "declared alternate mdBook path works without fallback",
        contract: "rust_prerequisite_copies",
        success: true,
        pattern: Some("agrees with its canonical authority"),
        mutation: Mutation::AlternateBook,
    },
    Case {
        name: "declared alternate CI path works without fallback",
        contract: "rust_prerequisite_copies",
        success: true,
        pattern: Some("agrees with its canonical authority"),
        mutation: Mutation::AlternateCi,
    },
    Case {
        name: "declared alternate JSON path works without fallback",
        contract: "fsmgen_gitlink_copies",
        success: true,
        pattern: Some("agrees with its canonical authority"),
        mutation: Mutation::AlternateJson,
    },
    Case {
        name: "missing secondary roles fail closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("required secondary copy role 'rust_ci' is missing"),
        mutation: Mutation::MissingRoles,
    },
    Case {
        name: "duplicate secondary roles fail closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("secondary copy role 'rust_book' is declared more than once"),
        mutation: Mutation::DuplicateRoles,
    },
    Case {
        name: "unknown secondary roles fail closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("unknown secondary copy role 'hidden_fallback'"),
        mutation: Mutation::UnknownRoles,
    },
    Case {
        name: "unsafe declared secondary paths fail closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("secondary copy role 'rust_ci' has unsafe or missing declared path"),
        mutation: Mutation::UnsafeSecondary,
    },
    Case {
        name: "missing declared secondary markers fail closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("secondary copy role 'rust_book' marker must occur exactly once"),
        mutation: Mutation::MissingMarker,
    },
    Case {
        name: "unknown adapter contracts fail closed",
        contract: "unknown_contract",
        success: false,
        pattern: Some("expected one registry contract 'unknown_contract'"),
        mutation: Mutation::None,
    },
    Case {
        name: "non-verified adapter contracts fail closed",
        contract: "rust_prerequisite_copies",
        success: false,
        pattern: Some("is not a verified copy"),
        mutation: Mutation::NonVerified,
    },
];

fn mutate(fixture: &mut Fixture, mutation: Mutation) -> Result<(), String> {
    match mutation {
        Mutation::None => {}
        Mutation::PatchSemver => write_text(
            &fixture.root,
            "Cargo.toml",
            "[workspace.package]\nrust-version = \"1.95.0\"\n",
        )?,
        Mutation::ReadmeDrift => {
            write_text(&fixture.root, "README.md", "# Fixture\n\n- Rust `1.96.0`\n")?;
            set_field(
                contract_mut(fixture, "rust_prerequisite_copies"),
                "field_marker",
                text("- Rust `1.96.0`"),
            );
        }
        Mutation::BookDrift => {
            write_text(
                &fixture.root,
                "docs/book/src/getting-started.md",
                "# Getting Started\n\n- Rust `1.96.0`\n",
            )?;
            set_field(
                secondary_copy_mut(fixture, "rust_prerequisite_copies", "rust_book"),
                "field_marker",
                text("- Rust `1.96.0`"),
            );
        }
        Mutation::CiDrift => {
            write_text(
                &fixture.root,
                ".github/workflows/ci.yml",
                "toolchain: 1.96.0\n",
            )?;
            set_field(
                secondary_copy_mut(fixture, "rust_prerequisite_copies", "rust_ci"),
                "field_marker",
                text("toolchain: 1.96.0"),
            );
        }
        Mutation::UnsupportedSemver => write_text(
            &fixture.root,
            "Cargo.toml",
            "[workspace.package]\nrust-version = \"1.95-beta\"\n",
        )?,
        Mutation::DuplicateRust => write_text(
            &fixture.root,
            "README.md",
            "- Rust `1.95.0`\n- Rust `1.95.0`\n",
        )?,
        Mutation::FeedbackDrift => {
            let wrong = "0".repeat(40);
            write_text(
                &fixture.root,
                "docs/FSMGEN_FEEDBACK.md",
                &format!(
                    "# Feedback\n\n## Current downstream boundary\n\nPinned object `{wrong}`.\n\n## Open correspondence\n"
                ),
            )?;
            set_field(
                contract_mut(fixture, "fsmgen_gitlink_copies"),
                "field_marker",
                text(&wrong),
            );
        }
        Mutation::FeedbackJsonDrift => {
            let wrong = "0".repeat(40);
            let value = object(vec![(
                "current_root",
                object(vec![("required_literals", Py::List(vec![text(&wrong)]))]),
            )]);
            write_text(
                &fixture.root,
                "doctrine/live_document_size/fsmgen_feedback.json",
                &(encode(&value)? + "\n"),
            )?;
            set_field(
                secondary_copy_mut(fixture, "fsmgen_gitlink_copies", "feedback_contract_json"),
                "field_marker",
                text(&wrong),
            );
        }
        Mutation::NonGitlink => {
            let blob = git_output(&fixture.root, &["hash-object", "-w", "README.md"])?;
            git(
                &fixture.root,
                &[
                    "update-index",
                    "--add",
                    "--cacheinfo",
                    &format!("100644,{blob},subs/fsmgen"),
                ],
            )?;
        }
        Mutation::MissingGitlink => git(
            &fixture.root,
            &["update-index", "--force-remove", "subs/fsmgen"],
        )?,
        Mutation::DuplicateBoundary => {
            let hash = &fixture.gitlink;
            write_text(
                &fixture.root,
                "docs/FSMGEN_FEEDBACK.md",
                &format!(
                    "# Feedback\n\n## Current downstream boundary\n\n{hash} and {hash}.\n\n## Open correspondence\n"
                ),
            )?;
        }
        Mutation::InvalidFeedbackJson => write_text(
            &fixture.root,
            "doctrine/live_document_size/fsmgen_feedback.json",
            &format!("{{invalid \"{}\"\n", fixture.gitlink),
        )?,
        Mutation::AlternateBook => {
            write_text(&fixture.root, "declared/book-copy.md", "- Rust `1.95.0`\n")?;
            write_text(
                &fixture.root,
                "docs/book/src/getting-started.md",
                "- Rust `9.9.9`\n",
            )?;
            set_field(
                secondary_copy_mut(fixture, "rust_prerequisite_copies", "rust_book"),
                "path",
                text("declared/book-copy.md"),
            );
        }
        Mutation::AlternateCi => {
            write_text(&fixture.root, "declared/ci-copy.yml", "toolchain: 1.95.0\n")?;
            write_text(
                &fixture.root,
                ".github/workflows/ci.yml",
                "toolchain: 9.9.9\n",
            )?;
            set_field(
                secondary_copy_mut(fixture, "rust_prerequisite_copies", "rust_ci"),
                "path",
                text("declared/ci-copy.yml"),
            );
        }
        Mutation::AlternateJson => {
            let content = read_text(
                &fixture.root,
                "doctrine/live_document_size/fsmgen_feedback.json",
            )?;
            write_text(&fixture.root, "declared/feedback-copy.json", &content)?;
            write_text(
                &fixture.root,
                "doctrine/live_document_size/fsmgen_feedback.json",
                "{broken\n",
            )?;
            set_field(
                secondary_copy_mut(fixture, "fsmgen_gitlink_copies", "feedback_contract_json"),
                "path",
                text("declared/feedback-copy.json"),
            );
        }
        Mutation::MissingRoles => {
            let contract = contract_mut(fixture, "rust_prerequisite_copies");
            if let Some(Py::List(copies)) = field_mut(contract, "secondary_copies") {
                copies.pop();
            }
        }
        Mutation::DuplicateRoles => set_field(
            secondary_copy_mut(fixture, "rust_prerequisite_copies", "rust_ci"),
            "role",
            text("rust_book"),
        ),
        Mutation::UnknownRoles => set_field(
            secondary_copy_mut(fixture, "rust_prerequisite_copies", "rust_ci"),
            "role",
            text("hidden_fallback"),
        ),
        Mutation::UnsafeSecondary => set_field(
            secondary_copy_mut(fixture, "rust_prerequisite_copies", "rust_ci"),
            "path",
            text("../ci.yml"),
        ),
        Mutation::MissingMarker => set_field(
            secondary_copy_mut(fixture, "rust_prerequisite_copies", "rust_book"),
            "field_marker",
            text("- Rust `1.94.0`"),
        ),
        Mutation::NonVerified => set_field(
            contract_mut(fixture, "rust_prerequisite_copies"),
            "classification",
            text("authored_intent"),
        ),
    }
    Ok(())
}

fn report_result(
    name: &str,
    number: usize,
    passed: bool,
    output: &str,
    quiet: bool,
    output_sink: &mut dyn Output,
) {
    if passed {
        if !quiet {
            output_sink.out(&format!("ok {number} - {name}\n"));
        }
    } else {
        output_sink.out(&format!("not ok {number} - {name}\n"));
        for line in output.lines() {
            output_sink.out(&format!("# {line}\n"));
        }
    }
}

/// Run all 25 checks with the original `--quiet` or verbose output contract.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the eventual native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn Output) -> i32 {
    let quiet = match arguments {
        [] => false,
        [argument] if argument == "--quiet" => true,
        _ => {
            output.err("Usage: cargo xtask check derived-state-authority-tests [--quiet]\n");
            return 2;
        }
    };
    let mut number = 0;
    let mut failures = 0;
    let source = match fs::read_to_string(root.join("scripts/check_derived_state_authorities.pl")) {
        Ok(source) => source,
        Err(error) => {
            output.err(&format!("cannot read adapter source: {error}\n"));
            return 1;
        }
    };
    let hidden: Vec<&str> = [
        "docs/book/src/getting-started.md",
        ".github/workflows/ci.yml",
        "doctrine/live_document_size/fsmgen_feedback.json",
    ]
    .into_iter()
    .filter(|literal| source.contains(literal))
    .collect();
    number += 1;
    if !hidden.is_empty() {
        failures += 1;
    }
    report_result(
        "adapter source contains no secondary copy path literals",
        number,
        hidden.is_empty(),
        &format!("hidden paths: {}\n", hidden.join(" ")),
        quiet,
        output,
    );
    for (index, case) in CASES.iter().enumerate() {
        let fixture_root = root.join(format!(
            "derived-state-authority-tests.{}.{}",
            std::process::id(),
            index
        ));
        let mut fixture = match new_fixture(&fixture_root) {
            Ok(fixture) => fixture,
            Err(error) => {
                output.err(&format!("{error}\n"));
                return 1;
            }
        };
        if let Err(error) = mutate(&mut fixture, case.mutation) {
            output.err(&format!("{error}\n"));
            return 1;
        }
        if let Err(error) = save_registry(&fixture) {
            output.err(&format!("{error}\n"));
            return 1;
        }
        let (status, adapter_output) = run_adapter(&fixture, case.contract);
        let mut passed = if case.success {
            status == 0
        } else {
            status != 0
        };
        if let Some(pattern) = case.pattern {
            passed &= adapter_output.contains(pattern);
        }
        number += 1;
        if !passed {
            failures += 1;
        }
        report_result(case.name, number, passed, &adapter_output, quiet, output);
    }
    if !quiet {
        output.out(&format!("1..{number}\n"));
    }
    if failures > 0 {
        output.err(&format!(
            "derived-state-authority-tests: {failures} of {number} checks failed\n"
        ));
        return 1;
    }
    if number != 25 {
        output.err(&format!(
            "derived-state-authority-tests: ran {number} checks, declaration expects 25 — re-derive the declaration beside the suite\n"
        ));
        return 1;
    }
    output.err("derived-state-authority-tests: 25/25 Rust/gitlink authority checks pass\n");
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn all_twenty_five_checks_pass() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(&root, &["--quiet".to_string()], &mut output),
            0,
            "{}",
            output.text
        );
        assert_eq!(
            output.text,
            "derived-state-authority-tests: 25/25 Rust/gitlink authority checks pass\n"
        );
    }
}
