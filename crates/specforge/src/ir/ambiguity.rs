//! Flag vague / under-specified spec prose (`AMBIGUITY-PHRASE-DETECTOR`).
//!
//! A **flag-only** detector grounded in the requirements-engineering "weak phrase" literature
//! (Wilson, Rosenberg & Hyatt's requirements-quality work; Berry & Kamsties): certain phrases mark
//! language a tool cannot pin down ("as appropriate", "if necessary", "TBD", …). Surfacing
//! them keeps SpecForge honest — it says where the spec is vague instead of silently treating
//! it as precise. This never changes an extracted fact; it only adds a `validate` finding.

use crate::ir::evidence::ExtractedStatement;

/// Vagueness / under-specification markers from requirements-quality literature plus domain idioms
/// ("implementation-defined", "vendor-specific") that mark genuinely under-specified behaviour.
///
/// The RFC 2119 modal verbs (MUST/SHALL/SHOULD/MAY) are deliberately ABSENT — they carry
/// normative *strength*, not vagueness, and are handled by the constraint/obligation
/// extraction; flagging every "may" would be noise. All entries are lowercase; matching is
/// case-insensitive. Overlapping forms are avoided (e.g. only "but not limited to", not the
/// bare "not limited to") so one occurrence yields one finding.
pub const WEAK_PHRASES: &[&str] = &[
    "as appropriate",
    "as applicable",
    "as required",
    "as necessary",
    "as needed",
    "where appropriate",
    "where applicable",
    "where necessary",
    "if appropriate",
    "if necessary",
    "if possible",
    "if practical",
    "if needed",
    "and/or",
    "etc.",
    "to be determined",
    "to be defined",
    "to be decided",
    "tbd",
    "but not limited to",
    "as a minimum",
    "at a minimum",
    "implementation-defined",
    "implementation defined",
    "implementation specific",
    "vendor-specific",
    "vendor specific",
];

/// One flagged statement: the statement id and the weak phrase that matched.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WeakPhraseFinding {
    pub statement_id: String,
    pub phrase: String,
}

/// Scan statements for vagueness markers. A statement contributes one finding per distinct
/// matched phrase (lexicon order); statements are visited in input order. Pure + deterministic.
pub fn weak_phrase_findings(statements: &[ExtractedStatement]) -> Vec<WeakPhraseFinding> {
    let mut out = Vec::new();
    for stmt in statements {
        let lowered = stmt.text.to_ascii_lowercase();
        for phrase in WEAK_PHRASES {
            if lowered.contains(phrase) {
                out.push(WeakPhraseFinding {
                    statement_id: stmt.statement_id.clone(),
                    phrase: (*phrase).to_string(),
                });
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::evidence::{EvidenceModality, StatementClass};

    fn stmt(id: &str, text: &str) -> ExtractedStatement {
        ExtractedStatement {
            statement_id: id.to_string(),
            class: StatementClass::NormativeStatement,
            modality: EvidenceModality::Text,
            text: text.to_string(),
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        }
    }

    #[test]
    fn flags_a_weak_phrase() {
        let s = vec![stmt(
            "s1",
            "The Completer drives PSLVERR LOW as appropriate.",
        )];
        let f = weak_phrase_findings(&s);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].statement_id, "s1");
        assert_eq!(f[0].phrase, "as appropriate");
    }

    #[test]
    fn does_not_flag_a_precise_statement() {
        // A normative, precise sentence carries no vagueness marker.
        let s = vec![stmt(
            "s2",
            "PADDR must remain stable until the transfer completes.",
        )];
        assert!(weak_phrase_findings(&s).is_empty());
    }

    #[test]
    fn matching_is_case_insensitive() {
        let s = vec![stmt("s3", "The reset value is IMPLEMENTATION-DEFINED.")];
        let f = weak_phrase_findings(&s);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].phrase, "implementation-defined");
    }

    #[test]
    fn modal_verbs_alone_are_not_flagged() {
        // MUST/SHALL/SHOULD/MAY are normative strength, not vagueness — never flagged here.
        let s = vec![stmt(
            "s4",
            "The Requester may drive PSTRB and should keep it stable; it shall not change.",
        )];
        assert!(weak_phrase_findings(&s).is_empty());
    }

    #[test]
    fn the_committed_lexicon_excludes_modal_verbs() {
        for modal in ["must", "shall", "should", "may", "might", "can"] {
            assert!(
                !WEAK_PHRASES
                    .iter()
                    .any(|p| p.split_whitespace().any(|w| w == modal)),
                "modal verb '{modal}' must not be a weak-phrase marker"
            );
        }
    }
}
