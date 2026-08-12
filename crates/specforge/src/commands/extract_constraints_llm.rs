//! `extract-constraints-llm` — the LLM-primary, Rust-grounded constraint extractor
//! (`EXTRACTION-QUALITY-GAUGE.5`): the replace-vs-patch test. Per distinct source sentence the LLM
//! proposes structured `(subject, kind, condition)` constraints; Rust grounds each (subject must
//! type as `Signal` or as a catalog-declared `Field`, condition must appear in the source) and the
//! result REPLACES `signal_constraints` — with field-subject obligations routed to the
//! field-scoped `message_field_constraints` surface instead of polluting the signal surface or
//! being dropped (`.FIELD.4`). Re-run `nli-verify` afterwards to compare its gauge against the
//! patched Pattern extractor.
//!
//! `LLM-PRIMARY-PROMOTION.2`: the replacement core is exposed as [`promote_constraints`] so
//! `converge` can run the same promotion post-stability (opt-in), and the replaced surface is
//! manifest-recorded as `constraints.llm_primary` — no silent surface swaps.

use crate::cli::{ExtractConstraintsLlmArgs, VlmProviderArg};
use crate::error::Result;
use crate::ir::condition_extract::is_grounded_in_source;
use crate::ir::constraint_extract_llm::{
    DEFAULT_EXTRACT_MODEL, GroundedConstraint, dedup_constraints, dedup_field_constraints,
    ground_constraint_typed, propose_constraints_llm,
};
use crate::ir::entity_typing::{
    EntityType, declared_signal_catalog, resolve_unique_document_identifier,
};
use crate::ir::evidence::{EvidenceIr, ExtractorTier};
use crate::ir::extractor::{ExtractorRunEntry, SurfaceManifest};
use std::collections::BTreeSet;
use std::path::Path;

/// What one LLM-primary constraint promotion did to a persisted EvidenceIR.
#[derive(Debug, Clone)]
pub struct ConstraintPromotionReport {
    /// `signal_constraints` size before the replace (the Pattern surface).
    pub pattern_before: usize,
    /// Distinct source sentences sent to the model (one call each).
    pub sentences: usize,
    /// Grounded signal constraints before dedup.
    pub grounded: usize,
    /// Signal constraints kept after provenance-merging dedup (the new surface size).
    pub kept: usize,
    /// Grounded field-subject constraints before dedup.
    pub field_grounded: usize,
    /// Field constraints kept after dedup.
    pub field_kept: usize,
}

/// Run the LLM-primary grounded extractor over the persisted EvidenceIR at `evidence_ir_path`,
/// REPLACE its `signal_constraints` (and `message_field_constraints`) with the grounded result,
/// record the swap in the extraction manifest (`constraints.llm_primary`), and persist. The
/// recall universe is deliberately the existing constraints' distinct source sentences — this is
/// a refinement of what the Pattern surface found, not a new discovery pass
/// (`LLM-PRIMARY-PROMOTION.1`). `max_sentences == 0` means all.
pub fn promote_constraints(
    evidence_ir_path: &Path,
    provider: VlmProviderArg,
    model: &str,
    max_sentences: usize,
) -> Result<ConstraintPromotionReport> {
    let mut ir = EvidenceIr::load_from_path(evidence_ir_path)?;

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
    let pattern_before = ir.signal_constraints.len();
    let declared_signal_catalog = declared_signal_catalog(&ir);
    let declared_signals = declared_signal_catalog
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let declared_field_catalog = ir
        .message_field_records
        .iter()
        .map(|field| field.name.clone())
        .fold(Vec::new(), |mut fields, field| {
            if !fields.contains(&field) {
                fields.push(field);
            }
            fields
        });
    let declared_fields = declared_field_catalog
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut declared_carriers = declared_signal_catalog;
    for field in declared_field_catalog {
        if !declared_carriers.contains(&field) {
            declared_carriers.push(field);
        }
    }

    let mut new_constraints = Vec::new();
    let mut new_field_constraints = Vec::new();
    let mut n = 0usize;
    let mut field_n = 0usize;
    for (i, (sid, sentence)) in sentences.iter().enumerate() {
        if max_sentences != 0 && i >= max_sentences {
            break;
        }
        for raw in propose_constraints_llm(sentence, &declared_carriers, provider, model) {
            let signal_id = format!("llm_sigcon_{n:04}");
            let field_id = format!("llm_fieldcon_{field_n:04}");
            // .1 grounding: an exact current-document declaration is the only typing authority.
            // The model cannot turn its own proposal into the declaration that validates it.
            let type_subject = |s: &str| {
                if resolve_unique_document_identifier(
                    s,
                    declared_signals.iter().map(String::as_str),
                )
                .is_some()
                {
                    EntityType::Signal
                } else if resolve_unique_document_identifier(
                    s,
                    declared_fields.iter().map(String::as_str),
                )
                .is_some()
                {
                    EntityType::Field
                } else {
                    EntityType::Unknown
                }
            };
            // `.FIELD.4` — catalog containers declaring a field subject (provenance on the record).
            let field_containers = |name: &str| {
                let resolved = resolve_unique_document_identifier(
                    name,
                    ir.message_field_records
                        .iter()
                        .map(|field| field.name.as_str()),
                );
                let mut containers: Vec<String> = Vec::new();
                for f in &ir.message_field_records {
                    if resolved.is_some_and(|identity| f.name.trim() == identity)
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
    // `.3b` — restore the build-path invariant on the replaced surface: refine
    // asserted/deasserted kinds via the document's persisted resolved polarity, exactly as
    // the build applies before persisting. Without this the promoted shape feeds the
    // temporal layer a symbolic ASSERTED/DEASSERTED value the document already grounds to a
    // level. Runs BEFORE dedup so the canonical dedup key sees the refined kind.
    crate::ir::evidence::apply_persisted_polarity_to_constraints(
        &mut new_constraints,
        &ir.signal_polarities,
    );
    // .4 — collapse exact-duplicate obligations (same subject, kind, value, condition),
    // merging the duplicates' supporting statements so provenance is preserved.
    let grounded = new_constraints.len();
    let new_constraints = dedup_constraints(new_constraints);
    let kept = new_constraints.len();
    let field_grounded = new_field_constraints.len();
    let new_field_constraints = dedup_field_constraints(new_field_constraints);
    let field_kept = new_field_constraints.len();
    ir.signal_constraints = new_constraints;
    ir.message_field_constraints = new_field_constraints;
    // `LLM-PRIMARY-PROMOTION.2`: the surface swap leaves a manifest trail, so the
    // fingerprint/cluster plane can see which documents carry the promoted surface.
    ir.extraction_manifest
        .record_surface_manifest(SurfaceManifest {
            surface: "signal_constraints".to_string(),
            eligible: 1,
            entries: vec![ExtractorRunEntry {
                name: "constraints.llm_primary".to_string(),
                tier: ExtractorTier::Nlp,
                eligible: true,
                produced: grounded,
                kept,
            }],
        });
    // The replaced surface invalidates any persisted NLI gauge (its measured ids are gone);
    // dropping it here keeps the artifact honest even when no re-measure follows immediately.
    ir.extraction_quality_gauge = None;
    ir.write_to_disk()?;
    Ok(ConstraintPromotionReport {
        pattern_before,
        sentences: sentences.len(),
        grounded,
        kept,
        field_grounded,
        field_kept,
    })
}

pub fn run(args: ExtractConstraintsLlmArgs) -> Result<()> {
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

    let report = promote_constraints(
        &args.evidence_ir,
        args.vlm_provider,
        &model,
        args.max_sentences,
    )?;
    println!("distinct source sentences: {}", report.sentences);
    println!(
        "constraints: {before} (Pattern) → {grounded} (LLM-primary, grounded) → {after} \
         (deduped; {merged} duplicate record(s) merged); written to disk",
        before = report.pattern_before,
        grounded = report.grounded,
        after = report.kept,
        merged = report.grounded - report.kept
    );
    println!(
        "field constraints: {field_grounded} (field-subject obligations, grounded) → \
         {field_after} (deduped; {merged} duplicate record(s) merged)",
        field_grounded = report.field_grounded,
        field_after = report.field_kept,
        merged = report.field_grounded - report.field_kept
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::source::SourceIr;
    use std::fs;

    // LLM-PRIMARY-PROMOTION.2 — the promotion core's no-LLM-call surface effects: with zero
    // existing constraints there are zero source sentences (zero provider calls), the replace
    // still runs, the swap is manifest-recorded as `constraints.llm_primary`, and any persisted
    // NLI gauge is dropped (its measured ids are definitively gone after a replace).
    #[test]
    fn promote_constraints_records_manifest_and_drops_stale_gauge() -> Result<()> {
        let tempdir = tempfile::tempdir()?;
        let source = tempdir.path().join("spec.md");
        fs::write(&source, "# Spec\nSome non-normative content.\n")?;
        let source_ir = SourceIr::build(&source, &tempdir.path().join("source_ir"))?;
        source_ir.write_test_fixture_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &tempdir.path().join("evidence_ir"),
        )?;
        evidence_ir.signal_constraints.clear();
        evidence_ir.extraction_quality_gauge =
            Some(crate::ir::evidence::ExtractionQualityGaugeRecord {
                model: "m".into(),
                constraints_total: 0,
                entailed: 0,
                not_entailed: 0,
                abstained: 0,
                not_entailed_constraint_ids: Vec::new(),
            });
        evidence_ir.write_to_disk()?;
        let path = evidence_ir.artifact_layout.evidence_ir_path.clone();

        let report = promote_constraints(&path, VlmProviderArg::Ollama, "unused-model", 0)?;
        assert_eq!(report.pattern_before, 0);
        assert_eq!(report.sentences, 0, "no sentences → no provider calls");
        assert_eq!(report.kept, 0);

        let reloaded = EvidenceIr::load_from_path(&path)?;
        let surface = reloaded
            .extraction_manifest
            .surfaces
            .iter()
            .find(|s| s.surface == "signal_constraints")
            .expect("the promotion records a signal_constraints surface manifest");
        assert_eq!(surface.entries.len(), 1);
        assert_eq!(surface.entries[0].name, "constraints.llm_primary");
        assert_eq!(surface.entries[0].tier, ExtractorTier::Nlp);
        assert!(
            reloaded.extraction_quality_gauge.is_none(),
            "a replaced surface invalidates the persisted gauge"
        );
        Ok(())
    }
}
