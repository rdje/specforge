//! `extract-contracts` — the live prose→`ActorContract` producer
//! (`CVE-PROSE-EXTRACTION.3`).
//!
//! Parallel to `nlp_enrich`: for each `NormativeStatement` prose sentence in an
//! `EvidenceIR`, ask the provider (Qwen via Ollama by default) for a single
//! `ActorContract`-shaped JSON object, then route the response through the
//! fails-closed `parse_constrained_contract` + `apply_entailment_to_contract`
//! honesty machinery (`crate::ir::cve`). Survivors land on
//! `EvidenceIR.extracted_contracts`; `SemanticIr::build` later folds them into
//! `actor_contracts` before fusion/fidelity.
//!
//! Safety is structural (the program thesis): a `none` answer is skipped,
//! malformed JSON is a `schema_reject` (NO contract), and an unverifiable
//! contract is rerouted to `Residual` — the producer can never fabricate a
//! `Lowerable` contract the prose does not license.
//!
//! NOTE: `build_text_chat_request` / `extract_chat_content` are mirrored from
//! `nlp_enrich` (which keeps them private). DRY-ing the shared text transport
//! into one helper is a tracked follow-up; duplicating here keeps this leaf
//! bounded and zero-risk to the in-use `nlp_enrich` command.

use std::fs;
use std::process::Command;

use crate::cli::{ExtractContractsArgs, VlmProviderArg};
use crate::error::{AppError, Result};
use crate::ir::contract::ActorContract;
use crate::ir::cve::{
    ConstrainedExtractionStats, actor_contract_json_schema_summary, apply_entailment_to_contract,
    parse_constrained_contract,
};
use crate::ir::evidence::{EvidenceIr, StatementClass};

/// Environment variable overriding the LLM helper script (for unit testing).
/// Same variable as `enrich` / `nlp-enrich` so one mock can cover all three.
const VLM_HELPER_ENV: &str = "SPECFORGE_VLM_HELPER";

/// Minimum word count for a prose statement to be a contract candidate
/// (mirrors `nlp_enrich`'s candidate floor — very short fragments carry no
/// extractable timed obligation).
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
enum CandidateOutcome {
    /// The model emitted the `none` sentinel — no contract licensed.
    Skipped,
    /// The model claimed a contract but emitted invalid JSON — fails closed.
    SchemaReject,
    /// A valid contract (already entailment-gated: a `Fail` is now `Residual`).
    Accepted(Box<ActorContract>),
}

/// Strip a single Markdown code fence (```json … ``` or ``` … ```) if present.
fn strip_code_fences(raw: &str) -> String {
    let t = raw.trim();
    let t = t
        .strip_prefix("```json")
        .or_else(|| t.strip_prefix("```"))
        .unwrap_or(t)
        .trim_start();
    t.strip_suffix("```").unwrap_or(t).trim().to_string()
}

/// True when the response is the "no contract" sentinel: empty, a bare `none`,
/// or a `{"kind":"none"}`-style object. Distinguishes "no contract" (skip)
/// from "claimed a contract but emitted invalid JSON" (schema_reject).
fn is_none_sentinel(cleaned: &str) -> bool {
    let t = cleaned.trim().trim_matches('"');
    if t.is_empty() || t.eq_ignore_ascii_case("none") {
        return true;
    }
    serde_json::from_str::<serde_json::Value>(cleaned)
        .ok()
        .and_then(|v| {
            v.get("kind")
                .and_then(|k| k.as_str())
                .map(|s| s.eq_ignore_ascii_case("none"))
        })
        .unwrap_or(false)
}

/// Pure decision for one provider response (no I/O) — the testable core.
/// Provenance is overridden to the actual source statement (the model's
/// self-reported provenance is never trusted), and the `contract_id` is
/// namespaced `cve:<statement_id>` so extracted contracts are distinguishable
/// from temporal-projected (`contract_*`) and template (`tmpl:*`) ones.
fn classify_response(raw: &str, statement_id: &str, sentence: &str) -> CandidateOutcome {
    let cleaned = strip_code_fences(raw);
    if is_none_sentinel(&cleaned) {
        return CandidateOutcome::Skipped;
    }
    match parse_constrained_contract(&cleaned) {
        Ok(mut c) => {
            c.contract_id = format!("cve:{statement_id}");
            c.provenance.source_text = sentence.to_string();
            c.provenance.supporting_statement_ids = vec![statement_id.to_string()];
            apply_entailment_to_contract(&mut c, sentence);
            CandidateOutcome::Accepted(Box::new(c))
        }
        Err(_diag) => CandidateOutcome::SchemaReject,
    }
}

/// Build the constrained-decoding prompt: the provider-facing schema summary
/// plus a strict "emit JSON or `none`, never invent" instruction.
fn build_contract_prompt(sentence: &str) -> String {
    format!(
        "You extract at most ONE timed hardware contract from a single specification sentence.\n\
         Rules:\n\
         - If the sentence does not state a concrete timed obligation on a named signal, reply with exactly: none\n\
         - Otherwise reply with ONLY a JSON object conforming to this schema (no prose, no markdown fences):\n\
         {schema}\n\
         Use signal names exactly as they appear in the sentence. Never invent a signal, value, or cycle bound that is not present in the sentence.\n\
         Sentence: \"{sentence}\"",
        schema = actor_contract_json_schema_summary(),
    )
}

/// OpenAI-compatible text-only chat request (mirrors `nlp_enrich`).
fn build_text_chat_request(model: &str, prompt: &str) -> String {
    let prompt_escaped = prompt
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    format!(
        r#"{{"model": "{model}", "messages": [{{"role": "user", "content": "{prompt_escaped}"}}], "max_tokens": 512, "temperature": 0}}"#
    )
}

/// Extract assistant message content from an OpenAI-compatible chat response
/// (mirrors `nlp_enrich`).
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

/// Call the provider for one statement, returning the raw assistant text.
/// Honors the `SPECFORGE_VLM_HELPER` test override (same arg shape as
/// `nlp_enrich`: `--statement-id` / `--sentence`).
fn call_provider_for_contract(
    sentence: &str,
    statement_id: &str,
    model: &str,
    api_url: &str,
    provider: VlmProviderArg,
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
    let prompt = build_contract_prompt(sentence);
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
    let request_path = tempdir.path().join("contract_request.json");
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

/// Owned `(statement_id, text)` candidates so the loop does not borrow the IR
/// while we later mutate it.
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

/// Extract `ActorContract`s from prose and write them onto the EvidenceIR.
pub fn run(args: ExtractContractsArgs) -> Result<()> {
    let evidence_ir_path = if args.evidence_ir.exists() {
        args.evidence_ir.clone()
    } else {
        return Err(AppError::MissingPath(args.evidence_ir));
    };
    let mut ir = EvidenceIr::load_from_path(&evidence_ir_path)?;

    println!("command: extract-contracts");
    println!("mode: {}", if args.dry_run { "dry-run" } else { "execute" });
    println!("evidence_ir_path: {}", evidence_ir_path.display());
    println!("document_key: {}", ir.document_identity.document_key);

    let work = candidate_work(&ir, args.max_statements);
    println!("candidate_statements: {}", work.len());

    if matches!(args.provider, VlmProviderArg::Skip) {
        println!("llm_provider: skip");
        println!("extraction: skipped");
        println!(
            "hint: re-run with --provider ollama --model qwen2.5vl:7b to extract {} candidate(s)",
            work.len()
        );
        return Ok(());
    }

    let model = args
        .model
        .clone()
        .unwrap_or_else(|| default_model(args.provider));
    let url = api_url(args.provider);
    println!("llm_provider: {}", provider_name(args.provider));
    println!("llm_model: {model}");

    if args.dry_run {
        println!("extraction: dry-run (no LLM calls)");
        return Ok(());
    }

    let mut extracted: Vec<ActorContract> = Vec::new();
    let mut schema_rejects = 0usize;
    let candidates_seen = work.len();
    for (statement_id, sentence) in &work {
        let raw = call_provider_for_contract(sentence, statement_id, &model, url, args.provider)?;
        match classify_response(&raw, statement_id, sentence) {
            CandidateOutcome::Skipped => {}
            CandidateOutcome::SchemaReject => schema_rejects += 1,
            CandidateOutcome::Accepted(c) => extracted.push(*c),
        }
    }
    let contracts_accepted = extracted.len();
    println!("extracted_contracts: {contracts_accepted}");
    println!("schema_rejects: {schema_rejects}");

    ir.extracted_contracts = extracted;
    ir.constrained_extraction_stats = Some(ConstrainedExtractionStats {
        candidates_seen,
        schema_rejects,
        contracts_accepted,
    });
    ir.write_to_disk()?;
    println!("wrote: {}", evidence_ir_path.display());
    println!("next: re-run `specforge semantic` to fold these into actor_contracts");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::contract::LoweringDisposition;

    /// A valid `ActorContract` JSON whose signals the sentence mentions →
    /// entailment Pass → Accepted Lowerable.
    fn valid_drive_json() -> &'static str {
        r#"{"contract_id":"x","kind":"guarantee","obligation":{"kind":"drive","signal":"GRANT","value":"1"},"edge":"rising","clock_signal":"clk","provenance":{"source_text":"","modality":"prose"},"lowering":{"kind":"lowerable"},"automation_confidence":"medium"}"#
    }

    #[test]
    fn none_sentinel_skips() {
        assert!(matches!(
            classify_response("none", "s1", "anything"),
            CandidateOutcome::Skipped
        ));
        assert!(matches!(
            classify_response("  None  ", "s1", "anything"),
            CandidateOutcome::Skipped
        ));
        assert!(matches!(
            classify_response(r#"{"kind":"none"}"#, "s1", "anything"),
            CandidateOutcome::Skipped
        ));
    }

    #[test]
    fn malformed_json_fails_closed_to_schema_reject() {
        assert!(matches!(
            classify_response("this is not json", "s1", "GRANT goes high"),
            CandidateOutcome::SchemaReject
        ));
        // Claims a contract object but is missing required fields → fails closed.
        assert!(matches!(
            classify_response(r#"{"contract_id":"x"}"#, "s1", "GRANT goes high"),
            CandidateOutcome::SchemaReject
        ));
    }

    #[test]
    fn valid_contract_whose_signals_appear_is_accepted_lowerable() {
        let sentence = "GRANT is driven to 1 on clk";
        match classify_response(valid_drive_json(), "s7", sentence) {
            CandidateOutcome::Accepted(c) => {
                assert_eq!(c.contract_id, "cve:s7");
                assert_eq!(c.provenance.source_text, sentence);
                assert_eq!(
                    c.provenance.supporting_statement_ids,
                    vec!["s7".to_string()]
                );
                // Entailment Pass (GRANT + clk both appear) ⇒ stays Lowerable.
                assert!(matches!(c.lowering, LoweringDisposition::Lowerable));
            }
            other => panic!("expected Accepted, got {other:?}"),
        }
    }

    #[test]
    fn valid_contract_whose_signals_are_absent_is_entailment_residual() {
        // Sentence mentions neither GRANT nor clk ⇒ entailment Fail ⇒ the
        // honesty doctrine reroutes the Lowerable contract to Residual.
        let sentence = "the bus arbitrates fairly among requesters";
        match classify_response(valid_drive_json(), "s8", sentence) {
            CandidateOutcome::Accepted(c) => match &c.lowering {
                LoweringDisposition::Residual { reason } => {
                    assert!(reason.starts_with("entailment fail:"), "{reason}");
                }
                other => panic!("expected entailment Residual, got {other:?}"),
            },
            other => panic!("expected Accepted(Residual), got {other:?}"),
        }
    }

    #[test]
    fn strip_code_fences_unwraps_markdown_json() {
        let fenced = "```json\n{\"k\":1}\n```";
        assert_eq!(strip_code_fences(fenced), "{\"k\":1}");
        assert_eq!(strip_code_fences("  plain  "), "plain");
    }
}
