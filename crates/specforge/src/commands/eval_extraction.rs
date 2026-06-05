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
use crate::ir::evidence::EvidenceIr;
use crate::ir::semantic::{SemanticIr, TemporalRuleRecord};
use crate::ir::source::{ActorSignalRelation, SignalConstraintRecord};
use std::collections::BTreeSet;
use std::path::Path;

/// The typed records produced for one `(doc, task)`. The two LLM tasks run the real
/// extraction command; the temporal task builds the SemanticIR (deterministic parser).
enum TaskRecords {
    Constraints(Vec<SignalConstraintRecord>),
    Relations(Vec<ActorSignalRelation>),
    TemporalRules(Vec<TemporalRuleRecord>),
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
) -> Result<TaskRecords> {
    let source = evidence_root.join(doc_key).join("evidence_ir.json");
    let temp = tempfile::tempdir()?;
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
            Ok(TaskRecords::Constraints(enriched.signal_constraints))
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
            Ok(TaskRecords::Relations(enriched.actor_signal_relations))
        }
        EvalTask::TemporalRule => {
            // Temporal rules come from the deterministic EvidenceIR->SemanticIR lowering, not
            // an LLM command (provider/model are unused for this task). Build the SemanticIR
            // from the temp copy — all artifacts confined to the temp dir, corpus untouched —
            // and read its temporal_rules.
            let semantic = SemanticIr::build(&temp_path, temp.path())?;
            Ok(TaskRecords::TemporalRules(semantic.temporal_rules))
        }
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
        crate::commands::llm_text::provider_name(provider),
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
    let items = eval::load_eval_dataset(&args.dataset)?;
    let provider = args.provider;
    let model = args
        .model
        .clone()
        .unwrap_or_else(|| crate::commands::llm_text::default_model(provider));
    let evidence_root = args.evidence_root.clone();

    println!("command: eval-extraction");
    println!(
        "dataset: {} ({} items)",
        args.dataset.display(),
        items.len()
    );
    println!("evidence_root: {}", evidence_root.display());
    println!(
        "provider: {} | model: {}",
        crate::commands::llm_text::provider_name(provider),
        model
    );
    if matches!(provider, VlmProviderArg::Skip) {
        println!("note: --provider skip => deterministic pattern baseline (no LLM calls)");
    }

    let predicted = build_predictions(&items, |doc_key, task| {
        extract_on_copy(&evidence_root, doc_key, task, provider, args.model.clone())
    })?;

    let scores = eval::score_dataset(&items, &predicted);
    print!("{}", format_report(&scores, provider, &model));

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
            EvalTask::TemporalRule => unreachable!("no temporal_rule items in this test"),
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
}
