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
