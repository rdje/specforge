//! `WIRE-BASED-100.6/.7` — derived grounding filters that drop OVER-GENERATED facts (the APB
//! precision defects). Universal-language structure only — no hardcoded chip-spec names (ADR 0006):
//! function words can't be actors; a normative obligation must be grounded in a normative modal.

/// Universal English function words — a single one of these can never be a relation ACTOR
/// (e.g. "For read transfers…" mis-extracted "For" as an actor).
const FUNCTION_WORDS: &[&str] = &[
    "for", "when", "if", "while", "the", "a", "an", "to", "of", "in", "on", "at", "by", "as",
    "and", "or", "but", "with", "that", "this", "these", "those", "it", "its", "from", "into",
    "each", "any", "all", "both", "is", "are", "be",
];

/// Document meta-words — a phrase naming the spec/protocol/document ITSELF is not an agent
/// (e.g. "APB protocol" mis-extracted as an actor).
const META_WORDS: &[&str] = &[
    "protocol",
    "specification",
    "interface",
    "architecture",
    "document",
    "standard",
    "manual",
];

/// A relation actor is valid only if it is a real AGENT — not a bare function word, not the spec's
/// own name. `.6`.
pub fn is_valid_actor(actor: &str) -> bool {
    let a = actor.trim().to_ascii_lowercase();
    if a.is_empty() {
        return false;
    }
    let words: Vec<&str> = a.split_whitespace().collect();
    if words.len() == 1 && FUNCTION_WORDS.contains(&words[0]) {
        return false;
    }
    if words.iter().any(|w| META_WORDS.contains(w)) {
        return false;
    }
    true
}

/// Universal normative modals — an obligation must be grounded in one of these in its source. A
/// purely descriptive sentence ("PRDATA for read data") carries none.
const NORMATIVE_MODALS: &[&str] = &[
    "must",
    "shall",
    "required to",
    "is required",
    "are required",
    "mandatory",
    "prohibited",
    "responsible for",
    "not permitted",
    "is not allowed",
];

/// A constraint's source is NORMATIVE (not merely descriptive) iff it carries a normative modal.
pub fn is_normative_source(source_text: &str) -> bool {
    let s = source_text.to_ascii_lowercase();
    NORMATIVE_MODALS.iter().any(|m| s.contains(m))
}

/// `.7` (precise) — the obligation must be normative *for its subject*: the SENTENCE that mentions
/// `subject` must itself carry a normative modal. Drops a constraint hallucinated from a descriptive
/// clause even when a *different* clause of the same source is normative (e.g. "PRDATA for read
/// data … the buses must have the same width" ⇏ "PRDATA must be stable").
pub fn is_normative_for_subject(source_text: &str, subject: &str) -> bool {
    let subj = subject.trim().to_ascii_lowercase();
    if subj.is_empty() {
        return is_normative_source(source_text);
    }
    source_text
        .split(['.', ';', '\n', '•'])
        .map(|s| s.to_ascii_lowercase())
        .any(|s| s.contains(&subj) && NORMATIVE_MODALS.iter().any(|m| s.contains(m)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_function_word_and_meta_actors() {
        assert!(!is_valid_actor("For"), "a preposition is not an actor");
        assert!(
            !is_valid_actor("APB protocol"),
            "the spec name is not an agent"
        );
        assert!(!is_valid_actor("the specification"));
        assert!(is_valid_actor("Completer"));
        assert!(is_valid_actor("Requester"));
        assert!(is_valid_actor("Home Node"));
    }

    #[test]
    fn normative_source_requires_a_modal() {
        assert!(is_normative_source(
            "PSTRB must be driven LOW for read transfers."
        ));
        assert!(is_normative_source("The Completer shall assert PREADY."));
        // descriptive sentence — no modal → not a source for an obligation.
        assert!(!is_normative_source(
            "The APB protocol has two independent data buses, PRDATA for read data and PWDATA."
        ));
    }

    #[test]
    fn normative_for_subject_is_clause_scoped() {
        // The modal ("must have the same width") is about the buses, NOT PRDATA's stability — so
        // "PRDATA must_be_stable" is correctly rejected even though the source contains "must".
        let src = "The APB protocol has two independent data buses, PRDATA for read data and PWDATA \
                   for write data. The read and write data buses must have the same width.";
        assert!(!is_normative_for_subject(src, "PRDATA"));
        // A real obligation on the subject passes.
        assert!(is_normative_for_subject(
            "For read transfers, the Requester must drive all bits of PSTRB LOW.",
            "PSTRB"
        ));
    }
}
