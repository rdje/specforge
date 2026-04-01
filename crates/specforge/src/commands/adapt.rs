use std::path::PathBuf;

use crate::cli::AdaptArgs;
use crate::error::Result;
use crate::ir::adapters::AdapterArtifact;

fn default_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("adapters")
}

pub fn run(args: AdaptArgs) -> Result<()> {
    let target = args.target.into();
    let adapter = AdapterArtifact::build(&args.intent_ir, target, &default_artifact_base_root())?;

    if args.dry_run {
        println!("command: adapt");
        println!("mode: dry-run");
        println!(
            "artifact_root: {}",
            adapter.artifact_layout.artifact_root.display()
        );
        println!("adapter_json:");
        println!("{}", adapter.to_pretty_json()?);
    } else {
        adapter.write_to_disk()?;
        println!("command: adapt");
        println!("mode: execute");
        println!(
            "adapter_artifact_path: {}",
            adapter.artifact_layout.adapter_artifact_path.display()
        );
        println!("intent_ir_path: {}", adapter.intent_ir_path.display());
        println!("document_key: {}", adapter.document_identity.document_key);
        println!("target: {}", adapter.target.as_str());
        println!("lowering_status: {}", adapter.lowering_status.as_str());
        if let Some(fsm) = &adapter.fsm {
            println!(
                "selected_root_kind: {}",
                fsm.root_kind_decision.selected_root_kind.as_str()
            );
            println!("signal_candidate_count: {}", fsm.signal_inventory.len());
            println!(
                "decision_tree_candidate_count: {}",
                fsm.decision_tree_candidates.len()
            );
            println!("state_candidate_count: {}", fsm.state_candidates.len());
            println!("renderable: {}", fsm.renderability.is_renderable);
        }
        println!(
            "residual_decision_count: {}",
            adapter.residual_decisions.len()
        );
        match &adapter.artifact_layout.emitted_target_path {
            Some(path) => println!("emitted_target_path: {}", path.display()),
            None => println!("emitted_target_path: none"),
        }
        println!("next_stage: target_specific_refinement");
    }

    Ok(())
}
