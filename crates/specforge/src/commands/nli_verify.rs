//! `nli-verify`: run the NLI entailment grounding gate over an EvidenceIR's
//! constraint claims (`NLI-ENTAILMENT-VERIFIER.3`). For each constraint it asks a
//! text LLM whether the source sentence *entails* the constraint-as-a-claim, and
//! reports the ones judged NOT entailed — likely hallucinations / residual
//! candidates (e.g. a condition mistaken for an obligation that a string-match
//! grounding waved through). Fail-safe: an unavailable provider (or `--vlm-provider
//! skip`) yields no flags — the gate abstains, never a false positive.
//!
//! `EXTRACTION-QUALITY-GAUGE.0`: the pass result is also persisted into the
//! artifact as `extraction_quality_gauge` (same back-annotation semantics as
//! `validate` — `write_to_disk` honors the recorded `artifact_layout`), so the
//! per-document quality measurement survives the terminal and `validate` /
//! `project-validation` can report it provider-free. `converge` reuses
//! [`measure_and_persist_gauge`] after stabilization as the standing measurement.

use crate::cli::{NliVerifyArgs, VlmProviderArg};
use crate::error::Result;
use crate::ir::evidence::{
    EvidenceIr, EvidenceMutationKind, ExtractionQualityGaugeRecord, tier_count_by_fact_key,
};
use crate::ir::nli_verify::{
    DEFAULT_NLI_MODEL, NliConformalPass, gauge_from_conformal_pass, nli_conformal_pass,
    verify_entailment,
};
use std::path::Path;

/// One gauge measurement over a persisted EvidenceIR: the built record, the full
/// NLI pass behind it (for per-item reporting), and whether it was persisted.
pub struct GaugeOutcome {
    pub record: ExtractionQualityGaugeRecord,
    pub pass: NliConformalPass,
    pub persisted: bool,
}

/// Run ONE NLI pass over the artifact's `signal_constraints`, build the
/// extraction-quality gauge, and back-annotate it into the artifact. Shared by
/// `nli-verify` and `converge` (the post-stability standing measurement) so there
/// is exactly one implementation of the gauge. A pass that labeled NOTHING
/// (provider down → every verdict `Unknown`) is **not** a measurement and is not
/// persisted — it must not overwrite a real prior gauge with vacuous data.
pub fn measure_and_persist_gauge(
    evidence_ir_path: &Path,
    provider: VlmProviderArg,
    model: &str,
) -> Result<GaugeOutcome> {
    let mut ir = EvidenceIr::load_from_path(evidence_ir_path)?;
    // One NLI pass → the not-entailed findings AND the NLI-oracle conformal samples
    // (tier-agreement confidence axis; the NLI verdict is the automatic correctness label).
    let tier_counts = tier_count_by_fact_key(&ir.fact_provenance);
    let pass = nli_conformal_pass(&ir.signal_constraints, &tier_counts, |source, claim| {
        verify_entailment(provider, model, "", source, claim)
    });
    let record = gauge_from_conformal_pass(&pass, ir.signal_constraints.len(), model);
    let persisted = record.entailed + record.not_entailed > 0;
    if persisted {
        ir.extraction_quality_gauge = Some(record.clone());
        ir.authorize_mutation(EvidenceMutationKind::QualityGauge)?;
        ir.write_to_disk()?;
    }
    Ok(GaugeOutcome {
        record,
        pass,
        persisted,
    })
}

/// Render the one-line per-document gauge summary shared by `nli-verify` and the
/// `converge` convergence summary.
pub fn gauge_summary_line(record: &ExtractionQualityGaugeRecord) -> String {
    let labeled = record.entailed + record.not_entailed;
    if labeled == 0 {
        return format!(
            "no labeled verdicts ({} constraint(s), all abstained)",
            record.constraints_total
        );
    }
    format!(
        "not_entailed {}/{} labeled ({:.1}%), abstained {} (model {})",
        record.not_entailed,
        labeled,
        record.not_entailed as f64 * 100.0 / labeled as f64,
        record.abstained,
        record.model
    )
}

pub fn run(args: NliVerifyArgs) -> Result<()> {
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| DEFAULT_NLI_MODEL.to_string());

    println!("command: nli-verify");
    println!("artifact: {}", args.artifact.display());

    // `Skip` is an explicit no-op — the gate abstains on every claim and the
    // artifact is left untouched.
    if matches!(args.vlm_provider, VlmProviderArg::Skip) {
        let ir = EvidenceIr::load_from_path(&args.artifact)?;
        println!("constraints: {}", ir.signal_constraints.len());
        println!("provider: skip (no entailment check performed)");
        return Ok(());
    }
    println!("provider: {:?}", args.vlm_provider);
    println!("model: {model}");

    let outcome = measure_and_persist_gauge(&args.artifact, args.vlm_provider, &model)?;
    println!("constraints: {}", outcome.record.constraints_total);

    println!("not_entailed_claims: {}", outcome.pass.not_entailed.len());
    for f in &outcome.pass.not_entailed {
        println!(
            "  not-entailed [{}] {}: claim={:?}  source={:?}",
            f.constraint_id, f.subject_signal, f.claim_text, f.source_text
        );
    }

    // NLI-oracle split-conformal calibration — the gated-metric unblock: a calibrated accept
    // threshold over tier-agreement, with the NLI verdict labeling each fact (no human gold).
    if !outcome.pass.samples.is_empty() {
        let alpha = 0.2;
        println!(
            "  -- NLI-oracle split-conformal (axis: tier-agreement; alpha={alpha}; n={}) --",
            outcome.pass.samples.len()
        );
        match crate::eval::conformal_threshold(&outcome.pass.samples, alpha) {
            Some(t) => println!(
                "    accept tier>={:.0}: coverage={:.3}  empirical_error={:.3}",
                t.threshold, t.coverage, t.empirical_error
            ),
            None => println!(
                "    no tier threshold meets alpha={alpha} (n={})",
                outcome.pass.samples.len()
            ),
        }
    }

    // EXTRACTION-QUALITY-GAUGE.0 — the persisted per-document quality report.
    println!(
        "extraction_quality_gauge: {}",
        gauge_summary_line(&outcome.record)
    );
    println!(
        "gauge_persisted: {}",
        if outcome.persisted {
            "true (back-annotated into the artifact)"
        } else {
            "false (nothing labeled — a vacuous pass never overwrites a real measurement)"
        }
    );
    Ok(())
}
