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
use crate::ir::source::SourceIr;

const BEHAVIORAL_SCHEMA_VERSION: u32 = 2;
const CONTRACT_PATH: &str = "doctrine/production_genericity/behavioral_qualification.json";
const REVIEW_RECIPE_MANIFEST_PATH: &str =
    "doctrine/production_genericity/reviewed_recipe_manifest.json";
const REVIEW_RECIPE_ROOT: &str = "doctrine/production_genericity/reviewed_recipes";
const TEMP_ROOT: &str = ".project-data/tmp";
const EVIDENCE_FILE: &str = "behavioral_evidence.json";
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

const SCHEMA_SYMBOL_FIELDS: &[&str] = &[
    "actor_name",
    "signal_name",
    "state_name",
    "source_state",
    "target_state",
    "symbol_name",
    "member_name",
    "base_name",
    "module_name",
    "top_name",
    "port_name",
    "instance_name",
    "source_module_name",
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehavioralRelation {
    UnchangedSource,
    AdversarialIdentity,
    SymbolAlpha,
    StructurePreservingParaphrase,
    HarmlessLayout,
}

impl BehavioralRelation {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UnchangedSource => "unchanged_source",
            Self::AdversarialIdentity => "adversarial_identity",
            Self::SymbolAlpha => "symbol_alpha",
            Self::StructurePreservingParaphrase => "structure_preserving_paraphrase",
            Self::HarmlessLayout => "harmless_layout",
        }
    }

    pub fn input_plane(self) -> &'static str {
        match self {
            Self::UnchangedSource | Self::AdversarialIdentity => "pdf_full_capture",
            Self::SymbolAlpha => "normalized_text_projection",
            Self::StructurePreservingParaphrase | Self::HarmlessLayout => "reviewed_variant",
        }
    }

    fn is_reviewed(self) -> bool {
        matches!(
            self,
            Self::StructurePreservingParaphrase | Self::HarmlessLayout
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

#[derive(Debug, Clone)]
struct StageArtifact {
    identity: ArtifactIdentity,
    value: Value,
}

#[derive(Debug)]
struct PipelineArtifacts {
    stages: BTreeMap<BehavioralStage, StageArtifact>,
    evidence_ir: EvidenceIr,
    semantic_ir: SemanticIr,
    document_key: String,
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
    preserved_conclusions: Vec<ReviewedConclusion>,
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

/// Execute one complete frozen relation and write machine-readable evidence below `output_root`.
pub fn qualify_behavioral_relation(
    request: &BehavioralQualificationRequest,
) -> Result<BehavioralQualificationReport> {
    validate_request(request)?;
    let repository = crate::project_data::repository_root()?;
    let output_root = prepare_output_root(&repository, &request.output_root)?;
    let source_authority = resolve_source_authority(&repository, &request.source_authority)?;
    let source_bytes = fs::read(&source_authority)?;
    let source_sha256 = sha256_bytes(&source_bytes);
    if source_sha256 != request.expected_source_sha256 {
        return Err(invalid(format!(
            "behavioral source SHA-256 differs: {source_sha256} != {}",
            request.expected_source_sha256
        )));
    }
    ensure_same_filesystem(&repository, &source_authority)?;
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
            let symbols = derive_source_symbol_catalog(
                &baseline.evidence_ir,
                &baseline.semantic_ir,
                &source_text,
            )?;
            if symbols.is_empty() {
                return Err(invalid(
                    "symbol-alpha baseline has no unambiguous source-bound identifier",
                ));
            }
            let (transformed, renames) =
                alpha_transform(&source_text, &symbols, request.transform_seed)?;
            let path = output_root.join("inputs/transformed").join(&portable_id);
            write_new(&path, transformed.as_bytes())?;
            (path, renames, Vec::new())
        }
        BehavioralRelation::StructurePreservingParaphrase | BehavioralRelation::HarmlessLayout => {
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
    let (mut state, stages, mut failures) =
        compare_pipelines(&baseline, &transformed, &normalization)?;
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
            if !matches!(extension.as_str(), "md" | "markdown") =>
        {
            Err(invalid(
                "text-projection behavioral relation requires normalized Markdown",
            ))
        }
        _ => Ok(()),
    }
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
        || manifest.owner != "SPEC-TO-INTENT-ALIGNMENT.6d.ii.f.ii.b"
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
    Ok(())
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
    let semantic_ir = SemanticIr::load_from_path(&semantic_path)?;

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
        semantic_ir,
    })
}

fn compare_pipelines(
    baseline: &PipelineArtifacts,
    transformed: &PipelineArtifacts,
    normalization: &NormalizationSpec,
) -> Result<(BehavioralRunState, Vec<StageComparison>, Vec<String>)> {
    let expected = BehavioralStage::ALL.into_iter().collect::<BTreeSet<_>>();
    let baseline_stages = baseline.stages.keys().copied().collect::<BTreeSet<_>>();
    let transformed_stages = transformed.stages.keys().copied().collect::<BTreeSet<_>>();
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
            .stages
            .get(&stage)
            .ok_or_else(|| invalid("baseline stage disappeared during comparison"))?;
        let right = transformed
            .stages
            .get(&stage)
            .ok_or_else(|| invalid("transformed stage disappeared during comparison"))?;
        let comparison = compare_stage(stage, left, right, normalization)?;
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
        && baseline.document_key == transformed.document_key
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
) -> Result<StageComparison> {
    let baseline_top_level_fields = object_len(&baseline.value)?;
    let transformed_top_level_fields = object_len(&transformed.value)?;
    let baseline_proof_claims = proof_claim_count(&baseline.value)?;
    let transformed_proof_claims = proof_claim_count(&transformed.value)?;
    let normalized_delta_paths = difference_paths(&baseline.value, &transformed.value);

    let mut left = basic_normalize(&baseline.value, normalization, false);
    let mut right = basic_normalize(&transformed.value, normalization, true);
    normalize_isf_interface_order(&mut left);
    normalize_isf_interface_order(&mut right);
    normalize_unordered_symbol_groups(&mut left);
    normalize_unordered_symbol_groups(&mut right);
    normalize_changed_hashes(&mut left, &mut right, None);
    normalize_relation_bound_scalars(&mut left, &mut right, normalization.relation, None);
    for _ in 0..8 {
        canonicalize_keyed_collections(&mut left);
        canonicalize_keyed_collections(&mut right);
        let mut derived_ids = BTreeMap::new();
        let mut reverse_ids = BTreeMap::new();
        collect_equivalent_id_pairs(&left, &right, "", &mut derived_ids, &mut reverse_ids)?;
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
        passed: undeclared_delta_paths.is_empty()
            && baseline_top_level_fields == transformed_top_level_fields
            && baseline_proof_claims == transformed_proof_claims,
        undeclared_delta_paths,
    })
}

fn derive_source_symbol_catalog(
    evidence_ir: &EvidenceIr,
    semantic_ir: &SemanticIr,
    source: &str,
) -> Result<Vec<String>> {
    let mut candidates = declared_signal_catalog(evidence_ir);
    let semantic = serde_json::to_value(semantic_ir)?;
    collect_schema_symbols(&semantic, &mut candidates);
    let mut seen = BTreeSet::new();
    candidates.retain(|candidate| {
        is_identifier(candidate)
            && contains_identifier(source, candidate)
            && seen.insert(candidate.to_ascii_lowercase())
    });
    Ok(candidates)
}

fn collect_schema_symbols(value: &Value, output: &mut Vec<String>) {
    match value {
        Value::Object(object) => {
            for (key, child) in object {
                if SCHEMA_SYMBOL_FIELDS.contains(&key.as_str()) {
                    if let Some(symbol) = child.as_str() {
                        output.push(symbol.to_string());
                    }
                } else if key == "signals"
                    && let Some(symbols) = child.as_array()
                {
                    output.extend(symbols.iter().filter_map(Value::as_str).map(str::to_string));
                }
                collect_schema_symbols(child, output);
            }
        }
        Value::Array(values) => {
            for child in values {
                collect_schema_symbols(child, output);
            }
        }
        _ => {}
    }
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
    mapping: &mut BTreeMap<String, String>,
    reverse: &mut BTreeMap<String, String>,
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
                        insert_bijective_mapping(mapping, reverse, right_id, left_id)?;
                    }
                }
            }
            for (field, left_child) in left {
                if let Some(right_child) = right.get(field) {
                    collect_equivalent_id_pairs(
                        left_child,
                        right_child,
                        &format!("{path}/{}", escape_pointer(field)),
                        mapping,
                        reverse,
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
                    mapping,
                    reverse,
                )?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn insert_bijective_mapping(
    mapping: &mut BTreeMap<String, String>,
    reverse: &mut BTreeMap<String, String>,
    transformed: &str,
    baseline: &str,
) -> Result<()> {
    if let Some(existing) = mapping.insert(transformed.to_string(), baseline.to_string())
        && existing != baseline
    {
        return Err(invalid(format!(
            "ambiguous_or_nonbijective_transform: transformed id {transformed} maps twice"
        )));
    }
    if let Some(existing) = reverse.insert(baseline.to_string(), transformed.to_string())
        && existing != transformed
    {
        return Err(invalid(format!(
            "ambiguous_or_nonbijective_transform: baseline id {baseline} maps twice"
        )));
    }
    Ok(())
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
            "behavioral source authority is unavailable: {} ({error})",
            source.display()
        ))
    })?;
    if !canonical.is_file() {
        return Err(invalid(format!(
            "behavioral source authority is not a file: {}",
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
        serde_json::json!({
            "stage": "semantic_ir",
            "document_identity": {"document_key": document_key, "display_name": format!("{document_key}.pdf")},
            "artifact_layout": {"artifact_root": format!(".project-data/tmp/run/{document_key}")},
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
        )?;
        assert!(result.passed, "{:?}", result.undeclared_delta_paths);
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
        assert_eq!(report.schema_version, 2);
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
