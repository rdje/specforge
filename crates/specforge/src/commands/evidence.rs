use std::path::PathBuf;

use crate::cli::EvidenceArgs;
use crate::error::Result;
use crate::ir::evidence::EvidenceIr;

fn default_artifact_base_root() -> PathBuf {
    PathBuf::from("generated").join("evidence_ir")
}

pub fn run(args: EvidenceArgs) -> Result<()> {
    let evidence_ir = EvidenceIr::build_with_prior_memory(
        &args.source_ir,
        &default_artifact_base_root(),
        Some(args.prior_memory.as_path()),
    )?;

    if args.dry_run {
        println!("command: evidence");
        println!("mode: dry-run");
        println!(
            "artifact_root: {}",
            evidence_ir.artifact_layout.artifact_root.display()
        );
        println!("evidence_ir_json:");
        println!("{}", evidence_ir.to_pretty_json()?);
    } else {
        evidence_ir.write_to_disk()?;
        println!("command: evidence");
        println!("mode: execute");
        println!(
            "evidence_ir_path: {}",
            evidence_ir.artifact_layout.evidence_ir_path.display()
        );
        println!("source_ir_path: {}", evidence_ir.source_ir_path.display());
        println!(
            "document_key: {}",
            evidence_ir.document_identity.document_key
        );
        println!("stage: {}", evidence_ir.stage.as_str());
        println!(
            "section_anchor_count: {}",
            evidence_ir.section_anchors.len()
        );
        println!("evidence_span_count: {}", evidence_ir.evidence_spans.len());
        println!(
            "visual_evidence_count: {}",
            evidence_ir.visual_evidence.len()
        );
        println!("evidence_link_count: {}", evidence_ir.evidence_links.len());
        println!(
            "extracted_statement_count: {}",
            evidence_ir.extracted_statements.len()
        );
        println!("prior_memory_path: {}", args.prior_memory.display());
        println!("next_stage: semantic_ir");
    }

    Ok(())
}
