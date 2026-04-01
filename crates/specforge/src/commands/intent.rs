use std::path::PathBuf;

use crate::cli::IntentArgs;
use crate::error::Result;
use crate::ir::intent::IntentIr;

fn default_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("intent_ir")
}

pub fn run(args: IntentArgs) -> Result<()> {
    let intent_ir = IntentIr::build(&args.semantic_ir, &default_artifact_base_root())?;

    if args.dry_run {
        println!("command: intent");
        println!("mode: dry-run");
        println!(
            "artifact_root: {}",
            intent_ir.artifact_layout.artifact_root.display()
        );
        println!("intent_ir_json:");
        println!("{}", intent_ir.to_pretty_json()?);
    } else {
        intent_ir.write_to_disk()?;
        println!("command: intent");
        println!("mode: execute");
        println!(
            "intent_ir_path: {}",
            intent_ir.artifact_layout.intent_ir_path.display()
        );
        println!("semantic_ir_path: {}", intent_ir.semantic_ir_path.display());
        println!("document_key: {}", intent_ir.document_identity.document_key);
        println!("stage: {}", intent_ir.stage.as_str());
        println!("intent_id: {}", intent_ir.intent_identity.intent_id);
        println!("actor_count: {}", intent_ir.actors.len());
        println!("behavior_count: {}", intent_ir.behaviors.len());
        println!("constraint_count: {}", intent_ir.constraints.len());
        println!("assumption_count: {}", intent_ir.assumptions.len());
        println!(
            "residual_decision_count: {}",
            intent_ir.residual_decisions.len()
        );
        println!("next_stage: adapters");
    }

    Ok(())
}
