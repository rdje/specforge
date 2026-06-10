//! `EXTRACTION-QUALITY-GAUGE.5` — the **LLM-primary, Rust-grounded constraint extractor**: the
//! decisive test of the harness thesis (*replace* the flat Pattern extractor, don't *patch* it).
//!
//! One pass per sentence: the LLM **judges** the structured constraints — `(subject, kind, condition)`
//! — extracting only genuine *signal* requirements and carrying each condition; Rust **grounds** every
//! field — the subject must type as a `Signal` (entity typing, `.1`), the `kind` must parse, the
//! condition must appear in the source (`.2`). What the model proposes ungrounded is dropped. This
//! composes the two proven components into a real extractor.

use crate::cli::VlmProviderArg;
use crate::commands::llm_text::{api_url, call_text_provider};
use crate::ir::entity_typing::{EntityType, is_valid_signal_subject};
use crate::ir::source::{AutomationConfidence, SignalConstraintKind, SignalConstraintRecord};
use serde::Deserialize;

/// A constraint as the LLM proposes it (before grounding).
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct RawConstraint {
    pub subject: String,
    /// Snake-case kind (`must_be_asserted`, `must_be_value`, …).
    pub kind: String,
    #[serde(default)]
    pub condition: Option<String>,
    /// For `must_be_value`.
    #[serde(default)]
    pub value: Option<String>,
}

/// Parse the kind (+ value for `must_be_value`) into a [`SignalConstraintKind`]; `None` if unknown.
pub fn parse_kind(kind: &str, value: Option<&str>) -> Option<SignalConstraintKind> {
    Some(match kind.trim().to_ascii_lowercase().as_str() {
        "must_be_asserted" | "asserted" => SignalConstraintKind::MustBeAsserted,
        "must_be_deasserted" | "deasserted" => SignalConstraintKind::MustBeDeasserted,
        "must_be_high" | "high" => SignalConstraintKind::MustBeHigh,
        "must_be_low" | "low" => SignalConstraintKind::MustBeLow,
        "must_be_stable" | "stable" => SignalConstraintKind::MustBeStable,
        "must_not_change" => SignalConstraintKind::MustNotChange,
        "must_hold_data" => SignalConstraintKind::MustHoldData,
        // `.8` — the model's natural spellings for a validity requirement normalize into the
        // value kind (the typed convention: "must be valid" = `must_be_value` + `VALID`).
        "must_be_value" | "must_be_valid" | "valid" => SignalConstraintKind::MustBeValue {
            value: value?.trim().to_string(),
        },
        _ => return None,
    })
}

/// Is this kind spelling in the `must_be_value` family (so a missing value may be recovered
/// from the source sentence rather than silently dropping the constraint)?
fn is_value_kind(kind: &str) -> bool {
    matches!(
        kind.trim().to_ascii_lowercase().as_str(),
        "must_be_value" | "must_be_valid" | "valid"
    )
}

/// Subordinate-clause introducers (universal English clause grammar — no signal/chip vocabulary,
/// ADR 0006). Each takes a clause whose subject states a *situation*, not an obligation.
const CONDITION_CLAUSE_MARKERS: &[&str] = &[
    "when ",
    "whenever ",
    "if ",
    "unless ",
    "while ",
    "until ",
    "after ",
    "before ",
    "provided that ",
    "as long as ",
];

/// `.3a` — does `subject` appear ONLY inside subordinate conditional clauses of `sentence`?
/// Such a token is the *condition's* subject — "ASKSTOP must be LOW **when ACTIVATEACK is
/// LOW**" obligates ASKSTOP, never ACTIVATEACK — so a constraint proposed on it is a
/// condition-read-as-obligation error and must be dropped. A subject with any occurrence in
/// the main clause (e.g. `PWAKEUP must remain asserted … if PWAKEUP and PSELx are asserted`)
/// is kept. Returns `false` when the subject does not occur at all (other gates own that).
pub fn is_condition_only_subject(subject: &str, sentence: &str) -> bool {
    let lowered = sentence.to_ascii_lowercase();
    let needle = subject.trim().to_ascii_lowercase();
    if needle.is_empty() {
        return false;
    }
    let occurrences = token_occurrences(&lowered, &needle);
    if occurrences.is_empty() {
        return false;
    }
    let spans = conditional_clause_spans(&lowered);
    occurrences
        .iter()
        .all(|&(start, end)| spans.iter().any(|&(cs, ce)| start >= cs && end <= ce))
}

/// Identifier-boundary occurrences of `needle` in `haystack` (both lowercased) as byte ranges.
fn token_occurrences(haystack: &str, needle: &str) -> Vec<(usize, usize)> {
    let is_ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut out = Vec::new();
    let mut from = 0;
    while let Some(pos) = haystack[from..].find(needle) {
        let start = from + pos;
        let end = start + needle.len();
        let before_ok = haystack[..start]
            .chars()
            .next_back()
            .is_none_or(|c| !is_ident(c));
        let after_ok = haystack[end..].chars().next().is_none_or(|c| !is_ident(c));
        if before_ok && after_ok {
            out.push((start, end));
        }
        from = start + 1;
    }
    out
}

/// Byte spans of every subordinate conditional clause: from a word-boundary clause marker to
/// the next clause punctuation (`,` `;` `:`) or sentence end (`.` `!` `?`). A marker followed
/// by a gerund (`while driving HREADYOUT LOW …`) introduces a concurrent *action* — the
/// obligation lives on its object — not a condition, and yields no span. Overlaps are fine —
/// only the union matters to the caller.
fn conditional_clause_spans(lowered: &str) -> Vec<(usize, usize)> {
    let is_ident = |c: char| c.is_ascii_alphanumeric() || c == '_';
    let mut spans = Vec::new();
    for marker in CONDITION_CLAUSE_MARKERS {
        let mut from = 0;
        while let Some(pos) = lowered[from..].find(marker) {
            let start = from + pos;
            let boundary_ok = lowered[..start]
                .chars()
                .next_back()
                .is_none_or(|c| !is_ident(c));
            let first_word = lowered[start + marker.len()..]
                .split(|c: char| !is_ident(c))
                .find(|w| !w.is_empty())
                .unwrap_or_default();
            let is_action_coordination = first_word.ends_with("ing");
            if boundary_ok && !is_action_coordination {
                let rest = &lowered[start..];
                let end = rest
                    .find([',', ';', ':', '.', '!', '?'])
                    .map_or(lowered.len(), |p| start + p);
                spans.push((start, end));
            }
            from = start + marker.len();
        }
    }
    spans
}

/// `.3b` — is `subject` framed PERMISSIVELY-ONLY in its source block? "It is recommended that
/// … `HPROT[0]` HIGH", "An alternative implementation would be for HSEL to be tied HIGH" state a
/// recommendation, an option, or a hypothetical — not an obligation — so a `must_*` proposal on
/// that subject is frame-ungrounded. The check is **scoped to the sentences containing the
/// subject** (same sentence split as `is_normative_for_subject`): a mandatory frame
/// (must/shall) in any subject-sentence grounds the proposal and wins outright ("… HEXOKAY
/// **must** be deasserted" behind an "It is permitted …" lead-in), and frame words in
/// *other* sentences of the block never contaminate this subject ("Although an OKAY response
/// **can** be given in a single cycle" does not soften the ERROR-procedure sentences that
/// follow). Universal normative vocabulary only (no signal/chip names, ADR 0006).
pub fn is_permissive_only_subject_frame(subject: &str, source_text: &str) -> bool {
    const PERMISSIVE: &[&str] = &[
        "recommended",
        "permitted",
        "permissible",
        "optional",
        "may",
        "can",
        "could",
        "would",
    ];
    let needle = subject.trim().to_ascii_lowercase();
    if needle.is_empty() {
        return false;
    }
    let mut saw_permissive = false;
    for sentence in source_text.split(['.', ';', '\n', '•']) {
        let lowered = sentence.to_ascii_lowercase();
        let has_token = |word: &str| !token_occurrences(&lowered, word).is_empty();
        if token_occurrences(&lowered, &needle).is_empty() {
            continue;
        }
        if has_token("must") || has_token("shall") {
            return false;
        }
        if PERMISSIVE.iter().any(|w| has_token(w)) {
            saw_permissive = true;
        }
    }
    saw_permissive
}

/// Ground one proposed constraint into a record, or drop it. `type_subject` is injected (production
/// = entity typing) so this is testable with no provider; `is_grounded` guards the condition.
#[allow(clippy::too_many_arguments)]
pub fn ground_constraint(
    raw: &RawConstraint,
    sentence: &str,
    statement_id: &str,
    constraint_id: &str,
    type_subject: impl Fn(&str) -> EntityType,
    is_grounded: impl Fn(&str, &str) -> bool,
) -> Option<SignalConstraintRecord> {
    // .1 — the subject must type as a Signal (the model may not invent non-signal subjects).
    if !is_valid_signal_subject(type_subject(&raw.subject)) {
        return None;
    }
    // .3a — a subject that appears only inside the sentence's conditional clauses is the
    // condition's subject, not an obligation's (condition-read-as-obligation) — drop.
    if is_condition_only_subject(&raw.subject, sentence) {
        return None;
    }
    // .3b — a subject framed permissively-only in its source sentences (recommendation/
    // option/hypothetical, no mandatory clause) cannot ground a must_* obligation — drop.
    if is_permissive_only_subject_frame(&raw.subject, sentence) {
        return None;
    }
    // .8 — a value-kind constraint whose value the model did not echo is no longer silently
    // dropped: recover the value from the SOURCE sentence via the Pattern extractor's own
    // binder grammar ("… must be <value>" — grounded, never fabricated). Unrecoverable → drop.
    let model_value = raw.value.as_deref().map(str::trim).filter(|v| {
        !v.is_empty() && !v.eq_ignore_ascii_case("none") && !v.eq_ignore_ascii_case("null")
    });
    let effective_value = model_value.map(str::to_string).or_else(|| {
        if is_value_kind(&raw.kind) {
            crate::ir::evidence::extract_protocol_state_value(&sentence.to_ascii_lowercase())
        } else {
            None
        }
    });
    let constraint_kind = parse_kind(&raw.kind, effective_value.as_deref())?;
    // .2 — keep a condition only if the source actually contains it.
    let condition_text = raw
        .condition
        .as_ref()
        .map(|c| c.trim().to_string())
        .filter(|c| !c.is_empty() && !c.eq_ignore_ascii_case("none") && is_grounded(c, sentence));
    Some(SignalConstraintRecord {
        constraint_id: constraint_id.to_string(),
        subject_signal: raw.subject.trim().to_string(),
        constraint_kind,
        target_value: effective_value,
        condition_text,
        negated: false,
        source_text: sentence.to_string(),
        supporting_statement_ids: vec![statement_id.to_string()],
        automation_confidence: AutomationConfidence::Medium,
    })
}

/// Default text model.
pub const DEFAULT_EXTRACT_MODEL: &str = "qwen2.5:14b-instruct";

/// The LLM extraction prompt — structured signal requirements with conditions, JSON only.
pub fn extraction_prompt(sentence: &str) -> String {
    format!(
        "Extract every normative requirement about a SIGNAL from the sentence below, as a JSON array. \
         A signal is a wire/pin/field that carries a value — NOT a table/figure reference, a feature, \
         a transaction name, a protocol state, or legal text. For each requirement output an object: \
         {{\"subject\": <signal name>, \"kind\": one of must_be_asserted|must_be_deasserted|\
         must_be_stable|must_be_high|must_be_low|must_not_change|must_hold_data|must_be_value, \
         \"condition\": <the when/until/before/after clause from the sentence, or null>, \"value\": \
         <only for must_be_value, else null>}}. A validity requirement — “<signal> must be valid” — \
         is kind must_be_value with value VALID. If the sentence states no signal requirement, output \
         []. Output ONLY the JSON array.\n\nSentence: {sentence}\n\nJSON:"
    )
}

/// Parse the LLM's JSON array of constraints (fail-safe → empty).
pub fn parse_constraints_json(resp: &str) -> Vec<RawConstraint> {
    let (start, end) = match (resp.find('['), resp.rfind(']')) {
        (Some(s), Some(e)) if e > s => (s, e),
        _ => return Vec::new(),
    };
    serde_json::from_str::<Vec<RawConstraint>>(&resp[start..=end]).unwrap_or_default()
}

/// Production: the LLM proposes the structured constraints for a sentence.
pub fn propose_constraints_llm(
    sentence: &str,
    provider: VlmProviderArg,
    model: &str,
) -> Vec<RawConstraint> {
    match call_text_provider(
        provider,
        model,
        api_url(provider),
        "",
        sentence,
        &extraction_prompt(sentence),
        256,
    ) {
        Ok(resp) => parse_constraints_json(&resp),
        Err(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::condition_extract::is_grounded_in_source;

    #[test]
    fn parses_a_json_array_with_prose_around_it() {
        let resp = "Here you go: [{\"subject\":\"TXSACTIVE\",\"kind\":\"must_be_asserted\",\"condition\":\"after a snoop\"}] done";
        let raws = parse_constraints_json(resp);
        assert_eq!(raws.len(), 1);
        assert_eq!(raws[0].subject, "TXSACTIVE");
    }

    #[test]
    fn grounds_a_signal_constraint_and_keeps_a_grounded_condition() {
        let raw = RawConstraint {
            subject: "TXSACTIVE".into(),
            kind: "must_be_asserted".into(),
            condition: Some("after receiving a snoop".into()),
            value: None,
        };
        let rec = ground_constraint(
            &raw,
            "TXSACTIVE must be asserted after receiving a snoop.",
            "s1",
            "c1",
            |_| EntityType::Signal,
            is_grounded_in_source,
        )
        .expect("grounded");
        assert_eq!(rec.subject_signal, "TXSACTIVE");
        assert_eq!(
            rec.condition_text.as_deref(),
            Some("after receiving a snoop")
        );
    }

    #[test]
    fn drops_a_non_signal_subject() {
        // The model proposed a constraint on a table ref — entity typing rejects it.
        let raw = RawConstraint {
            subject: "B13".into(),
            kind: "must_be_asserted".into(),
            condition: None,
            value: None,
        };
        let got = ground_constraint(
            &raw,
            "See Table B13.25.",
            "s1",
            "c1",
            |_| EntityType::StructuralRef,
            is_grounded_in_source,
        );
        assert!(got.is_none(), "a non-signal subject must be dropped");
    }

    #[test]
    fn condition_only_subject_when_clause_is_detected_and_obligation_subject_is_not() {
        // `.3a` — the measured AXI FP shape: the when-clause subject is not an obligation.
        let s = "- ASKSTOP must be LOW when ACTIVATEACK is LOW.";
        assert!(is_condition_only_subject("ACTIVATEACK", s));
        assert!(!is_condition_only_subject("ASKSTOP", s));
    }

    #[test]
    fn condition_only_subject_unless_clause_is_detected() {
        // `.3a` — the measured AHB FP shape: both the when- and the unless-clause subjects.
        let s = "The HAUSER signal must not change between cycles when HREADY is LOW, \
                 unless HRESP signal is ERROR.";
        assert!(is_condition_only_subject("HRESP", s));
        assert!(is_condition_only_subject("HREADY", s));
        assert!(!is_condition_only_subject("HAUSER", s));
    }

    #[test]
    fn subject_in_both_main_and_conditional_clause_is_kept() {
        // `.3a` — the measured APB shape: PWAKEUP recurs in the if-clause but its main-clause
        // occurrence keeps it; PSELx exists only in the if-clause and is dropped.
        let s = "- PWAKEUP must remain asserted until PREADY is asserted if PWAKEUP and \
                 PSELx are asserted in the same cycle.";
        assert!(!is_condition_only_subject("PWAKEUP", s));
        assert!(is_condition_only_subject("PSELx", s));
        assert!(is_condition_only_subject("PREADY", s));
    }

    #[test]
    fn recommendation_frame_is_permissive_only_for_its_subject() {
        // `.3b` — the probed AHB shape: a recommendation is not an obligation.
        let s = "It is recommended that a Manager sets HPROT[0] HIGH, to indicate a data \
                 access unless the access is specifically known to be an instruction access.";
        assert!(is_permissive_only_subject_frame("HPROT[0]", s));
    }

    #[test]
    fn hypothetical_alternative_frame_is_permissive_only_for_both_subjects() {
        // `.3b` — the probed AHB shape: an alternative-implementation hypothetical.
        let s = "An alternative implementation would be for HSEL to be tied HIGH on the \
                 Subordinates and the interconnect to override HTRANS to IDLE for unselected \
                 Subordinates.";
        assert!(is_permissive_only_subject_frame("HSEL", s));
        assert!(is_permissive_only_subject_frame("HTRANS", s));
    }

    #[test]
    fn permissive_lead_in_with_a_mandatory_subject_sentence_is_kept() {
        // `.3b` — the probed HEXOKAY shape: "permitted" frames the option, but the subject's
        // own sentence carries a REAL mandatory obligation; the proposal must be kept.
        let s = "It is permitted for a Manager to issue an Exclusive Write transfer, which \
                 has not been preceded by an Exclusive Read transfer in the same Exclusive \
                 access sequence. In this case, the Exclusive Write transfer must fail and \
                 the HEXOKAY response signal must be deasserted.";
        assert!(!is_permissive_only_subject_frame("HEXOKAY", s));
    }

    #[test]
    fn incidental_modal_in_another_sentence_does_not_contaminate_the_subject() {
        // `.3b` — the measured over-kill the per-item audit caught: "can" in the OKAY
        // sentence must not soften the ERROR-procedure sentences describing HRESP/HREADYOUT.
        let s = "Although an OKAY response can be given in a single cycle, the ERROR response \
                 requires two cycles. To start the ERROR response, the Subordinate drives \
                 HRESP HIGH to indicate ERROR while driving HREADYOUT LOW to extend the \
                 transfer for one extra cycle.";
        assert!(!is_permissive_only_subject_frame("HRESP", s));
        assert!(!is_permissive_only_subject_frame("HREADYOUT", s));
    }

    #[test]
    fn plain_obligation_sentence_is_not_permissive_only() {
        assert!(!is_permissive_only_subject_frame(
            "PBUSER",
            "PBUSER must be valid when PSEL, PENABLE, and PREADY are asserted."
        ));
        // No frame vocabulary at all — also not permissive-only (the descriptive class is
        // out of scope for this gate).
        assert!(!is_permissive_only_subject_frame(
            "PREADY",
            "PREADY is asserted by the Completer."
        ));
    }

    #[test]
    fn ground_constraint_drops_a_permissive_only_frame_proposal() {
        let raw = RawConstraint {
            subject: "HPROT".into(),
            kind: "must_be_high".into(),
            condition: None,
            value: None,
        };
        let got = ground_constraint(
            &raw,
            "It is recommended that a Manager sets HPROT[0] HIGH, to indicate a data access.",
            "s1",
            "c1",
            |_| EntityType::Signal,
            is_grounded_in_source,
        );
        assert!(
            got.is_none(),
            "a recommendation must not become an obligation"
        );
    }

    #[test]
    fn gerund_action_coordination_is_not_a_condition_and_spans_stop_at_sentence_end() {
        // The measured AHB ERROR-response shape: "while driving HREADYOUT LOW" prescribes a
        // concurrent ACTION on HREADYOUT (no condition span), and the next sentence's
        // main-clause "HREADYOUT is driven HIGH" must not be swallowed by any earlier span.
        let s = "To start the ERROR response, the Subordinate drives HRESP HIGH to indicate \
                 ERROR while driving HREADYOUT LOW to extend the transfer for one extra cycle. \
                 In the next cycle HREADYOUT is driven HIGH to end the transfer.";
        assert!(!is_condition_only_subject("HREADYOUT", s));
        assert!(!is_condition_only_subject("HRESP", s));
    }

    #[test]
    fn condition_span_does_not_cross_into_the_next_sentence() {
        // "when ACTIVATEACK is LOW." ends the span at the period: the next sentence's
        // main-clause PREADY occurrence stays uncovered.
        let s = "ASKSTOP must be LOW when ACTIVATEACK is LOW. PREADY must be HIGH.";
        assert!(is_condition_only_subject("ACTIVATEACK", s));
        assert!(!is_condition_only_subject("PREADY", s));
    }

    #[test]
    fn unconditional_sentence_never_marks_a_condition_only_subject() {
        let s = "For read transfers, the Requester must drive all bits of PSTRB LOW.";
        assert!(!is_condition_only_subject("PSTRB", s));
        // Absent subject: not this gate's call (entity typing owns it).
        assert!(!is_condition_only_subject("PREADY", s));
    }

    #[test]
    fn leading_conditional_clause_is_detected() {
        let s = "When PSEL is asserted, PENABLE must also be asserted.";
        assert!(is_condition_only_subject("PSEL", s));
        assert!(!is_condition_only_subject("PENABLE", s));
    }

    #[test]
    fn ground_constraint_drops_a_condition_only_subject() {
        // End-to-end: the model proposes the when-clause subject as a second obligation.
        let raw = RawConstraint {
            subject: "ACTIVATEACK".into(),
            kind: "must_be_value".into(),
            condition: Some("when ACTIVATEACK is LOW".into()),
            value: Some("LOW".into()),
        };
        let got = ground_constraint(
            &raw,
            "- ASKSTOP must be LOW when ACTIVATEACK is LOW.",
            "s1",
            "c1",
            |_| EntityType::Signal,
            is_grounded_in_source,
        );
        assert!(got.is_none(), "a condition-only subject must be dropped");
    }

    #[test]
    fn recovers_a_validity_value_from_the_source_when_the_model_omits_it() {
        // `.8` — the model names the value kind but echoes no value ("must be valid" has no
        // literal value token to copy); the value is recovered from the SOURCE sentence.
        let raw = RawConstraint {
            subject: "PNSE".into(),
            kind: "must_be_value".into(),
            condition: Some("when PSEL is asserted".into()),
            value: None,
        };
        let rec = ground_constraint(
            &raw,
            "PNSE must be valid when PSEL is asserted.",
            "s1",
            "c1",
            |_| EntityType::Signal,
            is_grounded_in_source,
        )
        .expect("kept via source-grounded value recovery");
        assert_eq!(
            rec.constraint_kind,
            SignalConstraintKind::MustBeValue {
                value: "VALID".into()
            }
        );
        assert_eq!(rec.target_value.as_deref(), Some("VALID"));
    }

    #[test]
    fn normalizes_the_must_be_valid_spelling_into_the_value_kind() {
        // `.8` — the model's natural spelling for a validity requirement is not an unknown kind.
        let raw = RawConstraint {
            subject: "PBUSER".into(),
            kind: "must_be_valid".into(),
            condition: None,
            value: None,
        };
        let rec = ground_constraint(
            &raw,
            "PBUSER must be valid when PSEL, PENABLE, and PREADY are asserted.",
            "s1",
            "c1",
            |_| EntityType::Signal,
            is_grounded_in_source,
        )
        .expect("kept");
        assert_eq!(
            rec.constraint_kind,
            SignalConstraintKind::MustBeValue {
                value: "VALID".into()
            }
        );
    }

    #[test]
    fn drops_a_value_kind_whose_value_is_not_recoverable_from_the_source() {
        // No binder phrase in the sentence → no grounded value → honest drop, never fabricated.
        let raw = RawConstraint {
            subject: "PNSE".into(),
            kind: "must_be_value".into(),
            condition: None,
            value: None,
        };
        let got = ground_constraint(
            &raw,
            "PNSE is described in the protection chapter.",
            "s1",
            "c1",
            |_| EntityType::Signal,
            is_grounded_in_source,
        );
        assert!(
            got.is_none(),
            "an ungroundable value must drop the constraint"
        );
    }

    #[test]
    fn keeps_an_explicit_model_value_over_source_recovery() {
        let raw = RawConstraint {
            subject: "HTRANS".into(),
            kind: "must_be_value".into(),
            condition: None,
            value: Some("NONSEQ".into()),
        };
        let rec = ground_constraint(
            &raw,
            "HTRANS must be NONSEQ for the first transfer.",
            "s1",
            "c1",
            |_| EntityType::Signal,
            is_grounded_in_source,
        )
        .expect("kept");
        assert_eq!(
            rec.constraint_kind,
            SignalConstraintKind::MustBeValue {
                value: "NONSEQ".into()
            }
        );
    }

    #[test]
    fn drops_an_ungrounded_condition_but_keeps_the_constraint() {
        let raw = RawConstraint {
            subject: "PCLK".into(),
            kind: "must_be_stable".into(),
            condition: Some("when the moon is full".into()),
            value: None,
        };
        let rec = ground_constraint(
            &raw,
            "PCLK must be stable.",
            "s1",
            "c1",
            |_| EntityType::Signal,
            is_grounded_in_source,
        )
        .expect("kept");
        assert_eq!(rec.condition_text, None, "hallucinated condition dropped");
    }
}
