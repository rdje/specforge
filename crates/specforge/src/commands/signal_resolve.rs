//! `signal-resolve` — Tier-3 LLM actor→signal relation extraction
//! (`R14-SIGNAL-RESOLVE.2`).
//!
//! Parallel to `extract-contracts`: for each `NormativeStatement` prose
//! sentence the Tier-1 (table) and Tier-2 (verb-pattern) extractors did not
//! resolve, ask the provider (Qwen via Ollama by default) for a JSON array of
//! `{actor, signal, relation:"drives"|"reads"}` objects — **every** relation the
//! sentence states (PURE-NLP-INTENT-EXTRACTION.2; a sentence with two edges
//! yields two). Grounded, deduped survivors are appended to
//! `EvidenceIR.actor_signal_relations` —
//! the existing KG field `SemanticIr::build` already consumes (→ actor ports,
//! graph-first direction). No new IR types; no `SemanticIr` wiring.
//!
//! Anti-fabrication: a `none` / unparseable / ungrounded answer yields NO
//! edge; the model can never inject an actor or signal the sentence (and the
//! grounding list, when supplied) does not support.
//!
//! The OpenAI-compatible text transport (curl + `SPECFORGE_VLM_HELPER` hook +
//! request/response shape) lives in the shared `crate::commands::llm_text`
//! helper; this module keeps only the relation-specific prompt + classifier.

use crate::cli::{SignalResolveArgs, VlmProviderArg};
use crate::commands::llm_text;
use crate::error::{AppError, Result};
use crate::ir::entity_typing::declared_signal_catalog;
use crate::ir::evidence::{
    EvidenceIr, ExtractorTier, FactKind, FactProvenanceRecord, StatementClass,
    actor_signal_relation_fact_key,
};
use crate::ir::source::{ActorSignalRelation, AutomationConfidence, RelationKind};

/// Minimum word count for a prose statement to be a relation candidate.
const MIN_CANDIDATE_WORDS: usize = 5;

fn strip_code_fences(raw: &str) -> String {
    let t = raw.trim();
    let t = t
        .strip_prefix("```json")
        .or_else(|| t.strip_prefix("```"))
        .unwrap_or(t)
        .trim_start();
    t.strip_suffix("```").unwrap_or(t).trim().to_string()
}

/// A signal identity is an opaque ASCII identifier. Whether that identifier denotes a
/// signal comes from the current document's grounding catalog, never from letter case.
fn is_signal_identifier(s: &str) -> bool {
    let mut characters = s.chars();
    let Some(first) = characters.next() else {
        return false;
    };
    (first.is_ascii_alphabetic() || first == '_')
        && characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn parse_relation_kind(s: &str) -> Option<RelationKind> {
    match s.trim().to_ascii_lowercase().as_str() {
        "drives" => Some(RelationKind::Drives),
        "reads" => Some(RelationKind::Reads),
        _ => None,
    }
}

/// Build a grounded `ActorSignalRelation` from one parsed JSON object — the shared gate
/// logic (anti-fabrication: actor non-empty, signal an identifier and, when a declared
/// signal list is given, a member of it). `relation_id` is the caller's namespaced id.
fn relation_from_value(
    value: &serde_json::Value,
    relation_id: &str,
    statement_id: &str,
    grounding: &[String],
) -> Option<ActorSignalRelation> {
    if value.get("type").and_then(|t| t.as_str()) == Some("none") {
        return None;
    }
    let actor = value
        .get("actor")
        .and_then(|a| a.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let signal = value
        .get("signal")
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let relation = parse_relation_kind(value.get("relation").and_then(|r| r.as_str())?)?;
    // Grounding gates (anti-fabrication).
    if actor.is_empty() || !is_signal_identifier(&signal) {
        return None;
    }
    if grounding.is_empty() || !grounding.iter().any(|grounded| grounded == &signal) {
        return None; // no current-document declaration authority
    }
    Some(ActorSignalRelation {
        relation_id: relation_id.to_string(),
        actor_name: actor,
        signal_name: signal,
        relation,
        source_statement_ids: vec![statement_id.to_string()],
        automation_confidence: AutomationConfidence::Medium,
    })
}

/// Parse **every** grounded relation from one provider response — a JSON array (a single
/// object is tolerated for robustness). Multi-relation recall: a sentence stating two edges
/// yields two, where the single-object path lost one (PURE-NLP-INTENT-EXTRACTION.2).
/// Duplicates within the one response are collapsed; ids are `r14:<statement_id>:<n>`.
fn classify_relation_responses(
    raw: &str,
    statement_id: &str,
    grounding: &[String],
) -> Vec<ActorSignalRelation> {
    let cleaned = strip_code_fences(raw);
    let value: serde_json::Value = match serde_json::from_str(&cleaned) {
        Ok(v) => v,
        Err(_) => return Vec::new(),
    };
    let items: Vec<&serde_json::Value> = match &value {
        serde_json::Value::Array(arr) => arr.iter().collect(),
        other => vec![other],
    };
    let mut out = Vec::new();
    let mut seen: std::collections::HashSet<(String, String, bool)> =
        std::collections::HashSet::new();
    for (idx, item) in items.iter().enumerate() {
        let id = format!("r14:{statement_id}:{}", idx + 1);
        if let Some(rel) = relation_from_value(item, &id, statement_id, grounding) {
            let key = (
                rel.actor_name.clone(),
                rel.signal_name.clone(),
                matches!(rel.relation, RelationKind::Drives),
            );
            if seen.insert(key) {
                out.push(rel);
            }
        }
    }
    out
}

fn build_relation_prompt(sentence: &str, grounding: &[String]) -> String {
    let grounding_section = if grounding.is_empty() {
        String::new()
    } else {
        format!(
            "Declared signal symbols in the current document: {}\n\n",
            grounding.join(", ")
        )
    };
    format!(
        "You extract typed actor-to-signal relations from one digital-hardware specification sentence.\n\
         Treat every document-owned actor and signal symbol as opaque.\n\
         From ONE sentence, extract EVERY actor->signal relation present (a sentence may \
         state more than one).\n\n\
         {grounding_section}\
         Sentence: \"{sentence}\"\n\n\
         Respond with ONLY a JSON array (no other text), one object per relation:\n\
           [{{\"actor\":\"<exact actor phrase from sentence>\",\"signal\":\"<exact declared signal>\",\"relation\":\"drives|reads\"}}]\n\
         - Include an object for each actor that drives/asserts/outputs a named signal, OR that \
         reads/samples/monitors one.\n\
         - If no actor->signal relation is extractable, return [].\n\n\
         Rules:\n\
         - signal must exactly match one symbol in the declared-signal list\n\
         - relation must be exactly \"drives\" or \"reads\"\n\
         - never invent a signal or actor not present in the sentence\n\
         - Only output the JSON array, nothing else"
    )
}

fn candidate_work(ir: &EvidenceIr, max_statements: usize) -> Vec<(String, String)> {
    let mut work: Vec<(String, String)> = ir
        .extracted_statements
        .iter()
        .filter(|s| matches!(s.class, StatementClass::NormativeStatement))
        .filter(|s| s.text.split_whitespace().count() >= MIN_CANDIDATE_WORDS)
        .map(|s| (s.statement_id.clone(), s.text.clone()))
        .collect();
    if max_statements > 0 && work.len() > max_statements {
        work.truncate(max_statements);
    }
    work
}

fn grounding_signals(arg: &Option<String>, ir: &EvidenceIr) -> Vec<String> {
    let mut declared = declared_signal_catalog(ir);

    match arg {
        Some(explicit) => {
            let requested = explicit
                .split(',')
                .map(str::trim)
                .filter(|signal| !signal.is_empty())
                .collect::<std::collections::BTreeSet<_>>();
            declared.retain(|signal| requested.contains(signal.as_str()));
            declared
        }
        None => declared,
    }
}

/// True when an equal (actor, signal, relation) edge already exists — Tier-3
/// must never double-count a Tier-1/2 edge.
fn is_duplicate(ir: &EvidenceIr, rel: &ActorSignalRelation) -> bool {
    ir.actor_signal_relations.iter().any(|r| {
        r.actor_name == rel.actor_name
            && r.signal_name == rel.signal_name
            && r.relation == rel.relation
    })
}

fn is_same_edge(a: &ActorSignalRelation, b: &ActorSignalRelation) -> bool {
    a.actor_name == b.actor_name && a.signal_name == b.signal_name && a.relation == b.relation
}

/// Extract actor→signal relations from prose and append them to the EvidenceIR.
pub fn run(args: SignalResolveArgs) -> Result<()> {
    let evidence_ir_path = if args.evidence_ir.exists() {
        args.evidence_ir.clone()
    } else {
        return Err(AppError::MissingPath(args.evidence_ir));
    };
    let mut ir = EvidenceIr::load_from_path(&evidence_ir_path)?;

    println!("command: signal-resolve");
    println!("mode: {}", if args.dry_run { "dry-run" } else { "execute" });
    println!("evidence_ir_path: {}", evidence_ir_path.display());
    println!("document_key: {}", ir.document_identity.document_key);
    println!(
        "existing_actor_signal_relations: {}",
        ir.actor_signal_relations.len()
    );

    let work = candidate_work(&ir, args.max_statements);
    println!("candidate_statements: {}", work.len());

    if matches!(args.provider, VlmProviderArg::Skip) {
        println!("llm_provider: skip");
        println!("resolution: skipped");
        println!(
            "hint: re-run with --provider ollama --model qwen2.5vl:7b to resolve {} candidate(s)",
            work.len()
        );
        return Ok(());
    }

    let model = args
        .model
        .clone()
        .unwrap_or_else(|| llm_text::default_model(args.provider));
    let url = llm_text::api_url(args.provider);
    let grounding = grounding_signals(&args.grounding_signals, &ir);
    println!("llm_provider: {}", llm_text::provider_name(args.provider));
    println!("llm_model: {model}");

    if args.dry_run {
        println!("resolution: dry-run (no LLM calls)");
        return Ok(());
    }

    let mut resolved: Vec<ActorSignalRelation> = Vec::new();
    let mut skipped = 0usize;
    let mut deduped = 0usize;
    for (statement_id, sentence) in &work {
        let prompt = build_relation_prompt(sentence, &grounding);
        let raw = llm_text::call_text_provider(
            args.provider,
            &model,
            url,
            statement_id,
            sentence,
            &prompt,
            512,
        )?;
        let rels = classify_relation_responses(&raw, statement_id, &grounding);
        if rels.is_empty() {
            skipped += 1;
        }
        for rel in rels {
            // PER-EXTRACTOR-FACT-TAGGING: tag each LLM (Nlp-tier) relation find here,
            // pre-dedup, so an overlap with a Pattern relation is recorded (the index is
            // deduped by the triple).
            let prov = FactProvenanceRecord {
                producer: ExtractorTier::Nlp,
                fact_kind: FactKind::ActorSignalRelation,
                canonical_key: actor_signal_relation_fact_key(&rel),
            };
            if !ir.fact_provenance.contains(&prov) {
                ir.fact_provenance.push(prov);
            }
            if is_duplicate(&ir, &rel) || resolved.iter().any(|r| is_same_edge(r, &rel)) {
                deduped += 1;
            } else {
                resolved.push(rel);
            }
        }
    }
    println!("resolved_relations: {}", resolved.len());
    println!("skipped: {skipped}");
    println!("deduped_against_existing: {deduped}");

    ir.actor_signal_relations.extend(resolved);
    ir.write_to_disk()?;
    println!("wrote: {}", evidence_ir_path.display());
    println!("next: re-run `specforge semantic` to fold these into the actor-relative graph");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relation_prompt_is_alpha_equivariant_and_uses_no_named_example() {
        let first = build_relation_prompt("orchid drives copper.", &["copper".to_string()])
            .replace("orchid", "<actor>")
            .replace("copper", "<signal>");
        let renamed = build_relation_prompt("juniper drives silver.", &["silver".to_string()])
            .replace("juniper", "<actor>")
            .replace("silver", "<signal>");
        assert_eq!(first, renamed);
        assert!(!first.contains("protocol specification analyzer"));
        assert!(!first.contains("AWVALID"));
        assert!(!first.contains("HTRANS"));
    }

    #[test]
    fn none_and_malformed_are_skipped() {
        assert!(classify_relation_responses(r#"{"type":"none"}"#, "s1", &[]).is_empty());
        assert!(classify_relation_responses("not json", "s1", &[]).is_empty());
    }

    #[test]
    fn valid_drives_relation_is_accepted_with_provenance() {
        let raw = r#"{"actor":"Manager","signal":"AWVALID","relation":"drives"}"#;
        let rels = classify_relation_responses(raw, "s7", &["AWVALID".to_string()]);
        assert_eq!(rels.len(), 1, "got {rels:?}");
        let r = &rels[0];
        assert_eq!(r.actor_name, "Manager");
        assert_eq!(r.signal_name, "AWVALID");
        assert_eq!(r.relation, RelationKind::Drives);
        assert_eq!(r.relation_id, "r14:s7:1");
        assert_eq!(r.source_statement_ids, vec!["s7".to_string()]);
    }

    #[test]
    fn valid_reads_relation_is_accepted() {
        let raw = r#"{"actor":"Subordinate","signal":"WDATA","relation":"reads"}"#;
        let rels = classify_relation_responses(raw, "s8", &["WDATA".to_string()]);
        assert_eq!(rels.len(), 1, "got {rels:?}");
        assert_eq!(rels[0].relation, RelationKind::Reads);
        assert_eq!(rels[0].signal_name, "WDATA");
    }

    #[test]
    fn identifier_syntax_and_relation_kind_are_enforced_without_case_authority() {
        let grounding = vec!["address".to_string()];
        assert!(
            !classify_relation_responses(
                r#"{"actor":"Manager","signal":"address","relation":"drives"}"#,
                "s1",
                &grounding
            )
            .is_empty()
        );
        assert!(
            classify_relation_responses(
                r#"{"actor":"Manager","signal":"bad-name","relation":"drives"}"#,
                "s1",
                &[]
            )
            .is_empty()
        );
        // relation not in {drives,reads}
        assert!(
            classify_relation_responses(
                r#"{"actor":"Manager","signal":"AWVALID","relation":"toggles"}"#,
                "s1",
                &[]
            )
            .is_empty()
        );
        // empty actor
        assert!(
            classify_relation_responses(
                r#"{"actor":"","signal":"AWVALID","relation":"drives"}"#,
                "s1",
                &[]
            )
            .is_empty()
        );
    }

    #[test]
    fn ungrounded_signal_is_skipped_when_grounding_supplied() {
        let grounding = vec!["AWVALID".to_string(), "AWREADY".to_string()];
        // signal not in the declared list → skipped (anti-fabrication)
        assert!(
            classify_relation_responses(
                r#"{"actor":"Manager","signal":"BOGUS","relation":"drives"}"#,
                "s1",
                &grounding
            )
            .is_empty()
        );
        // signal IN the declared list → accepted
        assert!(
            !classify_relation_responses(
                r#"{"actor":"Manager","signal":"AWVALID","relation":"drives"}"#,
                "s1",
                &grounding
            )
            .is_empty()
        );
    }

    #[test]
    fn empty_declaration_catalog_never_promotes_a_model_relation() {
        let raw = r#"{"actor":"Manager","signal":"plausibleName","relation":"drives"}"#;
        assert!(classify_relation_responses(raw, "s1", &[]).is_empty());
    }

    #[test]
    fn multiple_relations_in_one_response_are_all_extracted() {
        // PURE-NLP-INTENT-EXTRACTION.2: a sentence stating two edges yields two; an
        // ungrounded signal is still dropped (anti-fabrication).
        let raw = r#"[
            {"actor":"Manager","signal":"PADDR","relation":"drives"},
            {"actor":"Manager","signal":"PREADY","relation":"reads"},
            {"actor":"Manager","signal":"HXYZZY","relation":"drives"}
        ]"#;
        let grounding = vec!["PADDR".to_string(), "PREADY".to_string()];
        let rels = classify_relation_responses(raw, "s5", &grounding);
        assert_eq!(
            rels.len(),
            2,
            "two grounded edges; ungrounded HXYZZY dropped: {rels:?}"
        );
        assert!(
            rels.iter()
                .any(|r| r.signal_name == "PADDR" && matches!(r.relation, RelationKind::Drives))
        );
        assert!(
            rels.iter()
                .any(|r| r.signal_name == "PREADY" && matches!(r.relation, RelationKind::Reads))
        );
        assert!(rels.iter().all(|r| r.relation_id.starts_with("r14:s5:")));
    }

    #[test]
    fn multi_parser_tolerates_single_object_and_collapses_dupes() {
        let one = classify_relation_responses(
            r#"{"actor":"Requester","signal":"PSEL","relation":"drives"}"#,
            "s1",
            &["PSEL".to_string()],
        );
        assert_eq!(one.len(), 1);
        assert_eq!(one[0].signal_name, "PSEL");
        let dup = classify_relation_responses(
            r#"[{"actor":"M","signal":"PSEL","relation":"drives"},{"actor":"M","signal":"PSEL","relation":"drives"}]"#,
            "s2",
            &["PSEL".to_string()],
        );
        assert_eq!(dup.len(), 1, "intra-response dedup: {dup:?}");
    }
}
