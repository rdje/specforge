use std::cmp::Reverse;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::cli::{ProjectValidationArgs, RescanVlmProviderArg, ValidateArgs, VlmProviderArg};
use crate::commands::{doctor, validate};
use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::EvidenceIr;
use crate::ir::intent::IntentIr;
use crate::ir::semantic::SemanticIr;
use crate::ir::source::{
    SourceIr, ValidationFindingRecord, ValidationFindingSeverity, ValidationReportRecord,
};

const VALIDATION_SNAPSHOT_DOC: &str = "VALIDATION_SNAPSHOT.md";
const LIVE_STATUS_DOC: &str = "LIVE_ACHIEVEMENT_STATUS.md";
pub(crate) const VALIDATION_RESCAN_PLAN_PATH: &str = "generated/validation/rescan_plan.json";
const VALIDATION_PROJECTION_START: &str = "<!-- validation_projection:start -->";
const VALIDATION_PROJECTION_END: &str = "<!-- validation_projection:end -->";
const EVIDENCE_VISUAL_MOTIF_CORROBORATION_GUIDANCE: &str =
    "evidence_visual_motif_corroboration_guidance";
const SOURCE_VLM_ENRICHMENT_MISSING_SURFACE_RESCAN_GUIDANCE: &str =
    "source_vlm_enrichment_missing_surface_rescan_guidance";
const EVIDENCE_MISSING_VLM_OBSERVATIONS_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_missing_vlm_observations_surface_rescan_guidance";
const EVIDENCE_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_signal_polarity_conflict_surface_rescan_guidance";
const EVIDENCE_STRUCTURAL_KG_MISSING_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_structural_kg_missing_surface_rescan_guidance";
const EVIDENCE_NORMATIVE_RESIDUAL_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_normative_residual_surface_rescan_guidance";
const EVIDENCE_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_signal_semantic_conflict_surface_rescan_guidance";
const EVIDENCE_NEGATIVE_KNOWLEDGE_RESCAN_GUIDANCE: &str =
    "evidence_negative_knowledge_rescan_guidance";
const SEMANTIC_NEGATIVE_KNOWLEDGE_RESCAN_GUIDANCE: &str =
    "semantic_negative_knowledge_rescan_guidance";
const INTENT_NEGATIVE_KNOWLEDGE_RESCAN_GUIDANCE: &str = "intent_negative_knowledge_rescan_guidance";
const SEMANTIC_TEMPORAL_RULE_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_rule_surface_rescan_guidance";
const INTENT_TEMPORAL_RULE_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_rule_surface_rescan_guidance";
const SEMANTIC_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_role_arbitration_surface_rescan_guidance";
const INTENT_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_role_arbitration_surface_rescan_guidance";
const SEMANTIC_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_role_consensus_surface_rescan_guidance";
const INTENT_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_role_consensus_surface_rescan_guidance";
const SEMANTIC_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_alias_dependent_semantic_consensus_surface_rescan_guidance";
const INTENT_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_alias_dependent_semantic_consensus_surface_rescan_guidance";
const SEMANTIC_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_prior_guided_semantic_consensus_surface_rescan_guidance";
const INTENT_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_prior_guided_semantic_consensus_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_cycle_window_surface_rescan_guidance";
const INTENT_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_cycle_window_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_clock_grounding_surface_rescan_guidance";
const INTENT_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_clock_grounding_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_actor_grounding_surface_rescan_guidance";
const INTENT_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_actor_grounding_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_conflict_surface_rescan_guidance";
const INTENT_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_conflict_surface_rescan_guidance";
const SEMANTIC_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_graph_direction_coverage_surface_rescan_guidance";
const INTENT_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_graph_direction_coverage_surface_rescan_guidance";
const SEMANTIC_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_actor_port_gap_surface_rescan_guidance";
const INTENT_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_actor_port_gap_surface_rescan_guidance";
const SEMANTIC_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_graph_direction_conflict_surface_rescan_guidance";
const INTENT_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_graph_direction_conflict_surface_rescan_guidance";
const SEMANTIC_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_connectivity_missing_producer_surface_rescan_guidance";
const INTENT_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_connectivity_missing_producer_surface_rescan_guidance";
const SEMANTIC_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_connectivity_missing_consumer_surface_rescan_guidance";
const INTENT_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_connectivity_missing_consumer_surface_rescan_guidance";
const SEMANTIC_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_interface_signal_conflict_surface_rescan_guidance";
const INTENT_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_interface_signal_conflict_surface_rescan_guidance";
const SEMANTIC_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_signal_connectivity_conflict_surface_rescan_guidance";
const INTENT_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_signal_connectivity_conflict_surface_rescan_guidance";
const SEMANTIC_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_signal_polarity_conflict_surface_rescan_guidance";
const INTENT_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_signal_polarity_conflict_surface_rescan_guidance";
const SEMANTIC_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_signal_semantic_conflict_surface_rescan_guidance";
const INTENT_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_signal_semantic_conflict_surface_rescan_guidance";

#[derive(Debug, Clone)]
struct ProjectedArtifactSnapshot {
    document_key: String,
    display_name: String,
    stage: IrStage,
    artifact_path: PathBuf,
    replay_inputs: Vec<ProjectedReplayInput>,
    report: ValidationReportRecord,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ProjectedReplayInput {
    input_kind: &'static str,
    path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RescanVlmHintPolicy {
    provider: RescanVlmProviderArg,
    model: Option<String>,
}

impl RescanVlmHintPolicy {
    fn from_args(args: &ProjectValidationArgs) -> Self {
        Self {
            provider: args.rescan_vlm_provider,
            model: args.rescan_vlm_model.clone(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ProjectRescanReplayInput {
    pub(crate) input_kind: String,
    pub(crate) path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ProjectRescanCommandHint {
    pub(crate) intent: String,
    pub(crate) executable: String,
    pub(crate) args: Vec<String>,
    pub(crate) working_directory: String,
    pub(crate) display: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ProjectRescanValidationSnapshot {
    pub(crate) artifact_fingerprint: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) overall_score: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) grade: Option<String>,
    pub(crate) finding_count: usize,
    #[serde(default)]
    pub(crate) finding_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ProjectRescanValidationDelta {
    pub(crate) fingerprint_changed: bool,
    #[serde(default)]
    pub(crate) score_changed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) score_delta: Option<i32>,
    #[serde(default)]
    pub(crate) grade_changed: bool,
    pub(crate) finding_count_delta: i64,
    #[serde(default)]
    pub(crate) added_findings: Vec<String>,
    #[serde(default)]
    pub(crate) removed_findings: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ProjectRescanExecutionSummary {
    pub(crate) automation_status: String,
    pub(crate) arbitration_verdict: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub(crate) promotion_status: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub(crate) promotion_blockers: Vec<String>,
    #[serde(default)]
    pub(crate) promotion_review: ProjectRescanPromotionReview,
    pub(crate) before_validation: ProjectRescanValidationSnapshot,
    pub(crate) after_validation: ProjectRescanValidationSnapshot,
    pub(crate) validation_delta: ProjectRescanValidationDelta,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ProjectRescanPromotionReview {
    #[serde(default)]
    pub(crate) review_status: String,
    #[serde(default)]
    pub(crate) approval_policy: String,
    #[serde(default)]
    pub(crate) required_decisions: Vec<String>,
    #[serde(default)]
    pub(crate) approval_record_required: bool,
    #[serde(default)]
    pub(crate) canonical_mutation_allowed: bool,
}

pub(crate) const RESCAN_PROMOTION_NOT_PROMOTED_NO_CHANGE: &str = "not_promoted_no_change";
pub(crate) const RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED: &str =
    "not_promoted_review_required";
pub(crate) const RESCAN_PROMOTION_REVIEW_NOT_REVIEWABLE_NO_CHANGE: &str =
    "not_reviewable_no_change";
pub(crate) const RESCAN_PROMOTION_REVIEW_HUMAN_REVIEW_REQUIRED: &str = "human_review_required";
const RESCAN_PROMOTION_BLOCKER_CANONICAL_IR_NOT_MUTATED: &str =
    "canonical_ir_not_mutated_by_rescan_plan";
const RESCAN_PROMOTION_BLOCKER_VALIDATION_DELTA_NOT_TRUTH: &str =
    "validation_delta_is_not_truth_promotion";
const RESCAN_PROMOTION_BLOCKER_CURRENT_EVIDENCE_REVIEW: &str =
    "current_document_evidence_review_required";
const RESCAN_PROMOTION_BLOCKER_NO_DELTA: &str = "no_validation_delta_to_promote";
const RESCAN_PROMOTION_APPROVAL_POLICY_NO_DELTA: &str = "no_promotion_without_validation_delta";
const RESCAN_PROMOTION_APPROVAL_POLICY_HUMAN_REVIEW: &str =
    "current_document_evidence_review_before_canonical_mutation";

pub(crate) fn rescan_promotion_status_for(arbitration_verdict: &str) -> &'static str {
    if arbitration_verdict == "validated_no_change" {
        RESCAN_PROMOTION_NOT_PROMOTED_NO_CHANGE
    } else {
        RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED
    }
}

pub(crate) fn rescan_promotion_blockers_for(arbitration_verdict: &str) -> Vec<String> {
    let mut blockers = vec![RESCAN_PROMOTION_BLOCKER_CANONICAL_IR_NOT_MUTATED.to_string()];
    if arbitration_verdict == "validated_no_change" {
        blockers.push(RESCAN_PROMOTION_BLOCKER_NO_DELTA.to_string());
    } else {
        blockers.push(RESCAN_PROMOTION_BLOCKER_VALIDATION_DELTA_NOT_TRUTH.to_string());
        blockers.push(RESCAN_PROMOTION_BLOCKER_CURRENT_EVIDENCE_REVIEW.to_string());
    }
    blockers
}

pub(crate) fn rescan_promotion_review_for(
    arbitration_verdict: &str,
) -> ProjectRescanPromotionReview {
    if arbitration_verdict == "validated_no_change" {
        ProjectRescanPromotionReview {
            review_status: RESCAN_PROMOTION_REVIEW_NOT_REVIEWABLE_NO_CHANGE.to_string(),
            approval_policy: RESCAN_PROMOTION_APPROVAL_POLICY_NO_DELTA.to_string(),
            required_decisions: vec!["no_validation_delta_to_review".to_string()],
            approval_record_required: false,
            canonical_mutation_allowed: false,
        }
    } else {
        ProjectRescanPromotionReview {
            review_status: RESCAN_PROMOTION_REVIEW_HUMAN_REVIEW_REQUIRED.to_string(),
            approval_policy: RESCAN_PROMOTION_APPROVAL_POLICY_HUMAN_REVIEW.to_string(),
            required_decisions: vec![
                "current_document_evidence_supports_delta".to_string(),
                "validation_delta_reviewed_for_regression_or_improvement".to_string(),
                "canonical_ir_mutation_scope_is_explicitly_approved".to_string(),
                "prior_memory_not_used_as_truth_authority".to_string(),
            ],
            approval_record_required: true,
            canonical_mutation_allowed: false,
        }
    }
}

pub(crate) fn normalize_rescan_execution_summary(summary: &mut ProjectRescanExecutionSummary) {
    let expected_status = rescan_promotion_status_for(&summary.arbitration_verdict);
    if summary.promotion_status != expected_status {
        summary.promotion_status = expected_status.to_string();
    }

    for blocker in rescan_promotion_blockers_for(&summary.arbitration_verdict) {
        if !summary.promotion_blockers.contains(&blocker) {
            summary.promotion_blockers.push(blocker);
        }
    }

    let expected_review = rescan_promotion_review_for(&summary.arbitration_verdict);
    if summary.promotion_review != expected_review {
        summary.promotion_review = expected_review;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ProjectRescanRecommendation {
    pub(crate) document_key: String,
    pub(crate) display_name: String,
    pub(crate) stage: String,
    pub(crate) artifact_path: String,
    pub(crate) replay_inputs: Vec<ProjectRescanReplayInput>,
    pub(crate) finding_id: String,
    pub(crate) related_ids: Vec<String>,
    pub(crate) extractor_lane: String,
    pub(crate) corroboration_policy: String,
    pub(crate) recommended_action: String,
    pub(crate) recommended_commands: Vec<ProjectRescanCommandHint>,
    pub(crate) automation_status: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) execution_summary: Option<ProjectRescanExecutionSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ProjectRescanPlanRecord {
    pub(crate) schema_version: u32,
    pub(crate) generated_by: String,
    pub(crate) recommendation_count: usize,
    pub(crate) recommendations: Vec<ProjectRescanRecommendation>,
}

pub fn run(args: ProjectValidationArgs) -> Result<()> {
    let repo_root = canonicalize_existing_path(&args.repo_root)?;
    let live_status_path = repo_root.join(LIVE_STATUS_DOC);
    if !live_status_path.exists() {
        return Err(AppError::MissingPath(live_status_path));
    }

    let mut snapshots = Vec::new();
    for artifact in &args.artifacts {
        snapshots.push(project_artifact(artifact, &repo_root)?);
    }
    snapshots.sort_by(|left, right| {
        left.document_key
            .cmp(&right.document_key)
            .then_with(|| left.stage.as_str().cmp(right.stage.as_str()))
    });

    let previous_rescan_plan = read_existing_validation_rescan_plan(&repo_root)?;
    let rescan_vlm_policy = RescanVlmHintPolicy::from_args(&args);
    let mut rescan_recommendations =
        collect_rescan_recommendations(&snapshots, &repo_root, &rescan_vlm_policy);
    merge_previous_rescan_execution_state(
        &mut rescan_recommendations,
        previous_rescan_plan.as_ref(),
    );
    let rescan_plan_path =
        write_validation_rescan_plan(&repo_root, rescan_recommendations.clone())?;
    let snapshot_doc_path = repo_root.join(VALIDATION_SNAPSHOT_DOC);
    fs::write(
        &snapshot_doc_path,
        render_validation_snapshot_doc(&snapshots, &repo_root, &rescan_recommendations),
    )?;
    upsert_live_status_projection(
        &live_status_path,
        &snapshots,
        &repo_root,
        &rescan_recommendations,
    )?;

    println!("command: project-validation");
    println!("repo_root: {}", repo_root.display());
    println!("projected_artifacts: {}", snapshots.len());
    println!("rescan_recommendations: {}", rescan_recommendations.len());
    println!("validation_snapshot_path: {}", snapshot_doc_path.display());
    println!("rescan_plan_path: {}", rescan_plan_path.display());
    println!("live_status_path: {}", live_status_path.display());

    Ok(())
}

fn project_artifact(artifact: &Path, repo_root: &Path) -> Result<ProjectedArtifactSnapshot> {
    let artifact_path = normalize_input_path(artifact, repo_root);
    validate::run(ValidateArgs {
        artifact: artifact_path.clone(),
    })?;

    #[derive(Deserialize)]
    struct StageProbe {
        stage: IrStage,
    }

    let raw = fs::read_to_string(&artifact_path)?;
    let probe: StageProbe = serde_json::from_str(&raw).map_err(|error| {
        AppError::InvalidStageArtifact(format!(
            "cannot determine stage from {}: {error}",
            artifact_path.display()
        ))
    })?;

    match probe.stage {
        IrStage::SourceIr => {
            let ir = SourceIr::load_from_path(&artifact_path)?;
            projected_snapshot(
                ir.document_identity.document_key,
                ir.document_identity.display_name,
                probe.stage,
                artifact_path,
                vec![ProjectedReplayInput {
                    input_kind: "source_document",
                    path: ir.source.canonical_path,
                }],
                ir.validation_reports,
            )
        }
        IrStage::EvidenceIr => {
            let ir = EvidenceIr::load_from_path(&artifact_path)?;
            projected_snapshot(
                ir.document_identity.document_key,
                ir.document_identity.display_name,
                probe.stage,
                artifact_path,
                vec![ProjectedReplayInput {
                    input_kind: "source_ir",
                    path: ir.source_ir_path,
                }],
                ir.validation_reports,
            )
        }
        IrStage::SemanticIr => {
            let ir = SemanticIr::load_from_path(&artifact_path)?;
            projected_snapshot(
                ir.document_identity.document_key,
                ir.document_identity.display_name,
                probe.stage,
                artifact_path,
                vec![ProjectedReplayInput {
                    input_kind: "evidence_ir",
                    path: ir.evidence_ir_path,
                }],
                ir.validation_reports,
            )
        }
        IrStage::IntentIr => {
            let ir = IntentIr::load_from_path(&artifact_path)?;
            projected_snapshot(
                ir.document_identity.document_key,
                ir.document_identity.display_name,
                probe.stage,
                artifact_path,
                vec![ProjectedReplayInput {
                    input_kind: "semantic_ir",
                    path: ir.semantic_ir_path,
                }],
                ir.validation_reports,
            )
        }
    }
}

fn projected_snapshot(
    document_key: String,
    display_name: String,
    stage: IrStage,
    artifact_path: PathBuf,
    replay_inputs: Vec<ProjectedReplayInput>,
    validation_reports: Vec<ValidationReportRecord>,
) -> Result<ProjectedArtifactSnapshot> {
    let Some(report) = validation_reports.into_iter().next() else {
        return Err(AppError::InvalidStageArtifact(format!(
            "artifact at {} does not carry a persisted validation report after validation",
            artifact_path.display()
        )));
    };

    Ok(ProjectedArtifactSnapshot {
        document_key,
        display_name,
        stage,
        artifact_path,
        replay_inputs,
        report,
    })
}

fn render_validation_snapshot_doc(
    snapshots: &[ProjectedArtifactSnapshot],
    repo_root: &Path,
    rescan_recommendations: &[ProjectRescanRecommendation],
) -> String {
    let rescan_execution_counts = rescan_execution_counts(rescan_recommendations);
    let mut lines = vec![
        "# VALIDATION_SNAPSHOT".to_string(),
        "This file is auto-refreshed by `specforge project-validation <artifact>...`.".to_string(),
        "It summarizes the latest persisted validation reports projected from IR artifacts into the tracked live-doc surface.".to_string(),
        String::new(),
        "## Snapshot Summary".to_string(),
        format!("- Artifacts projected: {}", snapshots.len()),
        format!(
            "- Highest severity observed: {}",
            highest_severity_label(snapshots)
        ),
        format!(
            "- Targeted rescan recommendations: {}",
            rescan_recommendations.len()
        ),
        format!(
            "- Rescan execution summaries: {}",
            render_rescan_execution_counts(&rescan_execution_counts)
        ),
    ];

    if snapshots.is_empty() {
        lines.push("- Score-bearing artifacts: none".to_string());
    } else {
        lines.push("- Score-bearing artifacts:".to_string());
        for snapshot in snapshots {
            lines.push(format!(
                "  - `{}` (`{}`): `{}`",
                snapshot.display_name,
                snapshot.stage.as_str(),
                score_summary(&snapshot.report)
            ));
        }
    }

    lines.push(String::new());
    lines.push("## Targeted Rescan Recommendations".to_string());
    if rescan_recommendations.is_empty() {
        lines.push("- none".to_string());
    } else {
        for recommendation in rescan_recommendations {
            lines.push(format!(
                "### {} ({})",
                recommendation.display_name, recommendation.stage
            ));
            lines.push(format!(
                "- artifact_path: `{}`",
                recommendation.artifact_path
            ));
            lines.push(format!(
                "- replay_inputs: {}",
                render_replay_input_list(&recommendation.replay_inputs)
            ));
            lines.push(format!("- finding_id: `{}`", recommendation.finding_id));
            lines.push(format!(
                "- extractor_lane: `{}`",
                recommendation.extractor_lane
            ));
            lines.push(format!(
                "- corroboration_policy: `{}`",
                recommendation.corroboration_policy
            ));
            lines.push(format!(
                "- recommended_action: {}",
                recommendation.recommended_action
            ));
            lines.push(format!(
                "- related_ids: {}",
                render_inline_code_list(&recommendation.related_ids)
            ));
            lines.push(format!(
                "- automation_status: `{}`",
                recommendation.automation_status
            ));
            if let Some(summary) = recommendation.execution_summary.as_ref() {
                lines.push(format!(
                    "- execution_summary: `{}` (`{}`)",
                    summary.arbitration_verdict, summary.automation_status
                ));
                lines.push(format!(
                    "- promotion_gate: `{}` (blockers: {})",
                    summary_promotion_status(summary),
                    render_inline_code_list(&summary_promotion_blockers(summary))
                ));
                let promotion_review = summary_promotion_review(summary);
                lines.push(format!(
                    "- promotion_review: `{}` (policy `{}`, approval_record_required `{}`, canonical_mutation_allowed `{}`, required_decisions: {})",
                    promotion_review.review_status,
                    promotion_review.approval_policy,
                    promotion_review.approval_record_required,
                    promotion_review.canonical_mutation_allowed,
                    render_inline_code_list(&promotion_review.required_decisions)
                ));
                lines.push(format!(
                    "- validation_delta: fingerprint_changed `{}`, score_delta `{}`, grade_changed `{}`, finding_count_delta `{}`",
                    summary.validation_delta.fingerprint_changed,
                    render_signed_option_i32(summary.validation_delta.score_delta),
                    summary.validation_delta.grade_changed,
                    render_signed_i64(summary.validation_delta.finding_count_delta)
                ));
                lines.push(format!(
                    "- finding_delta: added {}; removed {}",
                    render_inline_code_list(&summary.validation_delta.added_findings),
                    render_inline_code_list(&summary.validation_delta.removed_findings)
                ));
            }
            if recommendation.recommended_commands.is_empty() {
                lines.push("- recommended_commands: none".to_string());
            } else {
                lines.push("- recommended_commands:".to_string());
                for command in &recommendation.recommended_commands {
                    lines.push(format!("  - `{}`: `{}`", command.intent, command.display));
                }
            }
            lines.push(String::new());
        }
    }

    lines.push(String::new());
    lines.push("## Projected Artifacts".to_string());

    for snapshot in snapshots {
        lines.push(format!(
            "### {} ({})",
            snapshot.display_name,
            snapshot.stage.as_str()
        ));
        lines.push(format!("- document_key: `{}`", snapshot.document_key));
        lines.push(format!(
            "- artifact_path: `{}`",
            repo_relative_display(&snapshot.artifact_path, repo_root)
        ));
        lines.push(format!(
            "- artifact_fingerprint: `{}`",
            snapshot.report.artifact_fingerprint
        ));
        lines.push(format!("- score: `{}`", score_summary(&snapshot.report)));
        lines.push(format!("- summary: {}", snapshot.report.summary));
        lines.push("- findings:".to_string());
        if snapshot.report.findings.is_empty() {
            lines.push("  - none".to_string());
        } else {
            for finding in sorted_findings(&snapshot.report.findings) {
                lines.push(format!(
                    "  - [{}:{}] {}",
                    finding.severity.as_str(),
                    finding.category,
                    finding.summary
                ));
            }
        }
        lines.push(String::new());
    }

    lines.join("\n")
}

fn render_live_status_projection(
    snapshots: &[ProjectedArtifactSnapshot],
    repo_root: &Path,
    rescan_recommendations: &[ProjectRescanRecommendation],
) -> String {
    let rescan_execution_counts = rescan_execution_counts(rescan_recommendations);
    let mut lines = vec!["- Latest projected validation snapshot:".to_string()];
    for snapshot in snapshots {
        lines.push(format!(
            "  - `{}` (`{}`): `{}` from `{}`",
            snapshot.display_name,
            snapshot.stage.as_str(),
            score_summary(&snapshot.report),
            repo_relative_display(&snapshot.artifact_path, repo_root)
        ));
    }
    lines.push("- Highest-signal projected findings:".to_string());
    for snapshot in snapshots {
        let finding_summary = primary_finding_summary(&snapshot.report);
        lines.push(format!(
            "  - `{}`: {}",
            snapshot.display_name, finding_summary
        ));
    }
    lines.push(format!(
        "- Rescan execution summaries: {}",
        render_rescan_execution_counts(&rescan_execution_counts)
    ));
    lines.push("- Targeted rescan queue:".to_string());
    if rescan_recommendations.is_empty() {
        lines.push("  - none".to_string());
    } else {
        for recommendation in rescan_recommendations.iter().take(8) {
            lines.push(format!(
                "  - `{}` (`{}`): `{}` for {} (replay `{}`, action: {}, {} command hint(s), `{}`{})",
                recommendation.display_name,
                recommendation.stage,
                recommendation.extractor_lane,
                render_inline_code_list(&recommendation.related_ids),
                render_replay_input_kind_chain(&recommendation.replay_inputs),
                truncate_projection_text(&recommendation.recommended_action, 96),
                recommendation.recommended_commands.len(),
                recommendation.automation_status,
                render_execution_summary_inline(recommendation.execution_summary.as_ref())
            ));
        }
        if rescan_recommendations.len() > 8 {
            lines.push(format!(
                "  - ... and {} more targeted rescan recommendation(s)",
                rescan_recommendations.len() - 8
            ));
        }
    }
    lines.join("\n")
}

fn upsert_live_status_projection(
    live_status_path: &Path,
    snapshots: &[ProjectedArtifactSnapshot],
    repo_root: &Path,
    rescan_recommendations: &[ProjectRescanRecommendation],
) -> Result<()> {
    let current = fs::read_to_string(live_status_path)?;
    let managed_block = format!(
        "{VALIDATION_PROJECTION_START}\n{}\n{VALIDATION_PROJECTION_END}",
        render_live_status_projection(snapshots, repo_root, rescan_recommendations)
    );
    let updated = replace_or_append_managed_section(
        &current,
        VALIDATION_PROJECTION_START,
        VALIDATION_PROJECTION_END,
        &managed_block,
    )?;
    fs::write(live_status_path, updated)?;
    Ok(())
}

fn read_existing_validation_rescan_plan(
    repo_root: &Path,
) -> Result<Option<ProjectRescanPlanRecord>> {
    let plan_path = repo_root.join(VALIDATION_RESCAN_PLAN_PATH);
    if !plan_path.exists() {
        return Ok(None);
    }

    let plan: ProjectRescanPlanRecord = serde_json::from_str(&fs::read_to_string(&plan_path)?)?;
    if plan.schema_version != 2 {
        return Ok(None);
    }

    Ok(Some(plan))
}

fn merge_previous_rescan_execution_state(
    recommendations: &mut [ProjectRescanRecommendation],
    previous_plan: Option<&ProjectRescanPlanRecord>,
) {
    let Some(previous_plan) = previous_plan else {
        return;
    };
    let previous_by_key = previous_plan
        .recommendations
        .iter()
        .filter(|recommendation| {
            recommendation.automation_status != "planned_not_executed"
                || recommendation.execution_summary.is_some()
        })
        .map(|recommendation| (rescan_recommendation_key(recommendation), recommendation))
        .collect::<HashMap<_, _>>();

    for recommendation in recommendations {
        if let Some(previous) = previous_by_key.get(&rescan_recommendation_key(recommendation)) {
            recommendation.automation_status = previous.automation_status.clone();
            recommendation.execution_summary = previous.execution_summary.clone();
            if let Some(summary) = recommendation.execution_summary.as_mut() {
                normalize_rescan_execution_summary(summary);
            }
        }
    }
}

fn rescan_recommendation_key(recommendation: &ProjectRescanRecommendation) -> String {
    let mut related_ids = recommendation.related_ids.clone();
    related_ids.sort();
    related_ids.dedup();
    let mut replay_inputs = recommendation
        .replay_inputs
        .iter()
        .map(|input| format!("{}:{}", input.input_kind, input.path))
        .collect::<Vec<_>>();
    replay_inputs.sort();
    replay_inputs.dedup();
    let command_fingerprint = recommendation
        .recommended_commands
        .iter()
        .map(|command| {
            format!(
                "{}\u{1d}{}\u{1d}{}\u{1d}{}",
                command.intent,
                command.executable,
                command.working_directory,
                command.args.join("\u{1f}")
            )
        })
        .collect::<Vec<_>>()
        .join("\u{1c}");
    [
        recommendation.document_key.as_str(),
        recommendation.stage.as_str(),
        recommendation.artifact_path.as_str(),
        recommendation.finding_id.as_str(),
        recommendation.extractor_lane.as_str(),
        &related_ids.join("\u{1f}"),
        &replay_inputs.join("\u{1f}"),
        &command_fingerprint,
    ]
    .join("\u{1e}")
}

fn write_validation_rescan_plan(
    repo_root: &Path,
    recommendations: Vec<ProjectRescanRecommendation>,
) -> Result<PathBuf> {
    let plan_path = repo_root.join(VALIDATION_RESCAN_PLAN_PATH);
    if let Some(parent) = plan_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let plan = ProjectRescanPlanRecord {
        schema_version: 2,
        generated_by: "specforge project-validation".to_string(),
        recommendation_count: recommendations.len(),
        recommendations,
    };
    fs::write(&plan_path, serde_json::to_string_pretty(&plan)?)?;
    Ok(plan_path)
}

fn collect_rescan_recommendations(
    snapshots: &[ProjectedArtifactSnapshot],
    repo_root: &Path,
    rescan_vlm_policy: &RescanVlmHintPolicy,
) -> Vec<ProjectRescanRecommendation> {
    let mut recommendations = Vec::new();
    for snapshot in snapshots {
        for finding in &snapshot.report.findings {
            if !is_rescan_guidance(finding) {
                continue;
            }

            let mut related_ids = finding.related_ids.clone();
            related_ids.sort();
            related_ids.dedup();
            let replay_inputs =
                recommendation_replay_inputs(snapshot.stage, snapshot, finding, repo_root);
            let recommended_commands = recommended_rescan_commands(
                snapshot.stage,
                &snapshot.artifact_path,
                snapshot,
                finding,
                rescan_vlm_policy,
                repo_root,
            );
            recommendations.push(ProjectRescanRecommendation {
                document_key: snapshot.document_key.clone(),
                display_name: snapshot.display_name.clone(),
                stage: snapshot.stage.as_str().to_string(),
                artifact_path: repo_relative_display(&snapshot.artifact_path, repo_root),
                replay_inputs,
                finding_id: finding.finding_id.clone(),
                related_ids,
                extractor_lane: extractor_lane_for_rescan(snapshot.stage, finding).to_string(),
                corroboration_policy:
                    "stronger_local_corroboration_required_before_canonical_promotion".to_string(),
                recommended_action: recommended_rescan_action(snapshot.stage, finding).to_string(),
                recommended_commands,
                automation_status: "planned_not_executed".to_string(),
                execution_summary: None,
            });
        }
    }
    recommendations.sort_by(|left, right| {
        left.document_key
            .cmp(&right.document_key)
            .then_with(|| left.stage.cmp(&right.stage))
            .then_with(|| left.finding_id.cmp(&right.finding_id))
    });
    recommendations
}

fn recommendation_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
    repo_root: &Path,
) -> Vec<ProjectRescanReplayInput> {
    replay_inputs_for_rescan(stage, snapshot, finding)
        .into_iter()
        .map(|input| ProjectRescanReplayInput {
            input_kind: input.input_kind.to_string(),
            path: repo_relative_display(&input.path, repo_root),
        })
        .collect()
}

fn replay_inputs_for_rescan(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Vec<ProjectedReplayInput> {
    if let Some(inputs) = source_vlm_enrichment_missing_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = evidence_missing_vlm_observations_replay_inputs(stage, snapshot, finding)
    {
        return inputs;
    }
    if let Some(inputs) = evidence_structural_kg_missing_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = evidence_normative_residual_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = evidence_signal_polarity_conflict_replay_inputs(stage, snapshot, finding)
    {
        return inputs;
    }
    if let Some(inputs) = evidence_signal_semantic_conflict_replay_inputs(stage, snapshot, finding)
    {
        return inputs;
    }
    if let Some(inputs) = negative_knowledge_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = semantic_role_consensus_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = alias_dependent_semantic_consensus_replay_inputs(stage, snapshot, finding)
    {
        return inputs;
    }
    if let Some(inputs) = prior_guided_semantic_consensus_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = connectivity_gap_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = interface_signal_conflict_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = temporal_conflict_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = signal_polarity_conflict_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = signal_semantic_conflict_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = signal_connectivity_conflict_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = graph_direction_conflict_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = actor_port_gap_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = graph_direction_coverage_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = temporal_actor_grounding_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = temporal_clock_grounding_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = temporal_cycle_window_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = semantic_role_arbitration_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    if let Some(inputs) = temporal_rule_surface_replay_inputs(stage, snapshot, finding) {
        return inputs;
    }
    snapshot.replay_inputs.clone()
}

fn source_vlm_enrichment_missing_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_source_vlm_enrichment_missing_rescan(stage, finding) {
        return None;
    }

    Some(vec![ProjectedReplayInput {
        input_kind: "source_ir",
        path: snapshot.artifact_path.clone(),
    }])
}

fn negative_knowledge_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_negative_knowledge_rescan(stage, finding) {
        return None;
    }

    let evidence_ir_path = evidence_input_for_snapshot_stage(stage, snapshot)?;
    let source_ir_path = source_ir_input_for_negative_knowledge_rescan(stage, snapshot)?;
    let mut inputs = vec![
        ProjectedReplayInput {
            input_kind: "source_ir",
            path: source_ir_path,
        },
        ProjectedReplayInput {
            input_kind: "evidence_ir",
            path: evidence_ir_path,
        },
    ];
    if stage == IrStage::IntentIr {
        inputs.push(ProjectedReplayInput {
            input_kind: "semantic_ir",
            path: semantic_input_for_snapshot(snapshot)?,
        });
    }
    Some(inputs)
}

fn evidence_signal_semantic_conflict_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_evidence_signal_semantic_conflict_rescan(stage, finding) {
        return None;
    }

    Some(vec![ProjectedReplayInput {
        input_kind: "evidence_ir",
        path: snapshot.artifact_path.clone(),
    }])
}

fn evidence_missing_vlm_observations_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_evidence_missing_vlm_observations_rescan(stage, finding) {
        return None;
    }

    snapshot
        .replay_inputs
        .iter()
        .find(|input| input.input_kind == "source_ir")
        .cloned()
        .map(|input| vec![input])
}

fn evidence_structural_kg_missing_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_evidence_structural_kg_missing_rescan(stage, finding) {
        return None;
    }

    Some(vec![ProjectedReplayInput {
        input_kind: "evidence_ir",
        path: snapshot.artifact_path.clone(),
    }])
}

fn evidence_normative_residual_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_evidence_normative_residual_rescan(stage, finding) {
        return None;
    }

    Some(vec![ProjectedReplayInput {
        input_kind: "evidence_ir",
        path: snapshot.artifact_path.clone(),
    }])
}

fn evidence_signal_polarity_conflict_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_evidence_signal_polarity_conflict_rescan(stage, finding) {
        return None;
    }

    Some(vec![ProjectedReplayInput {
        input_kind: "evidence_ir",
        path: snapshot.artifact_path.clone(),
    }])
}

fn temporal_rule_surface_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_temporal_rule_surface_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn semantic_role_consensus_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_semantic_role_consensus_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn alias_dependent_semantic_consensus_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_alias_dependent_semantic_consensus_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn prior_guided_semantic_consensus_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_prior_guided_semantic_consensus_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn connectivity_gap_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_connectivity_missing_producer_rescan(stage, finding)
        && !is_connectivity_missing_consumer_rescan(stage, finding)
    {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn signal_connectivity_conflict_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_signal_connectivity_conflict_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn interface_signal_conflict_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_interface_signal_conflict_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn temporal_conflict_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_temporal_conflict_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn signal_polarity_conflict_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_signal_polarity_conflict_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn signal_semantic_conflict_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_signal_semantic_conflict_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn temporal_cycle_window_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_temporal_cycle_window_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn graph_direction_coverage_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_graph_direction_coverage_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn graph_direction_conflict_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_graph_direction_conflict_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn actor_port_gap_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_actor_port_gap_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn temporal_actor_grounding_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_temporal_actor_grounding_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn temporal_clock_grounding_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_temporal_clock_grounding_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn semantic_role_arbitration_replay_inputs(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
) -> Option<Vec<ProjectedReplayInput>> {
    if !is_semantic_role_arbitration_rescan(stage, finding) {
        return None;
    }

    evidence_nlp_replay_inputs_for_stage(stage, snapshot)
}

fn evidence_nlp_replay_inputs_for_stage(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
) -> Option<Vec<ProjectedReplayInput>> {
    let mut inputs = vec![ProjectedReplayInput {
        input_kind: "evidence_ir",
        path: evidence_input_for_snapshot_stage(stage, snapshot)?,
    }];
    if stage == IrStage::IntentIr {
        inputs.push(ProjectedReplayInput {
            input_kind: "semantic_ir",
            path: semantic_input_for_snapshot(snapshot)?,
        });
    }
    Some(inputs)
}

fn recommended_rescan_commands(
    stage: IrStage,
    artifact_path: &Path,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
    rescan_vlm_policy: &RescanVlmHintPolicy,
    repo_root: &Path,
) -> Vec<ProjectRescanCommandHint> {
    if is_source_vlm_enrichment_missing_rescan(stage, finding) {
        return source_local_enrich_validate_rescan_commands(
            artifact_path,
            rescan_vlm_policy,
            repo_root,
        );
    }

    if is_evidence_structural_kg_missing_rescan(stage, finding) {
        return evidence_local_nlp_validate_rescan_commands(
            artifact_path,
            rescan_vlm_policy,
            repo_root,
        );
    }

    if is_evidence_normative_residual_rescan(stage, finding) {
        return evidence_local_nlp_validate_rescan_commands(
            artifact_path,
            rescan_vlm_policy,
            repo_root,
        );
    }

    if is_evidence_signal_polarity_conflict_rescan(stage, finding) {
        return evidence_local_nlp_validate_rescan_commands(
            artifact_path,
            rescan_vlm_policy,
            repo_root,
        );
    }

    if is_evidence_signal_semantic_conflict_rescan(stage, finding) {
        return evidence_local_nlp_validate_rescan_commands(
            artifact_path,
            rescan_vlm_policy,
            repo_root,
        );
    }

    if is_negative_knowledge_rescan(stage, finding)
        && let Some(commands) =
            negative_knowledge_rescan_commands(stage, artifact_path, snapshot, repo_root)
    {
        return commands;
    }

    if is_evidence_nlp_rebuild_rescan(stage, finding)
        && let Some(commands) = evidence_nlp_rebuild_rescan_commands(
            stage,
            artifact_path,
            snapshot,
            rescan_vlm_policy,
            repo_root,
        )
    {
        return commands;
    }

    let mut commands = Vec::new();
    if (is_visual_motif_corroboration_rescan(stage, finding)
        || is_evidence_missing_vlm_observations_rescan(stage, finding))
        && let Some(input) = snapshot
            .replay_inputs
            .iter()
            .find(|input| input.input_kind == "source_ir")
    {
        let mut enrich_args = vec![
            "enrich".to_string(),
            repo_relative_display(&input.path, repo_root),
            "--vlm-provider".to_string(),
            rescan_vlm_provider_name(select_rescan_vlm_provider(
                rescan_vlm_policy.provider,
                doctor::local_vlm_default_model_present,
            ))
            .to_string(),
        ];
        if let Some(model) = rescan_vlm_policy.model.as_ref() {
            enrich_args.push("--vlm-model".to_string());
            enrich_args.push(model.clone());
        }
        commands.push(specforge_command_hint("enrich_source_ir", enrich_args));
    }

    if let Some(input) = snapshot.replay_inputs.first() {
        let input_path = repo_relative_display(&input.path, repo_root);
        let rebuild_intent = match stage {
            IrStage::SourceIr => "rebuild_source_ir",
            IrStage::EvidenceIr => "rebuild_evidence_ir",
            IrStage::SemanticIr => "rebuild_semantic_ir",
            IrStage::IntentIr => "rebuild_intent_ir",
        };
        let rebuild_command = match stage {
            IrStage::SourceIr => Some(vec!["ingest".to_string(), input_path]),
            IrStage::EvidenceIr => Some(vec!["evidence".to_string(), input_path]),
            IrStage::SemanticIr => Some(vec!["semantic".to_string(), input_path]),
            IrStage::IntentIr => Some(vec!["intent".to_string(), input_path]),
        };
        if let Some(args) = rebuild_command {
            commands.push(specforge_command_hint(rebuild_intent, args));
        }
    }

    commands.push(specforge_command_hint(
        "validate_current_artifact",
        vec![
            "validate".to_string(),
            repo_relative_display(artifact_path, repo_root),
        ],
    ));
    commands
}

fn negative_knowledge_rescan_commands(
    stage: IrStage,
    artifact_path: &Path,
    snapshot: &ProjectedArtifactSnapshot,
    repo_root: &Path,
) -> Option<Vec<ProjectRescanCommandHint>> {
    let evidence_ir_path = evidence_input_for_snapshot_stage(stage, snapshot)?;
    let source_ir_path = source_ir_input_for_negative_knowledge_rescan(stage, snapshot)?;
    let mut commands = vec![specforge_command_hint(
        "rebuild_evidence_ir",
        vec![
            "evidence".to_string(),
            repo_relative_display(&source_ir_path, repo_root),
        ],
    )];
    if stage != IrStage::EvidenceIr {
        commands.push(specforge_command_hint(
            "rebuild_semantic_ir",
            vec![
                "semantic".to_string(),
                repo_relative_display(&evidence_ir_path, repo_root),
            ],
        ));
    }

    if stage == IrStage::IntentIr {
        let semantic_ir_path = semantic_input_for_snapshot(snapshot)?;
        commands.push(specforge_command_hint(
            "rebuild_intent_ir",
            vec![
                "intent".to_string(),
                repo_relative_display(&semantic_ir_path, repo_root),
            ],
        ));
    }

    commands.push(specforge_command_hint(
        "validate_current_artifact",
        vec![
            "validate".to_string(),
            repo_relative_display(artifact_path, repo_root),
        ],
    ));
    Some(commands)
}

fn evidence_local_nlp_validate_rescan_commands(
    artifact_path: &Path,
    rescan_vlm_policy: &RescanVlmHintPolicy,
    repo_root: &Path,
) -> Vec<ProjectRescanCommandHint> {
    vec![
        nlp_enrich_evidence_command(artifact_path, rescan_vlm_policy, repo_root),
        specforge_command_hint(
            "validate_current_artifact",
            vec![
                "validate".to_string(),
                repo_relative_display(artifact_path, repo_root),
            ],
        ),
    ]
}

fn source_local_enrich_validate_rescan_commands(
    artifact_path: &Path,
    rescan_vlm_policy: &RescanVlmHintPolicy,
    repo_root: &Path,
) -> Vec<ProjectRescanCommandHint> {
    let mut args = vec![
        "enrich".to_string(),
        repo_relative_display(artifact_path, repo_root),
        "--vlm-provider".to_string(),
        rescan_vlm_provider_name(select_rescan_vlm_provider(
            rescan_vlm_policy.provider,
            doctor::local_vlm_default_model_present,
        ))
        .to_string(),
    ];
    if let Some(model) = rescan_vlm_policy.model.as_ref() {
        args.push("--vlm-model".to_string());
        args.push(model.clone());
    }
    vec![
        specforge_command_hint("enrich_source_ir", args),
        specforge_command_hint(
            "validate_current_artifact",
            vec![
                "validate".to_string(),
                repo_relative_display(artifact_path, repo_root),
            ],
        ),
    ]
}

fn evidence_nlp_rebuild_rescan_commands(
    stage: IrStage,
    artifact_path: &Path,
    snapshot: &ProjectedArtifactSnapshot,
    rescan_vlm_policy: &RescanVlmHintPolicy,
    repo_root: &Path,
) -> Option<Vec<ProjectRescanCommandHint>> {
    let evidence_ir_path = evidence_input_for_snapshot_stage(stage, snapshot)?;
    let evidence_ir_display = repo_relative_display(&evidence_ir_path, repo_root);
    let mut commands = vec![nlp_enrich_evidence_command(
        &evidence_ir_path,
        rescan_vlm_policy,
        repo_root,
    )];
    commands.push(specforge_command_hint(
        "rebuild_semantic_ir",
        vec!["semantic".to_string(), evidence_ir_display],
    ));

    if stage == IrStage::IntentIr {
        let semantic_ir_path = semantic_input_for_snapshot(snapshot)?;
        commands.push(specforge_command_hint(
            "rebuild_intent_ir",
            vec![
                "intent".to_string(),
                repo_relative_display(&semantic_ir_path, repo_root),
            ],
        ));
    }

    commands.push(specforge_command_hint(
        "validate_current_artifact",
        vec![
            "validate".to_string(),
            repo_relative_display(artifact_path, repo_root),
        ],
    ));
    Some(commands)
}

fn evidence_input_for_snapshot_stage(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
) -> Option<PathBuf> {
    match stage {
        IrStage::SourceIr => None,
        IrStage::EvidenceIr => Some(snapshot.artifact_path.clone()),
        IrStage::SemanticIr => snapshot
            .replay_inputs
            .iter()
            .find(|input| input.input_kind == "evidence_ir")
            .map(|input| input.path.clone()),
        IrStage::IntentIr => {
            let semantic_ir_path = semantic_input_for_snapshot(snapshot)?;
            SemanticIr::load_from_path(&semantic_ir_path)
                .ok()
                .map(|semantic_ir| semantic_ir.evidence_ir_path)
        }
    }
}

fn source_ir_input_for_negative_knowledge_rescan(
    stage: IrStage,
    snapshot: &ProjectedArtifactSnapshot,
) -> Option<PathBuf> {
    let evidence_ir_path = evidence_input_for_snapshot_stage(stage, snapshot)?;
    EvidenceIr::load_from_path(&evidence_ir_path)
        .ok()
        .map(|evidence_ir| evidence_ir.source_ir_path)
}

fn semantic_input_for_snapshot(snapshot: &ProjectedArtifactSnapshot) -> Option<PathBuf> {
    snapshot
        .replay_inputs
        .iter()
        .find(|input| input.input_kind == "semantic_ir")
        .map(|input| input.path.clone())
}

fn nlp_enrich_evidence_command(
    evidence_ir_path: &Path,
    rescan_vlm_policy: &RescanVlmHintPolicy,
    repo_root: &Path,
) -> ProjectRescanCommandHint {
    let mut args = vec![
        "nlp-enrich".to_string(),
        repo_relative_display(evidence_ir_path, repo_root),
        "--vlm-provider".to_string(),
        rescan_vlm_provider_name(select_rescan_vlm_provider(
            rescan_vlm_policy.provider,
            doctor::local_vlm_default_model_present,
        ))
        .to_string(),
    ];
    if let Some(model) = rescan_vlm_policy.model.as_ref() {
        args.push("--vlm-model".to_string());
        args.push(model.clone());
    }
    specforge_command_hint("nlp_enrich_evidence_ir", args)
}

fn is_visual_motif_corroboration_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    stage == IrStage::EvidenceIr
        && finding.finding_id == EVIDENCE_VISUAL_MOTIF_CORROBORATION_GUIDANCE
}

fn is_source_vlm_enrichment_missing_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    stage == IrStage::SourceIr
        && finding.finding_id == SOURCE_VLM_ENRICHMENT_MISSING_SURFACE_RESCAN_GUIDANCE
}

fn is_evidence_missing_vlm_observations_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    stage == IrStage::EvidenceIr
        && finding.finding_id == EVIDENCE_MISSING_VLM_OBSERVATIONS_SURFACE_RESCAN_GUIDANCE
}

fn is_evidence_signal_semantic_conflict_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    stage == IrStage::EvidenceIr
        && finding.finding_id == EVIDENCE_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE
}

fn is_evidence_structural_kg_missing_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    stage == IrStage::EvidenceIr
        && finding.finding_id == EVIDENCE_STRUCTURAL_KG_MISSING_SURFACE_RESCAN_GUIDANCE
}

fn is_evidence_normative_residual_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    stage == IrStage::EvidenceIr
        && finding.finding_id == EVIDENCE_NORMATIVE_RESIDUAL_SURFACE_RESCAN_GUIDANCE
}

fn is_evidence_signal_polarity_conflict_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    stage == IrStage::EvidenceIr
        && finding.finding_id == EVIDENCE_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
}

fn is_negative_knowledge_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::EvidenceIr,
            EVIDENCE_NEGATIVE_KNOWLEDGE_RESCAN_GUIDANCE
        ) | (
            IrStage::SemanticIr,
            SEMANTIC_NEGATIVE_KNOWLEDGE_RESCAN_GUIDANCE
        ) | (IrStage::IntentIr, INTENT_NEGATIVE_KNOWLEDGE_RESCAN_GUIDANCE)
    )
}

fn is_temporal_rule_surface_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_TEMPORAL_RULE_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_TEMPORAL_RULE_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_actor_port_gap_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_semantic_role_arbitration_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_semantic_role_consensus_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_alias_dependent_semantic_consensus_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_prior_guided_semantic_consensus_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_connectivity_missing_producer_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_connectivity_missing_consumer_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_signal_connectivity_conflict_rescan(
    stage: IrStage,
    finding: &ValidationFindingRecord,
) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_interface_signal_conflict_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_temporal_conflict_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_signal_polarity_conflict_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_signal_semantic_conflict_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_temporal_cycle_window_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_temporal_clock_grounding_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_temporal_actor_grounding_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_graph_direction_coverage_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_graph_direction_conflict_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    matches!(
        (stage, finding.finding_id.as_str()),
        (
            IrStage::SemanticIr,
            SEMANTIC_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
        ) | (
            IrStage::IntentIr,
            INTENT_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
        )
    )
}

fn is_evidence_nlp_rebuild_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    is_temporal_rule_surface_rescan(stage, finding)
        || is_actor_port_gap_rescan(stage, finding)
        || is_semantic_role_arbitration_rescan(stage, finding)
        || is_semantic_role_consensus_rescan(stage, finding)
        || is_alias_dependent_semantic_consensus_rescan(stage, finding)
        || is_prior_guided_semantic_consensus_rescan(stage, finding)
        || is_connectivity_missing_producer_rescan(stage, finding)
        || is_connectivity_missing_consumer_rescan(stage, finding)
        || is_interface_signal_conflict_rescan(stage, finding)
        || is_temporal_conflict_rescan(stage, finding)
        || is_signal_polarity_conflict_rescan(stage, finding)
        || is_signal_semantic_conflict_rescan(stage, finding)
        || is_signal_connectivity_conflict_rescan(stage, finding)
        || is_graph_direction_conflict_rescan(stage, finding)
        || is_graph_direction_coverage_rescan(stage, finding)
        || is_temporal_actor_grounding_rescan(stage, finding)
        || is_temporal_clock_grounding_rescan(stage, finding)
        || is_temporal_cycle_window_rescan(stage, finding)
}

fn select_rescan_vlm_provider(
    policy: RescanVlmProviderArg,
    mut default_model_present: impl FnMut(VlmProviderArg) -> bool,
) -> VlmProviderArg {
    match policy {
        RescanVlmProviderArg::AutoLocal => {
            if default_model_present(VlmProviderArg::Ollama) {
                VlmProviderArg::Ollama
            } else if default_model_present(VlmProviderArg::LmStudio) {
                VlmProviderArg::LmStudio
            } else {
                VlmProviderArg::Ollama
            }
        }
        RescanVlmProviderArg::Ollama => VlmProviderArg::Ollama,
        RescanVlmProviderArg::LmStudio => VlmProviderArg::LmStudio,
        RescanVlmProviderArg::Skip => VlmProviderArg::Skip,
    }
}

fn rescan_vlm_provider_name(provider: VlmProviderArg) -> &'static str {
    match provider {
        VlmProviderArg::Ollama => "ollama",
        VlmProviderArg::LmStudio => "lmstudio",
        VlmProviderArg::Skip => "skip",
        VlmProviderArg::OpenAi => unreachable!("rescan VLM hints are local-only"),
    }
}

fn specforge_command_hint(intent: &str, specforge_args: Vec<String>) -> ProjectRescanCommandHint {
    let mut args = vec![
        "run".to_string(),
        "--manifest-path".to_string(),
        "Cargo.toml".to_string(),
        "--".to_string(),
    ];
    args.extend(specforge_args);
    let display = std::iter::once(shell_quote("cargo"))
        .chain(args.iter().map(|arg| shell_quote(arg)))
        .collect::<Vec<_>>()
        .join(" ");

    ProjectRescanCommandHint {
        intent: intent.to_string(),
        executable: "cargo".to_string(),
        args,
        working_directory: ".".to_string(),
        display,
    }
}

fn shell_quote(value: &str) -> String {
    if value.chars().all(|character| {
        character.is_ascii_alphanumeric() || matches!(character, '_' | '-' | '.' | '/' | ':' | '=')
    }) {
        return value.to_string();
    }

    format!("'{}'", value.replace('\'', "'\"'\"'"))
}

fn is_rescan_guidance(finding: &ValidationFindingRecord) -> bool {
    finding.category == "rescan_guidance" && !finding.related_ids.is_empty()
}

fn extractor_lane_for_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> &'static str {
    if is_source_vlm_enrichment_missing_rescan(stage, finding) {
        return "source_ir_visual_enrichment_rescan";
    }

    match stage {
        IrStage::SourceIr => "source_ir_normalization_rescan",
        IrStage::EvidenceIr => "evidence_ir_multimodal_semantic_corroboration",
        IrStage::SemanticIr => "semantic_ir_conflict_corroboration",
        IrStage::IntentIr => "intent_ir_canonical_surface_corroboration",
    }
}

fn recommended_rescan_action(stage: IrStage, finding: &ValidationFindingRecord) -> &'static str {
    if is_source_vlm_enrichment_missing_rescan(stage, finding) {
        return "run local visual enrichment on SourceIR and validate whether the related asset ids gain VLM enrichment instead of remaining timing/state diagrams without extracted observations";
    }

    if is_negative_knowledge_rescan(stage, finding) {
        return match stage {
            IrStage::EvidenceIr => {
                "restart from SourceIR through EvidenceIR, then validate whether the related evidence conflict or residual ids still reproduce from current-document evidence"
            }
            IrStage::SemanticIr => {
                "restart from SourceIR through EvidenceIR and SemanticIR, then validate whether the related conflict or residual ids still reproduce from current-document evidence"
            }
            IrStage::IntentIr => {
                "restart from SourceIR through EvidenceIR, SemanticIR, and IntentIR, then validate whether the related canonical conflict or residual ids still reproduce from current-document evidence"
            }
            IrStage::SourceIr => unreachable!(
                "negative-knowledge specialized action only applies to evidence/semantic/intent rescans"
            ),
        };
    }

    if is_temporal_rule_surface_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal source ids now lower into typed temporal rules"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal source ids survive as typed temporal rules without blind promotion"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "temporal-rule-surface specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_actor_port_gap_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related actor-signal relation ids now collapse into actor-relative port direction records"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related actor-signal relation ids survive as actor-relative port direction records instead of relation-only graph evidence"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "actor-port-gap specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_semantic_role_arbitration_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids now converge toward a decisive semantic-role outcome"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids now survive with a decisive semantic-role outcome instead of contested arbitration"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "semantic-role-arbitration specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_semantic_role_consensus_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain observation-backed semantic-role consensus"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with observation-backed semantic-role consensus instead of fallback-only role meaning"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "semantic-role-consensus specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_alias_dependent_semantic_consensus_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain direct or corroborating non-alias semantic-role consensus"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with direct or corroborating non-alias semantic-role consensus instead of alias-dependent carry-through"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "alias-dependent semantic-consensus specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_prior_guided_semantic_consensus_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain stronger current-document semantic-role consensus without depending on learned modality-reliability priors"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with stronger current-document semantic-role consensus instead of prior-guided carry-through"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "prior-guided semantic-consensus specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_connectivity_missing_producer_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain producer-side connectivity evidence"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with producer-side connectivity evidence instead of remaining producerless"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "connectivity-producer specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_connectivity_missing_consumer_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain consumer-side connectivity evidence"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with consumer-side connectivity evidence instead of remaining consumerless"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "connectivity-consumer specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_interface_signal_conflict_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related interface conflict ids collapse toward consistent direction/width declarations"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related interface conflict ids survive with consistent direction/width declarations instead of unresolved interface-shape ambiguity"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "interface-signal-conflict specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_temporal_conflict_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal conflict ids collapse toward a single locally corroborated timing obligation"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal conflict ids survive with a single locally corroborated timing obligation instead of contradictory value obligations"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "temporal-conflict specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_signal_polarity_conflict_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related polarity conflict ids collapse toward a single locally corroborated active-level interpretation"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related polarity conflict ids survive with a single locally corroborated active-level interpretation instead of unresolved polarity disagreement"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "signal-polarity-conflict specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_signal_semantic_conflict_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related semantic conflict ids collapse toward a single locally corroborated role meaning"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related semantic conflict ids survive with a single locally corroborated role meaning instead of unresolved semantic-role disagreement"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "signal-semantic-conflict specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_signal_connectivity_conflict_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related connectivity conflict ids collapse toward single-producer connectivity"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related connectivity conflict ids survive with single-producer connectivity instead of unresolved producer ambiguity"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "signal-connectivity-conflict specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_graph_direction_conflict_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related graph-direction conflict ids collapse toward one actor-relative direction per actor-signal edge"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related graph-direction conflict ids survive with one actor-relative direction per actor-signal edge instead of unresolved same-actor direction disagreement"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "graph-direction-conflict specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_temporal_cycle_window_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal rule ids gain explicit cycle-window bounds"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal rule ids survive with explicit cycle-window bounds instead of remaining unbounded"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "temporal-cycle-window specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_graph_direction_coverage_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain actor-relative graph direction coverage"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with actor-relative graph direction coverage instead of remaining graph-uncovered"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "graph-direction-coverage specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_temporal_actor_grounding_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal rule ids gain actor-relative drive/sample grounding"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal rule ids survive with actor-relative drive/sample grounding instead of remaining actor-ungrounded"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "temporal-actor-grounding specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_temporal_clock_grounding_rescan(stage, finding) {
        return match stage {
            IrStage::SemanticIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal rule ids gain explicit clock or edge grounding"
            }
            IrStage::IntentIr => {
                "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal rule ids survive with explicit clock or edge grounding instead of remaining clockless"
            }
            IrStage::SourceIr | IrStage::EvidenceIr => unreachable!(
                "temporal-clock-grounding specialized action only applies to semantic/intent rescans"
            ),
        };
    }

    if is_evidence_missing_vlm_observations_rescan(stage, finding) {
        return "rerun local visual enrichment from SourceIR, rebuild EvidenceIR, and validate whether the related visual ids gain timing/state observations instead of remaining VLM-unenriched";
    }

    if is_visual_motif_corroboration_rescan(stage, finding) {
        return "rerun local visual enrichment from SourceIR, rebuild EvidenceIR, and validate whether the related visual ids gain corroborated typed evidence";
    }

    if is_evidence_structural_kg_missing_rescan(stage, finding) {
        return "run local NLP enrichment on EvidenceIR and validate whether the related behavioral evidence ids collapse into actor-grounded graph relations instead of remaining structurally ungrounded";
    }

    if is_evidence_normative_residual_rescan(stage, finding) {
        return "run local NLP enrichment on EvidenceIR and validate whether the related normative residual statement ids collapse into typed constraints, rules, or structured evidence instead of remaining only partially structured";
    }

    if is_evidence_signal_polarity_conflict_rescan(stage, finding) {
        return "run local NLP enrichment on EvidenceIR and validate whether the related polarity conflict ids collapse toward a single locally corroborated active-level interpretation";
    }

    if is_evidence_signal_semantic_conflict_rescan(stage, finding) {
        return "run local NLP enrichment on EvidenceIR and validate whether the related semantic conflict ids collapse toward a single locally corroborated role meaning";
    }

    match stage {
        IrStage::SourceIr => {
            "reinspect source normalization around the related ids before downstream promotion"
        }
        IrStage::EvidenceIr => {
            "rescan supporting statements, tables, and visual evidence for the related evidence-stage conflict ids"
        }
        IrStage::SemanticIr => {
            "rebuild SemanticIR after targeted evidence rescans and require independent local support before resolving the related conflict or residual ids"
        }
        IrStage::IntentIr => {
            "rebuild IntentIR after targeted semantic/evidence rescans and keep the related canonical ids explicit until corroborated"
        }
    }
}

fn render_inline_code_list(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values
            .iter()
            .map(|value| format!("`{value}`"))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn render_replay_input_list(values: &[ProjectRescanReplayInput]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values
            .iter()
            .map(|value| format!("`{}:{}`", value.input_kind, value.path))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn render_replay_input_kind_chain(values: &[ProjectRescanReplayInput]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values
            .iter()
            .map(|value| value.input_kind.as_str())
            .collect::<Vec<_>>()
            .join(" -> ")
    }
}

fn truncate_projection_text(value: &str, max_chars: usize) -> String {
    let char_count = value.chars().count();
    if char_count <= max_chars {
        return value.to_string();
    }

    let truncated = value
        .chars()
        .take(max_chars.saturating_sub(3))
        .collect::<String>();
    format!("{truncated}...")
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
struct RescanExecutionCounts {
    total: usize,
    review_required: usize,
    possible_improvement: usize,
    regression: usize,
    neutral_change: usize,
    no_change: usize,
}

fn rescan_execution_counts(
    recommendations: &[ProjectRescanRecommendation],
) -> RescanExecutionCounts {
    let mut counts = RescanExecutionCounts::default();
    for summary in recommendations
        .iter()
        .filter_map(|recommendation| recommendation.execution_summary.as_ref())
    {
        counts.total += 1;
        if summary.arbitration_verdict.ends_with("_review_required") {
            counts.review_required += 1;
        }
        match summary.arbitration_verdict.as_str() {
            "possible_improvement_review_required" => counts.possible_improvement += 1,
            "regression_review_required" => counts.regression += 1,
            "neutral_change_review_required" => counts.neutral_change += 1,
            "validated_no_change" => counts.no_change += 1,
            _ => {}
        }
    }
    counts
}

fn render_rescan_execution_counts(counts: &RescanExecutionCounts) -> String {
    if counts.total == 0 {
        "0".to_string()
    } else {
        format!(
            "{} total; {} review required (possible improvement: {}, regression: {}, neutral change: {}); {} no-change",
            counts.total,
            counts.review_required,
            counts.possible_improvement,
            counts.regression,
            counts.neutral_change,
            counts.no_change
        )
    }
}

fn render_execution_summary_inline(summary: Option<&ProjectRescanExecutionSummary>) -> String {
    let Some(summary) = summary else {
        return String::new();
    };
    format!(
        ", verdict `{}`, promotion `{}`, review `{}`, score_delta `{}`, finding_count_delta `{}`",
        summary.arbitration_verdict,
        summary_promotion_status(summary),
        summary_promotion_review(summary).review_status,
        render_signed_option_i32(summary.validation_delta.score_delta),
        render_signed_i64(summary.validation_delta.finding_count_delta)
    )
}

fn summary_promotion_status(summary: &ProjectRescanExecutionSummary) -> &str {
    let expected_status = rescan_promotion_status_for(&summary.arbitration_verdict);
    if summary.promotion_status == expected_status {
        summary.promotion_status.as_str()
    } else {
        expected_status
    }
}

fn summary_promotion_blockers(summary: &ProjectRescanExecutionSummary) -> Vec<String> {
    let mut blockers = summary.promotion_blockers.clone();
    for blocker in rescan_promotion_blockers_for(&summary.arbitration_verdict) {
        if !blockers.contains(&blocker) {
            blockers.push(blocker);
        }
    }
    blockers
}

fn summary_promotion_review(
    summary: &ProjectRescanExecutionSummary,
) -> ProjectRescanPromotionReview {
    let mut review = summary.promotion_review.clone();
    let expected = rescan_promotion_review_for(&summary.arbitration_verdict);
    if review.review_status.is_empty() {
        review.review_status = expected.review_status;
    }
    if review.approval_policy.is_empty() {
        review.approval_policy = expected.approval_policy;
    }
    if review.required_decisions.is_empty() {
        review.required_decisions = expected.required_decisions;
    }
    review.approval_record_required = expected.approval_record_required;
    review.canonical_mutation_allowed = false;
    review
}

fn render_signed_option_i32(value: Option<i32>) -> String {
    value
        .map(render_signed_i32)
        .unwrap_or_else(|| "n/a".to_string())
}

fn render_signed_i32(value: i32) -> String {
    if value > 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

fn render_signed_i64(value: i64) -> String {
    if value > 0 {
        format!("+{value}")
    } else {
        value.to_string()
    }
}

fn replace_or_append_managed_section(
    input: &str,
    start_marker: &str,
    end_marker: &str,
    managed_block: &str,
) -> Result<String> {
    match (input.find(start_marker), input.find(end_marker)) {
        (Some(start), Some(end)) if end > start => {
            let before = &input[..start];
            let after = &input[end + end_marker.len()..];
            Ok(format!(
                "{}{}{}",
                before.trim_end(),
                if before.trim_end().is_empty() {
                    ""
                } else {
                    "\n\n"
                },
                managed_block.to_string() + after
            ))
        }
        (None, None) => {
            let mut output = input.trim_end().to_string();
            if !output.is_empty() {
                output.push_str("\n\n");
            }
            output.push_str("## Validation Projection\n");
            output.push_str(managed_block);
            output.push('\n');
            Ok(output)
        }
        _ => Err(AppError::InvalidBackendOutput(
            "malformed validation projection markers in LIVE_ACHIEVEMENT_STATUS.md".to_string(),
        )),
    }
}

fn sorted_findings(findings: &[ValidationFindingRecord]) -> Vec<&ValidationFindingRecord> {
    let mut sorted: Vec<_> = findings.iter().collect();
    sorted.sort_by_key(|finding| {
        (
            Reverse(severity_rank(finding.severity)),
            finding.category.clone(),
            finding.finding_id.clone(),
        )
    });
    sorted
}

fn primary_finding_summary(report: &ValidationReportRecord) -> String {
    sorted_findings(&report.findings)
        .into_iter()
        .next()
        .map(|finding| {
            format!(
                "[{}:{}] {}",
                finding.severity.as_str(),
                finding.category,
                finding.summary
            )
        })
        .unwrap_or_else(|| "none".to_string())
}

fn score_summary(report: &ValidationReportRecord) -> String {
    match (report.overall_score, report.grade.as_deref()) {
        (Some(score), Some(grade)) => format!("{score}/100 {grade}"),
        (Some(score), None) => format!("{score}/100"),
        (None, Some(grade)) => grade.to_string(),
        (None, None) => report.summary.clone(),
    }
}

fn highest_severity_label(snapshots: &[ProjectedArtifactSnapshot]) -> &'static str {
    let rank = snapshots
        .iter()
        .flat_map(|snapshot| snapshot.report.findings.iter())
        .map(|finding| severity_rank(finding.severity))
        .max()
        .unwrap_or(0);
    match rank {
        3 => "error",
        2 => "warning",
        1 => "info",
        _ => "none",
    }
}

fn severity_rank(severity: ValidationFindingSeverity) -> u8 {
    match severity {
        ValidationFindingSeverity::Info => 1,
        ValidationFindingSeverity::Warning => 2,
        ValidationFindingSeverity::Error => 3,
    }
}

fn normalize_input_path(path: &Path, repo_root: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        repo_root.join(path)
    }
}

fn repo_relative_display(path: &Path, repo_root: &Path) -> String {
    let absolute = normalize_input_path(path, repo_root);
    absolute
        .strip_prefix(repo_root)
        .unwrap_or(absolute.as_path())
        .display()
        .to_string()
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;
    use crate::ir::evidence::EvidenceIr;
    use crate::ir::intent::IntentIr;
    use crate::ir::semantic::SemanticIr;
    use crate::ir::source::SourceIr;

    #[test]
    fn project_validation_writes_snapshot_doc_and_updates_live_status() -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = tempdir.path();
        fs::write(
            repo_root.join(LIVE_STATUS_DOC),
            "# LIVE_ACHIEVEMENT_STATUS\n## Current snapshot\n- placeholder: Done\n",
        )?;

        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        let intent_artifact_base = repo_root.join("generated").join("intent_ir");
        fs::write(
            &source,
            "# Spec\nSignal DATA_IN is input width 8.\n\nSignal DATA_OUT is output width 8.\n\nBlock pass: DATA_OUT = DATA_IN.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        intent_ir.write_to_disk()?;

        run(ProjectValidationArgs {
            artifacts: vec![intent_ir.artifact_layout.intent_ir_path.clone()],
            repo_root: repo_root.to_path_buf(),
            rescan_vlm_provider: RescanVlmProviderArg::Ollama,
            rescan_vlm_model: None,
        })?;

        let snapshot_doc = fs::read_to_string(repo_root.join(VALIDATION_SNAPSHOT_DOC))?;
        assert!(snapshot_doc.contains("spec.md"));
        assert!(snapshot_doc.contains("intent_ir"));
        assert!(snapshot_doc.contains("35/100 NEEDS IMPROVEMENT"));
        assert!(snapshot_doc.contains("- Targeted rescan recommendations: 0"));

        let live_status = fs::read_to_string(repo_root.join(LIVE_STATUS_DOC))?;
        assert!(live_status.contains("## Validation Projection"));
        assert!(live_status.contains(VALIDATION_PROJECTION_START));
        assert!(live_status.contains("35/100 NEEDS IMPROVEMENT"));
        assert!(live_status.contains("- Targeted rescan queue:\n  - none"));

        let rescan_plan = fs::read_to_string(repo_root.join(VALIDATION_RESCAN_PLAN_PATH))?;
        assert!(rescan_plan.contains("\"schema_version\": 2"));
        assert!(rescan_plan.contains("\"recommendation_count\": 0"));

        let reloaded = IntentIr::load_from_path(&intent_ir.artifact_layout.intent_ir_path)?;
        assert_eq!(reloaded.validation_reports.len(), 1);

        Ok(())
    }

    #[test]
    fn project_validation_collects_negative_knowledge_rescan_guidance() -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "spec".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/spec/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with rescan guidance".to_string(),
                overall_score: Some(85),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: "intent_negative_knowledge_rescan_guidance".to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "known failure shape needs rescan".to_string(),
                    related_ids: vec![
                        "temporal_conflict_0002".to_string(),
                        "temporal_conflict_0001".to_string(),
                        "temporal_conflict_0001".to_string(),
                    ],
                }],
            },
        };
        let snapshots = vec![snapshot];

        let mut recommendations =
            collect_rescan_recommendations(&snapshots, &repo_root, &test_rescan_vlm_policy());
        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].related_ids,
            vec![
                "temporal_conflict_0001".to_string(),
                "temporal_conflict_0002".to_string()
            ]
        );
        assert_eq!(
            recommendations[0].extractor_lane,
            "intent_ir_canonical_surface_corroboration"
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "restart from SourceIR through EvidenceIR, SemanticIR, and IntentIR, then validate whether the related canonical conflict or residual ids still reproduce from current-document evidence"
        );
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "source_ir".to_string(),
                    path: "generated/source_ir/spec/source_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                }
            ]
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "rebuild_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "evidence".to_string(),
                "generated/source_ir/spec/source_ir.json".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "semantic".to_string(),
                "generated/evidence_ir/spec/evidence_ir.json".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "intent".to_string(),
                "generated/semantic_ir/spec/semantic_ir.json".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );
        assert_eq!(recommendations[0].automation_status, "planned_not_executed");
        recommendations[0].automation_status = "executed_validated_changed".to_string();
        recommendations[0].execution_summary = Some(execution_summary(
            "possible_improvement_review_required",
            Some(5),
            -1,
        ));

        let snapshot_doc = render_validation_snapshot_doc(&snapshots, &repo_root, &recommendations);
        assert!(snapshot_doc.contains("## Targeted Rescan Recommendations"));
        assert!(snapshot_doc.contains("intent_ir_canonical_surface_corroboration"));
        assert!(snapshot_doc.contains("temporal_conflict_0001"));
        assert!(snapshot_doc.contains("recommended_commands"));
        assert!(snapshot_doc.contains("generated/source_ir/spec/source_ir.json"));
        assert!(snapshot_doc.contains("generated/evidence_ir/spec/evidence_ir.json"));
        assert!(snapshot_doc.contains("generated/semantic_ir/spec/semantic_ir.json"));
        assert!(snapshot_doc.contains(
            "- Rescan execution summaries: 1 total; 1 review required (possible improvement: 1, regression: 0, neutral change: 0); 0 no-change"
        ));
        assert!(snapshot_doc.contains(
            "- execution_summary: `possible_improvement_review_required` (`executed_validated_changed`)"
        ));
        assert!(snapshot_doc.contains(
            "- promotion_gate: `not_promoted_review_required` (blockers: `canonical_ir_not_mutated_by_rescan_plan`, `validation_delta_is_not_truth_promotion`, `current_document_evidence_review_required`)"
        ));
        assert!(snapshot_doc.contains(
            "- promotion_review: `human_review_required` (policy `current_document_evidence_review_before_canonical_mutation`, approval_record_required `true`, canonical_mutation_allowed `false`"
        ));
        assert!(snapshot_doc.contains(
            "- validation_delta: fingerprint_changed `true`, score_delta `+5`, grade_changed `true`, finding_count_delta `-1`"
        ));
        assert!(snapshot_doc.contains("- finding_delta: added none; removed `finding_b`"));

        let live_projection =
            render_live_status_projection(&snapshots, &repo_root, &recommendations);
        assert!(live_projection.contains("- Targeted rescan queue:"));
        assert!(live_projection.contains("intent_ir_canonical_surface_corroboration"));
        assert!(live_projection.contains("replay `source_ir -> evidence_ir -> semantic_ir`"));
        assert!(live_projection.contains(
            "action: restart from SourceIR through EvidenceIR, SemanticIR, and IntentIR"
        ));
        assert!(live_projection.contains("4 command hint(s)"));
        assert!(live_projection.contains(
            "- Rescan execution summaries: 1 total; 1 review required (possible improvement: 1, regression: 0, neutral change: 0); 0 no-change"
        ));
        assert!(live_projection.contains(
            "verdict `possible_improvement_review_required`, promotion `not_promoted_review_required`, review `human_review_required`, score_delta `+5`, finding_count_delta `-1`"
        ));

        let plan_path = write_validation_rescan_plan(&repo_root, recommendations)?;
        let plan = fs::read_to_string(plan_path)?;
        assert!(plan.contains("\"schema_version\": 2"));
        assert!(plan.contains("\"recommendation_count\": 1"));
        assert!(plan.contains("\"corroboration_policy\""));
        assert!(plan.contains("\"replay_inputs\""));
        assert!(plan.contains("\"recommended_commands\""));
        assert!(plan.contains("\"rebuild_evidence_ir\""));
        assert!(plan.contains("\"rebuild_semantic_ir\""));
        assert!(plan.contains("\"rebuild_intent_ir\""));

        Ok(())
    }

    #[test]
    fn project_validation_collects_evidence_negative_knowledge_rescan_guidance() -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "spec".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::EvidenceIr,
            artifact_path: evidence_ir.artifact_layout.evidence_ir_path.clone(),
            replay_inputs: Vec::new(),
            report: ValidationReportRecord {
                report_id: "validation_evidence_ir_test".to_string(),
                validated_stage: IrStage::EvidenceIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "EvidenceIR validation with rescan guidance".to_string(),
                overall_score: Some(85),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: "evidence_negative_knowledge_rescan_guidance".to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "known failure shape needs evidence replay".to_string(),
                    related_ids: vec![
                        "semantic_conflict_0002".to_string(),
                        "semantic_conflict_0001".to_string(),
                        "semantic_conflict_0001".to_string(),
                    ],
                }],
            },
        };
        let snapshots = vec![snapshot];

        let recommendations =
            collect_rescan_recommendations(&snapshots, &repo_root, &test_rescan_vlm_policy());
        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].related_ids,
            vec![
                "semantic_conflict_0001".to_string(),
                "semantic_conflict_0002".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].extractor_lane,
            "evidence_ir_multimodal_semantic_corroboration"
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "restart from SourceIR through EvidenceIR, then validate whether the related evidence conflict or residual ids still reproduce from current-document evidence"
        );
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "source_ir".to_string(),
                    path: "generated/source_ir/spec/source_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 2);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "rebuild_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "evidence".to_string(),
                "generated/source_ir/spec/source_ir.json".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "validate_current_artifact"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "validate".to_string(),
                "generated/evidence_ir/spec/evidence_ir.json".to_string(),
            ]
        );
        assert_eq!(recommendations[0].automation_status, "planned_not_executed");

        Ok(())
    }

    #[test]
    fn project_validation_collects_visual_motif_rescan_guidance() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let snapshot = visual_motif_snapshot(repo_root);

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].extractor_lane,
            "evidence_ir_multimodal_semantic_corroboration"
        );
        assert_eq!(
            recommendations[0].corroboration_policy,
            "stronger_local_corroboration_required_before_canonical_promotion"
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "rerun local visual enrichment from SourceIR, rebuild EvidenceIR, and validate whether the related visual ids gain corroborated typed evidence"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "enrich_source_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "enrich".to_string(),
                "generated/source_ir/doc/source_ir.json".to_string(),
                "--vlm-provider".to_string(),
                "ollama".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
        assert_eq!(
            recommendations[0].related_ids,
            vec!["visual_0001".to_string()]
        );
    }

    #[test]
    fn project_validation_collects_evidence_signal_semantic_conflict_rescan_guidance() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::EvidenceIr,
            artifact_path: repo_root.join("generated/evidence_ir/doc/evidence_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "source_ir",
                path: repo_root.join("generated/source_ir/doc/source_ir.json"),
            }],
            report: ValidationReportRecord {
                report_id: "validation_evidence_ir_test".to_string(),
                validated_stage: IrStage::EvidenceIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "EvidenceIR validation with signal-semantic-conflict rescan guidance"
                    .to_string(),
                overall_score: None,
                grade: None,
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: EVIDENCE_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "EvidenceIR still carries unresolved semantic-role conflict ids"
                        .to_string(),
                    related_ids: vec!["semantic_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].extractor_lane,
            "evidence_ir_multimodal_semantic_corroboration"
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR and validate whether the related semantic conflict ids collapse toward a single locally corroborated role meaning"
        );
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 2);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "nlp-enrich".to_string(),
                "generated/evidence_ir/doc/evidence_ir.json".to_string(),
                "--vlm-provider".to_string(),
                "ollama".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "validate_current_artifact"
        );
        assert_eq!(
            recommendations[0].related_ids,
            vec!["semantic_conflict_0001".to_string()]
        );
    }

    #[test]
    fn project_validation_collects_evidence_structural_kg_missing_rescan_guidance() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::EvidenceIr,
            artifact_path: repo_root.join("generated/evidence_ir/doc/evidence_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "source_ir",
                path: repo_root.join("generated/source_ir/doc/source_ir.json"),
            }],
            report: ValidationReportRecord {
                report_id: "validation_evidence_ir_test".to_string(),
                validated_stage: IrStage::EvidenceIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "EvidenceIR validation with structural-KG-missing rescan guidance"
                    .to_string(),
                overall_score: None,
                grade: None,
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: EVIDENCE_STRUCTURAL_KG_MISSING_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "EvidenceIR still lacks actor-signal graph grounding for behavioral evidence ids"
                            .to_string(),
                    related_ids: vec!["sigcon_hready_asserted".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].extractor_lane,
            "evidence_ir_multimodal_semantic_corroboration"
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR and validate whether the related behavioral evidence ids collapse into actor-grounded graph relations instead of remaining structurally ungrounded"
        );
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 2);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "nlp-enrich".to_string(),
                "generated/evidence_ir/doc/evidence_ir.json".to_string(),
                "--vlm-provider".to_string(),
                "ollama".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "validate_current_artifact"
        );
        assert_eq!(
            recommendations[0].related_ids,
            vec!["sigcon_hready_asserted".to_string()]
        );
    }

    #[test]
    fn project_validation_collects_evidence_signal_polarity_conflict_rescan_guidance() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::EvidenceIr,
            artifact_path: repo_root.join("generated/evidence_ir/doc/evidence_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "source_ir",
                path: repo_root.join("generated/source_ir/doc/source_ir.json"),
            }],
            report: ValidationReportRecord {
                report_id: "validation_evidence_ir_test".to_string(),
                validated_stage: IrStage::EvidenceIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "EvidenceIR validation with signal-polarity-conflict rescan guidance"
                    .to_string(),
                overall_score: None,
                grade: None,
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: EVIDENCE_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "EvidenceIR still carries unresolved active-level conflict ids"
                        .to_string(),
                    related_ids: vec!["polarity_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].extractor_lane,
            "evidence_ir_multimodal_semantic_corroboration"
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR and validate whether the related polarity conflict ids collapse toward a single locally corroborated active-level interpretation"
        );
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 2);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "nlp-enrich".to_string(),
                "generated/evidence_ir/doc/evidence_ir.json".to_string(),
                "--vlm-provider".to_string(),
                "ollama".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "validate_current_artifact"
        );
        assert_eq!(
            recommendations[0].related_ids,
            vec!["polarity_conflict_0001".to_string()]
        );
    }

    #[test]
    fn project_validation_collects_evidence_normative_residual_rescan_guidance() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::EvidenceIr,
            artifact_path: repo_root.join("generated/evidence_ir/doc/evidence_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "source_ir",
                path: repo_root.join("generated/source_ir/doc/source_ir.json"),
            }],
            report: ValidationReportRecord {
                report_id: "validation_evidence_ir_test".to_string(),
                validated_stage: IrStage::EvidenceIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "EvidenceIR validation with normative-residual rescan guidance"
                    .to_string(),
                overall_score: None,
                grade: None,
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: EVIDENCE_NORMATIVE_RESIDUAL_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "EvidenceIR still carries partially structured normative statement ids"
                            .to_string(),
                    related_ids: vec!["stmt_normative_residual".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].extractor_lane,
            "evidence_ir_multimodal_semantic_corroboration"
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR and validate whether the related normative residual statement ids collapse into typed constraints, rules, or structured evidence instead of remaining only partially structured"
        );
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 2);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "validate_current_artifact"
        );
        assert_eq!(
            recommendations[0].related_ids,
            vec!["stmt_normative_residual".to_string()]
        );
    }

    #[test]
    fn project_validation_collects_temporal_rule_surface_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with temporal-rule-surface rescan guidance"
                    .to_string(),
                overall_score: Some(88),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_TEMPORAL_RULE_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "timing evidence exists but no typed temporal rules were derived"
                        .to_string(),
                    related_ids: vec!["timing_hready_setup".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal source ids now lower into typed temporal rules"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "nlp-enrich".to_string(),
                "generated/evidence_ir/doc/evidence_ir.json".to_string(),
                "--vlm-provider".to_string(),
                "ollama".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "semantic".to_string(),
                "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_actor_port_gap_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with actor-port-gap rescan guidance".to_string(),
                overall_score: Some(72),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "SemanticIR still carries actor-signal relation ids without actor-relative ports"
                            .to_string(),
                    related_ids: vec!["asr_0001".to_string(), "asr_0002".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related actor-signal relation ids now collapse into actor-relative port direction records"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_source_vlm_enrichment_missing_rescan_guidance() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SourceIr,
            artifact_path: repo_root.join("generated/source_ir/doc/source_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "source_document",
                path: repo_root.join("docs/spec.pdf"),
            }],
            report: ValidationReportRecord {
                report_id: "validation_source_ir_test".to_string(),
                validated_stage: IrStage::SourceIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SourceIR validation with missing-VLM rescan guidance".to_string(),
                overall_score: None,
                grade: None,
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SOURCE_VLM_ENRICHMENT_MISSING_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "SourceIR still carries timing/state diagram asset ids without VLM enrichment"
                            .to_string(),
                    related_ids: vec!["asset_timing_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].extractor_lane,
            "source_ir_visual_enrichment_rescan"
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local visual enrichment on SourceIR and validate whether the related asset ids gain VLM enrichment instead of remaining timing/state diagrams without extracted observations"
        );
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "source_ir".to_string(),
                path: "generated/source_ir/doc/source_ir.json".to_string(),
            }]
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 2);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "enrich_source_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "enrich".to_string(),
                "generated/source_ir/doc/source_ir.json".to_string(),
                "--vlm-provider".to_string(),
                "ollama".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "validate_current_artifact"
        );
        assert_eq!(
            recommendations[0].related_ids,
            vec!["asset_timing_0001".to_string()]
        );
    }

    #[test]
    fn project_validation_collects_evidence_missing_vlm_observations_rescan_guidance() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::EvidenceIr,
            artifact_path: repo_root.join("generated/evidence_ir/doc/evidence_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "source_ir",
                path: repo_root.join("generated/source_ir/doc/source_ir.json"),
            }],
            report: ValidationReportRecord {
                report_id: "validation_evidence_ir_test".to_string(),
                validated_stage: IrStage::EvidenceIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "EvidenceIR validation with missing-VLM rescan guidance".to_string(),
                overall_score: None,
                grade: None,
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: EVIDENCE_MISSING_VLM_OBSERVATIONS_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "EvidenceIR still carries visual evidence ids without timing/state observations"
                            .to_string(),
                    related_ids: vec!["visual_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].extractor_lane,
            "evidence_ir_multimodal_semantic_corroboration"
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "rerun local visual enrichment from SourceIR, rebuild EvidenceIR, and validate whether the related visual ids gain timing/state observations instead of remaining VLM-unenriched"
        );
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "source_ir".to_string(),
                path: "generated/source_ir/doc/source_ir.json".to_string(),
            }]
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "enrich_source_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "enrich".to_string(),
                "generated/source_ir/doc/source_ir.json".to_string(),
                "--vlm-provider".to_string(),
                "ollama".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
        assert_eq!(
            recommendations[0].related_ids,
            vec!["visual_0001".to_string()]
        );
    }

    #[test]
    fn project_validation_collects_semantic_role_arbitration_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with semantic-role-arbitration rescan guidance"
                    .to_string(),
                overall_score: Some(86),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "competing semantic role evidence remains non-decisive".to_string(),
                    related_ids: vec!["XCTRL".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids now converge toward a decisive semantic-role outcome"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_semantic_role_consensus_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with semantic-role-consensus rescan guidance"
                    .to_string(),
                overall_score: Some(85),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "fallback-only role meaning still lacks observation-backed consensus"
                        .to_string(),
                    related_ids: vec!["XREQ".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain observation-backed semantic-role consensus"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_alias_dependent_semantic_consensus_rescan_guidance_for_semantic_stage()
     {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary:
                    "SemanticIR validation with alias-dependent semantic-consensus rescan guidance"
                        .to_string(),
                overall_score: Some(84),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "resolved role meaning still depends only on alias-grounded evidence"
                        .to_string(),
                    related_ids: vec!["XREQ".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain direct or corroborating non-alias semantic-role consensus"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_prior_guided_semantic_consensus_rescan_guidance_for_semantic_stage()
     {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary:
                    "SemanticIR validation with prior-guided semantic-consensus rescan guidance"
                        .to_string(),
                overall_score: Some(83),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "resolved role meaning was strengthened by learned modality-reliability priors"
                            .to_string(),
                    related_ids: vec!["XCTRL".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain stronger current-document semantic-role consensus without depending on learned modality-reliability priors"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_temporal_cycle_window_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with temporal cycle-window rescan guidance"
                    .to_string(),
                overall_score: Some(82),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "typed temporal rules still lack explicit cycle-window bounds"
                        .to_string(),
                    related_ids: vec!["temporal_rule_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal rule ids gain explicit cycle-window bounds"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_temporal_clock_grounding_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with temporal clock-grounding rescan guidance"
                    .to_string(),
                overall_score: Some(81),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "typed temporal rules still lack explicit clock or edge grounding"
                        .to_string(),
                    related_ids: vec!["temporal_rule_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal rule ids gain explicit clock or edge grounding"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_temporal_actor_grounding_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with temporal actor-grounding rescan guidance"
                    .to_string(),
                overall_score: Some(80),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "typed temporal rules still lack actor-relative drive/sample grounding"
                            .to_string(),
                    related_ids: vec!["temporal_rule_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal rule ids gain actor-relative drive/sample grounding"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_graph_direction_coverage_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with graph-direction coverage rescan guidance"
                    .to_string(),
                overall_score: Some(79),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "canonical surface still lacks actor-relative graph direction coverage"
                            .to_string(),
                    related_ids: vec!["PSEL".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain actor-relative graph direction coverage"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_graph_direction_conflict_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with graph-direction conflict rescan guidance"
                    .to_string(),
                overall_score: Some(78),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "actor-relative graph still carries unresolved same-actor direction disagreement"
                            .to_string(),
                    related_ids: vec!["graph_direction_conflict:actor_completer:PREADY".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related graph-direction conflict ids collapse toward one actor-relative direction per actor-signal edge"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_keeps_graph_direction_conflict_and_coverage_replays_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with conflict and coverage guidance".to_string(),
                overall_score: Some(78),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![
                    ValidationFindingRecord {
                        finding_id: SEMANTIC_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
                            .to_string(),
                        severity: ValidationFindingSeverity::Info,
                        category: "rescan_guidance".to_string(),
                        summary:
                            "canonical surface still lacks actor-relative graph direction coverage"
                                .to_string(),
                        related_ids: vec!["PREADY".to_string()],
                    },
                    ValidationFindingRecord {
                        finding_id: SEMANTIC_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
                            .to_string(),
                        severity: ValidationFindingSeverity::Info,
                        category: "rescan_guidance".to_string(),
                        summary:
                            "actor-relative graph still carries unresolved same-actor direction disagreement"
                                .to_string(),
                        related_ids: vec![
                            "graph_direction_conflict:actor_completer:PREADY".to_string(),
                        ],
                    },
                ],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 2);
        let conflict = recommendations
            .iter()
            .find(|recommendation| {
                recommendation.finding_id
                    == SEMANTIC_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected graph-direction conflict replay recommendation");
        assert_eq!(
            conflict.related_ids,
            vec!["graph_direction_conflict:actor_completer:PREADY".to_string()]
        );
        assert_eq!(
            conflict.recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related graph-direction conflict ids collapse toward one actor-relative direction per actor-signal edge"
        );
        let expected_replay_inputs = vec![ProjectRescanReplayInput {
            input_kind: "evidence_ir".to_string(),
            path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
        }];
        assert_eq!(conflict.replay_inputs, expected_replay_inputs);
        assert_eq!(
            recommended_command_intents(conflict),
            vec![
                "nlp_enrich_evidence_ir",
                "rebuild_semantic_ir",
                "validate_current_artifact"
            ]
        );
        let coverage = recommendations
            .iter()
            .find(|recommendation| {
                recommendation.finding_id
                    == SEMANTIC_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected graph-direction coverage replay recommendation");
        assert_eq!(coverage.related_ids, vec!["PREADY".to_string()]);
        assert_eq!(
            coverage.recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain actor-relative graph direction coverage"
        );
        assert_eq!(coverage.replay_inputs, expected_replay_inputs);
        assert_eq!(
            recommended_command_intents(coverage),
            vec![
                "nlp_enrich_evidence_ir",
                "rebuild_semantic_ir",
                "validate_current_artifact"
            ]
        );
    }

    #[test]
    fn project_validation_collects_connectivity_missing_producer_rescan_guidance_for_semantic_stage()
     {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with connectivity-producer rescan guidance"
                    .to_string(),
                overall_score: Some(78),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "connectivity graph still lacks resolved producer-side connectivity evidence"
                            .to_string(),
                    related_ids: vec!["PREADY".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain producer-side connectivity evidence"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_connectivity_missing_consumer_rescan_guidance_for_semantic_stage()
     {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with connectivity-consumer rescan guidance"
                    .to_string(),
                overall_score: Some(78),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "connectivity graph still lacks resolved consumer-side connectivity evidence"
                            .to_string(),
                    related_ids: vec!["PSEL".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related signal ids gain consumer-side connectivity evidence"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_signal_connectivity_conflict_rescan_guidance_for_semantic_stage()
    {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with signal-connectivity-conflict rescan guidance"
                    .to_string(),
                overall_score: Some(77),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "structural graph still carries unresolved producer ambiguity"
                        .to_string(),
                    related_ids: vec!["signal_connectivity_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related connectivity conflict ids collapse toward single-producer connectivity"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_interface_signal_conflict_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with interface-signal-conflict rescan guidance"
                    .to_string(),
                overall_score: Some(76),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "canonical interface surface still carries unresolved direction/width disagreement"
                        .to_string(),
                    related_ids: vec![
                        "interface_signal_conflict_0001".to_string(),
                        "interface_signal_conflict_0002".to_string(),
                    ],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related interface conflict ids collapse toward consistent direction/width declarations"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_temporal_conflict_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with temporal-conflict rescan guidance".to_string(),
                overall_score: Some(75),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "typed timing surface still carries contradictory value obligations"
                        .to_string(),
                    related_ids: vec!["temporal_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related temporal conflict ids collapse toward a single locally corroborated timing obligation"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_signal_polarity_conflict_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with polarity-conflict rescan guidance"
                    .to_string(),
                overall_score: Some(75),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "canonical polarity surface still carries unresolved active-level disagreement"
                            .to_string(),
                    related_ids: vec!["polarity_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related polarity conflict ids collapse toward a single locally corroborated active-level interpretation"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_signal_semantic_conflict_rescan_guidance_for_semantic_stage() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let evidence_ir_path = repo_root.join("generated/evidence_ir/doc/evidence_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::SemanticIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "evidence_ir",
                path: evidence_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_semantic_ir_test".to_string(),
                validated_stage: IrStage::SemanticIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "SemanticIR validation with signal-semantic-conflict rescan guidance"
                    .to_string(),
                overall_score: Some(74),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: SEMANTIC_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "canonical semantic surface still carries unresolved semantic-role conflict ids"
                            .to_string(),
                    related_ids: vec!["semantic_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            }]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR, and validate whether the related semantic conflict ids collapse toward a single locally corroborated role meaning"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 3);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "validate_current_artifact"
        );
    }

    #[test]
    fn project_validation_collects_temporal_rule_surface_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with temporal-rule-surface rescan guidance"
                    .to_string(),
                overall_score: Some(82),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_TEMPORAL_RULE_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "semantic timing evidence exists but no typed temporal rules survived"
                        .to_string(),
                    related_ids: vec!["timing_hready_setup".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal source ids survive as typed temporal rules without blind promotion"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "nlp-enrich".to_string(),
                "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                "--vlm-provider".to_string(),
                "ollama".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "semantic".to_string(),
                "generated/evidence_ir/spec/evidence_ir.json".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "intent".to_string(),
                "generated/semantic_ir/spec/semantic_ir.json".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_semantic_role_arbitration_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal XCTRL is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with semantic-role-arbitration rescan guidance"
                    .to_string(),
                overall_score: Some(80),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "competing semantic role evidence remains non-decisive".to_string(),
                    related_ids: vec!["XCTRL".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids now survive with a decisive semantic-role outcome instead of contested arbitration"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_semantic_role_consensus_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal XREQ is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with semantic-role-consensus rescan guidance"
                    .to_string(),
                overall_score: Some(79),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "fallback-only role meaning still lacks observation-backed consensus"
                        .to_string(),
                    related_ids: vec!["XREQ".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with observation-backed semantic-role consensus instead of fallback-only role meaning"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_alias_dependent_semantic_consensus_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal XREQ is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary:
                    "IntentIR validation with alias-dependent semantic-consensus rescan guidance"
                        .to_string(),
                overall_score: Some(78),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "resolved role meaning still depends only on alias-grounded evidence"
                        .to_string(),
                    related_ids: vec!["XREQ".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with direct or corroborating non-alias semantic-role consensus instead of alias-dependent carry-through"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_prior_guided_semantic_consensus_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal XCTRL is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary:
                    "IntentIR validation with prior-guided semantic-consensus rescan guidance"
                        .to_string(),
                overall_score: Some(77),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "resolved role meaning was strengthened by learned modality-reliability priors"
                            .to_string(),
                    related_ids: vec!["XCTRL".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with stronger current-document semantic-role consensus instead of prior-guided carry-through"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_temporal_cycle_window_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with temporal cycle-window rescan guidance"
                    .to_string(),
                overall_score: Some(76),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "typed temporal rules still lack explicit cycle-window bounds"
                        .to_string(),
                    related_ids: vec!["temporal_rule_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal rule ids survive with explicit cycle-window bounds instead of remaining unbounded"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_temporal_clock_grounding_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with temporal clock-grounding rescan guidance"
                    .to_string(),
                overall_score: Some(75),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "typed temporal rules still lack explicit clock or edge grounding"
                        .to_string(),
                    related_ids: vec!["temporal_rule_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal rule ids survive with explicit clock or edge grounding instead of remaining clockless"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_temporal_actor_grounding_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with temporal actor-grounding rescan guidance"
                    .to_string(),
                overall_score: Some(74),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "typed temporal rules still lack actor-relative drive/sample grounding"
                            .to_string(),
                    related_ids: vec!["temporal_rule_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal rule ids survive with actor-relative drive/sample grounding instead of remaining actor-ungrounded"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_actor_port_gap_rescan_guidance_for_intent_stage() -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with actor-port-gap rescan guidance".to_string(),
                overall_score: Some(74),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "IntentIR still carries actor-signal relation ids without actor-relative ports"
                            .to_string(),
                    related_ids: vec!["asr_0001".to_string(), "asr_0002".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related actor-signal relation ids survive as actor-relative port direction records instead of relation-only graph evidence"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_graph_direction_coverage_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with graph-direction coverage rescan guidance"
                    .to_string(),
                overall_score: Some(73),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "canonical surface still lacks actor-relative graph direction coverage"
                            .to_string(),
                    related_ids: vec!["PSEL".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with actor-relative graph direction coverage instead of remaining graph-uncovered"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_graph_direction_conflict_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "Signal PADDR is input width 32.\n",
                "\n",
                "The Completer drives PREADY.\n",
                "\n",
                "The Requester reads PREADY.\n",
                "\n",
                "The Requester drives PADDR.\n",
                "\n",
                "The Completer samples PADDR.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let mut semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        let conflicting_port = semantic_ir
            .actor_ports
            .iter()
            .find(|port| port.actor_name == "Completer" && port.signal_name == "PREADY")
            .cloned()
            .expect("Completer PREADY actor port should exist");
        let mut conflicting_port = conflicting_port;
        conflicting_port.direction = crate::ir::semantic::ActorRelativeDirection::Input;
        semantic_ir.actor_ports.push(conflicting_port);
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with graph-direction conflict rescan guidance"
                    .to_string(),
                overall_score: Some(72),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "actor-relative graph still carries unresolved same-actor direction disagreement"
                            .to_string(),
                    related_ids: vec!["graph_direction_conflict:actor_completer:PREADY".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related graph-direction conflict ids survive with one actor-relative direction per actor-signal edge instead of unresolved same-actor direction disagreement"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_keeps_graph_direction_conflict_and_coverage_replays_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "Signal PADDR is input width 32.\n",
                "\n",
                "The Completer drives PREADY.\n",
                "\n",
                "The Requester reads PREADY.\n",
                "\n",
                "The Requester drives PADDR.\n",
                "\n",
                "The Completer samples PADDR.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let mut semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        let conflicting_port = semantic_ir
            .actor_ports
            .iter()
            .find(|port| port.actor_name == "Completer" && port.signal_name == "PREADY")
            .cloned()
            .expect("Completer PREADY actor port should exist");
        let mut conflicting_port = conflicting_port;
        conflicting_port.direction = crate::ir::semantic::ActorRelativeDirection::Input;
        semantic_ir.actor_ports.push(conflicting_port);
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with conflict and coverage guidance".to_string(),
                overall_score: Some(72),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![
                    ValidationFindingRecord {
                        finding_id: INTENT_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
                            .to_string(),
                        severity: ValidationFindingSeverity::Info,
                        category: "rescan_guidance".to_string(),
                        summary:
                            "canonical surface still lacks actor-relative graph direction coverage"
                                .to_string(),
                        related_ids: vec!["PREADY".to_string()],
                    },
                    ValidationFindingRecord {
                        finding_id: INTENT_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
                            .to_string(),
                        severity: ValidationFindingSeverity::Info,
                        category: "rescan_guidance".to_string(),
                        summary:
                            "actor-relative graph still carries unresolved same-actor direction disagreement"
                                .to_string(),
                        related_ids: vec![
                            "graph_direction_conflict:actor_completer:PREADY".to_string(),
                        ],
                    },
                ],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 2);
        let expected_replay_inputs = vec![
            ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
            },
            ProjectRescanReplayInput {
                input_kind: "semantic_ir".to_string(),
                path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
            },
        ];
        let conflict = recommendations
            .iter()
            .find(|recommendation| {
                recommendation.finding_id == INTENT_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected graph-direction conflict replay recommendation");
        assert_eq!(conflict.replay_inputs, expected_replay_inputs);
        assert_eq!(
            conflict.related_ids,
            vec!["graph_direction_conflict:actor_completer:PREADY".to_string()]
        );
        assert_eq!(
            conflict.recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related graph-direction conflict ids survive with one actor-relative direction per actor-signal edge instead of unresolved same-actor direction disagreement"
        );
        assert_eq!(
            recommended_command_intents(conflict),
            vec![
                "nlp_enrich_evidence_ir",
                "rebuild_semantic_ir",
                "rebuild_intent_ir",
                "validate_current_artifact"
            ]
        );
        let coverage = recommendations
            .iter()
            .find(|recommendation| {
                recommendation.finding_id == INTENT_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected graph-direction coverage replay recommendation");
        assert_eq!(coverage.replay_inputs, expected_replay_inputs);
        assert_eq!(coverage.related_ids, vec!["PREADY".to_string()]);
        assert_eq!(
            coverage.recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with actor-relative graph direction coverage instead of remaining graph-uncovered"
        );
        assert_eq!(
            recommended_command_intents(coverage),
            vec![
                "nlp_enrich_evidence_ir",
                "rebuild_semantic_ir",
                "rebuild_intent_ir",
                "validate_current_artifact"
            ]
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_connectivity_missing_producer_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with connectivity-producer rescan guidance"
                    .to_string(),
                overall_score: Some(72),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "connectivity graph still lacks resolved producer-side connectivity evidence"
                            .to_string(),
                    related_ids: vec!["PREADY".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with producer-side connectivity evidence instead of remaining producerless"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_connectivity_missing_consumer_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with connectivity-consumer rescan guidance"
                    .to_string(),
                overall_score: Some(72),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "connectivity graph still lacks resolved consumer-side connectivity evidence"
                            .to_string(),
                    related_ids: vec!["PSEL".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related signal ids survive with consumer-side connectivity evidence instead of remaining consumerless"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_signal_connectivity_conflict_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with signal-connectivity-conflict rescan guidance"
                    .to_string(),
                overall_score: Some(71),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "structural graph still carries unresolved producer ambiguity"
                        .to_string(),
                    related_ids: vec!["signal_connectivity_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related connectivity conflict ids survive with single-producer connectivity instead of unresolved producer ambiguity"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_interface_signal_conflict_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with interface-signal-conflict rescan guidance"
                    .to_string(),
                overall_score: Some(72),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "carried interface surface still carries unresolved direction/width disagreement"
                        .to_string(),
                    related_ids: vec![
                        "interface_signal_conflict_0001".to_string(),
                        "interface_signal_conflict_0002".to_string(),
                    ],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related interface conflict ids survive with consistent direction/width declarations instead of unresolved interface-shape ambiguity"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_temporal_conflict_rescan_guidance_for_intent_stage() -> Result<()>
    {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal HREADY is input width 1.\n\n",
                "Signal PREADY is output width 1.\n\n",
                "Clock clk.\n\n",
                "The Completer drives PREADY.\n\n",
                "The Requester reads PREADY.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir
            .signal_constraints
            .push(crate::ir::source::SignalConstraintRecord {
                constraint_id: "sigcon_pready_high".to_string(),
                subject_signal: "PREADY".to_string(),
                constraint_kind: crate::ir::source::SignalConstraintKind::MustBeHigh,
                target_value: None,
                condition_text: Some("when HREADY is LOW".to_string()),
                negated: false,
                source_text: "PREADY must be HIGH when HREADY is LOW.".to_string(),
                supporting_statement_ids: vec!["stmt_pready_high".to_string()],
                automation_confidence: crate::ir::source::AutomationConfidence::Medium,
            });
        evidence_ir
            .signal_constraints
            .push(crate::ir::source::SignalConstraintRecord {
                constraint_id: "sigcon_pready_low".to_string(),
                subject_signal: "PREADY".to_string(),
                constraint_kind: crate::ir::source::SignalConstraintKind::MustBeLow,
                target_value: None,
                condition_text: Some("when HREADY is LOW".to_string()),
                negated: false,
                source_text: "PREADY must be LOW when HREADY is LOW.".to_string(),
                supporting_statement_ids: vec!["stmt_pready_low".to_string()],
                automation_confidence: crate::ir::source::AutomationConfidence::Medium,
            });
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with temporal-conflict rescan guidance".to_string(),
                overall_score: Some(70),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE.to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "typed timing surface still carries contradictory value obligations"
                        .to_string(),
                    related_ids: vec!["temporal_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related temporal conflict ids survive with a single locally corroborated timing obligation instead of contradictory value obligations"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_signal_polarity_conflict_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            concat!(
                "# Reset\n\n",
                "Signal PRESETN is input width 1.\n\n",
                "PRESETN is active HIGH.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir
            .structured_tables
            .push(crate::ir::source::StructuredTableRecord {
                table_id: "table_reset_desc_conflict".to_string(),
                asset_id: "asset_reset_desc_conflict".to_string(),
                page_id: Some("page_0001".to_string()),
                caption_text: Some("Reset signal descriptions".to_string()),
                source_ref: None,
                table_kind: crate::ir::source::TableKind::SignalDescription,
                header_rows: vec![vec![
                    crate::ir::source::StructuredTableCellRecord {
                        text: "Signal".to_string(),
                        row_span: 1,
                        col_span: 1,
                        is_header: true,
                    },
                    crate::ir::source::StructuredTableCellRecord {
                        text: "Description".to_string(),
                        row_span: 1,
                        col_span: 1,
                        is_header: true,
                    },
                ]],
                body_rows: vec![vec![
                    crate::ir::source::StructuredTableCellRecord {
                        text: "PRESETN".to_string(),
                        row_span: 1,
                        col_span: 1,
                        is_header: false,
                    },
                    crate::ir::source::StructuredTableCellRecord {
                        text: "Active low reset.".to_string(),
                        row_span: 1,
                        col_span: 1,
                        is_header: false,
                    },
                ]],
                row_count: 1,
                col_count: 2,
            });
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with polarity-conflict rescan guidance"
                    .to_string(),
                overall_score: Some(72),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "carried polarity surface still carries unresolved active-level disagreement"
                            .to_string(),
                    related_ids: vec!["polarity_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related polarity conflict ids survive with a single locally corroborated active-level interpretation instead of unresolved polarity disagreement"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_collects_signal_semantic_conflict_rescan_guidance_for_intent_stage()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal XCTRL is input width 1.\n\n",
                "XCTRL indicates that the subordinate can accept the transfer.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir
            .structured_tables
            .push(crate::ir::source::StructuredTableRecord {
                table_id: "table_semantic_conflict".to_string(),
                asset_id: "asset_semantic_conflict".to_string(),
                page_id: Some("page_0001".to_string()),
                caption_text: Some("Control signal descriptions".to_string()),
                source_ref: None,
                table_kind: crate::ir::source::TableKind::SignalDescription,
                header_rows: vec![vec![
                    crate::ir::source::StructuredTableCellRecord {
                        text: "Signal".to_string(),
                        row_span: 1,
                        col_span: 1,
                        is_header: true,
                    },
                    crate::ir::source::StructuredTableCellRecord {
                        text: "Description".to_string(),
                        row_span: 1,
                        col_span: 1,
                        is_header: true,
                    },
                ]],
                body_rows: vec![vec![
                    crate::ir::source::StructuredTableCellRecord {
                        text: "XCTRL".to_string(),
                        row_span: 1,
                        col_span: 1,
                        is_header: false,
                    },
                    crate::ir::source::StructuredTableCellRecord {
                        text:
                            "Indicates that address and control information are valid for transfer."
                                .to_string(),
                        row_span: 1,
                        col_span: 1,
                        is_header: false,
                    },
                ]],
                row_count: 1,
                col_count: 2,
            });
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/doc/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with signal-semantic-conflict rescan guidance"
                    .to_string(),
                overall_score: Some(71),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: INTENT_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE
                        .to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary:
                        "canonical intent surface still carries unresolved semantic-role conflict ids"
                            .to_string(),
                    related_ids: vec!["semantic_conflict_0001".to_string()],
                }],
            },
        };

        let recommendations =
            collect_rescan_recommendations(&[snapshot], &repo_root, &test_rescan_vlm_policy());

        assert_eq!(recommendations.len(), 1);
        assert_eq!(
            recommendations[0].replay_inputs,
            vec![
                ProjectRescanReplayInput {
                    input_kind: "evidence_ir".to_string(),
                    path: "generated/evidence_ir/spec/evidence_ir.json".to_string(),
                },
                ProjectRescanReplayInput {
                    input_kind: "semantic_ir".to_string(),
                    path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                },
            ]
        );
        assert_eq!(
            recommendations[0].recommended_action,
            "run local NLP enrichment on EvidenceIR, rebuild SemanticIR and IntentIR, and validate whether the related semantic conflict ids survive with a single locally corroborated role meaning instead of unresolved semantic-role disagreement"
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 4);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "nlp_enrich_evidence_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "rebuild_semantic_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[2].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[3].intent,
            "validate_current_artifact"
        );

        Ok(())
    }

    #[test]
    fn project_validation_rescan_vlm_policy_can_emit_lmstudio_hint() {
        let tempdir = tempdir().expect("tempdir");
        let repo_root = tempdir.path();
        let policy = RescanVlmHintPolicy {
            provider: RescanVlmProviderArg::LmStudio,
            model: Some("qwen2.5vl:7b".to_string()),
        };

        let recommendations =
            collect_rescan_recommendations(&[visual_motif_snapshot(repo_root)], repo_root, &policy);

        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "enrich".to_string(),
                "generated/source_ir/doc/source_ir.json".to_string(),
                "--vlm-provider".to_string(),
                "lmstudio".to_string(),
                "--vlm-model".to_string(),
                "qwen2.5vl:7b".to_string(),
            ]
        );
    }

    #[test]
    fn project_validation_auto_rescan_vlm_policy_prefers_ready_lmstudio_when_ollama_absent() {
        let provider = select_rescan_vlm_provider(RescanVlmProviderArg::AutoLocal, |provider| {
            matches!(provider, VlmProviderArg::LmStudio)
        });

        assert_eq!(provider, VlmProviderArg::LmStudio);
    }

    #[test]
    fn project_validation_preserves_matching_rescan_execution_summary() -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/intent_ir/doc/intent_ir.json");
        let semantic_ir_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path,
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir_path,
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with rescan guidance".to_string(),
                overall_score: Some(85),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: "intent_negative_knowledge_rescan_guidance".to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "known failure shape needs rescan".to_string(),
                    related_ids: vec!["temporal_conflict_0001".to_string()],
                }],
            },
        };
        let snapshots = vec![snapshot];
        let mut previous_recommendations =
            collect_rescan_recommendations(&snapshots, repo_root, &test_rescan_vlm_policy());
        previous_recommendations[0].automation_status = "executed_validated_changed".to_string();
        previous_recommendations[0].execution_summary = Some(execution_summary(
            "regression_review_required",
            Some(-10),
            1,
        ));
        write_validation_rescan_plan(repo_root, previous_recommendations)?;

        let previous_plan = read_existing_validation_rescan_plan(repo_root)?;
        let mut refreshed_recommendations =
            collect_rescan_recommendations(&snapshots, repo_root, &test_rescan_vlm_policy());
        merge_previous_rescan_execution_state(
            &mut refreshed_recommendations,
            previous_plan.as_ref(),
        );

        assert_eq!(
            refreshed_recommendations[0].automation_status,
            "executed_validated_changed"
        );
        assert_eq!(
            refreshed_recommendations[0]
                .execution_summary
                .as_ref()
                .expect("execution summary")
                .arbitration_verdict,
            "regression_review_required"
        );
        assert_eq!(
            refreshed_recommendations[0]
                .execution_summary
                .as_ref()
                .expect("execution summary")
                .promotion_status,
            RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED
        );

        Ok(())
    }

    #[test]
    fn project_validation_does_not_preserve_execution_summary_when_replay_contract_changes()
    -> Result<()> {
        let tempdir = tempdir()?;
        let repo_root = fs::canonicalize(tempdir.path())?;
        let source = repo_root.join("spec.md");
        let source_artifact_base = repo_root.join("generated").join("source_ir");
        let evidence_artifact_base = repo_root.join("generated").join("evidence_ir");
        let semantic_artifact_base = repo_root.join("generated").join("semantic_ir");
        fs::write(
            &source,
            "# Spec\nSignal HREADY is input width 1.\nSignal DATA is output width 32.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let snapshot = ProjectedArtifactSnapshot {
            document_key: "spec".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: repo_root.join("generated/intent_ir/spec/intent_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "semantic_ir",
                path: semantic_ir.artifact_layout.semantic_ir_path.clone(),
            }],
            report: ValidationReportRecord {
                report_id: "validation_intent_ir_test".to_string(),
                validated_stage: IrStage::IntentIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "IntentIR validation with rescan guidance".to_string(),
                overall_score: Some(85),
                grade: Some("GOOD".to_string()),
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: "intent_negative_knowledge_rescan_guidance".to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "known failure shape needs rescan".to_string(),
                    related_ids: vec!["temporal_conflict_0001".to_string()],
                }],
            },
        };
        let snapshots = vec![snapshot];
        let mut previous_recommendations =
            collect_rescan_recommendations(&snapshots, &repo_root, &test_rescan_vlm_policy());
        previous_recommendations[0].replay_inputs = vec![ProjectRescanReplayInput {
            input_kind: "semantic_ir".to_string(),
            path: "generated/semantic_ir/spec/semantic_ir.json".to_string(),
        }];
        previous_recommendations[0].recommended_commands = vec![
            specforge_command_hint(
                "rebuild_intent_ir",
                vec![
                    "intent".to_string(),
                    "generated/semantic_ir/spec/semantic_ir.json".to_string(),
                ],
            ),
            specforge_command_hint(
                "validate_current_artifact",
                vec![
                    "validate".to_string(),
                    "generated/intent_ir/spec/intent_ir.json".to_string(),
                ],
            ),
        ];
        previous_recommendations[0].automation_status = "executed_validated_changed".to_string();
        previous_recommendations[0].execution_summary = Some(execution_summary(
            "regression_review_required",
            Some(-10),
            1,
        ));
        write_validation_rescan_plan(&repo_root, previous_recommendations)?;

        let previous_plan = read_existing_validation_rescan_plan(&repo_root)?;
        let mut refreshed_recommendations =
            collect_rescan_recommendations(&snapshots, &repo_root, &test_rescan_vlm_policy());
        merge_previous_rescan_execution_state(
            &mut refreshed_recommendations,
            previous_plan.as_ref(),
        );

        assert_eq!(
            refreshed_recommendations[0].automation_status,
            "planned_not_executed"
        );
        assert!(refreshed_recommendations[0].execution_summary.is_none());

        Ok(())
    }

    #[test]
    fn project_validation_normalizes_legacy_rescan_execution_summary_gate() -> Result<()> {
        let mut summary: ProjectRescanExecutionSummary = serde_json::from_str(
            r#"{
                "automation_status": "executed_validated_changed",
                "arbitration_verdict": "possible_improvement_review_required",
                "before_validation": {
                    "artifact_fingerprint": "before",
                    "overall_score": 80,
                    "grade": "GOOD",
                    "finding_count": 2,
                    "finding_ids": ["finding_a", "finding_b"]
                },
                "after_validation": {
                    "artifact_fingerprint": "after",
                    "overall_score": 85,
                    "grade": "EXCELLENT",
                    "finding_count": 1,
                    "finding_ids": ["finding_a"]
                },
                "validation_delta": {
                    "fingerprint_changed": true,
                    "score_changed": true,
                    "score_delta": 5,
                    "grade_changed": true,
                    "finding_count_delta": -1,
                    "added_findings": [],
                    "removed_findings": ["finding_b"]
                }
            }"#,
        )?;

        assert!(summary.promotion_status.is_empty());
        assert!(summary.promotion_blockers.is_empty());
        assert!(summary.promotion_review.review_status.is_empty());

        normalize_rescan_execution_summary(&mut summary);

        assert_eq!(
            summary.promotion_status,
            RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED
        );
        assert!(
            summary
                .promotion_blockers
                .contains(&"validation_delta_is_not_truth_promotion".to_string())
        );
        assert_eq!(
            summary.promotion_review.review_status,
            RESCAN_PROMOTION_REVIEW_HUMAN_REVIEW_REQUIRED
        );
        assert!(summary.promotion_review.approval_record_required);
        assert!(!summary.promotion_review.canonical_mutation_allowed);

        summary.promotion_status = "promoted_without_review".to_string();
        summary.promotion_blockers.clear();
        summary.promotion_review = ProjectRescanPromotionReview::default();
        normalize_rescan_execution_summary(&mut summary);

        assert_eq!(
            summary.promotion_status,
            RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED
        );
        assert!(
            summary
                .promotion_blockers
                .contains(&"current_document_evidence_review_required".to_string())
        );
        assert_eq!(
            summary.promotion_review.approval_policy,
            RESCAN_PROMOTION_APPROVAL_POLICY_HUMAN_REVIEW
        );

        Ok(())
    }

    fn execution_summary(
        arbitration_verdict: &str,
        score_delta: Option<i32>,
        finding_count_delta: i64,
    ) -> ProjectRescanExecutionSummary {
        ProjectRescanExecutionSummary {
            automation_status: "executed_validated_changed".to_string(),
            arbitration_verdict: arbitration_verdict.to_string(),
            promotion_status: rescan_promotion_status_for(arbitration_verdict).to_string(),
            promotion_blockers: rescan_promotion_blockers_for(arbitration_verdict),
            promotion_review: rescan_promotion_review_for(arbitration_verdict),
            before_validation: ProjectRescanValidationSnapshot {
                artifact_fingerprint: "before".to_string(),
                overall_score: Some(80),
                grade: Some("GOOD".to_string()),
                finding_count: 2,
                finding_ids: vec!["finding_a".to_string(), "finding_b".to_string()],
            },
            after_validation: ProjectRescanValidationSnapshot {
                artifact_fingerprint: "after".to_string(),
                overall_score: score_delta.map(|delta| (80 + delta) as u32),
                grade: Some("EXCELLENT".to_string()),
                finding_count: (2 + finding_count_delta) as usize,
                finding_ids: if finding_count_delta < 0 {
                    vec!["finding_a".to_string()]
                } else {
                    vec![
                        "finding_a".to_string(),
                        "finding_b".to_string(),
                        "finding_c".to_string(),
                    ]
                },
            },
            validation_delta: ProjectRescanValidationDelta {
                fingerprint_changed: true,
                score_changed: score_delta != Some(0),
                score_delta,
                grade_changed: true,
                finding_count_delta,
                added_findings: if finding_count_delta > 0 {
                    vec!["finding_c".to_string()]
                } else {
                    Vec::new()
                },
                removed_findings: if finding_count_delta < 0 {
                    vec!["finding_b".to_string()]
                } else {
                    Vec::new()
                },
            },
        }
    }

    fn test_rescan_vlm_policy() -> RescanVlmHintPolicy {
        RescanVlmHintPolicy {
            provider: RescanVlmProviderArg::Ollama,
            model: None,
        }
    }

    fn recommended_command_intents(recommendation: &ProjectRescanRecommendation) -> Vec<&str> {
        recommendation
            .recommended_commands
            .iter()
            .map(|command| command.intent.as_str())
            .collect()
    }

    fn visual_motif_snapshot(repo_root: &Path) -> ProjectedArtifactSnapshot {
        ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::EvidenceIr,
            artifact_path: repo_root.join("generated/evidence_ir/doc/evidence_ir.json"),
            replay_inputs: vec![ProjectedReplayInput {
                input_kind: "source_ir",
                path: repo_root.join("generated/source_ir/doc/source_ir.json"),
            }],
            report: ValidationReportRecord {
                report_id: "validation_evidence_ir_test".to_string(),
                validated_stage: IrStage::EvidenceIr,
                artifact_fingerprint: "fingerprint".to_string(),
                summary: "EvidenceIR validation with visual corroboration guidance".to_string(),
                overall_score: None,
                grade: None,
                metrics: Vec::new(),
                findings: vec![ValidationFindingRecord {
                    finding_id: "evidence_visual_motif_corroboration_guidance".to_string(),
                    severity: ValidationFindingSeverity::Info,
                    category: "rescan_guidance".to_string(),
                    summary: "prior classified visual needs multimodal corroboration".to_string(),
                    related_ids: vec!["visual_0001".to_string()],
                }],
            },
        }
    }
}
