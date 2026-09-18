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
    /// `.3j.1.a` — the obligation clause the model read, copied verbatim out of the sentence it was
    /// shown. A record cites its whole statement, so nothing downstream can tell WHICH obligation
    /// inside it produced the record; the model is the only reader that knows, so it is asked. Absent
    /// when the model did not answer, which is not itself a refusal — see [`clause_is_quoted_from`].
    #[serde(default)]
    pub clause: Option<String>,
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

/// `.3j.2.c.i` — the cells of one pipe-table row, outer delimiters dropped. Fewer than three cells
/// is not a matrix row: a two-cell row is a definition list, and its single value cell scopes nothing.
pub(crate) fn table_row_cells(text: &str) -> Option<Vec<&str>> {
    let trimmed = text.trim();
    if !trimmed.starts_with('|') || !trimmed.ends_with('|') {
        return None;
    }
    let cells: Vec<&str> = trimmed
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(str::trim)
        .collect();
    (cells.len() >= 3).then_some(cells)
}

/// `.3j.2.c.i` — a Markdown table's separator row (`|---|:--:|---|`). It carries no obligation and
/// must never be read as one, and it is also what makes the row above it the header.
pub(crate) fn is_separator_row(cells: &[&str]) -> bool {
    cells
        .iter()
        .all(|cell| !cell.is_empty() && cell.chars().all(|c| c == '-' || c == ':' || c == ' '))
}

/// `.3j.2.c.i` — does this cell STATE a binding, as `NAME = VALUE`? Structural, reading no
/// vocabulary: an identifier, `=`, and a value token. A comparison (`==`, `!=`, `>=`, `<=`) asks a
/// question rather than fixing a value and is excluded, and an equals sign loose in prose does not
/// qualify because the identifier must end immediately before it.
pub(crate) fn states_a_binding(cell: &str) -> bool {
    let bytes = cell.as_bytes();
    for (index, byte) in bytes.iter().enumerate() {
        if *byte != b'=' {
            continue;
        }
        if bytes.get(index + 1) == Some(&b'=')
            || (index > 0 && matches!(bytes[index - 1], b'=' | b'!' | b'>' | b'<'))
        {
            continue;
        }
        let left = cell[..index].trim_end();
        let name_len = left
            .chars()
            .rev()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '.')
            .count();
        let value_len = cell[index + 1..]
            .trim_start()
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '.')
            .count();
        if name_len >= 2
            && value_len >= 1
            && left[left.len() - name_len..]
                .starts_with(|c: char| c.is_ascii_alphabetic() || c == '_')
        {
            return true;
        }
    }
    false
}

/// `.3j.2.c.i` — is this span a matrix row whose first cell BINDS the configuration the rest of the
/// row describes?
///
/// `.3j.2.c` measured the whole persisted corpus: 109 spans are such a row, and the seven
/// obligations ever minted from one are all wrong as written. Every one asserts a cell's content
/// unconditionally — *"Subordinate LAPM is tied LOW"* — when the cell holds only under the
/// configuration its own row binds.
///
/// **Refusal, not reconstruction, and the distinction is load-bearing.** Lifting the row key into
/// the record's condition was adjudicated and rejected: a compatibility matrix scopes a cell on
/// **two** axes, and for 5 of the 109 rows the second one is the COLUMN header, which is a different
/// statement that the span was never shown. Reconstructing the row key there would produce an
/// obligation that is still wrong and now carries a condition making it look checked.
///
/// **A condition the model did emit is not evidence the scope was captured**, which is why this
/// refuses regardless of one. The condition check is [`is_grounded_in_source`], a literal-occurrence
/// test against the span — and the row key occurs in the span along with everything else in the row.
/// Measured: the one record of the seven that carries a `condition_text` holds the sentence's own
/// predicate, not its key.
pub fn span_binds_a_configuration_key(sentence: &str) -> bool {
    table_row_cells(sentence)
        .is_some_and(|cells| !is_separator_row(&cells) && states_a_binding(cells[0]))
}

#[allow(clippy::too_many_arguments)]
/// `.3j.1.a` — is the clause the model named actually IN the span it was shown?
///
/// `.3j` measured what happens when a positional subject gate judges an `llm_sigcon_*` record: it
/// refuses 7 of 149 and **4 of the 7 are correct records**, every one because the gate narrows to the
/// FIRST modal clause while the record was minted from a later one. The fix is to stop guessing which
/// obligation was read and to carry it — but a clause the model composed rather than quoted would put
/// the guess back, one layer down and harder to see. So the answer is checkable: the clause must occur
/// **literally** in the sentence, and a proposal that fails is dropped whole.
///
/// Whitespace is normalized on both sides before the comparison, because a model that re-wraps a long
/// clause has still quoted it; nothing else is relaxed. This never re-derives the clause from the
/// record's own kind or value — that would be a second reader of the same statement, and it would
/// disagree with the first exactly where it matters (`.3j.1`).
pub fn clause_is_quoted_from(clause: &str, sentence: &str) -> bool {
    fn squeeze(text: &str) -> String {
        text.split_whitespace().collect::<Vec<_>>().join(" ")
    }
    let clause = squeeze(clause);
    !clause.is_empty() && squeeze(sentence).contains(&clause)
}

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
    // `.3j.2.c.i` — the span is a matrix row that binds the configuration it describes, so no
    // obligation in it holds on its own. Refuse before anything else: this is a property of the
    // span, decided without reading the proposal at all.
    if span_binds_a_configuration_key(sentence) {
        return None;
    }
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
    // .3j.1.a — the model may name the obligation clause it read; when it does, that answer must be
    // a literal quote from the span it was shown. A composed clause is a fabricated span, and a
    // fabricated span is worse than none, because every gate downstream would trust it. Silence is
    // not a refusal here: .3j.1.a carries the clause and refuses a NON-substring, and whether a
    // missing clause should also refuse is .3j.1.b's decision, after it re-measures.
    if let Some(clause) = raw.clause.as_deref()
        && !clause.trim().is_empty()
        && !clause_is_quoted_from(clause, sentence)
    {
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
         <only for must_be_value, else null>, \"clause\": <the exact words of the obligation you read, \
         copied verbatim from the sentence>}}. A validity requirement — “<signal> must be valid” — \
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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
            clause: None,
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

    // `.3j.1.a` — the model names the obligation it read, and the answer is checkable.
    #[test]
    fn a_clause_quoted_from_the_span_is_accepted_however_it_was_wrapped() {
        let sentence = "WTAGUPDATE must be deasserted, and WTAG must be stable.";
        assert!(clause_is_quoted_from("WTAG must be stable", sentence));
        // A model that re-wraps a long clause has still quoted it.
        assert!(clause_is_quoted_from(
            "WTAGUPDATE   must\n be deasserted",
            sentence
        ));
        // Nothing else is relaxed: this is a quote test, not a similarity test.
        assert!(!clause_is_quoted_from("WTAG must be deasserted", sentence));
        assert!(!clause_is_quoted_from("   ", sentence));
    }

    // RED: a composed clause is a fabricated span, and a fabricated span is worse than none —
    // every positional gate downstream would trust it. The proposal is dropped whole.
    #[test]
    fn a_clause_the_model_composed_refuses_the_whole_proposal() {
        let sentence = "WTAGUPDATE must be deasserted, and WTAG must be stable.";
        let composed = RawConstraint {
            subject: "WTAG".into(),
            kind: "must_be_deasserted".into(),
            condition: None,
            value: None,
            // The `.3j` instance in one line: the subject is real and the sentence does bind it,
            // but NOT with this obligation. The model wrote a clause the document never did.
            clause: Some("WTAG must be deasserted".into()),
        };
        assert!(
            ground_constraint(
                &composed,
                sentence,
                "s_clause",
                "c_clause",
                |_| EntityType::Signal,
                is_grounded_in_source,
            )
            .is_none(),
            "a clause absent from its own span must refuse the proposal"
        );
        // …and the same proposal carrying the clause the document DOES state is kept, so the
        // refusal is the clause's doing and not the subject's.
        let quoted = RawConstraint {
            clause: Some("WTAG must be stable".into()),
            kind: "must_be_stable".into(),
            ..composed
        };
        let rec = ground_constraint(
            &quoted,
            sentence,
            "s_clause",
            "c_clause",
            |_| EntityType::Signal,
            is_grounded_in_source,
        )
        .expect("a quoted clause grounds");
        assert_eq!(rec.subject_signal, "WTAG");
    }

    // Silence is not a refusal: `.3j.1.a` carries the clause and refuses a NON-substring. Whether a
    // missing clause should also refuse is `.3j.1.b`'s decision, after it re-measures.
    #[test]
    fn a_proposal_without_a_clause_is_unchanged() {
        let raw = RawConstraint {
            subject: "PCLK".into(),
            kind: "must_be_stable".into(),
            condition: None,
            value: None,
            clause: None,
        };
        assert!(
            ground_constraint(
                &raw,
                "PCLK must be stable.",
                "s_none",
                "c_none",
                |_| EntityType::Signal,
                is_grounded_in_source,
            )
            .is_some(),
            "a proposal that names no clause is not refused by this leaf"
        );
    }

    #[test]
    fn the_prompt_asks_for_the_clause_that_was_read() {
        let prompt = extraction_prompt("PCLK must be stable.", &["PCLK".to_string()]);
        assert!(
            prompt.contains("\"clause\""),
            "the prompt must request the field"
        );
        assert!(
            prompt.contains("copied verbatim from the sentence"),
            "the prompt must ask for a quote, not a paraphrase"
        );
    }

    /// `EXTRACTION-QUALITY-GAUGE.3j.2.b.i` — the PRODUCTION composition: a subject the document's
    /// catalog does not hold, but which THIS span declares in apposition to *signal*, grounds — and the
    /// same subject in a span that does not declare it does not. Composed exactly as
    /// `promote_constraints` composes it, because the grammar being right and the record existing are
    /// two claims.
    #[test]
    fn a_span_local_appositive_grounds_a_subject_the_catalog_does_not_hold() {
        use crate::ir::entity_typing::resolve_unique_document_identifier;

        // The document declares the parameterised spelling only — APB's real shape, opaque here.
        let declared = ["XQPSX".to_string()];
        let ground = |sentence: &str| {
            let span_locals = crate::ir::evidence::locally_declared_signal_identifiers(sentence);
            let type_subject = |proposed: &str| {
                if resolve_unique_document_identifier(
                    proposed,
                    declared
                        .iter()
                        .map(String::as_str)
                        .chain(span_locals.iter().map(String::as_str)),
                )
                .is_some()
                {
                    EntityType::Signal
                } else {
                    EntityType::Unknown
                }
            };
            let raw = RawConstraint {
                subject: "XQPS".to_string(),
                kind: "must_be_asserted".to_string(),
                condition: None,
                value: None,
                clause: None,
            };
            ground_constraint_typed(
                &raw,
                sentence,
                "s_none",
                "c_none",
                "f_none",
                type_subject,
                is_grounded_in_source,
                |_| Vec::new(),
            )
        };

        let grounded = ground("The select signal, XQPS , is asserted, so the transfer begins.")
            .expect("a span-local appositive declaration must ground its own span's subject");
        let GroundedConstraint::Signal(record) = grounded else {
            panic!("a signal-typed subject must route to the signal surface");
        };
        assert_eq!(
            record.subject_signal, "XQPS",
            "the record carries the identifier the span declared, never the catalog's spelling"
        );

        // RED — the identical subject in a span that declares nothing stays ungrounded. The catalog
        // holds XQPSX and is not widened by anything this span says.
        assert!(
            ground("XQPS must be asserted before the transfer begins.").is_none(),
            "the local declaration must not leak out of the span that made it"
        );
    }

    /// `EXTRACTION-QUALITY-GAUGE.3j.2.a.i` — the PRODUCTION composition, not just the rule: the rewrite
    /// `promote_constraints` performs before grounding, followed by grounding itself, must put the
    /// signal's own name on the record. Composed here exactly as the call site composes it, because a
    /// rule that resolves correctly and a record that carries the resolved name are two claims.
    #[test]
    fn the_production_composition_records_the_signal_a_full_width_slice_names() {
        use crate::ir::entity_typing::{
            resolve_full_width_slice_alias, resolve_unique_document_identifier,
        };

        let declared = ["XQRLEN".to_string(), "XQRSNP".to_string()];
        let widths = |name: &str| match name {
            "XQRLEN" => Some(8),
            "XQRSNP" => Some(4),
            _ => None,
        };
        let type_subject = |proposed: &str| {
            if resolve_unique_document_identifier(proposed, declared.iter().map(String::as_str))
                .is_some()
            {
                EntityType::Signal
            } else {
                EntityType::Unknown
            }
        };
        // The call site's rewrite, verbatim.
        let ground = |subject: &str, value: &str, sentence: &str| {
            let mut raw = RawConstraint {
                subject: subject.to_string(),
                kind: "must_be_value".to_string(),
                condition: None,
                value: Some(value.to_string()),
                clause: None,
            };
            if resolve_unique_document_identifier(
                raw.subject.trim(),
                declared.iter().map(String::as_str),
            )
            .is_none()
                && let Some(identity) = resolve_full_width_slice_alias(
                    &raw.subject,
                    declared.iter().map(String::as_str),
                    widths,
                )
            {
                raw.subject = identity.to_string();
            }
            ground_constraint_typed(
                &raw,
                sentence,
                "s_none",
                "c_none",
                "f_none",
                type_subject,
                is_grounded_in_source,
                |_| Vec::new(),
            )
        };

        let grounded = ground(
            "XQRLEN[7:0]",
            "0x00",
            "The burst length must be 1, which means XQRLEN[7:0] must be 0x00.",
        )
        .expect("a full-width slice of a declared signal must ground");
        let GroundedConstraint::Signal(record) = grounded else {
            panic!("a signal-typed subject must route to the signal surface");
        };
        assert_eq!(
            record.subject_signal, "XQRLEN",
            "the record must carry the signal's own name, not the slice spelling"
        );
        assert_eq!(record.target_value.as_deref(), Some("0x00"));

        // RED — one bit of four is left alone, so it stays ungrounded rather than becoming a
        // whole-signal obligation the document never stated.
        assert!(
            ground("XQRSNP[3]", "0", "XQRSNP[3] must be tied 0.").is_none(),
            "a proper sub-slice must not reach the record surface at all"
        );
    }

    /// `.3j.2.a` — how a subject that CARRIES a declared name relates to that signal, given the width
    /// the document states for it. This is the decision the leaf turns on, kept pure so it can be
    /// controlled without a persisted artifact: a slice is an alias for its signal ONLY when it spans
    /// the whole stated width from bit 0. A proper sub-slice is not, because resolving it would
    /// STRENGTHEN the obligation; a slice whose signal states no width is not either, because the
    /// question cannot be answered and an unanswerable question is not a licence to resolve.
    fn carried_subject_class(subject: &str, stated_width: Option<u64>) -> &'static str {
        match (subject_slice(subject), stated_width) {
            (None, _) => "QUALIFIER-ONLY",
            (Some((high, low)), Some(width)) if low == 0 && u64::from(high) + 1 == width => {
                "FULL-WIDTH-ALIAS"
            }
            (Some(_), Some(_)) => "PROPER-SUB-SLICE",
            (Some(_), None) => "SLICE-WIDTH-UNKNOWN",
        }
    }

    /// `.3j.2.a` — the classifier the adjudication rests on, pinned in every direction it must
    /// separate. The RED half is the case the decision turns on: `AWSNOOP[3]` against a stated width
    /// of 4 must NOT read as an alias, because `AWSNOOP must be LOW` is a stronger obligation than
    /// `AWSNOOP[3] must be tied LOW` and the document never stated it.
    #[test]
    fn a_full_width_slice_is_an_alias_and_a_partial_slice_is_not() {
        // The three measured full-width instances: the span covers the stated width from bit 0.
        assert_eq!(
            carried_subject_class("AWCMO[1:0]", Some(2)),
            "FULL-WIDTH-ALIAS"
        );
        assert_eq!(
            carried_subject_class("ARLEN[7:0]", Some(8)),
            "FULL-WIDTH-ALIAS"
        );
        assert_eq!(
            carried_subject_class("ARCACHE[3:0]", Some(4)),
            "FULL-WIDTH-ALIAS"
        );
        // RED: one bit of four is not the signal, however it is spelled.
        assert_eq!(
            carried_subject_class("AWSNOOP[3]", Some(4)),
            "PROPER-SUB-SLICE",
            "a proper sub-slice must never resolve: doing so strengthens the obligation"
        );
        // A span that reaches the top but not bit 0 is still partial.
        assert_eq!(carried_subject_class("X[3:1]", Some(4)), "PROPER-SUB-SLICE");
        // Without a stated width the test cannot be evaluated — a third answer, not an alias.
        assert_eq!(
            carried_subject_class("LAPAS[2:1]", None),
            "SLICE-WIDTH-UNKNOWN"
        );
        assert_eq!(
            carried_subject_class("ARLEN[7:0]", None),
            "SLICE-WIDTH-UNKNOWN"
        );
        // No bracket at all is a qualifier question, not a slice question — and the leaf answered
        // NO to widening on a qualifier, so this class exists to be counted, not to be resolved.
        assert_eq!(
            carried_subject_class("WTAG bits", Some(4)),
            "QUALIFIER-ONLY"
        );
        assert_eq!(
            carried_subject_class("Subordinate LAPM", None),
            "QUALIFIER-ONLY"
        );
        // A bracket this grammar cannot read is not a slice claim.
        assert_eq!(carried_subject_class("X[n:0]", Some(4)), "QUALIFIER-ONLY");
    }

    /// `.3j.2.a` — the bracket span a subject spells, as `(high, low)`. `X[n]` is the one-bit span
    /// `(n, n)`; `X[hi:lo]` is `(hi, lo)`. A subject with no bracket, or one this grammar cannot
    /// read, yields `None` — it is a qualifier question, not a slice question.
    fn subject_slice(subject: &str) -> Option<(u32, u32)> {
        let open = subject.rfind('[')?;
        let close = subject[open..].find(']')? + open;
        let inner = subject[open + 1..close].trim();
        let (high, low) = match inner.split_once(':') {
            Some((high, low)) => (high.trim(), low.trim()),
            None => (inner, inner),
        };
        let (high, low) = (high.parse::<u32>().ok()?, low.parse::<u32>().ok()?);
        (high >= low).then_some((high, low))
    }

    /// `.3j.2.a` — the width `signal` states, read out of the SAME synthesised declaration sentence
    /// the catalog itself is built from (`Signal <name> is width <n>.`, at the start of the text or
    /// after a sentence boundary — `collect_known_signal_names`' own admission rule). `None` when
    /// the document declares the signal without ever stating a width, which is the common case: the
    /// catalog also admits names through `table_signal_declaration_provenance`, and that surface
    /// carries no width at all.
    fn stated_width(ir: &crate::ir::evidence::EvidenceIr, name: &str) -> Option<u64> {
        let needle = format!("signal {} is width ", name.to_ascii_lowercase());
        for statement in &ir.extracted_statements {
            let lowered = statement.text.to_ascii_lowercase();
            for (index, _) in lowered.match_indices(&needle) {
                if index != 0 && !lowered[..index].ends_with(". ") {
                    continue;
                }
                let tail = &lowered[index + needle.len()..];
                let digits: String = tail.chars().take_while(char::is_ascii_digit).collect();
                if let Ok(width) = digits.parse::<u64>()
                    && width > 0
                {
                    return Some(width);
                }
            }
        }
        None
    }

    /// `EXTRACTION-QUALITY-GAUGE.3j.2` — the current grounding closure REFUSES a bare common noun,
    /// and the persisted counter-example proves only that a superseded one did not.
    ///
    /// LTI's `llm_sigcon_0034` carries `subject_signal: "signal"`. It was minted on
    /// `2026-08-12 17:44`, when `promote_constraints` typed a subject through
    /// `classify_entity(gather_entity_evidence(...), |_| EntityType::Signal)` — an LLM judgment with
    /// the model stubbed to answer `Signal`, so every token the document did not positively
    /// contradict became a signal, and no catalog was consulted anywhere on that path (the prompt
    /// took no carrier list either). `declared_signal_catalog` did not exist until `9c38b569`, 78
    /// minutes later. This test pins what the production closure does NOW, in both directions.
    #[test]
    fn a_subject_the_catalog_does_not_declare_is_refused() {
        use crate::ir::entity_typing::resolve_unique_document_identifier;

        // The production closure of `promote_constraints`, over an opaque one-name catalog: the
        // rule never reads the spelling, so the token carries no document identity (ADR 0006).
        let declared = ["XQRVAL".to_string()];
        let type_subject = |proposed: &str| {
            if resolve_unique_document_identifier(proposed, declared.iter().map(String::as_str))
                .is_some()
            {
                EntityType::Signal
            } else {
                EntityType::Unknown
            }
        };
        let proposal = |subject: &str| RawConstraint {
            subject: subject.to_string(),
            kind: "must_be_value".to_string(),
            condition: None,
            value: Some("0".to_string()),
            clause: None,
        };

        assert!(
            ground_constraint_typed(
                &proposal("signal"),
                "The following signal must be 0.",
                "s_none",
                "c_none",
                "f_none",
                type_subject,
                is_grounded_in_source,
                |_| Vec::new(),
            )
            .is_none(),
            "a common noun the catalog does not declare must not ground"
        );
        assert!(
            ground_constraint_typed(
                &proposal("XQRVAL"),
                "The XQRVAL must be 0.",
                "s_none",
                "c_none",
                "f_none",
                type_subject,
                is_grounded_in_source,
                |_| Vec::new(),
            )
            .is_some(),
            "the identical proposal on a DECLARED subject still grounds — the refusal above is \
             the catalog's doing, not the shape's"
        );
    }

    /// `EXTRACTION-QUALITY-GAUGE.3j.2` local measurement, NOT a CI test (`--ignored`): re-runs the
    /// **real** grounding membership test over every persisted `llm_sigcon_*` record and reports
    /// which subjects the catalog would refuse today.
    ///
    /// The leaf exists because a PROXY census was wrong. Scanning the seven signal-bearing
    /// EvidenceIR surfaces flagged 36 of 149 subjects, and 20 after reducing bit-slice spellings —
    /// but its survivors included APB's `PSEL`, which is unquestionably declared. The production
    /// authority is not those surfaces: `promote_constraints` types a subject through
    /// [`resolve_unique_document_identifier`] against `declared_signal_catalog(ir)` first and the
    /// `message_field_records` names second, so this harness rebuilds **that** closure verbatim and
    /// asks it, rather than asking a surface union that no producer consults.
    ///
    /// Read-only over persisted artifacts: no provider, no rebuild, no mutation.
    /// Run (the crate is `specforge-core`: `ir/**` compiles into it by `#[path]`, so
    /// `-p specforge` filters this out and still exits 0 — `COMMIT-GATE-SINGLE-RUN.5`):
    /// `cargo test -p specforge-core --lib llm_constraint_subject_grounding_census -- --ignored --nocapture`
    #[test]
    #[ignore = "local measurement: walks the developer-local generated/evidence_ir corpus"]
    fn llm_constraint_subject_grounding_census_local_measurement() {
        use crate::ir::entity_typing::{
            declared_signal_catalog, resolve_unique_document_identifier,
        };
        use crate::ir::evidence::EvidenceIr;
        use std::collections::BTreeSet;
        use std::path::{Path, PathBuf};

        // The persisted-path contract refuses a traversal component, so the repository root is
        // reached by ancestry rather than by `../..`.
        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("crate dir has a repository root")
            .join("generated")
            .join("evidence_ir");
        let Ok(entries) = std::fs::read_dir(&root) else {
            eprintln!("no local corpus at {} — nothing to measure", root.display());
            return;
        };
        let mut paths: Vec<PathBuf> = entries
            .flatten()
            .map(|entry| entry.path().join("evidence_ir.json"))
            .filter(|path| path.is_file())
            .collect();
        paths.sort();

        let (mut records, mut exact, mut folded, mut field, mut unknown) = (0usize, 0, 0, 0, 0);
        let (mut carries_class, mut truncates_class, mut bare_class) = (0usize, 0usize, 0usize);
        let mut appositive_declared = 0usize;
        let (mut appositive_surface, mut appositive_surface_undeclared) = (0usize, 0usize);
        let (mut qualifier_only, mut full_width_alias) = (0usize, 0usize);
        let (mut proper_sub_slice, mut slice_width_unknown) = (0usize, 0usize);
        let mut resolved_by_shipped_alias = 0usize;
        for path in paths {
            // `load_for_inspection`, not `load_from_path`: every persisted artifact in this corpus
            // is schema 2 and the canonical loader refuses it as proofless. Inspection neutralizes
            // only the three retired protocol carriers, none of which feed the catalog this
            // measurement rebuilds — and a census must not be granted canonical authority anyway.
            let ir = match EvidenceIr::load_for_inspection(&path) {
                Ok(ir) => ir,
                Err(err) => {
                    eprintln!("{}: SKIP ({err})", path.display());
                    continue;
                }
            };
            let subjects: Vec<(String, String, String)> = ir
                .signal_constraints
                .iter()
                .filter(|c| c.constraint_id.starts_with("llm_sigcon_"))
                .map(|c| {
                    (
                        c.constraint_id.clone(),
                        c.subject_signal.clone(),
                        c.source_text.clone(),
                    )
                })
                .collect();
            if subjects.is_empty() {
                continue;
            }
            // The production typing authority, rebuilt exactly as `promote_constraints` builds it.
            let declared_signals = declared_signal_catalog(&ir)
                .into_iter()
                .collect::<BTreeSet<_>>();
            let declared_fields = ir
                .message_field_records
                .iter()
                .map(|f| f.name.clone())
                .collect::<BTreeSet<_>>();
            let widths = crate::ir::evidence::stated_signal_widths(&ir.extracted_statements);

            // `.3j.2.b.i` — the admission surface over this document's distinct spans.
            let mut spans: Vec<&str> = subjects.iter().map(|(_, _, text)| text.as_str()).collect();
            spans.sort_unstable();
            spans.dedup();
            let mut admitted: Vec<String> = Vec::new();
            for span in &spans {
                for token in crate::ir::evidence::locally_declared_signal_identifiers(span) {
                    if !admitted.contains(&token) {
                        admitted.push(token);
                    }
                }
            }
            appositive_surface += admitted.len();
            for token in &admitted {
                if resolve_unique_document_identifier(
                    token,
                    declared_signals.iter().map(String::as_str),
                )
                .is_none()
                {
                    appositive_surface_undeclared += 1;
                    println!("      APPOSITIVE-SURFACE (undeclared) {token:?}");
                }
            }

            let key = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            println!(
                "\n{key}: {} llm_sigcon records / catalog {} signals + {} fields",
                subjects.len(),
                declared_signals.len(),
                declared_fields.len()
            );
            for (id, subject, record_source) in subjects {
                records += 1;
                let signal = resolve_unique_document_identifier(
                    &subject,
                    declared_signals.iter().map(String::as_str),
                );
                let field_hit = resolve_unique_document_identifier(
                    &subject,
                    declared_fields.iter().map(String::as_str),
                );
                match (signal, field_hit) {
                    (Some(identity), _) if identity == subject.trim() => exact += 1,
                    (Some(identity), _) => {
                        folded += 1;
                        println!("  FOLDED   {id}  {subject:?} -> {identity:?}");
                    }
                    (None, Some(identity)) => {
                        field += 1;
                        println!("  FIELD    {id}  {subject:?} -> {identity:?}");
                    }
                    (None, None) => {
                        unknown += 1;
                        // WHY it is refused, which is the whole point: a subject that CARRIES a
                        // declared name is a spelling of a real signal the resolver cannot see
                        // past (a bit slice, a qualifier); a subject that merely TRUNCATES one is
                        // an incomplete spelling; a subject with no declared relative at all is
                        // the bare-common-noun class this leaf was opened for.
                        let folded_subject = subject.to_ascii_lowercase();
                        let carries: Vec<&str> = declared_signals
                            .iter()
                            .map(String::as_str)
                            .filter(|declared| {
                                !token_occurrences(&folded_subject, &declared.to_ascii_lowercase())
                                    .is_empty()
                            })
                            .collect();
                        let truncates: Vec<&str> = declared_signals
                            .iter()
                            .map(String::as_str)
                            .filter(|declared| {
                                declared.len() > folded_subject.trim().len()
                                    && declared
                                        .to_ascii_lowercase()
                                        .starts_with(folded_subject.trim())
                            })
                            .collect();
                        // `.3j.2.b.i` — would the SENTENCE declare this subject locally, in
                        // apposition to the domain word *signal*? That is the declaration form
                        // SPEC-TO-INTENT-ALIGNMENT.7a ruled legitimate, and the deterministic path
                        // already honours it while this one does not.
                        if token_occurrences(&record_source, &subject)
                            .iter()
                            .any(|&span| {
                                crate::ir::evidence::is_same_clause_signal_appositive(
                                    &record_source,
                                    span,
                                )
                            })
                        {
                            appositive_declared += 1;
                            println!("      APPOSITIVE-DECLARED  {id}  {subject:?}");
                        }
                        let (class, witnesses) = if !carries.is_empty() {
                            carries_class += 1;
                            // `.3j.2.a` — WHY it cannot be resolved past, which is the decision.
                            // The longest carried name is the most specific reading; a bracket
                            // span is compared against that name's STATED width, so a slice that
                            // covers the whole signal from bit 0 is an alias for it and any other
                            // slice is not. No stated width means the question cannot be answered,
                            // which is a third answer and not a licence to resolve.
                            let carried = carries
                                .iter()
                                .copied()
                                .max_by_key(|declared| declared.len())
                                .unwrap_or_default();
                            // `.3j.2.a.i` — and what the SHIPPED rule does with it, through the
                            // production function rather than through this harness's classifier.
                            if crate::ir::entity_typing::resolve_full_width_slice_alias(
                                &subject,
                                declared_signals.iter().map(String::as_str),
                                |name| widths.get(name).copied(),
                            )
                            .is_some()
                            {
                                resolved_by_shipped_alias += 1;
                            }
                            let sub = carried_subject_class(&subject, stated_width(&ir, carried));
                            match sub {
                                "QUALIFIER-ONLY" => qualifier_only += 1,
                                "FULL-WIDTH-ALIAS" => full_width_alias += 1,
                                "PROPER-SUB-SLICE" => proper_sub_slice += 1,
                                _ => slice_width_unknown += 1,
                            }
                            println!("  {:>26}  {id}  {subject:?} -> {carried:?}", sub);
                            ("CARRIES-DECLARED", carries)
                        } else if !truncates.is_empty() {
                            truncates_class += 1;
                            ("TRUNCATES-DECLARED", truncates)
                        } else {
                            bare_class += 1;
                            ("NO-DECLARED-RELATIVE", Vec::new())
                        };
                        let witness = witnesses
                            .iter()
                            .take(3)
                            .copied()
                            .collect::<Vec<_>>()
                            .join(", ");
                        println!("  UNGROUNDED  {id}  {subject:?}  [{class}] {witness}");
                    }
                }
            }
        }
        println!(
            "\nTOTAL {records} records: {exact} exact-signal / {folded} case-folded-signal / \
             {field} field / {unknown} ungrounded"
        );
        println!(
            "UNGROUNDED {unknown} = {carries_class} carries a declared name / \
             {truncates_class} truncates one / {bare_class} has no declared relative"
        );
        println!(
            "APPOSITIVE-DECLARED (.3j.2.b.i) {appositive_declared} of the {unknown} ungrounded \
             subjects are declared locally by their own sentence"
        );
        println!(
            "APPOSITIVE-SURFACE (.3j.2.b.i) the grammar declares {appositive_surface} distinct \
             identifiers across every visited span, {appositive_surface_undeclared} of them not in \
             their document's catalog"
        );
        println!(
            "CARRIES-DECLARED {carries_class} = {qualifier_only} qualifier-only / \
             {full_width_alias} full-width alias / {proper_sub_slice} proper sub-slice / \
             {slice_width_unknown} slice whose signal states no width"
        );
        println!(
            "SHIPPED full-width alias (.3j.2.a.i) resolves {resolved_by_shipped_alias} of the \
             {carries_class} carried-name subjects"
        );
    }
    /// `EXTRACTION-QUALITY-GAUGE.3j.2.c` local measurement, NOT a CI test (`--ignored`): how many
    /// persisted spans are matrix rows whose first cell BINDS the configuration the rest of the row
    /// describes, and how many obligations were minted from one without carrying that binding.
    ///
    /// The leaf was opened from six LTI records and diagnosed as a scoping loss whose key is inside
    /// the span. Reading the table the row came from shows that is **half** of it. `Table B12.2` is a
    /// two-dimensional compatibility matrix: the row key binds the MANAGER's properties and the
    /// COLUMN header binds the SUBORDINATE's, so a cell's obligations hold only under both. The
    /// column header is in a different statement, so for that axis it IS a source-assembly gap —
    /// which the leaf explicitly ruled out. This measurement reports both axes for exactly that
    /// reason, and separates the structural exposure (spans that could lose a binding) from the
    /// realised defect (records that did).
    ///
    /// Read-only over persisted artifacts: no provider, no rebuild, no mutation.
    /// Run (the crate is `specforge-core`: `ir/**` compiles into it by `#[path]`):
    /// `cargo test -p specforge-core --lib row_keyed_obligation_population -- --ignored --nocapture`
    #[test]
    #[ignore = "local measurement: walks the developer-local generated/evidence_ir corpus"]
    fn row_keyed_obligation_population_local_measurement() {
        use crate::ir::evidence::EvidenceIr;
        use std::collections::BTreeSet;
        use std::path::{Path, PathBuf};

        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("crate dir has a repository root")
            .join("generated")
            .join("evidence_ir");
        let Ok(entries) = std::fs::read_dir(&root) else {
            eprintln!("no local corpus at {} — nothing to measure", root.display());
            return;
        };
        let mut paths: Vec<PathBuf> = entries
            .flatten()
            .map(|entry| entry.path().join("evidence_ir.json"))
            .filter(|path| path.is_file())
            .collect();
        paths.sort();

        let (mut measured_rows, mut historical_rows) = (0usize, 0usize);
        let (mut measured_docs, mut historical_docs) = (0usize, 0usize);
        let (mut two_axis, mut one_axis) = (0usize, 0usize);
        let (mut records_from_keyed_row, mut without_condition) = (0usize, 0usize);
        let (mut llm_from_keyed_row, mut llm_without_condition) = (0usize, 0usize);
        for path in &paths {
            let key = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let (measured, ir) = match EvidenceIr::load_from_path(path) {
                Ok(ir) => (true, ir),
                Err(_) => match EvidenceIr::load_for_inspection(path) {
                    Ok(ir) => (false, ir),
                    Err(err) => {
                        eprintln!("{key}: UNREADABLE ({err})");
                        continue;
                    }
                },
            };
            // Key-scoped rows, and for each one whether the COLUMN axis binds as well. The header is
            // the row above this table's separator, found by walking back from the row itself.
            let statements = &ir.extracted_statements;
            let mut keyed_rows: BTreeSet<String> = BTreeSet::new();
            // Occurrences, not distinct text: a row repeated in two tables is two spans a model
            // would be asked about, and the axis tallies below must count the same thing the
            // exposure does or the two will not add up.
            let mut doc_occurrences = 0usize;
            let (mut doc_two_axis, mut doc_one_axis) = (0usize, 0usize);
            for (index, statement) in statements.iter().enumerate() {
                let Some(cells) = table_row_cells(&statement.text) else {
                    continue;
                };
                if is_separator_row(&cells) || !states_a_binding(cells[0]) {
                    continue;
                }
                keyed_rows.insert(statement.text.trim().to_string());
                doc_occurrences += 1;
                let header_binds = statements[..index]
                    .iter()
                    .rev()
                    .take(64)
                    .filter_map(|candidate| table_row_cells(&candidate.text))
                    .skip_while(|cells| !is_separator_row(cells))
                    .nth(1)
                    .is_some_and(|header| header.iter().skip(1).any(|cell| states_a_binding(cell)));
                if header_binds {
                    doc_two_axis += 1;
                } else {
                    doc_one_axis += 1;
                }
            }
            if keyed_rows.is_empty() {
                continue;
            }
            // The realised defect: obligations whose whole span is such a row. The SHIPPED
            // production predicate decides, not the harness's own reading of the row — so this
            // number is the reach of `.3j.2.c.i`'s guard and not an estimate of it.
            let (mut doc_records, mut doc_missing, mut doc_llm, mut doc_llm_missing) = (0, 0, 0, 0);
            for constraint in &ir.signal_constraints {
                if !span_binds_a_configuration_key(&constraint.source_text) {
                    continue;
                }
                doc_records += 1;
                let is_llm = constraint.constraint_id.starts_with("llm_sigcon_");
                doc_llm += usize::from(is_llm);
                if constraint.condition_text.is_none() {
                    doc_missing += 1;
                    doc_llm_missing += usize::from(is_llm);
                }
            }
            println!(
                "{stratum:10} {key:70} keyed rows={rows:<4} ({distinct} distinct; \
                 2-axis={two:<3} 1-axis={one:<3}) records={doc_records:<4} \
                 without condition={doc_missing:<4} (llm {doc_llm}/{doc_llm_missing})",
                stratum = if measured { "MEASURED" } else { "HISTORICAL" },
                rows = doc_occurrences,
                distinct = keyed_rows.len(),
                two = doc_two_axis,
                one = doc_one_axis,
            );
            if measured {
                measured_docs += 1;
                measured_rows += doc_occurrences;
            } else {
                historical_docs += 1;
                historical_rows += doc_occurrences;
            }
            two_axis += doc_two_axis;
            one_axis += doc_one_axis;
            records_from_keyed_row += doc_records;
            without_condition += doc_missing;
            llm_from_keyed_row += doc_llm;
            llm_without_condition += doc_llm_missing;
        }
        println!("\n--- EXTRACTION-QUALITY-GAUGE.3j.2.c row-keyed obligation population ---");
        println!(
            "documents holding a key-scoped matrix row      {measured_docs} measured / {historical_docs} historical"
        );
        println!(
            "key-scoped rows (occurrences)                  {measured_rows} measured / {historical_rows} historical"
        );
        assert_eq!(
            two_axis + one_axis,
            measured_rows + historical_rows,
            "every key-scoped row is classified on exactly one axis count"
        );
        println!(
            "  of which the COLUMN axis also binds          {two_axis} (source-assembly gap: the header is a different statement)"
        );
        println!(
            "  of which only the row axis binds             {one_axis} (recoverable inside the span)"
        );
        println!(
            "constraints minted from a key-scoped row       {records_from_keyed_row}  \
             <- the shipped .3j.2.c.i guard's reach, decided by the production predicate"
        );
        println!("  carrying NO condition (the defect)           {without_condition}");
        println!(
            "  of those, LLM-primary records                {llm_from_keyed_row} minted / {llm_without_condition} unconditional"
        );
    }

    /// `.3j.2.c.i` control — the row/binding grammar reads structure, never vocabulary. Opaque
    /// `XQ*` tokens so the rule cannot be reading a real name.
    #[test]
    fn a_key_scoped_matrix_row_is_recognised_by_structure_alone() {
        let row = "| XQA = True XQB = False | Compatible. | XQSIG is tied LOW. | Not compatible |";
        let cells = table_row_cells(row).expect("a four-cell pipe row is a table row");
        assert_eq!(cells.len(), 4);
        assert!(!is_separator_row(&cells));
        assert!(
            states_a_binding(cells[0]),
            "the first cell binds two properties, which is what scopes the rest of the row"
        );
        assert!(
            !states_a_binding(cells[2]),
            "an obligation cell states no binding — otherwise every cell would look like a key"
        );

        assert!(
            table_row_cells("| XQA | XQB |").is_none(),
            "a two-cell row is not a matrix row"
        );
        assert!(
            is_separator_row(&table_row_cells("|---|:--:|---|").expect("separator is a row")),
            "the separator row is what makes the row above it the header"
        );
        assert!(
            !states_a_binding("XQA == True"),
            "a comparison asks a question; it does not fix a value"
        );
        assert!(
            !states_a_binding("XQA != True"),
            "a negated comparison is not a binding either"
        );
        assert!(
            !states_a_binding("see = "),
            "an equals sign with no value token on its right is not a binding"
        );
        assert!(
            states_a_binding("XQ.C = 0"),
            "a dotted property name binds like any other"
        );
    }
    /// `.3j.2.c.i` A/B — the refusal and the leak, pinned in the SAME production function so the
    /// effect is demonstrably the span's doing and not a narrowing of what grounds at all.
    ///
    /// Direction 1: an obligation proposed from a matrix row whose first cell binds a configuration
    /// is refused. Direction 2: the identical proposal, from an ordinary sentence, still grounds.
    /// Neither holds without the other — a rule that refused both would pass direction 1 alone.
    #[test]
    fn an_obligation_from_a_key_scoped_matrix_row_is_refused_and_an_ordinary_one_is_not() {
        let proposal = RawConstraint {
            subject: "XQSIG".to_string(),
            kind: "must_be_value".to_string(),
            condition: None,
            value: Some("LOW".to_string()),
            clause: None,
        };
        let ground = |sentence: &str| {
            ground_constraint_typed(
                &proposal,
                sentence,
                "stmt_0001",
                "llm_sigcon_0000",
                "llm_fieldcon_0000",
                |_| EntityType::Signal,
                is_grounded_in_source_stub,
                |_| Vec::new(),
            )
        };

        assert!(
            ground(
                "| XQA = True XQB = False | Compatible. | XQSIG is tied LOW. | Not compatible |"
            )
            .is_none(),
            "a cell's obligation does not hold on its own — its row binds the configuration it \
             applies under, and nothing in the record would carry that"
        );
        assert!(
            ground("XQSIG is tied LOW.").is_some(),
            "the identical proposal on an ordinary sentence still grounds — the refusal above is \
             the SPAN's doing, not a narrowing of what may be minted"
        );
        assert!(
            ground("| XQSIG is tied LOW. | Always. | Everywhere. |").is_some(),
            "an ordinary table row binds nothing and is not refused; the guard reads the first \
             cell's binding, not the pipes"
        );
        assert!(
            ground("| XQA = True | XQSIG is tied LOW. |").is_some(),
            "a two-cell row is a definition list, not a matrix: its single value cell scopes nothing"
        );
    }

    /// A condition the model emitted is NOT evidence that the row key was captured, so the refusal
    /// does not exempt a proposal that carries one. Measured on the corpus: the single record of the
    /// seven with a `condition_text` holds the sentence's own predicate rather than its key.
    #[test]
    fn a_condition_does_not_exempt_a_key_scoped_matrix_row() {
        let proposal = RawConstraint {
            subject: "XQSIG".to_string(),
            kind: "must_be_value".to_string(),
            condition: Some("XQA = True XQB = False".to_string()),
            value: Some("LOW".to_string()),
            clause: None,
        };
        assert!(
            ground_constraint_typed(
                &proposal,
                "| XQA = True XQB = False | Compatible. | XQSIG is tied LOW. | Not compatible |",
                "stmt_0001",
                "llm_sigcon_0000",
                "llm_fieldcon_0000",
                |_| EntityType::Signal,
                is_grounded_in_source_stub,
                |_| Vec::new(),
            )
            .is_none(),
            "the condition check is literal occurrence in the span, and the key occurs there along \
             with everything else in the row — carrying it proves nothing about the scope"
        );
    }

    /// The condition oracle the A/B uses: literal occurrence, which is exactly what the production
    /// path's `is_grounded_in_source` tests. Stated here so the controls above depend on no
    /// behaviour beyond the guard they are pinning.
    fn is_grounded_in_source_stub(condition: &str, sentence: &str) -> bool {
        sentence.contains(condition)
    }
    /// `EXTRACTION-QUALITY-GAUGE.3j.3` local measurement, NOT a CI test (`--ignored`): the
    /// LLM-primary path's **recall ceiling**, which nothing in the product states.
    ///
    /// `promote_constraints` builds its universe from the distinct `source_text` of the constraints
    /// **already persisted**, one provider call each. It is a refinement pass, so a span the
    /// deterministic extractors never emitted a constraint from is a span the model is never shown —
    /// no prompt, no proposal, no chance. `.3j`'s whole programme measured the model's *precision*
    /// on what it sees; this measures how much it cannot see.
    ///
    /// The denominator is built in two steps so the ceiling is not overstated:
    /// 1. **mandatory-modal statements** — this repository's own RFC-2119 vocabulary, `must`/`shall`
    ///    as whole words plus the modal phrase `required to`, exactly as `is_descriptive_narration_
    ///    binding` reads it. A statement without one is not an obligation and is not a miss.
    /// 2. **…that also name a catalog-declared signal**, decided by `declared_signal_catalog` and
    ///    `token_occurrences` — the production catalog and the production token rule. An obligation
    ///    about something this document never declared as a signal could not have produced a signal
    ///    constraint anyway, so counting it would inflate the gap.
    ///
    /// What remains is the honest population: statements that state an obligation about a signal
    /// this document declares. The ceiling is how many of those the model is ever shown.
    ///
    /// Read-only over persisted artifacts: no provider, no rebuild, no mutation.
    /// Run: `cargo test -p specforge-core --lib llm_recall_ceiling -- --ignored --nocapture`
    #[test]
    #[ignore = "local measurement: walks the developer-local generated/evidence_ir corpus"]
    fn llm_recall_ceiling_local_measurement() {
        use crate::ir::entity_typing::declared_signal_catalog;
        use crate::ir::evidence::EvidenceIr;
        use std::collections::BTreeSet;
        use std::path::{Path, PathBuf};

        /// This repository's mandatory-modal vocabulary, whole-word. The only harness-side grammar
        /// in this measurement; everything else is a production function.
        fn states_an_obligation(text: &str) -> bool {
            let lowered = text.to_ascii_lowercase();
            if lowered.contains("required to") {
                return true;
            }
            lowered
                .split(|c: char| !c.is_ascii_alphanumeric())
                .any(|word| word == "must" || word == "shall")
        }

        let root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .ancestors()
            .nth(2)
            .expect("crate dir has a repository root")
            .join("generated")
            .join("evidence_ir");
        let Ok(entries) = std::fs::read_dir(&root) else {
            eprintln!("no local corpus at {} — nothing to measure", root.display());
            return;
        };
        let mut paths: Vec<PathBuf> = entries
            .flatten()
            .map(|entry| entry.path().join("evidence_ir.json"))
            .filter(|path| path.is_file())
            .collect();
        paths.sort();

        let (mut m_modal, mut m_population, mut m_visited, mut m_docs) = (0usize, 0, 0, 0usize);
        let (mut h_modal, mut h_population, mut h_visited, mut h_docs) = (0usize, 0, 0, 0usize);
        for path in &paths {
            let key = path
                .parent()
                .and_then(|p| p.file_name())
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_default();
            let (measured, ir) = match EvidenceIr::load_from_path(path) {
                Ok(ir) => (true, ir),
                Err(_) => match EvidenceIr::load_for_inspection(path) {
                    Ok(ir) => (false, ir),
                    Err(err) => {
                        eprintln!("{key}: UNREADABLE ({err})");
                        continue;
                    }
                },
            };
            let catalog = declared_signal_catalog(&ir);
            if catalog.is_empty() {
                continue;
            }
            // The LLM path's universe, computed exactly as `promote_constraints` computes it.
            let visited: BTreeSet<&str> = ir
                .signal_constraints
                .iter()
                .map(|c| c.source_text.as_str())
                .collect();
            let (mut modal, mut population, mut reached) = (0usize, 0usize, 0usize);
            for statement in &ir.extracted_statements {
                if !states_an_obligation(&statement.text) {
                    continue;
                }
                modal += 1;
                if !catalog
                    .iter()
                    .any(|name| !token_occurrences(&statement.text, name).is_empty())
                {
                    continue;
                }
                population += 1;
                if visited.contains(statement.text.as_str()) {
                    reached += 1;
                }
            }
            if population == 0 {
                continue;
            }
            println!(
                "{stratum:10} {key:70} modal={modal:<5} obligation-on-declared-signal={population:<5} \
                 shown to the model={reached:<5} ceiling={pct:.1}%",
                stratum = if measured { "MEASURED" } else { "HISTORICAL" },
                pct = 100.0 * reached as f64 / population as f64,
            );
            if measured {
                m_docs += 1;
                m_modal += modal;
                m_population += population;
                m_visited += reached;
            } else {
                h_docs += 1;
                h_modal += modal;
                h_population += population;
                h_visited += reached;
            }
        }
        let pct = |seen: usize, all: usize| {
            if all == 0 {
                0.0
            } else {
                100.0 * seen as f64 / all as f64
            }
        };
        println!("\n--- EXTRACTION-QUALITY-GAUGE.3j.3 LLM-primary recall ceiling ---");
        println!(
            "                                          {:>10}  {:>10}",
            "MEASURED", "HISTORICAL"
        );
        println!("documents with a declared-signal catalog  {m_docs:>10}  {h_docs:>10}");
        println!("statements stating an obligation          {m_modal:>10}  {h_modal:>10}");
        println!(
            "  …about a signal the document declares   {m_population:>10}  {h_population:>10}"
        );
        println!("  …the model is ever shown                {m_visited:>10}  {h_visited:>10}");
        println!(
            "RECALL CEILING                            {:>9.1}%  {:>9.1}%",
            pct(m_visited, m_population),
            pct(h_visited, h_population)
        );
        println!(
            "unreachable spans (no prompt, ever)       {:>10}  {:>10}",
            m_population - m_visited,
            h_population - h_visited
        );
    }
}
