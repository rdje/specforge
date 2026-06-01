//! `signal-resolve` — Tier-3 LLM actor→signal relation extraction
//! (`R14-SIGNAL-RESOLVE.2`).
//!
//! Parallel to `extract-contracts`: for each `NormativeStatement` prose
//! sentence the Tier-1 (table) and Tier-2 (verb-pattern) extractors did not
//! resolve, ask the provider (Qwen via Ollama by default) for one
//! `{actor, signal, relation:"drives"|"reads"}` JSON object. Grounded,
//! deduped survivors are appended to `EvidenceIR.actor_signal_relations` —
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
use crate::ir::evidence::{
    EvidenceIr, ExtractorTier, FactKind, FactProvenanceRecord, StatementClass,
    actor_signal_relation_fact_key,
};
use crate::ir::source::{ActorSignalRelation, AutomationConfidence, RelationKind};

/// Minimum word count for a prose statement to be a relation candidate.
const MIN_CANDIDATE_WORDS: usize = 5;

/// The outcome of classifying one provider response for one prose statement.
#[derive(Debug)]
enum RelationOutcome {
    /// `none` / unparseable / failed a grounding gate — no edge.
    Skipped,
    /// A grounded actor→signal edge.
    Accepted(Box<ActorSignalRelation>),
}

fn strip_code_fences(raw: &str) -> String {
    let t = raw.trim();
    let t = t
        .strip_prefix("```json")
        .or_else(|| t.strip_prefix("```"))
        .unwrap_or(t)
        .trim_start();
    t.strip_suffix("```").unwrap_or(t).trim().to_string()
}

/// A signal name is acceptable iff non-empty and uppercase-alphanumeric + `_`
/// (the `nlp_enrich` signal-name discipline — keeps prose words out of the KG).
fn is_signal_name(s: &str) -> bool {
    !s.is_empty()
        && s.chars()
            .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
}

fn parse_relation_kind(s: &str) -> Option<RelationKind> {
    match s.trim().to_ascii_lowercase().as_str() {
        "drives" => Some(RelationKind::Drives),
        "reads" => Some(RelationKind::Reads),
        _ => None,
    }
}

/// Pure decision for one provider response (no I/O) — the testable core.
/// Applies the grounding gates and builds a provenance-honest
/// `ActorSignalRelation` (`relation_id` namespaced `r14:<statement_id>`,
/// `source_statement_ids` = the real statement). `grounding` is the declared
/// signal list; when non-empty the extracted signal must be a member.
fn classify_relation_response(
    raw: &str,
    statement_id: &str,
    grounding: &[String],
) -> RelationOutcome {
    let cleaned = strip_code_fences(raw);
    let value: serde_json::Value = match serde_json::from_str(&cleaned) {
        Ok(v) => v,
        Err(_) => return RelationOutcome::Skipped,
    };
    if value.get("type").and_then(|t| t.as_str()) == Some("none") {
        return RelationOutcome::Skipped;
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
    let relation = match value.get("relation").and_then(|r| r.as_str()) {
        Some(r) => match parse_relation_kind(r) {
            Some(k) => k,
            None => return RelationOutcome::Skipped,
        },
        None => return RelationOutcome::Skipped,
    };
    // Grounding gates (anti-fabrication).
    if actor.is_empty() || !is_signal_name(&signal) {
        return RelationOutcome::Skipped;
    }
    if !grounding.is_empty() && !grounding.iter().any(|g| g == &signal) {
        return RelationOutcome::Skipped; // ungrounded signal
    }
    RelationOutcome::Accepted(Box::new(ActorSignalRelation {
        relation_id: format!("r14:{statement_id}"),
        actor_name: actor,
        signal_name: signal,
        relation,
        source_statement_ids: vec![statement_id.to_string()],
        automation_confidence: AutomationConfidence::Medium,
    }))
}

fn build_relation_prompt(sentence: &str, grounding: &[String]) -> String {
    let grounding_section = if grounding.is_empty() {
        String::new()
    } else {
        format!(
            "Known hardware signals in this specification: {}\n\n",
            grounding.join(", ")
        )
    };
    format!(
        "You are a hardware protocol specification analyzer.\n\
         From ONE sentence, extract a single actor->signal relation if present.\n\n\
         {grounding_section}\
         Sentence: \"{sentence}\"\n\n\
         Respond with exactly one JSON object (no other text):\n\
         - If an actor drives/asserts/outputs a named signal, OR reads/samples/monitors one:\n\
           {{\"actor\":\"ACTOR_NAME\",\"signal\":\"SIGNAL_NAME\",\"relation\":\"drives|reads\"}}\n\
         - If no actor->signal relation is extractable:\n\
           {{\"type\":\"none\"}}\n\n\
         Rules:\n\
         - signal must be an uppercase hardware signal name (e.g. AWVALID, HTRANS)\n\
         - relation must be exactly \"drives\" or \"reads\"\n\
         - never invent a signal or actor not present in the sentence\n\
         - Only output the JSON object, nothing else"
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

fn grounding_signals(arg: &Option<String>) -> Vec<String> {
    match arg {
        Some(explicit) if explicit.is_empty() => Vec::new(),
        Some(explicit) => explicit
            .split(',')
            .map(|s| s.trim().to_ascii_uppercase())
            .filter(|s| !s.is_empty())
            .collect(),
        None => Vec::new(),
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
    let grounding = grounding_signals(&args.grounding_signals);
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
        match classify_relation_response(&raw, statement_id, &grounding) {
            RelationOutcome::Skipped => skipped += 1,
            RelationOutcome::Accepted(rel) => {
                // PER-EXTRACTOR-FACT-TAGGING: tag this LLM (Nlp-tier) relation find
                // here, pre-dedup, so an overlap with a Pattern relation is recorded
                // (the index is deduped by the triple).
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
                    resolved.push(*rel);
                }
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
    fn none_and_malformed_are_skipped() {
        assert!(matches!(
            classify_relation_response(r#"{"type":"none"}"#, "s1", &[]),
            RelationOutcome::Skipped
        ));
        assert!(matches!(
            classify_relation_response("not json", "s1", &[]),
            RelationOutcome::Skipped
        ));
    }

    #[test]
    fn valid_drives_relation_is_accepted_with_provenance() {
        let raw = r#"{"actor":"Manager","signal":"AWVALID","relation":"drives"}"#;
        match classify_relation_response(raw, "s7", &[]) {
            RelationOutcome::Accepted(r) => {
                assert_eq!(r.actor_name, "Manager");
                assert_eq!(r.signal_name, "AWVALID");
                assert_eq!(r.relation, RelationKind::Drives);
                assert_eq!(r.relation_id, "r14:s7");
                assert_eq!(r.source_statement_ids, vec!["s7".to_string()]);
            }
            other => panic!("expected Accepted Drives, got {other:?}"),
        }
    }

    #[test]
    fn valid_reads_relation_is_accepted() {
        let raw = r#"{"actor":"Subordinate","signal":"WDATA","relation":"reads"}"#;
        match classify_relation_response(raw, "s8", &[]) {
            RelationOutcome::Accepted(r) => {
                assert_eq!(r.relation, RelationKind::Reads);
                assert_eq!(r.signal_name, "WDATA");
            }
            other => panic!("expected Accepted Reads, got {other:?}"),
        }
    }

    #[test]
    fn non_uppercase_signal_or_bad_relation_is_skipped() {
        // lowercase / prose-word signal rejected
        assert!(matches!(
            classify_relation_response(
                r#"{"actor":"Manager","signal":"address","relation":"drives"}"#,
                "s1",
                &[]
            ),
            RelationOutcome::Skipped
        ));
        // relation not in {drives,reads}
        assert!(matches!(
            classify_relation_response(
                r#"{"actor":"Manager","signal":"AWVALID","relation":"toggles"}"#,
                "s1",
                &[]
            ),
            RelationOutcome::Skipped
        ));
        // empty actor
        assert!(matches!(
            classify_relation_response(
                r#"{"actor":"","signal":"AWVALID","relation":"drives"}"#,
                "s1",
                &[]
            ),
            RelationOutcome::Skipped
        ));
    }

    #[test]
    fn ungrounded_signal_is_skipped_when_grounding_supplied() {
        let grounding = vec!["AWVALID".to_string(), "AWREADY".to_string()];
        // signal not in the declared list → skipped (anti-fabrication)
        assert!(matches!(
            classify_relation_response(
                r#"{"actor":"Manager","signal":"BOGUS","relation":"drives"}"#,
                "s1",
                &grounding
            ),
            RelationOutcome::Skipped
        ));
        // signal IN the declared list → accepted
        assert!(matches!(
            classify_relation_response(
                r#"{"actor":"Manager","signal":"AWVALID","relation":"drives"}"#,
                "s1",
                &grounding
            ),
            RelationOutcome::Accepted(_)
        ));
    }
}
