//! Composition of the first reviewed SpecForge trajectory snapshot.
//!
//! `SPEC-TO-INTENT-ALIGNMENT.5b` is deliberately separate from the generic controller in
//! [`super::trajectory`]. This module authenticates the frozen `.4c` vertical report and the
//! `.2` provider-free production-capability observation, derives the nine-dimensional input,
//! and evaluates it without changing either source authority.

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::source::TableKind;
use super::source_to_intent_eval::{CategoryStatus, QueryScore, VerticalEvalReport};
use super::trajectory::{
    CausalConfidence, ControllerAuthority, ControllerMode, DimensionObservation, EvidenceRef,
    Fraction, GapPriorityTier, GapUncertainty, HardGateObservation, ImprovementDirection,
    MetricObservation, MetricTarget, ReversibleSliceSize, TargetOperator, TaskOwner,
    TrajectoryControllerInput, TrajectoryDimension, TrajectoryGap, TrajectoryReport, Uncertainty,
    evaluate_trajectory,
};
use crate::error::{AppError, Result};

pub const CURRENT_VERTICAL_RESULT_PATH: &str =
    "crates/specforge/test_data/source_to_intent_vertical/result_snapshot.json";
pub const CURRENT_CAPABILITY_OBSERVATION_PATH: &str =
    "crates/specforge/test_data/trajectory/converge_provider_free_capabilities.json";
pub const CURRENT_CONTROLLER_INPUT_PATH: &str =
    "crates/specforge/test_data/trajectory/controller_input.json";
pub const CURRENT_TRAJECTORY_REPORT_PATH: &str =
    "crates/specforge/test_data/trajectory/trajectory_report.json";
pub const CURRENT_REPLAY_EVIDENCE_PATH: &str =
    "crates/specforge/test_data/trajectory/replays/aia_toc_current_binary_replay.json";
pub const OBJECTIVE_CONTRACT_PATH: &str = "doctrine/spec_to_intent_category_contract.json";
pub const TASK_TREE_PATH: &str = "docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md";

const VERTICAL_RESULT_SHA256: &str =
    "6b72f1fc2a5616542965f264bd87f23534b5274d19332c8a68bc6b88848547eb";
const OBJECTIVE_CONTRACT_SHA256: &str =
    "b5d0582f213b38a0273594a46e6a11cca3b1cce24a3c7156d300156b81ca7007";
// Updated only when the `.2` ledger-currentness test accepts a reviewed observation change.
const CAPABILITY_OBSERVATION_SHA256: &str =
    "b37f13d28b90a6e6b0fb4c554d0e9fc861ff5e993d3276743b15d0ad1736994e";
const REPLAY_EVIDENCE_SHA256: &str =
    "15e6a021a6e78cd23e9a31702f3a5b34bf45e43b3660fb8c38c2435d069fdd7a";
const REVIEWED_REVISION: &str = "03e89b66cf87fcc1ec0bf342f47a147261a8d739";
const AIA_DOCUMENT_KEY: &str = "1_0_2025_03_12_risc_v_advanced_interrupt_architecture";
const AIA_SOURCE_SHA256: &str = "2d359579dcb84c6d00a1b284db3a7f8ec8c87764d96406a45cbaa9052f04c7c8";
const TIMING_AUTHORITY_REVISION: &str = "46af2eca7a4c59e25a014e532716c92be371e29b";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityParticipation {
    Integrated,
    Scheduled,
    Omitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityRunState {
    Executed,
    InspectedOnly,
    NotExecuted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityProfile {
    pub vlm_provider: String,
    pub nlp_provider: String,
    pub rescan_plan_supplied: bool,
    pub promotion_executed: bool,
    pub extraction_quality_measured: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityObservationRecord {
    pub capability_id: String,
    pub command: String,
    pub entrypoint: String,
    pub participation: CapabilityParticipation,
    pub run_state: CapabilityRunState,
    pub reason: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CapabilityObservation {
    pub schema_version: u32,
    pub observation_id: String,
    pub profile: CapabilityProfile,
    pub production_capabilities: Vec<CapabilityObservationRecord>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplaySourceIdentity {
    portable_id: String,
    sha256: String,
    byte_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplayStageIdentities {
    source_ir_sha256: String,
    evidence_ir_sha256: String,
    semantic_ir_sha256: String,
    intent_ir_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplayTableObservation {
    table_id: String,
    table_kind: TableKind,
    row_count: usize,
    column_count: usize,
    expected_canonical_facts: usize,
    evidence_timing_constraints: usize,
    semantic_timing_constraints: usize,
    intent_timing_constraints: usize,
    intent_true_positives: usize,
    intent_false_positives: usize,
    intent_unprovenanced_records: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplayBaseline {
    reviewed_revision: String,
    result_path: String,
    result_sha256: String,
    document_key: String,
    source: ReplaySourceIdentity,
    stages: ReplayStageIdentities,
    observed_table: ReplayTableObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplaySourceCopy {
    repository_relative_path: String,
    sha256: String,
    byte_count: u64,
    verified_equal_to_authority: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CurrentReplay {
    production_revision: String,
    timing_authority_revision: String,
    source_copy: ReplaySourceCopy,
    command: String,
    scratch_output_root: String,
    stages: ReplayStageIdentities,
    observed_table: ReplayTableObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplayComparison {
    current_binary_replayed_documents: usize,
    reviewed_documents: usize,
    intent_false_positive_delta: i64,
    intent_unprovenanced_delta: i64,
    expected_true_positives_before: usize,
    expected_true_positives_after: usize,
    interpretation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplayCleanup {
    source_copy: String,
    scratch_roots: Vec<String>,
    status: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct CurrentReplayEvidence {
    schema_version: u32,
    replay_id: String,
    owner: String,
    baseline: ReplayBaseline,
    current_replay: CurrentReplay,
    comparison: ReplayComparison,
    cleanup: ReplayCleanup,
}

#[derive(Debug, Clone, Copy)]
struct DerivedCounts {
    documents: usize,
    cells: usize,
    categories: usize,
    supported_categories: usize,
    exact_source_regions: usize,
    exact_evidence_captures: usize,
    review_complete_documents: usize,
    intent_true_positives: usize,
    intent_false_positives: usize,
    intent_false_negatives: usize,
    capabilities: usize,
    integrated_capabilities: usize,
    non_omitted_capabilities: usize,
    executed_integrated_capabilities: usize,
    omitted_capabilities: usize,
}

/// Strictly load the provider-free `.2` capability observation.
pub fn load_capability_observation(path: &Path) -> Result<CapabilityObservation> {
    let bytes = read_repository_relative(path, "capability observation")?;
    let observation: CapabilityObservation = serde_json::from_slice(&bytes)?;
    validate_capability_observation(&observation).map_err(|problems| {
        AppError::InvalidStageArtifact(format!(
            "invalid trajectory capability observation: {}",
            problems.join("; ")
        ))
    })?;
    Ok(observation)
}

/// Validate the exact provider-free profile and complete 17-row `.2` ledger shape.
pub fn validate_capability_observation(
    observation: &CapabilityObservation,
) -> std::result::Result<(), Vec<String>> {
    let mut problems = Vec::new();
    if observation.schema_version != 1 {
        problems.push("schema_version must be 1".to_string());
    }
    if observation.observation_id.trim().is_empty() {
        problems.push("observation_id must not be empty".to_string());
    }
    if observation.profile.vlm_provider != "skip"
        || observation.profile.nlp_provider != "skip"
        || observation.profile.rescan_plan_supplied
        || observation.profile.promotion_executed
        || observation.profile.extraction_quality_measured
    {
        problems.push("observation must use the exact provider-free converge profile".to_string());
    }
    if observation.production_capabilities.len() != 17 {
        problems.push(format!(
            "provider-free ledger must contain 17 rows, got {}",
            observation.production_capabilities.len()
        ));
    }
    let mut ids = BTreeSet::new();
    for record in &observation.production_capabilities {
        if record.capability_id.trim().is_empty() || !ids.insert(record.capability_id.as_str()) {
            problems.push(format!(
                "capability_id must be non-empty and unique: '{}'",
                record.capability_id
            ));
        }
        for (field, value) in [
            ("command", record.command.as_str()),
            ("entrypoint", record.entrypoint.as_str()),
            ("reason", record.reason.as_str()),
        ] {
            if value.trim().is_empty() {
                problems.push(format!(
                    "capability '{}' {field} must not be empty",
                    record.capability_id
                ));
            }
        }
        if record.participation == CapabilityParticipation::Omitted
            && record.run_state != CapabilityRunState::NotExecuted
        {
            problems.push(format!(
                "omitted capability '{}' must be not_executed",
                record.capability_id
            ));
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems)
    }
}

fn load_current_replay_evidence(path: &Path) -> Result<CurrentReplayEvidence> {
    let bytes = read_repository_relative(path, "current-binary replay evidence")?;
    let replay: CurrentReplayEvidence = serde_json::from_slice(&bytes)?;
    validate_current_replay_evidence(&replay).map_err(|problems| {
        AppError::InvalidStageArtifact(format!(
            "invalid current-binary replay evidence: {}",
            problems.join("; ")
        ))
    })?;
    Ok(replay)
}

fn validate_current_replay_evidence(
    replay: &CurrentReplayEvidence,
) -> std::result::Result<(), Vec<String>> {
    let mut problems = Vec::new();
    if replay.schema_version != 1 {
        problems.push("schema_version must be 1".to_string());
    }
    if replay.owner != "SPEC-TO-INTENT-ALIGNMENT.6a" || replay.replay_id.trim().is_empty() {
        problems.push("replay identity must name owning leaf .6a".to_string());
    }
    if replay.baseline.reviewed_revision != REVIEWED_REVISION
        || replay.baseline.result_path != CURRENT_VERTICAL_RESULT_PATH
        || replay.baseline.result_sha256 != VERTICAL_RESULT_SHA256
        || replay.baseline.document_key != AIA_DOCUMENT_KEY
    {
        problems.push("baseline identity must match the frozen .4c AIA authority".to_string());
    }
    if replay.baseline.source.sha256 != AIA_SOURCE_SHA256
        || replay.current_replay.source_copy.sha256 != AIA_SOURCE_SHA256
        || replay.baseline.source.byte_count != replay.current_replay.source_copy.byte_count
        || !replay
            .current_replay
            .source_copy
            .verified_equal_to_authority
    {
        problems
            .push("replay source copy must be byte-identical to reviewed authority".to_string());
    }
    if replay.current_replay.timing_authority_revision != TIMING_AUTHORITY_REVISION
        || !is_git_revision(&replay.current_replay.production_revision)
    {
        problems.push("replay must name the production and timing-authority revisions".to_string());
    }
    validate_stage_hashes(&replay.baseline.stages, "baseline", &mut problems);
    validate_stage_hashes(
        &replay.current_replay.stages,
        "current replay",
        &mut problems,
    );
    let baseline = &replay.baseline.observed_table;
    if baseline.table_id != "table_0004"
        || baseline.table_kind != TableKind::TimingParameter
        || baseline.row_count != 20
        || baseline.column_count != 2
        || baseline.expected_canonical_facts != 0
        || baseline.evidence_timing_constraints != 19
        || baseline.semantic_timing_constraints != 19
        || baseline.intent_timing_constraints != 19
        || baseline.intent_true_positives != 0
        || baseline.intent_false_positives != 19
        || baseline.intent_unprovenanced_records != 19
    {
        problems.push("baseline must retain the exact reviewed 19-record TOC defect".to_string());
    }
    let current = &replay.current_replay.observed_table;
    if current.table_id != "table_0004"
        || current.table_kind != TableKind::Unknown
        || current.row_count != 20
        || current.column_count != 2
        || current.expected_canonical_facts != 0
        || current.evidence_timing_constraints != 0
        || current.semantic_timing_constraints != 0
        || current.intent_timing_constraints != 0
        || current.intent_true_positives != 0
        || current.intent_false_positives != 0
        || current.intent_unprovenanced_records != 0
    {
        problems.push(
            "current replay must retain the TOC shape while promoting zero timing facts"
                .to_string(),
        );
    }
    if replay.comparison.current_binary_replayed_documents != 1
        || replay.comparison.reviewed_documents != 12
        || replay.comparison.intent_false_positive_delta != -19
        || replay.comparison.intent_unprovenanced_delta != -19
        || replay.comparison.expected_true_positives_before != 0
        || replay.comparison.expected_true_positives_after != 0
        || replay.comparison.interpretation.trim().is_empty()
    {
        problems.push("comparison must state the exact 1/12, 19-to-zero result".to_string());
    }
    for path in [
        replay
            .current_replay
            .source_copy
            .repository_relative_path
            .as_str(),
        replay.current_replay.scratch_output_root.as_str(),
        replay.cleanup.source_copy.as_str(),
    ]
    .into_iter()
    .chain(replay.cleanup.scratch_roots.iter().map(String::as_str))
    {
        if !is_safe_relative_path(path) || !path.starts_with(".project-data/tmp/") {
            problems.push(format!(
                "replay scratch path is not repository-local: {path}"
            ));
        }
    }
    if replay.cleanup.status != "removed_and_residue_absent"
        || replay.cleanup.scratch_roots.len() != 2
    {
        problems.push("replay scratch cleanup must be complete and residue-free".to_string());
    }
    if replay.current_replay.command.trim().is_empty() {
        problems.push("replay reproduction command must not be empty".to_string());
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems)
    }
}

fn validate_stage_hashes(stages: &ReplayStageIdentities, label: &str, problems: &mut Vec<String>) {
    for (stage, digest) in [
        ("SourceIR", stages.source_ir_sha256.as_str()),
        ("EvidenceIR", stages.evidence_ir_sha256.as_str()),
        ("SemanticIR", stages.semantic_ir_sha256.as_str()),
        ("IntentIR", stages.intent_ir_sha256.as_str()),
    ] {
        if !is_sha256_digest(digest) {
            problems.push(format!("{label} {stage} hash must be lowercase SHA-256"));
        }
    }
}

fn is_sha256_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn is_git_revision(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn is_safe_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !value.is_empty()
        && !path.is_absolute()
        && !path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
}

/// Derive the current controller input from the frozen `.4c`, `.2`, and `.6a` authorities.
pub fn build_current_controller_input() -> Result<TrajectoryControllerInput> {
    let result_bytes = read_repository_relative(
        Path::new(CURRENT_VERTICAL_RESULT_PATH),
        "vertical result snapshot",
    )?;
    let result: VerticalEvalReport = serde_json::from_slice(&result_bytes)?;
    let capability = load_capability_observation(Path::new(CURRENT_CAPABILITY_OBSERVATION_PATH))?;
    let replay = load_current_replay_evidence(Path::new(CURRENT_REPLAY_EVIDENCE_PATH))?;
    let counts = derive_counts(&result, &capability)?;

    let result_evidence = evidence(
        CURRENT_VERTICAL_RESULT_PATH,
        VERTICAL_RESULT_SHA256,
        "frozen .4c reviewed vertical result",
    );
    let capability_evidence = evidence(
        CURRENT_CAPABILITY_OBSERVATION_PATH,
        CAPABILITY_OBSERVATION_SHA256,
        "provider-free .2 production-capability observation",
    );
    let objective_contract = evidence(
        OBJECTIVE_CONTRACT_PATH,
        OBJECTIVE_CONTRACT_SHA256,
        "category-aware source-to-IntentIR objective contract",
    );
    let replay_evidence = evidence(
        CURRENT_REPLAY_EVIDENCE_PATH,
        REPLAY_EVIDENCE_SHA256,
        "isolated .6a current-binary replay of the dominant frozen defect",
    );

    let intent_actual = counts.intent_true_positives + counts.intent_false_positives;
    let intent_expected = counts.intent_true_positives + counts.intent_false_negatives;
    let dimensions = vec![
        dimension(
            TrajectoryDimension::SourceCapture,
            "prove that each bounded source region and required modality is observable before semantic scoring",
            vec![
                metric(
                    "exact_source_region_capture",
                    "are all reviewed intent-bearing source regions found exactly",
                    counts.exact_source_regions,
                    counts.cells,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::HigherIsBetter,
                    false,
                    "complete bounded source-region gold",
                    "14 reviewed document-family-modality cells",
                    &result_evidence,
                ),
                metric(
                    "exact_required_modality_capture",
                    "are all reviewed required-modality captures found exactly",
                    counts.exact_evidence_captures,
                    counts.cells,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::HigherIsBetter,
                    false,
                    "complete bounded evidence-capture gold",
                    "14 reviewed document-family-modality cells",
                    &result_evidence,
                ),
            ],
        ),
        dimension(
            TrajectoryDimension::SemanticCorrectness,
            "retain only canonical IntentIR facts that match complete reviewed gold",
            vec![metric(
                "intent_canonical_precision",
                "what fraction of emitted canonical IntentIR facts is correct",
                counts.intent_true_positives,
                intent_actual,
                TargetOperator::Equal,
                1,
                1,
                ImprovementDirection::HigherIsBetter,
                false,
                "complete bounded canonical IntentIR answer keys",
                "all emitted canonical facts in 14 reviewed cells",
                &result_evidence,
            )],
        ),
        dimension(
            TrajectoryDimension::SemanticCompleteness,
            "preserve every expected canonical fact and support every reviewed category",
            vec![
                metric(
                    "intent_canonical_recall",
                    "what fraction of expected canonical facts reaches IntentIR correctly",
                    counts.intent_true_positives,
                    intent_expected,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::HigherIsBetter,
                    false,
                    "complete bounded canonical IntentIR answer keys",
                    "40 expected canonical facts in 14 reviewed cells",
                    &result_evidence,
                ),
                metric(
                    "supported_reviewed_categories",
                    "how many reviewed dominant-purpose categories meet the completeness contract",
                    counts.supported_categories,
                    counts.categories,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::HigherIsBetter,
                    false,
                    "category-aware completeness status",
                    "six reviewed dominant-purpose categories",
                    &result_evidence,
                ),
            ],
        ),
        dimension(
            TrajectoryDimension::StageConservation,
            "conserve each expected fact across every required boundary or explain it with a typed residual",
            vec![metric(
                "stage_conservation_or_residual",
                "what fraction of required boundary crossings is conserved or residualized",
                result.global.stage_conservation_or_residual.met,
                result.global.stage_conservation_or_residual.total,
                TargetOperator::Equal,
                1,
                1,
                ImprovementDirection::HigherIsBetter,
                false,
                "exact three-boundary vertical conservation accounting",
                "54 required stage-boundary crossings",
                &result_evidence,
            )],
        ),
        dimension(
            TrajectoryDimension::ProvenanceHonesty,
            "forbid fabricated canonical facts and retain source provenance on every promoted fact",
            vec![
                metric(
                    "canonical_provenance_closure",
                    "what fraction of emitted canonical facts closes required provenance",
                    result.global.canonical_provenance_closure.met,
                    result.global.canonical_provenance_closure.total,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::HigherIsBetter,
                    false,
                    "complete bounded provenance requirements",
                    "all emitted canonical records in 14 reviewed cells",
                    &result_evidence,
                ),
                metric(
                    "fabricated_canonical_fact_rate",
                    "what fraction of emitted canonical facts is fabricated",
                    result.global.fabricated_canonical_facts,
                    intent_actual,
                    TargetOperator::Equal,
                    0,
                    1,
                    ImprovementDirection::LowerIsBetter,
                    false,
                    "complete bounded canonical IntentIR answer keys",
                    "all emitted canonical facts in 14 reviewed cells",
                    &result_evidence,
                ),
            ],
        ),
        dimension(
            TrajectoryDimension::ProductionParticipation,
            "account for every production capability without implying that omitted islands participate",
            vec![
                metric(
                    "production_capability_accounting",
                    "is every production capability classified in the per-run ledger",
                    counts.capabilities,
                    counts.capabilities,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::ExactInvariant,
                    true,
                    "complete Clap-derived production-command partition",
                    "17 guarded production-capability rows",
                    &capability_evidence,
                ),
                metric(
                    "non_omitted_production_participation",
                    "what fraction of production capabilities is integrated or explicitly scheduled",
                    counts.non_omitted_capabilities,
                    counts.capabilities,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::HigherIsBetter,
                    false,
                    "provider-free canonical converge capability ledger",
                    "17 guarded production-capability rows",
                    &capability_evidence,
                ),
            ],
        ),
        dimension(
            TrajectoryDimension::GeneralizationRobustness,
            "retain reviewed oracle coverage across every supported dominant-purpose category",
            vec![metric(
                "reviewed_category_oracle_coverage",
                "does the reviewed population cover every category in the objective contract",
                counts.categories,
                6,
                TargetOperator::Equal,
                1,
                1,
                ImprovementDirection::HigherIsBetter,
                false,
                "balanced review-locked category population",
                "six required dominant-purpose categories",
                &result_evidence,
            )],
        ),
        dimension(
            TrajectoryDimension::OperationalConfidence,
            "separate complete reviewed oracles, artifact currency, and context-dependent execution availability",
            vec![
                metric(
                    "complete_review_scope_documents",
                    "how many evaluated documents have complete bounded review scope",
                    counts.review_complete_documents,
                    counts.documents,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::HigherIsBetter,
                    false,
                    "review-lock declarations in the frozen result",
                    "12 reviewed documents",
                    &result_evidence,
                ),
                metric(
                    "current_binary_replay_coverage",
                    "what fraction of the reviewed population has been replayed with the current binary",
                    replay.comparison.current_binary_replayed_documents,
                    replay.comparison.reviewed_documents,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::HigherIsBetter,
                    true,
                    "hash-pinned isolated four-stage current-binary replay",
                    "12 documents in the frozen reviewed population",
                    &replay_evidence,
                ),
                metric(
                    "provider_free_integrated_execution",
                    "what fraction of integrated capabilities executed in the provider-free observation",
                    counts.executed_integrated_capabilities,
                    counts.integrated_capabilities,
                    TargetOperator::Equal,
                    1,
                    1,
                    ImprovementDirection::HigherIsBetter,
                    false,
                    "provider-free canonical converge capability ledger",
                    "10 capabilities classified integrated in this run profile",
                    &capability_evidence,
                ),
            ],
        ),
        dimension(
            TrajectoryDimension::ExecutableReadiness,
            "require each reviewed document to account for its intent before downstream lowering is credited",
            vec![metric(
                "required_modality_document_accounting",
                "how many reviewed documents completely account for required modalities",
                result.global.required_modality_accounting.met,
                result.global.required_modality_accounting.total,
                TargetOperator::Equal,
                1,
                1,
                ImprovementDirection::HigherIsBetter,
                false,
                "category-aware document completeness contract",
                "12 reviewed documents",
                &result_evidence,
            )],
        ),
    ];

    let gaps = vec![
        gap(
            "current-binary-honesty-qualification",
            "operational_confidence",
            GapPriorityTier::HardInvariant,
            format!(
                "the frozen baseline has {} fabricated facts and {} provenance failures, but only {} of {} reviewed documents has current-binary replay evidence; that replay removes the largest 19-record family",
                result.global.fabricated_canonical_facts,
                result.global.canonical_provenance_closure.total
                    - result.global.canonical_provenance_closure.met,
                replay.comparison.current_binary_replayed_documents,
                replay.comparison.reviewed_documents,
            ),
            "current-binary replay covers all 12 reviewed documents before remaining fabrication or provenance repairs are ranked",
            "artifact_currency_before_semantic_diagnosis",
            replay.comparison.reviewed_documents
                - replay.comparison.current_binary_replayed_documents,
            CausalConfidence::High,
            ReversibleSliceSize::Medium,
            GapUncertainty::Exact,
            "establish current product truth, then repair only defects the current binary still reproduces",
            "cargo run --quiet -p specforge --example source_to_intent_replay -- <repository-relative-source> <.project-data/tmp/output-root> generated/prior_memory/corpus_memory.json <observed-table-id-or->",
            "SPEC-TO-INTENT-ALIGNMENT.6",
            &[result_evidence.clone(), replay_evidence.clone()],
        ),
        gap(
            "source-to-evidence-canonical-loss",
            "stage_conservation",
            GapPriorityTier::SourceEvidenceLoss,
            format!(
                "{} expected canonical facts disappear before EvidenceIR without disposition",
                result.global.unexplained_stage_drops
            ),
            "every expected canonical fact is conserved or explicitly residualized",
            "source_to_evidence_ir",
            result.global.unexplained_stage_drops,
            CausalConfidence::High,
            ReversibleSliceSize::Medium,
            GapUncertainty::Exact,
            "recover the dominant measured loss boundary after the honesty invariant",
            "cargo test -p specforge --lib ir::source_to_intent_eval::tests::reviewed_result_snapshot_is_current_and_names_the_upstream_loss_boundary",
            "SPEC-TO-INTENT-ALIGNMENT.7",
            std::slice::from_ref(&result_evidence),
        ),
        gap(
            "non-actionable-required-residuals",
            "stage_conservation",
            GapPriorityTier::PersistentResidual,
            format!(
                "{} of {} required residual observations are actionable",
                result.global.residual_actionability.met,
                result.global.residual_actionability.total
            ),
            "every required residual is typed, source-linked, and actionable",
            "evidence_to_semantic_ir",
            result.global.residual_actionability.total,
            CausalConfidence::High,
            ReversibleSliceSize::Medium,
            GapUncertainty::Exact,
            "make promotion losses operable without disguising missing canonical facts",
            "cargo test -p specforge --lib ir::source_to_intent_eval",
            "SPEC-TO-INTENT-ALIGNMENT.8",
            std::slice::from_ref(&result_evidence),
        ),
        gap(
            "omitted-production-capability-islands",
            "production_participation",
            GapPriorityTier::BreadthEfficiency,
            format!(
                "{} of {} production capabilities are explicitly omitted in provider-free converge",
                counts.omitted_capabilities, counts.capabilities
            ),
            "integrate or explicitly schedule each capability whose reviewed value is demonstrated",
            "canonical_converge_orchestration",
            counts.omitted_capabilities,
            CausalConfidence::Medium,
            ReversibleSliceSize::Large,
            GapUncertainty::Bounded,
            "measure and compose capability breadth only after semantic honesty and conservation",
            "cargo test -p specforge --lib commands::converge::tests::provider_free_capability_report_names_every_current_capability_island",
            "SPEC-TO-INTENT-ALIGNMENT.9",
            std::slice::from_ref(&capability_evidence),
        ),
    ];

    Ok(TrajectoryControllerInput {
        schema_version: 1,
        snapshot_id: "specforge-source-to-intent-reviewed-v2".to_string(),
        owner: "SPEC-TO-INTENT-ALIGNMENT.6a".to_string(),
        reviewed_revision: REVIEWED_REVISION.to_string(),
        objective_contract,
        stall_window: 3,
        dimensions,
        hard_gates: vec![hard_gate(
            "complete_current_binary_replay",
            "does every reviewed document have a hash-pinned current-binary replay",
            replay.comparison.reviewed_documents
                - replay.comparison.current_binary_replayed_documents,
            &replay_evidence,
        )],
        history: Vec::new(),
        gaps,
        authority: ControllerAuthority {
            mode: ControllerMode::ReportOnly,
            canonical_semantic_mutation_allowed: false,
            task_tree_review_required: true,
        },
    })
}

/// Evaluate the first reviewed snapshot with the generic `.5a` controller.
pub fn evaluate_current_trajectory() -> Result<TrajectoryReport> {
    let input = build_current_controller_input()?;
    evaluate_trajectory(&input).map_err(|problems| {
        AppError::InvalidStageArtifact(format!(
            "invalid composed trajectory input: {}",
            problems.join("; ")
        ))
    })
}

/// Render stable pretty JSON with the repository's required terminal newline.
pub fn pretty_json_bytes<T: Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut bytes = serde_json::to_vec_pretty(value)?;
    bytes.push(b'\n');
    Ok(bytes)
}

/// Regenerate the two derived snapshot artifacts below the repository root.
pub fn write_current_trajectory_artifacts() -> Result<()> {
    let repository = crate::project_data::repository_root()?;
    let input = build_current_controller_input()?;
    let report = evaluate_current_trajectory()?;
    fs::write(
        repository.join(CURRENT_CONTROLLER_INPUT_PATH),
        pretty_json_bytes(&input)?,
    )?;
    fs::write(
        repository.join(CURRENT_TRAJECTORY_REPORT_PATH),
        pretty_json_bytes(&report)?,
    )?;
    Ok(())
}

/// Fail unless both tracked trajectory artifacts equal a fresh composition byte for byte.
pub fn check_current_trajectory_artifacts() -> Result<()> {
    let repository = crate::project_data::repository_root()?;
    let expected = [
        (
            CURRENT_CONTROLLER_INPUT_PATH,
            pretty_json_bytes(&build_current_controller_input()?)?,
        ),
        (
            CURRENT_TRAJECTORY_REPORT_PATH,
            pretty_json_bytes(&evaluate_current_trajectory()?)?,
        ),
    ];
    for (path, expected_bytes) in expected {
        let actual = fs::read(repository.join(path))?;
        if actual != expected_bytes {
            return Err(AppError::InvalidStageArtifact(format!(
                "trajectory artifact is stale: {path}"
            )));
        }
    }
    Ok(())
}

fn derive_counts(
    result: &VerticalEvalReport,
    capability: &CapabilityObservation,
) -> Result<DerivedCounts> {
    let cells: Vec<_> = result
        .documents
        .iter()
        .flat_map(|document| &document.cells)
        .collect();
    let canonical: Vec<_> = cells
        .iter()
        .filter_map(|cell| cell.canonical.as_ref())
        .collect();
    let intent_true_positives = canonical
        .iter()
        .map(|scores| scores.intent_ir.true_positives)
        .sum();
    let intent_false_positives = canonical
        .iter()
        .map(|scores| scores.intent_ir.false_positives)
        .sum();
    let intent_false_negatives = canonical
        .iter()
        .map(|scores| scores.intent_ir.false_negatives)
        .sum();
    let integrated_capabilities = capability
        .production_capabilities
        .iter()
        .filter(|record| record.participation == CapabilityParticipation::Integrated)
        .count();
    let counts = DerivedCounts {
        documents: result.documents.len(),
        cells: cells.len(),
        categories: result.categories.len(),
        supported_categories: result
            .categories
            .iter()
            .filter(|category| category.status == CategoryStatus::Supported)
            .count(),
        exact_source_regions: cells
            .iter()
            .filter(|cell| query_is_exact(&cell.source_region))
            .count(),
        exact_evidence_captures: cells
            .iter()
            .filter(|cell| query_is_exact(&cell.evidence_capture))
            .count(),
        review_complete_documents: result
            .documents
            .iter()
            .filter(|document| document.review_scope_complete)
            .count(),
        intent_true_positives,
        intent_false_positives,
        intent_false_negatives,
        capabilities: capability.production_capabilities.len(),
        integrated_capabilities,
        non_omitted_capabilities: capability
            .production_capabilities
            .iter()
            .filter(|record| record.participation != CapabilityParticipation::Omitted)
            .count(),
        executed_integrated_capabilities: capability
            .production_capabilities
            .iter()
            .filter(|record| {
                record.participation == CapabilityParticipation::Integrated
                    && record.run_state == CapabilityRunState::Executed
            })
            .count(),
        omitted_capabilities: capability
            .production_capabilities
            .iter()
            .filter(|record| record.participation == CapabilityParticipation::Omitted)
            .count(),
    };
    let expected = [
        ("documents", counts.documents, 12),
        ("cells", counts.cells, 14),
        ("categories", counts.categories, 6),
        ("capabilities", counts.capabilities, 17),
    ];
    let mismatches: Vec<_> = expected
        .into_iter()
        .filter(|(_, actual, expected)| actual != expected)
        .map(|(label, actual, expected)| format!("{label}: expected {expected}, got {actual}"))
        .collect();
    if !mismatches.is_empty() {
        return Err(AppError::InvalidStageArtifact(format!(
            "current trajectory authority shape changed: {}",
            mismatches.join("; ")
        )));
    }
    Ok(counts)
}

fn query_is_exact(score: &QueryScore) -> bool {
    score.false_positives == 0 && score.false_negatives == 0 && score.unprovenanced_records == 0
}

fn dimension(
    dimension: TrajectoryDimension,
    objective: &str,
    metrics: Vec<MetricObservation>,
) -> DimensionObservation {
    DimensionObservation {
        dimension,
        objective: objective.to_string(),
        metrics,
    }
}

#[allow(clippy::too_many_arguments)]
fn metric(
    metric_id: &str,
    question: &str,
    numerator: usize,
    denominator: usize,
    target_operator: TargetOperator,
    target_numerator: u64,
    target_denominator: u64,
    improvement: ImprovementDirection,
    hard: bool,
    oracle: &str,
    population: &str,
    evidence: &EvidenceRef,
) -> MetricObservation {
    MetricObservation {
        metric_id: metric_id.to_string(),
        question: question.to_string(),
        measure: Some(fraction(numerator, denominator)),
        target: MetricTarget {
            operator: target_operator,
            value: Fraction::new(target_numerator, target_denominator),
        },
        improvement,
        material_change: Fraction::new(1, 100),
        hard,
        required: true,
        oracle: oracle.to_string(),
        population: population.to_string(),
        uncertainty: Uncertainty::Exact,
        evidence: vec![evidence.clone()],
    }
}

#[allow(clippy::too_many_arguments)]
fn gap(
    gap_id: &str,
    objective_id: &str,
    tier: GapPriorityTier,
    observed: String,
    target: &str,
    first_failing_stage: &str,
    affected_population: usize,
    causal_confidence: CausalConfidence,
    reversible_slice: ReversibleSliceSize,
    uncertainty: GapUncertainty,
    estimated_impact: &str,
    reproduction: &str,
    task_id: &str,
    evidence: &[EvidenceRef],
) -> TrajectoryGap {
    TrajectoryGap {
        gap_id: gap_id.to_string(),
        objective_id: objective_id.to_string(),
        tier,
        observed,
        target: target.to_string(),
        first_failing_stage: first_failing_stage.to_string(),
        affected_population,
        causal_confidence,
        reversible_slice,
        uncertainty,
        estimated_impact: estimated_impact.to_string(),
        reproduction: reproduction.to_string(),
        owner: TaskOwner {
            task_id: task_id.to_string(),
            task_tree_path: TASK_TREE_PATH.to_string(),
        },
        evidence: evidence.to_vec(),
    }
}

fn hard_gate(
    gate_id: &str,
    question: &str,
    violations: usize,
    evidence: &EvidenceRef,
) -> HardGateObservation {
    HardGateObservation {
        gate_id: gate_id.to_string(),
        question: question.to_string(),
        violations: violations as u64,
        target_violations: 0,
        evidence: vec![evidence.clone()],
    }
}

fn evidence(path: &str, sha256: &str, role: &str) -> EvidenceRef {
    EvidenceRef {
        path: path.to_string(),
        sha256: sha256.to_string(),
        role: role.to_string(),
    }
}

fn fraction(numerator: usize, denominator: usize) -> Fraction {
    Fraction::new(numerator as u64, denominator as u64)
}

fn read_repository_relative(path: &Path, label: &str) -> Result<Vec<u8>> {
    let path_text = path.to_string_lossy();
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(AppError::InvalidStageArtifact(format!(
            "{label} path must be repository-relative: {path_text}"
        )));
    }
    let repository = crate::project_data::repository_root()?;
    let absolute = repository.join(path).canonicalize()?;
    if !absolute.starts_with(&repository) {
        return Err(AppError::InvalidStageArtifact(format!(
            "{label} path escapes the repository: {path_text}"
        )));
    }
    Ok(fs::read(absolute)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::trajectory::{DimensionStatus, HistoryStatus, TrajectoryState};

    #[test]
    fn current_snapshot_derives_exact_authority_counts_and_hard_first_proposal() -> Result<()> {
        let input = build_current_controller_input()?;
        let report = evaluate_current_trajectory()?;

        assert_eq!(input.dimensions.len(), 9);
        assert!(input.history.is_empty());
        assert_eq!(report.state, TrajectoryState::Diverging);
        assert_eq!(report.history_status, HistoryStatus::InsufficientHistory);
        assert!(
            report
                .state_reasons
                .contains(&"hard_gate_failed:complete_current_binary_replay:11>0".to_string())
        );
        assert_eq!(
            report
                .recommendation
                .as_ref()
                .map(|proposal| proposal.task_id.as_str()),
            Some("SPEC-TO-INTENT-ALIGNMENT.6")
        );
        assert_eq!(report.ranked_gaps.len(), 4);
        assert_eq!(report.ranked_gaps[0].tier, GapPriorityTier::HardInvariant);
        assert_eq!(
            report.ranked_gaps[1].tier,
            GapPriorityTier::SourceEvidenceLoss
        );
        assert_eq!(
            report.ranked_gaps[2].tier,
            GapPriorityTier::PersistentResidual
        );
        assert_eq!(
            report.ranked_gaps[3].tier,
            GapPriorityTier::BreadthEfficiency
        );
        assert!(
            report
                .dimensions
                .iter()
                .any(|dimension| dimension.status == DimensionStatus::MeetsTarget)
        );
        assert!(
            report
                .dimensions
                .iter()
                .any(|dimension| dimension.status == DimensionStatus::Deficit)
        );
        let measure = |metric_id: &str| {
            report
                .dimensions
                .iter()
                .flat_map(|dimension| &dimension.metrics)
                .find(|metric| metric.metric_id == metric_id)
                .and_then(|metric| metric.measure)
                .unwrap_or_else(|| panic!("missing exact metric: {metric_id}"))
        };
        for (metric_id, expected) in [
            ("exact_source_region_capture", Fraction::new(14, 14)),
            ("intent_canonical_precision", Fraction::new(7, 48)),
            ("intent_canonical_recall", Fraction::new(7, 40)),
            ("stage_conservation_or_residual", Fraction::new(21, 54)),
            ("canonical_provenance_closure", Fraction::new(3, 48)),
            ("production_capability_accounting", Fraction::new(17, 17)),
            (
                "non_omitted_production_participation",
                Fraction::new(12, 17),
            ),
            ("provider_free_integrated_execution", Fraction::new(5, 10)),
            (
                "required_modality_document_accounting",
                Fraction::new(0, 12),
            ),
            ("current_binary_replay_coverage", Fraction::new(1, 12)),
        ] {
            assert_eq!(measure(metric_id), expected, "metric {metric_id}");
        }
        Ok(())
    }

    #[test]
    fn persisted_controller_input_and_report_are_byte_current() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let input = build_current_controller_input()?;
        let report = evaluate_current_trajectory()?;
        assert_eq!(
            fs::read(repository.join(CURRENT_CONTROLLER_INPUT_PATH))?,
            pretty_json_bytes(&input)?
        );
        assert_eq!(
            fs::read(repository.join(CURRENT_TRAJECTORY_REPORT_PATH))?,
            pretty_json_bytes(&report)?
        );
        Ok(())
    }

    #[test]
    fn capability_observation_rejects_profile_and_row_mutants() -> Result<()> {
        let mut observation =
            load_capability_observation(Path::new(CURRENT_CAPABILITY_OBSERVATION_PATH))?;
        observation.profile.nlp_provider = "ollama".to_string();
        assert!(
            validate_capability_observation(&observation)
                .expect_err("live provider mutant must fail")
                .iter()
                .any(|problem| problem.contains("provider-free"))
        );

        let mut observation =
            load_capability_observation(Path::new(CURRENT_CAPABILITY_OBSERVATION_PATH))?;
        observation.production_capabilities.pop();
        assert!(
            validate_capability_observation(&observation)
                .expect_err("missing row mutant must fail")
                .iter()
                .any(|problem| problem.contains("17 rows"))
        );
        Ok(())
    }

    #[test]
    fn current_replay_evidence_rejects_currency_and_preservation_mutants() -> Result<()> {
        let replay = load_current_replay_evidence(Path::new(CURRENT_REPLAY_EVIDENCE_PATH))?;

        let mut incomplete = replay.clone();
        incomplete.cleanup.status = "pending".to_string();
        assert!(
            validate_current_replay_evidence(&incomplete)
                .expect_err("unclean replay evidence must fail")
                .iter()
                .any(|problem| problem.contains("cleanup"))
        );

        let mut fabricated = replay.clone();
        fabricated
            .current_replay
            .observed_table
            .intent_false_positives = 1;
        assert!(
            validate_current_replay_evidence(&fabricated)
                .expect_err("a surviving TOC fabrication must fail")
                .iter()
                .any(|problem| problem.contains("promoting zero"))
        );

        let mut lost = replay;
        lost.comparison.expected_true_positives_after = 1;
        assert!(
            validate_current_replay_evidence(&lost)
                .expect_err("a changed reviewed true-positive population must fail")
                .iter()
                .any(|problem| problem.contains("19-to-zero"))
        );
        Ok(())
    }
}
