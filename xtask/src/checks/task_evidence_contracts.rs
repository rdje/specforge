//! The single eight-contract active-task list, run through the native checker in-process
//! (`RUSTDOC-AND-XTASK.15.y`).

use std::fs;
use std::path::Path;

use super::{Output, active_task_evidence};

const CONTRACTS: [&str; 8] = [
    "doctrine/live_document_size/active_task_evidence.json",
    "doctrine/live_document_size/corpus_task_evidence.json",
    "doctrine/live_document_size/spec_to_intent_task_evidence.json",
    "doctrine/live_document_size/claim_verification_task_evidence.json",
    "doctrine/live_document_size/pressure_headroom_task_evidence.json",
    "doctrine/live_document_size/extraction_quality_gauge_task_evidence.json",
    "doctrine/live_document_size/kg_isf_completeness_task_evidence.json",
    "doctrine/live_document_size/extraction_representation_gaps_task_evidence.json",
];

fn check_contracts(root: &Path, contracts: &[&str], output: &mut dyn Output) -> i32 {
    let mut failed = false;
    for contract in contracts {
        let arguments = vec![
            "--root".to_string(),
            root.to_string_lossy().into_owned(),
            "--contract".to_string(),
            (*contract).to_string(),
            "--check".to_string(),
        ];
        if active_task_evidence::run_to(root, &arguments, output) != 0 {
            failed = true;
        }
    }
    i32::from(failed)
}

/// Run all eight contracts with the original `--root` argument contract.
pub fn run(root: &Path, arguments: &[String]) -> i32 {
    run_to(root, arguments, &mut super::Terminal)
}

/// `run`, writing to `output` for the native `LIVE-DOC-SIZE` orchestrator.
pub fn run_to(root: &Path, arguments: &[String], output: &mut dyn Output) -> i32 {
    let root = match arguments {
        [] => root.to_path_buf(),
        [flag, directory] if flag == "--root" => match fs::canonicalize(directory) {
            Ok(root) if root.is_dir() => root,
            _ => return usage(output),
        },
        _ => return usage(output),
    };
    check_contracts(&root, &CONTRACTS, output)
}

fn usage(output: &mut dyn Output) -> i32 {
    let program = std::env::var("SPECFORGE_SHIM_PATH")
        .unwrap_or_else(|_| "cargo xtask check task-evidence-contracts".to_string());
    output.err(&format!("Usage: {program} [--root PROJECT_ROOT]\n"));
    2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_contract_runs_and_failures_accumulate() {
        let root = crate::repository_root();
        let mut output = super::super::Captured::default();
        let status = check_contracts(
            &root,
            &[
                CONTRACTS[0],
                "doctrine/live_document_size/no-such-contract.json",
            ],
            &mut output,
        );
        assert_eq!(status, 1);
        assert!(output.text.contains("migrated/complete contract is valid."));
        assert!(output.text.contains("no-such-contract.json"));
        assert!(output.text.contains("FAILED with"));
    }
}
