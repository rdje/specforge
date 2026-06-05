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
/// hypothesis) — e.g. `<signal> must be stable`, `<signal> must be <value> for
/// read transfers`. The constraint's own `source_text` is the
/// premise. Any `condition_text` is **carried into the claim** so a conditional
/// constraint is judged against the same condition the source states (otherwise
/// "PSTRB must be LOW" reads as not-entailed by "for read transfers, … LOW").
pub fn constraint_claim_text(c: &crate::ir::source::SignalConstraintRecord) -> String {
    use crate::ir::source::SignalConstraintKind as K;
    // `MustNotChange` is inherently negative; the rest take the `negated` flag.
    let base = if matches!(c.constraint_kind, K::MustNotChange) {
        format!("{} must not change", c.subject_signal)
    } else {
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
    };
    match c.condition_text.as_deref().map(str::trim) {
        Some(cond) if !cond.is_empty() => format!("{base} {cond}"),
        _ => base,
    }
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

/// Render an `ActorContract`'s obligation as an NLI hypothesis — or `None` when
/// the obligation cannot be phrased as a clean claim (in which case it is **not
/// gated**, never NLI-checked against a claim we cannot state faithfully).
pub fn obligation_claim_text(c: &crate::ir::contract::ActorContract) -> Option<String> {
    use crate::ir::contract::{EventExpr, Obligation, Window};
    let sig_of = |e: &EventExpr| -> Option<String> {
        match e {
            EventExpr::Edge { signal, .. } | EventExpr::Level { signal, .. } => {
                Some(signal.clone())
            }
            EventExpr::HandshakeFire { valid, .. } => Some(valid.clone()),
            EventExpr::Start | EventExpr::PhaseBoundary { .. } => None,
        }
    };
    match &c.obligation {
        Obligation::Drive { signal, value } => Some(format!("{signal} must be {value}")),
        Obligation::Stable { signal, .. } => Some(format!("{signal} must be stable")),
        Obligation::Eventually {
            target,
            window: Window::Within { max, .. },
        } => sig_of(target).map(|s| format!("{s} must occur within {max} cycles")),
        Obligation::HandshakeBarrier { valid, ready } => {
            Some(format!("the {valid}/{ready} handshake must complete"))
        }
        Obligation::Mutex { a, b } => Some(format!("{a} and {b} are mutually exclusive")),
        // Un-phrasable as a single clean claim → not gated.
        Obligation::Eventually { .. }
        | Obligation::Observe { .. }
        | Obligation::Persist { .. }
        | Obligation::Sequence { .. }
        | Obligation::OrderedBefore { .. } => None,
    }
}

/// Run the NLI gate over a contract set: each *phrasable* contract is verified
/// (premise = its `provenance.source_text`), and `NotEntailed` ones are
/// **demoted** — removed from the kept set and returned as
/// `ResidualDecisionPacket`s. `Entailed`, `Unknown`, and un-phrasable contracts
/// are **kept** (the existing pipeline stands). Verifier injected → hermetic.
/// Demote-not-delete: a verifier error costs a review item, not a lost fact.
pub fn nli_gate_contracts(
    contracts: Vec<crate::ir::contract::ActorContract>,
    verify: impl Fn(&str, &str) -> NliVerdict,
) -> (
    Vec<crate::ir::contract::ActorContract>,
    Vec<crate::ir::source::ResidualDecisionPacket>,
) {
    let mut kept = Vec::new();
    let mut residuals = Vec::new();
    for c in contracts {
        match obligation_claim_text(&c) {
            Some(claim)
                if matches!(
                    verify(&c.provenance.source_text, &claim),
                    NliVerdict::NotEntailed
                ) =>
            {
                residuals.push(crate::ir::source::ResidualDecisionPacket {
                    packet_id: format!("{NLI_RESIDUAL_PREFIX}{}", c.contract_id),
                    question: format!(
                        "Does the source sentence support the contract claim '{claim}'?"
                    ),
                    why_unresolved: format!(
                        "NLI: the source sentence does not entail the contract claim '{claim}' \
                         — demoted to a residual for review (NLI-INTENT-GATE)"
                    ),
                    automation_confidence: c.automation_confidence,
                    candidate_interpretations: vec![],
                });
            }
            _ => kept.push(c),
        }
    }
    (kept, residuals)
}

/// Apply the NLI gate to a built `IntentIr` in place: demote NotEntailed
/// contracts into `residual_decisions`. Returns the number demoted.
pub fn apply_nli_gate(
    intent_ir: &mut crate::ir::intent::IntentIr,
    verify: impl Fn(&str, &str) -> NliVerdict,
) -> usize {
    let contracts = std::mem::take(&mut intent_ir.actor_contracts);
    let (kept, residuals) = nli_gate_contracts(contracts, verify);
    let demoted = residuals.len();
    intent_ir.actor_contracts = kept;
    intent_ir.residual_decisions.extend(residuals);
    demoted
}

/// Prefix on the `packet_id` of every residual the NLI gate creates.
pub const NLI_RESIDUAL_PREFIX: &str = "nli_unentailed_";

/// Count the residual decisions the NLI gate produced (read-only; no LLM call —
/// the demotion is already recorded in the artifact). Powers the
/// `nli_demoted_contracts` validate metric.
pub fn nli_demoted_count(residuals: &[crate::ir::source::ResidualDecisionPacket]) -> usize {
    residuals
        .iter()
        .filter(|r| r.packet_id.starts_with(NLI_RESIDUAL_PREFIX))
        .count()
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
    fn constraint_claim_text_carries_condition() {
        use SignalConstraintKind as K;
        // A conditional constraint must state its condition, so it is judged
        // against the same condition the source states (precision; the real-APB
        // run flagged "PSTRB must be LOW" purely because the claim dropped "for
        // read transfers").
        let mut c = cons(
            "c",
            "PSTRB",
            K::MustBeLow,
            "For read transfers, drive PSTRB LOW.",
        );
        c.condition_text = Some("for read transfers".into());
        assert_eq!(
            constraint_claim_text(&c),
            "PSTRB must be LOW for read transfers"
        );
        // An empty/whitespace condition adds nothing.
        c.condition_text = Some("  ".into());
        assert_eq!(constraint_claim_text(&c), "PSTRB must be LOW");
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

    // NLI-INTENT-GATE: obligation claim rendering + the demote-to-residual gate.

    fn contract(
        id: &str,
        source: &str,
        obligation: crate::ir::contract::Obligation,
    ) -> crate::ir::contract::ActorContract {
        use crate::ir::contract::{
            ContractKind, ContractProvenance, EvidenceModality, LoweringDisposition,
        };
        use crate::ir::semantic::ClockEdge;
        crate::ir::contract::ActorContract {
            contract_id: id.into(),
            source_rule_id: Some(id.into()),
            actor_name: Some("A".into()),
            kind: ContractKind::Guarantee,
            guard: None,
            guard_candidates: vec![],
            obligation,
            clock_signal: None,
            edge: ClockEdge::Rising,
            channel: None,
            phase: None,
            provenance: ContractProvenance {
                supporting_statement_ids: vec![],
                source_text: source.into(),
                modality: EvidenceModality::Prose,
            },
            lowering: LoweringDisposition::Lowerable,
            automation_confidence: crate::ir::source::AutomationConfidence::Medium,
        }
    }

    #[test]
    fn obligation_claim_text_renders_phrasable_and_none() {
        use crate::ir::contract::Obligation;
        let drive = contract(
            "c1",
            "x",
            Obligation::Drive {
                signal: "PADDR".into(),
                value: "1".into(),
            },
        );
        assert_eq!(
            obligation_claim_text(&drive).as_deref(),
            Some("PADDR must be 1")
        );
        // Observe is un-phrasable as a clean claim → None (not gated).
        let observe = contract(
            "c2",
            "x",
            Obligation::Observe {
                signal: "PADDR".into(),
            },
        );
        assert_eq!(obligation_claim_text(&observe), None);
    }

    #[test]
    fn nli_gate_demotes_not_entailed_to_residual() {
        use crate::ir::contract::Obligation;
        let cs = vec![
            contract(
                "c1",
                "PADDR must be stable.",
                Obligation::Stable {
                    signal: "PADDR".into(),
                    during: crate::ir::contract::Window::SameCycle,
                },
            ),
            contract(
                "c2",
                "PBUSER must be valid when PSEL is asserted.",
                Obligation::Drive {
                    signal: "PSEL".into(),
                    value: "1".into(),
                },
            ),
        ];
        // Mock verifier: the PSEL contract (condition mistaken for an obligation)
        // is NOT entailed; the PADDR one is.
        let (kept, residuals) = nli_gate_contracts(cs, |_src, claim| {
            if claim.contains("PSEL") {
                NliVerdict::NotEntailed
            } else {
                NliVerdict::Entailed
            }
        });
        assert_eq!(kept.len(), 1, "the entailed PADDR contract is kept");
        assert_eq!(kept[0].contract_id, "c1");
        assert_eq!(
            residuals.len(),
            1,
            "the not-entailed PSEL contract is demoted"
        );
        assert_eq!(residuals[0].packet_id, "nli_unentailed_c2");
        assert!(residuals[0].why_unresolved.contains("does not entail"));
    }

    #[test]
    fn nli_gate_keeps_unphrasable_and_unknown() {
        use crate::ir::contract::Obligation;
        // Un-phrasable (Observe) → kept, verifier never consulted.
        let observe = vec![contract(
            "c1",
            "x",
            Obligation::Observe { signal: "S".into() },
        )];
        let (kept, residuals) = nli_gate_contracts(observe, |_, _| {
            panic!("must not verify an un-phrasable obligation")
        });
        assert_eq!(kept.len(), 1);
        assert!(residuals.is_empty());
        // Unknown (provider down) → kept, never demoted.
        let drive = vec![contract(
            "c2",
            "x",
            Obligation::Drive {
                signal: "S".into(),
                value: "1".into(),
            },
        )];
        let (kept2, residuals2) = nli_gate_contracts(drive, |_, _| NliVerdict::Unknown);
        assert_eq!(kept2.len(), 1);
        assert!(residuals2.is_empty());
    }

    #[test]
    fn nli_demoted_count_only_counts_gate_residuals() {
        use crate::ir::source::ResidualDecisionPacket;
        let packet = |id: &str| ResidualDecisionPacket {
            packet_id: id.into(),
            question: "q".into(),
            why_unresolved: "w".into(),
            automation_confidence: crate::ir::source::AutomationConfidence::Medium,
            candidate_interpretations: vec![],
        };
        let residuals = vec![
            packet("nli_unentailed_c1"),
            packet("temporal_residual_c2"), // a non-NLI residual must not count
            packet("nli_unentailed_c3"),
        ];
        assert_eq!(nli_demoted_count(&residuals), 2);
        assert_eq!(nli_demoted_count(&[]), 0);
    }
}
