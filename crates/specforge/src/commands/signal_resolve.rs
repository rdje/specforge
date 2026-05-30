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
//! NOTE: `build_text_chat_request` / `extract_chat_content` mirror
//! `nlp_enrich` / `extract_contracts` (kept private there). A shared text
//! transport is a tracked DRY follow-up; duplicating keeps this leaf bounded
//! and zero-risk to the in-use `nlp_enrich`.

use std::fs;
use std::process::Command;

use crate::cli::{SignalResolveArgs, VlmProviderArg};
use crate::error::{AppError, Result};
use crate::ir::evidence::{EvidenceIr, StatementClass};
use crate::ir::source::{ActorSignalRelation, AutomationConfidence, RelationKind};

/// Test override hook (same var as `enrich` / `nlp-enrich` / `extract-contracts`).
const VLM_HELPER_ENV: &str = "SPECFORGE_VLM_HELPER";

/// Minimum word count for a prose statement to be a relation candidate.
const MIN_CANDIDATE_WORDS: usize = 5;

fn provider_name(provider: VlmProviderArg) -> &'static str {
    match provider {
        VlmProviderArg::Ollama => "ollama",
        VlmProviderArg::OpenAi => "openai",
        VlmProviderArg::LmStudio => "lmstudio",
        VlmProviderArg::Skip => "skip",
    }
}

fn default_model(provider: VlmProviderArg) -> String {
    match provider {
        VlmProviderArg::Ollama | VlmProviderArg::LmStudio => "qwen2.5vl:7b".to_string(),
        VlmProviderArg::OpenAi => "gpt-4o".to_string(),
        VlmProviderArg::Skip => String::new(),
    }
}

fn api_url(provider: VlmProviderArg) -> &'static str {
    match provider {
        VlmProviderArg::Ollama => "http://localhost:11434/v1/chat/completions",
        VlmProviderArg::OpenAi => "https://api.openai.com/v1/chat/completions",
        VlmProviderArg::LmStudio => "http://localhost:1234/v1/chat/completions",
        VlmProviderArg::Skip => "",
    }
}

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

fn build_text_chat_request(model: &str, prompt: &str) -> String {
    let prompt_escaped = prompt
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    format!(
        r#"{{"model": "{model}", "messages": [{{"role": "user", "content": "{prompt_escaped}"}}], "max_tokens": 256, "temperature": 0}}"#
    )
}

fn extract_chat_content(response_json: &str) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct ChatResponse {
        choices: Vec<ChatChoice>,
    }
    #[derive(serde::Deserialize)]
    struct ChatChoice {
        message: ChatMessage,
    }
    #[derive(serde::Deserialize)]
    struct ChatMessage {
        content: serde_json::Value,
    }
    let response: ChatResponse = serde_json::from_str(response_json).map_err(|e| {
        AppError::InvalidStageArtifact(format!(
            "invalid LLM response: {e}\nraw: {}",
            &response_json[..response_json.len().min(256)]
        ))
    })?;
    let content = response
        .choices
        .into_iter()
        .next()
        .ok_or_else(|| AppError::InvalidStageArtifact("LLM response has no choices".to_string()))?
        .message
        .content;
    match content {
        serde_json::Value::String(s) => Ok(s),
        serde_json::Value::Array(parts) => {
            for part in &parts {
                if let Some(text) = part.get("text").and_then(|t| t.as_str()) {
                    return Ok(text.to_string());
                }
            }
            Err(AppError::InvalidStageArtifact(
                "LLM response content array has no text part".to_string(),
            ))
        }
        other => Ok(other.to_string()),
    }
}

fn call_provider_for_relation(
    sentence: &str,
    statement_id: &str,
    model: &str,
    api_url: &str,
    provider: VlmProviderArg,
    grounding: &[String],
) -> Result<String> {
    if let Some(helper_path) = std::env::var_os(VLM_HELPER_ENV) {
        let output = Command::new(&helper_path)
            .arg("--statement-id")
            .arg(statement_id)
            .arg("--sentence")
            .arg(sentence)
            .output()?;
        if output.status.success() {
            return Ok(String::from_utf8_lossy(&output.stdout).trim().to_string());
        }
        return Err(AppError::ExternalCommandFailed {
            program: helper_path.display().to_string(),
            exit_code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }
    let prompt = build_relation_prompt(sentence, grounding);
    let request_body = build_text_chat_request(model, &prompt);
    let mut cmd = Command::new("curl");
    cmd.arg("-s")
        .arg("-X")
        .arg("POST")
        .arg(api_url)
        .arg("-H")
        .arg("Content-Type: application/json");
    if matches!(provider, VlmProviderArg::OpenAi) {
        let api_key =
            std::env::var("OPENAI_API_KEY").map_err(|_| AppError::MissingRuntimeDependency {
                dependency: "OPENAI_API_KEY",
                resolution: "Set OPENAI_API_KEY environment variable".to_string(),
            })?;
        cmd.arg("-H")
            .arg(format!("Authorization: Bearer {api_key}"));
    }
    let tempdir = tempfile::tempdir()?;
    let request_path = tempdir.path().join("relation_request.json");
    fs::write(&request_path, &request_body)?;
    cmd.arg("-d").arg(format!("@{}", request_path.display()));
    let output = cmd.output()?;
    if !output.status.success() {
        return Err(AppError::ExternalCommandFailed {
            program: "curl".to_string(),
            exit_code: output.status.code(),
            stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        });
    }
    extract_chat_content(&String::from_utf8_lossy(&output.stdout))
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
        .unwrap_or_else(|| default_model(args.provider));
    let url = api_url(args.provider);
    let grounding = grounding_signals(&args.grounding_signals);
    println!("llm_provider: {}", provider_name(args.provider));
    println!("llm_model: {model}");

    if args.dry_run {
        println!("resolution: dry-run (no LLM calls)");
        return Ok(());
    }

    let mut resolved: Vec<ActorSignalRelation> = Vec::new();
    let mut skipped = 0usize;
    let mut deduped = 0usize;
    for (statement_id, sentence) in &work {
        let raw = call_provider_for_relation(
            sentence,
            statement_id,
            &model,
            url,
            args.provider,
            &grounding,
        )?;
        match classify_relation_response(&raw, statement_id, &grounding) {
            RelationOutcome::Skipped => skipped += 1,
            RelationOutcome::Accepted(rel) => {
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

fn is_same_edge(a: &ActorSignalRelation, b: &ActorSignalRelation) -> bool {
    a.actor_name == b.actor_name && a.signal_name == b.signal_name && a.relation == b.relation
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
