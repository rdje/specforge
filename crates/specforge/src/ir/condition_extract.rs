//! `EXTRACTION-QUALITY-GAUGE.2` — **condition capture**: stop flattening conditional requirements.
//!
//! CHI's dominant error mode: a conditional/temporal statement ("TXSACTIVE must be asserted *after
//! receiving a snoop*, *until the last flit*") is extracted as a bare, unconditional constraint
//! ("TXSACTIVE must be asserted") — the condition, which IS the content, is dropped. The flattened
//! claim is then NLI-not-entailed (the source only requires it *under* the condition).
//!
//! Same harness as entity typing: Rust **gathers** the source + bare obligation; the LLM **judges**
//! the condition clause; Rust **grounds** it — a proposed condition is accepted only if its content
//! words actually appear in the source (no hallucinated conditions). The captured condition populates
//! `condition_text`, so `constraint_claim_text` re-includes it and the claim becomes entailed.

use crate::llm_text::{api_url, call_text_provider};
use crate::provider::VlmProviderArg;

/// What a constraint's condition is extracted from.
#[derive(Debug, Clone, Default)]
pub struct ConditionEvidence {
    /// The bare obligation, e.g. "TXSACTIVE must be asserted".
    pub base_claim: String,
    /// The sentence the constraint came from.
    pub source_text: String,
    /// A condition already on the constraint (if any) — kept as-is.
    pub existing_condition: Option<String>,
}

/// `true` when the proposed condition is grounded in the source — most of its significant words
/// (len > 3) appear in the source sentence. The guard that stops the LLM inventing a condition.
pub fn is_grounded_in_source(condition: &str, source: &str) -> bool {
    let src = source.to_ascii_lowercase();
    let words: Vec<String> = condition
        .split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| w.len() > 3)
        .map(|w| w.to_ascii_lowercase())
        .collect();
    if words.is_empty() {
        return false;
    }
    let hits = words.iter().filter(|w| src.contains(w.as_str())).count();
    hits as f64 / words.len() as f64 >= 0.6
}

/// Extract a constraint's condition: keep an existing one; else the injected LLM `propose`s it and
/// Rust grounds it against the source. Returns `None` for a genuinely unconditional requirement.
/// `propose` is injected → unit-testable with no provider.
pub fn extract_condition(
    ev: &ConditionEvidence,
    propose: impl Fn(&ConditionEvidence) -> Option<String>,
) -> Option<String> {
    if let Some(existing) = &ev.existing_condition
        && !existing.trim().is_empty()
    {
        return Some(existing.clone());
    }
    let proposed = propose(ev)?;
    let proposed = proposed.trim();
    if proposed.is_empty() || proposed.eq_ignore_ascii_case("none") {
        return None;
    }
    // GROUND: a condition the source does not contain is a hallucination — drop it.
    if is_grounded_in_source(proposed, &ev.source_text) {
        Some(proposed.to_string())
    } else {
        None
    }
}

/// Default text model (the local model the rest of the pipeline uses).
pub const DEFAULT_CONDITION_MODEL: &str = "qwen2.5:14b-instruct";

/// The LLM judgment prompt — extract the condition clause from the source for this requirement.
pub fn condition_prompt(ev: &ConditionEvidence) -> String {
    format!(
        "A requirement was extracted from a digital-hardware specification sentence. Copy the CONDITION or \
         TEMPORAL scope under which the requirement applies — the when / until / before / after / \
         while / on-receiving clause taken from the sentence. If the requirement is unconditional \
         (always holds), answer exactly NONE. Treat document-owned symbols as opaque and copy the \
         clause without interpreting familiar names. Answer with the clause only, no preamble.\n\n\
         Sentence: {}\n\
         Requirement: {}\n\
         Condition:",
        ev.source_text, ev.base_claim,
    )
}

/// Production `propose` for [`extract_condition`].
pub fn propose_condition_llm(
    ev: &ConditionEvidence,
    provider: VlmProviderArg,
    model: &str,
) -> Option<String> {
    match call_text_provider(
        provider,
        model,
        api_url(provider),
        "",
        &ev.source_text,
        &condition_prompt(ev),
        48,
    ) {
        Ok(resp) => {
            let r = resp.trim();
            if r.is_empty() || r.eq_ignore_ascii_case("none") {
                None
            } else {
                Some(r.to_string())
            }
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(base: &str, source: &str) -> ConditionEvidence {
        ConditionEvidence {
            base_claim: base.to_string(),
            source_text: source.to_string(),
            existing_condition: None,
        }
    }

    #[test]
    fn keeps_an_existing_condition() {
        let mut e = ev("PREADY must be asserted", "…");
        e.existing_condition = Some("for read transfers".to_string());
        let got = extract_condition(&e, |_| Some("something else".to_string()));
        assert_eq!(got.as_deref(), Some("for read transfers"));
    }

    #[test]
    fn captures_a_grounded_condition() {
        let e = ev(
            "TXSACTIVE must be asserted",
            "TXSACTIVE must be asserted after receiving an initiating snoop or SnpDVMOp flit.",
        );
        let got = extract_condition(&e, |_| {
            Some("after receiving an initiating snoop".to_string())
        });
        assert_eq!(got.as_deref(), Some("after receiving an initiating snoop"));
    }

    #[test]
    fn rejects_a_hallucinated_condition() {
        let e = ev(
            "TXSACTIVE must be asserted",
            "TXSACTIVE must be asserted after receiving a snoop.",
        );
        // The LLM invents a condition absent from the source → grounding drops it.
        let got = extract_condition(&e, |_| {
            Some("when the cache line is in the dirty state".to_string())
        });
        assert_eq!(got, None, "ungrounded condition must be rejected");
    }

    #[test]
    fn none_means_unconditional() {
        let e = ev("PCLK must be stable", "PCLK is the clock.");
        assert_eq!(extract_condition(&e, |_| Some("NONE".to_string())), None);
        assert_eq!(extract_condition(&e, |_| None), None);
    }

    #[test]
    fn condition_prompt_is_alpha_equivariant_and_opaque() {
        let first = condition_prompt(&ev(
            "orchid must be asserted",
            "orchid must be asserted after juniper rises",
        ))
        .replace("orchid", "<subject>")
        .replace("juniper", "<condition>");
        let renamed = condition_prompt(&ev(
            "copper must be asserted",
            "copper must be asserted after silver rises",
        ))
        .replace("copper", "<subject>")
        .replace("silver", "<condition>");
        assert_eq!(first, renamed);
        assert!(first.contains("document-owned symbols as opaque"));
        assert!(!first.contains("chip-specification"));
    }
}
