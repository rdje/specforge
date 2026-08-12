//! `EXTRACTION-QUALITY-GAUGE.5` — the **LLM-primary, Rust-grounded constraint extractor**: the
//! decisive test of the harness thesis (*replace* the flat Pattern extractor, don't *patch* it).
//!
//! One pass per sentence: the LLM **judges** the structured constraints — `(subject, kind, condition)`
//! — extracting only genuine *signal* requirements and carrying each condition; Rust **grounds** every
//! field — the subject must type as a `Signal` (entity typing, `.1`), the `kind` must parse, the
//! condition must appear in the source (`.2`). What the model proposes ungrounded is dropped. This
//! composes the two proven components into a real extractor.

use crate::ir::entity_typing::{EntityType, is_valid_signal_subject};
use crate::ir::evidence::MessageFieldConstraintRecord;
use crate::ir::source::{AutomationConfidence, SignalConstraintKind, SignalConstraintRecord};
use crate::llm_text::{api_url, call_text_provider};
use crate::provider::VlmProviderArg;
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
    let needle = subject.trim();
    if needle.is_empty() {
        return false;
    }
    let occurrences = token_occurrences(sentence, needle);
    if occurrences.is_empty() {
        return false;
    }
    let spans = conditional_clause_spans(&lowered);
    occurrences
        .iter()
        .all(|&(start, end)| spans.iter().any(|&(cs, ce)| start >= cs && end <= ce))
}

/// Exact, identifier-boundary occurrences of `needle` in `haystack` as byte ranges. Callers may
/// pass lowercased language when matching grammar words, but opaque identifiers remain original.
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
    let needle = subject.trim();
    if needle.is_empty() {
        return false;
    }
    let mut saw_permissive = false;
    for sentence in source_text.split(['.', ';', '\n', '•']) {
        let lowered = sentence.to_ascii_lowercase();
        let has_token = |word: &str| !token_occurrences(&lowered, word).is_empty();
        if token_occurrences(sentence, needle).is_empty() {
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

/// A grounded constraint, typed by what its subject IS (`.FIELD.4`): an obligation on a wire
/// lands in the canonical signal surface; an obligation on a declared message field ("the TagOp
/// field … must be 0b00") lands in the field-scoped surface instead of polluting the signal
/// inventory or being silently dropped.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GroundedConstraint {
    Signal(SignalConstraintRecord),
    Field(MessageFieldConstraintRecord),
}

/// Ground one proposed constraint into a typed record, or drop it. Every gate is SHARED between
/// the signal and field outcomes — a field obligation must pass the same discipline (`.3a`
/// condition-only subject, `.3b` permissive frame, `.8` value recovery, `.2` condition
/// grounding); only the subject's entity type decides which surface the record belongs to.
/// `type_subject` is injected (production = entity typing) so this is testable with no provider;
/// `field_containers` reports the catalog containers declaring a field subject (provenance).
/// `LLM-PRIMARY-PROMOTION.3a` candidate syntax. Signal identities are opaque; only the
/// document's typed entity catalog may decide whether an identifier is a signal or field.
fn is_snap_candidate_token(token: &str) -> bool {
    let mut characters = token.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

/// Case-insensitive "exactly one edit away" (one substitution, insertion, or deletion).
/// Equal-ignoring-case strings return `false` — there is nothing to fix, and the caller only
/// reaches here after the proposed spelling already failed to ground.
fn within_one_edit_ignore_case(a: &str, b: &str) -> bool {
    let a: Vec<char> = a.chars().map(|c| c.to_ascii_lowercase()).collect();
    let b: Vec<char> = b.chars().map(|c| c.to_ascii_lowercase()).collect();
    if a == b {
        return false;
    }
    let (short, long) = if a.len() <= b.len() {
        (&a, &b)
    } else {
        (&b, &a)
    };
    match long.len() - short.len() {
        0 => {
            short
                .iter()
                .zip(long.iter())
                .filter(|(x, y)| x != y)
                .count()
                == 1
        }
        1 => {
            // One insertion in `long`: walk both, allowing exactly one skip in `long`.
            let (mut i, mut j, mut skipped) = (0usize, 0usize, false);
            while i < short.len() && j < long.len() {
                if short[i] == long[j] {
                    i += 1;
                    j += 1;
                } else if skipped {
                    return false;
                } else {
                    skipped = true;
                    j += 1;
                }
            }
            true
        }
        _ => false,
    }
}

/// `LLM-PRIMARY-PROMOTION.3a` — document-grounded recovery of a MODEL-MISSPELLED subject. The
/// model sometimes emits a one-character-off spelling of a signal it otherwise read perfectly
/// (probed live: `SYCOREQ` for the sentence's coordinated `SYSCOREQ and SYSCOACK must be
/// deasserted…`, temp 0, reproducible). A misspelled record either dies downstream at the
/// SemanticIR declared-signal filter (silent recall loss in a measured temporal-gate case) or
/// survives as a phantom name. The trigger is *the proposal does not occur in its
/// own source sentence*: this extractor's subjects are quotes from the sentence, so an absent
/// subject is suspect per se. Snap it to the document's OWN token iff every one of these holds:
/// - the candidate literally appears in the source sentence as a signal-shaped identifier
///   token (`is_snap_candidate_token`),
/// - it types as a valid subject (Signal, or a declared Field) against the document,
/// - it is within ONE edit of the proposal (case-insensitive), and
/// - it is the ONLY such candidate — any ambiguity and no snap happens.
///
/// Nothing is fabricated: the correction target is the sentence's own text, and the corrected
/// subject still passes every downstream grounding gate. No name lists (ADR 0006).
pub fn snap_subject_to_sentence_token(
    proposed: &str,
    sentence: &str,
    type_subject: impl Fn(&str) -> EntityType,
) -> Option<String> {
    let proposed = proposed.trim();
    if proposed.is_empty() {
        return None;
    }
    let mut candidates: Vec<String> = Vec::new();
    for token in sentence.split(|c: char| !(c.is_ascii_alphanumeric() || c == '_')) {
        if !is_snap_candidate_token(token) || candidates.iter().any(|c| c == token) {
            continue;
        }
        if !within_one_edit_ignore_case(proposed, token) {
            continue;
        }
        let token_type = type_subject(token);
        if is_valid_signal_subject(token_type) || token_type == EntityType::Field {
            candidates.push(token.to_string());
        }
    }
    if candidates.len() == 1 {
        candidates.pop()
    } else {
        None
    }
}

#[allow(clippy::too_many_arguments)]
pub fn ground_constraint_typed(
    raw: &RawConstraint,
    sentence: &str,
    statement_id: &str,
    signal_constraint_id: &str,
    field_constraint_id: &str,
    type_subject: impl Fn(&str) -> EntityType,
    is_grounded: impl Fn(&str, &str) -> bool,
    field_containers: impl Fn(&str) -> Vec<String>,
) -> Option<GroundedConstraint> {
    // LLM-PRIMARY-PROMOTION.3a — a subject the model did NOT copy from its own source
    // sentence is suspect (the extractor's subjects are quotes): when the sentence holds
    // exactly one declared token within one edit of it, the model misspelled that token —
    // snap to the document's own spelling BEFORE typing. A subject that does occur in the
    // sentence is never rewritten.
    let mut subject = raw.subject.trim().to_string();
    if !subject.is_empty()
        && token_occurrences(sentence, &subject).is_empty()
        && let Some(snapped) = snap_subject_to_sentence_token(&subject, sentence, &type_subject)
    {
        subject = snapped;
    }
    if subject.is_empty() || token_occurrences(sentence, &subject).is_empty() {
        return None;
    }
    // .1/.FIELD.3 — the subject must type as a Signal or a declared Field (the model may not
    // invent subjects; actors, transactions, table refs, boilerplate are all dropped).
    let subject_type = type_subject(&subject);
    if !is_valid_signal_subject(subject_type) && subject_type != EntityType::Field {
        return None;
    }
    // .3a — a subject that appears only inside the sentence's conditional clauses is the
    // condition's subject, not an obligation's (condition-read-as-obligation) — drop.
    if is_condition_only_subject(&subject, sentence) {
        return None;
    }
    // .3b — a subject framed permissively-only in its source sentences (recommendation/
    // option/hypothetical, no mandatory clause) cannot ground a must_* obligation — drop.
    if is_permissive_only_subject_frame(&subject, sentence) {
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
    Some(if subject_type == EntityType::Field {
        GroundedConstraint::Field(MessageFieldConstraintRecord {
            constraint_id: field_constraint_id.to_string(),
            subject_field: subject.clone(),
            containers: field_containers(&subject),
            constraint_kind,
            target_value: effective_value,
            condition_text,
            negated: false,
            source_text: sentence.to_string(),
            supporting_statement_ids: vec![statement_id.to_string()],
            automation_confidence: AutomationConfidence::Medium,
        })
    } else {
        GroundedConstraint::Signal(SignalConstraintRecord {
            constraint_id: signal_constraint_id.to_string(),
            subject_signal: subject.clone(),
            constraint_kind,
            target_value: effective_value,
            condition_text,
            negated: false,
            source_text: sentence.to_string(),
            supporting_statement_ids: vec![statement_id.to_string()],
            automation_confidence: AutomationConfidence::Medium,
        })
    })
}

/// Ground one proposed constraint into a SIGNAL record, or drop it — the signal-only view of
/// [`ground_constraint_typed`] (a field-typed subject yields `None` here; callers that want the
/// field surface use the typed function).
pub fn ground_constraint(
    raw: &RawConstraint,
    sentence: &str,
    statement_id: &str,
    constraint_id: &str,
    type_subject: impl Fn(&str) -> EntityType,
    is_grounded: impl Fn(&str, &str) -> bool,
) -> Option<SignalConstraintRecord> {
    match ground_constraint_typed(
        raw,
        sentence,
        statement_id,
        constraint_id,
        constraint_id,
        type_subject,
        is_grounded,
        |_| Vec::new(),
    )? {
        GroundedConstraint::Signal(rec) => Some(rec),
        GroundedConstraint::Field(_) => None,
    }
}

/// `.4` — the shared dedup core: collapse records with an identical canonical key into the FIRST
/// occurrence (stable ids and order; the map is lookup-only, so no hash-iteration order can reach
/// the output — `EVIDENCE-DETERMINISM`), merging the duplicates' supporting statements into the
/// kept record so provenance is preserved, never lost.
fn dedup_merge_by<T>(
    records: Vec<T>,
    key_of: impl Fn(&T) -> String,
    supporting_ids: impl Fn(&mut T) -> &mut Vec<String>,
) -> Vec<T> {
    use std::collections::HashMap;
    let mut kept: Vec<T> = Vec::new();
    let mut index_by_key: HashMap<String, usize> = HashMap::new();
    for mut rec in records {
        let key = key_of(&rec);
        match index_by_key.get(&key) {
            Some(&i) => {
                for sid in std::mem::take(supporting_ids(&mut rec)) {
                    if !supporting_ids(&mut kept[i]).contains(&sid) {
                        supporting_ids(&mut kept[i]).push(sid);
                    }
                }
            }
            None => {
                index_by_key.insert(key, kept.len());
                kept.push(rec);
            }
        }
    }
    kept
}

/// Normalized condition component of a dedup key.
fn condition_key(condition_text: Option<&str>) -> String {
    condition_text.unwrap_or("").trim().to_string()
}

fn exact_signal_constraint_key(record: &SignalConstraintRecord) -> String {
    let target = match &record.constraint_kind {
        SignalConstraintKind::MustBeValue { value } => Some(value.as_str()),
        _ => record.target_value.as_deref(),
    };
    format!(
        "{}|{}|{}|{}",
        record.subject_signal.trim(),
        record.constraint_kind.as_str(),
        record.negated,
        target.unwrap_or("").trim(),
    )
}

/// `.4` — collapse exact-duplicate SIGNAL constraints by (subject, kind incl. value, negation,
/// condition): the same obligation re-extracted from the same or another sentence yields ONE
/// record carrying every supporting statement.
pub fn dedup_constraints(records: Vec<SignalConstraintRecord>) -> Vec<SignalConstraintRecord> {
    dedup_merge_by(
        records,
        |rec| {
            format!(
                "{}|{}",
                exact_signal_constraint_key(rec),
                condition_key(rec.condition_text.as_deref())
            )
        },
        |rec| &mut rec.supporting_statement_ids,
    )
}

/// `.FIELD.4` — the same provenance-merging dedup for FIELD constraints, keyed by (field subject,
/// kind incl. value, negation, condition). The subject key is the field name — containers are
/// catalog provenance for the same declared field, not part of the obligation's identity.
pub fn dedup_field_constraints(
    records: Vec<MessageFieldConstraintRecord>,
) -> Vec<MessageFieldConstraintRecord> {
    dedup_merge_by(
        records,
        |rec| {
            let value = match &rec.constraint_kind {
                SignalConstraintKind::MustBeValue { value } => Some(value.as_str()),
                _ => rec.target_value.as_deref(),
            };
            format!(
                "{}|{}|{}|{}|{}",
                rec.subject_field.trim(),
                rec.constraint_kind.as_str(),
                rec.negated,
                value.unwrap_or("").trim(),
                condition_key(rec.condition_text.as_deref())
            )
        },
        |rec| &mut rec.supporting_statement_ids,
    )
}

/// Default text model.
pub const DEFAULT_EXTRACT_MODEL: &str = "qwen2.5:14b-instruct";

/// The LLM extraction prompt — structured value-carrier requirements with conditions, JSON only.
pub fn extraction_prompt(sentence: &str, declared_carriers: &[String]) -> String {
    let mut stable_catalog = Vec::new();
    for carrier in declared_carriers {
        if !stable_catalog.contains(carrier) {
            stable_catalog.push(carrier.clone());
        }
    }
    let declaration_catalog = if stable_catalog.is_empty() {
        "none".to_string()
    } else {
        stable_catalog.join(", ")
    };
    format!(
        "Extract every normative requirement about a declared digital value carrier from the sentence below, as a JSON array. \
         A value carrier is an explicitly named signal/net/pin or register/message field — NOT a table/figure reference, feature, \
         transaction name, state name, or legal text. Treat its document-owned name as opaque and copy it exactly. For each requirement output an object: \
         {{\"subject\": <exact carrier name>, \"kind\": one of must_be_asserted|must_be_deasserted|\
         must_be_stable|must_be_high|must_be_low|must_not_change|must_hold_data|must_be_value, \
         \"condition\": <the when/until/before/after clause from the sentence, or null>, \"value\": \
         <only for must_be_value, else null>}}. A validity requirement — “<signal> must be valid” — \
         is kind must_be_value with value VALID. Use only exact symbols from this current-document \
         declaration catalog: {declaration_catalog}. If the catalog is none or the sentence states no declared-carrier requirement, output \
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
    declared_carriers: &[String],
    provider: VlmProviderArg,
    model: &str,
) -> Vec<RawConstraint> {
    match call_text_provider(
        provider,
        model,
        api_url(provider),
        "",
        sentence,
        &extraction_prompt(sentence, declared_carriers),
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
    fn extraction_prompt_separates_typed_carriers_and_uses_opaque_names() {
        let first = extraction_prompt("orchid must remain stable.", &["orchid".to_string()])
            .replace("orchid", "<carrier>");
        let renamed = extraction_prompt("juniper must remain stable.", &["juniper".to_string()])
            .replace("juniper", "<carrier>");
        assert_eq!(first, renamed);
        assert!(first.contains("register/message field"));
        assert!(first.contains("document-owned name as opaque"));
        assert!(!first.contains("AWVALID"));
        assert!(!first.contains("HTRANS"));
        assert!(extraction_prompt("anything", &[]).contains("catalog is none"));
    }

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
    fn grounding_does_not_treat_case_folded_sibling_as_subject_evidence() {
        let raw = RawConstraint {
            subject: "sig".into(),
            kind: "must_be_asserted".into(),
            condition: None,
            value: None,
        };
        let record = ground_constraint(
            &raw,
            "SIG must be asserted.",
            "s_case",
            "c_case",
            |subject| {
                if matches!(subject, "sig" | "SIG") {
                    EntityType::Signal
                } else {
                    EntityType::Unknown
                }
            },
            is_grounded_in_source,
        );
        assert!(record.is_none());
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

    fn record(
        id: &str,
        subject: &str,
        kind: SignalConstraintKind,
        condition: Option<&str>,
        stmt: &str,
    ) -> SignalConstraintRecord {
        SignalConstraintRecord {
            constraint_id: id.to_string(),
            subject_signal: subject.to_string(),
            constraint_kind: kind,
            target_value: None,
            condition_text: condition.map(str::to_string),
            negated: false,
            source_text: String::new(),
            supporting_statement_ids: vec![stmt.to_string()],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn dedup_collapses_exact_duplicates_and_merges_provenance() {
        // `.4` — the measured AHB shape: HRESP must_be_high re-extracted from the same block.
        let records = vec![
            record("c1", "HRESP", SignalConstraintKind::MustBeHigh, None, "s1"),
            record("c2", "HRESP", SignalConstraintKind::MustBeHigh, None, "s2"),
        ];
        let out = dedup_constraints(records);
        assert_eq!(out.len(), 1);
        assert_eq!(out[0].constraint_id, "c1", "first occurrence wins");
        assert_eq!(
            out[0].supporting_statement_ids,
            vec!["s1".to_string(), "s2".to_string()],
            "duplicate provenance merged, never lost"
        );
    }

    #[test]
    fn dedup_keeps_records_with_different_conditions_or_values() {
        let records = vec![
            record(
                "c1",
                "PWUSER",
                SignalConstraintKind::MustNotChange,
                Some("when HREADY is LOW"),
                "s1",
            ),
            record(
                "c2",
                "PWUSER",
                SignalConstraintKind::MustNotChange,
                None,
                "s2",
            ),
            record(
                "c3",
                "HTRANS",
                SignalConstraintKind::MustBeValue {
                    value: "IDLE".into(),
                },
                None,
                "s3",
            ),
            record(
                "c4",
                "HTRANS",
                SignalConstraintKind::MustBeValue {
                    value: "NONSEQ".into(),
                },
                None,
                "s4",
            ),
        ];
        let out = dedup_constraints(records);
        assert_eq!(
            out.len(),
            4,
            "different condition / value = different facts"
        );
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

    // LLM-PRIMARY-PROMOTION.3a — the model-misspelled-subject snap.

    #[test]
    fn misspelled_subject_snaps_to_the_sentences_own_token() {
        // The probed live shape: the coordinated-subject sentence loses SYSCOREQ because the
        // model writes `SYCOREQ`; the snap recovers the document's own spelling and the
        // grounded record carries the CORRECT subject (with its condition intact).
        let type_subject = |s: &str| {
            if s == "SYSCOREQ" || s == "SYSCOACK" {
                EntityType::Signal
            } else {
                EntityType::Boilerplate
            }
        };
        let raw = RawConstraint {
            subject: "SYCOREQ".into(),
            kind: "must_be_deasserted".into(),
            condition: Some("when ARESETn is asserted".into()),
            value: None,
        };
        let got = ground_constraint_typed(
            &raw,
            "SYSCOREQ and SYSCOACK must be deasserted when ARESETn is asserted.",
            "s1",
            "cs",
            "cf",
            type_subject,
            is_grounded_in_source,
            |_| Vec::new(),
        )
        .expect("a one-edit misspelling of the sentence's own declared token must snap");
        let GroundedConstraint::Signal(rec) = got else {
            panic!("a signal-typed snap must yield a signal record");
        };
        assert_eq!(
            rec.subject_signal, "SYSCOREQ",
            "the DOCUMENT's spelling wins"
        );
        assert_eq!(
            rec.condition_text.as_deref(),
            Some("when ARESETn is asserted")
        );
        assert_eq!(rec.constraint_kind, SignalConstraintKind::MustBeDeasserted);
    }

    #[test]
    fn snap_fires_even_when_a_deferring_judge_would_accept_the_typo() {
        // The PRODUCTION shape that the first cut missed: entity typing DEFERS to the LLM
        // judge for an undeclared, non-structural token, so the misspelled `SYCOREQ` types as
        // Signal and would ground as a phantom name (dying later at the SemanticIR
        // declared-signal filter — the measured AXI temporal-gate failure). The trigger is
        // therefore absence-from-sentence, not typing failure.
        let raw = RawConstraint {
            subject: "SYCOREQ".into(),
            kind: "must_be_deasserted".into(),
            condition: Some("when ARESETn is asserted".into()),
            value: None,
        };
        let got = ground_constraint_typed(
            &raw,
            "SYSCOREQ and SYSCOACK must be deasserted when ARESETn is asserted.",
            "s1",
            "cs",
            "cf",
            |_| EntityType::Signal, // the deferring judge: everything "is" a signal
            is_grounded_in_source,
            |_| Vec::new(),
        )
        .expect("the snap corrects the spelling before typing");
        let GroundedConstraint::Signal(rec) = got else {
            panic!("signal record expected");
        };
        assert_eq!(
            rec.subject_signal, "SYSCOREQ",
            "the document's spelling wins over the model's typo"
        );
    }

    #[test]
    fn snap_requires_an_unambiguous_candidate() {
        // Two sentence tokens are both one edit away and both type as signals — ambiguous,
        // so the honest drop stands.
        let type_subject = |s: &str| {
            if s == "XREQB" || s == "XREQC" {
                EntityType::Signal
            } else {
                EntityType::Boilerplate
            }
        };
        assert_eq!(
            snap_subject_to_sentence_token(
                "XREQA",
                "XREQB and XREQC must be asserted.",
                type_subject
            ),
            None
        );
        // With exactly one candidate the snap succeeds.
        assert_eq!(
            snap_subject_to_sentence_token("XREQA", "XREQB must be asserted.", type_subject)
                .as_deref(),
            Some("XREQB")
        );
    }

    #[test]
    fn snap_is_bounded_to_one_edit_and_document_entity_typing() {
        let type_subject = |s: &str| {
            if s == "SYSCOREQ" || s == "XREQ" || s == "mixedCase" {
                EntityType::Signal
            } else {
                EntityType::Boilerplate
            }
        };
        // More than one edit away → no snap.
        assert_eq!(
            snap_subject_to_sentence_token("SYREQ", "SYSCOREQ must be deasserted.", type_subject),
            None
        );
        // Identifier length and case do not carry semantic authority.
        assert_eq!(
            snap_subject_to_sentence_token("XRE", "XREQ must be asserted.", type_subject)
                .as_deref(),
            Some("XREQ")
        );
        assert_eq!(
            snap_subject_to_sentence_token("Whan", "When XREQ is HIGH.", type_subject),
            None
        );
        assert_eq!(
            snap_subject_to_sentence_token("mixedCas", "mixedCase must be asserted.", type_subject)
                .as_deref(),
            Some("mixedCase")
        );
        // Equal-ignoring-case is not a typo (typing already had its chance) → no snap.
        assert_eq!(
            snap_subject_to_sentence_token("syscoreq", "SYSCOREQ must be deasserted.", |_| {
                EntityType::Boilerplate
            }),
            None
        );
    }

    #[test]
    fn snap_never_rewrites_a_groundable_subject() {
        // A subject that types fine is used verbatim even with a near-twin in the sentence.
        let raw = RawConstraint {
            subject: "SYSCOACK".into(),
            kind: "must_be_deasserted".into(),
            condition: None,
            value: None,
        };
        let got = ground_constraint_typed(
            &raw,
            "SYSCOREQ and SYSCOACK must be deasserted.",
            "s1",
            "cs",
            "cf",
            |_| EntityType::Signal,
            is_grounded_in_source,
            |_| Vec::new(),
        )
        .expect("a groundable subject grounds");
        let GroundedConstraint::Signal(rec) = got else {
            panic!("signal record expected");
        };
        assert_eq!(rec.subject_signal, "SYSCOACK");
    }

    #[test]
    fn field_subject_grounds_to_a_field_constraint_with_catalog_containers() {
        // `.FIELD.4` — the persisted CHI shape: an obligation on the TagOp FIELD routes to the
        // field-scoped surface (subject typed by the `.FIELD.3` catalog), with the catalog's
        // containers as provenance — never into signal_constraints.
        let raw = RawConstraint {
            subject: "TagOp".into(),
            kind: "must_be_value".into(),
            condition: Some("For all other REQ channel messages".into()),
            value: Some("0b00".into()),
        };
        let got = ground_constraint_typed(
            &raw,
            "For all other REQ channel messages, the TagOp field is inapplicable and must \
             be 0b00.",
            "s1",
            "c-signal",
            "c-field",
            |_| EntityType::Field,
            is_grounded_in_source,
            |name| {
                assert_eq!(name, "TagOp");
                vec!["Request channel".to_string()]
            },
        )
        .expect("a declared-field obligation must ground");
        let GroundedConstraint::Field(rec) = got else {
            panic!("a field subject must yield a field-scoped record");
        };
        assert_eq!(rec.constraint_id, "c-field");
        assert_eq!(rec.subject_field, "TagOp");
        assert_eq!(rec.containers, vec!["Request channel".to_string()]);
        assert_eq!(
            rec.constraint_kind,
            SignalConstraintKind::MustBeValue {
                value: "0b00".into()
            }
        );
        assert_eq!(
            rec.condition_text.as_deref(),
            Some("For all other REQ channel messages")
        );
    }

    #[test]
    fn field_constraints_pass_the_same_grounding_gates() {
        // `.FIELD.4` — a field obligation gets NO discipline discount. A field subject that
        // appears only in a conditional clause is the condition's subject (.3a)…
        let when_clause = RawConstraint {
            subject: "TagOp".into(),
            kind: "must_be_high".into(),
            condition: None,
            value: None,
        };
        assert!(
            ground_constraint_typed(
                &when_clause,
                "XDATAV must be HIGH when the TagOp field is zero.",
                "s1",
                "cs",
                "cf",
                |_| EntityType::Field,
                is_grounded_in_source,
                |_| Vec::new(),
            )
            .is_none(),
            "the .3a condition-only gate applies to field subjects too"
        );
        // …and a permissively-framed field proposal cannot ground a must_* obligation (.3b).
        let recommended = RawConstraint {
            subject: "TagOp".into(),
            kind: "must_be_value".into(),
            condition: None,
            value: Some("0b00".into()),
        };
        assert!(
            ground_constraint_typed(
                &recommended,
                "It is recommended that the TagOp field is set to 0b00.",
                "s1",
                "cs",
                "cf",
                |_| EntityType::Field,
                is_grounded_in_source,
                |_| Vec::new(),
            )
            .is_none(),
            "the .3b permissive-frame gate applies to field subjects too"
        );
    }

    #[test]
    fn typed_grounding_still_drops_an_invented_subject() {
        // Neither a signal nor a declared field — a transaction name stays out of BOTH surfaces.
        let raw = RawConstraint {
            subject: "ReadNoSnp".into(),
            kind: "must_be_asserted".into(),
            condition: None,
            value: None,
        };
        let got = ground_constraint_typed(
            &raw,
            "A ReadNoSnp transaction must be issued first.",
            "s1",
            "cs",
            "cf",
            |_| EntityType::Transaction,
            is_grounded_in_source,
            |_| Vec::new(),
        );
        assert!(got.is_none(), "a non-signal, non-field subject is dropped");
    }

    #[test]
    fn the_signal_only_view_still_drops_a_field_subject() {
        // `ground_constraint` is the signal-only view: a field-typed subject never lands in
        // the signal surface (the `.FIELD.3` behavior, preserved through the `.FIELD.4` split).
        let raw = RawConstraint {
            subject: "DBID".into(),
            kind: "must_not_change".into(),
            condition: None,
            value: None,
        };
        let got = ground_constraint(
            &raw,
            "The DBID field must not change.",
            "s1",
            "c1",
            |_| EntityType::Field,
            is_grounded_in_source,
        );
        assert!(got.is_none(), "a field is not a signal-constraint subject");
    }

    fn field_record(
        id: &str,
        subject: &str,
        kind: SignalConstraintKind,
        condition: Option<&str>,
        stmt: &str,
    ) -> MessageFieldConstraintRecord {
        MessageFieldConstraintRecord {
            constraint_id: id.to_string(),
            subject_field: subject.to_string(),
            containers: vec!["Request channel".to_string()],
            constraint_kind: kind,
            target_value: None,
            condition_text: condition.map(str::to_string),
            negated: false,
            source_text: String::new(),
            supporting_statement_ids: vec![stmt.to_string()],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn dedup_field_constraints_collapses_duplicates_and_merges_provenance() {
        // `.FIELD.4` — the same `.4` provenance-merging dedup, on the field surface.
        let records = vec![
            field_record(
                "f1",
                "TagOp",
                SignalConstraintKind::MustBeValue {
                    value: "0b00".into(),
                },
                None,
                "s1",
            ),
            field_record(
                "f2",
                "TagOp",
                SignalConstraintKind::MustBeValue {
                    value: "0b00".into(),
                },
                None,
                "s2",
            ),
            field_record(
                "f3",
                "TagOp",
                SignalConstraintKind::MustBeValue {
                    value: "0b00".into(),
                },
                Some("when tags are present"),
                "s3",
            ),
        ];
        let out = dedup_field_constraints(records);
        assert_eq!(out.len(), 2, "different condition = a different fact");
        assert_eq!(out[0].constraint_id, "f1", "first occurrence wins");
        assert_eq!(
            out[0].supporting_statement_ids,
            vec!["s1".to_string(), "s2".to_string()],
            "duplicate provenance merged, never lost"
        );
        assert_eq!(out[1].constraint_id, "f3");
    }

    #[test]
    fn dedup_keeps_case_distinct_document_identifiers_and_values() {
        let fields = vec![
            field_record(
                "f1",
                "TagOp",
                SignalConstraintKind::MustBeValue {
                    value: "idle".into(),
                },
                None,
                "s1",
            ),
            field_record(
                "f2",
                "tagop",
                SignalConstraintKind::MustBeValue {
                    value: "IDLE".into(),
                },
                None,
                "s2",
            ),
        ];
        assert_eq!(dedup_field_constraints(fields).len(), 2);
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
