use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::{
    EnrichArgs, EvidenceArgs, IngestArgs, IntentArgs, NlpEnrichArgs, RescanPlanArgs, SemanticArgs,
    ValidateArgs, VlmProviderArg,
};
use crate::commands::{enrich, evidence, ingest, intent, nlp_enrich, semantic, validate};
use crate::error::{AppError, Result};

use super::project_validation::{
    ProjectRescanCommandHint, ProjectRescanExecutionSummary, ProjectRescanPlanRecord,
    ProjectRescanRecommendation, ProjectRescanValidationDelta, ProjectRescanValidationSnapshot,
    rescan_promotion_blockers_for, rescan_promotion_status_for,
};

const SUPPORTED_RESCAN_PLAN_SCHEMA_VERSION: u32 = 2;
const PLANNED_NOT_EXECUTED: &str = "planned_not_executed";
const EXECUTED_VALIDATED_CHANGED: &str = "executed_validated_changed";
const EXECUTED_VALIDATED_NO_CHANGE: &str = "executed_validated_no_change";
const ARBITRATION_VALIDATED_NO_CHANGE: &str = "validated_no_change";
pub(crate) const ARBITRATION_POSSIBLE_IMPROVEMENT_REVIEW_REQUIRED: &str =
    "possible_improvement_review_required";
pub(crate) const ARBITRATION_REGRESSION_REVIEW_REQUIRED: &str = "regression_review_required";
pub(crate) const ARBITRATION_NEUTRAL_CHANGE_REVIEW_REQUIRED: &str =
    "neutral_change_review_required";

pub fn run(args: RescanPlanArgs) -> Result<()> {
    run_plan(args).map(|_| ())
}

pub(crate) fn run_plan(args: RescanPlanArgs) -> Result<RescanPlanRunReport> {
    let plan_path = args.plan;
    let mut plan = load_rescan_plan(&plan_path)?;
    let selected_indices =
        selected_pending_indices(&plan, args.limit, args.document_key.as_deref());
    let pending_recommendations =
        selected_pending_indices(&plan, 0, args.document_key.as_deref()).len();
    let mut report = RescanPlanRunReport {
        plan_path: plan_path.clone(),
        execute: args.execute,
        document_key_filter: args.document_key.clone(),
        schema_version: plan.schema_version,
        recommendation_count: plan.recommendation_count,
        pending_recommendations,
        selected_recommendations: selected_indices.len(),
        executed_validated_changed: 0,
        executed_validated_no_change: 0,
        execution_summaries: Vec::new(),
    };

    println!("command: rescan-plan");
    println!("mode: {}", if args.execute { "execute" } else { "dry-run" });
    println!("plan_path: {}", plan_path.display());
    println!(
        "document_key_filter: {}",
        args.document_key.as_deref().unwrap_or("all")
    );
    println!("schema_version: {}", plan.schema_version);
    println!("recommendation_count: {}", plan.recommendation_count);
    println!("pending_recommendations: {pending_recommendations}");
    println!("selected_recommendations: {}", selected_indices.len());

    if selected_indices.is_empty() {
        println!("rescan_queue: empty");
        return Ok(report);
    }

    if !args.execute {
        print_dry_run_plan(&plan, &selected_indices);
        return Ok(report);
    }

    for index in selected_indices {
        let recommendation = plan.recommendations[index].clone();
        let outcome = execute_recommendation(&recommendation, &args.prior_memory)?;
        match outcome.automation_status {
            EXECUTED_VALIDATED_CHANGED => report.executed_validated_changed += 1,
            EXECUTED_VALIDATED_NO_CHANGE => report.executed_validated_no_change += 1,
            _ => {}
        }
        plan.recommendations[index].automation_status = outcome.automation_status.to_string();
        plan.recommendations[index].execution_summary = Some(outcome.execution_summary.clone());
        report.execution_summaries.push(outcome.execution_summary);
    }

    fs::write(&plan_path, serde_json::to_string_pretty(&plan)?)?;
    println!("status_update_path: {}", plan_path.display());
    Ok(report)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct RescanPlanRunReport {
    pub(crate) plan_path: PathBuf,
    pub(crate) execute: bool,
    pub(crate) document_key_filter: Option<String>,
    pub(crate) schema_version: u32,
    pub(crate) recommendation_count: usize,
    pub(crate) pending_recommendations: usize,
    pub(crate) selected_recommendations: usize,
    pub(crate) executed_validated_changed: usize,
    pub(crate) executed_validated_no_change: usize,
    pub(crate) execution_summaries: Vec<ProjectRescanExecutionSummary>,
}

impl RescanPlanRunReport {
    pub(crate) fn review_required_count(&self) -> usize {
        self.execution_summaries
            .iter()
            .filter(|summary| summary.arbitration_verdict.ends_with("_review_required"))
            .count()
    }

    pub(crate) fn arbitration_verdict_count(&self, verdict: &str) -> usize {
        self.execution_summaries
            .iter()
            .filter(|summary| summary.arbitration_verdict == verdict)
            .count()
    }
}

fn load_rescan_plan(plan_path: &Path) -> Result<ProjectRescanPlanRecord> {
    if !plan_path.exists() {
        return Err(AppError::MissingPath(plan_path.to_path_buf()));
    }

    let plan: ProjectRescanPlanRecord = serde_json::from_str(&fs::read_to_string(plan_path)?)?;
    if plan.schema_version != SUPPORTED_RESCAN_PLAN_SCHEMA_VERSION {
        return Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan supports schema_version {SUPPORTED_RESCAN_PLAN_SCHEMA_VERSION}, got {} at {}",
            plan.schema_version,
            plan_path.display()
        )));
    }
    if plan.recommendation_count != plan.recommendations.len() {
        return Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan recommendation_count mismatch at {}: header says {}, recommendations len is {}",
            plan_path.display(),
            plan.recommendation_count,
            plan.recommendations.len()
        )));
    }

    Ok(plan)
}

fn selected_pending_indices(
    plan: &ProjectRescanPlanRecord,
    limit: usize,
    document_key: Option<&str>,
) -> Vec<usize> {
    let pending = plan
        .recommendations
        .iter()
        .enumerate()
        .filter_map(|(index, recommendation)| {
            let document_matches =
                document_key.is_none_or(|document_key| recommendation.document_key == document_key);
            (document_matches && recommendation.automation_status == PLANNED_NOT_EXECUTED)
                .then_some(index)
        });

    if limit == 0 {
        pending.collect()
    } else {
        pending.take(limit).collect()
    }
}

fn print_dry_run_plan(plan: &ProjectRescanPlanRecord, selected_indices: &[usize]) {
    println!("{}", render_dry_run_plan(plan, selected_indices));
}

fn render_dry_run_plan(plan: &ProjectRescanPlanRecord, selected_indices: &[usize]) -> String {
    let mut lines = vec!["rescan_queue:".to_string()];
    for index in selected_indices {
        let recommendation = &plan.recommendations[*index];
        lines.push(format!(
            "- {} {} {}",
            recommendation.document_key, recommendation.stage, recommendation.finding_id
        ));
        lines.push(format!("  artifact_path: {}", recommendation.artifact_path));
        lines.push(format!(
            "  replay_inputs: {}",
            render_replay_inputs(&recommendation.replay_inputs)
        ));
        lines.push(format!(
            "  extractor_lane: {}",
            recommendation.extractor_lane
        ));
        lines.push(format!(
            "  recommended_action: {}",
            recommendation.recommended_action
        ));
        lines.push(format!(
            "  related_ids: {}",
            render_string_list(&recommendation.related_ids)
        ));
        lines.push(format!(
            "  automation_status: {}",
            recommendation.automation_status
        ));
        if recommendation.recommended_commands.is_empty() {
            lines.push("  recommended_commands: none".to_string());
        } else {
            lines.push("  recommended_commands:".to_string());
            for command in &recommendation.recommended_commands {
                lines.push(format!("  - {}: {}", command.intent, command.display));
            }
        }
    }
    lines.join("\n")
}

fn render_replay_inputs(inputs: &[super::project_validation::ProjectRescanReplayInput]) -> String {
    if inputs.is_empty() {
        "none".to_string()
    } else {
        inputs
            .iter()
            .map(|input| format!("{}:{}", input.input_kind, input.path))
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn render_string_list(values: &[String]) -> String {
    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

fn execute_recommendation(
    recommendation: &ProjectRescanRecommendation,
    prior_memory: &Path,
) -> Result<RescanExecutionOutcome> {
    println!(
        "executing_recommendation: {} {} {}",
        recommendation.document_key, recommendation.stage, recommendation.finding_id
    );
    let artifact_path = PathBuf::from(&recommendation.artifact_path);
    let before = validate_and_snapshot(&artifact_path)?;
    println!(
        "before_validation: fingerprint={} score={} findings={}",
        before.artifact_fingerprint,
        score_label(before.overall_score, before.grade.as_deref()),
        before.finding_count
    );

    for command in &recommendation.recommended_commands {
        let invocation = parse_command_hint(command)?;
        println!("executing_command_hint: {}", command.intent);
        execute_invocation(invocation, prior_memory)?;
    }

    let after = validate_and_snapshot(&artifact_path)?;
    println!(
        "after_validation: fingerprint={} score={} findings={}",
        after.artifact_fingerprint,
        score_label(after.overall_score, after.grade.as_deref()),
        after.finding_count
    );

    let automation_status = execution_status(&before, &after);
    let validation_delta = validation_delta(&before, &after);
    let arbitration_verdict = arbitration_verdict(&validation_delta);
    println!("execution_status: {automation_status}");
    println!("arbitration_verdict: {arbitration_verdict}");

    Ok(RescanExecutionOutcome {
        automation_status,
        execution_summary: ProjectRescanExecutionSummary {
            automation_status: automation_status.to_string(),
            arbitration_verdict: arbitration_verdict.to_string(),
            promotion_status: rescan_promotion_status_for(arbitration_verdict).to_string(),
            promotion_blockers: rescan_promotion_blockers_for(arbitration_verdict),
            promotion_review: super::project_validation::rescan_promotion_review_for(
                arbitration_verdict,
            ),
            before_validation: before,
            after_validation: after,
            validation_delta,
        },
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RescanInvocation {
    Ingest(PathBuf),
    Enrich {
        source_ir: PathBuf,
        vlm_provider: VlmProviderArg,
        vlm_model: Option<String>,
        classify_only: bool,
    },
    NlpEnrich {
        evidence_ir: PathBuf,
        vlm_provider: VlmProviderArg,
        vlm_model: Option<String>,
    },
    Evidence(PathBuf),
    Semantic(PathBuf),
    Intent(PathBuf),
    Validate(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RescanExecutionOutcome {
    automation_status: &'static str,
    execution_summary: ProjectRescanExecutionSummary,
}

fn validate_and_snapshot(artifact_path: &Path) -> Result<ProjectRescanValidationSnapshot> {
    validate::run(ValidateArgs {
        artifact: artifact_path.to_path_buf(),
    })?;
    let report = read_validation_report(artifact_path)?;
    let mut finding_ids = report
        .findings
        .iter()
        .map(|finding| finding.finding_id.clone())
        .collect::<Vec<_>>();
    finding_ids.sort();
    finding_ids.dedup();
    Ok(ProjectRescanValidationSnapshot {
        artifact_fingerprint: report.artifact_fingerprint,
        overall_score: report.overall_score,
        grade: report.grade,
        finding_count: report.findings.len(),
        finding_ids,
    })
}

fn read_validation_report(
    artifact_path: &Path,
) -> Result<crate::ir::source::ValidationReportRecord> {
    let report_path = validation_report_path_for(artifact_path)?;
    if !report_path.exists() {
        return Err(AppError::MissingPath(report_path));
    }

    Ok(serde_json::from_str(&fs::read_to_string(report_path)?)?)
}

fn validation_report_path_for(artifact_path: &Path) -> Result<PathBuf> {
    let artifact_dir = artifact_path.parent().ok_or_else(|| {
        AppError::InvalidStageArtifact(format!(
            "cannot derive validation report path for {}",
            artifact_path.display()
        ))
    })?;
    Ok(artifact_dir.join("validation_report.json"))
}

fn execution_status(
    before: &ProjectRescanValidationSnapshot,
    after: &ProjectRescanValidationSnapshot,
) -> &'static str {
    if before == after {
        EXECUTED_VALIDATED_NO_CHANGE
    } else {
        EXECUTED_VALIDATED_CHANGED
    }
}

fn validation_delta(
    before: &ProjectRescanValidationSnapshot,
    after: &ProjectRescanValidationSnapshot,
) -> ProjectRescanValidationDelta {
    let mut before_ids = before.finding_ids.clone();
    before_ids.sort();
    before_ids.dedup();
    let mut after_ids = after.finding_ids.clone();
    after_ids.sort();
    after_ids.dedup();

    let added_findings = after_ids
        .iter()
        .filter(|finding_id| !before_ids.contains(finding_id))
        .cloned()
        .collect();
    let removed_findings = before_ids
        .iter()
        .filter(|finding_id| !after_ids.contains(finding_id))
        .cloned()
        .collect();

    ProjectRescanValidationDelta {
        fingerprint_changed: before.artifact_fingerprint != after.artifact_fingerprint,
        score_changed: before.overall_score != after.overall_score,
        score_delta: match (before.overall_score, after.overall_score) {
            (Some(before), Some(after)) => Some(after as i32 - before as i32),
            _ => None,
        },
        grade_changed: before.grade != after.grade,
        finding_count_delta: after.finding_count as i64 - before.finding_count as i64,
        added_findings,
        removed_findings,
    }
}

fn arbitration_verdict(delta: &ProjectRescanValidationDelta) -> &'static str {
    if !delta.fingerprint_changed
        && !delta.score_changed
        && delta.score_delta.unwrap_or(0) == 0
        && !delta.grade_changed
        && delta.finding_count_delta == 0
        && delta.added_findings.is_empty()
        && delta.removed_findings.is_empty()
    {
        return ARBITRATION_VALIDATED_NO_CHANGE;
    }

    if delta.score_delta.is_some_and(|score_delta| score_delta < 0)
        || delta.finding_count_delta > 0
        || !delta.added_findings.is_empty()
    {
        return ARBITRATION_REGRESSION_REVIEW_REQUIRED;
    }

    if delta.score_delta.is_some_and(|score_delta| score_delta > 0)
        || delta.finding_count_delta < 0
        || !delta.removed_findings.is_empty()
    {
        return ARBITRATION_POSSIBLE_IMPROVEMENT_REVIEW_REQUIRED;
    }

    ARBITRATION_NEUTRAL_CHANGE_REVIEW_REQUIRED
}

fn score_label(score: Option<u32>, grade: Option<&str>) -> String {
    match (score, grade) {
        (Some(score), Some(grade)) => format!("{score}/100 {grade}"),
        (Some(score), None) => format!("{score}/100"),
        (None, Some(grade)) => grade.to_string(),
        (None, None) => "n/a".to_string(),
    }
}

fn parse_command_hint(command: &ProjectRescanCommandHint) -> Result<RescanInvocation> {
    if command.executable != "cargo" {
        return Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan refuses non-cargo executable `{}` for command intent `{}`",
            command.executable, command.intent
        )));
    }
    if command.working_directory != "." {
        return Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan refuses non-repository working directory `{}` for command intent `{}`",
            command.working_directory, command.intent
        )));
    }

    let specforge_args = specforge_args_from_cargo_hint(command)?;
    match (command.intent.as_str(), specforge_args) {
        ("rebuild_source_ir", [subcommand, source]) if subcommand == "ingest" => {
            Ok(RescanInvocation::Ingest(PathBuf::from(source)))
        }
        ("enrich_source_ir", args) => parse_enrich_command_hint_args(args),
        ("nlp_enrich_evidence_ir", args) => parse_nlp_enrich_command_hint_args(args),
        ("rebuild_evidence_ir", [subcommand, source_ir]) if subcommand == "evidence" => {
            Ok(RescanInvocation::Evidence(PathBuf::from(source_ir)))
        }
        ("rebuild_semantic_ir", [subcommand, evidence_ir]) if subcommand == "semantic" => {
            Ok(RescanInvocation::Semantic(PathBuf::from(evidence_ir)))
        }
        ("rebuild_intent_ir", [subcommand, semantic_ir]) if subcommand == "intent" => {
            Ok(RescanInvocation::Intent(PathBuf::from(semantic_ir)))
        }
        ("validate_current_artifact", [subcommand, artifact]) if subcommand == "validate" => {
            Ok(RescanInvocation::Validate(PathBuf::from(artifact)))
        }
        _ => Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan refuses unsupported command hint `{}` with args {:?}",
            command.intent, command.args
        ))),
    }
}

fn specforge_args_from_cargo_hint(command: &ProjectRescanCommandHint) -> Result<&[String]> {
    let expected_prefix = ["run", "--manifest-path", "Cargo.toml", "--"];
    if command.args.len() <= expected_prefix.len() {
        return Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan command hint `{}` is missing specforge args",
            command.intent
        )));
    }

    for (actual, expected) in command.args.iter().zip(expected_prefix) {
        if actual != expected {
            return Err(AppError::InvalidStageArtifact(format!(
                "rescan-plan refuses command hint `{}` with non-standard cargo prefix {:?}",
                command.intent, command.args
            )));
        }
    }

    Ok(&command.args[expected_prefix.len()..])
}

fn parse_enrich_command_hint_args(args: &[String]) -> Result<RescanInvocation> {
    if args.len() < 2 || args[0] != "enrich" {
        return Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan refuses malformed enrich command hint args {args:?}"
        )));
    }

    let source_ir = PathBuf::from(&args[1]);
    let mut vlm_provider = None;
    let mut vlm_model = None;
    let mut classify_only = false;
    let mut index = 2;
    while index < args.len() {
        match args[index].as_str() {
            "--vlm-provider" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(AppError::InvalidStageArtifact(
                        "rescan-plan enrich hint is missing --vlm-provider value".to_string(),
                    ));
                };
                if vlm_provider
                    .replace(parse_local_rescan_vlm_provider(value)?)
                    .is_some()
                {
                    return Err(AppError::InvalidStageArtifact(
                        "rescan-plan enrich hint repeats --vlm-provider".to_string(),
                    ));
                }
                index += 2;
            }
            "--vlm-model" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(AppError::InvalidStageArtifact(
                        "rescan-plan enrich hint is missing --vlm-model value".to_string(),
                    ));
                };
                if vlm_model
                    .replace(parse_vlm_model_hint_value(value, "enrich")?)
                    .is_some()
                {
                    return Err(AppError::InvalidStageArtifact(
                        "rescan-plan enrich hint repeats --vlm-model".to_string(),
                    ));
                }
                index += 2;
            }
            "--classify-only" => {
                if classify_only {
                    return Err(AppError::InvalidStageArtifact(
                        "rescan-plan enrich hint repeats --classify-only".to_string(),
                    ));
                }
                classify_only = true;
                index += 1;
            }
            other => {
                return Err(AppError::InvalidStageArtifact(format!(
                    "rescan-plan refuses unsupported enrich hint arg `{other}` in {args:?}"
                )));
            }
        }
    }

    let vlm_provider = vlm_provider.ok_or_else(|| {
        AppError::InvalidStageArtifact(
            "rescan-plan enrich hint requires explicit --vlm-provider".to_string(),
        )
    })?;

    Ok(RescanInvocation::Enrich {
        source_ir,
        vlm_provider,
        vlm_model,
        classify_only,
    })
}

fn parse_nlp_enrich_command_hint_args(args: &[String]) -> Result<RescanInvocation> {
    if args.len() < 2 || args[0] != "nlp-enrich" {
        return Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan refuses malformed nlp-enrich command hint args {args:?}"
        )));
    }

    let evidence_ir = PathBuf::from(&args[1]);
    let mut vlm_provider = None;
    let mut vlm_model = None;
    let mut index = 2;
    while index < args.len() {
        match args[index].as_str() {
            "--vlm-provider" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(AppError::InvalidStageArtifact(
                        "rescan-plan nlp-enrich hint is missing --vlm-provider value".to_string(),
                    ));
                };
                if vlm_provider
                    .replace(parse_local_rescan_vlm_provider(value)?)
                    .is_some()
                {
                    return Err(AppError::InvalidStageArtifact(
                        "rescan-plan nlp-enrich hint repeats --vlm-provider".to_string(),
                    ));
                }
                index += 2;
            }
            "--vlm-model" => {
                let Some(value) = args.get(index + 1) else {
                    return Err(AppError::InvalidStageArtifact(
                        "rescan-plan nlp-enrich hint is missing --vlm-model value".to_string(),
                    ));
                };
                if vlm_model
                    .replace(parse_vlm_model_hint_value(value, "nlp-enrich")?)
                    .is_some()
                {
                    return Err(AppError::InvalidStageArtifact(
                        "rescan-plan nlp-enrich hint repeats --vlm-model".to_string(),
                    ));
                }
                index += 2;
            }
            other => {
                return Err(AppError::InvalidStageArtifact(format!(
                    "rescan-plan refuses unsupported nlp-enrich hint arg `{other}` in {args:?}"
                )));
            }
        }
    }

    let vlm_provider = vlm_provider.ok_or_else(|| {
        AppError::InvalidStageArtifact(
            "rescan-plan nlp-enrich hint requires explicit --vlm-provider".to_string(),
        )
    })?;

    Ok(RescanInvocation::NlpEnrich {
        evidence_ir,
        vlm_provider,
        vlm_model,
    })
}

fn parse_vlm_model_hint_value(value: &str, hint_kind: &str) -> Result<String> {
    if value.starts_with("--") {
        return Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan {hint_kind} hint is missing --vlm-model value"
        )));
    }

    Ok(value.to_string())
}

fn parse_local_rescan_vlm_provider(value: &str) -> Result<VlmProviderArg> {
    match value {
        "ollama" => Ok(VlmProviderArg::Ollama),
        "lmstudio" | "lm-studio" => Ok(VlmProviderArg::LmStudio),
        "skip" => Ok(VlmProviderArg::Skip),
        "openai" | "open-ai" => Err(AppError::InvalidStageArtifact(
            "rescan-plan enrich hints intentionally reject OpenAI provider; use local ollama/lmstudio/skip for replay"
                .to_string(),
        )),
        other => Err(AppError::InvalidStageArtifact(format!(
            "rescan-plan refuses unsupported VLM provider `{other}` in enrich hint"
        ))),
    }
}

fn execute_invocation(invocation: RescanInvocation, prior_memory: &Path) -> Result<()> {
    match invocation {
        RescanInvocation::Ingest(source) => ingest::run(IngestArgs {
            source,
            dry_run: false,
        }),
        RescanInvocation::Enrich {
            source_ir,
            vlm_provider,
            vlm_model,
            classify_only,
        } => enrich::run(EnrichArgs {
            source_ir,
            vlm_provider,
            vlm_model,
            classify_only,
            dry_run: false,
        }),
        RescanInvocation::NlpEnrich {
            evidence_ir,
            vlm_provider,
            vlm_model,
        } => nlp_enrich::run(NlpEnrichArgs {
            evidence_ir,
            vlm_provider,
            vlm_model,
            dry_run: false,
            max_sentences: 0,
            grounding_signals: None,
        }),
        RescanInvocation::Evidence(source_ir) => evidence::run(EvidenceArgs {
            source_ir,
            prior_memory: prior_memory.to_path_buf(),
            dry_run: false,
        }),
        RescanInvocation::Semantic(evidence_ir) => semantic::run(SemanticArgs {
            evidence_ir,
            dry_run: false,
        }),
        RescanInvocation::Intent(semantic_ir) => intent::run(IntentArgs {
            semantic_ir,
            dry_run: false,
        }),
        RescanInvocation::Validate(artifact) => validate::run(ValidateArgs { artifact }),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use super::*;
    use crate::commands::project_validation::{
        ProjectRescanCommandHint, ProjectRescanPlanRecord, ProjectRescanRecommendation,
        ProjectRescanReplayInput, RESCAN_PROMOTION_NOT_PROMOTED_NO_CHANGE,
        RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED,
    };
    use crate::ir::source::SourceIr;

    #[test]
    fn rescan_plan_cli_dry_run_accepts_empty_schema_v2_plan() -> Result<()> {
        let tempdir = tempdir()?;
        let plan_path = tempdir.path().join("rescan_plan.json");
        let plan = ProjectRescanPlanRecord {
            schema_version: 2,
            generated_by: "test".to_string(),
            recommendation_count: 0,
            recommendations: Vec::new(),
        };
        fs::write(&plan_path, serde_json::to_string_pretty(&plan)?)?;

        run(RescanPlanArgs {
            plan: plan_path,
            execute: false,
            limit: 0,
            document_key: None,
            prior_memory: PathBuf::from("generated/prior_memory/corpus_memory.json"),
        })?;

        Ok(())
    }

    #[test]
    fn rescan_plan_rejects_untrusted_command_hints() {
        let command = ProjectRescanCommandHint {
            intent: "rebuild_intent_ir".to_string(),
            executable: "rm".to_string(),
            args: vec!["-rf".to_string(), ".".to_string()],
            working_directory: ".".to_string(),
            display: "rm -rf .".to_string(),
        };

        assert!(parse_command_hint(&command).is_err());
    }

    #[test]
    fn rescan_plan_rejects_unknown_command_intents() {
        let unknown_validation_intent = command_hint(
            "validate_generated_artifact",
            vec!["validate", "generated/intent_ir/doc/intent_ir.json"],
        );
        assert!(parse_command_hint(&unknown_validation_intent).is_err());

        let unknown_rebuild_intent = command_hint(
            "rebuild_adapter_ir",
            vec![
                "adapt",
                "generated/intent_ir/doc/intent_ir.json",
                "--target",
                "fsm",
            ],
        );
        assert!(parse_command_hint(&unknown_rebuild_intent).is_err());
    }

    #[test]
    fn rescan_plan_rejects_non_repository_cargo_hints() {
        let mut wrong_workdir = command_hint(
            "rebuild_intent_ir",
            vec!["intent", "generated/semantic_ir/doc/semantic_ir.json"],
        );
        wrong_workdir.working_directory = "generated".to_string();
        assert!(parse_command_hint(&wrong_workdir).is_err());

        let mut wrong_manifest = command_hint(
            "rebuild_intent_ir",
            vec!["intent", "generated/semantic_ir/doc/semantic_ir.json"],
        );
        wrong_manifest.args[2] = "crates/specforge/Cargo.toml".to_string();
        assert!(parse_command_hint(&wrong_manifest).is_err());

        let mut missing_specforge_args = command_hint(
            "rebuild_intent_ir",
            vec!["intent", "generated/semantic_ir/doc/semantic_ir.json"],
        );
        missing_specforge_args.args.truncate(4);
        assert!(parse_command_hint(&missing_specforge_args).is_err());
    }

    #[test]
    fn rescan_plan_rejects_malformed_cargo_prefix_tokens() {
        let mut wrong_cargo_subcommand = command_hint(
            "rebuild_intent_ir",
            vec!["intent", "generated/semantic_ir/doc/semantic_ir.json"],
        );
        wrong_cargo_subcommand.args[0] = "test".to_string();
        assert!(parse_command_hint(&wrong_cargo_subcommand).is_err());

        let mut missing_separator = command_hint(
            "rebuild_intent_ir",
            vec!["intent", "generated/semantic_ir/doc/semantic_ir.json"],
        );
        missing_separator.args[3] = "intent".to_string();
        assert!(parse_command_hint(&missing_separator).is_err());
    }

    #[test]
    fn rescan_plan_rejects_inconsistent_plan_count() -> Result<()> {
        let tempdir = tempdir()?;
        let plan_path = tempdir.path().join("rescan_plan.json");
        let plan = ProjectRescanPlanRecord {
            schema_version: 2,
            generated_by: "test".to_string(),
            recommendation_count: 1,
            recommendations: Vec::new(),
        };
        fs::write(&plan_path, serde_json::to_string_pretty(&plan)?)?;

        assert!(load_rescan_plan(&plan_path).is_err());
        Ok(())
    }

    #[test]
    fn rescan_plan_execute_marks_validated_no_change() -> Result<()> {
        let tempdir = tempdir()?;
        let source_path = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source_path, "# Spec\nSignal READY is input width 1.\n")?;
        let source_ir = SourceIr::build(&source_path, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        let artifact_path = source_ir.artifact_layout.source_ir_path.clone();
        let plan_path = tempdir.path().join("rescan_plan.json");
        let mut plan = ProjectRescanPlanRecord {
            schema_version: 2,
            generated_by: "test".to_string(),
            recommendation_count: 1,
            recommendations: vec![recommendation("doc", PLANNED_NOT_EXECUTED)],
        };
        plan.recommendations[0].artifact_path = artifact_path.display().to_string();
        plan.recommendations[0].recommended_commands = vec![command_hint(
            "validate_current_artifact",
            vec!["validate", artifact_path.to_str().unwrap()],
        )];
        fs::write(&plan_path, serde_json::to_string_pretty(&plan)?)?;

        run(RescanPlanArgs {
            plan: plan_path.clone(),
            execute: true,
            limit: 1,
            document_key: None,
            prior_memory: PathBuf::from("generated/prior_memory/corpus_memory.json"),
        })?;

        let updated: ProjectRescanPlanRecord =
            serde_json::from_str(&fs::read_to_string(plan_path)?)?;
        assert_eq!(
            updated.recommendations[0].automation_status,
            EXECUTED_VALIDATED_NO_CHANGE
        );
        let execution_summary = updated.recommendations[0]
            .execution_summary
            .as_ref()
            .expect("execution summary");
        assert_eq!(
            execution_summary.arbitration_verdict,
            ARBITRATION_VALIDATED_NO_CHANGE
        );
        assert_eq!(
            execution_summary.promotion_status,
            RESCAN_PROMOTION_NOT_PROMOTED_NO_CHANGE
        );
        assert!(
            execution_summary
                .promotion_blockers
                .contains(&"canonical_ir_not_mutated_by_rescan_plan".to_string())
        );
        assert_eq!(
            execution_summary.promotion_review.review_status,
            crate::commands::project_validation::RESCAN_PROMOTION_REVIEW_NOT_REVIEWABLE_NO_CHANGE
        );
        assert!(!execution_summary.promotion_review.approval_record_required);
        assert!(
            !execution_summary
                .promotion_review
                .canonical_mutation_allowed
        );
        assert_eq!(execution_summary.before_validation.finding_count, 0);
        assert_eq!(execution_summary.after_validation.finding_count, 0);

        Ok(())
    }

    #[test]
    fn rescan_plan_parses_whitelisted_intent_rebuild_hint() -> Result<()> {
        let command = command_hint(
            "rebuild_intent_ir",
            vec!["intent", "generated/semantic_ir/doc/semantic_ir.json"],
        );

        assert_eq!(
            parse_command_hint(&command)?,
            RescanInvocation::Intent(PathBuf::from("generated/semantic_ir/doc/semantic_ir.json"))
        );

        Ok(())
    }

    #[test]
    fn rescan_plan_ignores_display_string_for_execution_parsing() -> Result<()> {
        let mut command = command_hint(
            "rebuild_intent_ir",
            vec!["intent", "generated/semantic_ir/doc/semantic_ir.json"],
        );
        command.display = "rm -rf .".to_string();

        assert_eq!(
            parse_command_hint(&command)?,
            RescanInvocation::Intent(PathBuf::from("generated/semantic_ir/doc/semantic_ir.json"))
        );

        Ok(())
    }

    #[test]
    fn rescan_plan_parses_whitelisted_stage_command_hints() -> Result<()> {
        let ingest = command_hint("rebuild_source_ir", vec!["ingest", "specs/doc.md"]);
        assert_eq!(
            parse_command_hint(&ingest)?,
            RescanInvocation::Ingest(PathBuf::from("specs/doc.md"))
        );

        let evidence = command_hint(
            "rebuild_evidence_ir",
            vec!["evidence", "generated/source_ir/doc/source_ir.json"],
        );
        assert_eq!(
            parse_command_hint(&evidence)?,
            RescanInvocation::Evidence(PathBuf::from("generated/source_ir/doc/source_ir.json"))
        );

        let semantic = command_hint(
            "rebuild_semantic_ir",
            vec!["semantic", "generated/evidence_ir/doc/evidence_ir.json"],
        );
        assert_eq!(
            parse_command_hint(&semantic)?,
            RescanInvocation::Semantic(PathBuf::from("generated/evidence_ir/doc/evidence_ir.json"))
        );

        let validate = command_hint(
            "validate_current_artifact",
            vec!["validate", "generated/intent_ir/doc/intent_ir.json"],
        );
        assert_eq!(
            parse_command_hint(&validate)?,
            RescanInvocation::Validate(PathBuf::from("generated/intent_ir/doc/intent_ir.json"))
        );

        Ok(())
    }

    #[test]
    fn rescan_plan_rejects_extra_stage_command_args() {
        let ingest_extra = command_hint(
            "rebuild_source_ir",
            vec!["ingest", "specs/doc.md", "--dry-run"],
        );
        assert!(parse_command_hint(&ingest_extra).is_err());

        let evidence_extra = command_hint(
            "rebuild_evidence_ir",
            vec![
                "evidence",
                "generated/source_ir/doc/source_ir.json",
                "--dry-run",
            ],
        );
        assert!(parse_command_hint(&evidence_extra).is_err());

        let semantic_extra = command_hint(
            "rebuild_semantic_ir",
            vec![
                "semantic",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--dry-run",
            ],
        );
        assert!(parse_command_hint(&semantic_extra).is_err());

        let validate_extra = command_hint(
            "validate_current_artifact",
            vec![
                "validate",
                "generated/intent_ir/doc/intent_ir.json",
                "--strict",
            ],
        );
        assert!(parse_command_hint(&validate_extra).is_err());
    }

    #[test]
    fn rescan_plan_rejects_missing_stage_command_paths() {
        let ingest_missing_path = command_hint("rebuild_source_ir", vec!["ingest"]);
        assert!(parse_command_hint(&ingest_missing_path).is_err());

        let evidence_missing_artifact = command_hint("rebuild_evidence_ir", vec!["evidence"]);
        assert!(parse_command_hint(&evidence_missing_artifact).is_err());

        let semantic_missing_artifact = command_hint("rebuild_semantic_ir", vec!["semantic"]);
        assert!(parse_command_hint(&semantic_missing_artifact).is_err());

        let validate_missing_artifact = command_hint("validate_current_artifact", vec!["validate"]);
        assert!(parse_command_hint(&validate_missing_artifact).is_err());
    }

    #[test]
    fn rescan_plan_rejects_command_intent_subcommand_mismatches() {
        let evidence_intent_with_semantic_subcommand = command_hint(
            "rebuild_evidence_ir",
            vec!["semantic", "generated/evidence_ir/doc/evidence_ir.json"],
        );
        assert!(parse_command_hint(&evidence_intent_with_semantic_subcommand).is_err());

        let validate_intent_with_intent_subcommand = command_hint(
            "validate_current_artifact",
            vec!["intent", "generated/semantic_ir/doc/semantic_ir.json"],
        );
        assert!(parse_command_hint(&validate_intent_with_intent_subcommand).is_err());
    }

    #[test]
    fn rescan_plan_parses_whitelisted_local_enrich_hint() -> Result<()> {
        let command = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "ollama",
                "--vlm-model",
                "qwen2.5vl:7b",
            ],
        );

        assert_eq!(
            parse_command_hint(&command)?,
            RescanInvocation::Enrich {
                source_ir: PathBuf::from("generated/source_ir/doc/source_ir.json"),
                vlm_provider: VlmProviderArg::Ollama,
                vlm_model: Some("qwen2.5vl:7b".to_string()),
                classify_only: false,
            }
        );

        Ok(())
    }

    #[test]
    fn rescan_plan_parses_whitelisted_local_nlp_enrich_hint() -> Result<()> {
        let command = command_hint(
            "nlp_enrich_evidence_ir",
            vec![
                "nlp-enrich",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--vlm-provider",
                "ollama",
                "--vlm-model",
                "qwen2.5vl:7b",
            ],
        );

        assert_eq!(
            parse_command_hint(&command)?,
            RescanInvocation::NlpEnrich {
                evidence_ir: PathBuf::from("generated/evidence_ir/doc/evidence_ir.json"),
                vlm_provider: VlmProviderArg::Ollama,
                vlm_model: Some("qwen2.5vl:7b".to_string()),
            }
        );

        Ok(())
    }

    #[test]
    fn rescan_plan_parses_whitelisted_lmstudio_and_skip_hints() -> Result<()> {
        let enrich = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "lm-studio",
                "--classify-only",
            ],
        );
        assert_eq!(
            parse_command_hint(&enrich)?,
            RescanInvocation::Enrich {
                source_ir: PathBuf::from("generated/source_ir/doc/source_ir.json"),
                vlm_provider: VlmProviderArg::LmStudio,
                vlm_model: None,
                classify_only: true,
            }
        );

        let nlp_enrich = command_hint(
            "nlp_enrich_evidence_ir",
            vec![
                "nlp-enrich",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--vlm-provider",
                "skip",
            ],
        );
        assert_eq!(
            parse_command_hint(&nlp_enrich)?,
            RescanInvocation::NlpEnrich {
                evidence_ir: PathBuf::from("generated/evidence_ir/doc/evidence_ir.json"),
                vlm_provider: VlmProviderArg::Skip,
                vlm_model: None,
            }
        );

        Ok(())
    }

    #[test]
    fn rescan_plan_rejects_malformed_local_provider_hints() {
        let duplicate_provider = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "ollama",
                "--vlm-provider",
                "skip",
            ],
        );
        assert!(parse_command_hint(&duplicate_provider).is_err());

        let missing_provider_value = command_hint(
            "nlp_enrich_evidence_ir",
            vec![
                "nlp-enrich",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--vlm-provider",
            ],
        );
        assert!(parse_command_hint(&missing_provider_value).is_err());

        let unsupported_arg = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "ollama",
                "--unexpected",
            ],
        );
        assert!(parse_command_hint(&unsupported_arg).is_err());
    }

    #[test]
    fn rescan_plan_rejects_duplicate_model_and_classify_flags() {
        let duplicate_enrich_model = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "ollama",
                "--vlm-model",
                "model-a",
                "--vlm-model",
                "model-b",
            ],
        );
        assert!(parse_command_hint(&duplicate_enrich_model).is_err());

        let duplicate_classify_only = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "ollama",
                "--classify-only",
                "--classify-only",
            ],
        );
        assert!(parse_command_hint(&duplicate_classify_only).is_err());

        let duplicate_nlp_model = command_hint(
            "nlp_enrich_evidence_ir",
            vec![
                "nlp-enrich",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--vlm-provider",
                "ollama",
                "--vlm-model",
                "model-a",
                "--vlm-model",
                "model-b",
            ],
        );
        assert!(parse_command_hint(&duplicate_nlp_model).is_err());
    }

    #[test]
    fn rescan_plan_rejects_missing_model_values() {
        let missing_enrich_model = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "ollama",
                "--vlm-model",
            ],
        );
        assert!(parse_command_hint(&missing_enrich_model).is_err());

        let missing_nlp_model = command_hint(
            "nlp_enrich_evidence_ir",
            vec![
                "nlp-enrich",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--vlm-provider",
                "ollama",
                "--vlm-model",
            ],
        );
        assert!(parse_command_hint(&missing_nlp_model).is_err());
    }

    #[test]
    fn rescan_plan_rejects_flag_shaped_model_values() {
        let flag_shaped_enrich_model = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "ollama",
                "--vlm-model",
                "--classify-only",
            ],
        );
        assert!(parse_command_hint(&flag_shaped_enrich_model).is_err());

        let flag_shaped_nlp_model = command_hint(
            "nlp_enrich_evidence_ir",
            vec![
                "nlp-enrich",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--vlm-provider",
                "ollama",
                "--vlm-model",
                "--vlm-provider",
            ],
        );
        assert!(parse_command_hint(&flag_shaped_nlp_model).is_err());
    }

    #[test]
    fn rescan_plan_rejects_missing_explicit_provider_hints() {
        let missing_enrich_provider = command_hint(
            "enrich_source_ir",
            vec!["enrich", "generated/source_ir/doc/source_ir.json"],
        );
        assert!(parse_command_hint(&missing_enrich_provider).is_err());

        let missing_nlp_provider = command_hint(
            "nlp_enrich_evidence_ir",
            vec!["nlp-enrich", "generated/evidence_ir/doc/evidence_ir.json"],
        );
        assert!(parse_command_hint(&missing_nlp_provider).is_err());
    }

    #[test]
    fn rescan_plan_rejects_flag_shaped_provider_values() {
        let flag_shaped_enrich_provider = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "--vlm-model",
                "model-a",
            ],
        );
        assert!(parse_command_hint(&flag_shaped_enrich_provider).is_err());

        let flag_shaped_nlp_provider = command_hint(
            "nlp_enrich_evidence_ir",
            vec![
                "nlp-enrich",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--vlm-provider",
                "--vlm-model",
                "model-a",
            ],
        );
        assert!(parse_command_hint(&flag_shaped_nlp_provider).is_err());
    }

    #[test]
    fn rescan_plan_rejects_unknown_local_provider_values() {
        let unknown_enrich_provider = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "remote-gpu",
            ],
        );
        assert!(parse_command_hint(&unknown_enrich_provider).is_err());

        let unknown_nlp_provider = command_hint(
            "nlp_enrich_evidence_ir",
            vec![
                "nlp-enrich",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--vlm-provider",
                "remote-gpu",
            ],
        );
        assert!(parse_command_hint(&unknown_nlp_provider).is_err());
    }

    #[test]
    fn rescan_plan_rejects_openai_enrich_hints() {
        let command = command_hint(
            "enrich_source_ir",
            vec![
                "enrich",
                "generated/source_ir/doc/source_ir.json",
                "--vlm-provider",
                "openai",
            ],
        );

        assert!(parse_command_hint(&command).is_err());
    }

    #[test]
    fn rescan_plan_rejects_openai_nlp_enrich_hints() {
        let command = command_hint(
            "nlp_enrich_evidence_ir",
            vec![
                "nlp-enrich",
                "generated/evidence_ir/doc/evidence_ir.json",
                "--vlm-provider",
                "openai",
            ],
        );

        assert!(parse_command_hint(&command).is_err());
    }

    #[test]
    fn rescan_plan_selects_only_planned_pending_targets_with_limit() {
        let plan = ProjectRescanPlanRecord {
            schema_version: 2,
            generated_by: "test".to_string(),
            recommendation_count: 3,
            recommendations: vec![
                recommendation("doc_a", PLANNED_NOT_EXECUTED),
                recommendation("doc_b", EXECUTED_VALIDATED_NO_CHANGE),
                recommendation("doc_c", PLANNED_NOT_EXECUTED),
            ],
        };

        assert_eq!(selected_pending_indices(&plan, 1, None), vec![0]);
        assert_eq!(selected_pending_indices(&plan, 0, None), vec![0, 2]);
        assert_eq!(selected_pending_indices(&plan, 0, Some("doc_c")), vec![2]);
        assert_eq!(
            selected_pending_indices(&plan, 0, Some("doc_b")),
            Vec::<usize>::new()
        );
        assert_eq!(
            selected_pending_indices(&plan, 0, Some("missing_doc")),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn rescan_plan_applies_limit_after_document_key_filtering() {
        let plan = ProjectRescanPlanRecord {
            schema_version: 2,
            generated_by: "test".to_string(),
            recommendation_count: 4,
            recommendations: vec![
                recommendation("doc_a", PLANNED_NOT_EXECUTED),
                recommendation("doc_b", PLANNED_NOT_EXECUTED),
                recommendation("doc_target", PLANNED_NOT_EXECUTED),
                recommendation("doc_target", PLANNED_NOT_EXECUTED),
            ],
        };

        assert_eq!(
            selected_pending_indices(&plan, 1, Some("doc_target")),
            vec![2]
        );
        assert_eq!(
            selected_pending_indices(&plan, 2, Some("doc_target")),
            vec![2, 3]
        );
    }

    #[test]
    fn rescan_plan_dry_run_render_surfaces_replay_boundary_and_action() {
        let mut plan = ProjectRescanPlanRecord {
            schema_version: 2,
            generated_by: "test".to_string(),
            recommendation_count: 1,
            recommendations: vec![recommendation("doc", PLANNED_NOT_EXECUTED)],
        };
        plan.recommendations[0].replay_inputs = vec![
            ProjectRescanReplayInput {
                input_kind: "source_ir".to_string(),
                path: "generated/source_ir/doc/source_ir.json".to_string(),
            },
            ProjectRescanReplayInput {
                input_kind: "evidence_ir".to_string(),
                path: "generated/evidence_ir/doc/evidence_ir.json".to_string(),
            },
            ProjectRescanReplayInput {
                input_kind: "semantic_ir".to_string(),
                path: "generated/semantic_ir/doc/semantic_ir.json".to_string(),
            },
        ];
        plan.recommendations[0].recommended_action =
            "restart from SourceIR through EvidenceIR, SemanticIR, and IntentIR, then validate whether the related canonical conflict or residual ids still reproduce from current-document evidence"
                .to_string();

        let rendered = render_dry_run_plan(&plan, &[0]);

        assert!(rendered.contains("rescan_queue:"));
        assert!(rendered.contains("artifact_path: generated/intent_ir/doc/intent_ir.json"));
        assert!(rendered.contains("extractor_lane: intent_ir_canonical_surface_corroboration"));
        assert!(rendered.contains(
            "replay_inputs: source_ir:generated/source_ir/doc/source_ir.json, evidence_ir:generated/evidence_ir/doc/evidence_ir.json, semantic_ir:generated/semantic_ir/doc/semantic_ir.json"
        ));
        assert!(rendered.contains(
            "recommended_action: restart from SourceIR through EvidenceIR, SemanticIR, and IntentIR"
        ));
        assert!(rendered.contains("related_ids: temporal_conflict_0001"));
        assert!(rendered.contains("automation_status: planned_not_executed"));
        assert!(rendered.contains("recommended_commands:"));
        assert!(rendered.contains(
            "  - validate_current_artifact: cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/doc/semantic_ir.json"
        ));
    }

    #[test]
    fn rescan_plan_dry_run_render_surfaces_empty_replay_fields_as_none() {
        let mut plan = ProjectRescanPlanRecord {
            schema_version: 2,
            generated_by: "test".to_string(),
            recommendation_count: 1,
            recommendations: vec![recommendation("doc", PLANNED_NOT_EXECUTED)],
        };
        plan.recommendations[0].replay_inputs.clear();
        plan.recommendations[0].related_ids.clear();
        plan.recommendations[0].recommended_commands.clear();

        let rendered = render_dry_run_plan(&plan, &[0]);

        assert!(rendered.contains("replay_inputs: none"));
        assert!(rendered.contains("related_ids: none"));
        assert!(rendered.contains("recommended_commands: none"));
    }

    #[test]
    fn rescan_plan_dry_run_render_uses_command_display_for_review() {
        let mut plan = ProjectRescanPlanRecord {
            schema_version: 2,
            generated_by: "test".to_string(),
            recommendation_count: 1,
            recommendations: vec![recommendation("doc", PLANNED_NOT_EXECUTED)],
        };
        plan.recommendations[0].recommended_commands[0].display =
            "review-visible command text".to_string();

        let rendered = render_dry_run_plan(&plan, &[0]);

        assert!(rendered.contains("  - validate_current_artifact: review-visible command text"));
    }

    #[test]
    fn rescan_plan_execution_status_tracks_validation_deltas() {
        let before = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "aaa".to_string(),
            overall_score: Some(80),
            grade: Some("GOOD".to_string()),
            finding_count: 1,
            finding_ids: vec!["finding_a".to_string()],
        };
        let after_same = before.clone();
        let after_changed = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "bbb".to_string(),
            overall_score: Some(85),
            grade: Some("GOOD".to_string()),
            finding_count: 1,
            finding_ids: vec!["finding_a".to_string()],
        };

        assert_eq!(
            execution_status(&before, &after_same),
            EXECUTED_VALIDATED_NO_CHANGE
        );
        assert_eq!(
            execution_status(&before, &after_changed),
            EXECUTED_VALIDATED_CHANGED
        );
    }

    #[test]
    fn rescan_plan_validation_delta_sorts_and_deduplicates_finding_changes() {
        let before = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "aaa".to_string(),
            overall_score: Some(80),
            grade: Some("GOOD".to_string()),
            finding_count: 4,
            finding_ids: vec![
                "z_existing".to_string(),
                "a_removed".to_string(),
                "a_removed".to_string(),
                "m_removed".to_string(),
            ],
        };
        let after = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "bbb".to_string(),
            overall_score: Some(80),
            grade: Some("GOOD".to_string()),
            finding_count: 4,
            finding_ids: vec![
                "z_existing".to_string(),
                "b_added".to_string(),
                "b_added".to_string(),
                "a_added".to_string(),
            ],
        };

        let delta = validation_delta(&before, &after);

        assert_eq!(
            delta.added_findings,
            vec!["a_added".to_string(), "b_added".to_string()]
        );
        assert_eq!(
            delta.removed_findings,
            vec!["a_removed".to_string(), "m_removed".to_string()]
        );
    }

    #[test]
    fn rescan_plan_arbitration_verdict_tracks_validation_direction() {
        let before = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "aaa".to_string(),
            overall_score: Some(80),
            grade: Some("GOOD".to_string()),
            finding_count: 2,
            finding_ids: vec!["finding_a".to_string(), "finding_b".to_string()],
        };
        let possible_improvement = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "bbb".to_string(),
            overall_score: Some(85),
            grade: Some("GOOD".to_string()),
            finding_count: 1,
            finding_ids: vec!["finding_a".to_string()],
        };
        let regression = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "ccc".to_string(),
            overall_score: Some(70),
            grade: Some("NEEDS IMPROVEMENT".to_string()),
            finding_count: 3,
            finding_ids: vec![
                "finding_a".to_string(),
                "finding_b".to_string(),
                "finding_c".to_string(),
            ],
        };
        let neutral_change = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "ddd".to_string(),
            ..before.clone()
        };

        let possible_improvement_delta = validation_delta(&before, &possible_improvement);
        assert_eq!(possible_improvement_delta.score_delta, Some(5));
        assert_eq!(possible_improvement_delta.finding_count_delta, -1);
        assert_eq!(
            arbitration_verdict(&possible_improvement_delta),
            ARBITRATION_POSSIBLE_IMPROVEMENT_REVIEW_REQUIRED
        );
        assert_eq!(
            rescan_promotion_status_for(arbitration_verdict(&possible_improvement_delta)),
            RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED
        );
        assert!(
            rescan_promotion_blockers_for(arbitration_verdict(&possible_improvement_delta))
                .contains(&"validation_delta_is_not_truth_promotion".to_string())
        );

        let regression_delta = validation_delta(&before, &regression);
        assert_eq!(regression_delta.score_delta, Some(-10));
        assert_eq!(regression_delta.finding_count_delta, 1);
        assert_eq!(
            arbitration_verdict(&regression_delta),
            ARBITRATION_REGRESSION_REVIEW_REQUIRED
        );
        assert_eq!(
            rescan_promotion_status_for(arbitration_verdict(&regression_delta)),
            RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED
        );

        let neutral_change_delta = validation_delta(&before, &neutral_change);
        assert_eq!(
            arbitration_verdict(&neutral_change_delta),
            ARBITRATION_NEUTRAL_CHANGE_REVIEW_REQUIRED
        );
        assert_eq!(
            rescan_promotion_status_for(arbitration_verdict(&neutral_change_delta)),
            RESCAN_PROMOTION_NOT_PROMOTED_REVIEW_REQUIRED
        );
    }

    #[test]
    fn rescan_plan_arbitration_treats_score_presence_changes_as_neutral_review() {
        let before = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "aaa".to_string(),
            overall_score: None,
            grade: None,
            finding_count: 0,
            finding_ids: Vec::new(),
        };
        let scored_after = ProjectRescanValidationSnapshot {
            overall_score: Some(80),
            ..before.clone()
        };
        let graded_after = ProjectRescanValidationSnapshot {
            grade: Some("GOOD".to_string()),
            ..before.clone()
        };

        let scored_delta = validation_delta(&before, &scored_after);
        assert!(scored_delta.score_changed);
        assert_eq!(scored_delta.score_delta, None);
        assert_eq!(
            arbitration_verdict(&scored_delta),
            ARBITRATION_NEUTRAL_CHANGE_REVIEW_REQUIRED
        );

        let graded_delta = validation_delta(&before, &graded_after);
        assert!(graded_delta.grade_changed);
        assert_eq!(
            arbitration_verdict(&graded_delta),
            ARBITRATION_NEUTRAL_CHANGE_REVIEW_REQUIRED
        );
    }

    #[test]
    fn rescan_plan_arbitration_prioritizes_added_findings_over_removed_findings() {
        let before = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "aaa".to_string(),
            overall_score: Some(80),
            grade: Some("GOOD".to_string()),
            finding_count: 2,
            finding_ids: vec!["finding_old".to_string(), "finding_shared".to_string()],
        };
        let after = ProjectRescanValidationSnapshot {
            artifact_fingerprint: "bbb".to_string(),
            finding_ids: vec!["finding_new".to_string(), "finding_shared".to_string()],
            ..before.clone()
        };

        let delta = validation_delta(&before, &after);

        assert_eq!(delta.score_delta, Some(0));
        assert_eq!(delta.finding_count_delta, 0);
        assert_eq!(delta.added_findings, vec!["finding_new".to_string()]);
        assert_eq!(delta.removed_findings, vec!["finding_old".to_string()]);
        assert_eq!(
            arbitration_verdict(&delta),
            ARBITRATION_REGRESSION_REVIEW_REQUIRED
        );
    }

    #[test]
    fn rescan_plan_score_label_formats_sparse_validation_scores() {
        assert_eq!(score_label(Some(91), Some("EXCELLENT")), "91/100 EXCELLENT");
        assert_eq!(score_label(Some(72), None), "72/100");
        assert_eq!(score_label(None, Some("GOOD")), "GOOD");
        assert_eq!(score_label(None, None), "n/a");
    }

    fn command_hint(intent: &str, specforge_args: Vec<&str>) -> ProjectRescanCommandHint {
        let mut args = vec![
            "run".to_string(),
            "--manifest-path".to_string(),
            "Cargo.toml".to_string(),
            "--".to_string(),
        ];
        args.extend(specforge_args.into_iter().map(str::to_string));
        ProjectRescanCommandHint {
            intent: intent.to_string(),
            executable: "cargo".to_string(),
            args,
            working_directory: ".".to_string(),
            display: "cargo run --manifest-path Cargo.toml -- intent generated/semantic_ir/doc/semantic_ir.json"
                .to_string(),
        }
    }

    fn recommendation(document_key: &str, automation_status: &str) -> ProjectRescanRecommendation {
        ProjectRescanRecommendation {
            document_key: document_key.to_string(),
            display_name: format!("{document_key}.pdf"),
            stage: "intent_ir".to_string(),
            artifact_path: format!("generated/intent_ir/{document_key}/intent_ir.json"),
            replay_inputs: Vec::new(),
            finding_id: "intent_negative_knowledge_rescan_guidance".to_string(),
            related_ids: vec!["temporal_conflict_0001".to_string()],
            extractor_lane: "intent_ir_canonical_surface_corroboration".to_string(),
            corroboration_policy:
                "stronger_local_corroboration_required_before_canonical_promotion".to_string(),
            recommended_action: "rebuild IntentIR after targeted semantic/evidence rescans"
                .to_string(),
            recommended_commands: vec![command_hint(
                "validate_current_artifact",
                vec!["validate", "generated/intent_ir/doc/intent_ir.json"],
            )],
            automation_status: automation_status.to_string(),
            execution_summary: None,
        }
    }
}
