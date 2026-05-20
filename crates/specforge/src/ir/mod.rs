pub mod adapters;
pub mod contract;
pub mod evidence;
pub mod fidelity;
pub mod fusion;
pub mod intent;
pub mod isf_ir;
pub mod prior_memory;
pub mod protocol_graph;
pub mod semantic;
pub mod source;

use serde::{Deserialize, Serialize};

/// Serializes tests that shell out to the external `subs/fsmgen/bin/fsmgen`
/// Perl binary. That binary resolves paths relative to its own location and
/// performs an internal `cd`; under cargo's parallel test execution the
/// concurrent invocations race and intermittently fail with
/// `Can't cd to : No such file or directory` (a CWD/contention flake, not an
/// `.isf` correctness issue). Every fsmgen-invoking test takes this lock so
/// at most one runs the binary at a time.
#[cfg(test)]
pub(crate) static FSMGEN_TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

/// Run `subs/fsmgen/bin/fsmgen --strict --check --json <isf_path>` for tests,
/// serialized via [`FSMGEN_TEST_LOCK`] and pinned to the fsmgen repo root as
/// CWD (mirrors the downstream issue-bundle protocol's "run from the FSMGen
/// repository root" requirement, so `FindBin`/internal `cd` resolve
/// deterministically). `isf_path` must be absolute. A poisoned lock is
/// recovered so one failing locked test does not cascade-fail the rest.
#[cfg(test)]
pub(crate) fn run_fsmgen_strict_check(isf_path: &std::path::Path) -> std::process::Output {
    let _guard = FSMGEN_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let fsmgen_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../subs/fsmgen");
    std::process::Command::new(fsmgen_root.join("bin/fsmgen"))
        .args(["--strict", "--check", "--json"])
        .arg(isf_path)
        .current_dir(&fsmgen_root)
        .output()
        .expect("run fsmgen")
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum IrStage {
    SourceIr,
    EvidenceIr,
    SemanticIr,
    IntentIr,
    IsfAdapter,
}

impl IrStage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SourceIr => "source_ir",
            Self::EvidenceIr => "evidence_ir",
            Self::SemanticIr => "semantic_ir",
            Self::IntentIr => "intent_ir",
            Self::IsfAdapter => "isf_adapter",
        }
    }
}
