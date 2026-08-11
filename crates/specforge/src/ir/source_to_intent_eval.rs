//! Deterministic source-to-IntentIR vertical evaluation.
//!
//! `SPEC-TO-INTENT-ALIGNMENT.4a` deliberately keeps the evaluator independent of extractor
//! implementation types. A reviewed dataset carries bounded JSON stage snapshots plus data-defined
//! JSON-pointer queries. The engine scores those snapshots without document, vendor, protocol, signal,
//! or layout branches in runtime code. This lets the next slice lock held-out source evidence before any
//! extraction behavior is changed and lets mutation tests prove that the oracle detects controlled faults.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{AppError, Result};

const DATASET_SCHEMA_VERSION: u32 = 1;

/// The six reviewed dominant-purpose categories from the source-to-Intent completeness contract.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub enum VerticalCategory {
    WireProtocol,
    RegisterIp,
    PlatformSystemIp,
    CpuIsa,
    PhysicalLink,
    MethodologyGuide,
}

impl VerticalCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WireProtocol => "wire-protocol",
            Self::RegisterIp => "register-ip",
            Self::PlatformSystemIp => "platform-system-ip",
            Self::CpuIsa => "cpu-isa",
            Self::PhysicalLink => "physical-link",
            Self::MethodologyGuide => "methodology-guide",
        }
    }
}

/// Whether a reviewed source cell should become canonical intent or an explicit disposition.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case", deny_unknown_fields)]
pub enum ExpectedDisposition {
    Canonical,
    Residual,
    NonApplicable,
}

/// Portable source identity. External sources are identified without persisting a host path.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "location", rename_all = "snake_case", deny_unknown_fields)]
pub enum SourceIdentity {
    Repository {
        relative_path: String,
        sha256: String,
    },
    ExternalReadOnly {
        portable_id: String,
        sha256: String,
        necessity: String,
    },
}

/// One pinned stage identity and the bounded, reviewed JSON snapshot evaluated by this engine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageSnapshot {
    pub original_sha256: String,
    pub snapshot: Value,
}

/// Four-stage snapshots for one reviewed document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageSnapshots {
    pub source_ir: StageSnapshot,
    pub evidence_ir: StageSnapshot,
    pub semantic_ir: StageSnapshot,
    pub intent_ir: StageSnapshot,
}

/// Original artifact identities retained in the report without duplicating stage snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageIdentities {
    pub source_ir_sha256: String,
    pub evidence_ir_sha256: String,
    pub semantic_ir_sha256: String,
    pub intent_ir_sha256: String,
}

/// Data-defined predicate applied to each object in a query collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "operator", rename_all = "snake_case", deny_unknown_fields)]
pub enum QueryPredicate {
    Equals { field: String, value: Value },
    Prefix { field: String, prefix: String },
    ArrayContainsAny { field: String, values: Vec<Value> },
    RecursiveContains { needle: String },
}

/// A closed-world query over a bounded stage-snapshot collection.
///
/// `expected_keys` is exhaustive for this reviewed source scope. A key is the `|`-joined canonical JSON
/// rendering of `key_fields`; absent fields render as `null`, so missing structure cannot compare equal by
/// accident. Duplicate actual keys remain duplicate predictions and therefore count as false positives.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordQuery {
    pub collection_pointer: String,
    #[serde(default)]
    pub predicates: Vec<QueryPredicate>,
    pub key_fields: Vec<String>,
    pub expected_keys: Vec<String>,
    #[serde(default)]
    pub provenance_fields: Vec<String>,
    #[serde(default)]
    pub required_provenance_ids: Vec<String>,
}

/// Canonical fact queries at the three promoted stages.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CanonicalStageQueries {
    pub evidence_ir: RecordQuery,
    pub semantic_ir: RecordQuery,
    pub intent_ir: RecordQuery,
}

/// Required residuals at the two stages that can explain a promotion loss.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidualQueries {
    pub semantic_ir: RecordQuery,
    pub intent_ir: RecordQuery,
    pub required_actionability_fields: Vec<String>,
}

/// One independently reviewed document × family × modality cell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalCell {
    pub cell_id: String,
    pub semantic_family: String,
    pub modality: String,
    pub oracle: String,
    pub review_scope: String,
    pub complete_gold: bool,
    pub expected_disposition: ExpectedDisposition,
    pub source_region: RecordQuery,
    pub evidence_capture: RecordQuery,
    #[serde(default)]
    pub canonical: Option<CanonicalStageQueries>,
    #[serde(default)]
    pub residual: Option<ResidualQueries>,
}

/// One review-locked document and its bounded vertical cells.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalDocument {
    pub document_key: String,
    pub category: VerticalCategory,
    pub source: SourceIdentity,
    pub review_scope_complete: bool,
    pub present_modalities: Vec<String>,
    pub stages: StageSnapshots,
    pub cells: Vec<VerticalCell>,
}

/// Versioned input contract for deterministic vertical evaluation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerticalEvalDataset {
    pub schema_version: u32,
    pub dataset_id: String,
    pub owner: String,
    pub selection_boundary_commit: String,
    pub selection_claim: String,
    pub minimum_documents_per_category: usize,
    pub documents: Vec<VerticalDocument>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FirstFailingStage {
    SourceIr,
    SourceToEvidenceIr,
    EvidenceToSemanticIr,
    SemanticToIntentIr,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryScore {
    pub actual_keys: Vec<String>,
    pub expected_keys: Vec<String>,
    pub matched_keys: Vec<String>,
    pub actual_total: usize,
    pub expected_total: usize,
    pub true_positives: usize,
    pub false_positives: usize,
    pub false_negatives: usize,
    pub precision: Option<f64>,
    pub recall: Option<f64>,
    pub unprovenanced_records: usize,
}

impl QueryScore {
    fn exact(&self) -> bool {
        self.false_positives == 0 && self.false_negatives == 0 && self.unprovenanced_records == 0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CanonicalStageScores {
    pub evidence_ir: QueryScore,
    pub semantic_ir: QueryScore,
    pub intent_ir: QueryScore,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResidualScores {
    pub semantic_ir: QueryScore,
    pub intent_ir: QueryScore,
    pub semantic_actionable: bool,
    pub intent_actionable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StageBoundary {
    SourceToEvidenceIr,
    EvidenceToSemanticIr,
    SemanticToIntentIr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BoundaryScore {
    pub boundary: StageBoundary,
    pub expected: usize,
    pub conserved_or_residualized: usize,
    pub unexplained_drops: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CellResult {
    pub cell_id: String,
    pub semantic_family: String,
    pub modality: String,
    pub oracle: String,
    pub review_scope: String,
    pub expected_disposition: ExpectedDisposition,
    pub complete_gold: bool,
    pub source_region: QueryScore,
    pub evidence_capture: QueryScore,
    pub canonical: Option<CanonicalStageScores>,
    pub residual: Option<ResidualScores>,
    pub boundaries: Vec<BoundaryScore>,
    pub disposition_accounted: bool,
    pub first_failing_stage: Option<FirstFailingStage>,
    pub hard_failures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DocumentResult {
    pub document_key: String,
    pub category: VerticalCategory,
    pub source: SourceIdentity,
    pub stages: StageIdentities,
    pub review_scope_complete: bool,
    pub present_modalities: usize,
    pub accounted_modalities: usize,
    pub cells: Vec<CellResult>,
    pub hard_failures: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CategoryStatus {
    Supported,
    Incomplete,
    Unmeasurable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CategoryResult {
    pub category: VerticalCategory,
    pub document_count: usize,
    pub cell_count: usize,
    pub hard_failure_count: usize,
    pub status: CategoryStatus,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RatioCount {
    pub met: usize,
    pub total: usize,
    pub ratio: Option<f64>,
}

impl RatioCount {
    fn new(met: usize, total: usize) -> Self {
        Self {
            met,
            total,
            ratio: (total > 0).then_some(met as f64 / total as f64),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalMetrics {
    pub intent_bearing_source_region_disposition: RatioCount,
    pub required_modality_accounting: RatioCount,
    pub canonical_provenance_closure: RatioCount,
    pub stage_conservation_or_residual: RatioCount,
    pub residual_actionability: RatioCount,
    pub fabricated_canonical_facts: usize,
    pub unexplained_stage_drops: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VerticalEvalReport {
    pub schema_version: u32,
    pub dataset_id: String,
    pub owner: String,
    pub selection_boundary_commit: String,
    pub selection_claim: String,
    pub minimum_documents_per_category: usize,
    pub documents: Vec<DocumentResult>,
    pub categories: Vec<CategoryResult>,
    pub global: GlobalMetrics,
}

/// Load and validate a tracked vertical-evaluation dataset.
pub fn load_dataset(path: &Path) -> Result<VerticalEvalDataset> {
    let path_text = path.to_string_lossy();
    if !is_safe_relative_path(&path_text) {
        return Err(AppError::InvalidStageArtifact(format!(
            "source-to-intent dataset path must be repository-relative: {path_text}"
        )));
    }
    let repository = crate::project_data::repository_root()?;
    let absolute = repository.join(path).canonicalize()?;
    if !absolute.starts_with(&repository) {
        return Err(AppError::InvalidStageArtifact(format!(
            "source-to-intent dataset path escapes the repository: {path_text}"
        )));
    }
    let bytes = fs::read(absolute)?;
    let dataset: VerticalEvalDataset = serde_json::from_slice(&bytes)?;
    validate_dataset(&dataset).map_err(|problems| {
        AppError::InvalidStageArtifact(format!(
            "invalid source-to-intent evaluation dataset: {}",
            problems.join("; ")
        ))
    })?;
    Ok(dataset)
}

/// Validate schema, uniqueness, portable identities, and every data-defined query before scoring.
pub fn validate_dataset(dataset: &VerticalEvalDataset) -> std::result::Result<(), Vec<String>> {
    let mut problems = Vec::new();
    if dataset.schema_version != DATASET_SCHEMA_VERSION {
        problems.push(format!(
            "schema_version must be {DATASET_SCHEMA_VERSION}, got {}",
            dataset.schema_version
        ));
    }
    require_text(&dataset.dataset_id, "dataset_id", &mut problems);
    require_text(&dataset.owner, "owner", &mut problems);
    require_text(&dataset.selection_claim, "selection_claim", &mut problems);
    if !is_git_commit(&dataset.selection_boundary_commit) {
        problems.push("selection_boundary_commit must be a lowercase 40-hex Git id".to_string());
    }
    if dataset.minimum_documents_per_category == 0 {
        problems.push("minimum_documents_per_category must be positive".to_string());
    }
    if dataset.documents.is_empty() {
        problems.push("documents must not be empty".to_string());
    }

    let mut document_keys = BTreeSet::new();
    let mut cell_ids = BTreeSet::new();
    for document in &dataset.documents {
        let label = format!("document '{}'", document.document_key);
        require_text(&document.document_key, "document_key", &mut problems);
        if !document_keys.insert(document.document_key.clone()) {
            problems.push(format!("duplicate {label}"));
        }
        validate_source_identity(&document.source, &label, &mut problems);
        validate_stage_snapshot(
            &document.stages.source_ir,
            &label,
            "source_ir",
            &mut problems,
        );
        validate_stage_snapshot(
            &document.stages.evidence_ir,
            &label,
            "evidence_ir",
            &mut problems,
        );
        validate_stage_snapshot(
            &document.stages.semantic_ir,
            &label,
            "semantic_ir",
            &mut problems,
        );
        validate_stage_snapshot(
            &document.stages.intent_ir,
            &label,
            "intent_ir",
            &mut problems,
        );

        let mut modalities = BTreeSet::new();
        for modality in &document.present_modalities {
            require_text(modality, "present modality", &mut problems);
            if !modalities.insert(modality.clone()) {
                problems.push(format!("{label} repeats present modality '{modality}'"));
            }
        }
        if modalities.is_empty() {
            problems.push(format!(
                "{label} must declare at least one present modality"
            ));
        }
        if document.cells.is_empty() {
            problems.push(format!("{label} must contain at least one reviewed cell"));
        }
        for cell in &document.cells {
            let cell_label = format!("{label} cell '{}'", cell.cell_id);
            require_text(&cell.cell_id, "cell_id", &mut problems);
            require_text(&cell.semantic_family, "semantic_family", &mut problems);
            require_text(&cell.modality, "modality", &mut problems);
            require_text(&cell.oracle, "oracle", &mut problems);
            require_text(&cell.review_scope, "review_scope", &mut problems);
            if !cell_ids.insert(cell.cell_id.clone()) {
                problems.push(format!("duplicate cell_id '{}'", cell.cell_id));
            }
            if !modalities.contains(&cell.modality) {
                problems.push(format!(
                    "{cell_label} modality '{}' is absent from present_modalities",
                    cell.modality
                ));
            }
            validate_query(
                &cell.source_region,
                &cell_label,
                "source_region",
                &mut problems,
            );
            validate_query(
                &cell.evidence_capture,
                &cell_label,
                "evidence_capture",
                &mut problems,
            );
            if cell.expected_disposition == ExpectedDisposition::Canonical
                && cell.canonical.is_none()
            {
                problems.push(format!(
                    "{cell_label} canonical disposition needs canonical queries"
                ));
            }
            if cell.expected_disposition != ExpectedDisposition::Canonical
                && cell.residual.is_none()
            {
                problems.push(format!(
                    "{cell_label} non-canonical disposition needs residual queries"
                ));
            }
            if let Some(canonical) = &cell.canonical {
                validate_query(
                    &canonical.evidence_ir,
                    &cell_label,
                    "canonical.evidence_ir",
                    &mut problems,
                );
                validate_query(
                    &canonical.semantic_ir,
                    &cell_label,
                    "canonical.semantic_ir",
                    &mut problems,
                );
                validate_query(
                    &canonical.intent_ir,
                    &cell_label,
                    "canonical.intent_ir",
                    &mut problems,
                );
                let evidence_gold = multiset(&canonical.evidence_ir.expected_keys);
                if evidence_gold != multiset(&canonical.semantic_ir.expected_keys)
                    || evidence_gold != multiset(&canonical.intent_ir.expected_keys)
                {
                    problems.push(format!(
                        "{cell_label} canonical stage queries must carry the same exhaustive gold keys"
                    ));
                }
            }
            if let Some(residual) = &cell.residual {
                validate_query(
                    &residual.semantic_ir,
                    &cell_label,
                    "residual.semantic_ir",
                    &mut problems,
                );
                validate_query(
                    &residual.intent_ir,
                    &cell_label,
                    "residual.intent_ir",
                    &mut problems,
                );
                if residual.required_actionability_fields.is_empty() {
                    problems.push(format!(
                        "{cell_label} residual actionability fields must not be empty"
                    ));
                }
                for field in &residual.required_actionability_fields {
                    validate_pointer(
                        field,
                        &cell_label,
                        "residual actionability field",
                        &mut problems,
                    );
                }
            }
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems)
    }
}

/// Evaluate a validated dataset and return a deterministic, machine-readable report.
pub fn evaluate_dataset(
    dataset: &VerticalEvalDataset,
) -> std::result::Result<VerticalEvalReport, Vec<String>> {
    validate_dataset(dataset)?;
    let mut documents = Vec::new();
    for document in &dataset.documents {
        documents.push(evaluate_document(document)?);
    }
    documents.sort_by(|left, right| left.document_key.cmp(&right.document_key));

    let categories = summarize_categories(dataset, &documents);
    let global = summarize_global(&documents);
    Ok(VerticalEvalReport {
        schema_version: DATASET_SCHEMA_VERSION,
        dataset_id: dataset.dataset_id.clone(),
        owner: dataset.owner.clone(),
        selection_boundary_commit: dataset.selection_boundary_commit.clone(),
        selection_claim: dataset.selection_claim.clone(),
        minimum_documents_per_category: dataset.minimum_documents_per_category,
        documents,
        categories,
        global,
    })
}

fn evaluate_document(
    document: &VerticalDocument,
) -> std::result::Result<DocumentResult, Vec<String>> {
    let mut problems = Vec::new();
    let mut cells = Vec::new();
    for cell in &document.cells {
        match evaluate_cell(document, cell) {
            Ok(result) => cells.push(result),
            Err(problem) => problems.push(format!(
                "document '{}' cell '{}': {problem}",
                document.document_key, cell.cell_id
            )),
        }
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    cells.sort_by(|left, right| left.cell_id.cmp(&right.cell_id));
    let accounted_modalities = document
        .present_modalities
        .iter()
        .filter(|modality| {
            let modality = modality.as_str();
            let matching: Vec<_> = cells
                .iter()
                .filter(|cell| cell.modality == modality)
                .collect();
            !matching.is_empty()
                && matching
                    .iter()
                    .all(|cell| cell.evidence_capture.exact() && cell.disposition_accounted)
        })
        .count();
    let mut hard_failures: Vec<String> = cells
        .iter()
        .flat_map(|cell| {
            cell.hard_failures
                .iter()
                .map(|failure| format!("{}:{failure}", cell.cell_id))
        })
        .collect();
    for modality in &document.present_modalities {
        if !cells
            .iter()
            .any(|cell| cell.modality.as_str() == modality.as_str())
        {
            hard_failures.push(format!("required_modality_unaccounted:{modality}"));
        }
    }
    hard_failures.sort();
    Ok(DocumentResult {
        document_key: document.document_key.clone(),
        category: document.category,
        source: document.source.clone(),
        stages: StageIdentities {
            source_ir_sha256: document.stages.source_ir.original_sha256.clone(),
            evidence_ir_sha256: document.stages.evidence_ir.original_sha256.clone(),
            semantic_ir_sha256: document.stages.semantic_ir.original_sha256.clone(),
            intent_ir_sha256: document.stages.intent_ir.original_sha256.clone(),
        },
        review_scope_complete: document.review_scope_complete,
        present_modalities: document.present_modalities.len(),
        accounted_modalities,
        cells,
        hard_failures,
    })
}

fn evaluate_cell(
    document: &VerticalDocument,
    cell: &VerticalCell,
) -> std::result::Result<CellResult, String> {
    let source_region = score_query(&document.stages.source_ir.snapshot, &cell.source_region)?;
    let evidence_capture = score_query(
        &document.stages.evidence_ir.snapshot,
        &cell.evidence_capture,
    )?;
    let canonical = cell
        .canonical
        .as_ref()
        .map(
            |queries| -> std::result::Result<CanonicalStageScores, String> {
                Ok(CanonicalStageScores {
                    evidence_ir: score_query(
                        &document.stages.evidence_ir.snapshot,
                        &queries.evidence_ir,
                    )?,
                    semantic_ir: score_query(
                        &document.stages.semantic_ir.snapshot,
                        &queries.semantic_ir,
                    )?,
                    intent_ir: score_query(
                        &document.stages.intent_ir.snapshot,
                        &queries.intent_ir,
                    )?,
                })
            },
        )
        .transpose()?;
    let residual = cell
        .residual
        .as_ref()
        .map(|queries| -> std::result::Result<ResidualScores, String> {
            let semantic_ir =
                score_query(&document.stages.semantic_ir.snapshot, &queries.semantic_ir)?;
            let intent_ir = score_query(&document.stages.intent_ir.snapshot, &queries.intent_ir)?;
            Ok(ResidualScores {
                semantic_actionable: query_records_actionable(
                    &document.stages.semantic_ir.snapshot,
                    &queries.semantic_ir,
                    &queries.required_actionability_fields,
                )?,
                intent_actionable: query_records_actionable(
                    &document.stages.intent_ir.snapshot,
                    &queries.intent_ir,
                    &queries.required_actionability_fields,
                )?,
                semantic_ir,
                intent_ir,
            })
        })
        .transpose()?;

    let residual_semantic_ok = residual
        .as_ref()
        .is_some_and(|scores| scores.semantic_ir.exact() && scores.semantic_actionable);
    let residual_intent_ok = residual
        .as_ref()
        .is_some_and(|scores| scores.intent_ir.exact() && scores.intent_actionable);
    let disposition_accounted = match cell.expected_disposition {
        ExpectedDisposition::Canonical => canonical
            .as_ref()
            .is_some_and(|scores| scores.intent_ir.exact()),
        ExpectedDisposition::Residual | ExpectedDisposition::NonApplicable => {
            residual_semantic_ok && residual_intent_ok
        }
    };

    let mut hard_failures = Vec::new();
    if !source_region.exact() {
        hard_failures.push("source_region_missing_or_ambiguous".to_string());
    }
    if !evidence_capture.exact() {
        hard_failures.push("required_modality_capture_failed".to_string());
    }
    if let Some(scores) = &canonical {
        if scores.intent_ir.false_positives > 0 {
            hard_failures.push(format!(
                "fabricated_canonical_facts:{}",
                scores.intent_ir.false_positives
            ));
        }
        if scores.intent_ir.false_negatives > 0 {
            hard_failures.push(format!(
                "reviewed_gold_misses:{}",
                scores.intent_ir.false_negatives
            ));
        }
        if scores.intent_ir.unprovenanced_records > 0 {
            hard_failures.push(format!(
                "canonical_provenance_loss:{}",
                scores.intent_ir.unprovenanced_records
            ));
        }
    }
    if cell.expected_disposition != ExpectedDisposition::Canonical
        && (!residual_semantic_ok || !residual_intent_ok)
    {
        hard_failures.push("required_residual_missing_or_inactionable".to_string());
    }

    let boundaries = boundary_scores(
        cell.expected_disposition,
        &evidence_capture,
        canonical.as_ref(),
        residual.as_ref(),
    );
    let unexplained: usize = boundaries.iter().map(|score| score.unexplained_drops).sum();
    if unexplained > 0 {
        hard_failures.push(format!("unexplained_stage_drops:{unexplained}"));
    }
    if !disposition_accounted {
        hard_failures.push("source_region_disposition_unaccounted".to_string());
    }
    hard_failures.sort();
    hard_failures.dedup();

    let first_failing_stage = if !source_region.exact() {
        Some(FirstFailingStage::SourceIr)
    } else if !evidence_capture.exact()
        || canonical.as_ref().is_some_and(|scores| {
            !scores.evidence_ir.exact()
                || (scores.evidence_ir.false_positives > 0
                    && cell.expected_disposition != ExpectedDisposition::Canonical)
        })
    {
        Some(FirstFailingStage::SourceToEvidenceIr)
    } else if canonical
        .as_ref()
        .is_some_and(|scores| !scores.semantic_ir.exact())
        || (cell.expected_disposition != ExpectedDisposition::Canonical && !residual_semantic_ok)
    {
        Some(FirstFailingStage::EvidenceToSemanticIr)
    } else if canonical
        .as_ref()
        .is_some_and(|scores| !scores.intent_ir.exact())
        || (cell.expected_disposition != ExpectedDisposition::Canonical && !residual_intent_ok)
    {
        Some(FirstFailingStage::SemanticToIntentIr)
    } else {
        None
    };

    Ok(CellResult {
        cell_id: cell.cell_id.clone(),
        semantic_family: cell.semantic_family.clone(),
        modality: cell.modality.clone(),
        oracle: cell.oracle.clone(),
        review_scope: cell.review_scope.clone(),
        expected_disposition: cell.expected_disposition,
        complete_gold: cell.complete_gold,
        source_region,
        evidence_capture,
        canonical,
        residual,
        boundaries,
        disposition_accounted,
        first_failing_stage,
        hard_failures,
    })
}

fn boundary_scores(
    disposition: ExpectedDisposition,
    evidence_capture: &QueryScore,
    canonical: Option<&CanonicalStageScores>,
    residual: Option<&ResidualScores>,
) -> Vec<BoundaryScore> {
    if let Some(canonical) = canonical {
        let residual_semantic = residual
            .filter(|scores| scores.semantic_actionable)
            .map(|scores| &scores.semantic_ir.matched_keys)
            .cloned()
            .unwrap_or_default();
        let residual_intent = residual
            .filter(|scores| scores.intent_actionable)
            .map(|scores| &scores.intent_ir.matched_keys)
            .cloned()
            .unwrap_or_default();
        let source_expected = canonical.evidence_ir.expected_total;
        let source_conserved = (canonical.evidence_ir.true_positives
            + multiset_intersection_count(
                &missing_keys(&canonical.evidence_ir),
                &residual_semantic,
            ))
        .min(source_expected);
        let evidence_expected = canonical.evidence_ir.true_positives;
        let evidence_conserved = (multiset_intersection_count(
            &canonical.evidence_ir.matched_keys,
            &canonical.semantic_ir.matched_keys,
        ) + multiset_intersection_count(
            &missing_between(
                &canonical.evidence_ir.matched_keys,
                &canonical.semantic_ir.matched_keys,
            ),
            &residual_semantic,
        ))
        .min(evidence_expected);
        let semantic_expected = canonical.semantic_ir.true_positives;
        let semantic_conserved = (multiset_intersection_count(
            &canonical.semantic_ir.matched_keys,
            &canonical.intent_ir.matched_keys,
        ) + multiset_intersection_count(
            &missing_between(
                &canonical.semantic_ir.matched_keys,
                &canonical.intent_ir.matched_keys,
            ),
            &residual_intent,
        ))
        .min(semantic_expected);
        vec![
            boundary(
                StageBoundary::SourceToEvidenceIr,
                source_expected,
                source_conserved,
            ),
            boundary(
                StageBoundary::EvidenceToSemanticIr,
                evidence_expected,
                evidence_conserved,
            ),
            boundary(
                StageBoundary::SemanticToIntentIr,
                semantic_expected,
                semantic_conserved,
            ),
        ]
    } else {
        let semantic_ok =
            residual.is_some_and(|scores| scores.semantic_ir.exact() && scores.semantic_actionable);
        let intent_ok =
            residual.is_some_and(|scores| scores.intent_ir.exact() && scores.intent_actionable);
        let capture_ok = evidence_capture.exact();
        let expected = usize::from(disposition != ExpectedDisposition::Canonical);
        vec![
            boundary(
                StageBoundary::SourceToEvidenceIr,
                expected,
                usize::from(capture_ok && expected == 1),
            ),
            boundary(
                StageBoundary::EvidenceToSemanticIr,
                expected,
                usize::from(semantic_ok && expected == 1),
            ),
            boundary(
                StageBoundary::SemanticToIntentIr,
                expected,
                usize::from(intent_ok && expected == 1),
            ),
        ]
    }
}

fn boundary(boundary: StageBoundary, expected: usize, conserved: usize) -> BoundaryScore {
    BoundaryScore {
        boundary,
        expected,
        conserved_or_residualized: conserved,
        unexplained_drops: expected.saturating_sub(conserved),
    }
}

fn summarize_categories(
    dataset: &VerticalEvalDataset,
    documents: &[DocumentResult],
) -> Vec<CategoryResult> {
    let mut categories = Vec::new();
    for category in [
        VerticalCategory::WireProtocol,
        VerticalCategory::RegisterIp,
        VerticalCategory::PlatformSystemIp,
        VerticalCategory::CpuIsa,
        VerticalCategory::PhysicalLink,
        VerticalCategory::MethodologyGuide,
    ] {
        let matching: Vec<_> = documents
            .iter()
            .filter(|document| document.category == category)
            .collect();
        let hard_failure_count = matching
            .iter()
            .map(|document| document.hard_failures.len())
            .sum();
        let complete_scope = matching.iter().all(|document| {
            document.review_scope_complete && document.cells.iter().all(|cell| cell.complete_gold)
        });
        let status = if matching.len() < dataset.minimum_documents_per_category || !complete_scope {
            CategoryStatus::Unmeasurable
        } else if hard_failure_count > 0 {
            CategoryStatus::Incomplete
        } else {
            CategoryStatus::Supported
        };
        categories.push(CategoryResult {
            category,
            document_count: matching.len(),
            cell_count: matching.iter().map(|document| document.cells.len()).sum(),
            hard_failure_count,
            status,
        });
    }
    categories
}

fn summarize_global(documents: &[DocumentResult]) -> GlobalMetrics {
    let cells: Vec<&CellResult> = documents
        .iter()
        .flat_map(|document| document.cells.iter())
        .collect();
    let dispositions_met = cells
        .iter()
        .filter(|cell| cell.disposition_accounted)
        .count();
    let modality_total: usize = documents
        .iter()
        .map(|document| document.present_modalities)
        .sum();
    let modality_met: usize = documents
        .iter()
        .map(|document| document.accounted_modalities)
        .sum();
    let mut provenance_total = 0;
    let mut provenance_met = 0;
    let mut fabricated = 0;
    let mut residual_total = 0;
    let mut residual_met = 0;
    let mut boundary_total = 0;
    let mut boundary_met = 0;
    let mut unexplained = 0;
    for cell in &cells {
        if let Some(canonical) = &cell.canonical {
            provenance_total += canonical.intent_ir.actual_total;
            provenance_met += canonical
                .intent_ir
                .actual_total
                .saturating_sub(canonical.intent_ir.unprovenanced_records);
            fabricated += canonical.intent_ir.false_positives;
        }
        if let Some(residual) = &cell.residual {
            provenance_total += residual.intent_ir.actual_total;
            provenance_met += residual
                .intent_ir
                .actual_total
                .saturating_sub(residual.intent_ir.unprovenanced_records);
            residual_total += 2;
            residual_met +=
                usize::from(residual.semantic_actionable && residual.semantic_ir.exact());
            residual_met += usize::from(residual.intent_actionable && residual.intent_ir.exact());
        }
        for boundary in &cell.boundaries {
            boundary_total += boundary.expected;
            boundary_met += boundary.conserved_or_residualized;
            unexplained += boundary.unexplained_drops;
        }
    }
    GlobalMetrics {
        intent_bearing_source_region_disposition: RatioCount::new(dispositions_met, cells.len()),
        required_modality_accounting: RatioCount::new(modality_met, modality_total),
        canonical_provenance_closure: RatioCount::new(provenance_met, provenance_total),
        stage_conservation_or_residual: RatioCount::new(boundary_met, boundary_total),
        residual_actionability: RatioCount::new(residual_met, residual_total),
        fabricated_canonical_facts: fabricated,
        unexplained_stage_drops: unexplained,
    }
}

fn score_query(snapshot: &Value, query: &RecordQuery) -> std::result::Result<QueryScore, String> {
    let records = matching_records(snapshot, query)?;
    let mut actual_keys: Vec<String> = records
        .iter()
        .map(|record| record_key(record, &query.key_fields))
        .collect();
    actual_keys.sort();
    let mut expected_keys = query.expected_keys.clone();
    expected_keys.sort();
    let actual = multiset(&actual_keys);
    let expected = multiset(&expected_keys);
    let mut matched_keys = Vec::new();
    for (key, expected_count) in &expected {
        let count = (*expected_count).min(*actual.get(key).unwrap_or(&0));
        matched_keys.extend(std::iter::repeat_n(key.clone(), count));
    }
    let true_positives = matched_keys.len();
    let actual_total = actual_keys.len();
    let expected_total = expected_keys.len();
    let unprovenanced_records = if query.required_provenance_ids.is_empty() {
        0
    } else {
        records
            .iter()
            .filter(|record| {
                !query.provenance_fields.iter().any(|field| {
                    record.pointer(field).is_some_and(|value| {
                        query
                            .required_provenance_ids
                            .iter()
                            .any(|id| recursively_contains(value, id))
                    })
                })
            })
            .count()
    };
    Ok(QueryScore {
        actual_keys,
        expected_keys,
        matched_keys,
        actual_total,
        expected_total,
        true_positives,
        false_positives: actual_total.saturating_sub(true_positives),
        false_negatives: expected_total.saturating_sub(true_positives),
        precision: (actual_total > 0).then_some(true_positives as f64 / actual_total as f64),
        recall: (expected_total > 0).then_some(true_positives as f64 / expected_total as f64),
        unprovenanced_records,
    })
}

fn matching_records<'a>(
    snapshot: &'a Value,
    query: &RecordQuery,
) -> std::result::Result<Vec<&'a Value>, String> {
    let collection = snapshot
        .pointer(&query.collection_pointer)
        .ok_or_else(|| format!("missing collection pointer '{}'", query.collection_pointer))?
        .as_array()
        .ok_or_else(|| {
            format!(
                "collection pointer '{}' is not an array",
                query.collection_pointer
            )
        })?;
    Ok(collection
        .iter()
        .filter(|record| {
            query
                .predicates
                .iter()
                .all(|predicate| predicate_matches(record, predicate))
        })
        .collect())
}

fn predicate_matches(record: &Value, predicate: &QueryPredicate) -> bool {
    match predicate {
        QueryPredicate::Equals { field, value } => record.pointer(field) == Some(value),
        QueryPredicate::Prefix { field, prefix } => record
            .pointer(field)
            .and_then(Value::as_str)
            .is_some_and(|value| value.starts_with(prefix)),
        QueryPredicate::ArrayContainsAny { field, values } => record
            .pointer(field)
            .and_then(Value::as_array)
            .is_some_and(|actual| values.iter().any(|value| actual.contains(value))),
        QueryPredicate::RecursiveContains { needle } => recursively_contains(record, needle),
    }
}

fn recursively_contains(value: &Value, needle: &str) -> bool {
    match value {
        Value::String(text) => text == needle,
        Value::Array(values) => values
            .iter()
            .any(|value| recursively_contains(value, needle)),
        Value::Object(values) => values
            .values()
            .any(|value| recursively_contains(value, needle)),
        Value::Null | Value::Bool(_) | Value::Number(_) => false,
    }
}

fn record_key(record: &Value, key_fields: &[String]) -> String {
    key_fields
        .iter()
        .map(|field| canonical_value(record.pointer(field).unwrap_or(&Value::Null)))
        .collect::<Vec<_>>()
        .join("|")
}

fn canonical_value(value: &Value) -> String {
    match value {
        Value::String(text) => text.clone(),
        _ => serde_json::to_string(value).expect("serde_json::Value always serializes"),
    }
}

fn query_records_actionable(
    snapshot: &Value,
    query: &RecordQuery,
    fields: &[String],
) -> std::result::Result<bool, String> {
    let records = matching_records(snapshot, query)?;
    Ok(!records.is_empty()
        && records.iter().all(|record| {
            fields.iter().all(|field| {
                record.pointer(field).is_some_and(|value| match value {
                    Value::Null => false,
                    Value::String(text) => !text.trim().is_empty(),
                    Value::Array(values) => !values.is_empty(),
                    Value::Object(values) => !values.is_empty(),
                    Value::Bool(_) | Value::Number(_) => true,
                })
            })
        }))
}

fn missing_keys(score: &QueryScore) -> Vec<String> {
    missing_between(&score.expected_keys, &score.matched_keys)
}

fn missing_between(expected: &[String], actual: &[String]) -> Vec<String> {
    let expected = multiset(expected);
    let actual = multiset(actual);
    let mut missing = Vec::new();
    for (key, expected_count) in expected {
        let count = expected_count.saturating_sub(*actual.get(&key).unwrap_or(&0));
        missing.extend(std::iter::repeat_n(key, count));
    }
    missing
}

fn multiset(values: &[String]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for value in values {
        *counts.entry(value.clone()).or_default() += 1;
    }
    counts
}

fn multiset_intersection_count(left: &[String], right: &[String]) -> usize {
    let left = multiset(left);
    let right = multiset(right);
    left.iter()
        .map(|(key, count)| (*count).min(*right.get(key).unwrap_or(&0)))
        .sum()
}

fn validate_source_identity(identity: &SourceIdentity, label: &str, problems: &mut Vec<String>) {
    match identity {
        SourceIdentity::Repository {
            relative_path,
            sha256,
        } => {
            if !is_safe_relative_path(relative_path) {
                problems.push(format!(
                    "{label} repository source path is not root-relative and safe"
                ));
            }
            if !is_sha256_digest(sha256) {
                problems.push(format!("{label} repository source SHA-256 is invalid"));
            }
        }
        SourceIdentity::ExternalReadOnly {
            portable_id,
            sha256,
            necessity,
        } => {
            require_text(portable_id, "external portable_id", problems);
            require_text(necessity, "external necessity", problems);
            if portable_id.contains('/') || portable_id.contains('\\') {
                problems.push(format!(
                    "{label} external portable_id must not persist a path"
                ));
            }
            if !is_sha256_digest(sha256) {
                problems.push(format!("{label} external source SHA-256 is invalid"));
            }
        }
    }
}

fn validate_stage_snapshot(
    snapshot: &StageSnapshot,
    label: &str,
    stage: &str,
    problems: &mut Vec<String>,
) {
    if !is_sha256_digest(&snapshot.original_sha256) {
        problems.push(format!("{label} {stage} original SHA-256 is invalid"));
    }
    if !snapshot.snapshot.is_object() {
        problems.push(format!("{label} {stage} snapshot must be a JSON object"));
    }
}

fn validate_query(query: &RecordQuery, cell: &str, name: &str, problems: &mut Vec<String>) {
    validate_pointer(&query.collection_pointer, cell, name, problems);
    if query.key_fields.is_empty() {
        problems.push(format!("{cell} {name} key_fields must not be empty"));
    }
    for field in &query.key_fields {
        validate_pointer(field, cell, &format!("{name} key field"), problems);
    }
    for field in &query.provenance_fields {
        validate_pointer(field, cell, &format!("{name} provenance field"), problems);
    }
    if multiset(&query.expected_keys)
        .values()
        .any(|count| *count > 1)
    {
        problems.push(format!("{cell} {name} repeats an expected fact key"));
    }
    if !query.required_provenance_ids.is_empty() && query.provenance_fields.is_empty() {
        problems.push(format!(
            "{cell} {name} requires provenance ids but declares no provenance fields"
        ));
    }
    for predicate in &query.predicates {
        match predicate {
            QueryPredicate::Equals { field, .. } => {
                validate_pointer(field, cell, &format!("{name} predicate field"), problems)
            }
            QueryPredicate::Prefix { field, prefix } => {
                validate_pointer(field, cell, &format!("{name} predicate field"), problems);
                require_text(prefix, "predicate prefix", problems);
            }
            QueryPredicate::ArrayContainsAny { field, values } => {
                validate_pointer(field, cell, &format!("{name} predicate field"), problems);
                if values.is_empty() {
                    problems.push(format!(
                        "{cell} {name} array predicate values must not be empty"
                    ));
                }
            }
            QueryPredicate::RecursiveContains { needle } => {
                require_text(needle, "recursive predicate needle", problems)
            }
        }
    }
    if multiset(&query.required_provenance_ids)
        .values()
        .any(|count| *count > 1)
    {
        problems.push(format!("{cell} {name} repeats a required provenance id"));
    }
}

fn validate_pointer(pointer: &str, cell: &str, name: &str, problems: &mut Vec<String>) {
    if !pointer.starts_with('/') || pointer.contains("//") {
        problems.push(format!(
            "{cell} {name} '{pointer}' is not a canonical JSON pointer"
        ));
    }
}

fn require_text(value: &str, field: &str, problems: &mut Vec<String>) {
    if value.trim().is_empty() {
        problems.push(format!("{field} must not be empty"));
    }
}

fn is_safe_relative_path(value: &str) -> bool {
    let path = Path::new(value);
    !value.trim().is_empty()
        && !value.contains('\\')
        && !value
            .split('/')
            .next()
            .is_some_and(|part| part.contains(':'))
        && !path.is_absolute()
        && path.components().all(|component| {
            matches!(component, Component::Normal(_) | Component::CurDir)
                && component != Component::CurDir
        })
}

fn is_sha256_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn is_git_commit(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const ZERO_SHA: &str = "0000000000000000000000000000000000000000000000000000000000000000";

    fn query(
        collection: &str,
        source_id: &str,
        expected: &[&str],
        provenance: bool,
    ) -> RecordQuery {
        RecordQuery {
            collection_pointer: collection.to_string(),
            predicates: vec![QueryPredicate::Equals {
                field: "/scope".to_string(),
                value: json!(source_id),
            }],
            key_fields: vec!["/key".to_string()],
            expected_keys: expected.iter().map(|value| (*value).to_string()).collect(),
            provenance_fields: provenance
                .then(|| "/source_ids".to_string())
                .into_iter()
                .collect(),
            required_provenance_ids: provenance
                .then(|| source_id.to_string())
                .into_iter()
                .collect(),
        }
    }

    fn source_query(source_id: &str) -> RecordQuery {
        RecordQuery {
            collection_pointer: "/regions".to_string(),
            predicates: vec![QueryPredicate::Equals {
                field: "/id".to_string(),
                value: json!(source_id),
            }],
            key_fields: vec!["/id".to_string()],
            expected_keys: vec![source_id.to_string()],
            provenance_fields: Vec::new(),
            required_provenance_ids: Vec::new(),
        }
    }

    fn capture_query(source_id: &str) -> RecordQuery {
        RecordQuery {
            collection_pointer: "/captures".to_string(),
            predicates: vec![QueryPredicate::Equals {
                field: "/id".to_string(),
                value: json!(source_id),
            }],
            key_fields: vec!["/id".to_string()],
            expected_keys: vec![source_id.to_string()],
            provenance_fields: vec!["/source_ids".to_string()],
            required_provenance_ids: vec![source_id.to_string()],
        }
    }

    fn canonical_cell(cell_id: &str, source_id: &str, modality: &str, fact: &str) -> VerticalCell {
        VerticalCell {
            cell_id: cell_id.to_string(),
            semantic_family: "behavior".to_string(),
            modality: modality.to_string(),
            oracle: "reviewer".to_string(),
            review_scope: "one exact source region".to_string(),
            complete_gold: true,
            expected_disposition: ExpectedDisposition::Canonical,
            source_region: source_query(source_id),
            evidence_capture: capture_query(source_id),
            canonical: Some(CanonicalStageQueries {
                evidence_ir: query("/facts", source_id, &[fact], true),
                semantic_ir: query("/facts", source_id, &[fact], true),
                intent_ir: query("/facts", source_id, &[fact], true),
            }),
            residual: None,
        }
    }

    fn snapshot(value: Value) -> StageSnapshot {
        StageSnapshot {
            original_sha256: ZERO_SHA.to_string(),
            snapshot: value,
        }
    }

    fn baseline_dataset() -> VerticalEvalDataset {
        let source = json!({"regions":[{"id":"s1"},{"id":"s2"}]});
        let evidence = json!({
            "captures":[
                {"id":"s1","source_ids":["s1"]},
                {"id":"s2","source_ids":["s2"]}
            ],
            "facts":[
                {"key":"A","scope":"s1","source_ids":["s1"]},
                {"key":"B","scope":"s2","source_ids":["s2"]}
            ],
            "residuals":[]
        });
        let promoted = json!({
            "facts":[
                {"key":"A","scope":"s1","source_ids":["s1"]},
                {"key":"B","scope":"s2","source_ids":["s2"]}
            ],
            "residuals":[]
        });
        VerticalEvalDataset {
            schema_version: 1,
            dataset_id: "synthetic-vertical".to_string(),
            owner: "SPEC-TO-INTENT-ALIGNMENT.4a".to_string(),
            selection_boundary_commit: "1111111111111111111111111111111111111111".to_string(),
            selection_claim: "synthetic mutation-control population".to_string(),
            minimum_documents_per_category: 1,
            documents: vec![VerticalDocument {
                document_key: "synthetic".to_string(),
                category: VerticalCategory::WireProtocol,
                source: SourceIdentity::Repository {
                    relative_path: "corpus/synthetic.pdf".to_string(),
                    sha256: ZERO_SHA.to_string(),
                },
                review_scope_complete: true,
                present_modalities: vec!["prose".to_string(), "table".to_string()],
                stages: StageSnapshots {
                    source_ir: snapshot(source),
                    evidence_ir: snapshot(evidence),
                    semantic_ir: snapshot(promoted.clone()),
                    intent_ir: snapshot(promoted),
                },
                cells: vec![
                    canonical_cell("cell-prose", "s1", "prose", "A"),
                    canonical_cell("cell-table", "s2", "table", "B"),
                ],
            }],
        }
    }

    fn evaluate(dataset: &VerticalEvalDataset) -> VerticalEvalReport {
        evaluate_dataset(dataset).expect("valid synthetic dataset")
    }

    #[test]
    fn baseline_is_exact_and_supported_for_its_complete_synthetic_scope() {
        let report = evaluate(&baseline_dataset());
        assert_eq!(report.owner, "SPEC-TO-INTENT-ALIGNMENT.4a");
        assert_eq!(
            report.documents[0].source,
            baseline_dataset().documents[0].source
        );
        assert_eq!(report.documents[0].stages.source_ir_sha256, ZERO_SHA);
        assert_eq!(report.documents[0].cells[0].oracle, "reviewer");
        assert_eq!(
            report.documents[0].cells[0].review_scope,
            "one exact source region"
        );
        assert_eq!(report.global.fabricated_canonical_facts, 0);
        assert_eq!(report.global.unexplained_stage_drops, 0);
        assert_eq!(report.global.required_modality_accounting.ratio, Some(1.0));
        assert_eq!(report.global.canonical_provenance_closure.ratio, Some(1.0));
        assert_eq!(report.categories[0].status, CategoryStatus::Supported);
        assert!(
            report.documents[0]
                .cells
                .iter()
                .all(|cell| cell.first_failing_stage.is_none())
        );
    }

    #[test]
    fn omission_mutation_is_caught_at_source_to_evidence() {
        let mut dataset = baseline_dataset();
        dataset.documents[0].stages.evidence_ir.snapshot["facts"] = json!([
            {"key":"B","scope":"s2","source_ids":["s2"]}
        ]);
        let report = evaluate(&dataset);
        let cell = report.documents[0]
            .cells
            .iter()
            .find(|cell| cell.cell_id == "cell-prose")
            .unwrap();
        assert_eq!(
            cell.first_failing_stage,
            Some(FirstFailingStage::SourceToEvidenceIr)
        );
        assert_eq!(
            cell.canonical.as_ref().unwrap().evidence_ir.false_negatives,
            1
        );
        assert!(report.global.unexplained_stage_drops > 0);
    }

    #[test]
    fn fabrication_mutation_is_counted_on_canonical_intent() {
        let mut dataset = baseline_dataset();
        dataset.documents[0].stages.intent_ir.snapshot["facts"]
            .as_array_mut()
            .unwrap()
            .push(json!({"key":"INVENTED","scope":"s1","source_ids":["s1"]}));
        let report = evaluate(&dataset);
        assert_eq!(report.global.fabricated_canonical_facts, 1);
        assert_eq!(report.categories[0].status, CategoryStatus::Incomplete);
    }

    #[test]
    fn provenance_loss_mutation_breaks_closure() {
        let mut dataset = baseline_dataset();
        dataset.documents[0].stages.intent_ir.snapshot["facts"][0]["source_ids"] = json!([]);
        let report = evaluate(&dataset);
        assert_eq!(report.global.canonical_provenance_closure.met, 1);
        assert_eq!(report.global.canonical_provenance_closure.total, 2);
        assert!(
            report.documents[0]
                .hard_failures
                .iter()
                .any(|failure| { failure.contains("canonical_provenance_loss") })
        );
    }

    #[test]
    fn silent_stage_drop_mutation_is_not_hidden_by_prior_stage_success() {
        let mut dataset = baseline_dataset();
        dataset.documents[0].stages.intent_ir.snapshot["facts"] = json!([
            {"key":"B","scope":"s2","source_ids":["s2"]}
        ]);
        let report = evaluate(&dataset);
        let cell = report.documents[0]
            .cells
            .iter()
            .find(|cell| cell.cell_id == "cell-prose")
            .unwrap();
        assert_eq!(
            cell.first_failing_stage,
            Some(FirstFailingStage::SemanticToIntentIr)
        );
        assert_eq!(
            cell.boundaries[2],
            BoundaryScore {
                boundary: StageBoundary::SemanticToIntentIr,
                expected: 1,
                conserved_or_residualized: 0,
                unexplained_drops: 1,
            }
        );
    }

    #[test]
    fn missing_modality_mutation_breaks_required_modality_accounting() {
        let mut dataset = baseline_dataset();
        dataset.documents[0]
            .cells
            .retain(|cell| cell.modality != "table");
        let report = evaluate(&dataset);
        assert_eq!(report.global.required_modality_accounting.met, 1);
        assert_eq!(report.global.required_modality_accounting.total, 2);
        assert!(
            report.documents[0]
                .hard_failures
                .contains(&"required_modality_unaccounted:table".to_string())
        );
    }

    #[test]
    fn residual_must_be_actionable_and_survive_both_promoted_stages() {
        let mut dataset = baseline_dataset();
        let document = &mut dataset.documents[0];
        document.cells[0] = VerticalCell {
            cell_id: "cell-prose".to_string(),
            semantic_family: "software_guidance".to_string(),
            modality: "prose".to_string(),
            oracle: "reviewer".to_string(),
            review_scope: "one exact source region".to_string(),
            complete_gold: true,
            expected_disposition: ExpectedDisposition::NonApplicable,
            source_region: source_query("s1"),
            evidence_capture: capture_query("s1"),
            canonical: None,
            residual: Some(ResidualQueries {
                semantic_ir: query("/residuals", "s1", &["A"], true),
                intent_ir: query("/residuals", "s1", &["A"], true),
                required_actionability_fields: vec![
                    "/reason".to_string(),
                    "/first_failing_stage".to_string(),
                    "/replay".to_string(),
                ],
            }),
        };
        let residual = json!({
            "key":"A",
            "scope":"s1",
            "source_ids":["s1"],
            "reason":"outside executable digital intent",
            "first_failing_stage":"evidence_to_semantic_ir",
            "replay":"review source region s1"
        });
        document.stages.semantic_ir.snapshot["residuals"] = json!([residual.clone()]);
        document.stages.intent_ir.snapshot["residuals"] = json!([residual]);
        let baseline = evaluate(&dataset);
        assert!(baseline.documents[0].cells[0].disposition_accounted);

        dataset.documents[0].stages.intent_ir.snapshot["residuals"][0]["replay"] = json!("");
        let mutated = evaluate(&dataset);
        assert!(!mutated.documents[0].cells[0].disposition_accounted);
        assert_eq!(mutated.global.residual_actionability.met, 1);
        assert_eq!(mutated.global.residual_actionability.total, 2);
        assert_eq!(mutated.global.required_modality_accounting.met, 1);
        assert_eq!(mutated.global.required_modality_accounting.total, 2);
    }

    #[test]
    fn incomplete_gold_stays_unmeasurable_when_a_provisional_score_fails() {
        let mut dataset = baseline_dataset();
        dataset.documents[0].cells[0].complete_gold = false;
        dataset.documents[0].stages.intent_ir.snapshot["facts"] = json!([]);
        let report = evaluate(&dataset);
        assert_eq!(report.categories[0].status, CategoryStatus::Unmeasurable);
        assert!(!report.documents[0].hard_failures.is_empty());
    }

    #[test]
    fn report_serialization_is_deterministic() {
        let report = evaluate(&baseline_dataset());
        let first = serde_json::to_string_pretty(&report).unwrap();
        let second = serde_json::to_string_pretty(&evaluate(&baseline_dataset())).unwrap();
        assert_eq!(first, second);
    }

    #[test]
    fn public_schema_and_serde_dataset_contract_stay_in_lockstep() {
        let schema: Value = serde_json::from_str(include_str!(
            "../../../../doctrine/spec_to_intent_vertical_eval_schema.json"
        ))
        .unwrap();
        assert_eq!(schema["properties"]["schema_version"]["const"], json!(1));
        assert_eq!(
            schema["$defs"]["category"]["enum"]
                .as_array()
                .unwrap()
                .len(),
            6
        );
        assert_eq!(
            schema["$defs"]["source_identity"]["oneOf"]
                .as_array()
                .unwrap()
                .len(),
            2
        );

        let encoded = serde_json::to_value(baseline_dataset()).unwrap();
        let decoded: VerticalEvalDataset = serde_json::from_value(encoded).unwrap();
        assert_eq!(decoded, baseline_dataset());
    }

    #[test]
    fn dataset_rejects_absolute_and_external_host_paths() {
        let mut dataset = baseline_dataset();
        dataset.documents[0].source = SourceIdentity::Repository {
            relative_path: "/tmp/source.pdf".to_string(),
            sha256: ZERO_SHA.to_string(),
        };
        assert!(
            validate_dataset(&dataset)
                .unwrap_err()
                .iter()
                .any(|problem| problem.contains("root-relative"))
        );

        dataset.documents[0].source = SourceIdentity::ExternalReadOnly {
            portable_id: "/Volumes/private/source.pdf".to_string(),
            sha256: ZERO_SHA.to_string(),
            necessity: "category coverage".to_string(),
        };
        assert!(
            validate_dataset(&dataset)
                .unwrap_err()
                .iter()
                .any(|problem| problem.contains("must not persist a path"))
        );
    }

    #[test]
    fn dataset_loader_resolves_a_safe_path_from_the_repository_root() {
        let temporary = crate::project_data::tempdir().unwrap();
        let dataset_path = temporary.path().join("vertical-eval.json");
        fs::write(
            &dataset_path,
            serde_json::to_vec_pretty(&baseline_dataset()).unwrap(),
        )
        .unwrap();
        let repository = crate::project_data::repository_root().unwrap();
        let relative = dataset_path.strip_prefix(repository).unwrap();

        assert_eq!(load_dataset(relative).unwrap(), baseline_dataset());
    }
}
