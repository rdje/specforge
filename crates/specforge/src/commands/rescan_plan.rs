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
const EXECUTED: &str = "executed";

pub fn run(args: RescanPlanArgs) -> Result<()> {
    let plan_path = args.plan;
    let mut plan = load_rescan_plan(&plan_path)?;
    let selected_indices = selected_pending_indices(&plan, args.limit);

    println!("command: rescan-plan");
    println!("mode: {}", if args.execute { "execute" } else { "dry-run" });
    println!("plan_path: {}", plan_path.display());
    println!("schema_version: {}", plan.schema_version);
    println!("recommendation_count: {}", plan.recommendation_count);
    println!(
        "pending_recommendations: {}",
        selected_pending_indices(&plan, 0).len()
    );
    println!("selected_recommendations: {}", selected_indices.len());

    if selected_indices.is_empty() {
        println!("rescan_queue: empty");
        return Ok(());
    }

    if !args.execute {
        print_dry_run_plan(&plan, &selected_indices);
        return Ok(());
    }

    for index in selected_indices {
        let recommendation = plan.recommendations[index].clone();
        execute_recommendation(&recommendation, &args.prior_memory)?;
        plan.recommendations[index].automation_status = EXECUTED.to_string();
    }

    fs::write(&plan_path, serde_json::to_string_pretty(&plan)?)?;
    println!("status_update_path: {}", plan_path.display());
    Ok(())
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

fn selected_pending_indices(plan: &ProjectRescanPlanRecord, limit: usize) -> Vec<usize> {
    let pending = plan
        .recommendations
        .iter()
        .enumerate()
        .filter_map(|(index, recommendation)| {
            (recommendation.automation_status == PLANNED_NOT_EXECUTED).then_some(index)
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
) -> Result<()> {
    println!(
        "executing_recommendation: {} {} {}",
        recommendation.document_key, recommendation.stage, recommendation.finding_id
    );

    for command in &recommendation.recommended_commands {
        let invocation = parse_command_hint(command)?;
        println!("executing_command_hint: {}", command.intent);
        execute_invocation(invocation, prior_memory)?;
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum RescanInvocation {
    Ingest(PathBuf),
    Evidence(PathBuf),
    Semantic(PathBuf),
    Intent(PathBuf),
    Validate(PathBuf),
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
                recommendation("doc_b", EXECUTED),
                recommendation("doc_c", PLANNED_NOT_EXECUTED),
            ],
        };

        assert_eq!(selected_pending_indices(&plan, 1), vec![0]);
        assert_eq!(selected_pending_indices(&plan, 0), vec![0, 2]);
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
