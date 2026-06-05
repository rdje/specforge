//! `nli-verify`: run the NLI entailment grounding gate over an EvidenceIR's
//! constraint claims (`NLI-ENTAILMENT-VERIFIER.3`). For each constraint it asks a
//! text LLM whether the source sentence *entails* the constraint-as-a-claim, and
//! reports the ones judged NOT entailed — likely hallucinations / residual
//! candidates (e.g. a condition mistaken for an obligation that a string-match
//! grounding waved through). Fail-safe: an unavailable provider (or `--vlm-provider
//! skip`) yields no flags — the gate abstains, never a false positive.

use crate::cli::{NliVerifyArgs, VlmProviderArg};
use crate::error::Result;
use crate::ir::evidence::EvidenceIr;
use crate::ir::nli_verify::{DEFAULT_NLI_MODEL, nli_claim_findings, verify_entailment};

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
    let findings = nli_claim_findings(&ir.signal_constraints, |source, claim| {
        verify_entailment(provider, &model, "", source, claim)
    });

    println!("not_entailed_claims: {}", findings.len());
    for f in &findings {
        println!(
            "  not-entailed [{}] {}: claim={:?}  source={:?}",
            f.constraint_id, f.subject_signal, f.claim_text, f.source_text
        );
    }
    Ok(())
}
