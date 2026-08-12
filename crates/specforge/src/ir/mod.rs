pub mod adapters;
pub mod ambiguity;
pub mod condition_extract;
pub mod constraint_extract_llm;
pub mod contract;
pub mod corpus_cluster;
pub mod cve;
pub mod derivation;
pub mod entity_typing;
pub mod evidence;
pub mod extraction_filters;
pub mod extractor;
pub mod fidelity;
pub mod figure_region;
pub mod fusion;
pub mod intent;
pub mod isf_ir;
pub mod nli_verify;
pub mod nlp_relation_extract;
pub mod normative_vocab;
pub mod prior_memory;
pub mod protocol_graph;
pub mod register_bits;
pub mod semantic;
pub mod source;
pub mod temporal_ltl;
pub mod waveform;

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
    let fsmgen_root = crate::project_data::repository_root()
        .expect("current SpecForge repository root")
        .join("subs/fsmgen");
    let temporary = crate::project_data::tempdir().expect("project-local fsmgen workspace");
    let mut command = std::process::Command::new(fsmgen_root.join("bin/fsmgen"));
    crate::project_data::configure_command(&mut command).expect("project-local fsmgen environment");
    command
        .env("TMPDIR", temporary.path())
        .env("TMP", temporary.path())
        .env("TEMP", temporary.path())
        .args(["--strict", "--check", "--json"])
        .arg(isf_path)
        .current_dir(&fsmgen_root)
        .output()
        .expect("run fsmgen")
}

/// Run `subs/fsmgen/bin/fsmgen --emit-schedule-json <isf_path>` for tests, serialized via
/// [`FSMGEN_TEST_LOCK`] and pinned to the fsmgen repo root as CWD (same protocol as
/// [`run_fsmgen_strict_check`]). The scheduler report JSON carries `inferred_storage[].fields[]`
/// (`name, msb, lsb, width, access, reset, enum`), the introspection surface used to assert that
/// emitted register field maps round-trip through FSMGen (DOC-INTENT-TAXONOMY.4a.ii). `isf_path`
/// must be absolute.
#[cfg(test)]
pub(crate) fn run_fsmgen_schedule_json(isf_path: &std::path::Path) -> std::process::Output {
    let _guard = FSMGEN_TEST_LOCK
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner);
    let fsmgen_root = crate::project_data::repository_root()
        .expect("current SpecForge repository root")
        .join("subs/fsmgen");
    let temporary = crate::project_data::tempdir().expect("project-local fsmgen workspace");
    let mut command = std::process::Command::new(fsmgen_root.join("bin/fsmgen"));
    crate::project_data::configure_command(&mut command).expect("project-local fsmgen environment");
    command
        .env("TMPDIR", temporary.path())
        .env("TMP", temporary.path())
        .env("TEMP", temporary.path())
        .args(["--emit-schedule-json"])
        .arg(isf_path)
        .current_dir(&fsmgen_root)
        .output()
        .expect("run fsmgen")
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
