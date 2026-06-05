//! `NLI-ENTAILMENT-VERIFIER`: a semantic "does the source actually say this?" gate.
//!
//! Premise = a source statement; hypothesis = a claim SpecForge extracted. The
//! verifier asks a text LLM whether the source **entails** the claim and keeps
//! only entailed claims (the SNLI/Bowman entailment framing for hallucination
//! mitigation). It is an **additive strengthening** gate, not the sole grounding:
//! a confident `NotEntailed` routes the claim to a residual, but a provider error
//! or unclear answer **abstains** (the existing rule-based grounding stands) — a
//! provider outage must never nuke extraction.
//!
//! Built on a *text* model (`qwen2.5:14b-instruct`), not the VLM: this is a
//! text-reasoning task, and the entailment framing was empirically the right one
//! (see KM `local-llm-for-text-reasoning`). It reuses the `commands::llm_text`
//! transport **and** its `SPECFORGE_VLM_HELPER` hermetic test hook, so no CI test
//! requires Ollama.

use crate::cli::VlmProviderArg;
use crate::commands::llm_text::{api_url, call_text_provider};

/// Default text model for NLI entailment (a text-only instruct LLM, not the VLM).
pub const DEFAULT_NLI_MODEL: &str = "qwen2.5:14b-instruct";

/// The entailment verdict for a `(source, claim)` pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NliVerdict {
    /// The source logically supports the claim.
    Entailed,
    /// The source does NOT support the claim (it adds / changes / contradicts,
    /// or mistakes a condition for an obligation).
    NotEntailed,
    /// No clear verdict (provider error, ambiguous answer). Fail-closed — never
    /// treated as `Entailed`.
    Unknown,
}

/// What the gate does with a verdict — additive and fail-safe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NliGateAction {
    /// Verified — keep the claim.
    Keep,
    /// Likely hallucination — route the claim to a residual decision.
    RouteResidual,
    /// Could not verify — leave the claim's disposition unchanged (the existing
    /// grounding stands). A provider outage must not break extraction.
    Abstain,
}

/// Map a verdict to the gate action.
pub fn gate_action(verdict: NliVerdict) -> NliGateAction {
    match verdict {
        NliVerdict::Entailed => NliGateAction::Keep,
        NliVerdict::NotEntailed => NliGateAction::RouteResidual,
        NliVerdict::Unknown => NliGateAction::Abstain,
    }
}

/// Build the deterministic entailment prompt (premise = `source`, hypothesis =
/// `claim`). The "a condition / added / changed / contradicted fact is NOT
/// entailed" rule is the load-bearing instruction — it is exactly the nuance a
/// string-match gate misses and a strong text model handles.
pub fn entailment_prompt(source: &str, claim: &str) -> String {
    format!(
        "Source (from a chip-protocol specification): \"{source}\"\n\
         Claim: \"{claim}\"\n\
         Does the Source logically SUPPORT (entail) the Claim? A claim is NOT \
         entailed if it adds, changes, or contradicts what the Source states — \
         for example a CONDITION (\"when X is asserted ...\") is not an OBLIGATION \
         on X; a different value, actor, or signal; or any fact the Source does \
         not state.\n\
         Answer with ONLY one word: ENTAILED or NOT_ENTAILED."
    )
}

/// Parse a raw LLM response into a verdict. **Fail-closed:** anything that is not
/// an unambiguous `ENTAILED` becomes `Unknown`, and a `NOT_ENTAILED` (in any
/// spacing) always wins over a substring `ENTAILED`.
pub fn parse_nli_verdict(response: &str) -> NliVerdict {
    let up = response.to_ascii_uppercase();
    if up.contains("NOT_ENTAILED") || up.contains("NOT ENTAILED") || up.contains("NOTENTAILED") {
        return NliVerdict::NotEntailed;
    }
    if up.contains("ENTAILED") {
        return NliVerdict::Entailed;
    }
    NliVerdict::Unknown
}

/// Verify whether `source` entails `claim` via the text provider. **Fail-closed:**
/// any provider error → `Unknown` (→ `Abstain`), so an outage never breaks
/// extraction. `source_statement_id` / `source` double as the
/// `SPECFORGE_VLM_HELPER` mock keys, so the whole path is hermetically testable.
pub fn verify_entailment(
    provider: VlmProviderArg,
    model: &str,
    source_statement_id: &str,
    source: &str,
    claim: &str,
) -> NliVerdict {
    let prompt = entailment_prompt(source, claim);
    match call_text_provider(
        provider,
        model,
        api_url(provider),
        source_statement_id,
        source,
        &prompt,
        16,
    ) {
        Ok(resp) => parse_nli_verdict(&resp),
        Err(_) => NliVerdict::Unknown,
    }
}

/// Render a `SignalConstraintRecord` as a natural-language claim (the NLI
/// hypothesis) — e.g. `PADDR must be stable`, `PSTRB must be LOW`,
/// `HTRANS must be IDLE`. The constraint's own `source_text` is the premise.
pub fn constraint_claim_text(c: &crate::ir::source::SignalConstraintRecord) -> String {
    use crate::ir::source::SignalConstraintKind as K;
    // `MustNotChange` is inherently negative; the rest take the `negated` flag.
    if matches!(c.constraint_kind, K::MustNotChange) {
        return format!("{} must not change", c.subject_signal);
    }
    let verb = if c.negated { "must not" } else { "must" };
    let what = match &c.constraint_kind {
        K::MustBeHigh => "be HIGH".to_string(),
        K::MustBeLow => "be LOW".to_string(),
        K::MustBeAsserted => "be asserted".to_string(),
        K::MustBeDeasserted => "be deasserted".to_string(),
        K::MustBeStable => "be stable".to_string(),
        K::MustHoldData => "hold its data".to_string(),
        K::MustBeValue { value } => format!("be {value}"),
        K::MustNotChange => unreachable!("handled above"),
    };
    format!("{} {verb} {what}", c.subject_signal)
}

/// A constraint the NLI verifier judged NOT entailed by its own source sentence
/// — a likely hallucination / residual candidate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NliClaimFinding {
    pub constraint_id: String,
    pub subject_signal: String,
    pub claim_text: String,
    pub source_text: String,
}

/// Run the (injected) entailment verifier over each constraint — premise =
/// `constraint.source_text`, hypothesis = `constraint_claim_text` — and collect
/// the ones judged `NotEntailed` (likely hallucinations → residual candidates).
/// `Entailed` is kept; `Unknown` **abstains** (a provider outage yields no
/// findings, never a false flag). The verifier is a parameter, so this is fully
/// testable with no provider or network — production passes a closure over
/// [`verify_entailment`].
pub fn nli_claim_findings(
    constraints: &[crate::ir::source::SignalConstraintRecord],
    verify: impl Fn(&str, &str) -> NliVerdict,
) -> Vec<NliClaimFinding> {
    constraints
        .iter()
        .filter_map(|c| {
            let claim_text = constraint_claim_text(c);
            match verify(&c.source_text, &claim_text) {
                NliVerdict::NotEntailed => Some(NliClaimFinding {
                    constraint_id: c.constraint_id.clone(),
                    subject_signal: c.subject_signal.clone(),
                    claim_text,
                    source_text: c.source_text.clone(),
                }),
                NliVerdict::Entailed | NliVerdict::Unknown => None,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

    fn cons(
        id: &str,
        subj: &str,
        kind: SignalConstraintKind,
        source: &str,
    ) -> SignalConstraintRecord {
        SignalConstraintRecord {
            constraint_id: id.into(),
            subject_signal: subj.into(),
            constraint_kind: kind,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: source.into(),
            supporting_statement_ids: vec![],
            automation_confidence: crate::ir::source::AutomationConfidence::Medium,
        }
    }

    #[test]
    fn constraint_claim_text_renders_each_kind() {
        use SignalConstraintKind as K;
        assert_eq!(
            constraint_claim_text(&cons("c", "PADDR", K::MustBeStable, "x")),
            "PADDR must be stable"
        );
        assert_eq!(
            constraint_claim_text(&cons("c", "PSTRB", K::MustBeLow, "x")),
            "PSTRB must be LOW"
        );
        assert_eq!(
            constraint_claim_text(&cons(
                "c",
                "HTRANS",
                K::MustBeValue {
                    value: "IDLE".into()
                },
                "x"
            )),
            "HTRANS must be IDLE"
        );
        assert_eq!(
            constraint_claim_text(&cons("c", "HAUSER", K::MustNotChange, "x")),
            "HAUSER must not change"
        );
    }

    #[test]
    fn nli_claim_findings_collects_only_not_entailed() {
        use SignalConstraintKind as K;
        let cs = vec![
            cons(
                "c1",
                "PADDR",
                K::MustBeStable,
                "PADDR must be stable until the transfer completes.",
            ),
            cons(
                "c2",
                "PSEL",
                K::MustBeAsserted,
                "PBUSER must be valid when PSEL is asserted.",
            ),
        ];
        // Mock verifier: the PSEL claim (a condition mistaken for an obligation)
        // is NOT entailed; the PADDR one is.
        let findings = nli_claim_findings(&cs, |_src, claim| {
            if claim.contains("PSEL") {
                NliVerdict::NotEntailed
            } else {
                NliVerdict::Entailed
            }
        });
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].subject_signal, "PSEL");
        assert_eq!(findings[0].claim_text, "PSEL must be asserted");
    }

    #[test]
    fn nli_claim_findings_abstains_on_unknown() {
        use SignalConstraintKind as K;
        let cs = vec![cons("c1", "PADDR", K::MustBeStable, "x")];
        // Unknown (provider down) → no findings; never a false flag.
        assert!(nli_claim_findings(&cs, |_, _| NliVerdict::Unknown).is_empty());
    }

    #[test]
    fn prompt_states_premise_hypothesis_and_the_one_word_format() {
        let p = entailment_prompt(
            "PREADY is asserted by the Completer.",
            "The Completer drives PREADY.",
        );
        assert!(
            p.contains("PREADY is asserted by the Completer."),
            "premise: {p}"
        );
        assert!(
            p.contains("The Completer drives PREADY."),
            "hypothesis: {p}"
        );
        assert!(p.contains("ENTAILED or NOT_ENTAILED"), "format: {p}");
        // The condition-vs-obligation rule is present (the load-bearing nuance).
        assert!(p.to_uppercase().contains("CONDITION"), "rule: {p}");
    }

    #[test]
    fn parse_not_entailed_beats_substring_entailed() {
        // "NOT_ENTAILED" contains "ENTAILED" — the negative must win.
        assert_eq!(parse_nli_verdict("NOT_ENTAILED"), NliVerdict::NotEntailed);
        assert_eq!(parse_nli_verdict("not entailed"), NliVerdict::NotEntailed);
        assert_eq!(
            parse_nli_verdict("Answer: NOT ENTAILED."),
            NliVerdict::NotEntailed
        );
    }

    #[test]
    fn parse_entailed_and_fail_closed_on_garbage() {
        assert_eq!(parse_nli_verdict("ENTAILED"), NliVerdict::Entailed);
        assert_eq!(parse_nli_verdict("  entailed  "), NliVerdict::Entailed);
        // Anything unclear is Unknown (never silently Entailed).
        assert_eq!(parse_nli_verdict(""), NliVerdict::Unknown);
        assert_eq!(parse_nli_verdict("maybe?"), NliVerdict::Unknown);
        assert_eq!(parse_nli_verdict("yes"), NliVerdict::Unknown);
    }

    #[test]
    fn gate_action_is_additive_and_fail_safe() {
        assert_eq!(gate_action(NliVerdict::Entailed), NliGateAction::Keep);
        assert_eq!(
            gate_action(NliVerdict::NotEntailed),
            NliGateAction::RouteResidual
        );
        // Unknown abstains — a provider outage must not break extraction.
        assert_eq!(gate_action(NliVerdict::Unknown), NliGateAction::Abstain);
    }
}
