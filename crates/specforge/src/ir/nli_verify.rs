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

use crate::llm_text::{api_url, call_text_provider};
use crate::provider::VlmProviderArg;

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
        "Source (from a digital-hardware specification): \"{source}\"\n\
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
        Some(cond) if !cond.is_empty() => {
            // The condition clause is stored connective-stripped (`extract_condition_clause`
            // returns the text *after* "when"/"while"/…), so appending it bare reads as broken
            // English — "PAUSER must be VALID PSELx is asserted" — which the NLI judge flags on
            // phrasing alone. Re-join with a connective ("when" by default) so the claim is a
            // grammatical sentence; if the stored clause already begins with one, keep it.
            // Subordinating conjunctions AND prepositions that already begin a grammatical
            // condition phrase ("for read transfers", "during the access phase") — only a bare
            // clause ("PSELx is asserted") needs a "when" prepended.
            const CONNECTIVES: &[&str] = &[
                "when", "while", "unless", "during", "if", "until", "after", "before", "for", "in",
                "on", "at", "with", "without", "whenever", "once", "as",
            ];
            let first = cond
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_ascii_lowercase();
            if CONNECTIVES.contains(&first.as_str()) {
                format!("{base} {cond}")
            } else {
                format!("{base} when {cond}")
            }
        }
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

/// One NLI pass producing BOTH the not-entailed findings AND the conformal-calibration samples —
/// `(tier_count, is_correct)` where the NLI verdict is the **correctness oracle** (`Entailed` →
/// correct, `NotEntailed` → incorrect, `Unknown` → no label / abstain) and tier-agreement is the
/// confidence axis. This is the NLI-oracle unblock for conformal calibration (TABLE-GRITS-CONFORMAL):
/// it labels EVERY produced fact automatically, so a corpus run yields thousands of `(score,
/// is_correct)` pairs instead of the handful a small per-statement human gold gives. It is *not
/// circular* — tier-agreement (the axis) is independent of NLI (the oracle).
#[derive(Debug, Default)]
pub struct NliConformalPass {
    pub not_entailed: Vec<NliClaimFinding>,
    pub samples: Vec<(f64, bool)>,
}

/// Run the injected verifier once per constraint, collecting findings + calibration samples.
/// `tier_counts` maps `signal_constraint_fact_key(c)` → number of distinct extractor tiers that
/// found the fact. Verifier injected → fully testable without a provider.
pub fn nli_conformal_pass(
    constraints: &[crate::ir::source::SignalConstraintRecord],
    tier_counts: &std::collections::HashMap<String, usize>,
    verify: impl Fn(&str, &str) -> NliVerdict,
) -> NliConformalPass {
    let mut out = NliConformalPass::default();
    for c in constraints {
        let claim_text = constraint_claim_text(c);
        let tier = *tier_counts
            .get(&crate::ir::evidence::signal_constraint_fact_key(c))
            .unwrap_or(&1) as f64;
        match verify(&c.source_text, &claim_text) {
            NliVerdict::Entailed => out.samples.push((tier, true)),
            NliVerdict::NotEntailed => {
                out.samples.push((tier, false));
                out.not_entailed.push(NliClaimFinding {
                    constraint_id: c.constraint_id.clone(),
                    subject_signal: c.subject_signal.clone(),
                    claim_text,
                    source_text: c.source_text.clone(),
                });
            }
            NliVerdict::Unknown => {}
        }
    }
    out
}

/// Build the persistable per-document extraction-quality gauge from ONE completed NLI pass
/// (`EXTRACTION-QUALITY-GAUGE.0`) — derived from [`nli_conformal_pass`]'s output, so persisting
/// the gauge never costs a second sweep of LLM calls. Counts are exact projections of the pass:
/// `entailed`/`not_entailed` are the labeled verdicts, `abstained` is every constraint the oracle
/// could not label (provider error / unclear answer), and the not-entailed ids keep the per-item
/// review routing behind the aggregate (constraint encounter order — deterministic).
pub fn gauge_from_conformal_pass(
    pass: &NliConformalPass,
    constraints_total: usize,
    model: &str,
) -> crate::ir::evidence::ExtractionQualityGaugeRecord {
    crate::ir::evidence::ExtractionQualityGaugeRecord {
        model: model.to_string(),
        constraints_total,
        entailed: pass.samples.iter().filter(|(_, ok)| *ok).count(),
        not_entailed: pass.not_entailed.len(),
        abstained: constraints_total.saturating_sub(pass.samples.len()),
        not_entailed_constraint_ids: pass
            .not_entailed
            .iter()
            .map(|f| f.constraint_id.clone())
            .collect(),
    }
}

/// Whether a persisted gauge still describes the artifact's current `signal_constraints`
/// surface. Stale when the surface size changed since measurement, or when a recorded
/// not-entailed constraint id no longer exists — the latter catches a same-size REPLACEMENT of
/// the surface (`extract-constraints-llm` re-keys ids to `llm_sigcon_*`). A stale gauge is still
/// reported by `validate`, but flagged so nobody mistakes an old measurement for a current one.
pub fn gauge_is_stale(
    gauge: &crate::ir::evidence::ExtractionQualityGaugeRecord,
    constraints: &[crate::ir::source::SignalConstraintRecord],
) -> bool {
    gauge.constraints_total != constraints.len()
        || gauge
            .not_entailed_constraint_ids
            .iter()
            .any(|id| !constraints.iter().any(|c| &c.constraint_id == id))
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
    fn constraint_claim_text_joins_a_bare_condition_clause_with_when() {
        use SignalConstraintKind as K;
        let mut c = cons("c", "PSEL", K::MustBeAsserted, "...");
        // A bare clause (starts with a signal/noun) gets a "when" so the claim is grammatical
        // (else the NLI judge flags "PSEL must be asserted PSELx is asserted" on phrasing alone).
        c.condition_text = Some("PSELx is asserted".into());
        assert_eq!(
            constraint_claim_text(&c),
            "PSEL must be asserted when PSELx is asserted"
        );
        // A clause already beginning with a connective/preposition is kept verbatim.
        c.condition_text = Some("during the access phase".into());
        assert_eq!(
            constraint_claim_text(&c),
            "PSEL must be asserted during the access phase"
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
    fn nli_conformal_pass_labels_samples_by_verdict_and_tier() {
        use SignalConstraintKind as K;
        let c1 = cons("c1", "PSEL", K::MustBeStable, "s"); // Entailed, tier 2
        let c2 = cons("c2", "PADDR", K::MustBeStable, "s"); // NotEntailed, tier 1
        let c3 = cons("c3", "PWDATA", K::MustBeStable, "s"); // Unknown → no sample
        let mut tier_counts = std::collections::HashMap::new();
        tier_counts.insert(crate::ir::evidence::signal_constraint_fact_key(&c1), 2);
        // c2/c3 absent → tier 1.
        let verify = |_src: &str, claim: &str| {
            if claim.contains("PSEL") {
                NliVerdict::Entailed
            } else if claim.contains("PADDR") {
                NliVerdict::NotEntailed
            } else {
                NliVerdict::Unknown
            }
        };
        let pass = nli_conformal_pass(&[c1, c2, c3], &tier_counts, verify);
        // findings: only the NotEntailed one.
        assert_eq!(pass.not_entailed.len(), 1);
        assert_eq!(pass.not_entailed[0].subject_signal, "PADDR");
        // samples: (tier 2, correct) for PSEL, (tier 1, incorrect) for PADDR; PWDATA Unknown → none.
        assert_eq!(pass.samples.len(), 2);
        assert!(pass.samples.contains(&(2.0, true)));
        assert!(pass.samples.contains(&(1.0, false)));
    }

    #[test]
    fn gauge_from_conformal_pass_projects_counts_and_ids() {
        use SignalConstraintKind as K;
        let c1 = cons("c1", "PSEL", K::MustBeStable, "s"); // Entailed
        let c2 = cons("c2", "PADDR", K::MustBeStable, "s"); // NotEntailed
        let c3 = cons("c3", "PWDATA", K::MustBeStable, "s"); // Unknown → abstained
        let verify = |_src: &str, claim: &str| {
            if claim.contains("PSEL") {
                NliVerdict::Entailed
            } else if claim.contains("PADDR") {
                NliVerdict::NotEntailed
            } else {
                NliVerdict::Unknown
            }
        };
        let pass = nli_conformal_pass(&[c1, c2, c3], &std::collections::HashMap::new(), verify);
        let gauge = gauge_from_conformal_pass(&pass, 3, "test-model");
        assert_eq!(gauge.model, "test-model");
        assert_eq!(gauge.constraints_total, 3);
        assert_eq!(gauge.entailed, 1);
        assert_eq!(gauge.not_entailed, 1);
        assert_eq!(
            gauge.abstained, 1,
            "the Unknown verdict is an honest no-label"
        );
        assert_eq!(gauge.not_entailed_constraint_ids, vec!["c2".to_string()]);
    }

    #[test]
    fn gauge_is_stale_on_count_change_or_replaced_ids() {
        use SignalConstraintKind as K;
        let constraints = vec![
            cons("c1", "PSEL", K::MustBeStable, "s"),
            cons("c2", "PADDR", K::MustBeStable, "s"),
        ];
        let gauge = crate::ir::evidence::ExtractionQualityGaugeRecord {
            model: "m".into(),
            constraints_total: 2,
            entailed: 1,
            not_entailed: 1,
            abstained: 0,
            not_entailed_constraint_ids: vec!["c2".into()],
        };
        // Same surface → fresh.
        assert!(!gauge_is_stale(&gauge, &constraints));
        // Surface size changed → stale.
        assert!(gauge_is_stale(&gauge, &constraints[..1]));
        // Same size but the measured ids were replaced (the extract-constraints-llm
        // re-key case) → stale.
        let replaced = vec![
            cons("llm_sigcon_0000", "PSEL", K::MustBeStable, "s"),
            cons("llm_sigcon_0001", "PADDR", K::MustBeStable, "s"),
        ];
        assert!(gauge_is_stale(&gauge, &replaced));
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
        assert!(p.contains("digital-hardware specification"));
        assert!(!p.contains("chip-protocol"));
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
