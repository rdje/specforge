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
    pub(crate) before_validation: ProjectRescanValidationSnapshot,
    pub(crate) after_validation: ProjectRescanValidationSnapshot,
    pub(crate) validation_delta: ProjectRescanValidationDelta,
}

pub(crate) const RESCAN_PROMOTION_NOT_PROMOTED_NO_CHANGE: &str = "not_promoted_no_change";
pub(crate) const RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED: &str =
    "not_promoted_review_required";
const RESCAN_PROMOTION_BLOCKER_CANONICAL_IR_NOT_MUTATED: &str =
    "canonical_ir_not_mutated_by_rescan_plan";
const RESCAN_PROMOTION_BLOCKER_VALIDATION_DELTA_NOT_TRUTH: &str =
    "validation_delta_is_not_truth_promotion";
const RESCAN_PROMOTION_BLOCKER_CURRENT_EVIDENCE_REVIEW: &str =
    "current_document_evidence_review_required";
const RESCAN_PROMOTION_BLOCKER_NO_DELTA: &str = "no_validation_delta_to_promote";

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
                "  - `{}` (`{}`): `{}` for {} ({} command hint(s), `{}`{})",
                recommendation.display_name,
                recommendation.stage,
                recommendation.extractor_lane,
                render_inline_code_list(&recommendation.related_ids),
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
    [
        recommendation.document_key.as_str(),
        recommendation.stage.as_str(),
        recommendation.artifact_path.as_str(),
        recommendation.finding_id.as_str(),
        recommendation.extractor_lane.as_str(),
        &related_ids.join("\u{1f}"),
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
            let replay_inputs = recommendation_replay_inputs(snapshot, repo_root);
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
                extractor_lane: extractor_lane_for_rescan(snapshot.stage).to_string(),
                corroboration_policy:
                    "stronger_local_corroboration_required_before_canonical_promotion".to_string(),
                recommended_action: recommended_rescan_action(snapshot.stage).to_string(),
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
    snapshot: &ProjectedArtifactSnapshot,
    repo_root: &Path,
) -> Vec<ProjectRescanReplayInput> {
    snapshot
        .replay_inputs
        .iter()
        .map(|input| ProjectRescanReplayInput {
            input_kind: input.input_kind.to_string(),
            path: repo_relative_display(&input.path, repo_root),
        })
        .collect()
}

fn recommended_rescan_commands(
    stage: IrStage,
    artifact_path: &Path,
    snapshot: &ProjectedArtifactSnapshot,
    finding: &ValidationFindingRecord,
    rescan_vlm_policy: &RescanVlmHintPolicy,
    repo_root: &Path,
) -> Vec<ProjectRescanCommandHint> {
    let mut commands = Vec::new();
    if is_visual_motif_corroboration_rescan(stage, finding) {
        if let Some(input) = snapshot
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

fn is_visual_motif_corroboration_rescan(stage: IrStage, finding: &ValidationFindingRecord) -> bool {
    stage == IrStage::EvidenceIr
        && finding.finding_id == EVIDENCE_VISUAL_MOTIF_CORROBORATION_GUIDANCE
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

fn extractor_lane_for_rescan(stage: IrStage) -> &'static str {
    match stage {
        IrStage::SourceIr => "source_ir_normalization_rescan",
        IrStage::EvidenceIr => "evidence_ir_multimodal_semantic_corroboration",
        IrStage::SemanticIr => "semantic_ir_conflict_corroboration",
        IrStage::IntentIr => "intent_ir_canonical_surface_corroboration",
    }
}

fn recommended_rescan_action(stage: IrStage) -> &'static str {
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
        ", verdict `{}`, promotion `{}`, score_delta `{}`, finding_count_delta `{}`",
        summary.arbitration_verdict,
        summary_promotion_status(summary),
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
        let repo_root = tempdir.path();
        let artifact_path = repo_root.join("generated/intent_ir/doc/intent_ir.json");
        let semantic_ir_path = repo_root.join("generated/semantic_ir/doc/semantic_ir.json");
        let snapshot = ProjectedArtifactSnapshot {
            document_key: "doc".to_string(),
            display_name: "Spec.pdf".to_string(),
            stage: IrStage::IntentIr,
            artifact_path: artifact_path.clone(),
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
            collect_rescan_recommendations(&snapshots, repo_root, &test_rescan_vlm_policy());
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
            recommendations[0].replay_inputs,
            vec![ProjectRescanReplayInput {
                input_kind: "semantic_ir".to_string(),
                path: "generated/semantic_ir/doc/semantic_ir.json".to_string(),
            }]
        );
        assert_eq!(recommendations[0].recommended_commands.len(), 2);
        assert_eq!(
            recommendations[0].recommended_commands[0].intent,
            "rebuild_intent_ir"
        );
        assert_eq!(
            recommendations[0].recommended_commands[0].args,
            vec![
                "run".to_string(),
                "--manifest-path".to_string(),
                "Cargo.toml".to_string(),
                "--".to_string(),
                "intent".to_string(),
                "generated/semantic_ir/doc/semantic_ir.json".to_string(),
            ]
        );
        assert_eq!(
            recommendations[0].recommended_commands[1].intent,
            "validate_current_artifact"
        );
        assert_eq!(recommendations[0].automation_status, "planned_not_executed");
        recommendations[0].automation_status = "executed_validated_changed".to_string();
        recommendations[0].execution_summary = Some(execution_summary(
            "possible_improvement_review_required",
            Some(5),
            -1,
        ));

        let snapshot_doc = render_validation_snapshot_doc(&snapshots, repo_root, &recommendations);
        assert!(snapshot_doc.contains("## Targeted Rescan Recommendations"));
        assert!(snapshot_doc.contains("intent_ir_canonical_surface_corroboration"));
        assert!(snapshot_doc.contains("temporal_conflict_0001"));
        assert!(snapshot_doc.contains("recommended_commands"));
        assert!(snapshot_doc.contains("generated/semantic_ir/doc/semantic_ir.json"));
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
            "- validation_delta: fingerprint_changed `true`, score_delta `+5`, grade_changed `true`, finding_count_delta `-1`"
        ));
        assert!(snapshot_doc.contains("- finding_delta: added none; removed `finding_b`"));

        let live_projection =
            render_live_status_projection(&snapshots, repo_root, &recommendations);
        assert!(live_projection.contains("- Targeted rescan queue:"));
        assert!(live_projection.contains("intent_ir_canonical_surface_corroboration"));
        assert!(live_projection.contains("2 command hint(s)"));
        assert!(live_projection.contains(
            "- Rescan execution summaries: 1 total; 1 review required (possible improvement: 1, regression: 0, neutral change: 0); 0 no-change"
        ));
        assert!(live_projection.contains(
            "verdict `possible_improvement_review_required`, promotion `not_promoted_review_required`, score_delta `+5`, finding_count_delta `-1`"
        ));

        let plan_path = write_validation_rescan_plan(repo_root, recommendations)?;
        let plan = fs::read_to_string(plan_path)?;
        assert!(plan.contains("\"schema_version\": 2"));
        assert!(plan.contains("\"recommendation_count\": 1"));
        assert!(plan.contains("\"corroboration_policy\""));
        assert!(plan.contains("\"replay_inputs\""));
        assert!(plan.contains("\"recommended_commands\""));
        assert!(plan.contains("\"rebuild_intent_ir\""));

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

        summary.promotion_status = "promoted_without_review".to_string();
        summary.promotion_blockers.clear();
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
