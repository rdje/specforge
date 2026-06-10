//! `extract-constraints-llm` — the LLM-primary, Rust-grounded constraint extractor
//! (`EXTRACTION-QUALITY-GAUGE.5`): the replace-vs-patch test. Per distinct source sentence the LLM
//! proposes structured `(subject, kind, condition)` constraints; Rust grounds each (subject must type
//! as `Signal`, condition must appear in the source) and the result REPLACES `signal_constraints`.
//! Re-run `nli-verify` afterwards to compare its gauge against the patched Pattern extractor.

use crate::cli::{ExtractConstraintsLlmArgs, VlmProviderArg};
use crate::error::Result;
use crate::ir::condition_extract::is_grounded_in_source;
use crate::ir::constraint_extract_llm::{
    DEFAULT_EXTRACT_MODEL, dedup_constraints, ground_constraint, propose_constraints_llm,
};
use crate::ir::entity_typing::{EntityType, classify_entity, gather_entity_evidence};
use crate::ir::evidence::EvidenceIr;
use std::collections::BTreeSet;

pub fn run(args: ExtractConstraintsLlmArgs) -> Result<()> {
    let mut ir = EvidenceIr::load_from_path(&args.evidence_ir)?;
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| DEFAULT_EXTRACT_MODEL.to_string());

    println!("command: extract-constraints-llm");
    println!("artifact: {}", args.evidence_ir.display());
    println!("provider: {:?}  model: {model}", args.vlm_provider);
    if matches!(args.vlm_provider, VlmProviderArg::Skip) {
        println!("provider: skip → no-op");
        return Ok(());
    }
    let provider = args.vlm_provider;

    // Distinct source sentences from the existing constraints (one LLM call each).
    let mut seen = BTreeSet::new();
    let mut sentences: Vec<(String, String)> = Vec::new();
    for c in &ir.signal_constraints {
        if seen.insert(c.source_text.clone()) {
            let sid = c
                .supporting_statement_ids
                .first()
                .cloned()
                .unwrap_or_default();
            sentences.push((sid, c.source_text.clone()));
        }
    }
    let before = ir.signal_constraints.len();
    println!("distinct source sentences: {}", sentences.len());

    let mut new_constraints = Vec::new();
    let mut n = 0usize;
    for (i, (sid, sentence)) in sentences.iter().enumerate() {
        if args.max_sentences != 0 && i >= args.max_sentences {
            break;
        }
        for raw in propose_constraints_llm(sentence, provider, &model) {
            let id = format!("llm_sigcon_{n:04}");
            // .1 grounding: type the subject. The extraction prompt already self-filters non-signals;
            // Rust is the backstop (rejects structural refs; honours declared signals).
            let type_subject = |s: &str| {
                classify_entity(
                    &gather_entity_evidence(s, &ir, std::slice::from_ref(sentence)),
                    |_| EntityType::Signal,
                )
            };
            if let Some(rec) = ground_constraint(
                &raw,
                sentence,
                sid,
                &id,
                type_subject,
                is_grounded_in_source,
            ) {
                new_constraints.push(rec);
                n += 1;
            }
        }
    }
    // .4 — collapse exact-duplicate obligations (same subject, kind, value, condition),
    // merging the duplicates' supporting statements so provenance is preserved.
    let grounded = new_constraints.len();
    let new_constraints = dedup_constraints(new_constraints);
    let after = new_constraints.len();
    ir.signal_constraints = new_constraints;
    ir.write_to_disk()?;
    println!(
        "constraints: {before} (Pattern) → {grounded} (LLM-primary, grounded) → {after} \
         (deduped; {merged} duplicate record(s) merged); written to disk",
        merged = grounded - after
    );
    Ok(())
}
