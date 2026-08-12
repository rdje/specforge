//! Bounded-LLM relation extraction (`PURE-NLP-INTENT-EXTRACTION`) — a pure-NLP path
//! parallel to the deterministic pattern extractor.
//!
//! The model **proposes** `actor → signal` relations from a sentence; the document's own
//! declared signals **decide** which survive — a proposal naming a signal the spec never
//! declares is dropped. So the model generalizes the *method* (reading constructions the
//! patterns miss) while ADR 0006 holds: every name comes from the sentence and the declared
//! set, none is memorized. Classifying the verb as drive/read is the model's job (its
//! strength); validating the names is the engine's.
//!
//! This is the first increment of the formerly-parked pure-NLP tree, unparked once the
//! grammar engine was validated on real data (`REEXTRACTION-REMEASURE`). It is opt-in and
//! additive — the deterministic extractor remains the default.

use crate::cli::VlmProviderArg;
use crate::commands::llm_text::{api_url, call_text_provider};
use crate::ir::source::{ActorSignalRelation, AutomationConfidence, RelationKind};
use std::collections::HashSet;

/// Build the extraction prompt. The model returns a JSON array of
/// `{"actor","relation","signal"}` (relation = `drives`|`reads`). The known signals are
/// listed so the model uses the document's own names rather than inventing them.
pub fn relation_extract_prompt(sentence: &str, known_signals: &[String]) -> String {
    let mut signals = known_signals.to_vec();
    signals.sort(); // deterministic prompt
    format!(
        "You extract actor-signal relationships from one hardware-specification sentence.\n\
         An actor (a Manager, Requester, Completer, Subordinate, ...) either DRIVES \
         (sources/changes) or READS (observes/samples) a signal.\n\
         Known signals: {}\n\
         Sentence: \"{}\"\n\
         Return ONLY a JSON array of objects \
         {{\"actor\":\"<name>\",\"relation\":\"drives\"|\"reads\",\"signal\":\"<one known signal>\"}}. \
         Use only signals from the Known signals list; take actor names from the sentence. \
         If there are none, return [].",
        signals.join(", "),
        sentence.replace('"', "'"),
    )
}

#[derive(serde::Deserialize)]
struct RawRelation {
    #[serde(default)]
    actor: String,
    #[serde(default)]
    relation: String,
    #[serde(default)]
    signal: String,
}

/// Parse the model's JSON array into **validated** relations. Bounded doctrine: a proposal
/// survives only if its signal is in `known_signals` (case-insensitive), its relation is
/// `drives`/`reads`, and its actor is non-empty. Duplicates are collapsed. Never panics —
/// malformed output yields an empty result.
pub fn parse_nlp_relations(
    response: &str,
    statement_id: &str,
    known_signals: &HashSet<String>,
) -> Vec<ActorSignalRelation> {
    let json = match (response.find('['), response.rfind(']')) {
        (Some(a), Some(b)) if b > a => &response[a..=b],
        _ => return Vec::new(),
    };
    let raws: Vec<RawRelation> = serde_json::from_str(json).unwrap_or_default();
    let mut out = Vec::new();
    let mut seen: HashSet<(String, String, bool)> = HashSet::new();
    for raw in raws {
        let actor = raw.actor.trim().to_string();
        let proposed_signal = raw.signal.trim();
        let signal = if known_signals.contains(proposed_signal) {
            proposed_signal.to_string()
        } else {
            let mut matches = known_signals
                .iter()
                .filter(|known| known.eq_ignore_ascii_case(proposed_signal));
            let Some(canonical) = matches.next() else {
                continue;
            };
            if matches.next().is_some() {
                continue;
            }
            canonical.clone()
        };
        let relation = match raw.relation.trim().to_ascii_lowercase().as_str() {
            "drives" => RelationKind::Drives,
            "reads" => RelationKind::Reads,
            _ => continue,
        };
        // Bounded: the signal must be one the document itself declares.
        if actor.is_empty() {
            continue;
        }
        let key = (
            actor.clone(),
            signal.clone(),
            matches!(relation, RelationKind::Drives),
        );
        if !seen.insert(key) {
            continue;
        }
        let id = out.len() + 1;
        out.push(ActorSignalRelation {
            relation_id: format!("nlp_rel_{statement_id}_{id}"),
            actor_name: actor,
            signal_name: signal,
            relation,
            source_statement_ids: vec![statement_id.to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
    }
    out
}

/// Extract relations from one sentence via the text provider, bounded by `known_signals`.
/// Fail-open: a provider error yields no relations (the deterministic extractor still runs).
/// `statement_id`/`sentence` double as the `SPECFORGE_VLM_HELPER` mock keys, so the whole
/// path is hermetically testable.
pub fn nlp_extract_relations(
    provider: VlmProviderArg,
    model: &str,
    statement_id: &str,
    sentence: &str,
    known_signals: &HashSet<String>,
) -> Vec<ActorSignalRelation> {
    let known: Vec<String> = known_signals.iter().cloned().collect();
    let prompt = relation_extract_prompt(sentence, &known);
    match call_text_provider(
        provider,
        model,
        api_url(provider),
        statement_id,
        sentence,
        &prompt,
        256,
    ) {
        Ok(resp) => parse_nlp_relations(&resp, statement_id, known_signals),
        Err(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signals(names: &[&str]) -> HashSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn parse_keeps_only_declared_signals_and_valid_directions() {
        let known = signals(&["PSEL", "PREADY"]);
        // Two valid; one names an UNDECLARED signal (dropped); one bad direction (dropped).
        let resp = r#"junk before [
            {"actor":"Requester","relation":"drives","signal":"PSEL"},
            {"actor":"Completer","relation":"reads","signal":"PREADY"},
            {"actor":"Requester","relation":"drives","signal":"HXYZZY"},
            {"actor":"Requester","relation":"floats","signal":"PSEL"}
        ] junk after"#;
        let rels = parse_nlp_relations(resp, "s1", &known);
        assert_eq!(
            rels.len(),
            2,
            "only declared-signal, valid-direction proposals survive: {rels:?}"
        );
        assert!(rels.iter().any(|r| r.actor_name == "Requester"
            && matches!(r.relation, RelationKind::Drives)
            && r.signal_name == "PSEL"));
        assert!(rels.iter().any(|r| r.actor_name == "Completer"
            && matches!(r.relation, RelationKind::Reads)
            && r.signal_name == "PREADY"));
        assert!(
            !rels.iter().any(|r| r.signal_name == "HXYZZY"),
            "a signal the document never declares is dropped (ADR 0006 / bounded doctrine)"
        );
    }

    #[test]
    fn parse_dedups_exact_identity_and_preserves_case_distinct_actors() {
        let known = signals(&["PSEL"]);
        let resp = r#"[
            {"actor":"Requester","relation":"drives","signal":"psel"},
            {"actor":"Requester","relation":"drives","signal":"PSEL"},
            {"actor":"REQUESTER","relation":"drives","signal":"PSEL"}
        ]"#;
        let rels = parse_nlp_relations(resp, "s2", &known);
        assert_eq!(
            rels.len(),
            2,
            "only the exact duplicate collapses: {rels:?}"
        );
        assert!(rels.iter().all(|relation| relation.signal_name == "PSEL"));
        assert!(
            rels.iter()
                .any(|relation| relation.actor_name == "Requester")
        );
        assert!(
            rels.iter()
                .any(|relation| relation.actor_name == "REQUESTER")
        );
        // No JSON array at all → empty, never panics.
        assert!(parse_nlp_relations("the model said no", "s3", &known).is_empty());
    }

    #[test]
    fn parse_preserves_document_spelling_and_rejects_ambiguous_case_folding() {
        let known = signals(&["mixedCase"]);
        let response = r#"[{"actor":"orchid","relation":"drives","signal":"MIXEDCASE"}]"#;
        let relations = parse_nlp_relations(response, "s4", &known);
        assert_eq!(relations.len(), 1);
        assert_eq!(relations[0].signal_name, "mixedCase");

        let ambiguous = signals(&["sig", "SIG"]);
        let response = r#"[{"actor":"orchid","relation":"drives","signal":"SiG"}]"#;
        assert!(parse_nlp_relations(response, "s5", &ambiguous).is_empty());
    }

    #[test]
    fn prompt_lists_known_signals_and_the_sentence() {
        let p = relation_extract_prompt(
            "The Requester drives PSEL.",
            &["PREADY".into(), "PSEL".into()],
        );
        assert!(p.contains("PREADY") && p.contains("PSEL"));
        assert!(p.contains("The Requester drives PSEL."));
        assert!(p.contains("drives") && p.contains("reads"));
    }
}
