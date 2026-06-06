//! `entity-type` — type a document's constraint subjects (the entity-discrimination prototype,
//! `EXTRACTION-QUALITY-GAUGE.1`). Rust gathers each token's grounding evidence; the LLM judges; Rust
//! grounds the judgment; the enforcement gate keeps only `Signal`-typed subjects. Reports the type
//! breakdown + which subjects would be filtered (the spurious-subject errors CHI exposed).

use crate::cli::{EntityTypeArgs, VlmProviderArg};
use crate::error::Result;
use crate::ir::entity_typing::{
    DEFAULT_ENTITY_MODEL, classify_entity, gather_entity_evidence, is_valid_signal_subject,
    propose_entity_type_llm,
};
use crate::ir::evidence::EvidenceIr;
use std::collections::BTreeMap;

pub fn run(args: EntityTypeArgs) -> Result<()> {
    let ir = EvidenceIr::load_from_path(&args.evidence_ir)?;
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| DEFAULT_ENTITY_MODEL.to_string());

    // Distinct constraint subjects + the sentences they appear in (their grounding context).
    let mut context: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for c in &ir.signal_constraints {
        context
            .entry(c.subject_signal.clone())
            .or_default()
            .push(c.source_text.clone());
    }

    println!("command: entity-type");
    println!("artifact: {}", args.evidence_ir.display());
    println!("distinct constraint subjects: {}", context.len());
    println!("provider: {:?}  model: {model}", args.vlm_provider);
    if matches!(args.vlm_provider, VlmProviderArg::Skip) {
        println!(
            "note: skip → grounding-only (the LLM defers to Unknown; ungrounded tokens stay Unknown)"
        );
    }
    let provider = args.vlm_provider;

    let mut by_type: BTreeMap<&'static str, usize> = BTreeMap::new();
    let mut filtered: Vec<String> = Vec::new();
    let mut kept = 0usize;
    for (i, (subject, ctx)) in context.iter().enumerate() {
        if args.max_subjects != 0 && i >= args.max_subjects {
            break;
        }
        let evidence = gather_entity_evidence(subject, &ir, ctx);
        let ty = classify_entity(&evidence, |e| propose_entity_type_llm(e, provider, &model));
        *by_type.entry(ty.as_str()).or_default() += 1;
        if is_valid_signal_subject(ty) {
            kept += 1;
        } else {
            filtered.push(format!("{subject}={}", ty.as_str()));
        }
    }

    println!("=== entity-type breakdown ===");
    for (ty, n) in &by_type {
        println!("  {ty:<14} {n}");
    }
    println!(
        "=== enforcement: {kept} subjects kept (signal), {} filtered (non-signal) ===",
        filtered.len()
    );
    if !filtered.is_empty() {
        println!(
            "filtered (sample): {}",
            filtered
                .iter()
                .take(24)
                .cloned()
                .collect::<Vec<_>>()
                .join("   ")
        );
    }
    Ok(())
}
