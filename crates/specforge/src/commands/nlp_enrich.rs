use std::fs;
use std::process::Command;

use crate::cli::{NlpEnrichArgs, VlmProviderArg};
use crate::error::{AppError, Result};
use crate::ir::evidence::{EvidenceIr, StatementClass};
use crate::ir::source::{
    AutomationConfidence, ConditionalRuleRecord, SignalConstraintKind, SignalConstraintRecord,
};

/// Environment variable overriding the LLM helper script (for unit testing).
/// Same variable as used by `specforge enrich` so a single mock can cover both.
const VLM_HELPER_ENV: &str = "SPECFORGE_VLM_HELPER";

/// Enrich an EvidenceIR artifact with LLM-extracted NLP constraints.
///
/// Architecture (NLP Level 3 — Layers B+C):
///   Input:  EvidenceIR with `NormativeStatement` sentences that Level 1/2 pattern-matching
///           could not classify into `SignalValueConstraint` or `ConditionalRule`.
///   Action: Send each sentence to a small LLM (qwen2.5vl:7b via Ollama, or OpenAI, LM Studio)
///           with a structured extraction prompt grounded by known declared signal names.
///   Passes: Runs up to `max_passes` iterations; each pass re-checks sentences not yet
///           extracted and stops when a full pass yields zero new records (convergence).
///   Output: New `SignalConstraintRecord` or `ConditionalRuleRecord` entries appended to the
///           EvidenceIR's `signal_constraints` / `conditional_rules` fields.
///           The EvidenceIR JSON is written back to disk.
///
/// Downstream: re-run `specforge semantic` after this step to pick up the new records.
pub fn run(args: NlpEnrichArgs) -> Result<()> {
    let evidence_ir_path = if args.evidence_ir.exists() {
        args.evidence_ir.clone()
    } else {
        return Err(AppError::MissingPath(args.evidence_ir));
    };

    let mut evidence_ir = EvidenceIr::load_from_path(&evidence_ir_path)?;

    println!("command: nlp-enrich");
    println!("mode: {}", if args.dry_run { "dry-run" } else { "execute" });
    println!("evidence_ir_path: {}", evidence_ir_path.display());
    println!(
        "document_key: {}",
        evidence_ir.document_identity.document_key
    );
    println!(
        "existing_signal_constraints: {}",
        evidence_ir.signal_constraints.len()
    );
    println!(
        "existing_conditional_rules: {}",
        evidence_ir.conditional_rules.len()
    );

    match args.vlm_provider {
        VlmProviderArg::Skip => {
            // Count candidates so the dry-run hint is informative.
            let candidates = count_candidate_statements(&evidence_ir);
            println!("llm_provider: skip");
            println!("nlp_enrichment: skipped");
            println!(
                "hint: re-run with --vlm-provider ollama --vlm-model qwen2.5vl:7b to enrich {candidates} sentences"
            );
            return Ok(());
        }
        provider => {
            let provider_name = match provider {
                VlmProviderArg::Ollama => "ollama",
                VlmProviderArg::OpenAi => "openai",
                VlmProviderArg::LmStudio => "lmstudio",
                VlmProviderArg::Skip => unreachable!(),
            };
            println!("llm_provider: {provider_name}");

            let model = args.vlm_model.unwrap_or_else(|| match provider {
                VlmProviderArg::Ollama => "qwen2.5vl:7b".to_string(),
                VlmProviderArg::OpenAi => "gpt-4o".to_string(),
                VlmProviderArg::LmStudio => "qwen2.5vl:7b".to_string(),
                VlmProviderArg::Skip => unreachable!(),
            });
            println!("llm_model: {model}");

            let api_url = match provider {
                VlmProviderArg::Ollama => "http://localhost:11434/v1/chat/completions".to_string(),
                VlmProviderArg::OpenAi => "https://api.openai.com/v1/chat/completions".to_string(),
                VlmProviderArg::LmStudio => "http://localhost:1234/v1/chat/completions".to_string(),
                VlmProviderArg::Skip => unreachable!(),
            };

            // Layer B: Build grounding signal list.
            // Explicit --grounding-signals overrides auto-extraction.
            // Empty string disables grounding.
            let grounding_signals: Vec<String> = match &args.grounding_signals {
                Some(explicit) if explicit.is_empty() => Vec::new(), // disabled
                Some(explicit) => explicit
                    .split(',')
                    .map(|s| s.trim().to_ascii_uppercase())
                    .filter(|s| !s.is_empty())
                    .collect(),
                None => auto_extract_declared_signals(&evidence_ir), // auto
            };
            println!("grounding_signals: {}", grounding_signals.len());
            if !grounding_signals.is_empty() {
                println!(
                    "  known_signals: {}",
                    grounding_signals
                        .iter()
                        .take(10)
                        .cloned()
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }

            // Layer C: multi-pass loop with convergence check.
            let max_passes = args.max_passes.max(1);
            let mut total_signal_constraints = 0usize;
            let mut total_conditional_rules = 0usize;
            let mut total_calls = 0usize;
            let mut total_errors = 0usize;

            for pass in 1..=max_passes {
                println!("--- pass {pass}/{max_passes} ---");

                // Re-derive candidates each pass so already-extracted sentences are skipped.
                let existing_signal_texts: std::collections::HashSet<String> = evidence_ir
                    .signal_constraints
                    .iter()
                    .map(|r| r.source_text.clone())
                    .collect();
                let existing_rule_texts: std::collections::HashSet<String> = evidence_ir
                    .conditional_rules
                    .iter()
                    .map(|r| r.source_text.clone())
                    .collect();

                let candidate_statements: Vec<_> = evidence_ir
                    .extracted_statements
                    .iter()
                    .filter(|s| {
                        matches!(s.class, StatementClass::NormativeStatement)
                            && !existing_signal_texts.contains(&s.text)
                            && !existing_rule_texts.contains(&s.text)
                            && s.text.split_whitespace().count() >= 5
                    })
                    .cloned()
                    .collect();

                let total_candidates = candidate_statements.len();
                let limit = if args.max_sentences == 0 {
                    total_candidates
                } else {
                    args.max_sentences.min(total_candidates)
                };

                println!("normative_statement_candidates: {total_candidates}");
                println!("sentences_this_pass: {limit}");

                if limit == 0 {
                    println!("convergence: no remaining candidates — stopping.");
                    break;
                }

                let mut signal_counter = evidence_ir.signal_constraints.len() + 1;
                let mut rule_counter = evidence_ir.conditional_rules.len() + 1;
                let mut new_signal_constraints = Vec::new();
                let mut new_conditional_rules = Vec::new();
                let mut calls_made = 0usize;
                let mut errors = 0usize;
                let mut skipped_none = 0usize;

                for statement in candidate_statements.iter().take(limit) {
                    if args.dry_run {
                        println!(
                            "  [dry-run] would enrich: \"{}\"",
                            truncate_for_display(&statement.text, 80)
                        );
                        continue;
                    }

                    calls_made += 1;
                    match call_llm_for_sentence(
                        &statement.text,
                        &statement.statement_id,
                        &model,
                        &api_url,
                        provider,
                        &grounding_signals,
                    ) {
                        Ok(NlpExtractionResult::SignalConstraint(mut record)) => {
                            record.constraint_id = format!("nlp3_sigcon_{signal_counter:04}");
                            record.source_text = statement.text.clone();
                            signal_counter += 1;
                            println!(
                                "  signal_constraint: {} {:?} ({})",
                                record.subject_signal,
                                record.constraint_kind,
                                truncate_for_display(&statement.text, 60)
                            );
                            new_signal_constraints.push(record);
                        }
                        Ok(NlpExtractionResult::ConditionalRule(mut record)) => {
                            record.rule_id = format!("nlp3_condrule_{rule_counter:04}");
                            record.source_text = statement.text.clone();
                            rule_counter += 1;
                            println!(
                                "  conditional_rule: antecedent='{}' ({})",
                                truncate_for_display(&record.antecedent_text, 40),
                                truncate_for_display(&statement.text, 60)
                            );
                            new_conditional_rules.push(record);
                        }
                        Ok(NlpExtractionResult::None) => {
                            skipped_none += 1;
                        }
                        Err(e) => {
                            eprintln!(
                                "warning: LLM call failed for statement {}: {e}",
                                statement.statement_id
                            );
                            errors += 1;
                        }
                    }
                }

                println!("llm_calls_made: {calls_made}");
                println!("new_signal_constraints: {}", new_signal_constraints.len());
                println!("new_conditional_rules: {}", new_conditional_rules.len());
                println!("no_extraction: {skipped_none}");
                println!("errors: {errors}");

                total_calls += calls_made;
                total_errors += errors;

                let pass_extracted = new_signal_constraints.len() + new_conditional_rules.len();
                total_signal_constraints += new_signal_constraints.len();
                total_conditional_rules += new_conditional_rules.len();

                if !args.dry_run && pass_extracted > 0 {
                    // Form 1: backannotation — reclassify the original ExtractedStatement entries
                    // whose text was successfully extracted this pass.  This closes the feedback
                    // loop from Level 3 back to Level 1/2: the sentence is no longer an opaque
                    // NormativeStatement; it now carries the precise class the LLM discovered.
                    //
                    // SignalConstraint extraction → StatementClass::SignalValueConstraint
                    // ConditionalRule extraction  → StatementClass::ConditionalRule
                    //
                    // Subsequent passes see the updated classes and skip these sentences
                    // automatically (the candidate filter already excludes non-NormativeStatement).
                    let signal_texts: std::collections::HashSet<&str> = new_signal_constraints
                        .iter()
                        .map(|r| r.source_text.as_str())
                        .collect();
                    let rule_texts: std::collections::HashSet<&str> = new_conditional_rules
                        .iter()
                        .map(|r| r.source_text.as_str())
                        .collect();

                    let mut backannotated = 0usize;
                    for stmt in &mut evidence_ir.extracted_statements {
                        if matches!(stmt.class, StatementClass::NormativeStatement) {
                            if signal_texts.contains(stmt.text.as_str()) {
                                stmt.class = StatementClass::SignalValueConstraint;
                                backannotated += 1;
                            } else if rule_texts.contains(stmt.text.as_str()) {
                                stmt.class = StatementClass::ConditionalRule;
                                backannotated += 1;
                            }
                        }
                    }
                    if backannotated > 0 {
                        println!(
                            "  backannotated: {backannotated} NormativeStatements reclassified"
                        );
                    }

                    evidence_ir
                        .signal_constraints
                        .extend(new_signal_constraints);
                    evidence_ir.conditional_rules.extend(new_conditional_rules);
                    // Write after every pass so progress is durable.
                    evidence_ir.write_to_disk()?;
                }

                // Layer C convergence: stop if nothing was extracted this pass.
                if pass_extracted == 0 && !args.dry_run {
                    println!("convergence: pass extracted 0 new records — stopping.");
                    break;
                }
            } // end for pass

            if !args.dry_run {
                println!("--- summary ---");
                println!("total_llm_calls: {total_calls}");
                println!("total_new_signal_constraints: {total_signal_constraints}");
                println!("total_new_conditional_rules: {total_conditional_rules}");
                println!("total_errors: {total_errors}");
                println!(
                    "enriched_evidence_ir_path: {}",
                    evidence_ir.artifact_layout.evidence_ir_path.display()
                );
                println!(
                    "total_signal_constraints: {}",
                    evidence_ir.signal_constraints.len()
                );
                println!(
                    "total_conditional_rules: {}",
                    evidence_ir.conditional_rules.len()
                );
                println!(
                    "next_stage: specforge semantic (re-run to pick up NLP Level 3 constraints)"
                );
            }
        }
    }

    Ok(())
}

/// Count NormativeStatement sentences not yet covered by existing records.
fn count_candidate_statements(evidence_ir: &EvidenceIr) -> usize {
    let existing_signal_texts: std::collections::HashSet<String> = evidence_ir
        .signal_constraints
        .iter()
        .map(|r| r.source_text.clone())
        .collect();
    let existing_rule_texts: std::collections::HashSet<String> = evidence_ir
        .conditional_rules
        .iter()
        .map(|r| r.source_text.clone())
        .collect();
    evidence_ir
        .extracted_statements
        .iter()
        .filter(|s| {
            matches!(s.class, StatementClass::NormativeStatement)
                && !existing_signal_texts.contains(&s.text)
                && !existing_rule_texts.contains(&s.text)
                && s.text.split_whitespace().count() >= 5
        })
        .count()
}

/// Auto-extract declared signal names from synthesized `Signal X is input/output` statements
/// in the EvidenceIR. These come from signal description tables and are authoritative.
/// Used as grounding context so the LLM can resolve implicit/pronoun references.
fn auto_extract_declared_signals(evidence_ir: &EvidenceIr) -> Vec<String> {
    let mut signals: Vec<String> = evidence_ir
        .extracted_statements
        .iter()
        .filter_map(|s| parse_signal_declaration_name(&s.text))
        .collect();
    signals.sort();
    signals.dedup();
    signals
}

/// Parse the signal name from a synthesized `Signal X is input/output [width N].` statement.
fn parse_signal_declaration_name(text: &str) -> Option<String> {
    let lowered = text.to_ascii_lowercase();
    if !lowered.starts_with("signal ") {
        return None;
    }
    // After "Signal ": the next whitespace-delimited token is the signal name.
    let rest = &text[7..];
    let name: String = rest
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .next()
        .unwrap_or("")
        .to_ascii_uppercase();
    if name.len() >= 2 && name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
        Some(name)
    } else {
        None
    }
}

/// Result of one LLM extraction call on a single normative sentence.
enum NlpExtractionResult {
    SignalConstraint(SignalConstraintRecord),
    ConditionalRule(ConditionalRuleRecord),
    /// LLM found no extractable hardware constraint.
    None,
}

/// Build the structured extraction prompt for a single normative sentence.
///
/// The prompt is deliberately concise to work well with small models (7B).
/// When `grounding_signals` is non-empty (Layer B), the known signal list is injected
/// before the sentence so the LLM can resolve implicit references like "it" or "the address".
fn build_nlp_prompt(sentence: &str, grounding_signals: &[String]) -> String {
    let grounding_section = if grounding_signals.is_empty() {
        String::new()
    } else {
        format!(
            "Known hardware signals in this specification: {}\n\n",
            grounding_signals.join(", ")
        )
    };
    format!(
        "You are a hardware protocol specification analyzer.\n\
         Extract a structured hardware constraint from the following sentence.\n\n\
         {grounding_section}\
         Sentence: \"{sentence}\"\n\n\
         Respond with exactly one JSON object (no other text):\n\
         - If a named signal has a behavioral constraint:\n\
           {{\"type\":\"signal_constraint\",\"subject_signal\":\"SIGNAL_NAME\",\
         \"constraint_kind\":\"must_not_change|must_be_stable|must_be_high|must_be_low|\
         must_be_asserted|must_be_deasserted|must_be_value\",\
         \"value\":\"OPTIONAL_ENUM_VALUE\",\"condition\":\"OPTIONAL_CONDITION_CLAUSE\",\
         \"negated\":false}}\n\
         - If a conditional behavioral rule (when X, Y shall ...):\n\
           {{\"type\":\"conditional_rule\",\"antecedent\":\"CONDITION_PHRASE\",\
         \"consequent_signal\":\"SIGNAL_OR_NULL\",\"consequent_action\":\"ACTION_PHRASE\"}}\n\
         - If no specific hardware constraint is extractable:\n\
           {{\"type\":\"none\"}}\n\n\
         Rules:\n\
         - subject_signal must be an uppercase hardware signal name (e.g. HTRANS, HREADY)\n\
         - Only output the JSON object, nothing else"
    )
}

/// Call the LLM with a text-only structured extraction prompt (no image).
fn call_llm_for_sentence(
    sentence: &str,
    statement_id: &str,
    model: &str,
    api_url: &str,
    provider: VlmProviderArg,
    grounding_signals: &[String],
) -> Result<NlpExtractionResult> {
    // Allow test override via SPECFORGE_VLM_HELPER env var.
    let raw_response = if let Some(helper_path) = std::env::var_os(VLM_HELPER_ENV) {
        let output = Command::new(&helper_path)
            .arg("--statement-id")
            .arg(statement_id)
            .arg("--sentence")
            .arg(sentence)
            .output()?;
        if output.status.success() {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        } else {
            return Err(AppError::ExternalCommandFailed {
                program: helper_path.display().to_string(),
                exit_code: output.status.code(),
                stderr: String::from_utf8_lossy(&output.stderr).to_string(),
            });
        }
    } else {
        let prompt = build_nlp_prompt(sentence, grounding_signals);
        let request_body = build_text_chat_request(model, &prompt);

        let mut cmd = Command::new("curl");
        cmd.arg("-s")
            .arg("-X")
            .arg("POST")
            .arg(api_url)
            .arg("-H")
            .arg("Content-Type: application/json");

        if matches!(provider, VlmProviderArg::OpenAi) {
            let api_key = std::env::var("OPENAI_API_KEY").map_err(|_| {
                AppError::MissingRuntimeDependency {
                    dependency: "OPENAI_API_KEY",
                    resolution: "Set OPENAI_API_KEY environment variable".to_string(),
                }
            })?;
            cmd.arg("-H")
                .arg(format!("Authorization: Bearer {api_key}"));
        }

        let tempdir = tempfile::tempdir()?;
        let request_path = tempdir.path().join("nlp_request.json");
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

        extract_chat_content(&String::from_utf8_lossy(&output.stdout))?
    };

    parse_nlp_response(&raw_response, statement_id)
}

/// Build an OpenAI-compatible text-only chat completions request (no image).
fn build_text_chat_request(model: &str, prompt: &str) -> String {
    let prompt_escaped = prompt
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n");
    format!(
        r#"{{"model": "{model}", "messages": [{{"role": "user", "content": "{prompt_escaped}"}}], "max_tokens": 256, "temperature": 0}}"#
    )
}

/// Extract assistant message content from an OpenAI-compatible chat response (shared logic).
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

/// Parse the LLM's JSON response into a typed NlpExtractionResult.
///
/// Robust to:
/// - JSON wrapped in markdown code fences (```json ... ```)
/// - Leading/trailing whitespace
/// - Extra fields (ignored by serde)
fn parse_nlp_response(raw: &str, statement_id: &str) -> Result<NlpExtractionResult> {
    // Strip markdown code fences if present.
    let json_str = strip_code_fence(raw.trim());

    let value: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => {
            // LLM returned non-JSON — treat as no extraction.
            return Ok(NlpExtractionResult::None);
        }
    };

    let extraction_type = value.get("type").and_then(|t| t.as_str()).unwrap_or("none");

    match extraction_type {
        "signal_constraint" => {
            let subject_signal = value
                .get("subject_signal")
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            if subject_signal.is_empty()
                || !subject_signal
                    .chars()
                    .all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            {
                // Invalid signal name — not extractable.
                return Ok(NlpExtractionResult::None);
            }

            let constraint_kind_str = value
                .get("constraint_kind")
                .and_then(|k| k.as_str())
                .unwrap_or("must_be_stable");
            let enum_value = value
                .get("value")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            let constraint_kind = parse_constraint_kind(constraint_kind_str, &enum_value);
            let condition_text = value
                .get("condition")
                .and_then(|c| c.as_str())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string());
            let negated = value
                .get("negated")
                .and_then(|n| n.as_bool())
                .unwrap_or(false);

            Ok(NlpExtractionResult::SignalConstraint(
                SignalConstraintRecord {
                    constraint_id: "nlp3_sigcon_0000".to_string(), // overwritten by caller
                    subject_signal,
                    constraint_kind,
                    target_value: None,
                    condition_text,
                    negated,
                    source_text: String::new(), // caller sets from statement.text
                    supporting_statement_ids: vec![statement_id.to_string()],
                    automation_confidence: AutomationConfidence::Medium,
                },
            ))
        }

        "conditional_rule" => {
            let antecedent = value
                .get("antecedent")
                .and_then(|a| a.as_str())
                .unwrap_or("")
                .trim()
                .to_string();
            if antecedent.is_empty() {
                return Ok(NlpExtractionResult::None);
            }
            let consequent_signal = value
                .get("consequent_signal")
                .and_then(|s| s.as_str())
                .filter(|s| !s.is_empty() && *s != "null")
                .map(|s| s.to_string());
            let consequent_action = value
                .get("consequent_action")
                .and_then(|a| a.as_str())
                .unwrap_or("")
                .trim()
                .to_string();

            Ok(NlpExtractionResult::ConditionalRule(
                ConditionalRuleRecord {
                    rule_id: "nlp3_condrule_0000".to_string(), // overwritten by caller
                    antecedent_text: antecedent,
                    consequent_signal,
                    consequent_action,
                    source_text: String::new(), // caller sets from statement.text
                    supporting_statement_ids: vec![statement_id.to_string()],
                    automation_confidence: AutomationConfidence::Medium,
                },
            ))
        }

        _ => Ok(NlpExtractionResult::None),
    }
}

/// Map constraint_kind string from LLM response to `SignalConstraintKind`.
fn parse_constraint_kind(kind_str: &str, enum_value: &str) -> SignalConstraintKind {
    match kind_str {
        "must_not_change" => SignalConstraintKind::MustNotChange,
        "must_be_stable" => SignalConstraintKind::MustBeStable,
        "must_be_high" => SignalConstraintKind::MustBeHigh,
        "must_be_low" => SignalConstraintKind::MustBeLow,
        "must_be_asserted" => SignalConstraintKind::MustBeAsserted,
        "must_be_deasserted" => SignalConstraintKind::MustBeDeasserted,
        "must_be_value" if !enum_value.is_empty() => SignalConstraintKind::MustBeValue {
            value: enum_value.to_string(),
        },
        _ => SignalConstraintKind::MustBeStable,
    }
}

/// Strip markdown code fences from an LLM response if present.
fn strip_code_fence(text: &str) -> &str {
    if let Some(inner) = text
        .strip_prefix("```json")
        .or_else(|| text.strip_prefix("```"))
    {
        if let Some(end) = inner.rfind("```") {
            return inner[..end].trim();
        }
        return inner.trim();
    }
    text
}

fn truncate_for_display(text: &str, max: usize) -> String {
    if text.len() <= max {
        text.to_string()
    } else {
        format!("{}…", &text[..max])
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::sync::{Mutex, OnceLock};

    use tempfile::tempdir;

    use super::*;
    use crate::cli::{NlpEnrichArgs, VlmProviderArg};
    use crate::error::Result;
    use crate::ir::evidence::{EvidenceIr, StatementClass};
    use crate::ir::source::SourceIr;

    /// Serializes all tests that read/write `SPECFORGE_VLM_HELPER` so they cannot
    /// race when `cargo test` runs them in parallel within the same process.
    fn vlm_helper_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    /// Create a minimal mock VLM helper script that returns a predefined JSON based
    /// on the --sentence argument content. Used via SPECFORGE_VLM_HELPER.
    fn write_mock_helper(dir: &std::path::Path, responses: &[(&str, &str)]) -> std::path::PathBuf {
        let script_path = dir.join("mock_nlp_helper.sh");
        let mut cases = String::new();
        for (keyword, response) in responses {
            cases.push_str(&format!(
                "if echo \"$SENTENCE\" | grep -q \"{keyword}\"; then\n  echo '{response}'\n  exit 0\nfi\n"
            ));
        }
        let script = format!(
            "#!/bin/bash\nSENTENCE=\"\"\nwhile [ $# -gt 0 ]; do\n  case \"$1\" in\n    --sentence) SENTENCE=\"$2\"; shift 2;;\n    *) shift;;\n  esac\ndone\n{cases}echo '{{\"type\":\"none\"}}'\n"
        );
        fs::write(&script_path, &script).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755)).unwrap();
        }
        script_path
    }

    #[test]
    fn nlp_enrich_upgrades_normative_statement_to_signal_constraint() -> Result<()> {
        // Tests the NLP Level 3 pipeline:
        //   EvidenceIR with NormativeStatement sentences
        //     → nlp-enrich calls LLM (mocked via SPECFORGE_VLM_HELPER)
        //     → new SignalConstraintRecord added to evidence_ir.signal_constraints
        let _lock = vlm_helper_lock();
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        // Write spec with a sentence that Level 2 misses ("cannot" not in keyword list).
        fs::write(
            &source,
            "# Protocol\nHTRANS cannot change during a waited transfer.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;

        // Verify the sentence is NormativeStatement (not SignalValueConstraint).
        let has_normative = evidence_ir
            .extracted_statements
            .iter()
            .any(|s| matches!(s.class, StatementClass::NormativeStatement));
        // This sentence may be SourceFact if "cannot" doesn't match — either way the NLP enrich
        // should handle it. We'll inject a NormativeStatement manually to keep the test robust.
        let _ = has_normative;

        // Inject a NormativeStatement sentence that the mock will classify.
        let injected_id = "stmt_injected_0001";
        evidence_ir
            .extracted_statements
            .push(crate::ir::evidence::ExtractedStatement {
                statement_id: injected_id.to_string(),
                text: "HTRANS cannot change during a waited transfer".to_string(),
                class: StatementClass::NormativeStatement,
                modality: crate::ir::evidence::EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            });
        evidence_ir.write_to_disk()?;

        // Write mock helper that returns a signal_constraint for "HTRANS".
        let helper = write_mock_helper(
            tempdir.path(),
            &[(
                "HTRANS",
                r#"{"type":"signal_constraint","subject_signal":"HTRANS","constraint_kind":"must_not_change","condition":"during a waited transfer","negated":false}"#,
            )],
        );

        unsafe { std::env::set_var(VLM_HELPER_ENV, &helper) };
        let result = run(NlpEnrichArgs {
            evidence_ir: evidence_ir.artifact_layout.evidence_ir_path.clone(),
            vlm_provider: VlmProviderArg::Ollama,
            vlm_model: Some("qwen2.5vl:7b".to_string()),
            dry_run: false,
            max_sentences: 0,
            grounding_signals: None,
            max_passes: 1,
        });
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };
        result?;

        // Reload EvidenceIR and verify new signal constraint was added.
        let enriched = EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;
        assert!(
            enriched
                .signal_constraints
                .iter()
                .any(|r| r.subject_signal == "HTRANS"
                    && matches!(r.constraint_kind, SignalConstraintKind::MustNotChange)
                    && r.automation_confidence == AutomationConfidence::Medium),
            "expected NLP Level 3 SignalConstraintRecord for HTRANS must_not_change"
        );

        Ok(())
    }

    #[test]
    fn nlp_enrich_dry_run_does_not_modify_evidence_ir() -> Result<()> {
        let _lock = vlm_helper_lock();
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(&source, "# Protocol\nHWRITE shall remain HIGH.\n")?;
        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let original_constraint_count = evidence_ir.signal_constraints.len();

        let helper = write_mock_helper(
            tempdir.path(),
            &[(
                "HWRITE",
                r#"{"type":"signal_constraint","subject_signal":"HWRITE","constraint_kind":"must_be_high","negated":false}"#,
            )],
        );

        unsafe { std::env::set_var(VLM_HELPER_ENV, &helper) };
        run(NlpEnrichArgs {
            evidence_ir: evidence_ir.artifact_layout.evidence_ir_path.clone(),
            vlm_provider: VlmProviderArg::Ollama,
            vlm_model: None,
            dry_run: true,
            max_sentences: 0,
            grounding_signals: None,
            max_passes: 1,
        })?;
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };

        // Reload and verify nothing changed.
        let reloaded = EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;
        assert_eq!(
            reloaded.signal_constraints.len(),
            original_constraint_count,
            "dry-run must not modify the evidence_ir on disk"
        );

        Ok(())
    }

    #[test]
    fn parse_nlp_response_handles_code_fence_wrapped_json() {
        let raw = "```json\n{\"type\":\"signal_constraint\",\"subject_signal\":\"HREADY\",\"constraint_kind\":\"must_be_high\",\"negated\":false}\n```";
        let result = parse_nlp_response(raw, "stmt_001").unwrap();
        assert!(
            matches!(result, NlpExtractionResult::SignalConstraint(r) if r.subject_signal == "HREADY")
        );
    }

    #[test]
    fn parse_nlp_response_returns_none_for_invalid_signal_name() {
        // LLM returns a lowercase or sentence-fragment signal name — reject it.
        let raw = r#"{"type":"signal_constraint","subject_signal":"the signal","constraint_kind":"must_be_high","negated":false}"#;
        let result = parse_nlp_response(raw, "stmt_002").unwrap();
        assert!(matches!(result, NlpExtractionResult::None));
    }

    #[test]
    fn parse_nlp_response_extracts_conditional_rule() {
        let raw = r#"{"type":"conditional_rule","antecedent":"HREADY is LOW","consequent_signal":"HTRANS","consequent_action":"shall remain NONSEQ"}"#;
        let result = parse_nlp_response(raw, "stmt_003").unwrap();
        assert!(
            matches!(result, NlpExtractionResult::ConditionalRule(r) if r.antecedent_text == "HREADY is LOW")
        );
    }

    // ── Form 1: backannotation ───────────────────────────────────────

    #[test]
    fn nlp_enrich_backannotates_extracted_statement_class() -> Result<()> {
        // After nlp-enrich extracts a SignalConstraintRecord from a NormativeStatement,
        // the original ExtractedStatement.class must be updated from NormativeStatement
        // to SignalValueConstraint (Form 1 backannotation feedback loop).
        let _lock = vlm_helper_lock();
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(&source, "# Protocol\nSome content.\n")?;
        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        // Inject a NormativeStatement that the mock LLM will classify as SignalConstraint.
        let target_text = "HWRITE shall remain HIGH during the burst";
        evidence_ir
            .extracted_statements
            .push(crate::ir::evidence::ExtractedStatement {
                statement_id: "stmt_backannotate_test".to_string(),
                text: target_text.to_string(),
                class: StatementClass::NormativeStatement,
                modality: crate::ir::evidence::EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            });
        evidence_ir.write_to_disk()?;

        let helper = write_mock_helper(
            tempdir.path(),
            &[(
                "HWRITE",
                r#"{"type":"signal_constraint","subject_signal":"HWRITE","constraint_kind":"must_be_high","negated":false}"#,
            )],
        );

        unsafe { std::env::set_var(VLM_HELPER_ENV, &helper) };
        run(NlpEnrichArgs {
            evidence_ir: evidence_ir.artifact_layout.evidence_ir_path.clone(),
            vlm_provider: VlmProviderArg::Ollama,
            vlm_model: Some("qwen2.5vl:7b".to_string()),
            dry_run: false,
            max_sentences: 0,
            grounding_signals: None,
            max_passes: 1,
        })?;
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };

        // Reload EvidenceIR and verify the statement was backannotated.
        let enriched = EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;
        let reclassified = enriched
            .extracted_statements
            .iter()
            .find(|s| s.text == target_text)
            .expect("injected statement should be present");
        assert_eq!(
            reclassified.class,
            StatementClass::SignalValueConstraint,
            "Form 1 backannotation: statement class must be updated from NormativeStatement \
             to SignalValueConstraint after successful LLM extraction"
        );

        Ok(())
    }

    // ── Layer B: grounding signals in prompt ────────────────────────────

    #[test]
    fn build_nlp_prompt_includes_grounding_signals_when_provided() {
        let grounding = vec![
            "HADDR".to_string(),
            "HTRANS".to_string(),
            "HREADY".to_string(),
        ];
        let prompt = build_nlp_prompt("The signal shall be held stable.", &grounding);
        assert!(
            prompt.contains("HADDR") && prompt.contains("HTRANS") && prompt.contains("HREADY"),
            "grounding signals must appear in the prompt"
        );
        assert!(
            prompt.contains("Known hardware signals"),
            "grounding section header must appear"
        );
    }

    #[test]
    fn build_nlp_prompt_without_grounding_has_no_known_signals_section() {
        let prompt = build_nlp_prompt("HTRANS shall be IDLE.", &[]);
        assert!(
            !prompt.contains("Known hardware signals"),
            "empty grounding must not produce a Known hardware signals section"
        );
    }

    #[test]
    fn parse_signal_declaration_name_extracts_uppercase_signal() {
        assert_eq!(
            parse_signal_declaration_name("Signal HADDR is output width 32."),
            Some("HADDR".to_string())
        );
        assert_eq!(
            parse_signal_declaration_name("Signal HTRANS is output width 2."),
            Some("HTRANS".to_string())
        );
        assert_eq!(
            parse_signal_declaration_name("Not a signal declaration."),
            None
        );
    }

    // ── Layer C: multi-pass convergence ───────────────────────────────

    #[test]
    fn nlp_enrich_multi_pass_stops_on_convergence() -> Result<()> {
        // With max_passes=3, if the first pass extracts everything, the second pass should
        // find 0 candidates and stop (convergence). This verifies the loop terminates.
        let _lock = vlm_helper_lock();
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");

        fs::write(
            &source,
            "# Protocol\nHTRANS cannot change during a waited transfer.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        // Inject a NormativeStatement for the mock to extract.
        evidence_ir
            .extracted_statements
            .push(crate::ir::evidence::ExtractedStatement {
                statement_id: "stmt_multi_pass".to_string(),
                text: "HTRANS cannot change during a waited transfer".to_string(),
                class: crate::ir::evidence::StatementClass::NormativeStatement,
                modality: crate::ir::evidence::EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            });
        evidence_ir.write_to_disk()?;

        let helper = write_mock_helper(
            tempdir.path(),
            &[(
                "HTRANS",
                r#"{"type":"signal_constraint","subject_signal":"HTRANS","constraint_kind":"must_not_change","negated":false}"#,
            )],
        );

        unsafe { std::env::set_var(VLM_HELPER_ENV, &helper) };
        let result = run(NlpEnrichArgs {
            evidence_ir: evidence_ir.artifact_layout.evidence_ir_path.clone(),
            vlm_provider: VlmProviderArg::Ollama,
            vlm_model: Some("qwen2.5vl:7b".to_string()),
            dry_run: false,
            max_sentences: 0,
            grounding_signals: None,
            max_passes: 3, // 3 passes requested, but should stop after 1 (convergence)
        });
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };
        result?;

        // Verify the constraint was added and the EvidenceIR is consistent.
        let enriched = EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;
        assert!(
            enriched
                .signal_constraints
                .iter()
                .any(|r| r.subject_signal == "HTRANS"),
            "HTRANS constraint should have been extracted in first pass"
        );

        Ok(())
    }
}
