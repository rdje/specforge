//! `eval-extraction` — score the LLM extraction passes against a labeled dataset.
//!
//! `LLM-EXTRACTION-EVAL.4`: the provider-gated runner. For each `(doc_key, task)` in
//! the dataset it runs the *real* extraction command (`nlp-enrich` / `signal-resolve`)
//! with the chosen provider/model **on a temp copy** of the document's EvidenceIR
//! (redirecting the IR's `artifact_layout` to a temp dir so the corpus artifact is never
//! mutated), reads the produced typed records, indexes them by statement provenance, and
//! scores them against the gold labels via [`crate::eval`].
//!
//! `--provider skip` makes the extraction commands no-op, so the temp copy keeps only the
//! deterministic pattern records — i.e. the **baseline**. A model A/B is two runs
//! (`skip` → `qwen2.5vl:7b` → `qwen3-vl:8b`) compared on the same dataset.

use crate::cli::{EvalExtractionArgs, NlpEnrichArgs, SignalResolveArgs, VlmProviderArg};
use crate::error::Result;
use crate::eval::{
    self, EvalItem, EvalTask, PredictedKeys, Scorecard, index_constraint_predictions,
    index_relation_predictions, index_temporal_rule_predictions,
};
use crate::ir::entity_typing::{EntityEvidence, classify_entity, propose_entity_type_llm};
use crate::ir::evidence::{
    EvidenceIr, ExtractorTier, FactProvenanceRecord, actor_signal_relation_fact_key,
    signal_constraint_fact_key,
};
use crate::ir::extraction_filters::{
    is_grounded_obligation_with, is_normative_for_subject, is_valid_actor, is_valid_actor_with,
};
use crate::ir::nli_verify::{constraint_claim_text, verify_entailment};
use crate::ir::semantic::{InterfaceSignalRecord, SemanticIr, TemporalRuleRecord};
use crate::ir::source::{ActorSignalRelation, RegisterRecord, SignalConstraintRecord};
use std::collections::BTreeSet;
use std::path::Path;

/// The typed records produced for one `(doc, task)`. The two LLM tasks run the real
/// extraction command; the temporal task builds the SemanticIR (deterministic parser).
#[derive(Clone)]
enum TaskRecords {
    Constraints(Vec<SignalConstraintRecord>),
    Relations(Vec<ActorSignalRelation>),
    TemporalRules(Vec<TemporalRuleRecord>),
    /// Deterministic protocol-structure surfaces read straight from EvidenceIR.
    SerialFrameFields(Vec<crate::ir::evidence::SerialFrameField>),
    ProtocolOperations(Vec<crate::ir::evidence::ProtocolOperationRecord>),
    ProtocolStates(Vec<crate::ir::evidence::ProtocolStateRecord>),
    /// Deterministic interface edge timing from EvidenceIR.
    InterfaceEdgeTimings(Vec<crate::ir::evidence::InterfaceEdgeTimingRecord>),
    /// PDF-VARIANT-DIGESTION.4a.1 — deterministic register-field records read straight from EvidenceIR.
    RegisterFields(Vec<RegisterRecord>),
    /// PDF-VARIANT-DIGESTION.4a.4 — declared interface signals from the deterministic SemanticIR inventory.
    DeclaredSignals(Vec<InterfaceSignalRecord>),
}

/// `WIRE-BASED-100.6/.7` — drop OVER-GENERATED facts before scoring: relations whose subject is not
/// a real actor (a function word / the spec's own name), and constraints hallucinated from a
/// non-normative (descriptive) source. Derived, universal-language checks (ADR 0006).
fn filter_overgenerated(
    records: TaskRecords,
    provider: VlmProviderArg,
    model: &str,
) -> TaskRecords {
    // `.6c`/`.7c` — automatic detection when a provider is available (the LLM generalizes beyond the
    // heuristic lists); pure heuristics under `--provider skip` (no LLM).
    let use_llm = !matches!(provider, VlmProviderArg::Skip);
    match records {
        TaskRecords::Relations(rs) => TaskRecords::Relations(
            rs.into_iter()
                .filter(|r| {
                    if use_llm {
                        is_valid_actor_with(&r.actor_name, |a| {
                            classify_entity(
                                &EntityEvidence {
                                    token: a.to_string(),
                                    appears_as_actor: true,
                                    ..Default::default()
                                },
                                |e| propose_entity_type_llm(e, provider, model),
                            )
                        })
                    } else {
                        is_valid_actor(&r.actor_name)
                    }
                })
                .collect(),
        ),
        TaskRecords::Constraints(cs) => TaskRecords::Constraints(
            cs.into_iter()
                .filter(|c| {
                    if use_llm {
                        is_grounded_obligation_with(
                            &c.source_text,
                            &constraint_claim_text(c),
                            |s, claim| verify_entailment(provider, model, "", s, claim),
                        )
                    } else {
                        is_normative_for_subject(&c.source_text, &c.subject_signal)
                    }
                })
                .collect(),
        ),
        other => other,
    }
}

/// Build predictions by invoking `extractor` once per unique `(doc_key, task)` in `items`
/// and indexing the produced records by statement provenance. The extractor is injected so
/// the orchestration is testable without running a real command.
fn build_predictions<F>(items: &[EvalItem], mut extractor: F) -> Result<PredictedKeys>
where
    F: FnMut(&str, EvalTask) -> Result<TaskRecords>,
{
    let mut predicted = PredictedKeys::new();
    let mut seen: BTreeSet<(String, EvalTask)> = BTreeSet::new();
    for item in items {
        if !seen.insert((item.doc_key.clone(), item.task)) {
            continue;
        }
        match extractor(&item.doc_key, item.task)? {
            TaskRecords::Constraints(records) => {
                index_constraint_predictions(&records, &mut predicted)
            }
            TaskRecords::Relations(records) => index_relation_predictions(&records, &mut predicted),
            TaskRecords::TemporalRules(records) => {
                index_temporal_rule_predictions(&records, &mut predicted)
            }
            TaskRecords::SerialFrameFields(records) => {
                eval::index_serial_frame_field_predictions(&records, &mut predicted)
            }
            TaskRecords::ProtocolOperations(records) => {
                eval::index_protocol_operation_predictions(&records, &mut predicted)
            }
            TaskRecords::ProtocolStates(records) => {
                eval::index_protocol_state_predictions(&records, &mut predicted)
            }
            TaskRecords::InterfaceEdgeTimings(records) => {
                eval::index_interface_edge_timing_predictions(&records, &mut predicted)
            }
            TaskRecords::RegisterFields(records) => {
                eval::index_register_field_predictions(&records, &mut predicted)
            }
            TaskRecords::DeclaredSignals(records) => {
                eval::index_declared_signal_predictions(&records, &mut predicted)
            }
        }
    }
    Ok(predicted)
}

/// Run the real extraction command for `(doc_key, task)` on a TEMP COPY of the document's
/// EvidenceIR (so the corpus artifact is never mutated) and return the produced records.
fn extract_on_copy(
    evidence_root: &Path,
    doc_key: &str,
    task: EvalTask,
    provider: VlmProviderArg,
    model: Option<String>,
) -> Result<(TaskRecords, Vec<FactProvenanceRecord>)> {
    let source = evidence_root.join(doc_key).join("evidence_ir.json");
    let temp = crate::project_data::tempdir()?;
    let mut ir = EvidenceIr::load_from_path(&source)?;
    // Redirect all writes to the temp dir; the command writes there, not over the corpus.
    ir.artifact_layout.artifact_root = temp.path().to_path_buf();
    ir.artifact_layout.evidence_ir_path = temp.path().join("evidence_ir.json");
    ir.write_to_disk()?;
    let temp_path = ir.artifact_layout.evidence_ir_path.clone();

    match task {
        EvalTask::SignalConstraint => {
            crate::commands::nlp_enrich::run(NlpEnrichArgs {
                evidence_ir: temp_path.clone(),
                vlm_provider: provider,
                vlm_model: model,
                dry_run: false,
                max_sentences: 0,
                grounding_signals: None,
            })?;
            let enriched = EvidenceIr::load_from_path(&temp_path)?;
            Ok((
                TaskRecords::Constraints(enriched.signal_constraints),
                enriched.fact_provenance,
            ))
        }
        EvalTask::ActorSignalRelation => {
            crate::commands::signal_resolve::run(SignalResolveArgs {
                evidence_ir: temp_path.clone(),
                provider,
                model,
                dry_run: false,
                max_statements: 0,
                grounding_signals: None,
            })?;
            let enriched = EvidenceIr::load_from_path(&temp_path)?;
            Ok((
                TaskRecords::Relations(enriched.actor_signal_relations),
                enriched.fact_provenance,
            ))
        }
        EvalTask::TemporalRule => {
            // Temporal rules come from the deterministic EvidenceIR->SemanticIR lowering, not
            // an LLM command (provider/model are unused for this task). Build the SemanticIR
            // from the temp copy — all artifacts confined to the temp dir, corpus untouched —
            // and read its temporal_rules.
            let semantic = SemanticIr::build(&temp_path, temp.path())?;
            Ok((
                TaskRecords::TemporalRules(semantic.temporal_rules),
                Vec::new(),
            ))
        }
        // Protocol-structure surfaces are deterministic EvidenceIR records; read them directly.
        EvalTask::SerialFrameField => Ok((
            TaskRecords::SerialFrameFields(ir.serial_frame_fields),
            Vec::new(),
        )),
        EvalTask::ProtocolOperation => Ok((
            TaskRecords::ProtocolOperations(ir.protocol_operations),
            Vec::new(),
        )),
        EvalTask::ProtocolState => {
            Ok((TaskRecords::ProtocolStates(ir.protocol_states), Vec::new()))
        }
        EvalTask::InterfaceEdgeTiming => Ok((
            TaskRecords::InterfaceEdgeTimings(ir.interface_edge_timings),
            Vec::new(),
        )),
        // PDF-VARIANT-DIGESTION.4a.1 — register fields are deterministic table-synthesized EvidenceIR
        // records (no LLM / provider); read them straight from the (already-loaded) EvidenceIR.
        EvalTask::RegisterField => {
            Ok((TaskRecords::RegisterFields(ir.register_records), Vec::new()))
        }
        // PDF-VARIANT-DIGESTION.4a.4 — the canonical declared-signal inventory lives on the SemanticIR
        // (interfaces[].signal_records), built deterministically from the EvidenceIR (no LLM / provider).
        // Build it from the temp copy (artifacts confined to the temp dir, corpus untouched).
        EvalTask::DeclaredSignal => {
            let semantic = SemanticIr::build(&temp_path, temp.path())?;
            let signals: Vec<InterfaceSignalRecord> = semantic
                .interfaces
                .iter()
                .flat_map(|interface| interface.signal_records.iter().cloned())
                .collect();
            Ok((TaskRecords::DeclaredSignals(signals), Vec::new()))
        }
    }
}

/// Per produced record: `(eval_key, tier_count, statement_ids)`. `tier_count` = the number of
/// DISTINCT extractor tiers (`Pattern`/`Nlp`/`Vlm`) whose `fact_provenance` recorded this fact —
/// a real agreement-confidence axis (a fact found by two tiers is more trustworthy than one). The
/// `eval_key` matches the gold's canonical key; the provenance key is a different format, so it is
/// recomputed per record from the same record. Temporal rules carry no tier provenance.
fn records_with_tier_counts(
    records: &TaskRecords,
    provenance: &[FactProvenanceRecord],
) -> Vec<(EvalTask, String, usize, Vec<String>)> {
    // tier presence (Pattern/Nlp/Vlm) per provenance key (ExtractorTier is not Hash → index it).
    let tier_idx = |t: &ExtractorTier| match t {
        ExtractorTier::Pattern => 0,
        ExtractorTier::Nlp => 1,
        ExtractorTier::Vlm => 2,
    };
    let mut seen: std::collections::HashMap<String, [bool; 3]> = std::collections::HashMap::new();
    for p in provenance {
        seen.entry(p.canonical_key.clone()).or_default()[tier_idx(&p.producer)] = true;
    }
    let count = |provkey: &str| {
        seen.get(provkey)
            .map(|flags| flags.iter().filter(|f| **f).count())
            .unwrap_or(1)
            .max(1)
    };
    match records {
        TaskRecords::Constraints(cs) => cs
            .iter()
            .map(|c| {
                (
                    EvalTask::SignalConstraint,
                    eval::signal_constraint_record_key(c),
                    count(&signal_constraint_fact_key(c)),
                    c.supporting_statement_ids.clone(),
                )
            })
            .collect(),
        TaskRecords::Relations(rs) => rs
            .iter()
            .map(|r| {
                (
                    EvalTask::ActorSignalRelation,
                    eval::actor_signal_relation_record_key(r),
                    count(&actor_signal_relation_fact_key(r)),
                    r.source_statement_ids.clone(),
                )
            })
            .collect(),
        TaskRecords::TemporalRules(_) => Vec::new(),
        // Structural surfaces are single-tier deterministic records; no multi-tier provenance.
        TaskRecords::SerialFrameFields(_)
        | TaskRecords::ProtocolOperations(_)
        | TaskRecords::ProtocolStates(_)
        | TaskRecords::InterfaceEdgeTimings(_)
        | TaskRecords::RegisterFields(_)
        | TaskRecords::DeclaredSignals(_) => Vec::new(),
    }
}

/// Render a human-readable per-task report.
fn format_report(
    scores: &std::collections::BTreeMap<EvalTask, Scorecard>,
    provider: VlmProviderArg,
    model: &str,
) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "=== Extraction eval (provider: {}, model: {}) ===\n",
        crate::llm_text::provider_name(provider),
        model
    ));
    for (task, card) in scores {
        out.push_str(&format!(
            "  {:<22} P={:.3} R={:.3} F1={:.3}  (tp={} fp={} fn={}; gold={} over {} statements)\n",
            task.as_str(),
            card.precision(),
            card.recall(),
            card.f1(),
            card.tp,
            card.fp,
            card.fn_count,
            card.gold_total,
            card.labeled_statements,
        ));
    }
    if scores.is_empty() {
        out.push_str("  (no labeled items)\n");
    }
    out
}

pub fn run(args: EvalExtractionArgs) -> Result<()> {
    let loaded = eval::load_eval_dataset(&args.dataset)?;
    let provider = args.provider;
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| crate::llm_text::default_model(provider));
    let evidence_root = args.evidence_root.clone();

    // WIRE-BASED-100.1 — content-anchored scoring: re-resolve each gold item's statement_id to the
    // CURRENT evidence by matching its input_text (re-ingest drifts statement ids). The expected facts
    // are unchanged; a label whose sentence is genuinely absent stays unresolved → a real miss.
    let mut statements: Vec<(String, String)> = Vec::new();
    for dk in loaded
        .iter()
        .map(|i| i.doc_key.clone())
        .collect::<BTreeSet<_>>()
    {
        if let Ok(ir) =
            EvidenceIr::load_from_path(&evidence_root.join(&dk).join("evidence_ir.json"))
        {
            statements.extend(
                ir.extracted_statements
                    .iter()
                    .map(|s| (s.statement_id.clone(), s.text.clone())),
            );
        }
    }
    let realigned = eval::realign_gold_statement_ids(&loaded, &statements, 0.7);
    let moved = realigned
        .iter()
        .zip(&loaded)
        .filter(|(a, b)| a.statement_id != b.statement_id)
        .count();
    let items = realigned;
    println!(
        "content-anchored: re-resolved {moved}/{} gold statement ids to current evidence",
        items.len()
    );

    println!("command: eval-extraction");
    println!(
        "dataset: {} ({} items)",
        args.dataset.display(),
        items.len()
    );
    println!("evidence_root: {}", evidence_root.display());
    println!(
        "provider: {} | model: {}",
        crate::llm_text::provider_name(provider),
        model
    );
    if matches!(provider, VlmProviderArg::Skip) {
        println!("note: --provider skip => deterministic pattern baseline (no LLM calls)");
    }

    // Stash each (doc, task)'s records + fact_provenance during the scoring pass, so conformal
    // calibration reuses the same extraction (no second LLM run).
    let mut conformal_input: Vec<(TaskRecords, Vec<FactProvenanceRecord>)> = Vec::new();
    let predicted = build_predictions(&items, |doc_key, task| {
        let (records, provenance) =
            extract_on_copy(&evidence_root, doc_key, task, provider, args.model.clone())?;
        // WIRE-BASED-100.6/.7(c) — drop over-generated facts (garbage actors, descriptive
        // hallucinations); automatic via the LLM when a provider is available, else heuristic.
        let records = filter_overgenerated(records, provider, &model);
        conformal_input.push((records.clone(), provenance));
        Ok(records)
    })?;

    let scores = eval::score_dataset(&items, &predicted);
    print!("{}", format_report(&scores, provider, &model));

    // WIRE-BASED-100 — source-tolerant + filtered scorecard: the principled per-fact view (recall
    // credits any valid source; precision strict after the .6/.7 filters).
    let st = eval::score_dataset_source_tolerant(&items, &predicted);
    if !st.is_empty() {
        println!("  -- source-tolerant + filtered (WIRE-BASED-100) --");
        for (task, sc) in &st {
            println!(
                "    {:<22} P={:.3} R={:.3} F1={:.3}  (tp={} fp={} fn={})",
                task.as_str(),
                sc.precision(),
                sc.recall(),
                sc.f1(),
                sc.tp,
                sc.fp,
                sc.fn_count
            );
        }
    }

    // PDF-VARIANT-DIGESTION.4a.2 — register-field "measure & surface" view: the strict per-fact key
    // (register|field|offset|width) is uninformative on docs whose register-field extraction recovers
    // field NAMES but not the owning register name (synthetic) or bit extent. So report field-NAME recall
    // (the part that works) SEPARATELY from the two completeness gaps, honestly, instead of one degenerate
    // ~0 headline. Register records are deterministic, so they are the same across providers.
    if items.iter().any(|i| i.task == EvalTask::RegisterField) {
        let mut reg_records: Vec<RegisterRecord> = Vec::new();
        for (records, _) in &conformal_input {
            if let TaskRecords::RegisterFields(rs) = records {
                reg_records.extend(rs.iter().cloned());
            }
        }
        let field_names: BTreeSet<String> = reg_records
            .iter()
            .flat_map(|r| r.fields.iter())
            .map(|f| f.field_name.trim().to_ascii_uppercase())
            .collect();
        let (found, total) = eval::register_field_name_recall(&items, &field_names);
        let (bits_found, bits_total) = eval::register_bit_structure_recall(&items, &reg_records);
        let (named, total_regs, with_bits, total_fields) =
            eval::register_field_completeness(&reg_records);
        let name_recall = if total > 0 {
            found as f64 / total as f64
        } else {
            0.0
        };
        let bit_recall = if bits_total > 0 {
            bits_found as f64 / bits_total as f64
        } else {
            0.0
        };
        println!(
            "  -- register-field surface (measure & surface; PDF-VARIANT-DIGESTION.4a.2/.4a.3) --"
        );
        println!("    field-name recall (register-agnostic)   {found}/{total} = {name_recall:.3}");
        println!(
            "    bit-structure recall (register-scoped)  {bits_found}/{bits_total} = {bit_recall:.3}"
        );
        println!(
            "    register-name association gap           {named}/{total_regs} extracted registers have a real (non-synthetic) name"
        );
        println!(
            "    bit-extent completeness gap             {with_bits}/{total_fields} extracted fields carry a bit position"
        );
    }

    // PDF-VARIANT-DIGESTION.4a.5 — declared-signal document-level precision. The statement-scoped scorer
    // can't see over-captures (a spurious signal is attributed to its own synthesized statement, not the
    // labeled one), so when the gold ENUMERATES every true signal (a small fully-specified bus like I2C)
    // report precision over the produced signal set and name the false positives. Recall is already
    // surfaced by the document-level fact-recall section below.
    if items.iter().any(|i| i.task == EvalTask::DeclaredSignal) {
        let predicted_names: BTreeSet<String> = conformal_input
            .iter()
            .filter_map(|(records, _)| match records {
                TaskRecords::DeclaredSignals(rs) => Some(rs),
                _ => None,
            })
            .flatten()
            .map(|r| r.signal_name.trim().to_ascii_uppercase())
            .collect();
        let (matched, predicted_total, false_positives) =
            eval::declared_signal_complete_gold_precision(&items, &predicted_names);
        let precision = if predicted_total > 0 {
            matched as f64 / predicted_total as f64
        } else {
            0.0
        };
        println!(
            "  -- declared-signal surface (complete-gold precision; PDF-VARIANT-DIGESTION.4a.5) --"
        );
        println!(
            "    precision (assumes gold enumerates all signals)  {matched}/{predicted_total} = {precision:.3}"
        );
        if !false_positives.is_empty() {
            println!(
                "    false positives ({}): {}",
                false_positives.len(),
                false_positives.join(", ")
            );
        }
    }

    // Per-relation-kind breakdown + MUC near-misses (relation task only).
    let by_kind = eval::score_relations_by_kind(&items, &predicted);
    if !by_kind.is_empty() {
        println!("  -- relations by kind --");
        for (kind, card) in &by_kind {
            println!(
                "    {:<8} P={:.3} R={:.3} F1={:.3}  (tp={} fp={} fn={})",
                kind,
                card.precision(),
                card.recall(),
                card.f1(),
                card.tp,
                card.fp,
                card.fn_count,
            );
        }
        let nm = eval::relation_near_misses(&items, &predicted);
        println!(
            "    near-miss  wrong_direction={} wrong_actor={}",
            nm.wrong_direction, nm.wrong_actor
        );
    }

    // Document-level fact recall (attribution-agnostic) — a gold fact found on ANY statement
    // counts, revealing recall the per-statement closed-world scorer hides when the extractor
    // attributes a fact to a different sentence than the gold.
    let recall = eval::score_fact_recall(&items, &predicted);
    if !recall.is_empty() {
        println!("  -- document-level fact recall (attribution-agnostic) --");
        for (task, (found, total)) in &recall {
            let r = if *total > 0 {
                *found as f64 / *total as f64
            } else {
                0.0
            };
            println!(
                "    {:<22} recall={r:.3}  ({found}/{total} gold facts found anywhere)",
                task.as_str()
            );
        }
        // Pinpoint the genuinely-missed gold facts (document-level) for review.
        let missed = eval::missed_gold_facts(&items, &predicted);
        for (task, keys) in &missed {
            if !keys.is_empty() {
                println!(
                    "    MISSED {} ({}): {}",
                    task.as_str(),
                    keys.len(),
                    keys.join("  ;  ")
                );
            }
        }
    }

    // Split-conformal calibration — confidence axis = extractor-tier agreement (a fact found by
    // more tiers is more trustworthy), label = whether the predicted fact is in gold. Reuses the
    // labeled eval set (no new gold); the accept threshold is calibrated at a target error.
    let gold_by_item: std::collections::BTreeMap<(EvalTask, String), BTreeSet<String>> = items
        .iter()
        .map(|i| {
            (
                (i.task, i.statement_id.clone()),
                i.gold.iter().map(|g| g.canonical_key()).collect(),
            )
        })
        .collect();
    let mut samples_by_task: std::collections::BTreeMap<EvalTask, Vec<(f64, bool)>> =
        std::collections::BTreeMap::new();
    for (records, provenance) in &conformal_input {
        for (task, eval_key, tier_count, stmt_ids) in records_with_tier_counts(records, provenance)
        {
            for sid in &stmt_ids {
                let key = (task, sid.clone());
                if let Some(gold) = gold_by_item.get(&key) {
                    samples_by_task
                        .entry(task)
                        .or_default()
                        .push((tier_count as f64, gold.contains(&eval_key)));
                }
            }
        }
    }
    if !samples_by_task.is_empty() {
        let alpha = 0.2;
        println!("  -- split-conformal accept threshold (axis: tier-agreement; alpha={alpha}) --");
        for (task, samples) in &samples_by_task {
            match eval::conformal_threshold(samples, alpha) {
                Some(t) => println!(
                    "    {:<22} threshold={:.2}  coverage={:.3}  empirical_error={:.3}  (n={})",
                    task.as_str(),
                    t.threshold,
                    t.coverage,
                    t.empirical_error,
                    samples.len()
                ),
                None => println!(
                    "    {:<22} no threshold meets alpha={alpha}  (n={} — too few/noisy)",
                    task.as_str(),
                    samples.len()
                ),
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::eval::GoldFact;
    use crate::ir::source::{AutomationConfidence, RelationKind, SignalConstraintKind};

    fn relation_item(statement_id: &str, actor: &str, relation: &str, signal: &str) -> EvalItem {
        EvalItem {
            task: EvalTask::ActorSignalRelation,
            doc_key: "doc".to_string(),
            statement_id: statement_id.to_string(),
            input_text: String::new(),
            grounding: vec![],
            gold: vec![GoldFact::Relation {
                actor: actor.to_string(),
                relation: relation.to_string(),
                signal: signal.to_string(),
            }],
            label_status: "agent_drafted".to_string(),
            label_note: String::new(),
        }
    }

    fn relation_record(
        actor: &str,
        relation: RelationKind,
        signal: &str,
        sid: &str,
    ) -> ActorSignalRelation {
        ActorSignalRelation {
            relation_id: "r".to_string(),
            actor_name: actor.to_string(),
            signal_name: signal.to_string(),
            relation,
            source_statement_ids: vec![sid.to_string()],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    fn constraint_item(statement_id: &str, subject: &str, kind: &str) -> EvalItem {
        EvalItem {
            task: EvalTask::SignalConstraint,
            doc_key: "doc".to_string(),
            statement_id: statement_id.to_string(),
            input_text: String::new(),
            grounding: vec![],
            gold: vec![GoldFact::Constraint {
                subject_signal: subject.to_string(),
                constraint_kind: kind.to_string(),
                negated: false,
                target_value: None,
            }],
            label_status: "agent_drafted".to_string(),
            label_note: String::new(),
        }
    }

    fn constraint_record(
        subject: &str,
        kind: SignalConstraintKind,
        sid: &str,
    ) -> SignalConstraintRecord {
        SignalConstraintRecord {
            constraint_id: "c".to_string(),
            subject_signal: subject.to_string(),
            constraint_kind: kind,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: String::new(),
            supporting_statement_ids: vec![sid.to_string()],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn build_predictions_runs_extractor_once_per_doc_task_and_scores() {
        let items = vec![
            relation_item("s1", "Manager", "drives", "HTRANS"),
            // second relation item, same doc+task => extractor must NOT be called again
            relation_item("s2", "Manager", "reads", "HREADY"),
            constraint_item("s3", "HADDR", "must_be_stable"),
        ];

        let mut relation_calls = 0;
        let mut constraint_calls = 0;
        let predicted = build_predictions(&items, |_doc, task| match task {
            EvalTask::ActorSignalRelation => {
                relation_calls += 1;
                // model "found" s1's edge (TP) but missed s2's; also a spurious one on s1 (FP)
                Ok(TaskRecords::Relations(vec![
                    relation_record("Manager", RelationKind::Drives, "HTRANS", "s1"),
                    relation_record("Manager", RelationKind::Drives, "HWDATA", "s1"),
                ]))
            }
            EvalTask::SignalConstraint => {
                constraint_calls += 1;
                Ok(TaskRecords::Constraints(vec![constraint_record(
                    "HADDR",
                    SignalConstraintKind::MustBeStable,
                    "s3",
                )]))
            }
            EvalTask::TemporalRule
            | EvalTask::SerialFrameField
            | EvalTask::ProtocolOperation
            | EvalTask::ProtocolState
            | EvalTask::InterfaceEdgeTiming
            | EvalTask::RegisterField
            | EvalTask::DeclaredSignal => {
                unreachable!("no structural/register-field/declared-signal items in this test")
            }
        })
        .unwrap();

        assert_eq!(relation_calls, 1, "extractor invoked once per (doc, task)");
        assert_eq!(constraint_calls, 1);

        let scores = eval::score_dataset(&items, &predicted);
        let rel = &scores[&EvalTask::ActorSignalRelation];
        assert_eq!(rel.tp, 1); // Manager drives HTRANS
        assert_eq!(rel.fn_count, 1); // Manager reads HREADY missed
        assert_eq!(rel.fp, 1); // Manager drives HWDATA spurious on s1
        let con = &scores[&EvalTask::SignalConstraint];
        assert_eq!(con.tp, 1);
        assert_eq!(con.fp, 0);
        assert_eq!(con.fn_count, 0);
    }

    #[test]
    fn build_predictions_indexes_temporal_rules() {
        use crate::eval::GoldFact;
        use crate::ir::semantic::{ClockEdge, TemporalPredicateRecord, TickPhase};
        let consequent = TemporalPredicateRecord::SignalValue {
            signal_name: "PADDR".to_string(),
            value: "VALID".to_string(),
            phase: TickPhase::PostTick,
        };
        let item = EvalItem {
            task: EvalTask::TemporalRule,
            doc_key: "apb".to_string(),
            statement_id: "s1".to_string(),
            input_text: String::new(),
            grounding: vec![],
            gold: vec![GoldFact::TemporalRule {
                edge: ClockEdge::Rising,
                antecedents: vec![],
                consequents: vec![consequent.clone()],
                cycle_window: None,
            }],
            label_status: "agent_drafted".to_string(),
            label_note: String::new(),
        };
        // The produced record carries a clock_signal (PCLK) + provenance the key must ignore.
        let record = TemporalRuleRecord {
            rule_id: "t".to_string(),
            clock_signal: Some("PCLK".to_string()),
            edge: ClockEdge::Rising,
            antecedents: vec![],
            consequents: vec![consequent],
            cycle_window: None,
            source_text: "x".to_string(),
            supporting_statement_ids: vec!["s1".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        };
        let items = [item];
        let predicted = build_predictions(&items, |_doc, task| {
            assert_eq!(task, EvalTask::TemporalRule);
            Ok(TaskRecords::TemporalRules(vec![record.clone()]))
        })
        .unwrap();
        let scores = eval::score_dataset(&items, &predicted);
        let card = &scores[&EvalTask::TemporalRule];
        assert_eq!(
            card.tp, 1,
            "gold rule matched (clock_signal + provenance ignored by the key)"
        );
        assert_eq!(card.fp, 0);
        assert_eq!(card.fn_count, 0);
    }

    #[test]
    fn format_report_renders_per_task_metrics() {
        let items = vec![relation_item("s1", "Manager", "drives", "HTRANS")];
        let predicted = build_predictions(&items, |_doc, _task| {
            Ok(TaskRecords::Relations(vec![relation_record(
                "Manager",
                RelationKind::Drives,
                "HTRANS",
                "s1",
            )]))
        })
        .unwrap();
        let scores = eval::score_dataset(&items, &predicted);
        let report = format_report(&scores, VlmProviderArg::Skip, "qwen2.5vl:7b");
        assert!(report.contains("actor_signal_relation"));
        assert!(report.contains("P=1.000"));
        assert!(report.contains("R=1.000"));
        assert!(report.contains("F1=1.000"));
    }

    #[test]
    fn records_with_tier_counts_counts_distinct_provenance_tiers() {
        use crate::ir::evidence::{
            ExtractorTier, FactKind, FactProvenanceRecord, signal_constraint_fact_key,
        };
        use crate::ir::source::SignalConstraintRecord;
        let mk = |sig: &str| SignalConstraintRecord {
            constraint_id: "c".to_string(),
            subject_signal: sig.to_string(),
            constraint_kind: SignalConstraintKind::MustBeStable,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: String::new(),
            supporting_statement_ids: vec!["s1".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        };
        let both = mk("PSEL"); // found by Pattern AND Nlp → tier 2
        let one = mk("PADDR"); // found by Pattern only → tier 1
        let prov = |t: ExtractorTier, c: &SignalConstraintRecord| FactProvenanceRecord {
            producer: t,
            fact_kind: FactKind::SignalConstraint,
            canonical_key: signal_constraint_fact_key(c),
        };
        let provenance = vec![
            prov(ExtractorTier::Pattern, &both),
            prov(ExtractorTier::Nlp, &both),
            prov(ExtractorTier::Pattern, &one),
        ];
        let recs = TaskRecords::Constraints(vec![both, one]);
        let out = records_with_tier_counts(&recs, &provenance);
        let tier = |needle: &str| {
            out.iter()
                .find(|(_, k, _, _)| k.contains(needle))
                .unwrap()
                .2
        };
        assert_eq!(tier("PSEL"), 2, "found by two tiers");
        assert_eq!(tier("PADDR"), 1, "found by one tier");
    }
}
