//! Whole-document behavioral-genericity qualification downstream of the production core.
//!
//! The harness copies one hash-pinned source into repository-local scratch, applies only a
//! declared deterministic transform, executes the current five-stage pipeline, and compares every
//! serialized field and proof claim. Transform recipes and expected relations never flow into core.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::error::{AppError, Result};
use crate::ir::adapters::{AdapterArtifact, AdapterTarget};
use crate::ir::entity_typing::declared_signal_catalog;
use crate::ir::evidence::EvidenceIr;
use crate::ir::intent::IntentIr;
use crate::ir::semantic::SemanticIr;
use crate::ir::source::{SourceIr, inspect_docling_runtime};

const BEHAVIORAL_SCHEMA_VERSION: u32 = 3;
const HELD_OUT_SCHEMA_VERSION: u32 = 2;
const CONTRACT_PATH: &str = "doctrine/production_genericity/behavioral_qualification.json";
const POPULATION_PATH: &str = "doctrine/production_genericity/behavioral_population.tsv";
const REVIEW_RECIPE_MANIFEST_PATH: &str =
    "doctrine/production_genericity/reviewed_recipe_manifest.json";
const REVIEW_RECIPE_ROOT: &str = "doctrine/production_genericity/reviewed_recipes";
const NEGATIVE_SENSITIVITY_MATRIX_PATH: &str =
    "doctrine/production_genericity/semantic_negative_matrix.json";
const TEMP_ROOT: &str = ".project-data/tmp";
const EVIDENCE_FILE: &str = "behavioral_evidence.json";
pub const HELD_OUT_EVIDENCE_FILE: &str = "behavioral_holdout_evidence.json";
const GROUPED_INTERFACE_PREFIX: &str = "semantic channel inferred from grouped interface signals: ";
const REVIEWED_RECIPE_SCHEMA_VERSION: u32 = 1;
const REVIEWED_COMPLEMENT: &str = "all_unlisted_leaf_values_and_all_proof_topology_exact";

const REVIEWED_PROVENANCE_FIELDS: &[&str] = &[
    "conclusion",
    "normalized_markdown",
    "responsibilities",
    "source_text",
    "statement",
    "text",
];

const RICH_CAPTURE_EXCLUSIONS: &[&str] = &[
    "content_elements",
    "document_sections",
    "page_artifacts",
    "structured_tables",
    "visual_assets",
];

const FAMILIAR_IDENTIFIERS: &[&str] = &[
    "clk", "reset_n", "valid", "ready", "req", "ack", "data", "enable", "state", "master", "slave",
    "address",
];

const COLLECTION_KEY_FIELDS: &[&str] = &[
    "actor_name",
    "instance_name",
    "member_name",
    "module_name",
    "port_name",
    "signal_name",
    "state_name",
    "symbol_name",
];

const IDENTITY_HASH_EXCLUSIONS: &[&str] = &[
    "implementation_sha256",
    "prior_sha256",
    "ruleset_sha256",
    "validation_sha256",
];

const DERIVED_ID_EXCLUSIONS: &[&str] = &[
    "axiom_id",
    "derivation_id",
    "prior_id",
    "proposal_id",
    "region_id",
    "rule_id",
    "span_id",
    "table_id",
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehavioralStage {
    SourceIr,
    EvidenceIr,
    SemanticIr,
    IntentIr,
    IsfAdapter,
}

impl BehavioralStage {
    const ALL: [Self; 5] = [
        Self::SourceIr,
        Self::EvidenceIr,
        Self::SemanticIr,
        Self::IntentIr,
        Self::IsfAdapter,
    ];

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehavioralRelation {
    UnchangedSource,
    AdversarialIdentity,
    SymbolAlpha,
    StructurePreservingParaphrase,
    HarmlessLayout,
    SemanticNegative,
}

impl BehavioralRelation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnchangedSource => "unchanged_source",
            Self::AdversarialIdentity => "adversarial_identity",
            Self::SymbolAlpha => "symbol_alpha",
            Self::StructurePreservingParaphrase => "structure_preserving_paraphrase",
            Self::HarmlessLayout => "harmless_layout",
            Self::SemanticNegative => "semantic_negative",
        }
    }

    pub fn input_plane(self) -> &'static str {
        match self {
            Self::UnchangedSource | Self::AdversarialIdentity => "pdf_full_capture",
            Self::SymbolAlpha => "normalized_text_projection",
            Self::StructurePreservingParaphrase | Self::HarmlessLayout | Self::SemanticNegative => {
                "reviewed_variant"
            }
        }
    }

    fn is_reviewed(self) -> bool {
        matches!(
            self,
            Self::StructurePreservingParaphrase | Self::HarmlessLayout | Self::SemanticNegative
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehavioralRunState {
    Pass,
    Fail,
    Unmeasurable,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehavioralQualificationRequest {
    pub relation: BehavioralRelation,
    pub source_authority: PathBuf,
    pub expected_source_sha256: String,
    pub output_root: PathBuf,
    pub prior_memory: PathBuf,
    pub production_revision: String,
    pub transform_seed: u64,
    pub review_recipe_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceIdentity {
    pub contract_sha256: String,
    pub production_revision: String,
    pub source_sha256: String,
    pub transform_recipe_sha256: String,
    pub prior_memory_sha256: String,
    pub tool_sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceAuthorityEvidence {
    pub portable_id: String,
    pub sha256: String,
    pub byte_count: u64,
    pub copied_to_repository_scratch: bool,
    pub same_repository_filesystem: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SymbolRename {
    pub source_ordinal: usize,
    pub original: String,
    pub replacement: String,
    pub occurrence_count: usize,
    pub familiar_adversarial_spelling: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewedChangeKind {
    SentenceParaphrase,
    HeadingLayout,
    TableLayout,
    WhitespaceLayout,
    FormattingLayout,
    SemanticTimingChange,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticNegativeKind {
    TimingGrammarAdmission,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RequiredDeltaKind {
    Added,
    Changed,
    Removed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewedSpanEvidence {
    pub change_id: String,
    pub kind: ReviewedChangeKind,
    pub baseline_start_byte: usize,
    pub baseline_byte_count: usize,
    pub transformed_byte_count: usize,
    pub occurrence_count: usize,
    pub baseline_sha256: String,
    pub transformed_sha256: String,
    pub allowed_provenance_fields: Vec<String>,
    pub allow_source_bound_identifier_projection: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewedRecipeEvidence {
    pub recipe_id: String,
    pub manifest_path: String,
    pub recipe_path: String,
    pub recipe_sha256: String,
    pub review_status: String,
    pub changed_spans: Vec<ReviewedSpanEvidence>,
    pub preserved_conclusions: usize,
    pub unaffected_complement: String,
    pub unmeasurable_source_surfaces: Vec<String>,
    pub semantic_negative: Option<SemanticNegativeEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RequiredDeltaEvidence {
    pub delta_id: String,
    pub stage: BehavioralStage,
    pub kind: RequiredDeltaKind,
    pub baseline_pointer: Option<String>,
    pub transformed_pointer: Option<String>,
    pub observed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SemanticNegativeEvidence {
    pub kind: SemanticNegativeKind,
    pub required_deltas: Vec<RequiredDeltaEvidence>,
    pub dependent_proof_deltas: usize,
    pub invariant_comparison_rejected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransformEvidence {
    pub relation: BehavioralRelation,
    pub seed: u64,
    pub baseline_source: String,
    pub transformed_source: String,
    pub baseline_document_key: String,
    pub transformed_document_key: String,
    pub source_bytes_equal: bool,
    pub lexical_order_changed: bool,
    pub symbol_renames: Vec<SymbolRename>,
    pub reviewed_recipe: Option<ReviewedRecipeEvidence>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactIdentity {
    pub path: String,
    pub sha256: String,
    pub byte_count: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageComparison {
    pub stage: BehavioralStage,
    pub baseline: ArtifactIdentity,
    pub transformed: ArtifactIdentity,
    pub baseline_top_level_fields: usize,
    pub transformed_top_level_fields: usize,
    pub baseline_proof_claims: usize,
    pub transformed_proof_claims: usize,
    pub compared_leaf_values: usize,
    pub normalized_delta_paths: Vec<String>,
    pub declared_delta_paths: Vec<String>,
    pub baseline_declared_proof_deltas: usize,
    pub transformed_declared_proof_deltas: usize,
    pub undeclared_delta_paths: Vec<String>,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BehavioralCoverage {
    pub required_stages: usize,
    pub completed_stages: usize,
    pub baseline_top_level_fields: usize,
    pub transformed_top_level_fields: usize,
    pub baseline_proof_claims: usize,
    pub transformed_proof_claims: usize,
    pub compared_leaf_values: usize,
    pub expected_symbol_deltas: usize,
    pub observed_symbol_deltas: usize,
    pub expected_reviewed_span_deltas: usize,
    pub observed_reviewed_span_deltas: usize,
    pub preserved_conclusions: usize,
    pub expected_semantic_deltas: usize,
    pub observed_semantic_deltas: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BehavioralQualificationReport {
    pub schema_version: u32,
    pub relation: BehavioralRelation,
    pub input_plane: String,
    pub state: BehavioralRunState,
    pub identity: EvidenceIdentity,
    pub source: SourceAuthorityEvidence,
    pub transform: TransformEvidence,
    pub stages: Vec<StageComparison>,
    pub coverage: BehavioralCoverage,
    pub failures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BehavioralQualificationAttempt {
    pub schema_version: u32,
    pub relation: BehavioralRelation,
    pub input_plane: String,
    pub state: BehavioralRunState,
    pub failure_id: Option<String>,
    pub detail: Option<String>,
    pub report: Option<BehavioralQualificationReport>,
}

/// One deterministic execution request for the frozen prospective held-out population.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeldOutQualificationRequest {
    pub output_root: PathBuf,
    pub retained_output_root: Option<PathBuf>,
    pub prior_memory: PathBuf,
    pub production_revision: String,
    pub transform_seed: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HeldOutExecutionMode {
    FreshPipeline,
    RetainedArtifactsRecompared,
    RetainedReportRevalidated,
    EligibilityPreflight,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeldOutSplitEvidence {
    pub selection_boundary_commit: String,
    pub calibration_document_keys: Vec<String>,
    pub prospective_document_keys: Vec<String>,
    pub overlapping_document_keys: Vec<String>,
    pub overlapping_source_sha256: Vec<String>,
    pub overlapping_normalized_markdown_sha256: Vec<String>,
    pub identity_disjoint: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeldOutDocumentEvidence {
    pub document_key: String,
    pub source_origin: String,
    pub source_locator: String,
    pub source_sha256: String,
    pub normalized_markdown_path: String,
    pub normalized_markdown_sha256: String,
    pub vendor: String,
    pub family: String,
    pub category: String,
    pub layout: String,
    pub vendor_novel: bool,
    pub family_novel: bool,
    pub text_semantic_records: usize,
    pub text_intent_records: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeldOutAttemptEvidence {
    pub document_key: String,
    pub relation: BehavioralRelation,
    pub input_plane: String,
    pub transform_seed: u64,
    pub execution_mode: HeldOutExecutionMode,
    pub state: BehavioralRunState,
    pub failure_id: Option<String>,
    pub detail: Option<String>,
    pub identity: Option<EvidenceIdentity>,
    pub coverage: Option<BehavioralCoverage>,
    pub completed_stages: usize,
    pub all_completed_stages_passed: bool,
    pub attempt_report_sha256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeldOutUncertainty {
    pub method: String,
    pub confidence_basis_points: Option<u32>,
    pub sample_size: usize,
    pub pass_numerator: usize,
    pub point_estimate_parts_per_million: Option<u32>,
    pub lower_parts_per_million: Option<u32>,
    pub upper_parts_per_million: Option<u32>,
    pub scope_limit: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeldOutStratumOutcome {
    pub relation: BehavioralRelation,
    pub dimension: String,
    pub value: String,
    pub document_keys: Vec<String>,
    pub declared_documents: usize,
    pub passes: usize,
    pub failures: usize,
    pub unmeasurable: usize,
    pub invalid: usize,
    pub state: BehavioralRunState,
    pub limitation: Option<String>,
    pub uncertainty: HeldOutUncertainty,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeldOutAggregateCoverage {
    pub declared_documents: Vec<String>,
    pub attempted_documents: Vec<String>,
    pub fully_completed_documents: Vec<String>,
    pub partially_completed_documents: Vec<String>,
    pub unmeasurable_documents: Vec<String>,
    pub invalid_documents: Vec<String>,
    pub declared_attempts: usize,
    pub pass_attempts: usize,
    pub fail_attempts: usize,
    pub unmeasurable_attempts: usize,
    pub invalid_attempts: usize,
    pub baseline_top_level_fields: usize,
    pub transformed_top_level_fields: usize,
    pub baseline_proof_claims: usize,
    pub transformed_proof_claims: usize,
    pub compared_leaf_values: usize,
    pub expected_deltas: usize,
    pub observed_deltas: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HeldOutQualificationReport {
    pub schema_version: u32,
    pub owner: String,
    pub contract_path: String,
    pub contract_sha256: String,
    pub population_path: String,
    pub population_sha256: String,
    pub production_revision: String,
    pub prior_memory_sha256: String,
    pub tool_sha256: String,
    pub retained_tool_sha256: Option<String>,
    pub retained_evidence_path: Option<String>,
    pub retained_evidence_sha256: Option<String>,
    pub leakage_boundary: String,
    pub eligible_relations: Vec<BehavioralRelation>,
    pub split: HeldOutSplitEvidence,
    pub documents: Vec<HeldOutDocumentEvidence>,
    pub attempts: Vec<HeldOutAttemptEvidence>,
    pub strata: Vec<HeldOutStratumOutcome>,
    pub coverage: HeldOutAggregateCoverage,
    pub final_signoff_deferred: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NegativeControlKind {
    Omission,
    Contradiction,
    RelationReversal,
    ValueChange,
    TimingChange,
    UndeclaredSymbol,
    MisleadingName,
    ProofCorruption,
    DisabledStage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NegativeControlResult {
    pub control_id: String,
    pub kind: NegativeControlKind,
    pub expected_state: BehavioralRunState,
    pub observed_state: BehavioralRunState,
    pub required_delta_observed: bool,
    pub invariant_comparison_rejected: bool,
    pub unaffected_complement_preserved: bool,
    pub passed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NegativeSensitivityReport {
    pub schema_version: u32,
    pub matrix_path: String,
    pub matrix_sha256: String,
    pub controls: Vec<NegativeControlResult>,
    pub attempt_dispositions: BTreeMap<String, BehavioralRunState>,
    pub passed: bool,
}

#[derive(Debug, Clone)]
struct StageArtifact {
    identity: ArtifactIdentity,
    value: Value,
}

#[derive(Debug)]
struct PipelineArtifacts {
    stages: BTreeMap<BehavioralStage, StageArtifact>,
    evidence_ir: EvidenceIr,
    document_key: String,
}

#[derive(Debug, Clone)]
struct BehavioralPopulationRow {
    document_key: String,
    source_origin: String,
    source_locator: String,
    source_sha256: String,
    normalized_markdown_path: String,
    normalized_markdown_sha256: String,
    vendor: String,
    family: String,
    category: String,
    layout: String,
    review_role: String,
    text_semantic_records: usize,
    text_intent_records: usize,
}

#[derive(Debug, Deserialize)]
struct HeldOutContractAuthority {
    schema_version: u32,
    owner: String,
    selection_boundary_commit: String,
    declarations: BTreeMap<String, String>,
    population_assertions: BTreeMap<String, usize>,
    frozen_census: HeldOutFrozenCensus,
    relations: Vec<HeldOutRelationAuthority>,
    held_out_policy: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct HeldOutFrozenCensus {
    prior_memory: HeldOutFileIdentity,
}

#[derive(Debug, Deserialize)]
struct HeldOutFileIdentity {
    path: String,
    sha256: String,
}

#[derive(Debug, Deserialize)]
struct HeldOutRelationAuthority {
    relation_id: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewedRecipeManifest {
    schema_version: u32,
    owner: String,
    recipes: Vec<ReviewedRecipeDeclaration>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewedRecipeDeclaration {
    recipe_id: String,
    relation: BehavioralRelation,
    path: String,
    sha256: String,
    source_authority: String,
    source_sha256: String,
    review_role: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewedTransformRecipe {
    schema_version: u32,
    recipe_id: String,
    relation: BehavioralRelation,
    source_authority: String,
    source_sha256: String,
    review_status: String,
    changed_spans: Vec<ReviewedSpanChange>,
    #[serde(default)]
    preserved_conclusions: Vec<ReviewedConclusion>,
    #[serde(default)]
    semantic_negative_kind: Option<SemanticNegativeKind>,
    #[serde(default)]
    required_deltas: Vec<ReviewedRequiredDelta>,
    #[serde(default)]
    dependent_proof_deltas: Vec<ReviewedProofDelta>,
    unaffected_complement: String,
    unmeasurable_source_surfaces: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewedSpanChange {
    change_id: String,
    kind: ReviewedChangeKind,
    exact_before: String,
    exact_after: String,
    expected_occurrences: usize,
    allowed_provenance_fields: Vec<String>,
    allow_source_bound_identifier_projection: bool,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewedConclusion {
    conclusion_id: String,
    stage: BehavioralStage,
    baseline_pointer: String,
    transformed_pointer: String,
    baseline_value: String,
    transformed_value: String,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewedRequiredDelta {
    delta_id: String,
    stage: BehavioralStage,
    kind: RequiredDeltaKind,
    baseline_pointer: Option<String>,
    transformed_pointer: Option<String>,
    baseline_value: Option<Value>,
    transformed_value: Option<Value>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewedProofAddress {
    stage: String,
    surface: String,
    stable_record_key: String,
    field_path: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReviewedProofDelta {
    delta_id: String,
    stages: Vec<BehavioralStage>,
    rule_id: String,
    address: ReviewedProofAddress,
}

#[derive(Debug, Clone)]
struct LoadedReviewedRecipe {
    manifest_path: String,
    recipe_path: String,
    recipe_sha256: String,
    recipe: ReviewedTransformRecipe,
}

#[derive(Debug, Clone, Serialize)]
struct TransformRecipe<'a> {
    relation: BehavioralRelation,
    seed: u64,
    baseline_source: &'a str,
    transformed_source: &'a str,
    symbol_renames: &'a [SymbolRename],
}

#[derive(Debug, Clone)]
struct NormalizationSpec {
    relation: BehavioralRelation,
    baseline_run_root: String,
    transformed_run_root: String,
    baseline_source: String,
    transformed_source: String,
    exact_transformed_to_baseline: BTreeMap<String, String>,
    identifier_transformed_to_baseline: BTreeMap<String, String>,
    reviewed_text_by_field: BTreeMap<String, BTreeMap<String, String>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum NegativeMutationOperation {
    Remove,
    Replace,
    DisableStage,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct NegativeSensitivityMatrix {
    schema_version: u32,
    owner: String,
    unaffected_complement: String,
    controls: Vec<NegativeControlDeclaration>,
    attempt_dispositions: BTreeMap<String, BehavioralRunState>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct NegativeControlDeclaration {
    control_id: String,
    kind: NegativeControlKind,
    stage: BehavioralStage,
    operation: NegativeMutationOperation,
    path: Option<String>,
    baseline_value: Option<Value>,
    transformed_value: Option<Value>,
    expected_state: BehavioralRunState,
    expected_failure_id: String,
}

/// Execute a qualification attempt while preserving the contract's fail/invalid/unmeasurable
/// disposition instead of forcing callers to infer it from an error string.
pub fn attempt_behavioral_relation(
    request: &BehavioralQualificationRequest,
) -> BehavioralQualificationAttempt {
    match qualify_behavioral_relation(request) {
        Ok(report) => {
            let failure_id = report
                .failures
                .first()
                .and_then(|failure| failure.split(':').next())
                .map(str::to_string);
            BehavioralQualificationAttempt {
                schema_version: BEHAVIORAL_SCHEMA_VERSION,
                relation: request.relation,
                input_plane: request.relation.input_plane().to_string(),
                state: report.state,
                failure_id,
                detail: None,
                report: Some(report),
            }
        }
        Err(error) => {
            let detail = error.to_string();
            let (state, failure_id) = classify_attempt_error(&detail);
            BehavioralQualificationAttempt {
                schema_version: BEHAVIORAL_SCHEMA_VERSION,
                relation: request.relation,
                input_plane: request.relation.input_plane().to_string(),
                state,
                failure_id: Some(failure_id.to_string()),
                detail: Some(detail),
                report: None,
            }
        }
    }
}

fn classify_attempt_error(detail: &str) -> (BehavioralRunState, &'static str) {
    for (failure_id, state) in [
        ("authority_unavailable", BehavioralRunState::Unmeasurable),
        ("provider_unavailable", BehavioralRunState::Unmeasurable),
        ("vacuous_baseline", BehavioralRunState::Unmeasurable),
        (
            "eligible_symbol_surface_absent",
            BehavioralRunState::Unmeasurable,
        ),
        (
            "ambiguous_or_nonbijective_transform",
            BehavioralRunState::Invalid,
        ),
        ("partial_or_escaped_run", BehavioralRunState::Invalid),
        ("stale_contract_or_population", BehavioralRunState::Invalid),
    ] {
        if detail.contains(failure_id) {
            return (state, failure_id);
        }
    }
    (BehavioralRunState::Invalid, "stale_contract_or_population")
}

fn classified_attempt_error(
    relation: BehavioralRelation,
    detail: &str,
) -> BehavioralQualificationAttempt {
    let (state, failure_id) = classify_attempt_error(detail);
    BehavioralQualificationAttempt {
        schema_version: BEHAVIORAL_SCHEMA_VERSION,
        relation,
        input_plane: relation.input_plane().to_string(),
        state,
        failure_id: Some(failure_id.to_string()),
        detail: Some(detail.to_string()),
        report: None,
    }
}

fn attempt_retained_behavioral_relation(
    request: &BehavioralQualificationRequest,
    retained_output_root: &Path,
    retained_attempt_root: &Path,
) -> BehavioralQualificationAttempt {
    match recompare_retained_behavioral_relation(
        request,
        retained_output_root,
        retained_attempt_root,
    ) {
        Ok(report) => {
            let failure_id = report
                .failures
                .first()
                .and_then(|failure| failure.split(':').next())
                .map(str::to_string);
            BehavioralQualificationAttempt {
                schema_version: BEHAVIORAL_SCHEMA_VERSION,
                relation: request.relation,
                input_plane: request.relation.input_plane().to_string(),
                state: report.state,
                failure_id,
                detail: None,
                report: Some(report),
            }
        }
        Err(error) => classified_attempt_error(request.relation, &error.to_string()),
    }
}

fn attempt_revalidated_behavioral_report(
    request: &BehavioralQualificationRequest,
    retained_output_root: &Path,
    retained_attempt_root: &Path,
) -> BehavioralQualificationAttempt {
    match revalidate_retained_behavioral_report(
        request,
        retained_output_root,
        retained_attempt_root,
    ) {
        Ok(report) => BehavioralQualificationAttempt {
            schema_version: BEHAVIORAL_SCHEMA_VERSION,
            relation: request.relation,
            input_plane: request.relation.input_plane().to_string(),
            state: report.state,
            failure_id: None,
            detail: None,
            report: Some(report),
        },
        Err(error) => classified_attempt_error(request.relation, &error.to_string()),
    }
}

fn ultimate_retained_artifact_output_root(
    repository: &Path,
    retained_output_root: &Path,
    aggregate: &Value,
) -> Result<PathBuf> {
    let mut output_root = retained_output_root.to_path_buf();
    let mut current = aggregate.clone();
    let mut visited = BTreeSet::new();
    for _ in 0..8 {
        if !visited.insert(output_root.clone()) {
            return Err(invalid(
                "stale_contract_or_population: retained evidence chain contains a cycle",
            ));
        }
        let Some(evidence_path) = current
            .get("retained_evidence_path")
            .and_then(Value::as_str)
        else {
            return Ok(output_root);
        };
        let evidence_relative = Path::new(evidence_path);
        validate_relative_path(evidence_relative, "retained evidence chain link")?;
        if !evidence_relative.starts_with(TEMP_ROOT)
            || evidence_relative.file_name().and_then(|name| name.to_str())
                != Some(HELD_OUT_EVIDENCE_FILE)
        {
            return Err(invalid(
                "partial_or_escaped_run: retained evidence chain left repository scratch",
            ));
        }
        let evidence = resolve_repository_file(
            repository,
            evidence_relative,
            "retained artifact authority evidence",
        )?;
        ensure_same_filesystem(repository, &evidence)?;
        let expected_sha256 = current
            .get("retained_evidence_sha256")
            .and_then(Value::as_str)
            .ok_or_else(|| {
                invalid(
                    "stale_contract_or_population: retained artifact authority digest is absent",
                )
            })?;
        let evidence_bytes = fs::read(&evidence)?;
        if sha256_bytes(&evidence_bytes) != expected_sha256 {
            return Err(invalid(
                "stale_contract_or_population: retained artifact authority digest differs",
            ));
        }
        output_root = evidence_relative
            .parent()
            .ok_or_else(|| {
                invalid("stale_contract_or_population: retained evidence path has no root")
            })?
            .to_path_buf();
        current = serde_json::from_slice(&evidence_bytes)?;
    }
    Err(invalid(
        "stale_contract_or_population: retained evidence chain exceeds eight links",
    ))
}

fn revalidate_retained_behavioral_report(
    request: &BehavioralQualificationRequest,
    retained_output_root: &Path,
    retained_attempt_root: &Path,
) -> Result<BehavioralQualificationReport> {
    if !matches!(
        request.relation,
        BehavioralRelation::UnchangedSource | BehavioralRelation::AdversarialIdentity
    ) {
        return Err(invalid(
            "stale_contract_or_population: only full-capture relations may revalidate retained reports",
        ));
    }
    validate_request(request)?;
    let repository = crate::project_data::repository_root()?;
    let aggregate_path = resolve_repository_file(
        &repository,
        &retained_output_root.join(HELD_OUT_EVIDENCE_FILE),
        "retained held-out aggregate evidence",
    )?;
    let aggregate: Value = serde_json::from_slice(&fs::read(&aggregate_path)?)?;
    if aggregate.get("schema_version").and_then(Value::as_u64)
        != Some(HELD_OUT_SCHEMA_VERSION as u64)
    {
        return Err(invalid(
            "stale_contract_or_population: retained aggregate is not revalidation-capable",
        ));
    }
    let report_path = resolve_repository_file(
        &repository,
        &retained_attempt_root.join(EVIDENCE_FILE),
        "retained behavioral evidence",
    )?;
    let report_bytes = fs::read(&report_path)?;
    let report_sha256 = sha256_bytes(&report_bytes);
    let report: BehavioralQualificationReport = serde_json::from_slice(&report_bytes)?;
    let document_key = &report.transform.baseline_document_key;
    let aggregate_attempt = aggregate
        .get("attempts")
        .and_then(Value::as_array)
        .and_then(|attempts| {
            attempts.iter().find(|attempt| {
                attempt.get("document_key").and_then(Value::as_str) == Some(document_key)
                    && attempt.get("relation").and_then(Value::as_str)
                        == Some(request.relation.as_str())
            })
        })
        .ok_or_else(|| {
            invalid("stale_contract_or_population: retained aggregate attempt is absent")
        })?;
    let retained_execution_mode = aggregate_attempt
        .get("execution_mode")
        .and_then(Value::as_str);
    if aggregate_attempt
        .get("attempt_report_sha256")
        .and_then(Value::as_str)
        != Some(&report_sha256)
        || aggregate_attempt.get("state").and_then(Value::as_str) != Some("pass")
        || !matches!(
            retained_execution_mode,
            Some("retained_artifacts_recompared" | "retained_report_revalidated")
        )
        || report.state != BehavioralRunState::Pass
        || !report.failures.is_empty()
        || report.stages.len() != BehavioralStage::ALL.len()
        || report.stages.iter().any(|stage| !stage.passed)
    {
        return Err(invalid(
            "stale_contract_or_population: retained passing report disposition differs",
        ));
    }
    let aggregate_contract_sha256 = aggregate
        .get("contract_sha256")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            invalid("stale_contract_or_population: retained contract identity absent")
        })?;
    let aggregate_prior_memory_sha256 = aggregate
        .get("prior_memory_sha256")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            invalid("stale_contract_or_population: retained prior-memory identity absent")
        })?;
    let aggregate_tool_sha256 = aggregate
        .get(
            if retained_execution_mode == Some("retained_report_revalidated") {
                "retained_tool_sha256"
            } else {
                "tool_sha256"
            },
        )
        .and_then(Value::as_str)
        .ok_or_else(|| invalid("stale_contract_or_population: retained tool identity absent"))?;
    if report.schema_version != BEHAVIORAL_SCHEMA_VERSION
        || report.relation != request.relation
        || report.input_plane != request.relation.input_plane()
        || report.identity.contract_sha256 != aggregate_contract_sha256
        || report.identity.contract_sha256 != sha256_file(&repository.join(CONTRACT_PATH))?
        || report.identity.production_revision != request.production_revision
        || report.identity.source_sha256 != request.expected_source_sha256
        || report.identity.prior_memory_sha256 != aggregate_prior_memory_sha256
        || report.identity.tool_sha256 != aggregate_tool_sha256
        || report.source.sha256 != request.expected_source_sha256
        || report.transform.relation != request.relation
        || report.transform.seed != request.transform_seed
    {
        return Err(invalid(
            "stale_contract_or_population: retained passing report identity differs",
        ));
    }
    let prior_memory = resolve_repository_file(&repository, &request.prior_memory, "prior memory")?;
    if sha256_file(&prior_memory)? != report.identity.prior_memory_sha256 {
        return Err(invalid(
            "stale_contract_or_population: retained prior-memory bytes differ",
        ));
    }
    let source_authority = resolve_source_authority(&repository, &request.source_authority)?;
    ensure_same_filesystem(&repository, &source_authority)?;
    let source_bytes = fs::read(&source_authority)?;
    if sha256_bytes(&source_bytes) != request.expected_source_sha256
        || source_bytes.len() as u64 != report.source.byte_count
    {
        return Err(invalid(
            "stale_contract_or_population: retained report source authority differs",
        ));
    }

    let artifact_output_root =
        ultimate_retained_artifact_output_root(&repository, retained_output_root, &aggregate)?;
    let artifact_attempt_root = artifact_output_root
        .join("attempts")
        .join(request.relation.as_str())
        .join(document_key);
    let baseline_source = retained_source_path(
        &repository,
        &artifact_attempt_root,
        &report.transform.baseline_source,
        "retained baseline source",
    )?;
    let transformed_source = retained_source_path(
        &repository,
        &artifact_attempt_root,
        &report.transform.transformed_source,
        "retained transformed source",
    )?;
    let baseline_source_bytes = fs::read(&baseline_source)?;
    let transformed_source_bytes = fs::read(&transformed_source)?;
    if sha256_bytes(&baseline_source_bytes) != request.expected_source_sha256
        || baseline_source_bytes.len() as u64 != report.source.byte_count
        || (baseline_source_bytes == transformed_source_bytes)
            != report.transform.source_bytes_equal
    {
        return Err(invalid(
            "stale_contract_or_population: retained report transform source differs",
        ));
    }
    let recipe = TransformRecipe {
        relation: request.relation,
        seed: request.transform_seed,
        baseline_source: &report.transform.baseline_source,
        transformed_source: &report.transform.transformed_source,
        symbol_renames: &report.transform.symbol_renames,
    };
    if sha256_bytes(&serde_json::to_vec(&recipe)?) != report.identity.transform_recipe_sha256 {
        return Err(invalid(
            "stale_contract_or_population: retained report transform recipe differs",
        ));
    }
    let baseline =
        load_retained_pipeline(&repository, &artifact_attempt_root, &report.stages, false)?;
    if baseline.document_key != report.transform.baseline_document_key {
        return Err(invalid(
            "stale_contract_or_population: retained baseline document identity differs",
        ));
    }
    drop(baseline);
    let transformed =
        load_retained_pipeline(&repository, &artifact_attempt_root, &report.stages, true)?;
    if transformed.document_key != report.transform.transformed_document_key {
        return Err(invalid(
            "stale_contract_or_population: retained transformed document identity differs",
        ));
    }
    drop(transformed);

    let output_root = prepare_output_root(&repository, &request.output_root)?;
    fs::write(output_root.join(EVIDENCE_FILE), &report_bytes)?;
    Ok(report)
}

fn retained_alpha_symbol_catalog(
    repository: &Path,
    retained_attempt_root: &Path,
    source_authority: &Path,
    expected_source_sha256: &str,
    document_key: &str,
) -> Result<Option<Vec<String>>> {
    let source = resolve_source_authority(repository, source_authority)?;
    ensure_same_filesystem(repository, &source)?;
    let source_bytes = fs::read(&source)?;
    let observed_source_sha256 = sha256_bytes(&source_bytes);
    if observed_source_sha256 != expected_source_sha256 {
        return Err(invalid(format!(
            "stale_contract_or_population: symbol-alpha source SHA-256 differs: {observed_source_sha256} != {expected_source_sha256}"
        )));
    }
    let evidence_relative = retained_attempt_root
        .join("baseline/evidence_ir")
        .join(document_key)
        .join("evidence_ir.json");
    if !evidence_relative.starts_with(retained_attempt_root) {
        return Err(invalid(
            "partial_or_escaped_run: retained alpha evidence escaped its attempt root",
        ));
    }
    let report_relative = retained_attempt_root.join(EVIDENCE_FILE);
    if !repository.join(&report_relative).is_file() {
        return Ok(None);
    }
    let report_path = resolve_repository_file(
        repository,
        &report_relative,
        "retained alpha attempt report",
    )?;
    let report: BehavioralQualificationReport = serde_json::from_slice(&fs::read(&report_path)?)?;
    let baseline_identity = report
        .stages
        .iter()
        .find(|stage| stage.stage == BehavioralStage::EvidenceIr)
        .map(|stage| &stage.baseline)
        .ok_or_else(|| {
            invalid("partial_or_escaped_run: retained alpha report lacks baseline EvidenceIR")
        })?;
    if report.relation != BehavioralRelation::SymbolAlpha
        || report.identity.source_sha256 != expected_source_sha256
        || report.transform.baseline_document_key != document_key
        || baseline_identity.path != evidence_relative.to_string_lossy()
    {
        return Err(invalid(
            "stale_contract_or_population: retained alpha report identity differs",
        ));
    }
    let evidence_path =
        resolve_repository_file(repository, &evidence_relative, "retained alpha EvidenceIR")?;
    ensure_same_filesystem(repository, &evidence_path)?;
    let evidence_bytes = fs::read(&evidence_path)?;
    if evidence_bytes.len() as u64 != baseline_identity.byte_count
        || sha256_bytes(&evidence_bytes) != baseline_identity.sha256
    {
        return Err(invalid(
            "stale_contract_or_population: retained alpha EvidenceIR identity differs",
        ));
    }
    let evidence_ir = EvidenceIr::load_from_path(&evidence_path)?;
    let source_text = String::from_utf8(source_bytes)
        .map_err(|_| invalid("symbol-alpha input must be UTF-8 normalized Markdown"))?;
    Ok(Some(derive_source_symbol_catalog(
        &evidence_ir,
        &source_text,
    )))
}

fn recompare_retained_behavioral_relation(
    request: &BehavioralQualificationRequest,
    retained_output_root: &Path,
    retained_attempt_root: &Path,
) -> Result<BehavioralQualificationReport> {
    if !matches!(
        request.relation,
        BehavioralRelation::UnchangedSource | BehavioralRelation::AdversarialIdentity
    ) {
        return Err(invalid(
            "stale_contract_or_population: only full-capture relations may reuse retained artifacts",
        ));
    }
    validate_request(request)?;
    validate_relative_path(retained_attempt_root, "retained behavioral attempt root")?;
    let repository = crate::project_data::repository_root()?;
    let retained_evidence_path = resolve_repository_file(
        &repository,
        &retained_attempt_root.join(EVIDENCE_FILE),
        "retained behavioral evidence",
    )?;
    ensure_same_filesystem(&repository, &retained_evidence_path)?;
    let retained: BehavioralQualificationReport =
        serde_json::from_slice(&fs::read(&retained_evidence_path)?)?;
    let retained_aggregate: Value = serde_json::from_slice(&fs::read(resolve_repository_file(
        &repository,
        &retained_output_root.join(HELD_OUT_EVIDENCE_FILE),
        "retained held-out aggregate evidence",
    )?)?)?;
    let retained_contract_sha256 = retained_aggregate
        .get("contract_sha256")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            invalid("stale_contract_or_population: retained aggregate lacks contract identity")
        })?;
    let retained_prior_memory_sha256 = retained_aggregate
        .get("prior_memory_sha256")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            invalid("stale_contract_or_population: retained aggregate lacks prior-memory identity")
        })?;
    let retained_production_revision = retained_aggregate
        .get("production_revision")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            invalid("stale_contract_or_population: retained aggregate lacks production identity")
        })?;
    if retained.schema_version != BEHAVIORAL_SCHEMA_VERSION
        || retained.relation != request.relation
        || retained.input_plane != request.relation.input_plane()
        || retained.transform.relation != request.relation
        || retained.transform.seed != request.transform_seed
        || retained.identity.production_revision != request.production_revision
        || retained.identity.source_sha256 != request.expected_source_sha256
        || retained.source.sha256 != request.expected_source_sha256
        || retained.stages.len() != BehavioralStage::ALL.len()
    {
        return Err(invalid(
            "stale_contract_or_population: retained behavioral report identity differs",
        ));
    }
    let source_authority = resolve_source_authority(&repository, &request.source_authority)?;
    ensure_same_filesystem(&repository, &source_authority)?;
    let source_bytes = fs::read(&source_authority)?;
    if sha256_bytes(&source_bytes) != request.expected_source_sha256
        || source_bytes.len() as u64 != retained.source.byte_count
    {
        return Err(invalid(
            "stale_contract_or_population: retained source authority differs",
        ));
    }
    let contract_sha256 = sha256_file(&repository.join(CONTRACT_PATH))?;
    let prior_memory = resolve_repository_file(&repository, &request.prior_memory, "prior memory")?;
    let prior_memory_sha256 = sha256_file(&prior_memory)?;
    if retained.identity.contract_sha256 != retained_contract_sha256
        || retained.identity.prior_memory_sha256 != retained_prior_memory_sha256
        || retained.identity.production_revision != retained_production_revision
    {
        return Err(invalid(
            "stale_contract_or_population: retained contract or prior-memory identity differs",
        ));
    }

    let baseline_source = retained_source_path(
        &repository,
        retained_attempt_root,
        &retained.transform.baseline_source,
        "retained baseline source",
    )?;
    let transformed_source = retained_source_path(
        &repository,
        retained_attempt_root,
        &retained.transform.transformed_source,
        "retained transformed source",
    )?;
    let baseline_source_bytes = fs::read(&baseline_source)?;
    let transformed_source_bytes = fs::read(&transformed_source)?;
    if sha256_bytes(&baseline_source_bytes) != request.expected_source_sha256
        || baseline_source_bytes.len() as u64 != retained.source.byte_count
        || (baseline_source_bytes == transformed_source_bytes)
            != retained.transform.source_bytes_equal
    {
        return Err(invalid(
            "stale_contract_or_population: retained transform source identity differs",
        ));
    }

    let baseline =
        load_retained_pipeline(&repository, retained_attempt_root, &retained.stages, false)?;
    let transformed =
        load_retained_pipeline(&repository, retained_attempt_root, &retained.stages, true)?;
    if baseline.document_key != retained.transform.baseline_document_key
        || transformed.document_key != retained.transform.transformed_document_key
    {
        return Err(invalid(
            "stale_contract_or_population: retained pipeline document identity differs",
        ));
    }

    let mut exact_transformed_to_baseline = BTreeMap::new();
    if request.relation == BehavioralRelation::AdversarialIdentity {
        exact_transformed_to_baseline.insert(
            transformed.document_key.clone(),
            baseline.document_key.clone(),
        );
        exact_transformed_to_baseline.insert(
            transformed_source
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_string(),
            baseline_source
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_string(),
        );
    }
    let normalization = NormalizationSpec {
        relation: request.relation,
        baseline_run_root: retained_attempt_root
            .join("baseline")
            .to_string_lossy()
            .into_owned(),
        transformed_run_root: retained_attempt_root
            .join("transformed")
            .to_string_lossy()
            .into_owned(),
        baseline_source: retained.transform.baseline_source.clone(),
        transformed_source: retained.transform.transformed_source.clone(),
        exact_transformed_to_baseline,
        identifier_transformed_to_baseline: BTreeMap::new(),
        reviewed_text_by_field: BTreeMap::new(),
    };
    let (state, stages, failures) =
        compare_pipelines(&baseline, &transformed, &normalization, None)?;
    let recipe = TransformRecipe {
        relation: request.relation,
        seed: request.transform_seed,
        baseline_source: &retained.transform.baseline_source,
        transformed_source: &retained.transform.transformed_source,
        symbol_renames: &retained.transform.symbol_renames,
    };
    let recipe_sha256 = sha256_bytes(&serde_json::to_vec(&recipe)?);
    if recipe_sha256 != retained.identity.transform_recipe_sha256 {
        return Err(invalid(
            "stale_contract_or_population: retained transform recipe identity differs",
        ));
    }
    let coverage = unreviewed_coverage(&stages, &retained.transform.symbol_renames);
    let report = BehavioralQualificationReport {
        schema_version: BEHAVIORAL_SCHEMA_VERSION,
        relation: request.relation,
        input_plane: request.relation.input_plane().to_string(),
        state,
        identity: EvidenceIdentity {
            contract_sha256,
            production_revision: request.production_revision.clone(),
            source_sha256: request.expected_source_sha256.clone(),
            transform_recipe_sha256: recipe_sha256,
            prior_memory_sha256,
            tool_sha256: sha256_bytes(include_bytes!("behavioral_genericity.rs")),
        },
        source: retained.source,
        transform: retained.transform,
        stages,
        coverage,
        failures,
    };
    let output_root = prepare_output_root(&repository, &request.output_root)?;
    fs::write(
        output_root.join(EVIDENCE_FILE),
        serde_json::to_vec_pretty(&report)?,
    )?;
    Ok(report)
}

fn retained_source_path(
    repository: &Path,
    retained_attempt_root: &Path,
    path: &str,
    label: &str,
) -> Result<PathBuf> {
    let relative = Path::new(path);
    if !relative.starts_with(retained_attempt_root.join("inputs")) {
        return Err(invalid(format!(
            "partial_or_escaped_run: {label} escaped its retained attempt root"
        )));
    }
    let resolved = resolve_repository_file(repository, relative, label)?;
    ensure_same_filesystem(repository, &resolved)?;
    Ok(resolved)
}

fn load_retained_pipeline(
    repository: &Path,
    retained_attempt_root: &Path,
    comparisons: &[StageComparison],
    transformed: bool,
) -> Result<PipelineArtifacts> {
    let mut stages = BTreeMap::new();
    let mut evidence_ir = None;
    let mut document_key = None;
    for stage in BehavioralStage::ALL {
        let matching = comparisons
            .iter()
            .filter(|comparison| comparison.stage == stage)
            .collect::<Vec<_>>();
        if matching.len() != 1 {
            return Err(invalid(
                "partial_or_escaped_run: retained stage set is incomplete or duplicated",
            ));
        }
        let identity = if transformed {
            &matching[0].transformed
        } else {
            &matching[0].baseline
        };
        let relative = Path::new(&identity.path);
        if !relative.starts_with(retained_attempt_root) {
            return Err(invalid(
                "partial_or_escaped_run: retained stage artifact escaped its attempt root",
            ));
        }
        let path = resolve_repository_file(repository, relative, "retained stage artifact")?;
        ensure_same_filesystem(repository, &path)?;
        let bytes = fs::read(&path)?;
        if bytes.len() as u64 != identity.byte_count || sha256_bytes(&bytes) != identity.sha256 {
            return Err(invalid(
                "stale_contract_or_population: retained stage artifact identity differs",
            ));
        }
        let value = serde_json::from_slice::<Value>(&bytes)?;
        if value.get("stage").and_then(Value::as_str) != Some(stage.as_str()) {
            return Err(invalid(format!(
                "partial_or_escaped_run: retained {} artifact names another stage",
                stage.as_str()
            )));
        }
        match stage {
            BehavioralStage::SourceIr => {
                SourceIr::load_from_path(&path)?;
            }
            BehavioralStage::EvidenceIr => {
                evidence_ir = Some(EvidenceIr::load_from_path(&path)?);
            }
            BehavioralStage::SemanticIr => {
                SemanticIr::load_from_path(&path)?;
            }
            BehavioralStage::IntentIr => {
                document_key = Some(
                    IntentIr::load_from_path(&path)?
                        .document_identity
                        .document_key,
                );
            }
            BehavioralStage::IsfAdapter => {
                AdapterArtifact::load_from_path(&path)?;
            }
        }
        stages.insert(
            stage,
            StageArtifact {
                identity: identity.clone(),
                value,
            },
        );
    }
    Ok(PipelineArtifacts {
        stages,
        evidence_ir: evidence_ir
            .ok_or_else(|| invalid("partial_or_escaped_run: retained pipeline lacks EvidenceIR"))?,
        document_key: document_key.ok_or_else(|| {
            invalid("partial_or_escaped_run: retained pipeline lacks IntentIR identity")
        })?,
    })
}

fn unreviewed_coverage(
    stages: &[StageComparison],
    symbol_renames: &[SymbolRename],
) -> BehavioralCoverage {
    BehavioralCoverage {
        required_stages: BehavioralStage::ALL.len(),
        completed_stages: stages.len(),
        baseline_top_level_fields: stages
            .iter()
            .map(|stage| stage.baseline_top_level_fields)
            .sum(),
        transformed_top_level_fields: stages
            .iter()
            .map(|stage| stage.transformed_top_level_fields)
            .sum(),
        baseline_proof_claims: stages.iter().map(|stage| stage.baseline_proof_claims).sum(),
        transformed_proof_claims: stages
            .iter()
            .map(|stage| stage.transformed_proof_claims)
            .sum(),
        compared_leaf_values: stages.iter().map(|stage| stage.compared_leaf_values).sum(),
        expected_symbol_deltas: symbol_renames.len(),
        observed_symbol_deltas: symbol_renames
            .iter()
            .filter(|rename| rename.occurrence_count > 0)
            .count(),
        expected_reviewed_span_deltas: 0,
        observed_reviewed_span_deltas: 0,
        preserved_conclusions: 0,
        expected_semantic_deltas: 0,
        observed_semantic_deltas: 0,
    }
}

/// Execute every recipe-free relation over the frozen prospective population and emit one
/// aggregate whose labels and outcomes remain downstream of the production core.
pub fn qualify_held_out_population(
    request: &HeldOutQualificationRequest,
) -> Result<HeldOutQualificationReport> {
    validate_held_out_request(request)?;
    let repository = crate::project_data::repository_root()?;
    let contract_path = repository.join(CONTRACT_PATH);
    let population_path = repository.join(POPULATION_PATH);
    let contract_bytes = fs::read(&contract_path)?;
    let population_bytes = fs::read(&population_path)?;
    let contract: HeldOutContractAuthority = serde_json::from_slice(&contract_bytes)?;
    validate_held_out_contract(&contract)?;
    let rows = parse_behavioral_population(&population_bytes)?;
    validate_behavioral_population(&rows, &contract)?;
    let retained_aggregate = if let Some(root) = &request.retained_output_root {
        let path = resolve_repository_file(
            &repository,
            &root.join(HELD_OUT_EVIDENCE_FILE),
            "retained held-out aggregate evidence",
        )?;
        Some(serde_json::from_slice::<Value>(&fs::read(path)?)?)
    } else {
        None
    };
    let retained_schema_version = retained_aggregate
        .as_ref()
        .and_then(|aggregate| aggregate.get("schema_version"))
        .and_then(Value::as_u64);
    let retained_artifact_output_root = match (&request.retained_output_root, &retained_aggregate) {
        (Some(root), Some(aggregate)) => Some(ultimate_retained_artifact_output_root(
            &repository,
            root,
            aggregate,
        )?),
        _ => None,
    };

    let calibration = rows
        .iter()
        .filter(|row| row.review_role == "reviewed_calibration")
        .collect::<Vec<_>>();
    let prospective = rows
        .iter()
        .filter(|row| row.review_role == "prospective_holdout")
        .collect::<Vec<_>>();
    let calibration_vendors = calibration
        .iter()
        .map(|row| row.vendor.as_str())
        .collect::<BTreeSet<_>>();
    let calibration_families = calibration
        .iter()
        .map(|row| row.family.as_str())
        .collect::<BTreeSet<_>>();

    let split = held_out_split_evidence(
        &contract.selection_boundary_commit,
        &calibration,
        &prospective,
    );
    if !split.identity_disjoint {
        return Err(invalid(
            "stale_contract_or_population: calibration and prospective identities overlap",
        ));
    }

    let documents = prospective
        .iter()
        .map(|row| HeldOutDocumentEvidence {
            document_key: row.document_key.clone(),
            source_origin: row.source_origin.clone(),
            source_locator: row.source_locator.clone(),
            source_sha256: row.source_sha256.clone(),
            normalized_markdown_path: row.normalized_markdown_path.clone(),
            normalized_markdown_sha256: row.normalized_markdown_sha256.clone(),
            vendor: row.vendor.clone(),
            family: row.family.clone(),
            category: row.category.clone(),
            layout: row.layout.clone(),
            vendor_novel: !calibration_vendors.contains(row.vendor.as_str()),
            family_novel: !calibration_families.contains(row.family.as_str()),
            text_semantic_records: row.text_semantic_records,
            text_intent_records: row.text_intent_records,
        })
        .collect::<Vec<_>>();

    let prior_memory =
        resolve_repository_file(&repository, &request.prior_memory, "held-out prior memory")?;
    if request.prior_memory != Path::new(&contract.frozen_census.prior_memory.path) {
        return Err(invalid(
            "stale_contract_or_population: held-out prior-memory path differs from contract",
        ));
    }
    let prior_memory_sha256 = sha256_file(&prior_memory)?;
    if prior_memory_sha256 != contract.frozen_census.prior_memory.sha256 {
        return Err(invalid(
            "stale_contract_or_population: held-out prior-memory digest differs from contract",
        ));
    }

    let relations = vec![
        BehavioralRelation::UnchangedSource,
        BehavioralRelation::AdversarialIdentity,
        BehavioralRelation::SymbolAlpha,
    ];
    let mut attempts = Vec::with_capacity(prospective.len() * relations.len());
    for (document_ordinal, row) in prospective.iter().enumerate() {
        for (relation_ordinal, relation) in relations.iter().copied().enumerate() {
            let transform_seed = request
                .transform_seed
                .wrapping_add((document_ordinal as u64) << 8)
                .wrapping_add(relation_ordinal as u64);
            let source_authority = held_out_source_authority(&repository, row, relation)?;
            let expected_source_sha256 = match relation {
                BehavioralRelation::SymbolAlpha => row.normalized_markdown_sha256.clone(),
                _ => row.source_sha256.clone(),
            };
            let attempt_root = request
                .output_root
                .join("attempts")
                .join(relation.as_str())
                .join(&row.document_key);
            eprintln!(
                "behavioral holdout: {}/{} {} {}",
                attempts.len() + 1,
                prospective.len() * relations.len(),
                row.document_key,
                relation.as_str()
            );
            let qualification_request = BehavioralQualificationRequest {
                relation,
                source_authority: source_authority.clone(),
                expected_source_sha256: expected_source_sha256.clone(),
                output_root: attempt_root.clone(),
                prior_memory: request.prior_memory.clone(),
                production_revision: request.production_revision.clone(),
                transform_seed,
                review_recipe_id: None,
            };
            let (attempt, execution_mode) = match (&request.retained_output_root, relation) {
                (Some(retained_root), BehavioralRelation::UnchangedSource)
                | (Some(retained_root), BehavioralRelation::AdversarialIdentity) => {
                    let retained_attempt_root = retained_root
                        .join("attempts")
                        .join(relation.as_str())
                        .join(&row.document_key);
                    if retained_schema_version == Some(HELD_OUT_SCHEMA_VERSION as u64) {
                        (
                            attempt_revalidated_behavioral_report(
                                &qualification_request,
                                retained_root,
                                &retained_attempt_root,
                            ),
                            HeldOutExecutionMode::RetainedReportRevalidated,
                        )
                    } else {
                        (
                            attempt_retained_behavioral_relation(
                                &qualification_request,
                                retained_root,
                                &retained_attempt_root,
                            ),
                            HeldOutExecutionMode::RetainedArtifactsRecompared,
                        )
                    }
                }
                (Some(_), BehavioralRelation::SymbolAlpha) => {
                    if row.text_semantic_records == 0 && row.text_intent_records == 0 {
                        (
                            classified_attempt_error(
                                relation,
                                "vacuous_baseline: frozen text projection has no semantic or intent records",
                            ),
                            HeldOutExecutionMode::EligibilityPreflight,
                        )
                    } else {
                        let retained_attempt_root = retained_artifact_output_root
                            .as_ref()
                            .expect("retained request has an artifact authority")
                            .join("attempts")
                            .join(relation.as_str())
                            .join(&row.document_key);
                        match retained_alpha_symbol_catalog(
                            &repository,
                            &retained_attempt_root,
                            &source_authority,
                            &expected_source_sha256,
                            &row.document_key,
                        ) {
                            Ok(Some(symbols)) if symbols.is_empty() => (
                                classified_attempt_error(
                                    relation,
                                    "eligible_symbol_surface_absent: symbol-alpha baseline has no typed opaque signal declaration",
                                ),
                                HeldOutExecutionMode::EligibilityPreflight,
                            ),
                            Ok(Some(_)) | Ok(None) => (
                                attempt_behavioral_relation(&qualification_request),
                                HeldOutExecutionMode::FreshPipeline,
                            ),
                            Err(error) => (
                                classified_attempt_error(relation, &error.to_string()),
                                HeldOutExecutionMode::EligibilityPreflight,
                            ),
                        }
                    }
                }
                (None, _) => (
                    attempt_behavioral_relation(&qualification_request),
                    HeldOutExecutionMode::FreshPipeline,
                ),
                (Some(_), _) => unreachable!("held-out relation list is closed above"),
            };
            let attempt_report_sha256 = if attempt.report.is_some() {
                let evidence_path = repository.join(&attempt_root).join(EVIDENCE_FILE);
                Some(sha256_file(&evidence_path).map_err(|error| {
                    invalid(format!(
                        "partial_or_escaped_run: completed held-out attempt has no readable evidence: {error}"
                    ))
                })?)
            } else {
                None
            };
            let identity = attempt
                .report
                .as_ref()
                .map(|report| report.identity.clone());
            let coverage = attempt
                .report
                .as_ref()
                .map(|report| report.coverage.clone());
            let completed_stages = attempt
                .report
                .as_ref()
                .map_or(0, |report| report.stages.len());
            let all_completed_stages_passed = attempt.report.as_ref().is_some_and(|report| {
                report.stages.len() == BehavioralStage::ALL.len()
                    && report.stages.iter().all(|stage| stage.passed)
            });
            attempts.push(HeldOutAttemptEvidence {
                document_key: row.document_key.clone(),
                relation,
                input_plane: relation.input_plane().to_string(),
                transform_seed,
                execution_mode,
                state: attempt.state,
                failure_id: attempt.failure_id,
                detail: attempt.detail,
                identity,
                coverage,
                completed_stages,
                all_completed_stages_passed,
                attempt_report_sha256,
            });
        }
    }

    if attempts.len() != prospective.len() * relations.len() {
        return Err(invalid(
            "partial_or_escaped_run: held-out attempt matrix is incomplete",
        ));
    }
    let strata = held_out_strata(&documents, &attempts, &relations);
    let coverage = aggregate_held_out_coverage(&documents, &attempts, relations.len());
    let tool_sha256 = sha256_file(&repository.join(file!()))?;
    let retained_tool_sha256 = retained_aggregate.as_ref().and_then(|aggregate| {
        let revalidated = aggregate
            .get("attempts")
            .and_then(Value::as_array)
            .is_some_and(|attempts| {
                attempts.iter().any(|attempt| {
                    attempt.get("execution_mode").and_then(Value::as_str)
                        == Some("retained_report_revalidated")
                })
            });
        aggregate
            .get(if revalidated {
                "retained_tool_sha256"
            } else {
                "tool_sha256"
            })
            .and_then(Value::as_str)
            .map(str::to_string)
    });
    let (retained_evidence_path, retained_evidence_sha256) =
        if let Some(root) = &request.retained_output_root {
            let path = root.join(HELD_OUT_EVIDENCE_FILE);
            (
                Some(path.to_string_lossy().into_owned()),
                Some(sha256_file(&repository.join(path))?),
            )
        } else {
            (None, None)
        };
    let leakage_boundary = contract
        .held_out_policy
        .get("leakage_rule")
        .and_then(Value::as_str)
        .ok_or_else(|| invalid("stale_contract_or_population: held-out leakage rule is absent"))?
        .to_string();
    let report = HeldOutQualificationReport {
        schema_version: HELD_OUT_SCHEMA_VERSION,
        owner: "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.iii".to_string(),
        contract_path: CONTRACT_PATH.to_string(),
        contract_sha256: sha256_bytes(&contract_bytes),
        population_path: POPULATION_PATH.to_string(),
        population_sha256: sha256_bytes(&population_bytes),
        production_revision: request.production_revision.clone(),
        prior_memory_sha256,
        tool_sha256,
        retained_tool_sha256,
        retained_evidence_path,
        retained_evidence_sha256,
        leakage_boundary,
        eligible_relations: relations,
        split,
        documents,
        attempts,
        strata,
        coverage,
        final_signoff_deferred: true,
    };
    let mut evidence_bytes = serde_json::to_vec_pretty(&report)?;
    evidence_bytes.push(b'\n');
    write_new(
        &repository
            .join(&request.output_root)
            .join(HELD_OUT_EVIDENCE_FILE),
        &evidence_bytes,
    )?;
    Ok(report)
}

fn validate_held_out_request(request: &HeldOutQualificationRequest) -> Result<()> {
    validate_relative_path(&request.output_root, "held-out output root")?;
    if !request.output_root.starts_with(TEMP_ROOT) || request.output_root == Path::new(TEMP_ROOT) {
        return Err(invalid(format!(
            "held-out output root must be a child of {TEMP_ROOT}: {}",
            request.output_root.display()
        )));
    }
    validate_relative_path(&request.prior_memory, "held-out prior memory")?;
    if !is_lower_hex(&request.production_revision, 40) {
        return Err(invalid(
            "held-out production revision must be a full Git id",
        ));
    }
    let repository = crate::project_data::repository_root()?;
    if repository.join(&request.output_root).exists() {
        return Err(invalid(format!(
            "held-out output root already exists: {}",
            request.output_root.display()
        )));
    }
    if let Some(retained_root) = &request.retained_output_root {
        validate_relative_path(retained_root, "retained held-out output root")?;
        if !retained_root.starts_with(TEMP_ROOT)
            || retained_root == Path::new(TEMP_ROOT)
            || request.output_root.starts_with(retained_root)
            || retained_root.starts_with(&request.output_root)
        {
            return Err(invalid(
                "retained held-out output root must be a distinct child of repository scratch",
            ));
        }
        let retained = repository.join(retained_root).canonicalize().map_err(|error| {
            invalid(format!(
                "authority_unavailable: retained held-out output root is unavailable: {} ({error})",
                retained_root.display()
            ))
        })?;
        let canonical_repository = repository.canonicalize()?;
        let canonical_temp = repository.join(TEMP_ROOT).canonicalize()?;
        if !retained.is_dir()
            || !retained.starts_with(&canonical_temp)
            || !canonical_temp.starts_with(&canonical_repository)
        {
            return Err(invalid(
                "partial_or_escaped_run: retained held-out output escaped repository scratch",
            ));
        }
        ensure_same_filesystem(&canonical_repository, &retained)?;
        resolve_repository_file(
            &repository,
            &retained_root.join(HELD_OUT_EVIDENCE_FILE),
            "retained held-out aggregate evidence",
        )?;
    }
    Ok(())
}

fn validate_held_out_contract(contract: &HeldOutContractAuthority) -> Result<()> {
    if contract.schema_version != 1
        || contract.owner != "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f"
        || !is_lower_hex(&contract.selection_boundary_commit, 40)
        || contract
            .declarations
            .get("current_population")
            .map(String::as_str)
            != Some(POPULATION_PATH)
        || contract
            .population_assertions
            .get("prospective_holdout_documents")
            .copied()
            != Some(17)
    {
        return Err(invalid(
            "stale_contract_or_population: held-out contract identity or denominator differs",
        ));
    }
    let relation_ids = contract
        .relations
        .iter()
        .map(|relation| relation.relation_id.as_str())
        .collect::<BTreeSet<_>>();
    for expected in ["unchanged_source", "adversarial_identity", "symbol_alpha"] {
        if !relation_ids.contains(expected) {
            return Err(invalid(format!(
                "stale_contract_or_population: held-out relation is absent: {expected}"
            )));
        }
    }
    if !contract
        .held_out_policy
        .get("leakage_rule")
        .and_then(Value::as_str)
        .is_some_and(|rule| rule.contains("Production core cannot read"))
    {
        return Err(invalid(
            "stale_contract_or_population: held-out leakage boundary differs",
        ));
    }
    Ok(())
}

fn parse_behavioral_population(bytes: &[u8]) -> Result<Vec<BehavioralPopulationRow>> {
    const HEADER: &str = "document_key\tsource_origin\tsource_locator\tsource_sha256\tsource_bytes\tnormalized_markdown_path\tnormalized_markdown_sha256\tpages\tvisual_assets\tstructured_tables\tcontent_elements\tsections\tvendor\tfamily\tcategory\tlayout\treview_role\ttext_semantic_records\ttext_intent_records";
    let text = std::str::from_utf8(bytes)
        .map_err(|_| invalid("stale_contract_or_population: population TSV is not UTF-8"))?;
    let mut lines = text.lines();
    if lines.next() != Some(HEADER) {
        return Err(invalid(
            "stale_contract_or_population: population TSV header differs",
        ));
    }
    let mut rows = Vec::new();
    for (ordinal, line) in lines.enumerate() {
        if line.is_empty() {
            return Err(invalid(format!(
                "stale_contract_or_population: population TSV has an empty row at {}",
                ordinal + 2
            )));
        }
        let fields = line.split('\t').collect::<Vec<_>>();
        if fields.len() != 19 || fields.iter().any(|field| field.is_empty()) {
            return Err(invalid(format!(
                "stale_contract_or_population: population row {} has incomplete fields",
                ordinal + 2
            )));
        }
        let parse_count = |index: usize, label: &str| -> Result<usize> {
            fields[index].parse::<usize>().map_err(|_| {
                invalid(format!(
                    "stale_contract_or_population: population row {} has invalid {label}",
                    ordinal + 2
                ))
            })
        };
        rows.push(BehavioralPopulationRow {
            document_key: fields[0].to_string(),
            source_origin: fields[1].to_string(),
            source_locator: fields[2].to_string(),
            source_sha256: fields[3].to_string(),
            normalized_markdown_path: fields[5].to_string(),
            normalized_markdown_sha256: fields[6].to_string(),
            vendor: fields[12].to_string(),
            family: fields[13].to_string(),
            category: fields[14].to_string(),
            layout: fields[15].to_string(),
            review_role: fields[16].to_string(),
            text_semantic_records: parse_count(17, "SemanticIR count")?,
            text_intent_records: parse_count(18, "IntentIR count")?,
        });
    }
    Ok(rows)
}

fn validate_behavioral_population(
    rows: &[BehavioralPopulationRow],
    contract: &HeldOutContractAuthority,
) -> Result<()> {
    let keys = rows
        .iter()
        .map(|row| row.document_key.as_str())
        .collect::<Vec<_>>();
    let unique = keys.iter().copied().collect::<BTreeSet<_>>();
    let prospective = rows
        .iter()
        .filter(|row| row.review_role == "prospective_holdout")
        .count();
    let calibration = rows
        .iter()
        .filter(|row| row.review_role == "reviewed_calibration")
        .count();
    if rows.len() != 24
        || unique.len() != rows.len()
        || !keys.windows(2).all(|window| window[0] < window[1])
        || prospective != 17
        || calibration != 7
        || rows.iter().any(|row| {
            !matches!(
                row.review_role.as_str(),
                "reviewed_calibration" | "prospective_holdout"
            )
        })
        || contract
            .population_assertions
            .get("current_documents")
            .copied()
            != Some(rows.len())
        || contract
            .population_assertions
            .get("reviewed_current_overlap")
            .copied()
            != Some(calibration)
    {
        return Err(invalid(
            "stale_contract_or_population: held-out population identity or split differs",
        ));
    }
    for row in rows {
        if !is_lower_hex(&row.source_sha256, 64)
            || !is_lower_hex(&row.normalized_markdown_sha256, 64)
            || !row
                .document_key
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
        {
            return Err(invalid(format!(
                "stale_contract_or_population: population identity is invalid for {}",
                row.document_key
            )));
        }
    }
    Ok(())
}

fn held_out_split_evidence(
    selection_boundary_commit: &str,
    calibration: &[&BehavioralPopulationRow],
    prospective: &[&BehavioralPopulationRow],
) -> HeldOutSplitEvidence {
    let calibration_keys = calibration
        .iter()
        .map(|row| row.document_key.clone())
        .collect::<BTreeSet<_>>();
    let prospective_keys = prospective
        .iter()
        .map(|row| row.document_key.clone())
        .collect::<BTreeSet<_>>();
    let calibration_sources = calibration
        .iter()
        .map(|row| row.source_sha256.clone())
        .collect::<BTreeSet<_>>();
    let prospective_sources = prospective
        .iter()
        .map(|row| row.source_sha256.clone())
        .collect::<BTreeSet<_>>();
    let calibration_markdown = calibration
        .iter()
        .map(|row| row.normalized_markdown_sha256.clone())
        .collect::<BTreeSet<_>>();
    let prospective_markdown = prospective
        .iter()
        .map(|row| row.normalized_markdown_sha256.clone())
        .collect::<BTreeSet<_>>();
    let overlapping_document_keys = calibration_keys
        .intersection(&prospective_keys)
        .cloned()
        .collect::<Vec<_>>();
    let overlapping_source_sha256 = calibration_sources
        .intersection(&prospective_sources)
        .cloned()
        .collect::<Vec<_>>();
    let overlapping_normalized_markdown_sha256 = calibration_markdown
        .intersection(&prospective_markdown)
        .cloned()
        .collect::<Vec<_>>();
    HeldOutSplitEvidence {
        selection_boundary_commit: selection_boundary_commit.to_string(),
        calibration_document_keys: calibration_keys.into_iter().collect(),
        prospective_document_keys: prospective_keys.into_iter().collect(),
        identity_disjoint: overlapping_document_keys.is_empty()
            && overlapping_source_sha256.is_empty()
            && overlapping_normalized_markdown_sha256.is_empty(),
        overlapping_document_keys,
        overlapping_source_sha256,
        overlapping_normalized_markdown_sha256,
    }
}

fn held_out_source_authority(
    repository: &Path,
    row: &BehavioralPopulationRow,
    relation: BehavioralRelation,
) -> Result<PathBuf> {
    if relation == BehavioralRelation::SymbolAlpha {
        return Ok(PathBuf::from(&row.normalized_markdown_path));
    }
    if row.source_origin == "repository_owned" {
        return Ok(PathBuf::from(&row.source_locator));
    }
    if row.source_origin != "external_input" {
        return Err(invalid(format!(
            "stale_contract_or_population: unsupported held-out source origin for {}",
            row.document_key
        )));
    }
    let source_ir_path = repository
        .join("generated/source_ir")
        .join(&row.document_key)
        .join("source_ir.json");
    let source_ir: Value = serde_json::from_slice(&fs::read(&source_ir_path).map_err(|error| {
        invalid(format!(
            "stale_contract_or_population: held-out SourceIR authority is unavailable for {}: {error}",
            row.document_key
        ))
    })?)?;
    let canonical = source_ir
        .pointer("/source/canonical_path")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            invalid(format!(
                "stale_contract_or_population: held-out SourceIR has no canonical source for {}",
                row.document_key
            ))
        })?;
    if Path::new(canonical)
        .file_name()
        .and_then(|value| value.to_str())
        != Some(row.source_locator.as_str())
    {
        return Err(invalid(format!(
            "stale_contract_or_population: held-out portable source authority differs for {}",
            row.document_key
        )));
    }
    Ok(PathBuf::from(canonical))
}

fn held_out_strata(
    documents: &[HeldOutDocumentEvidence],
    attempts: &[HeldOutAttemptEvidence],
    relations: &[BehavioralRelation],
) -> Vec<HeldOutStratumOutcome> {
    let mut dimensions = BTreeMap::<String, BTreeSet<String>>::new();
    dimensions
        .entry("overall".to_string())
        .or_default()
        .insert("all".to_string());
    for document in documents {
        dimensions
            .entry("vendor".to_string())
            .or_default()
            .insert(document.vendor.clone());
        dimensions
            .entry("family".to_string())
            .or_default()
            .insert(document.family.clone());
    }
    dimensions.insert(
        "vendor_novelty".to_string(),
        ["seen_in_calibration", "novel"]
            .into_iter()
            .map(str::to_string)
            .collect(),
    );
    dimensions.insert(
        "family_novelty".to_string(),
        ["seen_in_calibration", "novel"]
            .into_iter()
            .map(str::to_string)
            .collect(),
    );
    dimensions.insert(
        "category".to_string(),
        [
            "cpu-isa",
            "methodology-guide",
            "physical-link",
            "platform-system-ip",
            "register-ip",
            "wire-protocol",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
    );
    dimensions.insert(
        "layout".to_string(),
        ["compact", "medium", "long"]
            .into_iter()
            .map(str::to_string)
            .collect(),
    );

    let mut outcomes = Vec::new();
    for relation in relations {
        for (dimension, values) in &dimensions {
            for value in values {
                let document_keys = documents
                    .iter()
                    .filter(|document| held_out_document_in_stratum(document, dimension, value))
                    .map(|document| document.document_key.clone())
                    .collect::<Vec<_>>();
                let selected = attempts
                    .iter()
                    .filter(|attempt| {
                        attempt.relation == *relation
                            && document_keys.contains(&attempt.document_key)
                    })
                    .collect::<Vec<_>>();
                let passes = selected
                    .iter()
                    .filter(|attempt| attempt.state == BehavioralRunState::Pass)
                    .count();
                let failures = selected
                    .iter()
                    .filter(|attempt| attempt.state == BehavioralRunState::Fail)
                    .count();
                let unmeasurable = selected
                    .iter()
                    .filter(|attempt| attempt.state == BehavioralRunState::Unmeasurable)
                    .count();
                let invalid_count = selected
                    .iter()
                    .filter(|attempt| attempt.state == BehavioralRunState::Invalid)
                    .count();
                let state = if invalid_count > 0 {
                    BehavioralRunState::Invalid
                } else if failures > 0 {
                    BehavioralRunState::Fail
                } else if document_keys.is_empty() || unmeasurable > 0 {
                    BehavioralRunState::Unmeasurable
                } else {
                    BehavioralRunState::Pass
                };
                let limitation = if document_keys.is_empty() {
                    Some("no_prospective_denominator".to_string())
                } else if invalid_count > 0 {
                    Some("invalid_attempts_present".to_string())
                } else if unmeasurable > 0 {
                    Some("unmeasurable_attempts_excluded_from_interval".to_string())
                } else {
                    None
                };
                outcomes.push(HeldOutStratumOutcome {
                    relation: *relation,
                    dimension: dimension.clone(),
                    value: value.clone(),
                    document_keys,
                    declared_documents: selected.len(),
                    passes,
                    failures,
                    unmeasurable,
                    invalid: invalid_count,
                    state,
                    limitation,
                    uncertainty: held_out_uncertainty(passes, passes + failures),
                });
            }
        }
    }
    outcomes
}

fn held_out_document_in_stratum(
    document: &HeldOutDocumentEvidence,
    dimension: &str,
    value: &str,
) -> bool {
    match dimension {
        "overall" => value == "all",
        "vendor" => document.vendor == value,
        "family" => document.family == value,
        "category" => document.category == value,
        "layout" => document.layout == value,
        "vendor_novelty" => {
            document.vendor_novel == (value == "novel")
                && matches!(value, "novel" | "seen_in_calibration")
        }
        "family_novelty" => {
            document.family_novel == (value == "novel")
                && matches!(value, "novel" | "seen_in_calibration")
        }
        _ => false,
    }
}

fn held_out_uncertainty(passes: usize, completed: usize) -> HeldOutUncertainty {
    const SCOPE: &str = "Descriptive interval over completed documents in this frozen, non-random prospective stratum; it is not a claim about specifications outside the frozen population.";
    if completed == 0 {
        return HeldOutUncertainty {
            method: "unavailable_no_completed_denominator".to_string(),
            confidence_basis_points: None,
            sample_size: 0,
            pass_numerator: passes,
            point_estimate_parts_per_million: None,
            lower_parts_per_million: None,
            upper_parts_per_million: None,
            scope_limit: SCOPE.to_string(),
        };
    }
    let n = completed as f64;
    let proportion = passes as f64 / n;
    let z = 1.959_963_984_540_054_f64;
    let z_squared = z * z;
    let denominator = 1.0 + z_squared / n;
    let center = (proportion + z_squared / (2.0 * n)) / denominator;
    let margin = z * ((proportion * (1.0 - proportion) / n + z_squared / (4.0 * n * n)).sqrt())
        / denominator;
    let to_parts_per_million =
        |value: f64| -> u32 { (value.clamp(0.0, 1.0) * 1_000_000.0).round() as u32 };
    HeldOutUncertainty {
        method: "wilson_score_95_percent".to_string(),
        confidence_basis_points: Some(9_500),
        sample_size: completed,
        pass_numerator: passes,
        point_estimate_parts_per_million: Some(to_parts_per_million(proportion)),
        lower_parts_per_million: Some(to_parts_per_million(center - margin)),
        upper_parts_per_million: Some(to_parts_per_million(center + margin)),
        scope_limit: SCOPE.to_string(),
    }
}

fn aggregate_held_out_coverage(
    documents: &[HeldOutDocumentEvidence],
    attempts: &[HeldOutAttemptEvidence],
    relation_count: usize,
) -> HeldOutAggregateCoverage {
    let declared_documents = documents
        .iter()
        .map(|document| document.document_key.clone())
        .collect::<Vec<_>>();
    let attempted_documents = attempts
        .iter()
        .map(|attempt| attempt.document_key.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let mut fully_completed_documents = Vec::new();
    let mut partially_completed_documents = Vec::new();
    let mut unmeasurable_documents = Vec::new();
    let mut invalid_documents = Vec::new();
    for document in documents {
        let document_attempts = attempts
            .iter()
            .filter(|attempt| attempt.document_key == document.document_key)
            .collect::<Vec<_>>();
        let completed = document_attempts
            .iter()
            .filter(|attempt| {
                matches!(
                    attempt.state,
                    BehavioralRunState::Pass | BehavioralRunState::Fail
                )
            })
            .count();
        if completed == relation_count {
            fully_completed_documents.push(document.document_key.clone());
        } else if completed > 0 {
            partially_completed_documents.push(document.document_key.clone());
        }
        if document_attempts
            .iter()
            .any(|attempt| attempt.state == BehavioralRunState::Unmeasurable)
        {
            unmeasurable_documents.push(document.document_key.clone());
        }
        if document_attempts
            .iter()
            .any(|attempt| attempt.state == BehavioralRunState::Invalid)
        {
            invalid_documents.push(document.document_key.clone());
        }
    }
    let coverages = attempts
        .iter()
        .filter_map(|attempt| attempt.coverage.as_ref())
        .collect::<Vec<_>>();
    HeldOutAggregateCoverage {
        declared_documents,
        attempted_documents,
        fully_completed_documents,
        partially_completed_documents,
        unmeasurable_documents,
        invalid_documents,
        declared_attempts: documents.len() * relation_count,
        pass_attempts: attempts
            .iter()
            .filter(|attempt| attempt.state == BehavioralRunState::Pass)
            .count(),
        fail_attempts: attempts
            .iter()
            .filter(|attempt| attempt.state == BehavioralRunState::Fail)
            .count(),
        unmeasurable_attempts: attempts
            .iter()
            .filter(|attempt| attempt.state == BehavioralRunState::Unmeasurable)
            .count(),
        invalid_attempts: attempts
            .iter()
            .filter(|attempt| attempt.state == BehavioralRunState::Invalid)
            .count(),
        baseline_top_level_fields: coverages
            .iter()
            .map(|coverage| coverage.baseline_top_level_fields)
            .sum(),
        transformed_top_level_fields: coverages
            .iter()
            .map(|coverage| coverage.transformed_top_level_fields)
            .sum(),
        baseline_proof_claims: coverages
            .iter()
            .map(|coverage| coverage.baseline_proof_claims)
            .sum(),
        transformed_proof_claims: coverages
            .iter()
            .map(|coverage| coverage.transformed_proof_claims)
            .sum(),
        compared_leaf_values: coverages
            .iter()
            .map(|coverage| coverage.compared_leaf_values)
            .sum(),
        expected_deltas: coverages
            .iter()
            .map(|coverage| {
                coverage.expected_symbol_deltas
                    + coverage.expected_reviewed_span_deltas
                    + coverage.expected_semantic_deltas
            })
            .sum(),
        observed_deltas: coverages
            .iter()
            .map(|coverage| {
                coverage.observed_symbol_deltas
                    + coverage.observed_reviewed_span_deltas
                    + coverage.observed_semantic_deltas
            })
            .sum(),
    }
}

/// Execute the closed comparator-sensitivity matrix declared by conformance authority.
pub fn qualify_negative_sensitivity_matrix() -> Result<NegativeSensitivityReport> {
    let repository = crate::project_data::repository_root()?;
    let matrix_path = resolve_repository_file(
        &repository,
        Path::new(NEGATIVE_SENSITIVITY_MATRIX_PATH),
        "semantic-negative sensitivity matrix",
    )?;
    let matrix_bytes = fs::read(&matrix_path)?;
    let matrix: NegativeSensitivityMatrix = serde_json::from_slice(&matrix_bytes)?;
    validate_negative_sensitivity_matrix(&matrix)?;

    let baseline = synthetic_negative_stage_map();
    let normalization = NormalizationSpec {
        relation: BehavioralRelation::SemanticNegative,
        baseline_run_root: ".project-data/tmp/negative-matrix".to_string(),
        transformed_run_root: ".project-data/tmp/negative-matrix".to_string(),
        baseline_source: "negative-matrix.md".to_string(),
        transformed_source: "negative-matrix.md".to_string(),
        exact_transformed_to_baseline: BTreeMap::new(),
        identifier_transformed_to_baseline: BTreeMap::new(),
        reviewed_text_by_field: BTreeMap::new(),
    };
    let mut controls = Vec::new();
    for declaration in &matrix.controls {
        controls.push(execute_negative_control(
            declaration,
            &baseline,
            &normalization,
        )?);
    }
    let passed = controls.iter().all(|control| control.passed);
    Ok(NegativeSensitivityReport {
        schema_version: matrix.schema_version,
        matrix_path: NEGATIVE_SENSITIVITY_MATRIX_PATH.to_string(),
        matrix_sha256: sha256_bytes(&matrix_bytes),
        controls,
        attempt_dispositions: matrix.attempt_dispositions,
        passed,
    })
}

fn validate_negative_sensitivity_matrix(matrix: &NegativeSensitivityMatrix) -> Result<()> {
    let expected_kinds = [
        NegativeControlKind::Omission,
        NegativeControlKind::Contradiction,
        NegativeControlKind::RelationReversal,
        NegativeControlKind::ValueChange,
        NegativeControlKind::TimingChange,
        NegativeControlKind::UndeclaredSymbol,
        NegativeControlKind::MisleadingName,
        NegativeControlKind::ProofCorruption,
        NegativeControlKind::DisabledStage,
    ];
    let kinds = matrix
        .controls
        .iter()
        .map(|control| control.kind)
        .collect::<Vec<_>>();
    let mut ids = BTreeSet::new();
    if matrix.schema_version != 1
        || matrix.owner != "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.c"
        || matrix.unaffected_complement != REVIEWED_COMPLEMENT
        || kinds.len() != expected_kinds.len()
        || !expected_kinds.iter().all(|kind| kinds.contains(kind))
        || matrix
            .controls
            .iter()
            .any(|control| control.control_id.is_empty() || !ids.insert(&control.control_id))
    {
        return Err(invalid(
            "stale_contract_or_population: semantic-negative matrix identity or coverage differs",
        ));
    }
    let expected_dispositions = BTreeMap::from([
        (
            "ambiguous_or_nonbijective_transform".to_string(),
            BehavioralRunState::Invalid,
        ),
        (
            "authority_unavailable".to_string(),
            BehavioralRunState::Unmeasurable,
        ),
        (
            "partial_or_escaped_run".to_string(),
            BehavioralRunState::Invalid,
        ),
        (
            "provider_unavailable".to_string(),
            BehavioralRunState::Unmeasurable,
        ),
        (
            "stale_contract_or_population".to_string(),
            BehavioralRunState::Invalid,
        ),
        (
            "vacuous_baseline".to_string(),
            BehavioralRunState::Unmeasurable,
        ),
    ]);
    if matrix.attempt_dispositions != expected_dispositions {
        return Err(invalid(
            "stale_contract_or_population: semantic-negative attempt dispositions differ",
        ));
    }
    for control in &matrix.controls {
        let disabled = control.kind == NegativeControlKind::DisabledStage;
        if disabled != matches!(control.operation, NegativeMutationOperation::DisableStage)
            || disabled != (control.expected_state == BehavioralRunState::Invalid)
            || disabled != (control.expected_failure_id == "partial_or_escaped_run")
            || (!disabled
                && (control
                    .path
                    .as_deref()
                    .is_none_or(|path| !path.starts_with('/'))
                    || control.baseline_value.is_none()
                    || (matches!(control.operation, NegativeMutationOperation::Remove)
                        && control.transformed_value.is_some())
                    || (matches!(control.operation, NegativeMutationOperation::Replace)
                        && control.transformed_value.is_none())
                    || control.expected_state != BehavioralRunState::Fail
                    || control.expected_failure_id != "undeclared_semantic_delta"))
        {
            return Err(invalid(format!(
                "stale_contract_or_population: semantic-negative control schema differs: {}",
                control.control_id
            )));
        }
    }
    Ok(())
}

fn synthetic_negative_stage_map() -> BTreeMap<BehavioralStage, StageArtifact> {
    BehavioralStage::ALL
        .into_iter()
        .map(|stage| {
            let value = serde_json::json!({
                "stage": stage.as_str(),
                "document_identity": {"document_key": "negative_matrix"},
                "artifact_layout": {"artifact_root": ".project-data/tmp/negative-matrix"},
                "canonical_fact": {
                    "obligation": "drive",
                    "polarity": "required",
                    "relation": "producer_to_consumer",
                    "value": 1,
                    "timing_cycles": 2,
                    "symbol": "declared_signal",
                    "actor_name": "neutral_endpoint"
                },
                "residual_decisions": [],
                "proof_ledger": {
                    "schema_version": 1,
                    "ruleset_sha256": "1".repeat(64),
                    "claims": [{
                        "address": {
                            "stage": stage.as_str(),
                            "surface": "canonical_fact",
                            "stable_record_key": "record-00000000"
                        },
                        "conclusion_sha256": "2".repeat(64),
                        "rule_id": "negative.matrix.canonical_fact.v1",
                        "premises": [],
                        "symbol_uses": [],
                        "confidence": "deterministic"
                    }]
                }
            });
            (
                stage,
                StageArtifact {
                    identity: ArtifactIdentity {
                        path: format!(".project-data/tmp/negative-matrix/{}.json", stage.as_str()),
                        sha256: "0".repeat(64),
                        byte_count: 1,
                    },
                    value,
                },
            )
        })
        .collect()
}

fn execute_negative_control(
    declaration: &NegativeControlDeclaration,
    baseline: &BTreeMap<BehavioralStage, StageArtifact>,
    normalization: &NormalizationSpec,
) -> Result<NegativeControlResult> {
    let mut transformed = baseline.clone();
    let required_delta_observed = match declaration.operation {
        NegativeMutationOperation::DisableStage => transformed.remove(&declaration.stage).is_some(),
        NegativeMutationOperation::Remove => {
            let stage = transformed
                .get_mut(&declaration.stage)
                .ok_or_else(|| invalid("negative-matrix mutation stage is absent"))?;
            let pointer = declaration.path.as_deref().expect("validated remove path");
            stage.value.pointer(pointer) == declaration.baseline_value.as_ref()
                && remove_json_pointer(&mut stage.value, pointer).is_some()
                && stage.value.pointer(pointer).is_none()
        }
        NegativeMutationOperation::Replace => {
            let stage = transformed
                .get_mut(&declaration.stage)
                .ok_or_else(|| invalid("negative-matrix mutation stage is absent"))?;
            let pointer = declaration.path.as_deref().expect("validated replace path");
            let baseline_matches =
                stage.value.pointer(pointer) == declaration.baseline_value.as_ref();
            let replaced = set_json_pointer(
                &mut stage.value,
                pointer,
                declaration
                    .transformed_value
                    .clone()
                    .expect("validated replacement value"),
            );
            baseline_matches
                && replaced
                && stage.value.pointer(pointer) == declaration.transformed_value.as_ref()
        }
    };

    let (observed_state, _, failures) = compare_artifact_maps(
        baseline,
        &transformed,
        "negative_matrix",
        "negative_matrix",
        normalization,
        None,
    )?;
    let invariant_comparison_rejected = observed_state != BehavioralRunState::Pass
        && failures
            .iter()
            .any(|failure| failure.starts_with(&declaration.expected_failure_id));

    let mut restored = transformed;
    match declaration.operation {
        NegativeMutationOperation::DisableStage => {
            restored.insert(
                declaration.stage,
                baseline
                    .get(&declaration.stage)
                    .expect("baseline has every stage")
                    .clone(),
            );
        }
        NegativeMutationOperation::Remove | NegativeMutationOperation::Replace => {
            let stage = restored
                .get_mut(&declaration.stage)
                .expect("mutation retained its stage");
            set_json_pointer(
                &mut stage.value,
                declaration
                    .path
                    .as_deref()
                    .expect("validated mutation path"),
                declaration
                    .baseline_value
                    .clone()
                    .expect("validated baseline value"),
            );
        }
    }
    let unaffected_complement_preserved = stage_maps_equal(baseline, &restored);
    let passed = required_delta_observed
        && invariant_comparison_rejected
        && unaffected_complement_preserved
        && observed_state == declaration.expected_state;
    Ok(NegativeControlResult {
        control_id: declaration.control_id.clone(),
        kind: declaration.kind,
        expected_state: declaration.expected_state,
        observed_state,
        required_delta_observed,
        invariant_comparison_rejected,
        unaffected_complement_preserved,
        passed,
    })
}

fn stage_maps_equal(
    left: &BTreeMap<BehavioralStage, StageArtifact>,
    right: &BTreeMap<BehavioralStage, StageArtifact>,
) -> bool {
    left.len() == right.len()
        && left.iter().all(|(stage, artifact)| {
            right
                .get(stage)
                .is_some_and(|other| artifact.value == other.value)
        })
}

/// Execute one complete frozen relation and write machine-readable evidence below `output_root`.
pub fn qualify_behavioral_relation(
    request: &BehavioralQualificationRequest,
) -> Result<BehavioralQualificationReport> {
    validate_request(request)?;
    let repository = crate::project_data::repository_root()?;
    let source_authority = resolve_source_authority(&repository, &request.source_authority)?;
    let source_bytes = fs::read(&source_authority)?;
    let source_sha256 = sha256_bytes(&source_bytes);
    if source_sha256 != request.expected_source_sha256 {
        return Err(invalid(format!(
            "stale_contract_or_population: behavioral source SHA-256 differs: {source_sha256} != {}",
            request.expected_source_sha256
        )));
    }
    ensure_same_filesystem(&repository, &source_authority)?;
    ensure_relation_provider(request)?;
    let output_root = prepare_output_root(&repository, &request.output_root)?;
    let prior_memory = resolve_repository_file(&repository, &request.prior_memory, "prior memory")?;
    let prior_memory_sha256 = sha256_file(&prior_memory)?;
    let contract_sha256 = sha256_file(&repository.join(CONTRACT_PATH))?;
    let reviewed_recipe = load_reviewed_recipe(
        &repository,
        request,
        &source_sha256,
        &request.source_authority,
    )?;

    let extension = source_authority
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let portable_id = source_authority
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| invalid("behavioral source has no UTF-8 portable basename"))?
        .to_string();

    let baseline_input = output_root.join("inputs/baseline").join(&portable_id);
    copy_exact(&source_authority, &baseline_input)?;
    let baseline_run_root = output_root.join("baseline");
    let baseline = run_pipeline(
        &baseline_input,
        &baseline_run_root,
        &prior_memory,
        &repository,
    )?;

    let (transformed_input, renames, reviewed_spans) = match request.relation {
        BehavioralRelation::UnchangedSource => (baseline_input.clone(), Vec::new(), Vec::new()),
        BehavioralRelation::AdversarialIdentity => {
            let filename = format!(
                "clock_reset_handshake_reference_{:016x}.{}",
                request.transform_seed, extension
            );
            let path = output_root.join("inputs/transformed").join(filename);
            copy_exact(&source_authority, &path)?;
            (path, Vec::new(), Vec::new())
        }
        BehavioralRelation::SymbolAlpha => {
            let source_text = String::from_utf8(source_bytes.clone())
                .map_err(|_| invalid("symbol-alpha input must be UTF-8 normalized Markdown"))?;
            let symbols = derive_source_symbol_catalog(&baseline.evidence_ir, &source_text);
            if symbols.is_empty() {
                return Err(invalid(
                    "eligible_symbol_surface_absent: symbol-alpha baseline has no typed opaque signal declaration",
                ));
            }
            let (transformed, renames) =
                alpha_transform(&source_text, &symbols, request.transform_seed)?;
            let path = output_root.join("inputs/transformed").join(&portable_id);
            write_new(&path, transformed.as_bytes())?;
            (path, renames, Vec::new())
        }
        BehavioralRelation::StructurePreservingParaphrase
        | BehavioralRelation::HarmlessLayout
        | BehavioralRelation::SemanticNegative => {
            let source_text = String::from_utf8(source_bytes.clone()).map_err(|_| {
                invalid("reviewed transform input must be UTF-8 normalized Markdown")
            })?;
            let loaded = reviewed_recipe
                .as_ref()
                .ok_or_else(|| invalid("reviewed relation has no loaded recipe"))?;
            let (transformed, spans) = apply_reviewed_recipe(&source_text, &loaded.recipe)?;
            let path = output_root.join("inputs/transformed").join(&portable_id);
            write_new(&path, transformed.as_bytes())?;
            (path, Vec::new(), spans)
        }
    };

    let transformed_run_root = output_root.join("transformed");
    let transformed = run_pipeline(
        &transformed_input,
        &transformed_run_root,
        &prior_memory,
        &repository,
    )?;

    let baseline_source_relative = repository_relative(&repository, &baseline_input)?;
    let transformed_source_relative = repository_relative(&repository, &transformed_input)?;
    let identifier_transformed_to_baseline = renames
        .iter()
        .map(|rename| {
            (
                rename.replacement.to_ascii_lowercase(),
                rename.original.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let mut exact_transformed_to_baseline = BTreeMap::new();
    let mut reviewed_text_by_field = BTreeMap::<String, BTreeMap<String, String>>::new();
    let mut reviewed_text_reverse_by_field = BTreeMap::<String, BTreeMap<String, String>>::new();
    if request.relation == BehavioralRelation::AdversarialIdentity {
        exact_transformed_to_baseline.insert(
            transformed.document_key.clone(),
            baseline.document_key.clone(),
        );
        exact_transformed_to_baseline.insert(
            transformed_input
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_string(),
            baseline_input
                .file_name()
                .and_then(|value| value.to_str())
                .unwrap_or_default()
                .to_string(),
        );
    }
    if request.relation == BehavioralRelation::SymbolAlpha {
        exact_transformed_to_baseline
            .insert(sha256_file(&transformed_input)?, source_sha256.clone());
    }
    if let Some(loaded) = &reviewed_recipe {
        exact_transformed_to_baseline
            .insert(sha256_file(&transformed_input)?, source_sha256.clone());
        let mut identifier_projection_reverse = BTreeMap::new();
        for change in &loaded.recipe.changed_spans {
            for field in &change.allowed_provenance_fields {
                let mapping = reviewed_text_by_field.entry(field.clone()).or_default();
                if let Some(existing) =
                    mapping.insert(change.exact_after.clone(), change.exact_before.clone())
                    && existing != change.exact_before
                {
                    return Err(invalid(format!(
                        "ambiguous_or_nonbijective_transform: reviewed replacement maps twice in {field}: {}",
                        change.change_id
                    )));
                }
                let reverse = reviewed_text_reverse_by_field
                    .entry(field.clone())
                    .or_default();
                if let Some(existing) =
                    reverse.insert(change.exact_before.clone(), change.exact_after.clone())
                    && existing != change.exact_after
                {
                    return Err(invalid(format!(
                        "ambiguous_or_nonbijective_transform: reviewed replacement reverses twice in {field}: {}",
                        change.change_id
                    )));
                }
            }
            if change.allow_source_bound_identifier_projection {
                let transformed_projection =
                    source_bound_identifier_projection(&change.exact_after);
                let baseline_projection = source_bound_identifier_projection(&change.exact_before);
                if !transformed_projection.is_empty()
                    && transformed_projection != baseline_projection
                {
                    if let Some(existing) = exact_transformed_to_baseline
                        .insert(transformed_projection.clone(), baseline_projection.clone())
                        && existing != baseline_projection
                    {
                        return Err(invalid(format!(
                            "ambiguous_or_nonbijective_transform: reviewed identifier projection maps twice: {}",
                            change.change_id
                        )));
                    }
                    if let Some(existing) = identifier_projection_reverse
                        .insert(baseline_projection.clone(), transformed_projection.clone())
                        && existing != transformed_projection
                    {
                        return Err(invalid(format!(
                            "ambiguous_or_nonbijective_transform: reviewed identifier projection reverses twice: {}",
                            change.change_id
                        )));
                    }
                }
            }
        }
    }
    let normalization = NormalizationSpec {
        relation: request.relation,
        baseline_run_root: repository_relative(&repository, &baseline_run_root)?,
        transformed_run_root: repository_relative(&repository, &transformed_run_root)?,
        baseline_source: baseline_source_relative.clone(),
        transformed_source: transformed_source_relative.clone(),
        exact_transformed_to_baseline,
        identifier_transformed_to_baseline,
        reviewed_text_by_field,
    };
    let mut required_delta_evidence = Vec::new();
    let mut observed_dependent_proof_deltas = 0usize;
    let mut invariant_comparison_rejected = false;
    let (mut state, stages, mut failures) =
        if request.relation == BehavioralRelation::SemanticNegative {
            let loaded = reviewed_recipe
                .as_ref()
                .ok_or_else(|| invalid("semantic-negative relation has no loaded recipe"))?;
            let (invariant_state, _, _) =
                compare_pipelines(&baseline, &transformed, &normalization, None)?;
            invariant_comparison_rejected = invariant_state == BehavioralRunState::Fail;
            let (observed, proof_deltas, observation_failures) =
                observe_required_negative_deltas(&loaded.recipe, &baseline, &transformed)?;
            required_delta_evidence = observed;
            observed_dependent_proof_deltas = proof_deltas;
            let all_required_observed = observation_failures.is_empty();
            let (_, stages, mut failures) = compare_pipelines(
                &baseline,
                &transformed,
                &normalization,
                all_required_observed.then_some(&loaded.recipe),
            )?;
            failures.extend(observation_failures);
            if !invariant_comparison_rejected {
                failures.push(
                    "missing_expected_delta: invariant comparator accepted semantic negative"
                        .to_string(),
                );
            }
            (run_state(&failures), stages, failures)
        } else {
            compare_pipelines(&baseline, &transformed, &normalization, None)?
        };
    if let Some(loaded) = &reviewed_recipe {
        failures.extend(validate_preserved_conclusions(
            &loaded.recipe,
            &baseline,
            &transformed,
        )?);
        if fs::read(&baseline_input)? == fs::read(&transformed_input)? {
            failures.push("missing_expected_delta: reviewed source did not change".to_string());
        }
        state = run_state(&failures);
    }
    let lexical_order_changed = lexical_order_changed(&renames);
    let reviewed_recipe_evidence = reviewed_recipe
        .as_ref()
        .map(|loaded| ReviewedRecipeEvidence {
            recipe_id: loaded.recipe.recipe_id.clone(),
            manifest_path: loaded.manifest_path.clone(),
            recipe_path: loaded.recipe_path.clone(),
            recipe_sha256: loaded.recipe_sha256.clone(),
            review_status: loaded.recipe.review_status.clone(),
            changed_spans: reviewed_spans.clone(),
            preserved_conclusions: loaded.recipe.preserved_conclusions.len(),
            unaffected_complement: loaded.recipe.unaffected_complement.clone(),
            unmeasurable_source_surfaces: loaded.recipe.unmeasurable_source_surfaces.clone(),
            semantic_negative: loaded.recipe.semantic_negative_kind.map(|kind| {
                SemanticNegativeEvidence {
                    kind,
                    required_deltas: required_delta_evidence.clone(),
                    dependent_proof_deltas: observed_dependent_proof_deltas,
                    invariant_comparison_rejected,
                }
            }),
        });
    let transform = TransformEvidence {
        relation: request.relation,
        seed: request.transform_seed,
        baseline_source: baseline_source_relative,
        transformed_source: transformed_source_relative,
        baseline_document_key: baseline.document_key.clone(),
        transformed_document_key: transformed.document_key.clone(),
        source_bytes_equal: fs::read(&baseline_input)? == fs::read(&transformed_input)?,
        lexical_order_changed,
        symbol_renames: renames,
        reviewed_recipe: reviewed_recipe_evidence,
    };
    let recipe = TransformRecipe {
        relation: request.relation,
        seed: request.transform_seed,
        baseline_source: &transform.baseline_source,
        transformed_source: &transform.transformed_source,
        symbol_renames: &transform.symbol_renames,
    };
    let recipe_sha256 = if let Some(loaded) = &reviewed_recipe {
        loaded.recipe_sha256.clone()
    } else {
        sha256_bytes(&serde_json::to_vec(&recipe)?)
    };

    let coverage = BehavioralCoverage {
        required_stages: BehavioralStage::ALL.len(),
        completed_stages: stages.len(),
        baseline_top_level_fields: stages
            .iter()
            .map(|stage| stage.baseline_top_level_fields)
            .sum(),
        transformed_top_level_fields: stages
            .iter()
            .map(|stage| stage.transformed_top_level_fields)
            .sum(),
        baseline_proof_claims: stages.iter().map(|stage| stage.baseline_proof_claims).sum(),
        transformed_proof_claims: stages
            .iter()
            .map(|stage| stage.transformed_proof_claims)
            .sum(),
        compared_leaf_values: stages.iter().map(|stage| stage.compared_leaf_values).sum(),
        expected_symbol_deltas: transform.symbol_renames.len(),
        observed_symbol_deltas: transform
            .symbol_renames
            .iter()
            .filter(|rename| rename.occurrence_count > 0)
            .count(),
        expected_reviewed_span_deltas: reviewed_spans.len(),
        observed_reviewed_span_deltas: reviewed_spans
            .iter()
            .filter(|span| span.occurrence_count == 1)
            .count(),
        preserved_conclusions: reviewed_recipe
            .as_ref()
            .map_or(0, |loaded| loaded.recipe.preserved_conclusions.len()),
        expected_semantic_deltas: reviewed_recipe
            .as_ref()
            .map_or(0, |loaded| loaded.recipe.required_deltas.len()),
        observed_semantic_deltas: required_delta_evidence
            .iter()
            .filter(|delta| delta.observed)
            .count(),
    };
    let report = BehavioralQualificationReport {
        schema_version: BEHAVIORAL_SCHEMA_VERSION,
        relation: request.relation,
        input_plane: request.relation.input_plane().to_string(),
        state,
        identity: EvidenceIdentity {
            contract_sha256,
            production_revision: request.production_revision.clone(),
            source_sha256: source_sha256.clone(),
            transform_recipe_sha256: recipe_sha256,
            prior_memory_sha256,
            tool_sha256: sha256_bytes(include_bytes!("behavioral_genericity.rs")),
        },
        source: SourceAuthorityEvidence {
            portable_id,
            sha256: source_sha256,
            byte_count: source_bytes.len() as u64,
            copied_to_repository_scratch: true,
            same_repository_filesystem: true,
        },
        transform,
        stages,
        coverage,
        failures,
    };
    fs::write(
        output_root.join(EVIDENCE_FILE),
        serde_json::to_vec_pretty(&report)?,
    )?;
    Ok(report)
}

fn validate_request(request: &BehavioralQualificationRequest) -> Result<()> {
    validate_relative_path(&request.output_root, "behavioral output root")?;
    if !request.output_root.starts_with(TEMP_ROOT) || request.output_root == Path::new(TEMP_ROOT) {
        return Err(invalid(format!(
            "behavioral output root must be a child of {TEMP_ROOT}: {}",
            request.output_root.display()
        )));
    }
    validate_relative_path(&request.prior_memory, "behavioral prior memory")?;
    if !is_lower_hex(&request.expected_source_sha256, 64) {
        return Err(invalid("expected behavioral source SHA-256 is invalid"));
    }
    if !is_lower_hex(&request.production_revision, 40) {
        return Err(invalid(
            "behavioral production revision must be a full Git id",
        ));
    }
    let repository = crate::project_data::repository_root()?;
    if repository.join(&request.output_root).exists() {
        return Err(invalid(format!(
            "behavioral output root already exists: {}",
            request.output_root.display()
        )));
    }
    let extension = request
        .source_authority
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    if request.relation.is_reviewed() {
        let recipe_id = request.review_recipe_id.as_deref().ok_or_else(|| {
            invalid("reviewed behavioral relation requires a registered recipe id")
        })?;
        if recipe_id.len() > 128
            || recipe_id.is_empty()
            || !recipe_id
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
        {
            return Err(invalid("reviewed behavioral recipe id is not portable"));
        }
    } else if request.review_recipe_id.is_some() {
        return Err(invalid(
            "non-reviewed behavioral relation cannot receive review authority",
        ));
    }
    match request.relation {
        BehavioralRelation::UnchangedSource | BehavioralRelation::AdversarialIdentity
            if extension != "pdf" =>
        {
            Err(invalid("full-capture behavioral relations require a PDF"))
        }
        BehavioralRelation::SymbolAlpha
        | BehavioralRelation::StructurePreservingParaphrase
        | BehavioralRelation::HarmlessLayout
        | BehavioralRelation::SemanticNegative
            if !matches!(extension.as_str(), "md" | "markdown") =>
        {
            Err(invalid(
                "text-projection behavioral relation requires normalized Markdown",
            ))
        }
        _ => Ok(()),
    }
}

fn ensure_relation_provider(request: &BehavioralQualificationRequest) -> Result<()> {
    if matches!(
        request.relation,
        BehavioralRelation::UnchangedSource | BehavioralRelation::AdversarialIdentity
    ) && !inspect_docling_runtime()?.is_ready()
    {
        return Err(invalid(
            "provider_unavailable: repository-local Docling runtime is unavailable",
        ));
    }
    Ok(())
}

fn load_reviewed_recipe(
    repository: &Path,
    request: &BehavioralQualificationRequest,
    source_sha256: &str,
    source_authority: &Path,
) -> Result<Option<LoadedReviewedRecipe>> {
    if !request.relation.is_reviewed() {
        return Ok(None);
    }
    let requested_id = request
        .review_recipe_id
        .as_deref()
        .ok_or_else(|| invalid("reviewed relation has no recipe id"))?;
    let manifest_path = repository.join(REVIEW_RECIPE_MANIFEST_PATH);
    let manifest_bytes = fs::read(&manifest_path)?;
    let manifest: ReviewedRecipeManifest = serde_json::from_slice(&manifest_bytes)?;
    if manifest.schema_version != REVIEWED_RECIPE_SCHEMA_VERSION
        || manifest.owner != "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii"
    {
        return Err(invalid("reviewed recipe manifest identity is stale"));
    }
    let mut seen = BTreeSet::new();
    if manifest
        .recipes
        .iter()
        .any(|declaration| !seen.insert(declaration.recipe_id.as_str()))
    {
        return Err(invalid("reviewed recipe manifest contains duplicate ids"));
    }
    let declaration = manifest
        .recipes
        .iter()
        .find(|declaration| declaration.recipe_id == requested_id)
        .ok_or_else(|| {
            invalid(format!(
                "reviewed recipe id is not registered: {requested_id}"
            ))
        })?;
    if declaration.relation != request.relation
        || declaration.review_role != "reviewed_calibration"
        || declaration.source_sha256 != source_sha256
        || !is_lower_hex(&declaration.sha256, 64)
    {
        return Err(invalid(format!(
            "reviewed recipe declaration is stale or relation-mismatched: {requested_id}"
        )));
    }
    let requested_source = if source_authority.is_absolute() {
        return Err(invalid(
            "reviewed recipe source authority must be repository-relative",
        ));
    } else {
        source_authority.to_string_lossy().into_owned()
    };
    if declaration.source_authority != requested_source {
        return Err(invalid(format!(
            "reviewed recipe source authority differs: {} != {requested_source}",
            declaration.source_authority
        )));
    }
    let recipe_relative = Path::new(&declaration.path);
    validate_relative_path(recipe_relative, "reviewed recipe")?;
    if !recipe_relative.starts_with(REVIEW_RECIPE_ROOT) {
        return Err(invalid(format!(
            "reviewed recipe is outside its closed root: {}",
            declaration.path
        )));
    }
    let recipe_path = resolve_repository_file(repository, recipe_relative, "reviewed recipe")?;
    let canonical_recipe_root = repository.join(REVIEW_RECIPE_ROOT).canonicalize()?;
    if !recipe_path.starts_with(&canonical_recipe_root) {
        return Err(invalid(format!(
            "reviewed recipe escaped its closed root: {}",
            declaration.path
        )));
    }
    let recipe_bytes = fs::read(&recipe_path)?;
    let recipe_sha256 = sha256_bytes(&recipe_bytes);
    if recipe_sha256 != declaration.sha256 {
        return Err(invalid(format!(
            "reviewed recipe SHA-256 differs: {recipe_sha256} != {}",
            declaration.sha256
        )));
    }
    let recipe: ReviewedTransformRecipe = serde_json::from_slice(&recipe_bytes)?;
    validate_reviewed_recipe(&recipe, declaration)?;
    Ok(Some(LoadedReviewedRecipe {
        manifest_path: REVIEW_RECIPE_MANIFEST_PATH.to_string(),
        recipe_path: declaration.path.clone(),
        recipe_sha256,
        recipe,
    }))
}

fn validate_reviewed_recipe(
    recipe: &ReviewedTransformRecipe,
    declaration: &ReviewedRecipeDeclaration,
) -> Result<()> {
    if recipe.schema_version != REVIEWED_RECIPE_SCHEMA_VERSION
        || recipe.recipe_id != declaration.recipe_id
        || recipe.relation != declaration.relation
        || recipe.source_authority != declaration.source_authority
        || recipe.source_sha256 != declaration.source_sha256
        || recipe.review_status != "approved"
        || recipe.unaffected_complement != REVIEWED_COMPLEMENT
    {
        return Err(invalid(format!(
            "reviewed recipe identity or closed complement is stale: {}",
            declaration.recipe_id
        )));
    }
    let exclusions = recipe
        .unmeasurable_source_surfaces
        .iter()
        .map(String::as_str)
        .collect::<BTreeSet<_>>();
    if exclusions
        != RICH_CAPTURE_EXCLUSIONS
            .iter()
            .copied()
            .collect::<BTreeSet<_>>()
        || exclusions.len() != recipe.unmeasurable_source_surfaces.len()
    {
        return Err(invalid(format!(
            "reviewed recipe rich-capture exclusions differ: {}",
            declaration.recipe_id
        )));
    }
    if recipe.changed_spans.is_empty() || recipe.preserved_conclusions.is_empty() {
        return Err(invalid(format!(
            "reviewed recipe lacks changed spans or preserved conclusions: {}",
            declaration.recipe_id
        )));
    }
    let mut change_ids = BTreeSet::new();
    let mut change_kinds = BTreeSet::new();
    for change in &recipe.changed_spans {
        let fields = change
            .allowed_provenance_fields
            .iter()
            .map(String::as_str)
            .collect::<BTreeSet<_>>();
        if change.change_id.is_empty()
            || !change_ids.insert(change.change_id.as_str())
            || change.exact_before.is_empty()
            || change.exact_after.is_empty()
            || change.exact_before == change.exact_after
            || change.expected_occurrences != 1
            || fields.is_empty()
            || fields.len() != change.allowed_provenance_fields.len()
            || !change.allow_source_bound_identifier_projection
            || !fields
                .iter()
                .all(|field| REVIEWED_PROVENANCE_FIELDS.contains(field))
        {
            return Err(invalid(format!(
                "reviewed recipe change is ambiguous or opens unsafe provenance: {}",
                change.change_id
            )));
        }
        change_kinds.insert(change.kind);
    }
    let expected_kinds = match recipe.relation {
        BehavioralRelation::StructurePreservingParaphrase => {
            [ReviewedChangeKind::SentenceParaphrase]
                .into_iter()
                .collect::<BTreeSet<_>>()
        }
        BehavioralRelation::HarmlessLayout => [
            ReviewedChangeKind::HeadingLayout,
            ReviewedChangeKind::TableLayout,
            ReviewedChangeKind::WhitespaceLayout,
            ReviewedChangeKind::FormattingLayout,
        ]
        .into_iter()
        .collect::<BTreeSet<_>>(),
        BehavioralRelation::SemanticNegative => [ReviewedChangeKind::SemanticTimingChange]
            .into_iter()
            .collect::<BTreeSet<_>>(),
        _ => {
            return Err(invalid(
                "non-reviewed relation entered reviewed recipe validator",
            ));
        }
    };
    if change_kinds != expected_kinds {
        return Err(invalid(format!(
            "reviewed recipe change-kind coverage differs: {}",
            declaration.recipe_id
        )));
    }
    let mut conclusion_ids = BTreeSet::new();
    let mut conclusion_stages = BTreeSet::new();
    let mut changed_conclusion = false;
    for conclusion in &recipe.preserved_conclusions {
        if conclusion.conclusion_id.is_empty()
            || !conclusion_ids.insert(conclusion.conclusion_id.as_str())
            || !conclusion.baseline_pointer.starts_with('/')
            || !conclusion.transformed_pointer.starts_with('/')
            || conclusion.baseline_value.is_empty()
            || conclusion.transformed_value.is_empty()
            || !matches!(
                conclusion.stage,
                BehavioralStage::EvidenceIr
                    | BehavioralStage::SemanticIr
                    | BehavioralStage::IntentIr
            )
        {
            return Err(invalid(format!(
                "reviewed recipe conclusion is incomplete: {}",
                conclusion.conclusion_id
            )));
        }
        conclusion_stages.insert(conclusion.stage);
        changed_conclusion |= conclusion.baseline_value != conclusion.transformed_value;
    }
    if !conclusion_stages.contains(&BehavioralStage::SemanticIr)
        || !conclusion_stages.contains(&BehavioralStage::IntentIr)
        || (recipe.relation == BehavioralRelation::StructurePreservingParaphrase
            && !changed_conclusion)
    {
        return Err(invalid(format!(
            "reviewed recipe conclusion coverage differs: {}",
            declaration.recipe_id
        )));
    }
    if recipe.relation == BehavioralRelation::SemanticNegative {
        if recipe.semantic_negative_kind.is_none()
            || recipe.required_deltas.is_empty()
            || recipe.dependent_proof_deltas.is_empty()
        {
            return Err(invalid(format!(
                "semantic-negative recipe lacks kind, required delta, or dependent proof: {}",
                declaration.recipe_id
            )));
        }
        validate_required_delta_declarations(recipe)?;
    } else if recipe.semantic_negative_kind.is_some()
        || !recipe.required_deltas.is_empty()
        || !recipe.dependent_proof_deltas.is_empty()
    {
        return Err(invalid(format!(
            "equivalence recipe carries semantic-negative authority: {}",
            declaration.recipe_id
        )));
    }
    Ok(())
}

fn validate_required_delta_declarations(recipe: &ReviewedTransformRecipe) -> Result<()> {
    let mut delta_ids = BTreeSet::new();
    for delta in &recipe.required_deltas {
        let pointers_are_valid = delta
            .baseline_pointer
            .as_deref()
            .into_iter()
            .chain(delta.transformed_pointer.as_deref())
            .all(|pointer| pointer.starts_with('/'));
        let shape_is_valid = match delta.kind {
            RequiredDeltaKind::Added => {
                delta.baseline_pointer.is_none()
                    && delta.baseline_value.is_none()
                    && delta.transformed_pointer.is_some()
                    && delta.transformed_value.is_some()
            }
            RequiredDeltaKind::Changed => {
                delta.baseline_pointer.is_some()
                    && delta.baseline_value.is_some()
                    && delta.transformed_pointer.is_some()
                    && delta.transformed_value.is_some()
                    && delta.baseline_value != delta.transformed_value
            }
            RequiredDeltaKind::Removed => {
                delta.baseline_pointer.is_some()
                    && delta.baseline_value.is_some()
                    && delta.transformed_pointer.is_none()
                    && delta.transformed_value.is_none()
            }
        };
        if delta.delta_id.is_empty()
            || !delta_ids.insert(delta.delta_id.as_str())
            || !matches!(
                delta.stage,
                BehavioralStage::SemanticIr | BehavioralStage::IntentIr
            )
            || !pointers_are_valid
            || !shape_is_valid
        {
            return Err(invalid(format!(
                "semantic-negative required delta is incomplete: {}",
                delta.delta_id
            )));
        }
    }

    let mut proof_ids = BTreeSet::new();
    for proof in &recipe.dependent_proof_deltas {
        let stages = proof.stages.iter().copied().collect::<BTreeSet<_>>();
        let expected_stages = proof_propagation_stages(&proof.address.stage)?;
        if proof.delta_id.is_empty()
            || !proof_ids.insert(proof.delta_id.as_str())
            || proof.rule_id.is_empty()
            || proof.address.surface.is_empty()
            || proof.address.stable_record_key.is_empty()
            || stages.len() != proof.stages.len()
            || stages != expected_stages
        {
            return Err(invalid(format!(
                "semantic-negative dependent proof declaration is incomplete: {}",
                proof.delta_id
            )));
        }
    }
    Ok(())
}

fn proof_propagation_stages(address_stage: &str) -> Result<BTreeSet<BehavioralStage>> {
    let stages = match address_stage {
        "evidence_ir" => [
            BehavioralStage::EvidenceIr,
            BehavioralStage::SemanticIr,
            BehavioralStage::IntentIr,
            BehavioralStage::IsfAdapter,
        ]
        .into_iter()
        .collect(),
        "semantic_ir" => [
            BehavioralStage::SemanticIr,
            BehavioralStage::IntentIr,
            BehavioralStage::IsfAdapter,
        ]
        .into_iter()
        .collect(),
        "intent_ir" => [BehavioralStage::IntentIr, BehavioralStage::IsfAdapter]
            .into_iter()
            .collect(),
        other => {
            return Err(invalid(format!(
                "semantic-negative proof address stage cannot propagate: {other}"
            )));
        }
    };
    Ok(stages)
}

fn apply_reviewed_recipe(
    source: &str,
    recipe: &ReviewedTransformRecipe,
) -> Result<(String, Vec<ReviewedSpanEvidence>)> {
    let mut located = Vec::new();
    for change in &recipe.changed_spans {
        let matches = source
            .match_indices(&change.exact_before)
            .map(|(offset, _)| offset)
            .collect::<Vec<_>>();
        if matches.len() != change.expected_occurrences || source.contains(&change.exact_after) {
            return Err(invalid(format!(
                "ambiguous_or_nonbijective_transform: reviewed span is stale or ambiguous: {}",
                change.change_id
            )));
        }
        located.push((matches[0], change));
    }
    located.sort_by_key(|(offset, _)| *offset);
    for pair in located.windows(2) {
        let (left_offset, left) = pair[0];
        let (right_offset, _) = pair[1];
        if left_offset + left.exact_before.len() > right_offset {
            return Err(invalid(format!(
                "ambiguous_or_nonbijective_transform: reviewed spans overlap: {}",
                left.change_id
            )));
        }
    }
    let mut transformed = source.to_string();
    for (offset, change) in located.iter().rev() {
        transformed.replace_range(
            *offset..(*offset + change.exact_before.len()),
            &change.exact_after,
        );
    }
    let mut reversed = transformed.clone();
    for (_, change) in &located {
        if reversed.matches(&change.exact_after).count() != 1 {
            return Err(invalid(format!(
                "ambiguous_or_nonbijective_transform: transformed reviewed span is ambiguous: {}",
                change.change_id
            )));
        }
        reversed = reversed.replacen(&change.exact_after, &change.exact_before, 1);
    }
    if reversed != source {
        return Err(invalid(
            "ambiguous_or_nonbijective_transform: reviewed changed-span complement is incomplete",
        ));
    }
    let evidence = located
        .into_iter()
        .map(|(offset, change)| ReviewedSpanEvidence {
            change_id: change.change_id.clone(),
            kind: change.kind,
            baseline_start_byte: offset,
            baseline_byte_count: change.exact_before.len(),
            transformed_byte_count: change.exact_after.len(),
            occurrence_count: change.expected_occurrences,
            baseline_sha256: sha256_bytes(change.exact_before.as_bytes()),
            transformed_sha256: sha256_bytes(change.exact_after.as_bytes()),
            allowed_provenance_fields: change.allowed_provenance_fields.clone(),
            allow_source_bound_identifier_projection: change
                .allow_source_bound_identifier_projection,
        })
        .collect();
    Ok((transformed, evidence))
}

fn validate_preserved_conclusions(
    recipe: &ReviewedTransformRecipe,
    baseline: &PipelineArtifacts,
    transformed: &PipelineArtifacts,
) -> Result<Vec<String>> {
    let mut failures = Vec::new();
    for conclusion in &recipe.preserved_conclusions {
        let baseline_stage = baseline
            .stages
            .get(&conclusion.stage)
            .ok_or_else(|| invalid("reviewed baseline conclusion stage is missing"))?;
        let baseline_value = baseline_stage
            .value
            .pointer(&conclusion.baseline_pointer)
            .and_then(Value::as_str)
            .ok_or_else(|| {
                invalid(format!(
                    "reviewed baseline conclusion pointer is stale: {}",
                    conclusion.conclusion_id
                ))
            })?;
        if baseline_value != conclusion.baseline_value {
            return Err(invalid(format!(
                "reviewed baseline conclusion value is stale: {}",
                conclusion.conclusion_id
            )));
        }
        let transformed_value = transformed
            .stages
            .get(&conclusion.stage)
            .and_then(|stage| stage.value.pointer(&conclusion.transformed_pointer))
            .and_then(Value::as_str);
        if transformed_value != Some(conclusion.transformed_value.as_str()) {
            failures.push(format!(
                "missing_expected_delta: preserved conclusion differs: {}",
                conclusion.conclusion_id
            ));
        }
    }
    Ok(failures)
}

fn observe_required_negative_deltas(
    recipe: &ReviewedTransformRecipe,
    baseline: &PipelineArtifacts,
    transformed: &PipelineArtifacts,
) -> Result<(Vec<RequiredDeltaEvidence>, usize, Vec<String>)> {
    let mut evidence = Vec::new();
    let mut failures = Vec::new();
    for delta in &recipe.required_deltas {
        let baseline_stage = baseline
            .stages
            .get(&delta.stage)
            .ok_or_else(|| invalid("semantic-negative baseline stage is missing"))?;
        let transformed_stage = transformed
            .stages
            .get(&delta.stage)
            .ok_or_else(|| invalid("semantic-negative transformed stage is missing"))?;
        let observed = match delta.kind {
            RequiredDeltaKind::Added => {
                let pointer = delta
                    .transformed_pointer
                    .as_deref()
                    .expect("validated added pointer");
                baseline_stage.value.pointer(pointer).is_none()
                    && transformed_stage.value.pointer(pointer) == delta.transformed_value.as_ref()
            }
            RequiredDeltaKind::Changed => {
                baseline_stage.value.pointer(
                    delta
                        .baseline_pointer
                        .as_deref()
                        .expect("validated changed baseline pointer"),
                ) == delta.baseline_value.as_ref()
                    && transformed_stage.value.pointer(
                        delta
                            .transformed_pointer
                            .as_deref()
                            .expect("validated changed transformed pointer"),
                    ) == delta.transformed_value.as_ref()
            }
            RequiredDeltaKind::Removed => {
                let pointer = delta
                    .baseline_pointer
                    .as_deref()
                    .expect("validated removed pointer");
                baseline_stage.value.pointer(pointer) == delta.baseline_value.as_ref()
                    && transformed_stage.value.pointer(pointer).is_none()
            }
        };
        if !observed {
            failures.push(format!(
                "missing_expected_delta: semantic-negative delta was not observed: {}",
                delta.delta_id
            ));
        }
        evidence.push(RequiredDeltaEvidence {
            delta_id: delta.delta_id.clone(),
            stage: delta.stage,
            kind: delta.kind,
            baseline_pointer: delta.baseline_pointer.clone(),
            transformed_pointer: delta.transformed_pointer.clone(),
            observed,
        });
    }

    let mut observed_proof_deltas = 0usize;
    for proof in &recipe.dependent_proof_deltas {
        for stage in &proof.stages {
            let baseline_stage = baseline
                .stages
                .get(stage)
                .ok_or_else(|| invalid("semantic-negative proof baseline stage is missing"))?;
            let transformed_stage = transformed
                .stages
                .get(stage)
                .ok_or_else(|| invalid("semantic-negative proof transformed stage is missing"))?;
            let baseline_matches = matching_proof_claims(&baseline_stage.value, proof)?;
            let transformed_matches = matching_proof_claims(&transformed_stage.value, proof)?;
            if baseline_matches == 1 && transformed_matches == 0 {
                observed_proof_deltas += 1;
            } else {
                failures.push(format!(
                    "missing_expected_delta: dependent proof delta differs at {}: {}",
                    stage.as_str(),
                    proof.delta_id
                ));
            }
        }
    }
    Ok((evidence, observed_proof_deltas, failures))
}

fn strip_declared_negative_deltas(
    stage: BehavioralStage,
    baseline: &mut Value,
    transformed: &mut Value,
    recipe: &ReviewedTransformRecipe,
) -> Result<(Vec<String>, usize, usize)> {
    let mut paths = Vec::new();
    for delta in recipe
        .required_deltas
        .iter()
        .filter(|delta| delta.stage == stage)
    {
        match delta.kind {
            RequiredDeltaKind::Added => {
                let pointer = delta
                    .transformed_pointer
                    .as_deref()
                    .expect("validated added pointer");
                remove_json_pointer(transformed, pointer).ok_or_else(|| {
                    invalid(format!(
                        "semantic-negative declared added delta disappeared: {}",
                        delta.delta_id
                    ))
                })?;
                paths.push(format!("transformed:{pointer}"));
            }
            RequiredDeltaKind::Changed => {
                let baseline_pointer = delta
                    .baseline_pointer
                    .as_deref()
                    .expect("validated changed baseline pointer");
                let transformed_pointer = delta
                    .transformed_pointer
                    .as_deref()
                    .expect("validated changed transformed pointer");
                remove_json_pointer(baseline, baseline_pointer).ok_or_else(|| {
                    invalid(format!(
                        "semantic-negative declared baseline delta disappeared: {}",
                        delta.delta_id
                    ))
                })?;
                remove_json_pointer(transformed, transformed_pointer).ok_or_else(|| {
                    invalid(format!(
                        "semantic-negative declared transformed delta disappeared: {}",
                        delta.delta_id
                    ))
                })?;
                paths.push(format!("baseline:{baseline_pointer}"));
                paths.push(format!("transformed:{transformed_pointer}"));
            }
            RequiredDeltaKind::Removed => {
                let pointer = delta
                    .baseline_pointer
                    .as_deref()
                    .expect("validated removed pointer");
                remove_json_pointer(baseline, pointer).ok_or_else(|| {
                    invalid(format!(
                        "semantic-negative declared removed delta disappeared: {}",
                        delta.delta_id
                    ))
                })?;
                paths.push(format!("baseline:{pointer}"));
            }
        }
    }

    let mut baseline_proof_deltas = 0usize;
    let mut transformed_proof_deltas = 0usize;
    for proof in recipe
        .dependent_proof_deltas
        .iter()
        .filter(|proof| proof.stages.contains(&stage))
    {
        baseline_proof_deltas += remove_matching_proof_claims(baseline, proof)?;
        transformed_proof_deltas += remove_matching_proof_claims(transformed, proof)?;
    }
    Ok((paths, baseline_proof_deltas, transformed_proof_deltas))
}

fn matching_proof_claims(value: &Value, expected: &ReviewedProofDelta) -> Result<usize> {
    value
        .pointer("/proof_ledger/claims")
        .and_then(Value::as_array)
        .map(|claims| {
            claims
                .iter()
                .filter(|claim| proof_claim_matches(claim, expected))
                .count()
        })
        .ok_or_else(|| invalid("semantic-negative stage lacks proof claims"))
}

fn remove_matching_proof_claims(value: &mut Value, expected: &ReviewedProofDelta) -> Result<usize> {
    let claims = value
        .pointer_mut("/proof_ledger/claims")
        .and_then(Value::as_array_mut)
        .ok_or_else(|| invalid("semantic-negative stage lacks mutable proof claims"))?;
    let before = claims.len();
    claims.retain(|claim| !proof_claim_matches(claim, expected));
    Ok(before - claims.len())
}

fn proof_claim_matches(claim: &Value, expected: &ReviewedProofDelta) -> bool {
    claim.get("rule_id").and_then(Value::as_str) == Some(expected.rule_id.as_str())
        && claim.pointer("/address/stage").and_then(Value::as_str)
            == Some(expected.address.stage.as_str())
        && claim.pointer("/address/surface").and_then(Value::as_str)
            == Some(expected.address.surface.as_str())
        && claim
            .pointer("/address/stable_record_key")
            .and_then(Value::as_str)
            == Some(expected.address.stable_record_key.as_str())
        && claim.pointer("/address/field_path").and_then(Value::as_str)
            == expected.address.field_path.as_deref()
}

fn remove_json_pointer(value: &mut Value, pointer: &str) -> Option<Value> {
    let mut tokens = pointer
        .strip_prefix('/')?
        .split('/')
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .collect::<Vec<_>>();
    let final_token = tokens.pop()?;
    let mut parent = value;
    for token in tokens {
        parent = match parent {
            Value::Object(object) => object.get_mut(&token)?,
            Value::Array(values) => values.get_mut(token.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    match parent {
        Value::Object(object) => object.remove(&final_token),
        Value::Array(values) => {
            let index = final_token.parse::<usize>().ok()?;
            (index < values.len()).then(|| values.remove(index))
        }
        _ => None,
    }
}

fn set_json_pointer(value: &mut Value, pointer: &str, replacement: Value) -> bool {
    let Some(stripped) = pointer.strip_prefix('/') else {
        return false;
    };
    let mut tokens = stripped
        .split('/')
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .collect::<Vec<_>>();
    let Some(final_token) = tokens.pop() else {
        return false;
    };
    let mut parent = value;
    for token in tokens {
        parent = match parent {
            Value::Object(object) => match object.get_mut(&token) {
                Some(child) => child,
                None => return false,
            },
            Value::Array(values) => {
                let Some(index) = token.parse::<usize>().ok() else {
                    return false;
                };
                let Some(child) = values.get_mut(index) else {
                    return false;
                };
                child
            }
            _ => return false,
        };
    }
    match parent {
        Value::Object(object) => {
            object.insert(final_token, replacement);
            true
        }
        Value::Array(values) => {
            let Some(index) = final_token.parse::<usize>().ok() else {
                return false;
            };
            if let Some(slot) = values.get_mut(index) {
                *slot = replacement;
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn run_pipeline(
    source: &Path,
    run_root: &Path,
    prior_memory: &Path,
    repository: &Path,
) -> Result<PipelineArtifacts> {
    if run_root.exists() {
        return Err(invalid(format!(
            "behavioral run root already exists: {}",
            run_root.display()
        )));
    }
    let source_root = run_root.join("source_ir");
    let evidence_root = run_root.join("evidence_ir");
    let semantic_root = run_root.join("semantic_ir");
    let intent_root = run_root.join("intent_ir");
    let adapter_root = run_root.join("adapters");

    let mut source_ir = SourceIr::build(source, &source_root)?;
    source_ir.materialize()?;
    source_ir.write_to_disk()?;
    let source_path = source_ir.artifact_layout.source_ir_path.clone();
    SourceIr::load_from_path(&source_path)?;

    let evidence_ir =
        EvidenceIr::build_with_prior_memory(&source_path, &evidence_root, Some(prior_memory))?;
    evidence_ir.write_to_disk()?;
    let evidence_path = evidence_ir.artifact_layout.evidence_ir_path.clone();
    let evidence_ir = EvidenceIr::load_from_path(&evidence_path)?;

    let semantic_ir = SemanticIr::build(&evidence_path, &semantic_root)?;
    semantic_ir.write_to_disk()?;
    let semantic_path = semantic_ir.artifact_layout.semantic_ir_path.clone();
    SemanticIr::load_from_path(&semantic_path)?;

    let intent_ir = IntentIr::build(&semantic_path, &intent_root)?;
    intent_ir.write_to_disk()?;
    let intent_path = intent_ir.artifact_layout.intent_ir_path.clone();
    let intent_ir = IntentIr::load_from_path(&intent_path)?;

    let adapter = AdapterArtifact::build(&intent_path, AdapterTarget::Isf, &adapter_root)?;
    adapter.write_to_disk()?;
    let adapter_path = adapter.artifact_layout.adapter_artifact_path.clone();
    AdapterArtifact::load_from_path(&adapter_path)?;

    let stage_paths = [
        (BehavioralStage::SourceIr, source_path),
        (BehavioralStage::EvidenceIr, evidence_path),
        (BehavioralStage::SemanticIr, semantic_path),
        (BehavioralStage::IntentIr, intent_path),
        (BehavioralStage::IsfAdapter, adapter_path),
    ];
    let mut stages = BTreeMap::new();
    for (stage, path) in stage_paths {
        let bytes = fs::read(&path)?;
        let value = serde_json::from_slice::<Value>(&bytes)?;
        if value.get("stage").and_then(Value::as_str) != Some(stage.as_str()) {
            return Err(invalid(format!(
                "behavioral {} artifact names another stage",
                stage.as_str()
            )));
        }
        stages.insert(
            stage,
            StageArtifact {
                identity: ArtifactIdentity {
                    path: repository_relative(repository, &path)?,
                    sha256: sha256_bytes(&bytes),
                    byte_count: bytes.len() as u64,
                },
                value,
            },
        );
    }
    Ok(PipelineArtifacts {
        stages,
        document_key: intent_ir.document_identity.document_key.clone(),
        evidence_ir,
    })
}

fn compare_pipelines(
    baseline: &PipelineArtifacts,
    transformed: &PipelineArtifacts,
    normalization: &NormalizationSpec,
    semantic_negative: Option<&ReviewedTransformRecipe>,
) -> Result<(BehavioralRunState, Vec<StageComparison>, Vec<String>)> {
    compare_artifact_maps(
        &baseline.stages,
        &transformed.stages,
        &baseline.document_key,
        &transformed.document_key,
        normalization,
        semantic_negative,
    )
}

fn compare_artifact_maps(
    baseline: &BTreeMap<BehavioralStage, StageArtifact>,
    transformed: &BTreeMap<BehavioralStage, StageArtifact>,
    baseline_document_key: &str,
    transformed_document_key: &str,
    normalization: &NormalizationSpec,
    semantic_negative: Option<&ReviewedTransformRecipe>,
) -> Result<(BehavioralRunState, Vec<StageComparison>, Vec<String>)> {
    let expected = BehavioralStage::ALL.into_iter().collect::<BTreeSet<_>>();
    let baseline_stages = baseline.keys().copied().collect::<BTreeSet<_>>();
    let transformed_stages = transformed.keys().copied().collect::<BTreeSet<_>>();
    if !is_complete_stage_set(&baseline_stages, &expected)
        || !is_complete_stage_set(&transformed_stages, &expected)
    {
        return Ok((
            BehavioralRunState::Invalid,
            Vec::new(),
            vec!["partial_or_escaped_run: five-stage artifact set is incomplete".to_string()],
        ));
    }

    let mut comparisons = Vec::new();
    let mut failures = Vec::new();
    for stage in BehavioralStage::ALL {
        let left = baseline
            .get(&stage)
            .ok_or_else(|| invalid("baseline stage disappeared during comparison"))?;
        let right = transformed
            .get(&stage)
            .ok_or_else(|| invalid("transformed stage disappeared during comparison"))?;
        let comparison = compare_stage(stage, left, right, normalization, semantic_negative)?;
        if comparison.baseline_proof_claims == 0 || comparison.transformed_proof_claims == 0 {
            failures.push(format!(
                "partial_or_escaped_run: {} lacks proof claims",
                stage.as_str()
            ));
        }
        if !comparison.passed {
            failures.push(format!(
                "undeclared_semantic_delta: {} has {} unclassified paths",
                stage.as_str(),
                comparison.undeclared_delta_paths.len()
            ));
        }
        comparisons.push(comparison);
    }

    if normalization.relation == BehavioralRelation::AdversarialIdentity
        && baseline_document_key == transformed_document_key
    {
        failures.push("missing_expected_delta: document identity did not change".to_string());
    }
    let state = run_state(&failures);
    Ok((state, comparisons, failures))
}

fn run_state(failures: &[String]) -> BehavioralRunState {
    if failures
        .iter()
        .any(|failure| failure.starts_with("partial_or_escaped_run"))
    {
        BehavioralRunState::Invalid
    } else if failures.is_empty() {
        BehavioralRunState::Pass
    } else {
        BehavioralRunState::Fail
    }
}

fn is_complete_stage_set(
    observed: &BTreeSet<BehavioralStage>,
    expected: &BTreeSet<BehavioralStage>,
) -> bool {
    observed == expected
}

fn compare_stage(
    stage: BehavioralStage,
    baseline: &StageArtifact,
    transformed: &StageArtifact,
    normalization: &NormalizationSpec,
    semantic_negative: Option<&ReviewedTransformRecipe>,
) -> Result<StageComparison> {
    let baseline_top_level_fields = object_len(&baseline.value)?;
    let transformed_top_level_fields = object_len(&transformed.value)?;
    let baseline_proof_claims = proof_claim_count(&baseline.value)?;
    let transformed_proof_claims = proof_claim_count(&transformed.value)?;
    let normalized_delta_paths = difference_paths(&baseline.value, &transformed.value);

    let mut declared_left = baseline.value.clone();
    let mut declared_right = transformed.value.clone();
    let (declared_delta_paths, baseline_declared_proof_deltas, transformed_declared_proof_deltas) =
        if let Some(recipe) = semantic_negative {
            strip_declared_negative_deltas(stage, &mut declared_left, &mut declared_right, recipe)?
        } else {
            (Vec::new(), 0, 0)
        };
    let adjusted_baseline_top_level_fields = object_len(&declared_left)?;
    let adjusted_transformed_top_level_fields = object_len(&declared_right)?;
    let mut left = basic_normalize(&declared_left, normalization, false);
    let mut right = basic_normalize(&declared_right, normalization, true);
    normalize_isf_interface_order(&mut left);
    normalize_isf_interface_order(&mut right);
    normalize_unordered_symbol_groups(&mut left);
    normalize_unordered_symbol_groups(&mut right);
    normalize_changed_hashes(&mut left, &mut right, None);
    normalize_relation_bound_scalars(&mut left, &mut right, normalization.relation, None);
    for _ in 0..8 {
        canonicalize_keyed_collections(&mut left);
        canonicalize_keyed_collections(&mut right);
        let mut candidate_ids = Vec::new();
        collect_equivalent_id_pairs(&left, &right, "", &mut candidate_ids)?;
        let derived_ids = bijective_id_mapping(&candidate_ids);
        if derived_ids.is_empty() {
            break;
        }
        apply_exact_mapping(&mut right, &derived_ids);
    }
    canonicalize_keyed_collections(&mut right);
    let undeclared_delta_paths = difference_paths(&left, &right);
    let compared_leaf_values = leaf_count(&left).max(leaf_count(&right));
    Ok(StageComparison {
        stage,
        baseline: baseline.identity.clone(),
        transformed: transformed.identity.clone(),
        baseline_top_level_fields,
        transformed_top_level_fields,
        baseline_proof_claims,
        transformed_proof_claims,
        compared_leaf_values,
        normalized_delta_paths,
        declared_delta_paths,
        baseline_declared_proof_deltas,
        transformed_declared_proof_deltas,
        passed: undeclared_delta_paths.is_empty()
            && adjusted_baseline_top_level_fields == adjusted_transformed_top_level_fields
            && baseline_proof_claims.saturating_sub(baseline_declared_proof_deltas)
                == transformed_proof_claims.saturating_sub(transformed_declared_proof_deltas),
        undeclared_delta_paths,
    })
}

fn derive_source_symbol_catalog(evidence_ir: &EvidenceIr, source: &str) -> Vec<String> {
    let mut candidates = declared_signal_catalog(evidence_ir);
    let mut seen = BTreeSet::new();
    candidates.retain(|candidate| {
        is_identifier(candidate)
            && contains_identifier(source, candidate)
            && seen.insert(candidate.to_ascii_lowercase())
    });
    candidates
}

fn alpha_transform(
    source: &str,
    symbols: &[String],
    seed: u64,
) -> Result<(String, Vec<SymbolRename>)> {
    let source_tokens = identifier_tokens(source)
        .into_iter()
        .map(|token| token.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let mut used = source_tokens.clone();
    let mut mapping = BTreeMap::new();
    let mut renames = Vec::new();
    let mut lexical_indices = (0..symbols.len()).collect::<Vec<_>>();
    lexical_indices.sort_by_key(|index| symbols[*index].to_ascii_lowercase());
    let mut reverse_lexical_rank = vec![0usize; symbols.len()];
    for (rank, index) in lexical_indices.into_iter().enumerate() {
        reverse_lexical_rank[index] = symbols.len() - rank;
    }
    for (ordinal, original) in symbols.iter().enumerate() {
        let familiar_name = FAMILIAR_IDENTIFIERS
            .get(ordinal)
            .copied()
            .unwrap_or("generic");
        let familiar = ordinal < FAMILIAR_IDENTIFIERS.len();
        let base_replacement = format!(
            "signal_alias_{:06}_{familiar_name}_{seed:016x}",
            reverse_lexical_rank[ordinal]
        );
        let mut replacement = base_replacement.clone();
        let mut collision = 0usize;
        while used.contains(&replacement.to_ascii_lowercase()) {
            collision += 1;
            replacement = format!("{base_replacement}_{collision}");
        }
        used.insert(replacement.to_ascii_lowercase());
        if mapping
            .insert(original.to_ascii_lowercase(), replacement.clone())
            .is_some()
        {
            return Err(invalid(format!(
                "ambiguous_or_nonbijective_transform: duplicate folded symbol {original}"
            )));
        }
        renames.push(SymbolRename {
            source_ordinal: ordinal,
            original: original.clone(),
            replacement,
            occurrence_count: 0,
            familiar_adversarial_spelling: familiar,
        });
    }

    let (transformed, counts) = replace_identifier_tokens(source, &mapping);
    for rename in &mut renames {
        rename.occurrence_count = *counts
            .get(&rename.original.to_ascii_lowercase())
            .unwrap_or(&0);
        if rename.occurrence_count == 0 || contains_identifier(&transformed, &rename.original) {
            return Err(invalid(format!(
                "ambiguous_or_nonbijective_transform: incomplete occurrence coverage for {}",
                rename.original
            )));
        }
    }
    let unique_replacements = renames
        .iter()
        .map(|rename| rename.replacement.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    if unique_replacements.len() != renames.len() {
        return Err(invalid(
            "ambiguous_or_nonbijective_transform: replacement map is not bijective",
        ));
    }
    if renames.len() >= 2 && !lexical_order_changed(&renames) {
        return Err(invalid(
            "ambiguous_or_nonbijective_transform: lexical ordering did not vary",
        ));
    }
    Ok((transformed, renames))
}

fn replace_identifier_tokens(
    source: &str,
    mapping: &BTreeMap<String, String>,
) -> (String, BTreeMap<String, usize>) {
    let mut output = String::with_capacity(source.len());
    let mut counts = BTreeMap::new();
    let mut cursor = 0usize;
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if is_identifier_start(character) {
            let start = cursor;
            cursor += character.len_utf8();
            while cursor < source.len() {
                let next = source[cursor..].chars().next().expect("valid UTF-8 cursor");
                if !is_identifier_continue(next) {
                    break;
                }
                cursor += next.len_utf8();
            }
            let token = &source[start..cursor];
            let folded = token.to_ascii_lowercase();
            if let Some(replacement) = mapping.get(&folded) {
                output.push_str(replacement);
                *counts.entry(folded).or_default() += 1;
            } else {
                output.push_str(token);
            }
        } else {
            output.push(character);
            cursor += character.len_utf8();
        }
    }
    (output, counts)
}

fn source_bound_identifier_projection(source: &str) -> String {
    let mut output = String::with_capacity(source.len());
    let mut separator_pending = false;
    for character in source.chars() {
        if character.is_ascii_alphanumeric() {
            if separator_pending && !output.is_empty() {
                output.push('_');
            }
            output.push(character.to_ascii_lowercase());
            separator_pending = false;
        } else if !output.is_empty() {
            separator_pending = true;
        }
    }
    output
}

fn basic_normalize(value: &Value, spec: &NormalizationSpec, transformed: bool) -> Value {
    basic_normalize_at(value, spec, transformed, None)
}

fn basic_normalize_at(
    value: &Value,
    spec: &NormalizationSpec,
    transformed: bool,
    field: Option<&str>,
) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, child)| {
                    (
                        key.clone(),
                        basic_normalize_at(child, spec, transformed, Some(key)),
                    )
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(
            values
                .iter()
                .map(|child| basic_normalize_at(child, spec, transformed, field))
                .collect(),
        ),
        Value::String(text) => {
            let mut normalized = text.clone();
            let (run_root, source) = if transformed {
                (&spec.transformed_run_root, &spec.transformed_source)
            } else {
                (&spec.baseline_run_root, &spec.baseline_source)
            };
            normalized = normalized.replace(source, "$SOURCE");
            normalized = normalized.replace(run_root, "$RUN_ROOT");
            if transformed {
                let mut exact = spec
                    .exact_transformed_to_baseline
                    .iter()
                    .collect::<Vec<_>>();
                exact.sort_by_key(|(from, _)| std::cmp::Reverse(from.len()));
                for (from, to) in exact {
                    normalized = normalized.replace(from, to);
                }
                let mut identifiers = spec
                    .identifier_transformed_to_baseline
                    .iter()
                    .collect::<Vec<_>>();
                identifiers.sort_by_key(|(from, _)| std::cmp::Reverse(from.len()));
                for (from, to) in identifiers {
                    normalized = normalized.replace(from, to);
                }
                normalized = replace_identifier_tokens(
                    &normalized,
                    &spec.identifier_transformed_to_baseline,
                )
                .0;
                if let Some(reviewed) =
                    field.and_then(|field| spec.reviewed_text_by_field.get(field))
                {
                    let mut reviewed = reviewed.iter().collect::<Vec<_>>();
                    reviewed.sort_by_key(|(from, _)| std::cmp::Reverse(from.len()));
                    for (from, to) in reviewed {
                        normalized = normalized.replace(from, to);
                    }
                }
            }
            Value::String(normalized)
        }
        _ => value.clone(),
    }
}

fn canonicalize_keyed_collections(value: &mut Value) {
    canonicalize_collection(value, None);
}

fn canonicalize_collection(value: &mut Value, field: Option<&str>) {
    match value {
        Value::Object(object) => {
            for (child_field, child) in object {
                canonicalize_collection(child, Some(child_field));
            }
        }
        Value::Array(values) => {
            for child in values.iter_mut() {
                canonicalize_collection(child, None);
            }
            if values.len() > 1 && values.iter().all(stable_collection_key_is_present) {
                values.sort_by_cached_key(stable_collection_key);
            } else if field.is_some_and(is_unordered_reference_collection)
                && values.iter().all(Value::is_string)
            {
                values.sort_by(|left, right| left.as_str().cmp(&right.as_str()));
            }
        }
        _ => {}
    }
}

fn stable_collection_key_is_present(value: &Value) -> bool {
    stable_collection_key(value).is_some()
}

fn stable_collection_key(value: &Value) -> Option<String> {
    let object = value.as_object()?;
    object
        .iter()
        .any(|(field, child)| {
            (is_relation_derived_id(field) || COLLECTION_KEY_FIELDS.contains(&field.as_str()))
                && child.is_string()
        })
        .then(|| serde_json::to_string(&erase_relation_ids(value)).expect("JSON serialization"))
}

fn is_unordered_reference_collection(field: &str) -> bool {
    field == "signals" || field == "referenced_signal_names" || field.ends_with("_ids")
}

fn normalize_isf_interface_order(value: &mut Value) {
    let Some(source_text) = value.pointer_mut("/isf/source_text") else {
        return;
    };
    let Some(source_text) = source_text.as_str() else {
        return;
    };
    let mut lines = source_text.lines().map(str::to_string).collect::<Vec<_>>();
    let mut cursor = 0usize;
    while cursor < lines.len() {
        if lines[cursor].trim() != "(interface" {
            cursor += 1;
            continue;
        }
        let start = cursor + 1;
        let Some(end) = lines[start..]
            .iter()
            .position(|line| line.trim() == ")")
            .map(|offset| start + offset)
        else {
            return;
        };
        lines[start..end].sort();
        cursor = end + 1;
    }
    let trailing_newline = source_text.ends_with('\n');
    let mut normalized = lines.join("\n");
    if trailing_newline {
        normalized.push('\n');
    }
    *value.pointer_mut("/isf/source_text").expect("path exists") = Value::String(normalized);
}

fn normalize_unordered_symbol_groups(value: &mut Value) {
    match value {
        Value::Object(object) => {
            for child in object.values_mut() {
                normalize_unordered_symbol_groups(child);
            }
        }
        Value::Array(values) => {
            for child in values {
                normalize_unordered_symbol_groups(child);
            }
        }
        Value::String(text) => {
            let Some(symbols) = text.strip_prefix(GROUPED_INTERFACE_PREFIX) else {
                return;
            };
            let mut symbols = symbols.split(", ").collect::<Vec<_>>();
            if symbols.len() < 2 || symbols.iter().any(|symbol| !is_identifier(symbol)) {
                return;
            }
            symbols.sort_unstable_by_key(|symbol| symbol.to_ascii_lowercase());
            *text = format!("{GROUPED_INTERFACE_PREFIX}{}", symbols.join(", "));
        }
        _ => {}
    }
}

fn erase_relation_ids(value: &Value) -> Value {
    match value {
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(field, child)| {
                    let child = if is_relation_derived_id(field) && child.is_string() {
                        Value::String("$DERIVED_ID".to_string())
                    } else if is_relation_derived_id_list(field)
                        && child
                            .as_array()
                            .is_some_and(|values| values.iter().all(Value::is_string))
                    {
                        Value::Array(
                            child
                                .as_array()
                                .expect("checked identifier list")
                                .iter()
                                .map(|_| Value::String("$DERIVED_ID".to_string()))
                                .collect(),
                        )
                    } else {
                        erase_relation_ids(child)
                    };
                    (field.clone(), child)
                })
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(erase_relation_ids).collect()),
        _ => value.clone(),
    }
}

fn is_relation_derived_id(field: &str) -> bool {
    field.ends_with("_id") && !DERIVED_ID_EXCLUSIONS.contains(&field)
}

fn is_relation_derived_id_list(field: &str) -> bool {
    field.ends_with("_ids")
}

fn collect_equivalent_id_pairs(
    left: &Value,
    right: &Value,
    path: &str,
    candidates: &mut Vec<(String, String)>,
) -> Result<()> {
    if path.starts_with("/proof_context") || path.starts_with("/proof_ledger") {
        return Ok(());
    }
    match (left, right) {
        (Value::Object(left), Value::Object(right)) => {
            let equivalent_without_ids = erase_relation_ids(&Value::Object(left.clone()))
                == erase_relation_ids(&Value::Object(right.clone()));
            if equivalent_without_ids {
                for (field, left_child) in left {
                    let Some(right_child) = right.get(field) else {
                        continue;
                    };
                    if is_relation_derived_id(field)
                        && let (Some(left_id), Some(right_id)) =
                            (left_child.as_str(), right_child.as_str())
                        && left_id != right_id
                    {
                        candidates.push((right_id.to_string(), left_id.to_string()));
                    }
                }
            }
            for (field, left_child) in left {
                if let Some(right_child) = right.get(field) {
                    collect_equivalent_id_pairs(
                        left_child,
                        right_child,
                        &format!("{path}/{}", escape_pointer(field)),
                        candidates,
                    )?;
                }
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            for (index, (left_child, right_child)) in left.iter().zip(right).enumerate() {
                collect_equivalent_id_pairs(
                    left_child,
                    right_child,
                    &format!("{path}/{index}"),
                    candidates,
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn bijective_id_mapping(candidates: &[(String, String)]) -> BTreeMap<String, String> {
    let mut transformed_to_baseline = BTreeMap::<&str, BTreeSet<&str>>::new();
    let mut baseline_to_transformed = BTreeMap::<&str, BTreeSet<&str>>::new();
    for (transformed, baseline) in candidates {
        transformed_to_baseline
            .entry(transformed)
            .or_default()
            .insert(baseline);
        baseline_to_transformed
            .entry(baseline)
            .or_default()
            .insert(transformed);
    }
    transformed_to_baseline
        .into_iter()
        .filter_map(|(transformed, baselines)| {
            let baseline = baselines.iter().next().copied()?;
            (baselines.len() == 1 && baseline_to_transformed[baseline].len() == 1)
                .then(|| (transformed.to_string(), baseline.to_string()))
        })
        .collect()
}

fn apply_exact_mapping(value: &mut Value, mapping: &BTreeMap<String, String>) {
    match value {
        Value::Object(object) => {
            for child in object.values_mut() {
                apply_exact_mapping(child, mapping);
            }
        }
        Value::Array(values) => {
            for child in values {
                apply_exact_mapping(child, mapping);
            }
        }
        Value::String(text) => {
            let mut pairs = mapping.iter().collect::<Vec<_>>();
            pairs.sort_by_key(|(from, _)| std::cmp::Reverse(from.len()));
            for (from, to) in pairs {
                *text = text.replace(from, to);
            }
        }
        _ => {}
    }
}

fn normalize_changed_hashes(left: &mut Value, right: &mut Value, field: Option<&str>) {
    if field.is_some_and(|field| {
        field.ends_with("sha256") && !IDENTITY_HASH_EXCLUSIONS.contains(&field)
    }) && left.is_string()
        && right.is_string()
        && left != right
    {
        *left = Value::String("$RELATION_DIGEST".to_string());
        *right = Value::String("$RELATION_DIGEST".to_string());
        return;
    }
    match (left, right) {
        (Value::Object(left), Value::Object(right)) => {
            for (key, left_child) in left {
                if let Some(right_child) = right.get_mut(key) {
                    normalize_changed_hashes(left_child, right_child, Some(key));
                }
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            for (left_child, right_child) in left.iter_mut().zip(right) {
                normalize_changed_hashes(left_child, right_child, field);
            }
        }
        _ => {}
    }
}

fn normalize_relation_bound_scalars(
    left: &mut Value,
    right: &mut Value,
    relation: BehavioralRelation,
    field: Option<&str>,
) {
    if relation == BehavioralRelation::AdversarialIdentity
        && field == Some("stable_artifact_stem")
        && left.is_string()
        && right.is_string()
        && left != right
    {
        *left = Value::String("$DOCUMENT_STEM".to_string());
        *right = Value::String("$DOCUMENT_STEM".to_string());
        return;
    }
    if field == Some("scope")
        && left.as_str().is_some_and(|value| is_lower_hex(value, 64))
        && right.as_str().is_some_and(|value| is_lower_hex(value, 64))
        && left != right
    {
        *left = Value::String("$DOCUMENT_SCOPE".to_string());
        *right = Value::String("$DOCUMENT_SCOPE".to_string());
        return;
    }
    if matches!(
        relation,
        BehavioralRelation::SymbolAlpha
            | BehavioralRelation::StructurePreservingParaphrase
            | BehavioralRelation::HarmlessLayout
            | BehavioralRelation::SemanticNegative
    ) && field == Some("size_bytes")
        && left.is_number()
        && right.is_number()
        && left != right
    {
        *left = Value::String("$SOURCE_CONTENT_SIZE".to_string());
        *right = Value::String("$SOURCE_CONTENT_SIZE".to_string());
        return;
    }
    match (left, right) {
        (Value::Object(left), Value::Object(right)) => {
            for (key, left_child) in left {
                if let Some(right_child) = right.get_mut(key) {
                    normalize_relation_bound_scalars(left_child, right_child, relation, Some(key));
                }
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            for (left_child, right_child) in left.iter_mut().zip(right) {
                normalize_relation_bound_scalars(left_child, right_child, relation, field);
            }
        }
        _ => {}
    }
}

fn difference_paths(left: &Value, right: &Value) -> Vec<String> {
    let mut differences = Vec::new();
    collect_difference_paths(left, right, "", &mut differences);
    differences
}

fn collect_difference_paths(left: &Value, right: &Value, path: &str, output: &mut Vec<String>) {
    match (left, right) {
        (Value::Object(left), Value::Object(right)) => {
            let keys = left.keys().chain(right.keys()).collect::<BTreeSet<_>>();
            for key in keys {
                let child_path = format!("{path}/{}", escape_pointer(key));
                match (left.get(key), right.get(key)) {
                    (Some(left), Some(right)) => {
                        collect_difference_paths(left, right, &child_path, output)
                    }
                    _ => output.push(child_path),
                }
            }
        }
        (Value::Array(left), Value::Array(right)) => {
            let length = left.len().max(right.len());
            for index in 0..length {
                let child_path = format!("{path}/{index}");
                match (left.get(index), right.get(index)) {
                    (Some(left), Some(right)) => {
                        collect_difference_paths(left, right, &child_path, output)
                    }
                    _ => output.push(child_path),
                }
            }
        }
        _ if left != right => output.push(if path.is_empty() {
            "/".to_string()
        } else {
            path.to_string()
        }),
        _ => {}
    }
}

fn proof_claim_count(value: &Value) -> Result<usize> {
    value
        .pointer("/proof_ledger/claims")
        .and_then(Value::as_array)
        .map(Vec::len)
        .ok_or_else(|| invalid("behavioral stage lacks a proof ledger claim array"))
}

fn object_len(value: &Value) -> Result<usize> {
    value
        .as_object()
        .map(serde_json::Map::len)
        .ok_or_else(|| invalid("behavioral stage artifact must be a JSON object"))
}

fn leaf_count(value: &Value) -> usize {
    match value {
        Value::Object(object) => object.values().map(leaf_count).sum(),
        Value::Array(values) => values.iter().map(leaf_count).sum(),
        _ => 1,
    }
}

fn lexical_order_changed(renames: &[SymbolRename]) -> bool {
    if renames.len() < 2 {
        return false;
    }
    let mut original = renames.iter().collect::<Vec<_>>();
    original.sort_by_key(|rename| rename.original.to_ascii_lowercase());
    let mut transformed = renames.iter().collect::<Vec<_>>();
    transformed.sort_by_key(|rename| rename.replacement.to_ascii_lowercase());
    original
        .iter()
        .map(|rename| rename.source_ordinal)
        .ne(transformed.iter().map(|rename| rename.source_ordinal))
}

fn identifier_tokens(source: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut cursor = 0usize;
    while cursor < source.len() {
        let Some(character) = source[cursor..].chars().next() else {
            break;
        };
        if is_identifier_start(character) {
            let start = cursor;
            cursor += character.len_utf8();
            while cursor < source.len() {
                let next = source[cursor..].chars().next().expect("valid UTF-8 cursor");
                if !is_identifier_continue(next) {
                    break;
                }
                cursor += next.len_utf8();
            }
            tokens.push(source[start..cursor].to_string());
        } else {
            cursor += character.len_utf8();
        }
    }
    tokens
}

fn contains_identifier(source: &str, expected: &str) -> bool {
    identifier_tokens(source)
        .iter()
        .any(|token| token.eq_ignore_ascii_case(expected))
}

fn is_identifier(value: &str) -> bool {
    let mut characters = value.chars();
    characters.next().is_some_and(is_identifier_start) && characters.all(is_identifier_continue)
}

fn is_identifier_start(character: char) -> bool {
    character.is_ascii_alphabetic() || character == '_'
}

fn is_identifier_continue(character: char) -> bool {
    character.is_ascii_alphanumeric() || character == '_'
}

fn validate_relative_path(path: &Path, label: &str) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(invalid(format!(
            "{label} must be repository-relative: {}",
            path.display()
        )));
    }
    Ok(())
}

fn resolve_source_authority(repository: &Path, source: &Path) -> Result<PathBuf> {
    let candidate = if source.is_absolute() {
        source.to_path_buf()
    } else {
        repository.join(source)
    };
    let canonical = candidate.canonicalize().map_err(|error| {
        invalid(format!(
            "authority_unavailable: behavioral source authority is unavailable: {} ({error})",
            source.display()
        ))
    })?;
    if !canonical.is_file() {
        return Err(invalid(format!(
            "authority_unavailable: behavioral source authority is not a file: {}",
            source.display()
        )));
    }
    Ok(canonical)
}

fn prepare_output_root(repository: &Path, relative: &Path) -> Result<PathBuf> {
    let mut cursor = repository.to_path_buf();
    for component in relative.components() {
        cursor.push(component.as_os_str());
        match fs::symlink_metadata(&cursor) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(invalid(format!(
                    "partial_or_escaped_run: behavioral output traverses a symlink: {}",
                    cursor.display()
                )));
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    fs::create_dir_all(repository.join(TEMP_ROOT))?;
    fs::create_dir_all(repository.join(relative).join("inputs"))?;
    let canonical_repository = repository.canonicalize()?;
    let canonical_temp = repository.join(TEMP_ROOT).canonicalize()?;
    let canonical_output = repository.join(relative).canonicalize()?;
    if !canonical_temp.starts_with(&canonical_repository)
        || !canonical_output.starts_with(&canonical_temp)
    {
        return Err(invalid(format!(
            "partial_or_escaped_run: behavioral output escaped repository scratch: {}",
            relative.display()
        )));
    }
    ensure_same_filesystem(&canonical_repository, &canonical_output)?;
    Ok(canonical_output)
}

fn resolve_repository_file(repository: &Path, path: &Path, label: &str) -> Result<PathBuf> {
    validate_relative_path(path, label)?;
    let canonical = repository.join(path).canonicalize()?;
    if !canonical.starts_with(repository) || !canonical.is_file() {
        return Err(invalid(format!(
            "{label} must resolve to a repository file: {}",
            path.display()
        )));
    }
    Ok(canonical)
}

fn repository_relative(repository: &Path, path: &Path) -> Result<String> {
    let absolute = path.canonicalize()?;
    absolute
        .strip_prefix(repository)
        .map(|relative| relative.to_string_lossy().into_owned())
        .map_err(|_| {
            invalid(format!(
                "behavioral path escaped repository: {}",
                path.display()
            ))
        })
}

fn copy_exact(source: &Path, target: &Path) -> Result<()> {
    if target.exists() {
        return Err(invalid(format!(
            "behavioral input target already exists: {}",
            target.display()
        )));
    }
    if let Some(parent) = target.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::copy(source, target)?;
    if sha256_file(source)? != sha256_file(target)? {
        return Err(invalid("behavioral source copy failed digest verification"));
    }
    Ok(())
}

fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    if path.exists() {
        return Err(invalid(format!(
            "behavioral transform target already exists: {}",
            path.display()
        )));
    }
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes)?;
    Ok(())
}

fn sha256_file(path: &Path) -> Result<String> {
    Ok(sha256_bytes(&fs::read(path)?))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn escape_pointer(value: &str) -> String {
    value.replace('~', "~0").replace('/', "~1")
}

fn invalid(message: impl Into<String>) -> AppError {
    AppError::InvalidStageArtifact(message.into())
}

#[cfg(unix)]
fn ensure_same_filesystem(repository: &Path, source: &Path) -> Result<()> {
    use std::os::unix::fs::MetadataExt;

    if fs::metadata(repository)?.dev() != fs::metadata(source)?.dev() {
        return Err(invalid(format!(
            "behavioral source is not on the repository filesystem: {}",
            source.display()
        )));
    }
    Ok(())
}

#[cfg(not(unix))]
fn ensure_same_filesystem(_repository: &Path, _source: &Path) -> Result<()> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const PRIOR_MEMORY: &str = "generated/prior_memory/corpus_memory.json";
    const REVIEWED_SOURCE: &str = "generated/source_ir/um11732_v3_2022_02_17_i2s_bus_specification/normalized/um11732_v3_2022_02_17_i2s_bus_specification.md";
    const REVIEWED_SOURCE_SHA256: &str =
        "250339a784e657ac2c87a9762dfddbffc6a4bd42ed27da7e666c2f2db419753f";
    const PARAPHRASE_RECIPE_ID: &str = "um11732-v3-equivalent-minimum-timing-phrase-v1";
    const LAYOUT_RECIPE_ID: &str = "um11732-v3-heading-table-whitespace-formatting-v1";
    const NEGATIVE_RECIPE_ID: &str = "um11732-v3-at-least-timing-grammar-negative-v1";

    fn synthetic_stage(value: Value, name: &str) -> StageArtifact {
        StageArtifact {
            identity: ArtifactIdentity {
                path: format!(".project-data/tmp/test/{name}.json"),
                sha256: "0".repeat(64),
                byte_count: 1,
            },
            value,
        }
    }

    fn synthetic_value(document_key: &str, role: &str) -> Value {
        let stable_artifact_stem = match document_key {
            "original" => "Original.Interface.Specification",
            _ => "clock_reset_handshake_reference",
        };
        serde_json::json!({
            "stage": "semantic_ir",
            "document_identity": {"document_key": document_key, "display_name": format!("{document_key}.pdf")},
            "artifact_layout": {"artifact_root": format!(".project-data/tmp/run/{document_key}")},
            "source": {"stable_artifact_stem": stable_artifact_stem},
            "proof_context": {
                "field_premises": {
                    "source": {"stable_artifact_stem": stable_artifact_stem}
                }
            },
            "canonical_fact": {"record_id": format!("record_{document_key}"), "role": role},
            "proof_ledger": {
                "schema_version": 1,
                "ruleset_sha256": "1".repeat(64),
                "claims": [{
                    "address": {"stage": "semantic_ir", "surface": "canonical_fact", "stable_record_key": format!("record_{document_key}")},
                    "conclusion_sha256": "2".repeat(64),
                    "rule_id": "semantic.test",
                    "premises": [],
                    "symbol_uses": [],
                    "confidence": "deterministic"
                }]
            }
        })
    }

    fn synthetic_negative_recipe() -> ReviewedTransformRecipe {
        ReviewedTransformRecipe {
            schema_version: 1,
            recipe_id: "synthetic-negative".to_string(),
            relation: BehavioralRelation::SemanticNegative,
            source_authority: "synthetic.md".to_string(),
            source_sha256: "0".repeat(64),
            review_status: "approved".to_string(),
            changed_spans: Vec::new(),
            preserved_conclusions: Vec::new(),
            semantic_negative_kind: Some(SemanticNegativeKind::TimingGrammarAdmission),
            required_deltas: vec![ReviewedRequiredDelta {
                delta_id: "declared-role-change".to_string(),
                stage: BehavioralStage::SemanticIr,
                kind: RequiredDeltaKind::Changed,
                baseline_pointer: Some("/canonical_fact/role".to_string()),
                transformed_pointer: Some("/canonical_fact/role".to_string()),
                baseline_value: Some(Value::String("producer".to_string())),
                transformed_value: Some(Value::String("consumer".to_string())),
            }],
            dependent_proof_deltas: Vec::new(),
            unaffected_complement: REVIEWED_COMPLEMENT.to_string(),
            unmeasurable_source_surfaces: RICH_CAPTURE_EXCLUSIONS
                .iter()
                .map(|surface| (*surface).to_string())
                .collect(),
        }
    }

    fn minimal_pdf_bytes() -> Vec<u8> {
        let content = b"BT /F1 12 Tf 72 720 Td (Neutral interface contract) Tj ET";
        let objects = [
            b"<< /Type /Catalog /Pages 2 0 R >>".to_vec(),
            b"<< /Type /Pages /Kids [3 0 R] /Count 1 >>".to_vec(),
            b"<< /Type /Page /Parent 2 0 R /MediaBox [0 0 612 792] /Resources << /Font << /F1 4 0 R >> >> /Contents 5 0 R >>".to_vec(),
            b"<< /Type /Font /Subtype /Type1 /BaseFont /Helvetica >>".to_vec(),
            [
                format!("<< /Length {} >>\nstream\n", content.len()).into_bytes(),
                content.to_vec(),
                b"\nendstream".to_vec(),
            ]
            .concat(),
            b"<< /Title (Neutral Interface Contract) >>".to_vec(),
        ];
        let mut pdf = b"%PDF-1.4\n".to_vec();
        let mut offsets = Vec::new();
        for (index, object) in objects.iter().enumerate() {
            offsets.push(pdf.len());
            pdf.extend_from_slice(format!("{} 0 obj\n", index + 1).as_bytes());
            pdf.extend_from_slice(object);
            pdf.extend_from_slice(b"\nendobj\n");
        }
        let xref = pdf.len();
        pdf.extend_from_slice(format!("xref\n0 {}\n", objects.len() + 1).as_bytes());
        pdf.extend_from_slice(b"0000000000 65535 f \n");
        for offset in offsets {
            pdf.extend_from_slice(format!("{offset:010} 00000 n \n").as_bytes());
        }
        pdf.extend_from_slice(
            format!(
                "trailer\n<< /Size {} /Root 1 0 R /Info 6 0 R >>\nstartxref\n{xref}\n%%EOF\n",
                objects.len() + 1
            )
            .as_bytes(),
        );
        pdf
    }

    fn reviewed_request(
        repository: &Path,
        temporary: &Path,
        relation: BehavioralRelation,
        recipe_id: &str,
        output_name: &str,
    ) -> Result<BehavioralQualificationRequest> {
        Ok(BehavioralQualificationRequest {
            relation,
            source_authority: PathBuf::from(REVIEWED_SOURCE),
            expected_source_sha256: REVIEWED_SOURCE_SHA256.to_string(),
            output_root: temporary
                .join(output_name)
                .strip_prefix(repository)
                .map_err(|_| invalid("reviewed test output escaped repository"))?
                .to_path_buf(),
            prior_memory: PathBuf::from(PRIOR_MEMORY),
            production_revision: "0".repeat(40),
            transform_seed: 0,
            review_recipe_id: Some(recipe_id.to_string()),
        })
    }

    #[test]
    fn alpha_transform_is_deterministic_bijective_and_complete() -> Result<()> {
        let source = "Signal ORBIT is input. ORBIT drives CLEAR. Signal CLEAR is output.";
        let symbols = vec!["ORBIT".to_string(), "CLEAR".to_string()];
        let (first, first_recipe) = alpha_transform(source, &symbols, 7)?;
        let (second, second_recipe) = alpha_transform(source, &symbols, 7)?;
        assert_eq!(first, second);
        assert_eq!(first_recipe, second_recipe);
        assert_eq!(first_recipe.len(), 2);
        assert!(
            first_recipe
                .iter()
                .all(|rename| rename.occurrence_count > 0)
        );
        assert!(!contains_identifier(&first, "ORBIT"));
        assert!(!contains_identifier(&first, "CLEAR"));
        Ok(())
    }

    #[test]
    fn alpha_transform_rejects_case_folded_ambiguity() {
        let source = "Signal ALPHA is input. Signal alpha is output.";
        let symbols = vec!["ALPHA".to_string(), "alpha".to_string()];
        assert!(alpha_transform(source, &symbols, 9).is_err());
    }

    #[test]
    fn retained_authority_rejects_non_scratch_links_and_missing_alpha_report_runs_fresh()
    -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let escaped = serde_json::json!({
            "retained_evidence_path": "doctrine/production_genericity/behavioral_holdout_evidence.json",
            "retained_evidence_sha256": "0".repeat(64)
        });
        assert!(
            ultimate_retained_artifact_output_root(
                &repository,
                Path::new(".project-data/tmp/synthetic-retained-root"),
                &escaped,
            )
            .is_err()
        );
        assert_eq!(
            retained_alpha_symbol_catalog(
                &repository,
                Path::new(".project-data/tmp/nonexistent-retained-alpha-attempt"),
                Path::new(REVIEWED_SOURCE),
                REVIEWED_SOURCE_SHA256,
                "synthetic_document",
            )?,
            None
        );
        Ok(())
    }

    #[test]
    fn comparator_rejects_identity_coupled_semantic_delta() -> Result<()> {
        let baseline = synthetic_stage(synthetic_value("original", "producer"), "left");
        let transformed = synthetic_stage(synthetic_value("misleading", "consumer"), "right");
        let normalization = NormalizationSpec {
            relation: BehavioralRelation::AdversarialIdentity,
            baseline_run_root: ".project-data/tmp/run".to_string(),
            transformed_run_root: ".project-data/tmp/run".to_string(),
            baseline_source: "original.pdf".to_string(),
            transformed_source: "misleading.pdf".to_string(),
            exact_transformed_to_baseline: BTreeMap::from([
                ("misleading".to_string(), "original".to_string()),
                ("misleading.pdf".to_string(), "original.pdf".to_string()),
            ]),
            identifier_transformed_to_baseline: BTreeMap::new(),
            reviewed_text_by_field: BTreeMap::new(),
        };
        let result = compare_stage(
            BehavioralStage::SemanticIr,
            &baseline,
            &transformed,
            &normalization,
            None,
        )?;
        assert!(!result.passed);
        assert!(
            result
                .undeclared_delta_paths
                .contains(&"/canonical_fact/role".to_string())
        );
        Ok(())
    }

    #[test]
    fn comparator_accepts_declared_identity_only_delta() -> Result<()> {
        let baseline = synthetic_stage(synthetic_value("original", "producer"), "left");
        let transformed = synthetic_stage(synthetic_value("misleading", "producer"), "right");
        let normalization = NormalizationSpec {
            relation: BehavioralRelation::AdversarialIdentity,
            baseline_run_root: ".project-data/tmp/run".to_string(),
            transformed_run_root: ".project-data/tmp/run".to_string(),
            baseline_source: "original.pdf".to_string(),
            transformed_source: "misleading.pdf".to_string(),
            exact_transformed_to_baseline: BTreeMap::from([
                ("misleading".to_string(), "original".to_string()),
                ("misleading.pdf".to_string(), "original.pdf".to_string()),
            ]),
            identifier_transformed_to_baseline: BTreeMap::new(),
            reviewed_text_by_field: BTreeMap::new(),
        };
        let result = compare_stage(
            BehavioralStage::SemanticIr,
            &baseline,
            &transformed,
            &normalization,
            None,
        )?;
        assert!(result.passed, "{:?}", result.undeclared_delta_paths);
        Ok(())
    }

    #[test]
    fn comparator_reports_nonbijective_derived_ids_as_a_delta() -> Result<()> {
        let mut baseline_value = synthetic_value("original", "producer");
        let mut transformed_value = synthetic_value("original", "producer");
        baseline_value
            .as_object_mut()
            .expect("synthetic object")
            .insert(
                "contracts".to_string(),
                serde_json::json!([
                    {"contract_id": "baseline_a", "kind": "guarantee"},
                    {"contract_id": "baseline_b", "kind": "guarantee"}
                ]),
            );
        transformed_value
            .as_object_mut()
            .expect("synthetic object")
            .insert(
                "contracts".to_string(),
                serde_json::json!([
                    {"contract_id": "transformed_shared", "kind": "guarantee"},
                    {"contract_id": "transformed_shared", "kind": "guarantee"}
                ]),
            );
        let normalization = NormalizationSpec {
            relation: BehavioralRelation::SymbolAlpha,
            baseline_run_root: ".project-data/tmp/run".to_string(),
            transformed_run_root: ".project-data/tmp/run".to_string(),
            baseline_source: "synthetic.md".to_string(),
            transformed_source: "synthetic.md".to_string(),
            exact_transformed_to_baseline: BTreeMap::new(),
            identifier_transformed_to_baseline: BTreeMap::new(),
            reviewed_text_by_field: BTreeMap::new(),
        };
        let result = compare_stage(
            BehavioralStage::SemanticIr,
            &synthetic_stage(baseline_value, "left"),
            &synthetic_stage(transformed_value, "right"),
            &normalization,
            None,
        )?;
        assert!(!result.passed);
        assert!(
            result
                .undeclared_delta_paths
                .iter()
                .any(|path| path.ends_with("/contract_id"))
        );
        Ok(())
    }

    #[test]
    fn semantic_negative_allowance_rejects_an_extra_undeclared_delta() -> Result<()> {
        let baseline = synthetic_stage(synthetic_value("original", "producer"), "left");
        let mut transformed_value = synthetic_value("original", "consumer");
        transformed_value
            .pointer_mut("/canonical_fact")
            .and_then(Value::as_object_mut)
            .expect("synthetic canonical fact")
            .insert(
                "unexpected".to_string(),
                Value::String("must-fail".to_string()),
            );
        let transformed = synthetic_stage(transformed_value, "right");
        let normalization = NormalizationSpec {
            relation: BehavioralRelation::SemanticNegative,
            baseline_run_root: ".project-data/tmp/run".to_string(),
            transformed_run_root: ".project-data/tmp/run".to_string(),
            baseline_source: "synthetic.md".to_string(),
            transformed_source: "synthetic.md".to_string(),
            exact_transformed_to_baseline: BTreeMap::new(),
            identifier_transformed_to_baseline: BTreeMap::new(),
            reviewed_text_by_field: BTreeMap::new(),
        };
        let recipe = synthetic_negative_recipe();
        let result = compare_stage(
            BehavioralStage::SemanticIr,
            &baseline,
            &transformed,
            &normalization,
            Some(&recipe),
        )?;
        assert!(!result.passed);
        assert_eq!(
            result.undeclared_delta_paths,
            vec!["/canonical_fact/unexpected".to_string()]
        );
        Ok(())
    }

    #[test]
    fn incomplete_stage_set_is_rejected() {
        let expected = BehavioralStage::ALL.into_iter().collect::<BTreeSet<_>>();
        let mut incomplete = expected.clone();
        incomplete.remove(&BehavioralStage::IntentIr);
        assert!(!is_complete_stage_set(&incomplete, &expected));
        assert!(is_complete_stage_set(&expected, &expected));
    }

    #[test]
    fn reviewed_recipe_guards_reject_missing_authority_ambiguity_and_unsafe_provenance()
    -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let temporary = crate::project_data::tempdir()?;
        let mut missing_authority = reviewed_request(
            &repository,
            temporary.path(),
            BehavioralRelation::StructurePreservingParaphrase,
            PARAPHRASE_RECIPE_ID,
            "missing-authority",
        )?;
        missing_authority.review_recipe_id = None;
        assert!(validate_request(&missing_authority).is_err());

        let manifest: ReviewedRecipeManifest =
            serde_json::from_slice(&fs::read(repository.join(REVIEW_RECIPE_MANIFEST_PATH))?)?;
        let declaration = manifest
            .recipes
            .iter()
            .find(|declaration| declaration.recipe_id == PARAPHRASE_RECIPE_ID)
            .ok_or_else(|| invalid("paraphrase declaration missing in test"))?;
        let mut recipe: ReviewedTransformRecipe =
            serde_json::from_slice(&fs::read(repository.join(&declaration.path))?)?;
        let repeated = format!(
            "{}\n{}",
            recipe.changed_spans[0].exact_before, recipe.changed_spans[0].exact_before
        );
        assert!(apply_reviewed_recipe(&repeated, &recipe).is_err());

        let first = ReviewedSpanChange {
            change_id: "left".to_string(),
            kind: ReviewedChangeKind::SentenceParaphrase,
            exact_before: "abc".to_string(),
            exact_after: "ABC".to_string(),
            expected_occurrences: 1,
            allowed_provenance_fields: vec!["text".to_string()],
            allow_source_bound_identifier_projection: true,
        };
        let second = ReviewedSpanChange {
            change_id: "right".to_string(),
            exact_before: "bcd".to_string(),
            exact_after: "BCD".to_string(),
            ..first.clone()
        };
        recipe.changed_spans = vec![first, second];
        assert!(apply_reviewed_recipe("abcdef", &recipe).is_err());

        recipe = serde_json::from_slice(&fs::read(repository.join(&declaration.path))?)?;
        recipe.changed_spans[0].allowed_provenance_fields = vec!["role".to_string()];
        assert!(validate_reviewed_recipe(&recipe, declaration).is_err());
        Ok(())
    }

    #[test]
    fn full_reviewed_paraphrase_run_emits_five_stage_evidence() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let temporary = crate::project_data::tempdir()?;
        let request = reviewed_request(
            &repository,
            temporary.path(),
            BehavioralRelation::StructurePreservingParaphrase,
            PARAPHRASE_RECIPE_ID,
            "reviewed-paraphrase",
        )?;
        let report = qualify_behavioral_relation(&request)?;
        assert_eq!(
            report.state,
            BehavioralRunState::Pass,
            "failures={:#?}\nstages={:#?}",
            report.failures,
            report
                .stages
                .iter()
                .map(|stage| (&stage.stage, &stage.undeclared_delta_paths))
                .collect::<Vec<_>>()
        );
        assert_eq!(report.schema_version, 3);
        assert_eq!(report.stages.len(), 5);
        assert_eq!(report.coverage.expected_reviewed_span_deltas, 1);
        assert_eq!(report.coverage.observed_reviewed_span_deltas, 1);
        assert_eq!(report.coverage.preserved_conclusions, 8);
        assert!(!report.transform.source_bytes_equal);
        let evidence = report
            .transform
            .reviewed_recipe
            .as_ref()
            .ok_or_else(|| invalid("reviewed paraphrase evidence is missing"))?;
        assert_eq!(evidence.recipe_id, PARAPHRASE_RECIPE_ID);
        assert_eq!(evidence.changed_spans.len(), 1);
        assert_eq!(
            evidence.unmeasurable_source_surfaces,
            RICH_CAPTURE_EXCLUSIONS
        );
        Ok(())
    }

    #[test]
    fn full_reviewed_layout_run_emits_five_stage_evidence() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let temporary = crate::project_data::tempdir()?;
        let request = reviewed_request(
            &repository,
            temporary.path(),
            BehavioralRelation::HarmlessLayout,
            LAYOUT_RECIPE_ID,
            "reviewed-layout",
        )?;
        let report = qualify_behavioral_relation(&request)?;
        assert_eq!(
            report.state,
            BehavioralRunState::Pass,
            "failures={:#?}\nstages={:#?}",
            report.failures,
            report
                .stages
                .iter()
                .map(|stage| (&stage.stage, &stage.undeclared_delta_paths))
                .collect::<Vec<_>>()
        );
        assert_eq!(report.stages.len(), 5);
        assert_eq!(report.coverage.expected_reviewed_span_deltas, 4);
        assert_eq!(report.coverage.observed_reviewed_span_deltas, 4);
        assert_eq!(report.coverage.preserved_conclusions, 2);
        assert!(!report.transform.source_bytes_equal);
        let evidence = report
            .transform
            .reviewed_recipe
            .as_ref()
            .ok_or_else(|| invalid("reviewed layout evidence is missing"))?;
        assert_eq!(evidence.recipe_id, LAYOUT_RECIPE_ID);
        assert_eq!(evidence.changed_spans.len(), 4);
        assert_eq!(
            evidence
                .changed_spans
                .iter()
                .map(|span| span.kind)
                .collect::<BTreeSet<_>>(),
            [
                ReviewedChangeKind::HeadingLayout,
                ReviewedChangeKind::TableLayout,
                ReviewedChangeKind::WhitespaceLayout,
                ReviewedChangeKind::FormattingLayout,
            ]
            .into_iter()
            .collect()
        );
        Ok(())
    }

    #[test]
    fn full_semantic_negative_run_requires_declared_delta_and_rejects_invariance() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let temporary = crate::project_data::tempdir()?;
        let request = reviewed_request(
            &repository,
            temporary.path(),
            BehavioralRelation::SemanticNegative,
            NEGATIVE_RECIPE_ID,
            "semantic-negative",
        )?;
        let report = qualify_behavioral_relation(&request)?;
        assert_eq!(
            report.state,
            BehavioralRunState::Pass,
            "failures={:#?}\nstages={:#?}",
            report.failures,
            report
                .stages
                .iter()
                .map(|stage| {
                    (
                        &stage.stage,
                        &stage.declared_delta_paths,
                        &stage.undeclared_delta_paths,
                    )
                })
                .collect::<Vec<_>>()
        );
        assert_eq!(report.stages.len(), 5);
        assert_eq!(report.coverage.expected_semantic_deltas, 1);
        assert_eq!(report.coverage.observed_semantic_deltas, 1);
        let negative = report
            .transform
            .reviewed_recipe
            .as_ref()
            .and_then(|recipe| recipe.semantic_negative.as_ref())
            .ok_or_else(|| invalid("semantic-negative evidence is missing"))?;
        assert_eq!(negative.kind, SemanticNegativeKind::TimingGrammarAdmission);
        assert!(negative.invariant_comparison_rejected);
        assert_eq!(negative.dependent_proof_deltas, 3);
        assert!(negative.required_deltas.iter().all(|delta| delta.observed));
        assert_eq!(
            report
                .stages
                .iter()
                .map(|stage| stage.baseline_declared_proof_deltas)
                .sum::<usize>(),
            3
        );
        assert!(report.stages.iter().all(|stage| stage.passed));
        Ok(())
    }

    #[test]
    fn semantic_negative_matrix_rejects_all_nine_control_classes() -> Result<()> {
        let report = qualify_negative_sensitivity_matrix()?;
        assert!(report.passed, "{report:#?}");
        assert_eq!(report.controls.len(), 9);
        assert!(report.controls.iter().all(|control| control.passed));
        assert_eq!(
            report
                .controls
                .iter()
                .map(|control| control.kind)
                .collect::<BTreeSet<_>>()
                .len(),
            9
        );
        assert_eq!(report.attempt_dispositions.len(), 6);
        Ok(())
    }

    #[test]
    fn attempt_disposition_classifier_is_closed() {
        for failure_id in [
            "authority_unavailable",
            "provider_unavailable",
            "vacuous_baseline",
            "eligible_symbol_surface_absent",
        ] {
            assert_eq!(
                classify_attempt_error(&format!("{failure_id}: controlled")),
                (BehavioralRunState::Unmeasurable, failure_id)
            );
        }
        for failure_id in [
            "stale_contract_or_population",
            "ambiguous_or_nonbijective_transform",
            "partial_or_escaped_run",
        ] {
            assert_eq!(
                classify_attempt_error(&format!("{failure_id}: controlled")),
                (BehavioralRunState::Invalid, failure_id)
            );
        }
        assert_eq!(
            classify_attempt_error("unknown closed error"),
            (BehavioralRunState::Invalid, "stale_contract_or_population")
        );
    }

    #[test]
    fn held_out_population_split_strata_and_uncertainty_are_closed() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let contract: HeldOutContractAuthority =
            serde_json::from_slice(&fs::read(repository.join(CONTRACT_PATH))?)?;
        validate_held_out_contract(&contract)?;
        let rows = parse_behavioral_population(&fs::read(repository.join(POPULATION_PATH))?)?;
        validate_behavioral_population(&rows, &contract)?;
        let calibration = rows
            .iter()
            .filter(|row| row.review_role == "reviewed_calibration")
            .collect::<Vec<_>>();
        let prospective = rows
            .iter()
            .filter(|row| row.review_role == "prospective_holdout")
            .collect::<Vec<_>>();
        let split = held_out_split_evidence(
            &contract.selection_boundary_commit,
            &calibration,
            &prospective,
        );
        assert!(split.identity_disjoint);
        assert_eq!(split.calibration_document_keys.len(), 7);
        assert_eq!(split.prospective_document_keys.len(), 17);
        assert!(split.overlapping_document_keys.is_empty());
        assert!(split.overlapping_source_sha256.is_empty());
        assert!(split.overlapping_normalized_markdown_sha256.is_empty());

        let calibration_vendors = calibration
            .iter()
            .map(|row| row.vendor.as_str())
            .collect::<BTreeSet<_>>();
        let calibration_families = calibration
            .iter()
            .map(|row| row.family.as_str())
            .collect::<BTreeSet<_>>();
        let documents = prospective
            .iter()
            .map(|row| HeldOutDocumentEvidence {
                document_key: row.document_key.clone(),
                source_origin: row.source_origin.clone(),
                source_locator: row.source_locator.clone(),
                source_sha256: row.source_sha256.clone(),
                normalized_markdown_path: row.normalized_markdown_path.clone(),
                normalized_markdown_sha256: row.normalized_markdown_sha256.clone(),
                vendor: row.vendor.clone(),
                family: row.family.clone(),
                category: row.category.clone(),
                layout: row.layout.clone(),
                vendor_novel: !calibration_vendors.contains(row.vendor.as_str()),
                family_novel: !calibration_families.contains(row.family.as_str()),
                text_semantic_records: row.text_semantic_records,
                text_intent_records: row.text_intent_records,
            })
            .collect::<Vec<_>>();
        assert_eq!(documents.iter().filter(|row| row.vendor_novel).count(), 4);
        assert_eq!(documents.iter().filter(|row| row.family_novel).count(), 13);

        let relations = vec![
            BehavioralRelation::UnchangedSource,
            BehavioralRelation::AdversarialIdentity,
            BehavioralRelation::SymbolAlpha,
        ];
        let attempts = documents
            .iter()
            .flat_map(|document| {
                relations
                    .iter()
                    .copied()
                    .map(move |relation| HeldOutAttemptEvidence {
                        document_key: document.document_key.clone(),
                        relation,
                        input_plane: relation.input_plane().to_string(),
                        transform_seed: 0,
                        execution_mode: HeldOutExecutionMode::FreshPipeline,
                        state: if relation == BehavioralRelation::SymbolAlpha
                            && document.text_semantic_records == 0
                        {
                            BehavioralRunState::Unmeasurable
                        } else {
                            BehavioralRunState::Pass
                        },
                        failure_id: None,
                        detail: None,
                        identity: None,
                        coverage: None,
                        completed_stages: 0,
                        all_completed_stages_passed: false,
                        attempt_report_sha256: None,
                    })
            })
            .collect::<Vec<_>>();
        let strata = held_out_strata(&documents, &attempts, &relations);
        assert_eq!(strata.len(), 90);
        assert_eq!(
            strata
                .iter()
                .filter(|stratum| {
                    stratum.dimension == "category"
                        && matches!(stratum.value.as_str(), "cpu-isa" | "register-ip")
                })
                .count(),
            6
        );
        assert!(
            strata
                .iter()
                .filter(|stratum| {
                    stratum.dimension == "category"
                        && matches!(stratum.value.as_str(), "cpu-isa" | "register-ip")
                })
                .all(|stratum| {
                    stratum.state == BehavioralRunState::Unmeasurable
                        && stratum.limitation.as_deref() == Some("no_prospective_denominator")
                        && stratum.uncertainty.sample_size == 0
                })
        );
        let interval = held_out_uncertainty(16, 16);
        assert_eq!(interval.point_estimate_parts_per_million, Some(1_000_000));
        assert_eq!(interval.lower_parts_per_million, Some(806_392));
        assert_eq!(interval.upper_parts_per_million, Some(1_000_000));
        Ok(())
    }

    #[test]
    fn missing_source_attempt_is_unmeasurable_without_creating_output() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let temporary = crate::project_data::tempdir()?;
        let output = temporary.path().join("missing-source-attempt");
        let request = BehavioralQualificationRequest {
            relation: BehavioralRelation::SymbolAlpha,
            source_authority: PathBuf::from("does/not/exist.md"),
            expected_source_sha256: "0".repeat(64),
            output_root: output
                .strip_prefix(&repository)
                .map_err(|_| invalid("attempt test output escaped repository"))?
                .to_path_buf(),
            prior_memory: PathBuf::from(PRIOR_MEMORY),
            production_revision: "0".repeat(40),
            transform_seed: 1,
            review_recipe_id: None,
        };
        let attempt = attempt_behavioral_relation(&request);
        assert_eq!(attempt.state, BehavioralRunState::Unmeasurable);
        assert_eq!(attempt.failure_id.as_deref(), Some("authority_unavailable"));
        assert!(attempt.report.is_none());
        assert!(!output.exists());

        let stale_output = temporary.path().join("stale-source-attempt");
        let stale_request = BehavioralQualificationRequest {
            relation: BehavioralRelation::SemanticNegative,
            source_authority: PathBuf::from(REVIEWED_SOURCE),
            expected_source_sha256: "0".repeat(64),
            output_root: stale_output
                .strip_prefix(&repository)
                .map_err(|_| invalid("attempt test output escaped repository"))?
                .to_path_buf(),
            prior_memory: PathBuf::from(PRIOR_MEMORY),
            production_revision: "0".repeat(40),
            transform_seed: 1,
            review_recipe_id: Some(NEGATIVE_RECIPE_ID.to_string()),
        };
        let stale_attempt = attempt_behavioral_relation(&stale_request);
        assert_eq!(stale_attempt.state, BehavioralRunState::Invalid);
        assert_eq!(
            stale_attempt.failure_id.as_deref(),
            Some("stale_contract_or_population")
        );
        assert!(stale_attempt.report.is_none());
        assert!(!stale_output.exists());
        Ok(())
    }

    #[test]
    fn full_alpha_run_emits_five_stage_evidence() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let temporary = crate::project_data::tempdir()?;
        let source = temporary.path().join("alpha_contract.md");
        fs::write(
            &source,
            concat!(
                "# Contract\n",
                "Clock ORBIT.\n\n",
                "Reset CLEAR is asynchronous active low.\n\n",
                "Signal REQUEST is input width 1.\n\n",
                "Signal RESPONSE is output width 1.\n\n",
                "Block respond when REQUEST == 1: RESPONSE = 1.\n",
            ),
        )?;
        let output = temporary.path().join("qualification");
        let request = BehavioralQualificationRequest {
            relation: BehavioralRelation::SymbolAlpha,
            source_authority: source.clone(),
            expected_source_sha256: sha256_file(&source)?,
            output_root: output
                .strip_prefix(&repository)
                .map_err(|_| invalid("test output escaped repository"))?
                .to_path_buf(),
            prior_memory: PathBuf::from(PRIOR_MEMORY),
            production_revision: "0".repeat(40),
            transform_seed: 42,
            review_recipe_id: None,
        };
        let report = qualify_behavioral_relation(&request)?;
        assert_eq!(
            report.state,
            BehavioralRunState::Pass,
            "failures={:#?}\nstages={:#?}",
            report.failures,
            report
                .stages
                .iter()
                .map(|stage| {
                    (
                        &stage.stage,
                        stage
                            .undeclared_delta_paths
                            .iter()
                            .take(30)
                            .collect::<Vec<_>>(),
                    )
                })
                .collect::<Vec<_>>()
        );
        assert_eq!(report.stages.len(), 5);
        assert_eq!(report.coverage.required_stages, 5);
        assert_eq!(
            report.coverage.baseline_proof_claims,
            report.coverage.transformed_proof_claims
        );
        assert!(report.coverage.expected_symbol_deltas >= 2);
        assert!(report.transform.lexical_order_changed);
        assert!(
            report
                .transform
                .symbol_renames
                .iter()
                .all(|rename| rename.familiar_adversarial_spelling)
        );
        assert!(output.join(EVIDENCE_FILE).is_file());
        Ok(())
    }

    #[test]
    #[ignore = "requires the repository-local Docling provider"]
    fn full_pdf_adversarial_identity_run_emits_five_stage_evidence() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let temporary = crate::project_data::tempdir()?;
        let source = temporary.path().join("neutral_contract.pdf");
        fs::write(&source, minimal_pdf_bytes())?;
        let output = temporary.path().join("pdf-qualification");
        let request = BehavioralQualificationRequest {
            relation: BehavioralRelation::AdversarialIdentity,
            source_authority: source.clone(),
            expected_source_sha256: sha256_file(&source)?,
            output_root: output
                .strip_prefix(&repository)
                .map_err(|_| invalid("test output escaped repository"))?
                .to_path_buf(),
            prior_memory: PathBuf::from(PRIOR_MEMORY),
            production_revision: "0".repeat(40),
            transform_seed: 73,
            review_recipe_id: None,
        };
        let report = qualify_behavioral_relation(&request)?;
        assert_eq!(report.state, BehavioralRunState::Pass, "{report:#?}");
        assert_eq!(report.stages.len(), 5);
        assert!(report.transform.source_bytes_equal);
        assert_ne!(
            report.transform.baseline_document_key,
            report.transform.transformed_document_key
        );
        assert_ne!(
            report.transform.baseline_source,
            report.transform.transformed_source
        );
        assert!(output.join(EVIDENCE_FILE).is_file());
        Ok(())
    }

    #[test]
    #[ignore = "requires the repository-local Docling provider"]
    fn full_pdf_unchanged_run_emits_five_stage_evidence() -> Result<()> {
        let repository = crate::project_data::repository_root()?;
        let temporary = crate::project_data::tempdir()?;
        let source = temporary.path().join("unchanged_contract.pdf");
        fs::write(&source, minimal_pdf_bytes())?;
        let output = temporary.path().join("unchanged-qualification");
        let request = BehavioralQualificationRequest {
            relation: BehavioralRelation::UnchangedSource,
            source_authority: source.clone(),
            expected_source_sha256: sha256_file(&source)?,
            output_root: output
                .strip_prefix(&repository)
                .map_err(|_| invalid("test output escaped repository"))?
                .to_path_buf(),
            prior_memory: PathBuf::from(PRIOR_MEMORY),
            production_revision: "0".repeat(40),
            transform_seed: 0,
            review_recipe_id: None,
        };
        let report = qualify_behavioral_relation(&request)?;
        assert_eq!(report.state, BehavioralRunState::Pass, "{report:#?}");
        assert_eq!(report.stages.len(), 5);
        assert!(report.transform.source_bytes_equal);
        assert_eq!(
            report.transform.baseline_document_key,
            report.transform.transformed_document_key
        );
        assert_eq!(
            report.transform.baseline_source,
            report.transform.transformed_source
        );
        assert!(output.join(EVIDENCE_FILE).is_file());
        Ok(())
    }

    #[test]
    fn request_rejects_escaped_output_and_wrong_plane() {
        let escaped = BehavioralQualificationRequest {
            relation: BehavioralRelation::SymbolAlpha,
            source_authority: PathBuf::from("source.md"),
            expected_source_sha256: "0".repeat(64),
            output_root: PathBuf::from("../escaped"),
            prior_memory: PathBuf::from(PRIOR_MEMORY),
            production_revision: "0".repeat(40),
            transform_seed: 1,
            review_recipe_id: None,
        };
        assert!(validate_request(&escaped).is_err());

        let wrong_plane = BehavioralQualificationRequest {
            relation: BehavioralRelation::AdversarialIdentity,
            output_root: PathBuf::from(".project-data/tmp/wrong-plane"),
            ..escaped
        };
        assert!(validate_request(&wrong_plane).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn output_root_rejects_symlink_traversal() -> Result<()> {
        use std::os::unix::fs::symlink;

        let repository = crate::project_data::repository_root()?;
        let temporary = crate::project_data::tempdir()?;
        let target = temporary.path().join("redirect-target");
        fs::create_dir(&target)?;
        let link = temporary.path().join("redirect-link");
        symlink(&target, &link)?;
        let relative = link
            .join("qualification")
            .strip_prefix(&repository)
            .map_err(|_| invalid("test output escaped repository"))?
            .to_path_buf();
        assert!(prepare_output_root(&repository, &relative).is_err());
        Ok(())
    }
}
