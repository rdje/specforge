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

/// Document-level fact **recall** per task: a gold fact counts as found if it is predicted on
/// ANY statement (attribution-agnostic). This reveals true recall when the per-statement
/// closed-world scorer under-counts a fact the extractor attributes to a *different* sentence
/// than the gold did — observed on the real APB relations: every gold edge is present in the
/// EvidenceIR, none on the gold's own statement, so [`score_dataset`] reports R=0 while the
/// extractor in fact found them all. Returns `(found, total_gold)`. Precision is omitted on
/// purpose — the gold is a small labeled subset, so document-level precision is not meaningful.
pub fn score_fact_recall(
    items: &[EvalItem],
    predicted: &PredictedKeys,
) -> BTreeMap<EvalTask, (usize, usize)> {
    // Union all predicted fact keys per task, across statements.
    let mut pred_by_task: BTreeMap<EvalTask, BTreeSet<String>> = BTreeMap::new();
    for ((task, _statement_id), keys) in predicted.iter() {
        pred_by_task
            .entry(*task)
            .or_default()
            .extend(keys.iter().cloned());
    }
    let empty = BTreeSet::new();
    let mut out: BTreeMap<EvalTask, (usize, usize)> = BTreeMap::new();
    for item in items {
        let entry = out.entry(item.task).or_insert((0, 0));
        let pred = pred_by_task.get(&item.task).unwrap_or(&empty);
        for gold in &item.gold {
            entry.1 += 1;
            if pred.contains(&gold.canonical_key()) {
                entry.0 += 1;
            }
        }
    }
    out
}

/// Source-tolerant scorecard (`WIRE-BASED-100.1b`): RECALL credits a gold fact found *anywhere* in
/// the document (a fact has many valid source sentences — table or prose — the gold picks one
/// arbitrarily), while PRECISION stays strict — a predicted fact on a labeled statement whose key is
/// in NO gold fact is a false positive (this still catches over-generation like garbage actors). No
/// faking: a gold fact genuinely absent everywhere is a real miss.
pub fn score_dataset_source_tolerant(
    items: &[EvalItem],
    predicted: &PredictedKeys,
) -> BTreeMap<EvalTask, Scorecard> {
    let mut pred_anywhere: BTreeMap<EvalTask, BTreeSet<String>> = BTreeMap::new();
    for ((task, _), keys) in predicted.iter() {
        pred_anywhere
            .entry(*task)
            .or_default()
            .extend(keys.iter().cloned());
    }
    let mut gold_by_task: BTreeMap<EvalTask, BTreeSet<String>> = BTreeMap::new();
    let mut labeled: BTreeSet<(EvalTask, String)> = BTreeSet::new();
    for item in items {
        labeled.insert((item.task, item.statement_id.clone()));
        let g = gold_by_task.entry(item.task).or_default();
        for gold in &item.gold {
            g.insert(gold.canonical_key());
        }
    }
    let empty = BTreeSet::new();
    let mut out: BTreeMap<EvalTask, Scorecard> = BTreeMap::new();
    for (task, gold) in &gold_by_task {
        let pred_any = pred_anywhere.get(task).unwrap_or(&empty);
        let tp = gold.intersection(pred_any).count();
        let fn_count = gold.len() - tp;
        // Strict precision: predicted facts on this task's labeled statements not in ANY gold fact.
        let mut pred_on_labeled: BTreeSet<String> = BTreeSet::new();
        for ((t, sid), keys) in predicted.iter() {
            if t == task && labeled.contains(&(*task, sid.clone())) {
                pred_on_labeled.extend(keys.iter().cloned());
            }
        }
        let fp = pred_on_labeled.difference(gold).count();
        out.insert(
            *task,
            Scorecard {
                tp,
                fp,
                fn_count,
                gold_total: gold.len(),
                labeled_statements: labeled.iter().filter(|(t, _)| t == task).count(),
            },
        );
    }
    out
}

/// The gold facts NOT found anywhere in the predictions (document-level), per task — the
/// complement of [`score_fact_recall`]. Returns each missed fact's canonical key, so a review
/// can pinpoint *exactly* which gold fact the extractor misses (instead of unreliable manual
/// key reconstruction). Deterministic order (gold order within each item).
pub fn missed_gold_facts(
    items: &[EvalItem],
    predicted: &PredictedKeys,
) -> BTreeMap<EvalTask, Vec<String>> {
    let mut pred_by_task: BTreeMap<EvalTask, BTreeSet<String>> = BTreeMap::new();
    for ((task, _statement_id), keys) in predicted.iter() {
        pred_by_task
            .entry(*task)
            .or_default()
            .extend(keys.iter().cloned());
    }
    let empty = BTreeSet::new();
    let mut out: BTreeMap<EvalTask, Vec<String>> = BTreeMap::new();
    for item in items {
        let pred = pred_by_task.get(&item.task).unwrap_or(&empty);
        for gold in &item.gold {
            let key = gold.canonical_key();
            if !pred.contains(&key) {
                out.entry(item.task).or_default().push(key);
            }
        }
    }
    out
}

/// Per-relation-kind P/R/F1 for the `ActorSignalRelation` task — splits the keys by the
/// relation kind (the middle field of `ACTOR|kind|SIGNAL`), so **Drives** and **Reads** are
/// scored separately (other tasks are ignored). Keyed by `"drives"` / `"reads"`.
pub fn score_relations_by_kind(
    items: &[EvalItem],
    predicted: &PredictedKeys,
) -> BTreeMap<String, Scorecard> {
    let empty: BTreeSet<String> = BTreeSet::new();
    let mut out: BTreeMap<String, Scorecard> = BTreeMap::new();
    let kind_of = |k: &str| k.split('|').nth(1).unwrap_or("").to_string();
    for item in items {
        if item.task != EvalTask::ActorSignalRelation {
            continue;
        }
        let gold: BTreeSet<String> = item.gold.iter().map(GoldFact::canonical_key).collect();
        let pred = predicted
            .get(&(item.task, item.statement_id.clone()))
            .unwrap_or(&empty);
        for g in &gold {
            let card = out.entry(kind_of(g)).or_default();
            card.gold_total += 1;
            if pred.contains(g) {
                card.tp += 1;
            } else {
                card.fn_count += 1;
            }
        }
        for p in pred {
            if !gold.contains(p) {
                out.entry(kind_of(p)).or_default().fp += 1;
            }
        }
    }
    out
}

/// MUC-style near-misses for the `ActorSignalRelation` task: a missed gold relation that a
/// prediction *almost* matched — same (actor, signal) but the direction flipped, or same
/// (direction, signal) but a different actor named. More informative than counting these as
/// plain FP + FN.
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct RelationNearMiss {
    /// Right actor + signal, wrong direction (Drives vs Reads).
    pub wrong_direction: usize,
    /// Right direction + signal, a different actor named.
    pub wrong_actor: usize,
}

/// Compute [`RelationNearMiss`] counts over the labeled relation statements.
pub fn relation_near_misses(items: &[EvalItem], predicted: &PredictedKeys) -> RelationNearMiss {
    let empty: BTreeSet<String> = BTreeSet::new();
    let parts = |k: &str| -> (String, String, String) {
        let mut it = k.split('|');
        (
            it.next().unwrap_or("").to_string(),
            it.next().unwrap_or("").to_string(),
            it.next().unwrap_or("").to_string(),
        )
    };
    let mut nm = RelationNearMiss::default();
    for item in items {
        if item.task != EvalTask::ActorSignalRelation {
            continue;
        }
        let gold: BTreeSet<String> = item.gold.iter().map(GoldFact::canonical_key).collect();
        let pred = predicted
            .get(&(item.task, item.statement_id.clone()))
            .unwrap_or(&empty);
        for g in &gold {
            if pred.contains(g) {
                continue;
            }
            let (ga, gr, gs) = parts(g);
            let spurious: Vec<(String, String, String)> = pred
                .iter()
                .filter(|p| !gold.contains(*p))
                .map(|p| parts(p))
                .collect();
            if spurious
                .iter()
                .any(|(pa, pr, ps)| *pa == ga && *ps == gs && *pr != gr)
            {
                nm.wrong_direction += 1;
            } else if spurious
                .iter()
                .any(|(pa, pr, ps)| *pr == gr && *ps == gs && *pa != ga)
            {
                nm.wrong_actor += 1;
            }
        }
    }
    nm
}

/// GriTS-content (positional) similarity between a **gold** and **predicted** table, each
/// given as rows of cell text (header rows ++ body rows). Returns a [`Scorecard`] over cells
/// matched by identical `(row, col, normalized text)`: tp = cells present and equal in both,
/// fp = predicted cells with no gold match, fn = gold cells missed. Empty/whitespace cells are
/// ignored. This is the positional-alignment variant of GriTS_con — a sound first measure of
/// table-structure extraction quality when row/column order is preserved (docling does). The
/// full GriTS performs optimal 2-D alignment; positional matching is its lower bound.
/// The non-empty cells of a table grid as `(row, col, normalized text)`.
fn grid_cells(rows: &[Vec<String>]) -> BTreeSet<(usize, usize, String)> {
    let mut out = BTreeSet::new();
    for (r, row) in rows.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            let norm = cell.trim().to_ascii_lowercase();
            if !norm.is_empty() {
                out.insert((r, c, norm));
            }
        }
    }
    out
}

pub fn grits_content(gold_rows: &[Vec<String>], pred_rows: &[Vec<String>]) -> Scorecard {
    let gold = grid_cells(gold_rows);
    let pred = grid_cells(pred_rows);
    let tp = gold.intersection(&pred).count();
    Scorecard {
        tp,
        fp: pred.len() - tp,
        fn_count: gold.len() - tp,
        gold_total: gold.len(),
        labeled_statements: 1,
    }
}

/// Consensus table gold from N INDEPENDENT witness grids, plus the disagreement set. A cell is
/// GOLD when at least `min_agree` witnesses place the same `(row, col, text)`; a position where the
/// witnesses place DIFFERENT non-empty text (and no single value reaches `min_agree`) is a
/// DISAGREEMENT → flagged for human review (the small set where the cheap automated witnesses
/// split). This is weak-supervision / consensus *silver* gold — the κ inter-annotator idea applied
/// cross-tool (pdfplumber + qwen2.5vl; GRITS-CROSS-TOOL).
#[derive(Debug, Default, PartialEq, Eq)]
pub struct WitnessConsensus {
    /// Cells at least `min_agree` witnesses agree on — the silver gold.
    pub gold: BTreeSet<(usize, usize, String)>,
    /// Positions the witnesses split on (no value reached `min_agree`), each carrying the COMPETING
    /// `(text, witness_count)` pairs (most-supported first) — the actionable ADJUDICATION queue.
    /// An adjudicator (an evidence-grounded agent, or a human) resolves each cell against the
    /// rendered source — never by a correlated vote. This is the bounded-LLM principle: the
    /// witnesses propose, the source decides.
    pub disagreements: BTreeMap<(usize, usize), Vec<(String, usize)>>,
}

/// Build the [`WitnessConsensus`] over `witnesses` at agreement level `min_agree` (use 2 for
/// "both witnesses must agree").
pub fn witness_consensus(witnesses: &[Vec<Vec<String>>], min_agree: usize) -> WitnessConsensus {
    let mut tally: BTreeMap<(usize, usize), BTreeMap<String, usize>> = BTreeMap::new();
    for w in witnesses {
        for (r, c, text) in grid_cells(w) {
            *tally.entry((r, c)).or_default().entry(text).or_default() += 1;
        }
    }
    let mut out = WitnessConsensus::default();
    for ((r, c), texts) in tally {
        let (best_text, best_count) = texts
            .iter()
            .max_by_key(|(_, n)| **n)
            .map(|(t, n)| (t.clone(), *n))
            .expect("non-empty tally");
        if best_count >= min_agree {
            out.gold.insert((r, c, best_text));
        } else if texts.len() > 1 {
            // The witnesses split with no winner — queue the competing values for review,
            // most-supported first (ties broken lexically for determinism).
            let mut competing: Vec<(String, usize)> = texts.into_iter().collect();
            competing.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            out.disagreements.insert((r, c), competing);
        }
    }
    out
}

/// GriTS over a consensus gold cell-set vs a prediction grid (e.g. docling's). `tp` = gold cells
/// the prediction reproduces, `fn` = gold cells missed, `fp` = prediction cells absent from the
/// consensus gold.
pub fn grits_against_consensus(
    gold: &BTreeSet<(usize, usize, String)>,
    prediction: &[Vec<String>],
) -> Scorecard {
    let pred = grid_cells(prediction);
    let tp = gold.intersection(&pred).count();
    Scorecard {
        tp,
        fp: pred.len() - tp,
        fn_count: gold.len() - tp,
        gold_total: gold.len(),
        labeled_statements: 1,
    }
}

/// The positions where the consensus `gold` and the `prediction` disagree — each docling's CANDIDATE
/// ERROR, the adjudication queue. Per disputed `(row, col)`: the gold value and the prediction value
/// (`None` = that side has no cell there). An evidence-grounded adjudicator renders the region and
/// rules which matches the source. Cells where they agree are omitted.
pub fn gold_vs_prediction_mismatches(
    gold: &BTreeSet<(usize, usize, String)>,
    prediction: &[Vec<String>],
) -> Vec<(usize, usize, Option<String>, Option<String>)> {
    let gold_pos: BTreeMap<(usize, usize), String> =
        gold.iter().map(|(r, c, t)| ((*r, *c), t.clone())).collect();
    let pred_pos: BTreeMap<(usize, usize), String> = grid_cells(prediction)
        .into_iter()
        .map(|(r, c, t)| ((r, c), t))
        .collect();
    let positions: BTreeSet<(usize, usize)> =
        gold_pos.keys().chain(pred_pos.keys()).copied().collect();
    positions
        .into_iter()
        .filter_map(|pos| {
            let g = gold_pos.get(&pos).cloned();
            let p = pred_pos.get(&pos).cloned();
            (g != p).then_some((pos.0, pos.1, g, p))
        })
        .collect()
}

/// Content-word set of a sentence (alphanumeric tokens > 2 chars, lowercased).
fn content_words(text: &str) -> BTreeSet<String> {
    text.split(|c: char| !c.is_ascii_alphanumeric())
        .filter(|w| w.len() > 2)
        .map(|w| w.to_ascii_lowercase())
        .collect()
}

/// The current statement whose text best CONTAINS `input_text` (content-word overlap ≥ `min_overlap`),
/// or `None` if none clears the bar. Re-resolves a gold label to the right statement after a re-ingest
/// drifts `statement_id`s — WITHOUT faking: a label whose sentence is genuinely absent stays
/// unresolved and will legitimately score as a miss. `WIRE-BASED-100.1`.
pub fn best_statement_for_text(
    input_text: &str,
    statements: &[(String, String)],
    min_overlap: f64,
) -> Option<String> {
    let target = content_words(input_text);
    if target.is_empty() {
        return None;
    }
    statements
        .iter()
        .filter_map(|(id, text)| {
            let present = content_words(text);
            let hits = target.iter().filter(|w| present.contains(*w)).count();
            let overlap = hits as f64 / target.len() as f64;
            (overlap >= min_overlap).then_some((id.clone(), overlap))
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(id, _)| id)
}

/// Re-resolve each gold item's `statement_id` to the current evidence via [`best_statement_for_text`].
/// The expected facts stay UNCHANGED — only the stale pointer is corrected (re-ingest-proof scoring,
/// no inflation).
pub fn realign_gold_statement_ids(
    items: &[EvalItem],
    statements: &[(String, String)],
    min_overlap: f64,
) -> Vec<EvalItem> {
    items
        .iter()
        .map(|item| {
            let mut realigned = item.clone();
            if let Some(sid) = best_statement_for_text(&item.input_text, statements, min_overlap) {
                realigned.statement_id = sid;
            }
            realigned
        })
        .collect()
}

/// A risk-controlled accept threshold from a split-conformal calibration set.
#[derive(Debug, Clone, PartialEq)]
pub struct ConformalThreshold {
    /// Accept a prediction iff its confidence score `>= threshold`.
    pub threshold: f64,
    /// Fraction of the calibration set that would be accepted at this threshold.
    pub coverage: f64,
    /// Empirical error rate among the accepted calibration predictions.
    pub empirical_error: f64,
}

/// Split-conformal risk-controlling threshold: given calibration `(confidence, is_correct)`
/// pairs and a target error rate `alpha`, return the **lowest** score threshold whose accepted
/// set (`score >= threshold`) has a conservative error bound `(errors + 1) / (n + 1) <= alpha`
/// — i.e. the most coverage subject to the finite-sample risk guarantee. `None` if no threshold
/// meets the bound. The `+1` is the standard conformal finite-sample correction.
pub fn conformal_threshold(samples: &[(f64, bool)], alpha: f64) -> Option<ConformalThreshold> {
    let mut taus: Vec<f64> = samples.iter().map(|(s, _)| *s).collect();
    taus.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    taus.dedup_by(|a, b| (*a - *b).abs() < f64::EPSILON);
    // Ascending tau → the first satisfying threshold has the largest accepted set (max coverage).
    for &tau in &taus {
        let accepted: Vec<bool> = samples
            .iter()
            .filter(|(s, _)| *s >= tau)
            .map(|(_, ok)| *ok)
            .collect();
        if accepted.is_empty() {
            continue;
        }
        let n = accepted.len();
        let errors = accepted.iter().filter(|ok| !**ok).count();
        let bound = (errors as f64 + 1.0) / (n as f64 + 1.0);
        if bound <= alpha {
            return Some(ConformalThreshold {
                threshold: tau,
                coverage: n as f64 / samples.len() as f64,
                empirical_error: errors as f64 / n as f64,
            });
        }
    }
    None
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
    fn per_kind_scoring_and_near_misses_split_relations() {
        // gold s1: (Manager, Drives, HTRANS), (Manager, Reads, HREADY).
        let items = vec![EvalItem {
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
        }];
        // pred s1: exact (Manager drives HTRANS); and (Manager DRIVES HREADY) — a
        // flipped-direction near-miss of the missed gold (Manager reads HREADY).
        let mut predicted: PredictedKeys = PredictedKeys::new();
        index_relation_predictions(
            &[
                relation_record("p1", "Manager", RelationKind::Drives, "HTRANS", &["s1"]),
                relation_record("p2", "Manager", RelationKind::Drives, "HREADY", &["s1"]),
            ],
            &mut predicted,
        );

        let by_kind = score_relations_by_kind(&items, &predicted);
        assert_eq!(by_kind["drives"].tp, 1, "Manager drives HTRANS matched");
        assert_eq!(by_kind["drives"].fp, 1, "Manager drives HREADY spurious");
        assert_eq!(by_kind["reads"].fn_count, 1, "Manager reads HREADY missed");
        assert_eq!(by_kind["reads"].tp, 0);

        let nm = relation_near_misses(&items, &predicted);
        assert_eq!(
            nm.wrong_direction, 1,
            "Manager drives HREADY is a flipped-direction near-miss of Manager reads HREADY"
        );
        assert_eq!(nm.wrong_actor, 0);
    }

    #[test]
    fn document_level_recall_finds_facts_attributed_to_another_statement() {
        // gold relation on s1; the extractor produced it on s2 (a different statement). The
        // per-statement closed-world scorer misses it; document-level recall finds it — the
        // real APB pattern (every gold edge present, none on the gold's own statement).
        let items = vec![EvalItem {
            task: EvalTask::ActorSignalRelation,
            doc_key: "doc".to_string(),
            statement_id: "s1".to_string(),
            input_text: String::new(),
            grounding: vec![],
            gold: vec![GoldFact::Relation {
                actor: "Manager".to_string(),
                relation: "drives".to_string(),
                signal: "HTRANS".to_string(),
            }],
            label_status: "agent_drafted".to_string(),
            label_note: String::new(),
        }];
        let mut predicted: PredictedKeys = PredictedKeys::new();
        index_relation_predictions(
            &[relation_record(
                "p1",
                "Manager",
                RelationKind::Drives,
                "HTRANS",
                &["s2"],
            )],
            &mut predicted,
        );
        // per-statement: the cross-attributed fact is a miss.
        assert_eq!(
            score_dataset(&items, &predicted)[&EvalTask::ActorSignalRelation].tp,
            0
        );
        // document-level: found (1/1).
        let recall = score_fact_recall(&items, &predicted);
        assert_eq!(recall[&EvalTask::ActorSignalRelation], (1, 1));
    }

    #[test]
    fn missed_gold_facts_lists_the_unfound_gold() {
        // two gold relations; only one is predicted anywhere → the other is listed missed.
        let items = vec![EvalItem {
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
        }];
        let mut predicted: PredictedKeys = PredictedKeys::new();
        index_relation_predictions(
            &[relation_record(
                "p1",
                "Manager",
                RelationKind::Drives,
                "HTRANS",
                &["s9"],
            )],
            &mut predicted,
        );
        let missed = missed_gold_facts(&items, &predicted);
        let m = &missed[&EvalTask::ActorSignalRelation];
        assert_eq!(m.len(), 1, "only HREADY missed: {m:?}");
        assert!(m[0].contains("HREADY"));
    }

    #[test]
    fn grits_content_scores_table_cell_matches() {
        let gold = vec![
            vec!["Signal".to_string(), "Width".to_string()],
            vec!["PADDR".to_string(), "32".to_string()],
        ];
        // header matches; the (1,1) body cell differs (gold 32 vs pred 16).
        let pred = vec![
            vec!["Signal".to_string(), "Width".to_string()],
            vec!["PADDR".to_string(), "16".to_string()],
        ];
        let card = super::grits_content(&gold, &pred);
        assert_eq!(card.tp, 3, "Signal, Width, PADDR match");
        assert_eq!(card.fn_count, 1, "gold (1,1)=32 missed");
        assert_eq!(card.fp, 1, "pred (1,1)=16 spurious");
        // F1 = 2*3 / (2*3 + 1 + 1) = 6/8.
        assert!((card.f1() - 0.75).abs() < 1e-9);
    }

    #[test]
    fn witness_consensus_builds_gold_and_flags_disagreements() {
        // Two independent witnesses (pdfplumber + qwen2.5vl style): they agree on the header +
        // PADDR, and SPLIT on the width cell (1,1) — 32 vs 16.
        let w1 = vec![
            vec!["Signal".to_string(), "Width".to_string()],
            vec!["PADDR".to_string(), "32".to_string()],
        ];
        let w2 = vec![
            vec!["Signal".to_string(), "Width".to_string()],
            vec!["PADDR".to_string(), "16".to_string()],
        ];
        let cons = super::witness_consensus(&[w1, w2], 2);
        // Gold = the agreed cells; the split cell is NOT gold but IS a human-flag.
        assert!(cons.gold.contains(&(0, 0, "signal".to_string())));
        assert!(cons.gold.contains(&(1, 0, "paddr".to_string())));
        assert!(
            !cons.gold.iter().any(|(r, c, _)| (*r, *c) == (1, 1)),
            "the split cell must not be gold"
        );
        // (1,1) is flagged for human review, carrying BOTH competing values for adjudication.
        let competing = cons
            .disagreements
            .get(&(1, 1))
            .expect("(1,1) flagged for human review");
        assert_eq!(competing.len(), 2, "both witness values queued");
        let values: BTreeSet<&str> = competing.iter().map(|(t, _)| t.as_str()).collect();
        assert!(values.contains("32") && values.contains("16"));
        // docling (the system under test) scored against the consensus gold.
        let docling = vec![
            vec!["Signal".to_string(), "Width".to_string()],
            vec!["PADDR".to_string(), "32".to_string()],
        ];
        let card = super::grits_against_consensus(&cons.gold, &docling);
        assert_eq!(card.tp, 3, "Signal/Width/PADDR reproduced");
        assert_eq!(card.fn_count, 0, "no consensus-gold cell missed");
        assert_eq!(
            card.fp, 1,
            "docling's (1,1)=32 is not in the consensus gold"
        );
    }

    #[test]
    fn gold_vs_prediction_mismatches_lists_disputed_cells() {
        let mut gold = BTreeSet::new();
        gold.insert((0, 0, "a".to_string()));
        gold.insert((1, 1, "x".to_string()));
        // Prediction agrees at (0,0), differs at (1,1) (y vs x), adds a spurious (0,1).
        let pred = vec![
            vec!["a".to_string(), "z".to_string()],
            vec![String::new(), "y".to_string()],
        ];
        let m = super::gold_vs_prediction_mismatches(&gold, &pred);
        assert!(
            m.contains(&(0, 1, None, Some("z".to_string()))),
            "docling-only spurious cell flagged"
        );
        assert!(
            m.contains(&(1, 1, Some("x".to_string()), Some("y".to_string()))),
            "value disagreement flagged with both sides"
        );
        assert!(
            !m.iter().any(|(r, c, _, _)| (*r, *c) == (0, 0)),
            "the agreed cell is omitted"
        );
    }

    #[test]
    fn best_statement_resolves_a_drifted_id_by_content() {
        let statements = vec![
            (
                "s_old".to_string(),
                "Figure 3-1 Write transfer with no wait states".to_string(),
            ),
            (
                "s_new".to_string(),
                "The Access phase of the write transfer is shown where PENABLE is asserted. \
                 PREADY is asserted by the Completer at the rising edge."
                    .to_string(),
            ),
        ];
        // The gold sentence drifted off s_old (now a caption); content match re-resolves it to s_new.
        let got = super::best_statement_for_text(
            "PREADY is asserted by the Completer at the rising edge",
            &statements,
            0.7,
        );
        assert_eq!(got.as_deref(), Some("s_new"));
    }

    #[test]
    fn best_statement_refuses_a_low_overlap_match_no_faking() {
        let statements = vec![(
            "s1".to_string(),
            "Totally unrelated text about clock domains".to_string(),
        )];
        // No statement contains the gold sentence → None; the label legitimately scores as a miss.
        assert_eq!(
            super::best_statement_for_text(
                "PSTRB must be driven low for read transfers",
                &statements,
                0.7
            ),
            None
        );
    }

    #[test]
    fn conformal_threshold_controls_risk() {
        // Higher score ⇒ more likely correct.
        let samples = vec![
            (0.9, true),
            (0.85, true),
            (0.8, true),
            (0.7, true),
            (0.6, false),
            (0.5, true),
            (0.4, false),
            (0.3, false),
        ];
        let t = super::conformal_threshold(&samples, 0.25).expect("a threshold exists");
        // The lowest tau whose accepted set bounds risk is 0.7 (top 4, all correct).
        assert!((t.threshold - 0.7).abs() < 1e-9);
        assert!(t.empirical_error <= 0.25 + 1e-9);
        assert!((t.coverage - 0.5).abs() < 1e-9);
        // No threshold can meet alpha=0.01 (even the singleton top has bound 1/2).
        assert!(super::conformal_threshold(&samples, 0.01).is_none());
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
