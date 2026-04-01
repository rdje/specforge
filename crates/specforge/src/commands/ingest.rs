use std::path::PathBuf;

use crate::cli::IngestArgs;
use crate::error::Result;
use crate::ir::source::SourceIr;

fn default_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("source_ir")
}

pub fn run(args: IngestArgs) -> Result<()> {
    let mut source_ir = SourceIr::build(&args.source, &default_artifact_base_root())?;

    if args.dry_run {
        println!("command: ingest");
        println!("mode: dry-run");
        println!(
            "artifact_root: {}",
            source_ir.artifact_layout.artifact_root.display()
        );
        println!("source_ir_json:");
        println!("{}", source_ir.to_pretty_json()?);
    } else {
        source_ir.materialize()?;
        source_ir.write_to_disk()?;
        println!("command: ingest");
        println!("mode: execute");
        println!(
            "source_ir_path: {}",
            source_ir.artifact_layout.source_ir_path.display()
        );
        println!("document_key: {}", source_ir.document_identity.document_key);
        println!("stage: {}", source_ir.stage.as_str());
        if let Some(next_stage) = source_ir.downstream_stages.first() {
            println!("next_stage: {}", next_stage.as_str());
        }
        println!(
            "normalization_status: {}",
            source_ir.normalization_plan.status.as_str()
        );
        if let Some(promoted_markdown_path) = &source_ir.normalization_plan.promoted_markdown_path {
            println!(
                "promoted_markdown_path: {}",
                promoted_markdown_path.display()
            );
        }
        println!("page_artifact_count: {}", source_ir.page_artifacts.len());
        println!("visual_asset_count: {}", source_ir.visual_assets.len());
        println!(
            "automation_confidence: {}",
            source_ir.automation_confidence.as_str()
        );
        println!(
            "residual_decision_count: {}",
            source_ir.residual_decisions.len()
        );
        println!("planned_actions:");

        for step in &source_ir.planned_actions {
            println!("- {step}");
        }
        println!("adapter_targets:");
        for target in &source_ir.adapter_targets {
            println!("- {}", target.as_str());
        }
    }

    Ok(())
}
