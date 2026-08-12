//! `extract-conditions` — capture dropped conditions (`EXTRACTION-QUALITY-GAUGE.2`). Same harness as
//! entity typing: Rust gathers the source + bare obligation, the LLM judges the condition clause,
//! Rust grounds it against the source, and the captured condition populates `condition_text` so the
//! claim is no longer flat. Updates the EvidenceIR in place.

use crate::cli::{ExtractConditionsArgs, VlmProviderArg};
use crate::error::Result;
use crate::ir::condition_extract::{
    ConditionEvidence, DEFAULT_CONDITION_MODEL, extract_condition, propose_condition_llm,
};
use crate::ir::evidence::{EvidenceIr, EvidenceMutationKind};
use crate::ir::nli_verify::constraint_claim_text;

/// Source-text cues that a requirement is conditional — only these spend an LLM call.
const CONDITION_CUES: &[&str] = &[
    "when ",
    "until ",
    "before ",
    "after ",
    "while ",
    "unless ",
    "whenever ",
    "once ",
    "during ",
    "provided ",
    "if ",
    "on receiving",
    "in the same cycle",
];

pub fn run(args: ExtractConditionsArgs) -> Result<()> {
    let mut ir = EvidenceIr::load_from_path(&args.evidence_ir)?;
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| DEFAULT_CONDITION_MODEL.to_string());

    println!("command: extract-conditions");
    println!("artifact: {}", args.evidence_ir.display());
    println!("provider: {:?}  model: {model}", args.vlm_provider);
    if matches!(args.vlm_provider, VlmProviderArg::Skip) {
        println!("provider: skip → no-op");
        return Ok(());
    }
    let provider = args.vlm_provider;

    let mut processed = 0usize;
    let mut captured = 0usize;
    for c in ir.signal_constraints.iter_mut() {
        // Already conditional → leave it.
        if c.condition_text
            .as_deref()
            .map(|s| !s.trim().is_empty())
            .unwrap_or(false)
        {
            continue;
        }
        // Only spend an LLM call when the source plausibly carries a condition.
        let src = c.source_text.to_ascii_lowercase();
        if !CONDITION_CUES.iter().any(|w| src.contains(w)) {
            continue;
        }
        if args.max_constraints != 0 && processed >= args.max_constraints {
            break;
        }
        processed += 1;
        let ev = ConditionEvidence {
            base_claim: constraint_claim_text(c),
            source_text: c.source_text.clone(),
            existing_condition: None,
        };
        if let Some(cond) = extract_condition(&ev, |e| propose_condition_llm(e, provider, &model)) {
            c.condition_text = Some(cond);
            captured += 1;
        }
    }
    ir.authorize_mutation(EvidenceMutationKind::ConditionExtraction)?;
    ir.write_to_disk()?;
    println!(
        "candidate constraints (unconditional + source has a condition cue): {processed}; conditions captured + grounded: {captured}"
    );
    Ok(())
}
