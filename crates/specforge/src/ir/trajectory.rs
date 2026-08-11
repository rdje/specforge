//! Reviewable multi-metric trajectory classification and task ranking.
//!
//! `SPEC-TO-INTENT-ALIGNMENT.5a` keeps controller policy independent of the current
//! reviewed product result. Inputs retain nine separate objective dimensions, exact
//! denominators, uncertainty, evidence, hard gates, comparable history, and task-tree
//! ownership. The engine reports and recommends; it cannot mutate canonical semantics.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};

pub const TRAJECTORY_SCHEMA_VERSION: u32 = 1;
const MAX_EXACT_INTEGER: u64 = 1_000_000_000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrajectoryDimension {
    SourceCapture,
    SemanticCorrectness,
    SemanticCompleteness,
    StageConservation,
    ProvenanceHonesty,
    ProductionParticipation,
    GeneralizationRobustness,
    OperationalConfidence,
    ExecutableReadiness,
}

impl TrajectoryDimension {
    pub const ALL: [Self; 9] = [
        Self::SourceCapture,
        Self::SemanticCorrectness,
        Self::SemanticCompleteness,
        Self::StageConservation,
        Self::ProvenanceHonesty,
        Self::ProductionParticipation,
        Self::GeneralizationRobustness,
        Self::OperationalConfidence,
        Self::ExecutableReadiness,
    ];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TargetOperator {
    AtLeast,
    AtMost,
    Equal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ImprovementDirection {
    HigherIsBetter,
    LowerIsBetter,
    ExactInvariant,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fraction {
    pub numerator: u64,
    pub denominator: u64,
}

impl Fraction {
    pub const fn new(numerator: u64, denominator: u64) -> Self {
        Self {
            numerator,
            denominator,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EvidenceRef {
    pub path: String,
    pub sha256: String,
    pub role: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Uncertainty {
    Exact,
    Estimated {
        method: String,
        lower: Fraction,
        upper: Fraction,
        sample_size: usize,
    },
    Unavailable {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetricTarget {
    pub operator: TargetOperator,
    pub value: Fraction,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetricObservation {
    pub metric_id: String,
    pub question: String,
    pub measure: Option<Fraction>,
    pub target: MetricTarget,
    pub improvement: ImprovementDirection,
    pub material_change: Fraction,
    pub hard: bool,
    pub required: bool,
    pub oracle: String,
    pub population: String,
    pub uncertainty: Uncertainty,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionObservation {
    pub dimension: TrajectoryDimension,
    pub objective: String,
    pub metrics: Vec<MetricObservation>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct HardGateObservation {
    pub gate_id: String,
    pub question: String,
    pub violations: u64,
    pub target_violations: u64,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComparableSnapshot {
    pub revision: String,
    pub artifact_fingerprint: String,
    pub metrics: BTreeMap<String, Fraction>,
    pub hard_gate_violations: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapPriorityTier {
    HardInvariant,
    SourceEvidenceLoss,
    SemanticRegression,
    MissingOracle,
    PersistentResidual,
    BreadthEfficiency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CausalConfidence {
    High,
    Medium,
    Low,
}

impl CausalConfidence {
    fn rank(self) -> u8 {
        match self {
            Self::High => 0,
            Self::Medium => 1,
            Self::Low => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReversibleSliceSize {
    Small,
    Medium,
    Large,
}

impl ReversibleSliceSize {
    fn rank(self) -> u8 {
        match self {
            Self::Small => 0,
            Self::Medium => 1,
            Self::Large => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GapUncertainty {
    Exact,
    Bounded,
    High,
}

impl GapUncertainty {
    fn rank(self) -> u8 {
        match self {
            Self::Exact => 0,
            Self::Bounded => 1,
            Self::High => 2,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskOwner {
    pub task_id: String,
    pub task_tree_path: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrajectoryGap {
    pub gap_id: String,
    pub objective_id: String,
    pub tier: GapPriorityTier,
    pub observed: String,
    pub target: String,
    pub first_failing_stage: String,
    pub affected_population: usize,
    pub causal_confidence: CausalConfidence,
    pub reversible_slice: ReversibleSliceSize,
    pub uncertainty: GapUncertainty,
    pub estimated_impact: String,
    pub reproduction: String,
    pub owner: TaskOwner,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ControllerMode {
    ReportOnly,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ControllerAuthority {
    pub mode: ControllerMode,
    pub canonical_semantic_mutation_allowed: bool,
    pub task_tree_review_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrajectoryControllerInput {
    pub schema_version: u32,
    pub snapshot_id: String,
    pub owner: String,
    pub reviewed_revision: String,
    pub objective_contract: EvidenceRef,
    pub stall_window: usize,
    pub dimensions: Vec<DimensionObservation>,
    pub hard_gates: Vec<HardGateObservation>,
    pub history: Vec<ComparableSnapshot>,
    pub gaps: Vec<TrajectoryGap>,
    pub authority: ControllerAuthority,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DimensionStatus {
    MeetsTarget,
    Deficit,
    Unmeasurable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MetricResult {
    pub metric_id: String,
    pub measure: Option<Fraction>,
    pub target: MetricTarget,
    pub target_met: Option<bool>,
    pub hard: bool,
    pub required: bool,
    pub oracle: String,
    pub population: String,
    pub uncertainty: Uncertainty,
    pub evidence: Vec<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct DimensionResult {
    pub dimension: TrajectoryDimension,
    pub objective: String,
    pub status: DimensionStatus,
    pub metrics: Vec<MetricResult>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrajectoryState {
    Converging,
    Diverging,
    Stalled,
    Mixed,
    Unmeasurable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum HistoryStatus {
    Comparable,
    InsufficientHistory,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TaskProposal {
    pub task_id: String,
    pub task_tree_path: String,
    pub gap_id: String,
    pub rationale: String,
    pub reproduction: String,
    pub review_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TrajectoryReport {
    pub schema_version: u32,
    pub snapshot_id: String,
    pub owner: String,
    pub reviewed_revision: String,
    pub objective_contract: EvidenceRef,
    pub state: TrajectoryState,
    pub state_reasons: Vec<String>,
    pub history_status: HistoryStatus,
    pub dimensions: Vec<DimensionResult>,
    pub hard_gates: Vec<HardGateObservation>,
    pub ranked_gaps: Vec<TrajectoryGap>,
    pub recommendation: Option<TaskProposal>,
    pub authority: ControllerAuthority,
}

/// Load and validate a repository-relative trajectory-controller input.
pub fn load_controller_input(path: &Path) -> Result<TrajectoryControllerInput> {
    let path_text = path.to_string_lossy();
    if !is_safe_relative_path(&path_text) {
        return Err(AppError::InvalidStageArtifact(format!(
            "trajectory input path must be repository-relative: {path_text}"
        )));
    }
    let repository = crate::project_data::repository_root()?;
    let absolute = repository.join(path).canonicalize()?;
    if !absolute.starts_with(&repository) {
        return Err(AppError::InvalidStageArtifact(format!(
            "trajectory input path escapes the repository: {path_text}"
        )));
    }
    let input: TrajectoryControllerInput = serde_json::from_slice(&fs::read(absolute)?)?;
    validate_controller_input(&input).map_err(|problems| {
        AppError::InvalidStageArtifact(format!(
            "invalid trajectory controller input: {}",
            problems.join("; ")
        ))
    })?;
    Ok(input)
}

/// Validate the strict objective vector, evidence routes, history, gaps, and authority boundary.
pub fn validate_controller_input(
    input: &TrajectoryControllerInput,
) -> std::result::Result<(), Vec<String>> {
    let mut problems = Vec::new();
    if input.schema_version != TRAJECTORY_SCHEMA_VERSION {
        problems.push(format!(
            "schema_version must be {TRAJECTORY_SCHEMA_VERSION}"
        ));
    }
    for (label, value) in [
        ("snapshot_id", input.snapshot_id.as_str()),
        ("owner", input.owner.as_str()),
        ("reviewed_revision", input.reviewed_revision.as_str()),
    ] {
        if value.trim().is_empty() {
            problems.push(format!("{label} must not be empty"));
        }
    }
    if input.stall_window < 2 {
        problems.push("stall_window must be at least 2 comparable snapshots".to_string());
    }
    if input.authority.canonical_semantic_mutation_allowed {
        problems.push("controller must not authorize canonical semantic mutation".to_string());
    }
    if !input.authority.task_tree_review_required {
        problems.push("controller task proposals must require task-tree review".to_string());
    }

    validate_evidence_ref(
        &input.objective_contract,
        "objective_contract",
        &mut problems,
    );

    let mut seen_dimensions = BTreeSet::new();
    let mut metric_ids = BTreeSet::new();
    for dimension in &input.dimensions {
        if !seen_dimensions.insert(dimension.dimension) {
            problems.push(format!("duplicate dimension: {:?}", dimension.dimension));
        }
        if dimension.objective.trim().is_empty() {
            problems.push(format!(
                "dimension {:?} objective must not be empty",
                dimension.dimension
            ));
        }
        if dimension.metrics.is_empty() {
            problems.push(format!(
                "dimension {:?} must contain at least one metric",
                dimension.dimension
            ));
        }
        for metric in &dimension.metrics {
            validate_metric(metric, &mut metric_ids, &mut problems);
        }
    }
    let expected_dimensions: BTreeSet<_> = TrajectoryDimension::ALL.into_iter().collect();
    for missing in expected_dimensions.difference(&seen_dimensions) {
        problems.push(format!(
            "missing required trajectory dimension: {missing:?}"
        ));
    }
    for unexpected in seen_dimensions.difference(&expected_dimensions) {
        problems.push(format!("unexpected trajectory dimension: {unexpected:?}"));
    }

    let mut gate_ids = BTreeSet::new();
    for gate in &input.hard_gates {
        if gate.gate_id.trim().is_empty() || !gate_ids.insert(gate.gate_id.clone()) {
            problems.push(format!(
                "hard gate id must be non-empty and unique: '{}'",
                gate.gate_id
            ));
        }
        if gate.question.trim().is_empty() {
            problems.push(format!(
                "hard gate '{}' question must not be empty",
                gate.gate_id
            ));
        }
        if gate.evidence.is_empty() {
            problems.push(format!("hard gate '{}' needs evidence", gate.gate_id));
        }
        for evidence in &gate.evidence {
            validate_evidence_ref(
                evidence,
                &format!("hard gate '{}': evidence", gate.gate_id),
                &mut problems,
            );
        }
    }
    if input.hard_gates.is_empty() {
        problems.push("at least one hard gate is required".to_string());
    }

    let mut revisions = BTreeSet::new();
    for history in &input.history {
        if history.revision.trim().is_empty() || !revisions.insert(history.revision.clone()) {
            problems.push(format!(
                "history revision must be non-empty and unique: '{}'",
                history.revision
            ));
        }
        if !is_sha256_digest(&history.artifact_fingerprint) {
            problems.push(format!(
                "history '{}' artifact_fingerprint must be a lowercase SHA-256 digest",
                history.revision
            ));
        }
        for (metric_id, measure) in &history.metrics {
            if !metric_ids.contains(metric_id) {
                problems.push(format!(
                    "history '{}' names unknown metric '{}'",
                    history.revision, metric_id
                ));
            }
            validate_fraction(
                measure,
                &format!("history '{}' metric '{metric_id}'", history.revision),
                &mut problems,
            );
        }
        for gate_id in history.hard_gate_violations.keys() {
            if !gate_ids.contains(gate_id) {
                problems.push(format!(
                    "history '{}' names unknown hard gate '{}'",
                    history.revision, gate_id
                ));
            }
        }
    }

    let mut gap_ids = BTreeSet::new();
    for gap in &input.gaps {
        validate_gap(gap, &mut gap_ids, &mut problems);
    }
    let has_deficit = input
        .dimensions
        .iter()
        .flat_map(|dimension| &dimension.metrics)
        .any(|metric| {
            metric.required
                && metric
                    .measure
                    .is_none_or(|measure| !target_met(measure, &metric.target))
        })
        || input
            .hard_gates
            .iter()
            .any(|gate| gate.violations > gate.target_violations);
    if has_deficit && input.gaps.is_empty() {
        problems.push(
            "a controller input with deficits must provide at least one owned gap".to_string(),
        );
    }

    if problems.is_empty() {
        Ok(())
    } else {
        Err(problems)
    }
}

/// Classify trajectory and rank task-tree-owned gaps without mutating canonical data.
pub fn evaluate_trajectory(
    input: &TrajectoryControllerInput,
) -> std::result::Result<TrajectoryReport, Vec<String>> {
    validate_controller_input(input)?;
    let dimensions = summarize_dimensions(input);
    let (state, mut state_reasons) = classify_state(input, &dimensions);
    state_reasons.sort();
    state_reasons.dedup();
    let mut ranked_gaps = input.gaps.clone();
    ranked_gaps.sort_by(compare_gaps);
    let recommendation = ranked_gaps.first().map(|gap| TaskProposal {
        task_id: gap.owner.task_id.clone(),
        task_tree_path: gap.owner.task_tree_path.clone(),
        gap_id: gap.gap_id.clone(),
        rationale: format!(
            "tier {:?}; affects {} reviewed units; first failure {}; causal confidence {:?}",
            gap.tier, gap.affected_population, gap.first_failing_stage, gap.causal_confidence
        ),
        reproduction: gap.reproduction.clone(),
        review_required: true,
    });
    Ok(TrajectoryReport {
        schema_version: TRAJECTORY_SCHEMA_VERSION,
        snapshot_id: input.snapshot_id.clone(),
        owner: input.owner.clone(),
        reviewed_revision: input.reviewed_revision.clone(),
        objective_contract: input.objective_contract.clone(),
        state,
        state_reasons,
        history_status: if input.history.is_empty() {
            HistoryStatus::InsufficientHistory
        } else {
            HistoryStatus::Comparable
        },
        dimensions,
        hard_gates: input.hard_gates.clone(),
        ranked_gaps,
        recommendation,
        authority: input.authority.clone(),
    })
}

fn summarize_dimensions(input: &TrajectoryControllerInput) -> Vec<DimensionResult> {
    let mut results: Vec<_> = input
        .dimensions
        .iter()
        .map(|dimension| {
            let metrics: Vec<_> = dimension
                .metrics
                .iter()
                .map(|metric| MetricResult {
                    metric_id: metric.metric_id.clone(),
                    measure: metric.measure,
                    target: metric.target.clone(),
                    target_met: metric
                        .measure
                        .map(|measure| target_met(measure, &metric.target)),
                    hard: metric.hard,
                    required: metric.required,
                    oracle: metric.oracle.clone(),
                    population: metric.population.clone(),
                    uncertainty: metric.uncertainty.clone(),
                    evidence: metric.evidence.clone(),
                })
                .collect();
            let status = if metrics
                .iter()
                .any(|metric| metric.required && metric.measure.is_none())
            {
                DimensionStatus::Unmeasurable
            } else if metrics
                .iter()
                .any(|metric| metric.required && metric.target_met == Some(false))
            {
                DimensionStatus::Deficit
            } else {
                DimensionStatus::MeetsTarget
            };
            DimensionResult {
                dimension: dimension.dimension,
                objective: dimension.objective.clone(),
                status,
                metrics,
            }
        })
        .collect();
    results.sort_by_key(|result| result.dimension);
    results
}

fn classify_state(
    input: &TrajectoryControllerInput,
    dimensions: &[DimensionResult],
) -> (TrajectoryState, Vec<String>) {
    let mut reasons = Vec::new();
    for gate in &input.hard_gates {
        if gate.violations > gate.target_violations {
            reasons.push(format!(
                "hard_gate_failed:{}:{}>{}",
                gate.gate_id, gate.violations, gate.target_violations
            ));
        }
    }
    for dimension in dimensions {
        for metric in &dimension.metrics {
            if metric.hard && metric.required && metric.target_met == Some(false) {
                reasons.push(format!("hard_metric_failed:{}", metric.metric_id));
            }
        }
    }
    if !reasons.is_empty() {
        return (TrajectoryState::Diverging, reasons);
    }

    let unavailable: Vec<_> = dimensions
        .iter()
        .flat_map(|dimension| &dimension.metrics)
        .filter(|metric| metric.required && metric.measure.is_none())
        .map(|metric| metric.metric_id.clone())
        .collect();
    if !unavailable.is_empty() {
        reasons.extend(
            unavailable
                .into_iter()
                .map(|metric_id| format!("required_metric_unavailable:{metric_id}")),
        );
        return (TrajectoryState::Unmeasurable, reasons);
    }

    let Some(previous) = input.history.last() else {
        return (
            TrajectoryState::Unmeasurable,
            vec!["insufficient_comparable_history".to_string()],
        );
    };

    let mut improvements = Vec::new();
    let mut regressions = Vec::new();
    let mut missing_comparisons = Vec::new();
    for dimension in &input.dimensions {
        for metric in &dimension.metrics {
            let Some(current) = metric.measure else {
                continue;
            };
            if matches!(metric.uncertainty, Uncertainty::Estimated { .. }) {
                missing_comparisons.push(format!(
                    "{}:paired_uncertainty_history_required",
                    metric.metric_id
                ));
                continue;
            }
            let Some(prior) = previous.metrics.get(&metric.metric_id).copied() else {
                if metric.required {
                    missing_comparisons.push(metric.metric_id.clone());
                }
                continue;
            };
            if !materially_changed(current, prior, metric.material_change) {
                continue;
            }
            match metric.improvement {
                ImprovementDirection::HigherIsBetter => match compare_fraction(current, prior) {
                    Ordering::Greater => improvements.push(metric.metric_id.clone()),
                    Ordering::Less => regressions.push(metric.metric_id.clone()),
                    Ordering::Equal => {}
                },
                ImprovementDirection::LowerIsBetter => match compare_fraction(current, prior) {
                    Ordering::Less => improvements.push(metric.metric_id.clone()),
                    Ordering::Greater => regressions.push(metric.metric_id.clone()),
                    Ordering::Equal => {}
                },
                ImprovementDirection::ExactInvariant => regressions.push(metric.metric_id.clone()),
            }
        }
    }
    if !missing_comparisons.is_empty() {
        reasons.extend(
            missing_comparisons
                .into_iter()
                .map(|metric_id| format!("missing_comparable_metric:{metric_id}")),
        );
        return (TrajectoryState::Unmeasurable, reasons);
    }
    if !regressions.is_empty() && !improvements.is_empty() {
        reasons.extend(
            improvements
                .into_iter()
                .map(|metric_id| format!("material_improvement:{metric_id}")),
        );
        reasons.extend(
            regressions
                .into_iter()
                .map(|metric_id| format!("material_regression:{metric_id}")),
        );
        return (TrajectoryState::Mixed, reasons);
    }
    if !regressions.is_empty() {
        reasons.extend(
            regressions
                .into_iter()
                .map(|metric_id| format!("material_regression:{metric_id}")),
        );
        return (TrajectoryState::Diverging, reasons);
    }
    if !improvements.is_empty() {
        reasons.extend(
            improvements
                .into_iter()
                .map(|metric_id| format!("material_improvement:{metric_id}")),
        );
        return (TrajectoryState::Converging, reasons);
    }

    let deficits_remain = dimensions
        .iter()
        .any(|dimension| dimension.status == DimensionStatus::Deficit);
    if deficits_remain && input.history.len() + 1 >= input.stall_window {
        return (
            TrajectoryState::Stalled,
            vec![format!(
                "no_material_change_across_{}_snapshot_window",
                input.stall_window
            )],
        );
    }
    (
        TrajectoryState::Unmeasurable,
        vec!["insufficient_comparable_history_for_stall".to_string()],
    )
}

fn compare_gaps(left: &TrajectoryGap, right: &TrajectoryGap) -> Ordering {
    left.tier
        .cmp(&right.tier)
        .then_with(|| right.affected_population.cmp(&left.affected_population))
        .then_with(|| {
            left.causal_confidence
                .rank()
                .cmp(&right.causal_confidence.rank())
        })
        .then_with(|| {
            left.reversible_slice
                .rank()
                .cmp(&right.reversible_slice.rank())
        })
        .then_with(|| left.uncertainty.rank().cmp(&right.uncertainty.rank()))
        .then_with(|| left.gap_id.cmp(&right.gap_id))
}

fn validate_metric(
    metric: &MetricObservation,
    metric_ids: &mut BTreeSet<String>,
    problems: &mut Vec<String>,
) {
    if metric.metric_id.trim().is_empty() || !metric_ids.insert(metric.metric_id.clone()) {
        problems.push(format!(
            "metric id must be non-empty and globally unique: '{}'",
            metric.metric_id
        ));
    }
    for (field, value) in [
        ("question", metric.question.as_str()),
        ("oracle", metric.oracle.as_str()),
        ("population", metric.population.as_str()),
    ] {
        if value.trim().is_empty() {
            problems.push(format!(
                "metric '{}' {field} must not be empty",
                metric.metric_id
            ));
        }
    }
    validate_fraction(
        &metric.target.value,
        &format!("metric '{}' target", metric.metric_id),
        problems,
    );
    validate_fraction(
        &metric.material_change,
        &format!("metric '{}' material_change", metric.metric_id),
        problems,
    );
    if let Some(measure) = &metric.measure {
        validate_fraction(
            measure,
            &format!("metric '{}' measure", metric.metric_id),
            problems,
        );
    }
    match (&metric.measure, &metric.uncertainty) {
        (Some(_), Uncertainty::Unavailable { .. }) => problems.push(format!(
            "metric '{}' has a measure but marks uncertainty unavailable",
            metric.metric_id
        )),
        (None, Uncertainty::Exact | Uncertainty::Estimated { .. }) => problems.push(format!(
            "metric '{}' lacks a measure but claims measured uncertainty",
            metric.metric_id
        )),
        (_, Uncertainty::Unavailable { reason }) if reason.trim().is_empty() => {
            problems.push(format!(
                "metric '{}' unavailable reason must not be empty",
                metric.metric_id
            ))
        }
        (
            _,
            Uncertainty::Estimated {
                method,
                lower,
                upper,
                sample_size,
            },
        ) => {
            if method.trim().is_empty() || *sample_size == 0 {
                problems.push(format!(
                    "metric '{}' estimated uncertainty needs a method and positive sample_size",
                    metric.metric_id
                ));
            }
            validate_fraction(
                lower,
                &format!("metric '{}' uncertainty lower", metric.metric_id),
                problems,
            );
            validate_fraction(
                upper,
                &format!("metric '{}' uncertainty upper", metric.metric_id),
                problems,
            );
            if compare_fraction(*lower, *upper) == Ordering::Greater {
                problems.push(format!(
                    "metric '{}' uncertainty lower bound exceeds upper bound",
                    metric.metric_id
                ));
            }
            if let Some(measure) = metric.measure
                && (compare_fraction(measure, *lower) == Ordering::Less
                    || compare_fraction(measure, *upper) == Ordering::Greater)
            {
                problems.push(format!(
                    "metric '{}' measure lies outside its uncertainty interval",
                    metric.metric_id
                ));
            }
        }
        _ => {}
    }
    if metric.evidence.is_empty() {
        problems.push(format!("metric '{}' needs evidence", metric.metric_id));
    }
    for evidence in &metric.evidence {
        validate_evidence_ref(
            evidence,
            &format!("metric '{}': evidence", metric.metric_id),
            problems,
        );
    }
}

fn validate_gap(gap: &TrajectoryGap, gap_ids: &mut BTreeSet<String>, problems: &mut Vec<String>) {
    if gap.gap_id.trim().is_empty() || !gap_ids.insert(gap.gap_id.clone()) {
        problems.push(format!(
            "gap id must be non-empty and unique: '{}'",
            gap.gap_id
        ));
    }
    for (field, value) in [
        ("objective_id", gap.objective_id.as_str()),
        ("observed", gap.observed.as_str()),
        ("target", gap.target.as_str()),
        ("first_failing_stage", gap.first_failing_stage.as_str()),
        ("estimated_impact", gap.estimated_impact.as_str()),
        ("reproduction", gap.reproduction.as_str()),
    ] {
        if value.trim().is_empty() {
            problems.push(format!("gap '{}' {field} must not be empty", gap.gap_id));
        }
    }
    if gap.affected_population == 0 {
        problems.push(format!(
            "gap '{}' affected_population must be positive",
            gap.gap_id
        ));
    }
    if gap.owner.task_id.trim().is_empty() {
        problems.push(format!(
            "gap '{}' task owner id must not be empty",
            gap.gap_id
        ));
    }
    if gap.evidence.is_empty() {
        problems.push(format!("gap '{}' needs evidence", gap.gap_id));
    }
    for evidence in &gap.evidence {
        validate_evidence_ref(
            evidence,
            &format!("gap '{}': evidence", gap.gap_id),
            problems,
        );
    }
    validate_task_owner(&gap.owner, &gap.gap_id, problems);
}

fn validate_task_owner(owner: &TaskOwner, gap_id: &str, problems: &mut Vec<String>) {
    if !is_safe_relative_path(&owner.task_tree_path) {
        problems.push(format!(
            "gap '{gap_id}' task_tree_path must be repository-relative: {}",
            owner.task_tree_path
        ));
        return;
    }
    let Ok(repository) = crate::project_data::repository_root() else {
        problems.push(format!("gap '{gap_id}' cannot resolve repository root"));
        return;
    };
    let path = repository.join(&owner.task_tree_path);
    let Ok(canonical) = path.canonicalize() else {
        problems.push(format!(
            "gap '{gap_id}' task tree does not exist: {}",
            owner.task_tree_path
        ));
        return;
    };
    if !canonical.starts_with(&repository) {
        problems.push(format!("gap '{gap_id}' task tree escapes the repository"));
        return;
    }
    match fs::read_to_string(canonical) {
        Ok(contents) if contents.contains(&format!("- ID: `{}`", owner.task_id)) => {}
        Ok(_) => problems.push(format!(
            "gap '{gap_id}' owner '{}' is not declared in {}",
            owner.task_id, owner.task_tree_path
        )),
        Err(error) => problems.push(format!(
            "gap '{gap_id}' cannot read task tree '{}': {error}",
            owner.task_tree_path
        )),
    }
}

fn validate_evidence_ref(evidence: &EvidenceRef, label: &str, problems: &mut Vec<String>) {
    if evidence.role.trim().is_empty() {
        problems.push(format!("{label} role must not be empty"));
    }
    if !is_sha256_digest(&evidence.sha256) {
        problems.push(format!("{label} sha256 must be a lowercase digest"));
    }
    if !is_safe_relative_path(&evidence.path) {
        problems.push(format!(
            "{label} path must be repository-relative: {}",
            evidence.path
        ));
        return;
    }
    let Ok(repository) = crate::project_data::repository_root() else {
        problems.push(format!("{label} cannot resolve repository root"));
        return;
    };
    let path = repository.join(&evidence.path);
    match path.canonicalize() {
        Ok(canonical) if canonical.starts_with(&repository) => {}
        Ok(_) => problems.push(format!("{label} path escapes the repository")),
        Err(_) => problems.push(format!("{label} path does not exist: {}", evidence.path)),
    }
}

fn validate_fraction(value: &Fraction, label: &str, problems: &mut Vec<String>) {
    if value.denominator == 0 {
        problems.push(format!("{label} denominator must be positive"));
    }
    if value.numerator > MAX_EXACT_INTEGER || value.denominator > MAX_EXACT_INTEGER {
        problems.push(format!(
            "{label} exceeds the exact integer ceiling {MAX_EXACT_INTEGER}"
        ));
    }
    if value.denominator > 0 && value.numerator > value.denominator {
        problems.push(format!(
            "{label} must be a bounded ratio between zero and one"
        ));
    }
}

fn target_met(measure: Fraction, target: &MetricTarget) -> bool {
    match target.operator {
        TargetOperator::AtLeast => compare_fraction(measure, target.value) != Ordering::Less,
        TargetOperator::AtMost => compare_fraction(measure, target.value) != Ordering::Greater,
        TargetOperator::Equal => compare_fraction(measure, target.value) == Ordering::Equal,
    }
}

fn compare_fraction(left: Fraction, right: Fraction) -> Ordering {
    (u128::from(left.numerator) * u128::from(right.denominator))
        .cmp(&(u128::from(right.numerator) * u128::from(left.denominator)))
}

fn materially_changed(current: Fraction, previous: Fraction, threshold: Fraction) -> bool {
    let current_cross = u128::from(current.numerator) * u128::from(previous.denominator);
    let previous_cross = u128::from(previous.numerator) * u128::from(current.denominator);
    let difference = current_cross.abs_diff(previous_cross);
    let common_denominator = u128::from(current.denominator) * u128::from(previous.denominator);
    difference * u128::from(threshold.denominator)
        >= u128::from(threshold.numerator) * common_denominator
}

fn is_safe_relative_path(value: &str) -> bool {
    if value.is_empty()
        || value.starts_with('/')
        || value.starts_with('\\')
        || value.contains('\\')
        || value.split('/').any(|part| part == "..")
    {
        return false;
    }
    let bytes = value.as_bytes();
    !(bytes.len() >= 2 && bytes[0].is_ascii_alphabetic() && bytes[1] == b':')
}

fn is_sha256_digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    const ZERO_SHA: &str = "0000000000000000000000000000000000000000000000000000000000000000";
    const TASK_PATH: &str = "docs/tasks/SPEC-TO-INTENT-ALIGNMENT.md";

    fn evidence() -> EvidenceRef {
        EvidenceRef {
            path: "doctrine/spec_to_intent_category_contract.json".to_string(),
            sha256: ZERO_SHA.to_string(),
            role: "synthetic controller evidence".to_string(),
        }
    }

    fn metric_id(dimension: TrajectoryDimension) -> String {
        format!("{:?}", dimension).to_ascii_lowercase()
    }

    fn metric(dimension: TrajectoryDimension) -> MetricObservation {
        MetricObservation {
            metric_id: metric_id(dimension),
            question: "does the synthetic objective improve".to_string(),
            measure: Some(Fraction::new(8, 10)),
            target: MetricTarget {
                operator: TargetOperator::AtLeast,
                value: Fraction::new(0, 1),
            },
            improvement: ImprovementDirection::HigherIsBetter,
            material_change: Fraction::new(1, 100),
            hard: false,
            required: true,
            oracle: "synthetic exact oracle".to_string(),
            population: "ten synthetic items".to_string(),
            uncertainty: Uncertainty::Exact,
            evidence: vec![evidence()],
        }
    }

    fn gap(id: &str, tier: GapPriorityTier, affected_population: usize) -> TrajectoryGap {
        TrajectoryGap {
            gap_id: id.to_string(),
            objective_id: "synthetic-objective".to_string(),
            tier,
            observed: "deficit".to_string(),
            target: "closed".to_string(),
            first_failing_stage: "source_to_evidence_ir".to_string(),
            affected_population,
            causal_confidence: CausalConfidence::High,
            reversible_slice: ReversibleSliceSize::Small,
            uncertainty: GapUncertainty::Exact,
            estimated_impact: "restore synthetic coverage".to_string(),
            reproduction: "cargo test -p specforge --lib ir::trajectory".to_string(),
            owner: TaskOwner {
                task_id: "SPEC-TO-INTENT-ALIGNMENT.5a".to_string(),
                task_tree_path: TASK_PATH.to_string(),
            },
            evidence: vec![evidence()],
        }
    }

    fn history(
        input: &TrajectoryControllerInput,
        value: Fraction,
        revision: &str,
    ) -> ComparableSnapshot {
        ComparableSnapshot {
            revision: revision.to_string(),
            artifact_fingerprint: ZERO_SHA.to_string(),
            metrics: input
                .dimensions
                .iter()
                .flat_map(|dimension| &dimension.metrics)
                .map(|metric| (metric.metric_id.clone(), value))
                .collect(),
            hard_gate_violations: BTreeMap::from([("no_fabrication".to_string(), 0)]),
        }
    }

    fn baseline_input() -> TrajectoryControllerInput {
        TrajectoryControllerInput {
            schema_version: TRAJECTORY_SCHEMA_VERSION,
            snapshot_id: "synthetic-trajectory".to_string(),
            owner: "SPEC-TO-INTENT-ALIGNMENT.5a".to_string(),
            reviewed_revision: "synthetic-current".to_string(),
            objective_contract: evidence(),
            stall_window: 3,
            dimensions: TrajectoryDimension::ALL
                .into_iter()
                .map(|dimension| DimensionObservation {
                    dimension,
                    objective: "keep one visible synthetic objective per dimension".to_string(),
                    metrics: vec![metric(dimension)],
                })
                .collect(),
            hard_gates: vec![HardGateObservation {
                gate_id: "no_fabrication".to_string(),
                question: "are fabricated facts absent".to_string(),
                violations: 0,
                target_violations: 0,
                evidence: vec![evidence()],
            }],
            history: Vec::new(),
            gaps: Vec::new(),
            authority: ControllerAuthority {
                mode: ControllerMode::ReportOnly,
                canonical_semantic_mutation_allowed: false,
                task_tree_review_required: true,
            },
        }
    }

    #[test]
    fn exact_improvement_with_clean_hard_gates_is_converging() {
        let mut input = baseline_input();
        input
            .history
            .push(history(&input, Fraction::new(7, 10), "prior"));
        let report = evaluate_trajectory(&input).unwrap();
        assert_eq!(report.state, TrajectoryState::Converging);
        assert_eq!(report.history_status, HistoryStatus::Comparable);
    }

    #[test]
    fn seeded_fabrication_is_diverging_even_without_history() {
        let mut input = baseline_input();
        input.hard_gates[0].violations = 1;
        input
            .gaps
            .push(gap("fabrication", GapPriorityTier::HardInvariant, 1));
        let report = evaluate_trajectory(&input).unwrap();
        assert_eq!(report.state, TrajectoryState::Diverging);
        assert_eq!(report.history_status, HistoryStatus::InsufficientHistory);
        assert!(report.state_reasons[0].starts_with("hard_gate_failed:"));
    }

    #[test]
    fn exact_regression_without_a_gain_is_diverging() {
        let mut input = baseline_input();
        input
            .history
            .push(history(&input, Fraction::new(9, 10), "prior"));
        assert_eq!(
            evaluate_trajectory(&input).unwrap().state,
            TrajectoryState::Diverging
        );
    }

    #[test]
    fn simultaneous_material_gain_and_regression_is_mixed() {
        let mut input = baseline_input();
        let mut prior = history(&input, Fraction::new(8, 10), "prior");
        let first = input.dimensions[0].metrics[0].metric_id.clone();
        let second = input.dimensions[1].metrics[0].metric_id.clone();
        prior.metrics.insert(first, Fraction::new(7, 10));
        prior.metrics.insert(second, Fraction::new(9, 10));
        input.history.push(prior);
        assert_eq!(
            evaluate_trajectory(&input).unwrap().state,
            TrajectoryState::Mixed
        );
    }

    #[test]
    fn unchanged_deficit_across_the_declared_window_is_stalled() {
        let mut input = baseline_input();
        input.dimensions[0].metrics[0].target.value = Fraction::new(1, 1);
        input
            .gaps
            .push(gap("persistent", GapPriorityTier::PersistentResidual, 1));
        input
            .history
            .push(history(&input, Fraction::new(8, 10), "prior-1"));
        input
            .history
            .push(history(&input, Fraction::new(8, 10), "prior-2"));
        assert_eq!(
            evaluate_trajectory(&input).unwrap().state,
            TrajectoryState::Stalled
        );
    }

    #[test]
    fn unavailable_required_oracle_is_unmeasurable() {
        let mut input = baseline_input();
        input.dimensions[0].metrics[0].measure = None;
        input.dimensions[0].metrics[0].uncertainty = Uncertainty::Unavailable {
            reason: "no reviewed oracle".to_string(),
        };
        input
            .gaps
            .push(gap("oracle", GapPriorityTier::MissingOracle, 1));
        assert_eq!(
            evaluate_trajectory(&input).unwrap().state,
            TrajectoryState::Unmeasurable
        );
    }

    #[test]
    fn estimated_point_without_paired_history_uncertainty_cannot_claim_trend() {
        let mut input = baseline_input();
        input.dimensions[0].metrics[0].uncertainty = Uncertainty::Estimated {
            method: "document bootstrap".to_string(),
            lower: Fraction::new(7, 10),
            upper: Fraction::new(9, 10),
            sample_size: 12,
        };
        input
            .history
            .push(history(&input, Fraction::new(7, 10), "prior"));
        let report = evaluate_trajectory(&input).unwrap();
        assert_eq!(report.state, TrajectoryState::Unmeasurable);
        assert!(
            report
                .state_reasons
                .iter()
                .any(|reason| reason.contains("paired_uncertainty_history_required"))
        );
    }

    #[test]
    fn clean_snapshot_without_comparable_history_is_unmeasurable() {
        let report = evaluate_trajectory(&baseline_input()).unwrap();
        assert_eq!(report.state, TrajectoryState::Unmeasurable);
        assert_eq!(
            report.state_reasons,
            vec!["insufficient_comparable_history"]
        );
    }

    #[test]
    fn gap_ranking_is_hard_first_then_population_weighted() {
        let mut input = baseline_input();
        input.gaps = vec![
            gap("source-large", GapPriorityTier::SourceEvidenceLoss, 100),
            gap("hard-small", GapPriorityTier::HardInvariant, 1),
            gap("hard-large", GapPriorityTier::HardInvariant, 5),
        ];
        let report = evaluate_trajectory(&input).unwrap();
        let ids: Vec<_> = report
            .ranked_gaps
            .iter()
            .map(|gap| gap.gap_id.as_str())
            .collect();
        assert_eq!(ids, ["hard-large", "hard-small", "source-large"]);
        assert_eq!(report.recommendation.unwrap().gap_id, "hard-large");
    }

    #[test]
    fn semantic_mutation_authority_is_rejected() {
        let mut input = baseline_input();
        input.authority.canonical_semantic_mutation_allowed = true;
        assert!(
            validate_controller_input(&input)
                .unwrap_err()
                .iter()
                .any(|problem| problem.contains("must not authorize canonical semantic mutation"))
        );
    }

    #[test]
    fn omitting_one_objective_dimension_is_rejected() {
        let mut input = baseline_input();
        input.dimensions.pop();
        assert!(
            validate_controller_input(&input)
                .unwrap_err()
                .iter()
                .any(|problem| problem.contains("missing required trajectory dimension"))
        );
    }

    #[test]
    fn serde_rejects_unknown_input_fields() {
        let mut value = serde_json::to_value(baseline_input()).unwrap();
        value["silent_weighted_score"] = json!(0.99);
        assert!(serde_json::from_value::<TrajectoryControllerInput>(value).is_err());
    }

    #[test]
    fn absolute_evidence_path_is_rejected() {
        let mut input = baseline_input();
        input.objective_contract.path = "/tmp/objective.json".to_string();
        assert!(
            validate_controller_input(&input)
                .unwrap_err()
                .iter()
                .any(|problem| problem.contains("path must be repository-relative"))
        );
    }

    #[test]
    fn task_proposal_must_name_a_declared_owner() {
        let mut input = baseline_input();
        let mut unowned = gap("unowned", GapPriorityTier::HardInvariant, 1);
        unowned.owner.task_id = "NOT-A-REAL-TASK".to_string();
        input.gaps.push(unowned);
        assert!(
            validate_controller_input(&input)
                .unwrap_err()
                .iter()
                .any(|problem| problem.contains("is not declared"))
        );
    }

    #[test]
    fn report_serialization_is_deterministic_and_keeps_all_dimensions() {
        let input = baseline_input();
        let first = evaluate_trajectory(&input).unwrap();
        let second = evaluate_trajectory(&input).unwrap();
        assert_eq!(first.dimensions.len(), 9);
        assert_eq!(
            serde_json::to_vec_pretty(&first).unwrap(),
            serde_json::to_vec_pretty(&second).unwrap()
        );
    }

    #[test]
    fn public_schema_pins_nine_dimensions_and_report_only_authority() {
        let schema: serde_json::Value = serde_json::from_str(include_str!(
            "../../../../doctrine/trajectory_controller_input_schema.json"
        ))
        .unwrap();
        assert_eq!(schema["properties"]["schema_version"]["const"], json!(1));
        assert_eq!(
            schema["$defs"]["dimension"]["enum"]
                .as_array()
                .unwrap()
                .len(),
            TrajectoryDimension::ALL.len()
        );
        assert_eq!(
            schema["$defs"]["authority"]["properties"]["mode"]["const"],
            "report_only"
        );
        assert_eq!(
            schema["$defs"]["authority"]["properties"]["canonical_semantic_mutation_allowed"]["const"],
            false
        );
    }
}
