//! `extract-constraints-llm` — the LLM-primary, Rust-grounded constraint extractor
//! (`EXTRACTION-QUALITY-GAUGE.5`): the replace-vs-patch test. Per distinct source sentence the LLM
//! proposes structured `(subject, kind, condition)` constraints; Rust grounds each (subject must
//! type as `Signal` or as a catalog-declared `Field`, condition must appear in the source) and the
//! result REPLACES `signal_constraints` — with field-subject obligations routed to the
//! field-scoped `message_field_constraints` surface instead of polluting the signal surface or
//! being dropped (`.FIELD.4`). Re-run `nli-verify` afterwards to compare its gauge against the
//! patched Pattern extractor.

use crate::cli::{ExtractConstraintsLlmArgs, VlmProviderArg};
use crate::error::Result;
use crate::ir::condition_extract::is_grounded_in_source;
use crate::ir::constraint_extract_llm::{
    DEFAULT_EXTRACT_MODEL, GroundedConstraint, dedup_constraints, dedup_field_constraints,
    ground_constraint_typed, propose_constraints_llm,
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
    let mut new_field_constraints = Vec::new();
    let mut n = 0usize;
    let mut field_n = 0usize;
    for (i, (sid, sentence)) in sentences.iter().enumerate() {
        if args.max_sentences != 0 && i >= args.max_sentences {
            break;
        }
        for raw in propose_constraints_llm(sentence, provider, &model) {
            let signal_id = format!("llm_sigcon_{n:04}");
            let field_id = format!("llm_fieldcon_{field_n:04}");
            // .1 grounding: type the subject. The extraction prompt already self-filters non-signals;
            // Rust is the backstop (rejects structural refs; honours declared signals, and grounds
            // catalog-declared message fields to `Field` — `.FIELD.3`).
            let type_subject = |s: &str| {
                classify_entity(
                    &gather_entity_evidence(s, &ir, std::slice::from_ref(sentence)),
                    |_| EntityType::Signal,
                )
            };
            // `.FIELD.4` — catalog containers declaring a field subject (provenance on the record).
            let field_containers = |name: &str| {
                let up = name.trim().to_ascii_uppercase();
                let mut containers: Vec<String> = Vec::new();
                for f in &ir.message_field_records {
                    if f.name.trim().to_ascii_uppercase() == up
                        && !containers.contains(&f.container)
                    {
                        containers.push(f.container.clone());
                    }
                }
                containers
            };
            match ground_constraint_typed(
                &raw,
                sentence,
                sid,
                &signal_id,
                &field_id,
                type_subject,
                is_grounded_in_source,
                field_containers,
            ) {
                Some(GroundedConstraint::Signal(rec)) => {
                    new_constraints.push(rec);
                    n += 1;
                }
                Some(GroundedConstraint::Field(rec)) => {
                    new_field_constraints.push(rec);
                    field_n += 1;
                }
                None => {}
            }
        }
    }
    // .4 — collapse exact-duplicate obligations (same subject, kind, value, condition),
    // merging the duplicates' supporting statements so provenance is preserved.
    let grounded = new_constraints.len();
    let new_constraints = dedup_constraints(new_constraints);
    let after = new_constraints.len();
    let field_grounded = new_field_constraints.len();
    let new_field_constraints = dedup_field_constraints(new_field_constraints);
    let field_after = new_field_constraints.len();
    ir.signal_constraints = new_constraints;
    ir.message_field_constraints = new_field_constraints;
    ir.write_to_disk()?;
    println!(
        "constraints: {before} (Pattern) → {grounded} (LLM-primary, grounded) → {after} \
         (deduped; {merged} duplicate record(s) merged); written to disk",
        merged = grounded - after
    );
    println!(
        "field constraints: {field_grounded} (field-subject obligations, grounded) → \
         {field_after} (deduped; {merged} duplicate record(s) merged)",
        merged = field_grounded - field_after
    );
    Ok(())
}
