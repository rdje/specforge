use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::{
    EvidenceArgs, IngestArgs, IntentArgs, RescanPlanArgs, SemanticArgs, ValidateArgs,
};
use crate::commands::{evidence, ingest, intent, semantic, validate};
use crate::error::{AppError, Result};

use super::project_validation::{
    ProjectRescanCommandHint, ProjectRescanPlanRecord, ProjectRescanRecommendation,
};

const SUPPORTED_RESCAN_PLAN_SCHEMA_VERSION: u32 = 2;
const PLANNED_NOT_EXECUTED: &str = "planned_not_executed";
const EXECUTED_VALIDATED_CHANGED: &str = "executed_validated_changed";
const EXECUTED_VALIDATED_NO_CHANGE: &str = "executed_validated_no_change";

pub fn run(args: RescanPlanArgs) -> Result<()> {
    run_plan(args).map(|_| ())
}

pub fn run_plan(args: RescanPlanArgs) -> Result<RescanPlanRunReport> {
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
    }

    fs::write(&plan_path, serde_json::to_string_pretty(&plan)?)?;
    println!("status_update_path: {}", plan_path.display());
    Ok(report)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RescanPlanRunReport {
    pub plan_path: PathBuf,
    pub execute: bool,
    pub document_key_filter: Option<String>,
    pub schema_version: u32,
    pub recommendation_count: usize,
    pub pending_recommendations: usize,
    pub selected_recommendations: usize,
    pub executed_validated_changed: usize,
    pub executed_validated_no_change: usize,
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
            let document_matches = document_key.map_or(true, |document_key| {
                recommendation.document_key == document_key
            });
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
    println!("rescan_queue:");
    for index in selected_indices {
        let recommendation = &plan.recommendations[*index];
        println!(
            "- {} {} {}",
            recommendation.document_key, recommendation.stage, recommendation.finding_id
        );
        println!("  artifact_path: {}", recommendation.artifact_path);
        println!("  extractor_lane: {}", recommendation.extractor_lane);
        println!("  related_ids: {}", recommendation.related_ids.join(", "));
        println!("  recommended_commands:");
        for command in &recommendation.recommended_commands {
            println!("  - {}: {}", command.intent, command.display);
        }
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
    println!("execution_status: {automation_status}");

    Ok(RescanExecutionOutcome { automation_status })
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RescanInvocation {
    Ingest(PathBuf),
    Evidence(PathBuf),
    Semantic(PathBuf),
    Intent(PathBuf),
    Validate(PathBuf),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RescanValidationSnapshot {
    artifact_fingerprint: String,
    overall_score: Option<u32>,
    grade: Option<String>,
    finding_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RescanExecutionOutcome {
    automation_status: &'static str,
}

fn validate_and_snapshot(artifact_path: &Path) -> Result<RescanValidationSnapshot> {
    validate::run(ValidateArgs {
        artifact: artifact_path.to_path_buf(),
    })?;
    let report = read_validation_report(artifact_path)?;
    Ok(RescanValidationSnapshot {
        artifact_fingerprint: report.artifact_fingerprint,
        overall_score: report.overall_score,
        grade: report.grade,
        finding_count: report.findings.len(),
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
    before: &RescanValidationSnapshot,
    after: &RescanValidationSnapshot,
) -> &'static str {
    if before == after {
        EXECUTED_VALIDATED_NO_CHANGE
    } else {
        EXECUTED_VALIDATED_CHANGED
    }
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

fn execute_invocation(invocation: RescanInvocation, prior_memory: &Path) -> Result<()> {
    match invocation {
        RescanInvocation::Ingest(source) => ingest::run(IngestArgs {
            source,
            dry_run: false,
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
    }

    #[test]
    fn rescan_plan_execution_status_tracks_validation_deltas() {
        let before = RescanValidationSnapshot {
            artifact_fingerprint: "aaa".to_string(),
            overall_score: Some(80),
            grade: Some("GOOD".to_string()),
            finding_count: 1,
        };
        let after_same = before.clone();
        let after_changed = RescanValidationSnapshot {
            artifact_fingerprint: "bbb".to_string(),
            overall_score: Some(85),
            grade: Some("GOOD".to_string()),
            finding_count: 1,
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
        }
    }
}
