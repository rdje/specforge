use std::path::PathBuf;

use crate::cli::SemanticArgs;
use crate::error::Result;
use crate::ir::semantic::SemanticIr;

fn default_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("semantic_ir")
}

pub fn run(args: SemanticArgs) -> Result<()> {
    let semantic_ir = SemanticIr::build(&args.evidence_ir, &default_artifact_base_root())?;

    if args.dry_run {
        println!("command: semantic");
        println!("mode: dry-run");
        println!(
            "artifact_root: {}",
            semantic_ir.artifact_layout.artifact_root.display()
        );
        println!("semantic_ir_json:");
        println!("{}", semantic_ir.to_pretty_json()?);
    } else {
        semantic_ir.write_to_disk()?;
        println!("command: semantic");
        println!("mode: execute");
        println!(
            "semantic_ir_path: {}",
            semantic_ir.artifact_layout.semantic_ir_path.display()
        );
        println!(
            "evidence_ir_path: {}",
            semantic_ir.evidence_ir_path.display()
        );
        println!(
            "document_key: {}",
            semantic_ir.document_identity.document_key
        );
        println!("stage: {}", semantic_ir.stage.as_str());
        println!("actor_count: {}", semantic_ir.actors.len());
        println!("interface_count: {}", semantic_ir.interfaces.len());
        println!("phase_count: {}", semantic_ir.phases.len());
        println!("invariant_count: {}", semantic_ir.invariants.len());
        println!("contract_count: {}", semantic_ir.contracts.len());
        println!("gate_count: {}", semantic_ir.gates.len());
        println!("assertion_count: {}", semantic_ir.assertions.len());
        println!("abstraction_count: {}", semantic_ir.abstractions.len());
        println!(
            "decomposition_candidate_count: {}",
            semantic_ir.decomposition_candidates.len()
        );
        println!(
            "residual_decision_count: {}",
            semantic_ir.residual_decisions.len()
        );
        println!("next_stage: intent_ir");
    }

    Ok(())
}
