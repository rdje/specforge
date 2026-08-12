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
//! The OpenAI-compatible text transport (curl + `SPECFORGE_VLM_HELPER` hook +
//! request/response shape) lives in the shared `crate::commands::llm_text`
//! helper; this module keeps only the contract-specific prompt + classifier.

use crate::cli::{ExtractContractsArgs, VlmProviderArg};
use crate::commands::llm_text;
use crate::error::{AppError, Result};
use crate::ir::contract::ActorContract;
use crate::ir::contract::LoweringDisposition;
use crate::ir::cve::{
    ConstrainedExtractionStats, actor_contract_json_schema_summary, actor_contract_signal_names,
    apply_entailment_to_contract, parse_constrained_contract,
};
use crate::ir::entity_typing::declared_signal_catalog;
use crate::ir::evidence::{EvidenceIr, StatementClass};
use std::collections::BTreeSet;

/// Minimum word count for a prose statement to be a contract candidate
/// (mirrors `nlp_enrich`'s candidate floor — very short fragments carry no
/// extractable timed obligation).
const MIN_CANDIDATE_WORDS: usize = 5;

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
fn classify_response(
    raw: &str,
    statement_id: &str,
    sentence: &str,
    declared_signals: &BTreeSet<String>,
) -> CandidateOutcome {
    let cleaned = strip_code_fences(raw);
    if is_none_sentinel(&cleaned) {
        return CandidateOutcome::Skipped;
    }
    match parse_constrained_contract(&cleaned) {
        Ok(mut c) => {
            c.contract_id = format!("cve:{statement_id}");
            c.provenance.source_text = sentence.to_string();
            c.provenance.supporting_statement_ids = vec![statement_id.to_string()];
            let missing_signals = actor_contract_signal_names(&c)
                .into_iter()
                .filter(|signal| !declared_signals.contains(signal))
                .collect::<Vec<_>>();
            if !missing_signals.is_empty() {
                c.lowering = LoweringDisposition::Residual {
                    reason: format!(
                        "current-document declaration missing for contract signals: {missing_signals:?}"
                    ),
                };
            }
            apply_entailment_to_contract(&mut c, sentence);
            CandidateOutcome::Accepted(Box::new(c))
        }
        Err(_diag) => CandidateOutcome::SchemaReject,
    }
}

/// Build the constrained-decoding prompt: the provider-facing schema summary
/// plus a strict "emit JSON or `none`, never invent" instruction.
fn build_contract_prompt(sentence: &str, declared_signals: &[String]) -> String {
    let declaration_catalog = if declared_signals.is_empty() {
        "none".to_string()
    } else {
        declared_signals.join(", ")
    };
    format!(
        "You extract at most ONE typed timed contract from a digital-hardware specification sentence.\n\
         Treat every document-owned symbol as opaque; spelling never implies a role.\n\
         Declared signal symbols in the current document: {declaration_catalog}\n\
         Rules:\n\
         - If the declaration catalog is none, or the sentence does not state a concrete timed obligation on an exactly declared signal, reply with exactly: none\n\
         - Otherwise reply with ONLY a JSON object conforming to this schema (no prose, no markdown fences):\n\
         {schema}\n\
         Use only exact signal symbols from the declaration catalog and copy values/cycle bounds from the sentence. Never invent any of them.\n\
         Sentence: \"{sentence}\"",
        schema = actor_contract_json_schema_summary(),
    )
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
    let declared_signal_catalog = declared_signal_catalog(&ir);
    let declared_signals = declared_signal_catalog
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
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
        .unwrap_or_else(|| llm_text::default_model(args.provider));
    let url = llm_text::api_url(args.provider);
    println!("llm_provider: {}", llm_text::provider_name(args.provider));
    println!("llm_model: {model}");

    if args.dry_run {
        println!("extraction: dry-run (no LLM calls)");
        return Ok(());
    }

    let mut extracted: Vec<ActorContract> = Vec::new();
    let mut schema_rejects = 0usize;
    let candidates_seen = work.len();
    for (statement_id, sentence) in &work {
        let prompt = build_contract_prompt(sentence, &declared_signal_catalog);
        let raw = llm_text::call_text_provider(
            args.provider,
            &model,
            url,
            statement_id,
            sentence,
            &prompt,
            512,
        )?;
        match classify_response(&raw, statement_id, sentence, &declared_signals) {
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

    fn declarations(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|name| (*name).to_string()).collect()
    }

    /// A valid `ActorContract` JSON whose signals the sentence mentions →
    /// entailment Pass → Accepted Lowerable.
    fn valid_drive_json() -> &'static str {
        r#"{"contract_id":"x","kind":"guarantee","obligation":{"kind":"drive","signal":"GRANT","value":"1"},"edge":"rising","clock_signal":"clk","provenance":{"source_text":"","modality":"prose"},"lowering":{"kind":"lowerable"},"automation_confidence":"medium"}"#
    }

    #[test]
    fn none_sentinel_skips() {
        assert!(matches!(
            classify_response("none", "s1", "anything", &BTreeSet::new()),
            CandidateOutcome::Skipped
        ));
        assert!(matches!(
            classify_response("  None  ", "s1", "anything", &BTreeSet::new()),
            CandidateOutcome::Skipped
        ));
        assert!(matches!(
            classify_response(r#"{"kind":"none"}"#, "s1", "anything", &BTreeSet::new()),
            CandidateOutcome::Skipped
        ));
    }

    #[test]
    fn malformed_json_fails_closed_to_schema_reject() {
        assert!(matches!(
            classify_response(
                "this is not json",
                "s1",
                "GRANT goes high",
                &BTreeSet::new()
            ),
            CandidateOutcome::SchemaReject
        ));
        // Claims a contract object but is missing required fields → fails closed.
        assert!(matches!(
            classify_response(
                r#"{"contract_id":"x"}"#,
                "s1",
                "GRANT goes high",
                &BTreeSet::new()
            ),
            CandidateOutcome::SchemaReject
        ));
    }

    #[test]
    fn valid_contract_whose_signals_appear_is_accepted_lowerable() {
        let sentence = "GRANT is driven to 1 on clk";
        match classify_response(
            valid_drive_json(),
            "s7",
            sentence,
            &declarations(&["GRANT", "clk"]),
        ) {
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
        match classify_response(
            valid_drive_json(),
            "s8",
            sentence,
            &declarations(&["GRANT", "clk"]),
        ) {
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

    #[test]
    fn contract_prompt_is_alpha_equivariant_and_declaration_grounded() {
        let first = build_contract_prompt(
            "orchid rises on copper",
            &["orchid".to_string(), "copper".to_string()],
        )
        .replace("orchid", "<data>")
        .replace("copper", "<clock>");
        let renamed = build_contract_prompt(
            "juniper rises on silver",
            &["juniper".to_string(), "silver".to_string()],
        )
        .replace("juniper", "<data>")
        .replace("silver", "<clock>");
        assert_eq!(first, renamed);
        assert!(first.contains("spelling never implies a role"));
        assert!(build_contract_prompt("anything", &[]).contains("declaration catalog is none"));
    }

    #[test]
    fn undeclared_contract_signal_is_forced_to_residual() {
        match classify_response(
            valid_drive_json(),
            "s9",
            "GRANT is driven to 1 on clk",
            &declarations(&["GRANT"]),
        ) {
            CandidateOutcome::Accepted(contract) => assert!(matches!(
                contract.lowering,
                LoweringDisposition::Residual { ref reason }
                    if reason.contains("current-document declaration missing")
            )),
            other => panic!("expected an explicit residual, got {other:?}"),
        }
    }
}
