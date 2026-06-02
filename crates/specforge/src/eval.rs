//! Labeled precision/recall/F1 evaluation for the LLM extraction passes.
//!
//! This is the **pure scorer + dataset format** of the `LLM-EXTRACTION-EVAL` task-tree
//! (`docs/tasks/LLM-EXTRACTION-EVAL.md`). It is provider-free and fully unit-testable:
//! a runner (a separate command) executes the real extraction command with a chosen
//! model, converts the produced typed records into predicted canonical keys grouped by
//! `(task, statement_id)`, and hands them here to be scored against gold labels.
//!
//! Scoring is **closed-world over the labeled statements only**: for each labeled
//! statement, the model's predicted keys for that statement are compared to gold; keys
//! for unlabeled statements are never consulted (so an un-labeled-but-correct extraction
//! is not penalized). This requires the gold to be *complete per labeled statement*.
//!
//! Covers the extraction surfaces with crisp canonical keys:
//! - `nlp-enrich`     → [`SignalConstraintRecord`]
//! - `signal-resolve` → [`ActorSignalRelation`]
//! - temporal parser  → [`TemporalRuleRecord`] (the deterministic EvidenceIR→SemanticIR
//!   lowering; identity is the rule's logical content, provenance-free)

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::semantic::{
    ClockEdge, CycleWindowRecord, TemporalPredicateRecord, TemporalRuleRecord, TickPhase,
};
use crate::ir::source::{
    ActorSignalRelation, RelationKind, SignalConstraintKind, SignalConstraintRecord,
};

/// The LLM extraction tasks this eval covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvalTask {
    /// `nlp-enrich` → `SignalConstraintRecord`.
    SignalConstraint,
    /// `signal-resolve` → `ActorSignalRelation`.
    ActorSignalRelation,
    /// The deterministic temporal parser → `TemporalRuleRecord` (mined temporal rules).
    TemporalRule,
}

impl EvalTask {
    /// Stable label for reports/metrics.
    pub fn as_str(self) -> &'static str {
        match self {
            EvalTask::SignalConstraint => "signal_constraint",
            EvalTask::ActorSignalRelation => "actor_signal_relation",
            EvalTask::TemporalRule => "temporal_rule",
        }
    }
}

/// One gold typed output a correct extraction must produce for a labeled statement.
///
/// The `fact` tag selects the variant; each variant's task is recoverable via
/// [`GoldFact::task`]. An item with an empty `gold` list is a **negative** item (the
/// correct extraction for that statement is *nothing*).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "fact", rename_all = "snake_case")]
pub enum GoldFact {
    /// A signal constraint (`nlp-enrich`).
    Constraint {
        subject_signal: String,
        /// Snake-case constraint kind — see [`constraint_kind_str`]
        /// (`must_be_high`, `must_be_stable`, `must_be_value`, …).
        constraint_kind: String,
        #[serde(default)]
        negated: bool,
        #[serde(default)]
        target_value: Option<String>,
    },
    /// An actor→signal relation (`signal-resolve`).
    Relation {
        actor: String,
        /// `drives` or `reads`.
        relation: String,
        signal: String,
    },
    /// A mined temporal rule (the deterministic temporal parser). Authored with the *same*
    /// predicate shapes the IR uses (`TemporalPredicateRecord`), so the canonical key is
    /// computed identically on gold and on produced records. Identity is the rule's logical
    /// content — `rule_id`/`source_text`/provenance/confidence are deliberately not part of
    /// the gold (see [`GoldFact::canonical_key`]).
    TemporalRule {
        edge: ClockEdge,
        #[serde(default)]
        antecedents: Vec<TemporalPredicateRecord>,
        #[serde(default)]
        consequents: Vec<TemporalPredicateRecord>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        cycle_window: Option<CycleWindowRecord>,
    },
}

impl GoldFact {
    /// Which task this gold fact belongs to.
    pub fn task(&self) -> EvalTask {
        match self {
            GoldFact::Constraint { .. } => EvalTask::SignalConstraint,
            GoldFact::Relation { .. } => EvalTask::ActorSignalRelation,
            GoldFact::TemporalRule { .. } => EvalTask::TemporalRule,
        }
    }

    /// Canonical key — must match the key computed from a *produced* record so that set
    /// overlap measures agreement. Signals/actors are uppercased; kind/relation lowered.
    pub fn canonical_key(&self) -> String {
        match self {
            GoldFact::Constraint {
                subject_signal,
                constraint_kind,
                negated,
                target_value,
            } => constraint_key(
                subject_signal,
                constraint_kind,
                *negated,
                target_value.as_deref(),
            ),
            GoldFact::Relation {
                actor,
                relation,
                signal,
            } => relation_key(actor, relation, signal),
            GoldFact::TemporalRule {
                edge,
                antecedents,
                consequents,
                cycle_window,
            } => temporal_rule_key(*edge, antecedents, consequents, cycle_window.as_ref()),
        }
    }
}

/// Stable snake-case string for a [`SignalConstraintKind`] (the per-value payload of
/// `MustBeValue` is carried separately in the key's value slot).
pub fn constraint_kind_str(kind: &SignalConstraintKind) -> &'static str {
    match kind {
        SignalConstraintKind::MustBeHigh => "must_be_high",
        SignalConstraintKind::MustBeLow => "must_be_low",
        SignalConstraintKind::MustBeAsserted => "must_be_asserted",
        SignalConstraintKind::MustBeDeasserted => "must_be_deasserted",
        SignalConstraintKind::MustNotChange => "must_not_change",
        SignalConstraintKind::MustBeStable => "must_be_stable",
        SignalConstraintKind::MustHoldData => "must_hold_data",
        SignalConstraintKind::MustBeValue { .. } => "must_be_value",
    }
}

/// Stable snake-case string for a [`RelationKind`].
pub fn relation_kind_str(relation: &RelationKind) -> &'static str {
    match relation {
        RelationKind::Drives => "drives",
        RelationKind::Reads => "reads",
    }
}

fn constraint_key(subject: &str, kind: &str, negated: bool, target: Option<&str>) -> String {
    format!(
        "{}|{}|{}|{}",
        subject.trim().to_ascii_uppercase(),
        kind.trim().to_ascii_lowercase(),
        if negated { "neg" } else { "pos" },
        target.unwrap_or("").trim().to_ascii_uppercase(),
    )
}

fn relation_key(actor: &str, relation: &str, signal: &str) -> String {
    format!(
        "{}|{}|{}",
        actor.trim().to_ascii_uppercase(),
        relation.trim().to_ascii_lowercase(),
        signal.trim().to_ascii_uppercase(),
    )
}

/// Canonical key for a produced [`SignalConstraintRecord`] — matches a gold
/// `Constraint`'s [`GoldFact::canonical_key`]. For `MustBeValue` the variant's value is
/// the target; otherwise the record's `target_value` field is used.
pub fn signal_constraint_record_key(record: &SignalConstraintRecord) -> String {
    let target = match &record.constraint_kind {
        SignalConstraintKind::MustBeValue { value } => Some(value.as_str()),
        _ => record.target_value.as_deref(),
    };
    constraint_key(
        &record.subject_signal,
        constraint_kind_str(&record.constraint_kind),
        record.negated,
        target,
    )
}

/// Canonical key for a produced [`ActorSignalRelation`] — matches a gold `Relation`'s key.
pub fn actor_signal_relation_record_key(record: &ActorSignalRelation) -> String {
    relation_key(
        &record.actor_name,
        relation_kind_str(&record.relation),
        &record.signal_name,
    )
}

/// Stable snake-case string for a [`TickPhase`].
fn tick_phase_str(phase: TickPhase) -> &'static str {
    match phase {
        TickPhase::PreTick => "pre_tick",
        TickPhase::PostTick => "post_tick",
    }
}

/// Stable snake-case string for a [`ClockEdge`].
fn clock_edge_str(edge: ClockEdge) -> &'static str {
    match edge {
        ClockEdge::Rising => "rising",
        ClockEdge::Falling => "falling",
        ClockEdge::Unknown => "unknown",
    }
}

/// Normalized, order-stable key for a single temporal predicate. Signals/actors/values are
/// uppercased; phases are their snake_case strings — the same normalization the constraint
/// and relation keys use. The leading tag keeps different predicate kinds distinct.
fn temporal_predicate_key(pred: &TemporalPredicateRecord) -> String {
    let up = |s: &str| s.trim().to_ascii_uppercase();
    match pred {
        TemporalPredicateRecord::SignalValue {
            signal_name,
            value,
            phase,
        } => format!(
            "sv|{}|{}|{}",
            up(signal_name),
            up(value),
            tick_phase_str(*phase)
        ),
        TemporalPredicateRecord::ActorDrivesSignal {
            actor_name,
            signal_name,
            phase,
        } => format!(
            "ads|{}|{}|{}",
            up(actor_name),
            up(signal_name),
            tick_phase_str(*phase)
        ),
        TemporalPredicateRecord::ActorMaintainsSignalStable {
            actor_name,
            signal_name,
            from_phase,
            to_phase,
        } => format!(
            "amss|{}|{}|{}|{}",
            up(actor_name),
            up(signal_name),
            tick_phase_str(*from_phase),
            tick_phase_str(*to_phase)
        ),
        TemporalPredicateRecord::SignalStable {
            signal_name,
            from_phase,
            to_phase,
        } => format!(
            "ss|{}|{}|{}",
            up(signal_name),
            tick_phase_str(*from_phase),
            tick_phase_str(*to_phase)
        ),
        TemporalPredicateRecord::ActorSamplesSignal {
            actor_name,
            signal_name,
            phase,
        } => format!(
            "asm|{}|{}|{}",
            up(actor_name),
            up(signal_name),
            tick_phase_str(*phase)
        ),
        TemporalPredicateRecord::SignalSampled { signal_name, phase } => {
            format!("ssm|{}|{}", up(signal_name), tick_phase_str(*phase))
        }
        TemporalPredicateRecord::HandshakeComplete {
            valid_signal,
            ready_signal,
            phase,
        } => format!(
            "hc|{}|{}|{}",
            up(valid_signal),
            up(ready_signal),
            tick_phase_str(*phase)
        ),
    }
}

/// Canonical, **provenance-free** key for a temporal rule: clock edge + the *sorted*
/// antecedent and consequent predicate keys + the cycle window. Sorting makes the
/// conjunction order-insensitive; `rule_id`, `source_text`, `supporting_statement_ids`, and
/// `automation_confidence` are deliberately excluded — they are provenance, not identity.
fn temporal_rule_key(
    edge: ClockEdge,
    antecedents: &[TemporalPredicateRecord],
    consequents: &[TemporalPredicateRecord],
    cycle_window: Option<&CycleWindowRecord>,
) -> String {
    let mut ant: Vec<String> = antecedents.iter().map(temporal_predicate_key).collect();
    ant.sort();
    let mut cons: Vec<String> = consequents.iter().map(temporal_predicate_key).collect();
    cons.sort();
    let window = match cycle_window {
        Some(w) => format!(
            "{}..{}",
            w.min_cycles.map(|n| n.to_string()).unwrap_or_default(),
            w.max_cycles.map(|n| n.to_string()).unwrap_or_default(),
        ),
        None => String::new(),
    };
    format!(
        "{}|A:{}|C:{}|W:{}",
        clock_edge_str(edge),
        ant.join(","),
        cons.join(","),
        window,
    )
}

/// Canonical key for a produced [`TemporalRuleRecord`] — matches a gold `TemporalRule`'s key.
pub fn temporal_rule_record_key(record: &TemporalRuleRecord) -> String {
    temporal_rule_key(
        record.edge,
        &record.antecedents,
        &record.consequents,
        record.cycle_window.as_ref(),
    )
}

/// One labeled eval item: the gold typed outputs a correct extraction must produce for a
/// single statement of a single document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvalItem {
    /// Which extraction task this item evaluates.
    pub task: EvalTask,
    /// The document key whose `EvidenceIR` the runner will process.
    pub doc_key: String,
    /// The statement id within that document (matched against record provenance).
    pub statement_id: String,
    /// The statement text (for human review of the label).
    #[serde(default)]
    pub input_text: String,
    /// Declared-signal context the runner may pass to ground the LLM prompt.
    #[serde(default)]
    pub grounding: Vec<String>,
    /// Gold typed outputs (possibly empty = a negative item).
    pub gold: Vec<GoldFact>,
    /// `agent_drafted` (pending human review) or `human_reviewed`.
    #[serde(default)]
    pub label_status: String,
    /// Free-text note explaining the label.
    #[serde(default)]
    pub label_note: String,
}

impl EvalItem {
    /// Every gold fact must belong to the item's declared task.
    pub fn validate(&self) -> Result<()> {
        for fact in &self.gold {
            if fact.task() != self.task {
                return Err(AppError::InvalidStageArtifact(format!(
                    "eval item {}/{}: gold fact is a {:?} but item.task is {:?}",
                    self.doc_key,
                    self.statement_id,
                    fact.task(),
                    self.task,
                )));
            }
        }
        Ok(())
    }
}

/// Predicted canonical keys, grouped by `(task, statement_id)`. Built by the runner from
/// produced records (see [`index_constraint_predictions`] / [`index_relation_predictions`]).
pub type PredictedKeys = BTreeMap<(EvalTask, String), BTreeSet<String>>;

/// Add a batch of produced signal constraints to `into`, attributing each record's key to
/// every statement that supports it.
pub fn index_constraint_predictions(records: &[SignalConstraintRecord], into: &mut PredictedKeys) {
    for record in records {
        let key = signal_constraint_record_key(record);
        for statement_id in &record.supporting_statement_ids {
            into.entry((EvalTask::SignalConstraint, statement_id.clone()))
                .or_default()
                .insert(key.clone());
        }
    }
}

/// Add a batch of produced actor-signal relations to `into`, attributing each record's key
/// to every statement that supports it.
pub fn index_relation_predictions(records: &[ActorSignalRelation], into: &mut PredictedKeys) {
    for record in records {
        let key = actor_signal_relation_record_key(record);
        for statement_id in &record.source_statement_ids {
            into.entry((EvalTask::ActorSignalRelation, statement_id.clone()))
                .or_default()
                .insert(key.clone());
        }
    }
}

/// Add a batch of produced temporal rules to `into`, attributing each record's key to every
/// statement that supports it.
pub fn index_temporal_rule_predictions(records: &[TemporalRuleRecord], into: &mut PredictedKeys) {
    for record in records {
        let key = temporal_rule_record_key(record);
        for statement_id in &record.supporting_statement_ids {
            into.entry((EvalTask::TemporalRule, statement_id.clone()))
                .or_default()
                .insert(key.clone());
        }
    }
}

/// Precision / recall / F1 counts for one task.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Scorecard {
    /// True positives: gold keys the model also produced.
    pub tp: usize,
    /// False positives: model keys (on a labeled statement) absent from gold.
    pub fp: usize,
    /// False negatives: gold keys the model missed.
    pub fn_count: usize,
    /// Total gold keys across the task's labeled statements.
    pub gold_total: usize,
    /// Number of labeled statements scored for this task.
    pub labeled_statements: usize,
}

impl Scorecard {
    /// `tp / (tp + fp)` (0 when no predictions).
    pub fn precision(&self) -> f64 {
        let denom = self.tp + self.fp;
        if denom == 0 {
            0.0
        } else {
            self.tp as f64 / denom as f64
        }
    }

    /// `tp / (tp + fn)` (0 when no gold).
    pub fn recall(&self) -> f64 {
        let denom = self.tp + self.fn_count;
        if denom == 0 {
            0.0
        } else {
            self.tp as f64 / denom as f64
        }
    }

    /// Harmonic mean of precision and recall, computed from counts to avoid float-equality.
    pub fn f1(&self) -> f64 {
        let denom = 2 * self.tp + self.fp + self.fn_count;
        if denom == 0 {
            0.0
        } else {
            (2 * self.tp) as f64 / denom as f64
        }
    }
}

/// Score a labeled dataset against predicted keys, per task. Closed-world over the labeled
/// statements: only keys predicted *for a labeled statement* are counted.
pub fn score_dataset(
    items: &[EvalItem],
    predicted: &PredictedKeys,
) -> BTreeMap<EvalTask, Scorecard> {
    let mut out: BTreeMap<EvalTask, Scorecard> = BTreeMap::new();
    let empty: BTreeSet<String> = BTreeSet::new();
    for item in items {
        let card = out.entry(item.task).or_default();
        card.labeled_statements += 1;
        let gold: BTreeSet<String> = item.gold.iter().map(GoldFact::canonical_key).collect();
        card.gold_total += gold.len();
        let pred = predicted
            .get(&(item.task, item.statement_id.clone()))
            .unwrap_or(&empty);
        for key in &gold {
            if pred.contains(key) {
                card.tp += 1;
            } else {
                card.fn_count += 1;
            }
        }
        for key in pred {
            if !gold.contains(key) {
                card.fp += 1;
            }
        }
    }
    out
}

/// Load a labeled eval dataset from `path`, validated and deterministically ordered.
///
/// `path` may be either a **single `.json` file** containing a JSON array of items
/// (convenient for a small reviewable seed) or a **directory** of `*.json` files, one
/// item per file (convenient as the set grows).
pub fn load_eval_dataset(path: &Path) -> Result<Vec<EvalItem>> {
    let mut items = if path.is_dir() {
        let mut items = Vec::new();
        for entry in fs::read_dir(path)? {
            let file = entry?.path();
            if file.extension().and_then(|ext| ext.to_str()) != Some("json") {
                continue;
            }
            let text = fs::read_to_string(&file)?;
            let item: EvalItem = serde_json::from_str(&text).map_err(|err| {
                AppError::InvalidStageArtifact(format!(
                    "eval item {} is not valid JSON: {err}",
                    file.display()
                ))
            })?;
            items.push(item);
        }
        items
    } else {
        let text = fs::read_to_string(path)?;
        serde_json::from_str::<Vec<EvalItem>>(&text).map_err(|err| {
            AppError::InvalidStageArtifact(format!(
                "eval dataset {} is not a valid JSON array of items: {err}",
                path.display()
            ))
        })?
    };
    for item in &items {
        item.validate()?;
    }
    // Deterministic order so reports are stable across runs.
    items.sort_by(|a, b| {
        (a.task, &a.doc_key, &a.statement_id).cmp(&(b.task, &b.doc_key, &b.statement_id))
    });
    Ok(items)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::source::AutomationConfidence;

    fn constraint_record(
        id: &str,
        subject: &str,
        kind: SignalConstraintKind,
        negated: bool,
        target: Option<&str>,
        statements: &[&str],
    ) -> SignalConstraintRecord {
        SignalConstraintRecord {
            constraint_id: id.to_string(),
            subject_signal: subject.to_string(),
            constraint_kind: kind,
            target_value: target.map(str::to_string),
            condition_text: None,
            negated,
            source_text: String::new(),
            supporting_statement_ids: statements.iter().map(|s| s.to_string()).collect(),
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    fn relation_record(
        id: &str,
        actor: &str,
        relation: RelationKind,
        signal: &str,
        statements: &[&str],
    ) -> ActorSignalRelation {
        ActorSignalRelation {
            relation_id: id.to_string(),
            actor_name: actor.to_string(),
            signal_name: signal.to_string(),
            relation,
            source_statement_ids: statements.iter().map(|s| s.to_string()).collect(),
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn gold_and_record_keys_match_for_the_same_fact() {
        // Constraint: gold (snake kind, mixed case) == record (typed kind, mixed case).
        let gold = GoldFact::Constraint {
            subject_signal: "haddr".to_string(),
            constraint_kind: "MUST_BE_STABLE".to_string(),
            negated: false,
            target_value: None,
        };
        let record = constraint_record(
            "c1",
            "HADDR",
            SignalConstraintKind::MustBeStable,
            false,
            None,
            &["statement_1"],
        );
        assert_eq!(gold.canonical_key(), signal_constraint_record_key(&record));

        // MustBeValue: the variant's value is the target slot.
        let gold_v = GoldFact::Constraint {
            subject_signal: "HTRANS".to_string(),
            constraint_kind: "must_be_value".to_string(),
            negated: false,
            target_value: Some("nonseq".to_string()),
        };
        let record_v = constraint_record(
            "c2",
            "HTRANS",
            SignalConstraintKind::MustBeValue {
                value: "NONSEQ".to_string(),
            },
            false,
            None,
            &["statement_2"],
        );
        assert_eq!(
            gold_v.canonical_key(),
            signal_constraint_record_key(&record_v)
        );

        // Relation: gold (mixed case) == record (typed direction).
        let gold_r = GoldFact::Relation {
            actor: "manager".to_string(),
            relation: "DRIVES".to_string(),
            signal: "htrans".to_string(),
        };
        let record_r = relation_record("r1", "Manager", RelationKind::Drives, "HTRANS", &["s3"]);
        assert_eq!(
            gold_r.canonical_key(),
            actor_signal_relation_record_key(&record_r)
        );
    }

    #[test]
    fn keys_discriminate_negation_and_direction() {
        let pos = GoldFact::Constraint {
            subject_signal: "X".to_string(),
            constraint_kind: "must_be_high".to_string(),
            negated: false,
            target_value: None,
        };
        let neg = GoldFact::Constraint {
            subject_signal: "X".to_string(),
            constraint_kind: "must_be_high".to_string(),
            negated: true,
            target_value: None,
        };
        assert_ne!(pos.canonical_key(), neg.canonical_key());

        let drives = relation_record("r", "M", RelationKind::Drives, "S", &["s"]);
        let reads = relation_record("r", "M", RelationKind::Reads, "S", &["s"]);
        assert_ne!(
            actor_signal_relation_record_key(&drives),
            actor_signal_relation_record_key(&reads)
        );
    }

    #[test]
    fn score_dataset_computes_precision_recall_f1_closed_world() {
        // One labeled relation statement: gold = {Manager drives HTRANS, Manager reads HREADY}.
        let item = EvalItem {
            task: EvalTask::ActorSignalRelation,
            doc_key: "doc".to_string(),
            statement_id: "s1".to_string(),
            input_text: String::new(),
            grounding: vec![],
            gold: vec![
                GoldFact::Relation {
                    actor: "Manager".to_string(),
                    relation: "drives".to_string(),
                    signal: "HTRANS".to_string(),
                },
                GoldFact::Relation {
                    actor: "Manager".to_string(),
                    relation: "reads".to_string(),
                    signal: "HREADY".to_string(),
                },
            ],
            label_status: "agent_drafted".to_string(),
            label_note: String::new(),
        };

        // Model produced (for s1): the first gold edge (TP), plus a spurious edge (FP).
        // It also produced an edge for an UNLABELED statement s2 — must be ignored.
        let mut predicted: PredictedKeys = PredictedKeys::new();
        index_relation_predictions(
            &[
                relation_record("p1", "Manager", RelationKind::Drives, "HTRANS", &["s1"]),
                relation_record("p2", "Manager", RelationKind::Drives, "HWDATA", &["s1"]),
                relation_record("p3", "Subordinate", RelationKind::Drives, "PREADY", &["s2"]),
            ],
            &mut predicted,
        );

        let scores = score_dataset(&[item], &predicted);
        let card = &scores[&EvalTask::ActorSignalRelation];
        assert_eq!(card.tp, 1, "Manager drives HTRANS matched");
        assert_eq!(card.fn_count, 1, "Manager reads HREADY missed");
        assert_eq!(
            card.fp, 1,
            "Manager drives HWDATA is spurious on a labeled statement"
        );
        assert_eq!(card.gold_total, 2);
        assert_eq!(card.labeled_statements, 1);
        // precision 1/2, recall 1/2, F1 1/2.
        assert!((card.precision() - 0.5).abs() < 1e-9);
        assert!((card.recall() - 0.5).abs() < 1e-9);
        assert!((card.f1() - 0.5).abs() < 1e-9);
    }

    #[test]
    fn negative_item_penalizes_a_spurious_prediction() {
        // Correct extraction for s9 is NOTHING; the model invents a constraint → 1 FP, 0 gold.
        let item = EvalItem {
            task: EvalTask::SignalConstraint,
            doc_key: "doc".to_string(),
            statement_id: "s9".to_string(),
            input_text: String::new(),
            grounding: vec![],
            gold: vec![],
            label_status: "agent_drafted".to_string(),
            label_note: "no normative constraint here".to_string(),
        };
        let mut predicted = PredictedKeys::new();
        index_constraint_predictions(
            &[constraint_record(
                "c",
                "PSEL",
                SignalConstraintKind::MustBeHigh,
                false,
                None,
                &["s9"],
            )],
            &mut predicted,
        );
        let scores = score_dataset(&[item], &predicted);
        let card = &scores[&EvalTask::SignalConstraint];
        assert_eq!(card.tp, 0);
        assert_eq!(card.fn_count, 0);
        assert_eq!(card.fp, 1);
        assert!((card.precision() - 0.0).abs() < 1e-9);
    }

    #[test]
    fn validate_rejects_task_gold_mismatch() {
        let bad = EvalItem {
            task: EvalTask::SignalConstraint,
            doc_key: "d".to_string(),
            statement_id: "s".to_string(),
            input_text: String::new(),
            grounding: vec![],
            gold: vec![GoldFact::Relation {
                actor: "M".to_string(),
                relation: "drives".to_string(),
                signal: "S".to_string(),
            }],
            label_status: String::new(),
            label_note: String::new(),
        };
        assert!(bad.validate().is_err());
    }

    #[test]
    fn load_eval_dataset_round_trips_a_json_item() {
        let dir = tempfile::tempdir().unwrap();
        let json = r#"{
            "task": "actor_signal_relation",
            "doc_key": "apb",
            "statement_id": "statement_0184",
            "input_text": "The Completer drives PREADY.",
            "grounding": ["PREADY"],
            "gold": [{"fact": "relation", "actor": "Completer", "relation": "drives", "signal": "PREADY"}],
            "label_status": "agent_drafted",
            "label_note": "clear driver verb"
        }"#;
        std::fs::write(dir.path().join("item1.json"), json).unwrap();
        // a non-json file must be ignored
        std::fs::write(dir.path().join("README.md"), "ignore me").unwrap();

        let items = load_eval_dataset(dir.path()).unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].task, EvalTask::ActorSignalRelation);
        assert_eq!(items[0].statement_id, "statement_0184");
        assert_eq!(items[0].gold.len(), 1);
        assert_eq!(
            items[0].gold[0].canonical_key(),
            relation_key("Completer", "drives", "PREADY")
        );
    }

    #[test]
    fn load_eval_dataset_reads_a_single_array_file() {
        let dir = tempfile::tempdir().unwrap();
        let file = dir.path().join("seed.json");
        let json = r#"[
            {"task":"signal_constraint","doc_key":"d","statement_id":"s1",
             "gold":[{"fact":"constraint","subject_signal":"HADDR","constraint_kind":"must_be_stable"}]},
            {"task":"actor_signal_relation","doc_key":"d","statement_id":"s2",
             "gold":[{"fact":"relation","actor":"Manager","relation":"drives","signal":"HTRANS"}]}
        ]"#;
        std::fs::write(&file, json).unwrap();
        let items = load_eval_dataset(&file).unwrap();
        assert_eq!(items.len(), 2);
        // deterministic order: SignalConstraint sorts before ActorSignalRelation.
        assert_eq!(items[0].task, EvalTask::SignalConstraint);
        assert_eq!(items[1].task, EvalTask::ActorSignalRelation);
    }

    #[test]
    fn committed_seed_dataset_loads_and_validates() {
        // The real seed must parse, validate (gold fact <-> task), and cover both tasks.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data/llm_eval/seed_apb.json");
        let items = load_eval_dataset(&path).expect("seed dataset loads + validates");
        assert!(
            items.len() >= 16,
            "seed should have >= 16 items, got {}",
            items.len()
        );
        let constraints = items
            .iter()
            .filter(|i| i.task == EvalTask::SignalConstraint)
            .count();
        let relations = items
            .iter()
            .filter(|i| i.task == EvalTask::ActorSignalRelation)
            .count();
        assert!(
            constraints >= 8,
            "expected >= 8 constraint items, got {constraints}"
        );
        assert!(
            relations >= 8,
            "expected >= 8 relation items, got {relations}"
        );
        // at least one negative (empty gold) per task is present
        assert!(
            items
                .iter()
                .any(|i| i.task == EvalTask::SignalConstraint && i.gold.is_empty())
        );
        assert!(
            items
                .iter()
                .any(|i| i.task == EvalTask::ActorSignalRelation && i.gold.is_empty())
        );
    }

    fn temporal_rule_record(
        id: &str,
        edge: ClockEdge,
        antecedents: Vec<TemporalPredicateRecord>,
        consequents: Vec<TemporalPredicateRecord>,
        cycle_window: Option<CycleWindowRecord>,
        statements: &[&str],
    ) -> TemporalRuleRecord {
        TemporalRuleRecord {
            rule_id: id.to_string(),
            clock_signal: None,
            edge,
            antecedents,
            consequents,
            cycle_window,
            source_text: String::new(),
            supporting_statement_ids: statements.iter().map(|s| s.to_string()).collect(),
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn temporal_gold_and_record_keys_match_and_ignore_order_and_case() {
        // antecedent: PSEL high pre-tick; consequents: PADDR + PWRITE stable across the tick.
        let record = temporal_rule_record(
            "t1",
            ClockEdge::Rising,
            vec![TemporalPredicateRecord::SignalValue {
                signal_name: "PSEL".to_string(),
                value: "HIGH".to_string(),
                phase: TickPhase::PreTick,
            }],
            vec![
                TemporalPredicateRecord::SignalStable {
                    signal_name: "PADDR".to_string(),
                    from_phase: TickPhase::PreTick,
                    to_phase: TickPhase::PostTick,
                },
                TemporalPredicateRecord::SignalStable {
                    signal_name: "PWRITE".to_string(),
                    from_phase: TickPhase::PreTick,
                    to_phase: TickPhase::PostTick,
                },
            ],
            None,
            &["s1"],
        );

        // Gold: same logical content, consequents in REVERSE order, signals lower-cased.
        let gold = GoldFact::TemporalRule {
            edge: ClockEdge::Rising,
            antecedents: vec![TemporalPredicateRecord::SignalValue {
                signal_name: "psel".to_string(),
                value: "high".to_string(),
                phase: TickPhase::PreTick,
            }],
            consequents: vec![
                TemporalPredicateRecord::SignalStable {
                    signal_name: "pwrite".to_string(),
                    from_phase: TickPhase::PreTick,
                    to_phase: TickPhase::PostTick,
                },
                TemporalPredicateRecord::SignalStable {
                    signal_name: "paddr".to_string(),
                    from_phase: TickPhase::PreTick,
                    to_phase: TickPhase::PostTick,
                },
            ],
            cycle_window: None,
        };
        assert_eq!(gold.task(), EvalTask::TemporalRule);
        assert_eq!(gold.canonical_key(), temporal_rule_record_key(&record));
    }

    #[test]
    fn temporal_keys_discriminate_window_and_edge() {
        let cons = vec![TemporalPredicateRecord::SignalSampled {
            signal_name: "PRDATA".to_string(),
            phase: TickPhase::PostTick,
        }];
        let base = temporal_rule_record("t", ClockEdge::Rising, vec![], cons.clone(), None, &["s"]);
        let windowed = temporal_rule_record(
            "t",
            ClockEdge::Rising,
            vec![],
            cons.clone(),
            Some(CycleWindowRecord {
                min_cycles: Some(1),
                max_cycles: Some(1),
            }),
            &["s"],
        );
        assert_ne!(
            temporal_rule_record_key(&base),
            temporal_rule_record_key(&windowed),
            "cycle window is part of identity"
        );

        let falling = temporal_rule_record("t", ClockEdge::Falling, vec![], cons, None, &["s"]);
        assert_ne!(
            temporal_rule_record_key(&base),
            temporal_rule_record_key(&falling),
            "clock edge is part of identity"
        );
    }

    #[test]
    fn score_dataset_scores_temporal_rules_closed_world() {
        // s1 gold = one rule (PSEL high pre -> PADDR stable across the tick).
        let item = EvalItem {
            task: EvalTask::TemporalRule,
            doc_key: "apb".to_string(),
            statement_id: "s1".to_string(),
            input_text: String::new(),
            grounding: vec![],
            gold: vec![GoldFact::TemporalRule {
                edge: ClockEdge::Rising,
                antecedents: vec![TemporalPredicateRecord::SignalValue {
                    signal_name: "PSEL".to_string(),
                    value: "HIGH".to_string(),
                    phase: TickPhase::PreTick,
                }],
                consequents: vec![TemporalPredicateRecord::SignalStable {
                    signal_name: "PADDR".to_string(),
                    from_phase: TickPhase::PreTick,
                    to_phase: TickPhase::PostTick,
                }],
                cycle_window: None,
            }],
            label_status: "agent_drafted".to_string(),
            label_note: String::new(),
        };

        // matching rule on s1 (TP), a spurious rule on s1 (FP), one on unlabeled s2 (ignored).
        let matching = temporal_rule_record(
            "p1",
            ClockEdge::Rising,
            vec![TemporalPredicateRecord::SignalValue {
                signal_name: "PSEL".to_string(),
                value: "HIGH".to_string(),
                phase: TickPhase::PreTick,
            }],
            vec![TemporalPredicateRecord::SignalStable {
                signal_name: "PADDR".to_string(),
                from_phase: TickPhase::PreTick,
                to_phase: TickPhase::PostTick,
            }],
            None,
            &["s1"],
        );
        let spurious = temporal_rule_record(
            "p2",
            ClockEdge::Falling,
            vec![],
            vec![TemporalPredicateRecord::SignalSampled {
                signal_name: "PRDATA".to_string(),
                phase: TickPhase::PostTick,
            }],
            None,
            &["s1"],
        );
        let unlabeled = temporal_rule_record(
            "p3",
            ClockEdge::Rising,
            vec![],
            vec![TemporalPredicateRecord::SignalSampled {
                signal_name: "HREADY".to_string(),
                phase: TickPhase::PostTick,
            }],
            None,
            &["s2"],
        );

        let mut predicted = PredictedKeys::new();
        index_temporal_rule_predictions(&[matching, spurious, unlabeled], &mut predicted);

        let scores = score_dataset(&[item], &predicted);
        let card = &scores[&EvalTask::TemporalRule];
        assert_eq!(card.tp, 1, "the matching rule on s1");
        assert_eq!(card.fn_count, 0, "gold rule was produced");
        assert_eq!(card.fp, 1, "the Falling-edge sample rule is spurious on s1");
        assert_eq!(card.gold_total, 1);
        assert_eq!(card.labeled_statements, 1);
        assert!((card.precision() - 0.5).abs() < 1e-9);
        assert!((card.recall() - 1.0).abs() < 1e-9);
    }

    #[test]
    fn committed_temporal_seed_loads_and_validates() {
        // The temporal gold seed must parse, validate (gold fact <-> task), cover the
        // temporal_rule task, and carry negatives. Labels are drafted independently from the
        // APB prose; see `seed_apb_temporal.json` `label_note`s.
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data/llm_eval/seed_apb_temporal.json");
        let items = load_eval_dataset(&path).expect("temporal seed loads + validates");
        assert!(
            items.len() >= 6,
            "temporal seed should have >= 6 items, got {}",
            items.len()
        );
        assert!(
            items.iter().all(|i| i.task == EvalTask::TemporalRule),
            "every temporal-seed item is the temporal_rule task"
        );
        let negatives = items.iter().filter(|i| i.gold.is_empty()).count();
        assert!(
            negatives >= 2,
            "expected >= 2 negative items, got {negatives}"
        );
        // Positive gold facts produce well-formed, non-empty canonical keys.
        let keys: BTreeSet<String> = items
            .iter()
            .flat_map(|i| i.gold.iter().map(GoldFact::canonical_key))
            .collect();
        assert!(
            keys.iter()
                .all(|k| !k.is_empty() && k.starts_with("rising|")),
            "temporal keys are well-formed and edge-led"
        );
        // The faithful PBUSER gold carries the full 3-condition antecedent (the recall-gap
        // case the seed exists to surface).
        assert!(
            items.iter().any(|i| i.statement_id == "statement_0339"
                && i.gold.iter().any(|g| matches!(
                    g,
                    GoldFact::TemporalRule { antecedents, .. } if antecedents.len() == 3
                ))),
            "statement_0339 gold must keep all three asserted preconditions"
        );
    }
}
