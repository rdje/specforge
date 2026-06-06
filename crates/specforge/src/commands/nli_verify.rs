//! `nli-verify`: run the NLI entailment grounding gate over an EvidenceIR's
//! constraint claims (`NLI-ENTAILMENT-VERIFIER.3`). For each constraint it asks a
//! text LLM whether the source sentence *entails* the constraint-as-a-claim, and
//! reports the ones judged NOT entailed — likely hallucinations / residual
//! candidates (e.g. a condition mistaken for an obligation that a string-match
//! grounding waved through). Fail-safe: an unavailable provider (or `--vlm-provider
//! skip`) yields no flags — the gate abstains, never a false positive.

use crate::cli::{NliVerifyArgs, VlmProviderArg};
use crate::error::Result;
use crate::ir::evidence::{EvidenceIr, tier_count_by_fact_key};
use crate::ir::nli_verify::{DEFAULT_NLI_MODEL, nli_conformal_pass, verify_entailment};

pub fn run(args: NliVerifyArgs) -> Result<()> {
    let ir = EvidenceIr::load_from_path(&args.artifact)?;
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| DEFAULT_NLI_MODEL.to_string());

    println!("command: nli-verify");
    println!("artifact: {}", args.artifact.display());
    println!("constraints: {}", ir.signal_constraints.len());

    // `Skip` is an explicit no-op — the gate abstains on every claim.
    if matches!(args.vlm_provider, VlmProviderArg::Skip) {
        println!("provider: skip (no entailment check performed)");
        return Ok(());
    }
    println!("provider: {:?}", args.vlm_provider);
    println!("model: {model}");

    let provider = args.vlm_provider;
    // One NLI pass → the not-entailed findings AND the NLI-oracle conformal samples
    // (tier-agreement confidence axis; the NLI verdict is the automatic correctness label).
    let tier_counts = tier_count_by_fact_key(&ir.fact_provenance);
    let pass = nli_conformal_pass(&ir.signal_constraints, &tier_counts, |source, claim| {
        verify_entailment(provider, &model, "", source, claim)
    });

    println!("not_entailed_claims: {}", pass.not_entailed.len());
    for f in &pass.not_entailed {
        println!(
            "  not-entailed [{}] {}: claim={:?}  source={:?}",
            f.constraint_id, f.subject_signal, f.claim_text, f.source_text
        );
    }

    // NLI-oracle split-conformal calibration — the gated-metric unblock: a calibrated accept
    // threshold over tier-agreement, with the NLI verdict labeling each fact (no human gold).
    if !pass.samples.is_empty() {
        let alpha = 0.2;
        println!(
            "  -- NLI-oracle split-conformal (axis: tier-agreement; alpha={alpha}; n={}) --",
            pass.samples.len()
        );
        match crate::eval::conformal_threshold(&pass.samples, alpha) {
            Some(t) => println!(
                "    accept tier>={:.0}: coverage={:.3}  empirical_error={:.3}",
                t.threshold, t.coverage, t.empirical_error
            ),
            None => println!(
                "    no tier threshold meets alpha={alpha} (n={})",
                pass.samples.len()
            ),
        }
    }
    Ok(())
}
