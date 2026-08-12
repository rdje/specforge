use crate::cli::{NlpEnrichArgs, VlmProviderArg};
use crate::error::{AppError, Result};
use crate::ir::entity_typing::declared_signal_catalog;
use crate::ir::evidence::{
    EvidenceIr, ExtractorTier, FactKind, FactProvenanceRecord, StatementClass,
    signal_constraint_fact_key,
};
use crate::ir::source::{
    AutomationConfidence, ConditionalRuleRecord, SignalConstraintKind, SignalConstraintRecord,
};
use crate::llm_text;

/// Environment variable overriding the LLM helper script (for unit testing).
/// The shared `llm_text::call_text_provider` honors this same variable, so a
/// single mock covers `enrich`, `nlp-enrich`, `extract-contracts`, and
/// `signal-resolve`. Test-only here: the production path delegates to
/// `llm_text`, which owns its own copy of the name.
#[cfg(test)]
const VLM_HELPER_ENV: &str = "SPECFORGE_VLM_HELPER";

/// Enrich an EvidenceIR artifact with LLM-extracted NLP constraints.
///
/// Architecture (NLP Level 3 — Layers A–E + Forms 1+2):
///   Input:  EvidenceIR with `NormativeStatement` sentences that Level 1/2 pattern-matching
///           could not classify into `SignalValueConstraint` or `ConditionalRule`.
///   Action: Send each sentence to a small LLM (qwen2.5vl:7b via Ollama, or OpenAI, LM Studio)
///           with a structured extraction prompt grounded by known declared signal names.
///   Loop:   Iterates until the residual NormativeStatement count is STABLE between two
///           consecutive passes (residual(N) == residual(N-1)).  At the start of each pass,
///           Form 2 alias reclassification is applied first (free, no LLM calls), then the
///           remaining candidates are sent to the LLM.  Termination is guaranteed because the
///           residual pool is finite and can only decrease or stay flat.
///   Output: New `SignalConstraintRecord` or `ConditionalRuleRecord` entries appended to the
///           EvidenceIR's `signal_constraints` / `conditional_rules` fields.
///           The EvidenceIR JSON is written back to disk after each productive pass.
///
/// Downstream: re-run `specforge semantic` after this step to pick up the new records.
pub fn run(args: NlpEnrichArgs) -> Result<()> {
    let evidence_ir_path = if args.evidence_ir.exists() {
        args.evidence_ir.clone()
    } else {
        return Err(AppError::MissingPath(args.evidence_ir));
    };

    let mut evidence_ir = EvidenceIr::load_from_path(&evidence_ir_path)?;
    let normalized_existing_records = evidence_ir.dedup_loopback_records();

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
            // The current document owns the declaration catalog. An explicit list may narrow
            // that catalog, but cannot add external identities or disable grounding.
            let mut grounding_signals = auto_extract_declared_signals(&evidence_ir);
            if let Some(explicit) = &args.grounding_signals {
                let requested = explicit
                    .split(',')
                    .map(str::trim)
                    .filter(|signal| !signal.is_empty())
                    .collect::<std::collections::BTreeSet<_>>();
                grounding_signals.retain(|signal| requested.contains(signal.as_str()));
            }
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

            // Residual-stable convergence loop (Forms 1+2 integrated).
            // Each pass applies Form 2 alias reclassification first (free), counts the
            // remaining residual, then sends candidates to the LLM.
            // Termination criterion: residual(N) == residual(N-1) — nothing moved.
            // Guaranteed to terminate because the residual pool is finite and monotone.
            let mut prev_residual = usize::MAX; // sentinel: force the first pass to run
            let mut pass_number = 0usize;
            let mut total_signal_constraints = 0usize;
            let mut total_conditional_rules = 0usize;
            let mut total_calls = 0usize;
            let mut total_errors = 0usize;
            let mut total_alias_reclassified = 0usize;
            let mut normalized_records_written = false;

            loop {
                pass_number += 1;
                println!("--- pass {pass_number} ---");

                // Form 2: apply the accumulated alias map BEFORE counting residuals.
                // Aliases learned in previous passes reclassify NormativeStatements for free,
                // shrinking the candidate pool before the LLM is involved.
                if !args.dry_run {
                    let mut alias_counter = evidence_ir.signal_constraints.len() + 1;
                    let (alias_n, alias_records) =
                        evidence_ir.apply_alias_reclassification(&mut alias_counter);
                    if alias_n > 0 {
                        println!("  alias_reclassified: {alias_n} NormativeStatements via Form 2");
                        total_alias_reclassified += alias_n;
                        evidence_ir.signal_constraints.extend(alias_records);
                        evidence_ir.dedup_loopback_records();
                    }
                }

                // Count remaining residual AFTER alias reclassification.
                let current_residual = count_candidate_statements(&evidence_ir);
                println!("normative_statement_candidates: {current_residual}");

                // Convergence check: stop when residual is zero or hasn’t decreased.
                if current_residual == 0 {
                    println!("convergence: residual = 0 — all extractable sentences processed.");
                    break;
                }
                if current_residual >= prev_residual {
                    println!("convergence: residual stable at {current_residual} — stopping.");
                    break;
                }
                prev_residual = current_residual;

                // Re-derive the candidate list from the (now alias-reclassified) pool.
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

                let limit = if args.max_sentences == 0 {
                    candidate_statements.len()
                } else {
                    args.max_sentences.min(candidate_statements.len())
                };
                println!("sentences_this_pass: {limit}");

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
                            // Form 2: learn alias if signal name is not in the source text.
                            if let Some(phrase) =
                                extract_alias_phrase(&statement.text, &record.subject_signal)
                                && let std::collections::btree_map::Entry::Vacant(entry) =
                                    evidence_ir.signal_alias_map.entry(phrase)
                            {
                                println!(
                                    "  alias_learned: \"{}\" → {}",
                                    entry.key(),
                                    record.subject_signal
                                );
                                entry.insert(record.subject_signal.clone());
                            }
                            // PER-EXTRACTOR-FACT-TAGGING: record this NLP-tier
                            // find here (pre-dedup) so an overlap with a Pattern
                            // find is captured; the index is deduped by triple.
                            let prov = FactProvenanceRecord {
                                producer: ExtractorTier::Nlp,
                                fact_kind: FactKind::SignalConstraint,
                                canonical_key: signal_constraint_fact_key(&record),
                            };
                            if !evidence_ir.fact_provenance.contains(&prov) {
                                evidence_ir.fact_provenance.push(prov);
                            }
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

                // dry-run: show one pass of what would be done, then stop.
                if args.dry_run {
                    break;
                }

                let alias_map_size_before_llm = evidence_ir.signal_alias_map.len();

                if pass_extracted > 0 {
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
                    evidence_ir.dedup_loopback_records();
                    evidence_ir.refresh_signal_semantic_hints()?;
                    // Write after every pass so progress is durable.
                    evidence_ir.write_to_disk()?;
                    normalized_records_written = true;
                } else if evidence_ir.signal_alias_map.len() > alias_map_size_before_llm {
                    // New aliases were learned this pass but the LLM extracted nothing new.
                    // Persist so the alias map accumulates correctly.
                    evidence_ir.dedup_loopback_records();
                    evidence_ir.refresh_signal_semantic_hints()?;
                    evidence_ir.write_to_disk()?;
                    normalized_records_written = true;
                }
                // (No explicit convergence break here: the residual-stable check at the
                //  top of the next iteration handles it cleanly.)
            } // end convergence loop

            if !args.dry_run && normalized_existing_records && !normalized_records_written {
                evidence_ir.refresh_signal_semantic_hints()?;
                evidence_ir.write_to_disk()?;
            }

            if !args.dry_run {
                println!("--- summary ---");
                println!("total_llm_calls: {total_calls}");
                println!("total_new_signal_constraints: {total_signal_constraints}");
                println!("total_new_conditional_rules: {total_conditional_rules}");
                println!("total_alias_reclassified (Form 2): {total_alias_reclassified}");
                println!(
                    "signal_alias_map_size: {}",
                    evidence_ir.signal_alias_map.len()
                );
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
    declared_signal_catalog(evidence_ir)
}

/// Form 2: Extract a prose alias phrase for a signal from a sentence where Level 3
/// identified the signal but the name does not appear literally in the text.
///
/// Algorithm:
///  1. If the signal token already appears literally in the sentence — no alias needed.
///  2. Find the subject part (text before the first modal verb or copula).
///  3. Strip leading articles ("the", "a", "an", "its", ...).
///  4. Normalize to lowercase, trim surrounding punctuation, and limit to 4 words.
///  5. Reject single-word pronouns and phrases shorter than 4 characters.
///
/// Example:
///   sentence = "The address bus shall remain stable when HREADY is LOW"
///   subject_signal = "HADDR"
///   → Some("address bus")
fn extract_alias_phrase(sentence: &str, subject_signal: &str) -> Option<String> {
    // If the signal name appears literally, Level 1/2 would have caught it — skip.
    if sentence
        .split(|c: char| !c.is_ascii_alphanumeric() && c != '_')
        .any(|tok| tok.eq_ignore_ascii_case(subject_signal))
    {
        return None;
    }

    let lowered = sentence.to_ascii_lowercase();

    // Find the earliest modal verb / copula to locate the subject boundary.
    const MODAL_MARKERS: &[&str] = &[
        " shall ",
        " must ",
        " cannot ",
        " can not ",
        " will ",
        " may ",
        " is ",
        " are ",
        " was ",
        " were ",
    ];
    let subject_end = MODAL_MARKERS
        .iter()
        .filter_map(|marker| lowered.find(marker))
        .min()?;

    let subject_raw = sentence[..subject_end].trim();
    if subject_raw.is_empty() {
        return None;
    }
    if subject_has_markup_prefix(subject_raw) {
        return None;
    }

    // Strip leading articles / determiners then normalize.
    let stripped = strip_leading_articles(subject_raw);
    let normalized_subject = normalize_alias_subject_markup(stripped);
    let normalized = normalized_subject.to_ascii_lowercase();

    let words: Vec<String> = normalized
        .split_whitespace()
        .filter_map(clean_alias_word)
        .collect();

    // Reject single-word pronouns or trivially generic subjects.
    if words.is_empty()
        || (words.len() == 1
            && matches!(
                words[0].as_str(),
                "it" | "this" | "that" | "they" | "them" | "its" | "each" | "all"
            ))
    {
        return None;
    }

    // Need at least 2 words to form a useful alias phrase.
    if words.len() < 2 {
        return None;
    }

    // Limit to 4 words so the phrase stays general enough to match future sentences.
    let phrase = words
        .iter()
        .take(4)
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join(" ");

    if phrase.len() >= 4 {
        Some(phrase)
    } else {
        None
    }
}

fn clean_alias_word(word: &str) -> Option<String> {
    let word = word.trim_matches(|character: char| character.is_ascii_punctuation());
    if word.is_empty() {
        None
    } else {
        Some(word.to_string())
    }
}

fn normalize_alias_subject_markup(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len());
    let mut remaining = text;

    while let Some(link_start) = remaining.find('[') {
        normalized.push_str(&remaining[..link_start]);
        let after_open = &remaining[link_start + 1..];
        let Some(label_end) = after_open.find(']') else {
            normalized.push_str(&remaining[link_start..]);
            return normalized;
        };
        let label = &after_open[..label_end];
        let after_label = &after_open[label_end + 1..];
        if let Some(after_target_open) = after_label.strip_prefix('(')
            && let Some(target_end) = after_target_open.find(')')
        {
            normalized.push_str(label);
            remaining = &after_target_open[target_end + 1..];
            continue;
        }
        if let Some(after_reference_open) = after_label.strip_prefix('[')
            && let Some(reference_end) = after_reference_open.find(']')
        {
            normalized.push_str(label);
            remaining = &after_reference_open[reference_end + 1..];
            continue;
        }

        normalized.push('[');
        remaining = after_open;
    }

    normalized.push_str(remaining);
    normalized
}

fn subject_has_markup_prefix(subject: &str) -> bool {
    let subject = subject.trim_start();
    if subject.starts_with(['-', '|', '#', '*', '+', '>']) {
        return true;
    }

    if let Some(first_token) = subject.split_whitespace().next()
        && (is_numeric_outline_marker(first_token)
            || is_parenthesized_list_marker(first_token)
            || is_lettered_list_marker(first_token))
    {
        return true;
    }

    if subject_has_source_label_prefix(subject) {
        return true;
    }

    if let Some((marker, _rest)) = subject
        .split_once(['.', ')'])
        .filter(|(_, rest)| rest.starts_with(char::is_whitespace))
    {
        return !marker.is_empty() && marker.chars().all(|character| character.is_ascii_digit());
    }

    false
}

fn subject_has_source_label_prefix(subject: &str) -> bool {
    let mut tokens = subject.split_whitespace();
    let Some(first_token) = tokens.next() else {
        return false;
    };

    let label = first_token
        .trim_end_matches([':', '.', ')'])
        .to_ascii_lowercase();
    if !matches!(
        label.as_str(),
        "table" | "figure" | "fig" | "section" | "sec" | "clause" | "example"
    ) {
        return false;
    }

    if first_token.ends_with(':') {
        return true;
    }

    let Some(marker_token) = tokens.next() else {
        return false;
    };
    let marker = marker_token.trim_end_matches([':', ')']);
    let numeric_marker = marker.trim_end_matches('.');
    is_numeric_outline_marker(marker)
        || (!numeric_marker.is_empty()
            && numeric_marker
                .chars()
                .all(|character| character.is_ascii_digit()))
}

fn is_numeric_outline_marker(token: &str) -> bool {
    let marker = token.trim_end_matches(['.', ')']);
    if !marker.contains('.') {
        return false;
    }

    marker
        .split('.')
        .all(|part| !part.is_empty() && part.chars().all(|character| character.is_ascii_digit()))
}

fn is_parenthesized_list_marker(token: &str) -> bool {
    let Some(marker) = token
        .strip_prefix('(')
        .and_then(|rest| rest.strip_suffix(')'))
    else {
        return false;
    };

    !marker.is_empty()
        && marker.len() <= 3
        && marker
            .chars()
            .all(|character| character.is_ascii_alphanumeric())
}

fn is_lettered_list_marker(token: &str) -> bool {
    let marker = token.trim_end_matches(['.', ')']);
    marker.len() == 1
        && marker
            .chars()
            .all(|character| character.is_ascii_alphabetic())
        && token.len() == 2
}

/// Strip common leading determiners / articles from a noun phrase.
fn strip_leading_articles(text: &str) -> &str {
    let lowered = text.to_ascii_lowercase();
    for prefix in &[
        "the ", "a ", "an ", "this ", "its ", "each ", "all ", "every ", "any ",
    ] {
        if lowered.starts_with(prefix) {
            return &text[prefix.len()..];
        }
    }
    text
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
            "Declared signal symbols in the current document: {}\n\n",
            grounding_signals.join(", ")
        )
    };
    format!(
        "You extract one typed digital-hardware constraint from the following specification sentence.\n\
         Treat every document-owned symbol as opaque; spelling never implies a semantic role.\n\n\
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
         - subject_signal must exactly match one symbol in the declared-signal list; if that list is absent, do not emit a signal_constraint\n\
         - Only output the JSON object, nothing else"
    )
}

/// Call the LLM with a text-only structured extraction prompt (no image).
/// Per-sentence LLM response budget. Smaller than the other text commands
/// because `nlp-enrich` sends one request per unclassified normative statement;
/// the responses are short JSON objects.
const NLP_MAX_TOKENS: usize = 256;

fn call_llm_for_sentence(
    sentence: &str,
    statement_id: &str,
    model: &str,
    api_url: &str,
    provider: VlmProviderArg,
    grounding_signals: &[String],
) -> Result<NlpExtractionResult> {
    // Transport (curl + the SPECFORGE_VLM_HELPER test hook + OpenAI auth + the
    // OpenAI-compatible request/response shape) is shared via `llm_text`; this
    // command keeps only its prompt construction and typed response parsing.
    let prompt = build_nlp_prompt(sentence, grounding_signals);
    let raw_response = llm_text::call_text_provider(
        provider,
        model,
        api_url,
        statement_id,
        sentence,
        &prompt,
        NLP_MAX_TOKENS,
    )?;
    parse_nlp_response(&raw_response, statement_id, sentence, grounding_signals)
}

/// Parse the LLM's JSON response into a typed NlpExtractionResult.
///
/// Robust to:
/// - JSON wrapped in markdown code fences (```json ... ```)
/// - Leading/trailing whitespace
/// - Extra fields (ignored by serde)
fn grounded_signal_identifier(
    candidate: &str,
    sentence: &str,
    grounding_signals: &[String],
) -> Option<String> {
    let candidate = candidate.trim();
    let mut characters = candidate.chars();
    let first = characters.next()?;
    if !(first.is_ascii_alphabetic() || first == '_')
        || !characters.all(|character| character.is_ascii_alphanumeric() || character == '_')
    {
        return None;
    }

    let _ = sentence;
    grounding_signals
        .iter()
        .find(|declared| declared.as_str() == candidate)
        .cloned()
}

fn parse_nlp_response(
    raw: &str,
    statement_id: &str,
    sentence: &str,
    grounding_signals: &[String],
) -> Result<NlpExtractionResult> {
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
            let proposed_subject = value
                .get("subject_signal")
                .and_then(|s| s.as_str())
                .unwrap_or("")
                .trim();
            let Some(subject_signal) =
                grounded_signal_identifier(proposed_subject, sentence, grounding_signals)
            else {
                return Ok(NlpExtractionResult::None);
            };

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
            let proposed_consequent = value
                .get("consequent_signal")
                .and_then(|s| s.as_str())
                .map(str::trim)
                .filter(|signal| !signal.is_empty() && !signal.eq_ignore_ascii_case("null"));
            let consequent_signal = match proposed_consequent {
                Some(signal) => {
                    let Some(grounded) =
                        grounded_signal_identifier(signal, sentence, grounding_signals)
                    else {
                        return Ok(NlpExtractionResult::None);
                    };
                    Some(grounded)
                }
                None => None,
            };
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

    use tempfile::tempdir;

    use super::*;
    use crate::cli::{NlpEnrichArgs, VlmProviderArg};
    use crate::error::Result;
    use crate::ir::evidence::{EvidenceIr, StatementClass};
    use crate::ir::source::SourceIr;
    use crate::test_support::env_var_lock;

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

    fn add_declared_signal(evidence_ir: &mut EvidenceIr, signal: &str) {
        evidence_ir
            .extracted_statements
            .push(crate::ir::evidence::ExtractedStatement {
                statement_id: format!("stmt_declared_{signal}"),
                text: format!("Signal {signal} is input."),
                class: StatementClass::SourceFact,
                modality: crate::ir::evidence::EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            });
    }

    #[test]
    fn nlp_enrich_upgrades_normative_statement_to_signal_constraint() -> Result<()> {
        // Tests the NLP Level 3 pipeline:
        //   EvidenceIR with NormativeStatement sentences
        //     → nlp-enrich calls LLM (mocked via SPECFORGE_VLM_HELPER)
        //     → new SignalConstraintRecord added to evidence_ir.signal_constraints
        let _lock = env_var_lock();
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
        add_declared_signal(&mut evidence_ir, "HTRANS");
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
                    && r.automation_confidence == AutomationConfidence::Medium
                    && !r.supporting_statement_ids.is_empty()),
            "expected NLP Level 3 SignalConstraintRecord for HTRANS must_not_change"
        );

        // PER-EXTRACTOR-FACT-TAGGING: nlp-enrich must have tagged its HTRANS find
        // as an Nlp-tier signal-constraint fact-provenance entry.
        assert!(
            enriched
                .fact_provenance
                .iter()
                .any(|p| p.producer == ExtractorTier::Nlp
                    && p.fact_kind == FactKind::SignalConstraint
                    && p.canonical_key.starts_with("HTRANS|")),
            "expected an Nlp-tier fact-provenance entry for the HTRANS constraint"
        );

        Ok(())
    }

    #[test]
    fn nlp_enrich_dry_run_does_not_modify_evidence_ir() -> Result<()> {
        let _lock = env_var_lock();
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
        let result = parse_nlp_response(
            raw,
            "stmt_001",
            "HREADY must be HIGH.",
            &["HREADY".to_string()],
        )
        .unwrap();
        assert!(
            matches!(result, NlpExtractionResult::SignalConstraint(r) if r.subject_signal == "HREADY"
                && r.automation_confidence == AutomationConfidence::Medium
                && !r.supporting_statement_ids.is_empty())
        );
    }

    #[test]
    fn parse_nlp_response_returns_none_for_invalid_or_ungrounded_signal_name() {
        let raw = r#"{"type":"signal_constraint","subject_signal":"the signal","constraint_kind":"must_be_high","negated":false}"#;
        let result = parse_nlp_response(raw, "stmt_002", "The signal must be high.", &[]).unwrap();
        assert!(matches!(result, NlpExtractionResult::None));
        let ungrounded = r#"{"type":"signal_constraint","subject_signal":"invented","constraint_kind":"must_be_high","negated":false}"#;
        let result =
            parse_nlp_response(ungrounded, "stmt_002", "The channel must be high.", &[]).unwrap();
        assert!(matches!(result, NlpExtractionResult::None));

        let grounded = vec!["mixedCaseSignal".to_string()];
        let mixed_case = r#"{"type":"signal_constraint","subject_signal":"mixedCaseSignal","constraint_kind":"must_be_high","negated":false}"#;
        let result = parse_nlp_response(
            mixed_case,
            "stmt_002",
            "The channel must be high.",
            &grounded,
        )
        .unwrap();
        assert!(matches!(
            result,
            NlpExtractionResult::SignalConstraint(record)
                if record.subject_signal == "mixedCaseSignal"
        ));
    }

    #[test]
    fn parse_nlp_response_extracts_conditional_rule() {
        let raw = r#"{"type":"conditional_rule","antecedent":"HREADY is LOW","consequent_signal":"HTRANS","consequent_action":"shall remain NONSEQ"}"#;
        let result = parse_nlp_response(
            raw,
            "stmt_003",
            "When HREADY is LOW, HTRANS shall remain NONSEQ.",
            &["HREADY".to_string(), "HTRANS".to_string()],
        )
        .unwrap();
        assert!(
            matches!(result, NlpExtractionResult::ConditionalRule(r) if r.antecedent_text == "HREADY is LOW"
                && r.automation_confidence == AutomationConfidence::Medium
                && !r.supporting_statement_ids.is_empty())
        );
    }

    // ── Form 2: extract_alias_phrase ─────────────────────────────────

    #[test]
    fn extract_alias_phrase_returns_none_when_signal_appears_literally() {
        // "HADDR" IS in the text — Level 1/2 would have caught it, no alias needed.
        let result = extract_alias_phrase("HADDR shall remain stable", "HADDR");
        assert!(result.is_none(), "signal present literally → no alias");
    }

    #[test]
    fn extract_alias_phrase_extracts_noun_phrase_when_signal_absent() {
        // Classic implicit reference: "The address bus" → "HADDR"
        let result = extract_alias_phrase(
            "The address bus shall remain stable when HREADY is LOW",
            "HADDR",
        );
        assert_eq!(
            result.as_deref(),
            Some("address bus"),
            "should strip \"The\" and return the 2-word noun phrase"
        );
    }

    #[test]
    fn extract_alias_phrase_rejects_pronoun_only_subjects() {
        // "It shall be held HIGH" — "it" alone is too ambiguous.
        let result = extract_alias_phrase("It shall be held HIGH throughout the burst", "HTRANS");
        assert!(
            result.is_none(),
            "pronoun-only subject 'it' must be rejected as an alias phrase"
        );
    }

    #[test]
    fn extract_alias_phrase_limits_to_four_words() {
        // Very long subject — alias should be at most 4 words.
        let result = extract_alias_phrase(
            "The write enable control signal shall be deasserted when the burst ends",
            "HWRITE",
        );
        if let Some(phrase) = result {
            let word_count = phrase.split_whitespace().count();
            assert!(
                word_count <= 4,
                "alias phrase must not exceed 4 words, got: \"{phrase}\""
            );
        }
    }

    #[test]
    fn extract_alias_phrase_rejects_markdown_marker_prefixes() {
        assert!(
            extract_alias_phrase("- the address shall remain stable", "HADDR").is_none(),
            "bullet-style markdown prefixes must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("| the address bus shall remain stable", "HADDR").is_none(),
            "table-cell markdown prefixes must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("# the address bus shall remain stable", "HADDR").is_none(),
            "heading-style markdown prefixes must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("* the address bus shall remain stable", "HADDR").is_none(),
            "asterisk markdown bullets must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("+ the address bus shall remain stable", "HADDR").is_none(),
            "plus markdown bullets must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("> the address bus shall remain stable", "HADDR").is_none(),
            "quote-style markdown prefixes must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("1. the address bus shall remain stable", "HADDR").is_none(),
            "ordered-list markdown prefixes must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("2) the address bus shall remain stable", "HADDR").is_none(),
            "ordered-list paren prefixes must not become learned aliases"
        );
    }

    #[test]
    fn extract_alias_phrase_rejects_outline_and_lettered_marker_prefixes() {
        assert!(
            extract_alias_phrase("3.1 the address bus shall remain stable", "HADDR").is_none(),
            "numeric outline prefixes must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("4.2.1 the address bus shall remain stable", "HADDR").is_none(),
            "nested numeric outline prefixes must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("(a) the address bus shall remain stable", "HADDR").is_none(),
            "parenthesized lettered prefixes must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("(12) the address bus shall remain stable", "HADDR").is_none(),
            "parenthesized numeric prefixes must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("b) the address bus shall remain stable", "HADDR").is_none(),
            "lettered list prefixes must not become learned aliases"
        );
    }

    #[test]
    fn extract_alias_phrase_rejects_source_label_prefixes() {
        assert!(
            extract_alias_phrase("Table 3: The address bus shall remain stable", "HADDR").is_none(),
            "table labels must not become learned aliases"
        );
        assert!(
            extract_alias_phrase(
                "Figure 4.2 the write enable control must be stable",
                "HWRITE"
            )
            .is_none(),
            "figure labels must not become learned aliases"
        );
        assert!(
            extract_alias_phrase("Section 3.1 the address bus shall remain stable", "HADDR")
                .is_none(),
            "section labels must not become learned aliases"
        );
    }

    #[test]
    fn extract_alias_phrase_trims_wrapping_punctuation_from_alias_words() {
        assert_eq!(
            extract_alias_phrase("The `address bus` shall remain stable", "HADDR").as_deref(),
            Some("address bus"),
            "inline-code delimiters around prose aliases should not be learned literally"
        );
        assert_eq!(
            extract_alias_phrase("The address bus, shall remain stable", "HADDR").as_deref(),
            Some("address bus"),
            "trailing commas before the modal boundary should not pollute learned aliases"
        );
        assert_eq!(
            extract_alias_phrase("The address bus: must remain stable", "HADDR").as_deref(),
            Some("address bus"),
            "trailing colons before the modal boundary should not pollute learned aliases"
        );
    }

    #[test]
    fn extract_alias_phrase_uses_markdown_link_labels() {
        assert_eq!(
            extract_alias_phrase(
                "The [address bus](#address-bus) shall remain stable",
                "HADDR"
            )
            .as_deref(),
            Some("address bus"),
            "markdown link targets should not be learned as alias text"
        );
        assert_eq!(
            extract_alias_phrase(
                "The [write enable control](signals.md#write-enable) must be stable",
                "HWRITE"
            )
            .as_deref(),
            Some("write enable control"),
            "link labels should remain usable as learned prose aliases"
        );
    }

    #[test]
    fn extract_alias_phrase_uses_reference_link_labels() {
        assert_eq!(
            extract_alias_phrase(
                "The [address bus][address-bus-ref] shall remain stable",
                "HADDR"
            )
            .as_deref(),
            Some("address bus"),
            "reference-style link ids should not be learned as alias text"
        );
        assert_eq!(
            extract_alias_phrase("The [write enable control][] must be stable", "HWRITE")
                .as_deref(),
            Some("write enable control"),
            "collapsed reference-style links should still expose the visible label"
        );
    }

    #[test]
    fn nlp_enrich_learns_alias_and_stores_in_evidence_ir() -> Result<()> {
        // End-to-end test: when Level 3 extracts a constraint from a sentence where
        // the signal name is NOT in the text, the alias is stored in signal_alias_map.
        let _lock = env_var_lock();
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
        add_declared_signal(&mut evidence_ir, "HADDR");
        // Inject a NormativeStatement where the signal name is absent from the text.
        // "The address bus" is the prose subject; the mock LLM resolves it to HADDR.
        let alias_sentence = "The address bus shall remain stable when HREADY is LOW";
        evidence_ir
            .extracted_statements
            .push(crate::ir::evidence::ExtractedStatement {
                statement_id: "stmt_alias_learn".to_string(),
                text: alias_sentence.to_string(),
                class: crate::ir::evidence::StatementClass::NormativeStatement,
                modality: crate::ir::evidence::EvidenceModality::Text,
                evidence_span_ids: vec![],
                related_visual_evidence_ids: vec![],
            });
        evidence_ir.write_to_disk()?;

        // Mock LLM returns HADDR as the subject signal.
        let helper = write_mock_helper(
            tempdir.path(),
            &[(
                "address bus",
                r#"{"type":"signal_constraint","subject_signal":"HADDR","constraint_kind":"must_be_stable","negated":false}"#,
            )],
        );

        unsafe { std::env::set_var(VLM_HELPER_ENV, &helper) };
        run(NlpEnrichArgs {
            evidence_ir: evidence_ir.artifact_layout.evidence_ir_path.clone(),
            vlm_provider: VlmProviderArg::Ollama,
            vlm_model: Some("qwen2.5vl:7b".to_string()),
            dry_run: false,
            max_sentences: 0,
            grounding_signals: Some("HADDR".to_string()),
        })?;
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };

        let enriched = EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;

        // The alias map must contain "address bus" → "HADDR".
        assert_eq!(
            enriched
                .signal_alias_map
                .get("address bus")
                .map(String::as_str),
            Some("HADDR"),
            "Form 2: alias \"address bus\" → HADDR must be persisted in signal_alias_map"
        );
        assert!(enriched.signal_constraints.iter().any(|r| {
            r.subject_signal == "HADDR"
                && r.automation_confidence == AutomationConfidence::Medium
                && !r.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    // ── Form 1: backannotation ───────────────────────────────────────

    #[test]
    fn nlp_enrich_backannotates_extracted_statement_class() -> Result<()> {
        // After nlp-enrich extracts a SignalConstraintRecord from a NormativeStatement,
        // the original ExtractedStatement.class must be updated from NormativeStatement
        // to SignalValueConstraint (Form 1 backannotation feedback loop).
        let _lock = env_var_lock();
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
        add_declared_signal(&mut evidence_ir, "HWRITE");
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
        assert!(enriched.signal_constraints.iter().any(|r| {
            r.subject_signal == "HWRITE"
                && matches!(r.constraint_kind, SignalConstraintKind::MustBeHigh)
                && r.automation_confidence == AutomationConfidence::Medium
                && !r.supporting_statement_ids.is_empty()
        }));

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
            prompt.contains("Declared signal symbols in the current document"),
            "grounding section header must appear"
        );
    }

    #[test]
    fn nlp_prompt_is_alpha_equivariant_and_assigns_no_role_from_spelling() {
        let first = build_nlp_prompt("copper shall remain stable.", &["copper".to_string()])
            .replace("copper", "<signal>");
        let renamed = build_nlp_prompt("silver shall remain stable.", &["silver".to_string()])
            .replace("silver", "<signal>");
        assert_eq!(first, renamed);
        assert!(first.contains("spelling never implies a semantic role"));
        assert!(!first.contains("HTRANS"));
        assert!(!first.contains("HREADY"));
    }

    #[test]
    fn build_nlp_prompt_without_grounding_has_no_known_signals_section() {
        let prompt = build_nlp_prompt("HTRANS shall be IDLE.", &[]);
        assert!(
            !prompt.contains("Declared signal symbols in the current document"),
            "empty grounding must not produce a declared-signal section"
        );
    }

    // ── Layer C: multi-pass convergence ───────────────────────────────

    #[test]
    fn nlp_enrich_multi_pass_stops_on_convergence() -> Result<()> {
        // After the first pass extracts the only candidate, the residual drops to 0
        // and the loop stops on convergence (residual-stable criterion).
        let _lock = env_var_lock();
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
        add_declared_signal(&mut evidence_ir, "HTRANS");
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
        });
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };
        result?;

        // Verify the constraint was added and the EvidenceIR is consistent.
        let enriched = EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;
        assert!(
            enriched
                .signal_constraints
                .iter()
                .any(|r| r.subject_signal == "HTRANS"
                    && r.automation_confidence == AutomationConfidence::Medium
                    && !r.supporting_statement_ids.is_empty()),
            "HTRANS constraint should have been extracted in first pass"
        );

        Ok(())
    }

    #[test]
    fn nlp_enrich_dedups_duplicate_extractions_before_persisting() -> Result<()> {
        let _lock = env_var_lock();
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
        add_declared_signal(&mut evidence_ir, "HWRITE");

        let duplicate_text = "HWRITE shall remain HIGH during the burst";
        for suffix in ["a", "b"] {
            evidence_ir
                .extracted_statements
                .push(crate::ir::evidence::ExtractedStatement {
                    statement_id: format!("stmt_duplicate_{suffix}"),
                    text: duplicate_text.to_string(),
                    class: StatementClass::NormativeStatement,
                    modality: crate::ir::evidence::EvidenceModality::Text,
                    evidence_span_ids: vec![],
                    related_visual_evidence_ids: vec![],
                });
        }
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
        })?;
        unsafe { std::env::remove_var(VLM_HELPER_ENV) };

        let enriched = EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;
        let persisted_matches = enriched
            .signal_constraints
            .iter()
            .filter(|record| {
                record.subject_signal == "HWRITE"
                    && matches!(record.constraint_kind, SignalConstraintKind::MustBeHigh)
                    && record.source_text == duplicate_text
                    && record.automation_confidence == AutomationConfidence::Medium
                    && !record.supporting_statement_ids.is_empty()
            })
            .count();
        assert_eq!(
            persisted_matches, 1,
            "duplicate LLM extractions for identical source text must be deduplicated"
        );

        Ok(())
    }
}
