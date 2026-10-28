//! The complete `LIVE-DOC-SIZE` orchestrator, with every SpecForge component in-process
//! (`RUSTDOC-AND-XTASK.15.z`). The original shell remains its project-data-activating shim.

use std::fs;
use std::path::Path;

use crate::project_data;

use super::{
    Output, active_task_evidence, canonical_collection_catalogs, corpus_kb_currentness,
    derived_state_authority_tests, derived_state_contract_tests, derived_state_contracts,
    fact_card_catalog, fsmgen_feedback_protocol, fsmgen_request_bundles, knowledge_map_projection,
    knowledge_map_shard_contract, live_document_size_core, live_document_size_tests, readme_policy,
    roadmap_projection_contract, rolling_ledger_protocol, source_pdf_registry_currentness,
    task_evidence_contracts, task_tree_archive, task_tree_catalog, validation_snapshot_currentness,
};

type Check = fn(&Path, &[String], &mut dyn Output) -> i32;

const SELF_TESTS: [(Check, &[&str]); 22] = [
    (readme_policy::run_to, &["--self-test"]),
    (live_document_size_tests::run_to, &["--quiet"]),
    (derived_state_contract_tests::run_to, &["--quiet"]),
    (derived_state_authority_tests::run_to, &["--quiet"]),
    (rolling_ledger_protocol::run_to, &["--self-test"]),
    (roadmap_projection_contract::run_to, &["--self-test"]),
    (fsmgen_feedback_protocol::run_to, &["--self-test"]),
    (fsmgen_request_bundles::run_to, &["--self-test"]),
    (validation_snapshot_currentness::run_to, &["--self-test"]),
    (source_pdf_registry_currentness::run_to, &["--self-test"]),
    (corpus_kb_currentness::run_to, &["--self-test"]),
    (task_tree_catalog::run_to, &["--self-test"]),
    (task_tree_catalog::run_to, &["--check"]),
    (task_tree_archive::run_to, &["--self-test"]),
    (active_task_evidence::run_to, &["--self-test"]),
    (fact_card_catalog::run_to, &["--self-test"]),
    (fact_card_catalog::run_to, &["--check"]),
    (canonical_collection_catalogs::run_to, &["--self-test"]),
    (canonical_collection_catalogs::run_to, &["--check"]),
    (knowledge_map_shard_contract::run_to, &["--self-test"]),
    (knowledge_map_shard_contract::run_to, &["--check"]),
    (knowledge_map_projection::run_to, &[]),
];

const CHECKS: [(Check, &[&str]); 9] = [
    (
        rolling_ledger_protocol::run_to,
        &[
            "--registry",
            "doctrine/live_document_size/rolling_ledgers.jsonl",
        ],
    ),
    (
        roadmap_projection_contract::run_to,
        &[
            "--contract",
            "doctrine/live_document_size/roadmap_projection.json",
            "--check",
        ],
    ),
    (
        fsmgen_feedback_protocol::run_to,
        &[
            "--contract",
            "doctrine/live_document_size/fsmgen_feedback.json",
            "--check",
        ],
    ),
    (fsmgen_request_bundles::run_to, &["--check"]),
    (
        corpus_kb_currentness::run_to,
        &[
            "--contract",
            "doctrine/live_document_size/corpus_kb.json",
            "--check",
        ],
    ),
    (
        derived_state_contracts::run_to,
        &[
            "--registry",
            "doctrine/live_document_size/derived_state_contracts.jsonl",
            "--surfaces",
            "doctrine/live_document_size/surfaces.jsonl",
        ],
    ),
    (
        task_tree_archive::run_to,
        &[
            "--contract",
            "doctrine/live_document_size/task_tree_archive.json",
            "--check",
        ],
    ),
    (task_evidence_contracts::run_to, &[]),
    (
        live_document_size_core::run_to,
        &[
            "--registry",
            "doctrine/live_document_size/surfaces.jsonl",
            "--routes",
            "doctrine/readme_entrypoint/routed_destinations.tsv",
            "--authorities",
            "doctrine/live_document_size/ceiling_increase_authorities.jsonl",
        ],
    ),
];

fn call(check: Check, root: &Path, arguments: &[&str], output: &mut dyn Output, failed: &mut bool) {
    let arguments: Vec<String> = arguments
        .iter()
        .map(|argument| (*argument).to_string())
        .collect();
    let arguments = if arguments.is_empty() {
        Vec::new()
    } else {
        let mut with_root = vec!["--root".to_string(), root.to_string_lossy().into_owned()];
        with_root.extend(arguments);
        with_root
    };
    if check(root, &arguments, output) != 0 {
        *failed = true;
    }
}

fn call_default(
    check: Check,
    root: &Path,
    arguments: &[&str],
    output: &mut dyn Output,
    failed: &mut bool,
) {
    let arguments: Vec<String> = arguments
        .iter()
        .map(|argument| (*argument).to_string())
        .collect();
    if check(root, &arguments, output) != 0 {
        *failed = true;
    }
}

#[cfg(unix)]
fn executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn executable(path: &Path) -> bool {
    path.is_file()
}

/// Run the complete live-document-size gate with the original `--root` argument contract.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for callers composing other native checks.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn Output) -> i32 {
    let _environment = match project_data::activate(&crate::repository_root()) {
        Ok(environment) => environment,
        Err(error) => {
            output.err(&format!("{error}\n"));
            return 1;
        }
    };
    let root = match arguments {
        [] => root.to_path_buf(),
        [flag, directory] if flag == "--root" => match fs::canonicalize(directory) {
            Ok(root) if root.is_dir() => root,
            _ => return usage(output),
        },
        _ => return usage(output),
    };
    let mut failed = false;
    if !executable(&root.join("scripts/check_readme_policy.sh")) {
        output
            .err("live-document-size: required README route guard is missing or non-executable\n");
        failed = true;
    } else {
        call(readme_policy::run_to, &root, &[], output, &mut failed);
    }
    if root == crate::repository_root() {
        for (check, arguments) in SELF_TESTS {
            call_default(check, &root, arguments, output, &mut failed);
        }
    }
    for (check, arguments) in CHECKS {
        call(check, &root, arguments, output, &mut failed);
    }
    i32::from(failed)
}

fn usage(output: &mut dyn Output) -> i32 {
    let program = std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "cargo xtask check live-document-size".to_string());
    output.err(&format!("Usage: {program} [--root PROJECT_ROOT]\n"));
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_original_phases_are_complete_and_a_bad_root_is_refused() {
        assert_eq!(SELF_TESTS.len(), 22);
        assert_eq!(CHECKS.len(), 9);
        let root = crate::repository_root();
        let missing = root.join("generated/rdx-15z-not-a-root");
        let mut output = super::super::Captured::default();
        assert_eq!(
            run_to(
                &root,
                &["--root".to_string(), missing.to_string_lossy().into_owned()],
                &mut output,
            ),
            2
        );
        assert!(output.text.contains("[--root PROJECT_ROOT]"));

        fn probe(_root: &Path, arguments: &[String], output: &mut dyn Output) -> i32 {
            output.out(&format!("{arguments:?}\n"));
            0
        }
        let mut output = super::super::Captured::default();
        let mut failed = false;
        call(probe, &root, &["--self-test"], &mut output, &mut failed);
        assert!(!failed);
        assert_eq!(
            output.text,
            format!(
                "{:?}\n",
                vec![
                    "--root".to_string(),
                    root.to_string_lossy().into_owned(),
                    "--self-test".to_string()
                ]
            )
        );

        let mut output = super::super::Captured::default();
        call_default(probe, &root, &["--self-test"], &mut output, &mut failed);
        assert!(!failed);
        assert_eq!(output.text, "[\"--self-test\"]\n");
    }
}
