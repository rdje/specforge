use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;
use tempfile::tempdir;

use crate::cli::{KgBenchArgs, ValidateArgs};
use crate::commands::validate;
use crate::error::{AppError, Result};
use crate::ir::evidence::{
    EvidenceIr, SignalPolarity, SignalPolarityConflictRecord, SignalPolarityEvidenceSourceKind,
    SignalPolarityObservationRecord, SignalSemanticConflictObservationRecord,
    SignalSemanticConflictRecord, SignalSemanticHintSourceKind, SignalSemanticTag,
};
use crate::ir::intent::{IntentAssumption, IntentIr};
use crate::ir::prior_memory::{
    ActorTaxonomyPriorRecord, CorpusMemory, CorpusMemoryUpdatePolicyRecord,
    NegativeKnowledgePriorRecord, PriorSourceArtifactRecord,
    SemanticModalityReliabilityPriorRecord, SemanticPhrasePriorRecord, TableShapePriorRecord,
    TemporalPhrasePriorRecord, VisualMotifPriorRecord,
};
use crate::ir::semantic::{
    ActorPortRecord, ActorRelativeDirection, ClockEdge, CycleWindowRecord,
    InfrastructureSignalDistributionStatus, InfrastructureSignalKind, InfrastructureSignalRecord,
    InfrastructureSignalSourceStatus, InfrastructureTopologyKind, InterfaceRecord,
    InterfaceSignalConflictKind, InterfaceSignalConflictObservationRecord,
    InterfaceSignalConflictRecord, InterfaceSignalDirection, InterfaceSignalSemanticRole,
    RegularStateRecord, SemanticGroundingStrength, SemanticIr, SignalConnectivityConflictKind,
    SignalConnectivityConflictRecord, StateTransitionRecord, TemporalConflictRecord,
    TemporalPredicateRecord, TemporalRuleRecord, TickPhase,
};
use crate::ir::source::{
    ActorSignalRelation, RelationKind, ResidualDecisionPacket, ValidationFindingRecord,
    ValidationReportRecord,
};
use crate::ir::{intent, semantic, source};

const FIXTURE_FILE_NAME: &str = "fixture.json";

#[derive(Debug, Deserialize)]
struct KgBenchFixture {
    name: String,
    source: PathBuf,
    #[serde(default)]
    source_ir_patch: Option<SourceIrPatch>,
    #[serde(default)]
    evidence_ir_patch: Option<EvidenceIrPatch>,
    #[serde(default)]
    prior_memory_patch: Option<PriorMemoryPatch>,
    #[serde(default)]
    expectations: FixtureExpectations,
}

#[derive(Debug, Default, Deserialize)]
struct SourceIrPatch {
    #[serde(default)]
    structured_tables: Vec<crate::ir::source::StructuredTableRecord>,
    #[serde(default)]
    visual_assets: Vec<crate::ir::source::VisualAsset>,
    #[serde(default)]
    document_sections: Vec<crate::ir::source::ContentSectionRecord>,
}

#[derive(Debug, Default, Deserialize)]
struct EvidenceIrPatch {
    #[serde(default)]
    signal_alias_map: BTreeMap<String, String>,
    #[serde(default)]
    refresh_signal_semantic_hints: bool,
    #[serde(default)]
    signal_constraints: Vec<crate::ir::source::SignalConstraintRecord>,
}

#[derive(Debug, Default, Deserialize)]
struct PriorMemoryPatch {
    #[serde(default)]
    actor_taxonomy_priors: Vec<ActorTaxonomyPriorRecord>,
    #[serde(default)]
    semantic_phrase_priors: Vec<SemanticPhrasePriorRecord>,
    #[serde(default)]
    semantic_modality_reliability_priors: Vec<SemanticModalityReliabilityPriorRecord>,
    #[serde(default)]
    temporal_phrase_priors: Vec<TemporalPhrasePriorRecord>,
    #[serde(default)]
    table_shape_priors: Vec<TableShapePriorRecord>,
    #[serde(default)]
    visual_motif_priors: Vec<VisualMotifPriorRecord>,
    #[serde(default)]
    negative_knowledge_priors: Vec<NegativeKnowledgePriorRecord>,
}

#[derive(Debug, Default, Deserialize)]
struct FixtureExpectations {
    #[serde(default)]
    semantic: Option<CanonicalStageExpectations>,
    #[serde(default)]
    intent: Option<CanonicalStageExpectations>,
    #[serde(default)]
    validation: Option<ValidationExpectations>,
}

#[derive(Debug, Default, Deserialize)]
struct CanonicalStageExpectations {
    #[serde(default)]
    signal_names_include: Vec<String>,
    #[serde(default)]
    graph_direction_signal_names_include: Vec<String>,
    #[serde(default)]
    graph_direction_signal_names_exclude: Vec<String>,
    #[serde(default)]
    signal_directions_include: Vec<ExpectedSignalDirection>,
    #[serde(default)]
    signal_polarities_include: Vec<ExpectedSignalPolarity>,
    #[serde(default)]
    actor_ports_include: Vec<ExpectedActorPort>,
    #[serde(default)]
    actor_signal_relations_include: Vec<ExpectedActorSignalRelation>,
    #[serde(default)]
    infrastructure_signals_include: Vec<ExpectedInfrastructureSignal>,
    #[serde(default)]
    infrastructure_topologies_include: Vec<ExpectedInfrastructureTopology>,
    #[serde(default)]
    state_names_include: Vec<String>,
    #[serde(default)]
    state_names_exclude: Vec<String>,
    #[serde(default)]
    initial_state_names_include: Vec<String>,
    #[serde(default)]
    initial_state_names_exclude: Vec<String>,
    #[serde(default)]
    state_transitions_include: Vec<ExpectedStateTransition>,
    #[serde(default)]
    state_transitions_exclude: Vec<ExpectedStateTransition>,
    #[serde(default)]
    residual_decision_ids_include: Vec<String>,
    #[serde(default)]
    residual_decision_ids_exclude: Vec<String>,
    #[serde(default)]
    assumption_ids_include: Vec<String>,
    #[serde(default)]
    assumption_ids_exclude: Vec<String>,
    #[serde(default)]
    resolved_semantic_role_signal_names_include: Vec<String>,
    #[serde(default)]
    resolved_semantic_role_signal_names_exclude: Vec<String>,
    #[serde(default)]
    semantic_consensus_signal_names_include: Vec<String>,
    #[serde(default)]
    semantic_consensus_signal_names_exclude: Vec<String>,
    #[serde(default)]
    alias_dependent_semantic_consensus_signal_names_include: Vec<String>,
    #[serde(default)]
    alias_dependent_semantic_consensus_signal_names_exclude: Vec<String>,
    #[serde(default)]
    alias_dependent_semantic_candidate_signal_names_include: Vec<String>,
    #[serde(default)]
    alias_dependent_semantic_candidate_signal_names_exclude: Vec<String>,
    #[serde(default)]
    resolved_semantic_roles_include: Vec<ExpectedResolvedSemanticRole>,
    #[serde(default)]
    semantic_grounding_strengths_include: Vec<ExpectedSemanticGroundingStrength>,
    #[serde(default)]
    semantic_candidate_signal_names_include: Vec<String>,
    #[serde(default)]
    semantic_candidate_signal_names_exclude: Vec<String>,
    #[serde(default)]
    multiple_semantic_candidate_signal_names_include: Vec<String>,
    #[serde(default)]
    multiple_semantic_candidate_signal_names_exclude: Vec<String>,
    #[serde(default)]
    semantic_arbitration_signal_names_include: Vec<String>,
    #[serde(default)]
    semantic_arbitration_signal_names_exclude: Vec<String>,
    #[serde(default)]
    decisive_semantic_arbitration_signal_names_include: Vec<String>,
    #[serde(default)]
    decisive_semantic_arbitration_signal_names_exclude: Vec<String>,
    #[serde(default)]
    non_decisive_semantic_arbitration_signal_names_include: Vec<String>,
    #[serde(default)]
    non_decisive_semantic_arbitration_signal_names_exclude: Vec<String>,
    #[serde(default)]
    temporal_rules_include: Vec<ExpectedTemporalRule>,
    #[serde(default)]
    temporal_conflicts_include: Vec<ExpectedTemporalConflict>,
    #[serde(default)]
    signal_polarity_conflicts_include: Vec<ExpectedSignalPolarityConflict>,
    #[serde(default)]
    signal_semantic_conflicts_include: Vec<ExpectedSignalSemanticConflict>,
    #[serde(default)]
    signal_connectivity_conflicts_include: Vec<ExpectedSignalConnectivityConflict>,
    #[serde(default)]
    interface_signal_conflicts_include: Vec<ExpectedInterfaceSignalConflict>,
    temporal_rule_count: Option<usize>,
    temporal_rules_with_handshake_completion: Option<usize>,
    temporal_rules_with_alias_dependent_handshake_completion: Option<usize>,
    signal_polarity_conflicts: Option<usize>,
    signal_semantic_conflicts: Option<usize>,
    signal_connectivity_conflicts: Option<usize>,
    interface_signal_conflicts: Option<usize>,
    temporal_conflicts: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct ExpectedActorPort {
    actor_name: String,
    signal_name: String,
    direction: ActorRelativeDirection,
}

#[derive(Debug, Deserialize)]
struct ExpectedSignalDirection {
    signal_name: String,
    direction: InterfaceSignalDirection,
}

#[derive(Debug, Deserialize)]
struct ExpectedSignalPolarity {
    signal_name: String,
    polarity: SignalPolarity,
}

#[derive(Debug, Deserialize)]
struct ExpectedResolvedSemanticRole {
    signal_name: String,
    role: InterfaceSignalSemanticRole,
}

#[derive(Debug, Deserialize)]
struct ExpectedSemanticGroundingStrength {
    signal_name: String,
    strength: SemanticGroundingStrength,
}

#[derive(Debug, Deserialize)]
struct ExpectedActorSignalRelation {
    actor_name: String,
    signal_name: String,
    relation: RelationKind,
}

#[derive(Debug, Deserialize)]
struct ExpectedInfrastructureSignal {
    signal_name: String,
    #[serde(default)]
    kind: Option<InfrastructureSignalKind>,
    #[serde(default)]
    source_status: Option<InfrastructureSignalSourceStatus>,
    #[serde(default)]
    distribution_status: Option<InfrastructureSignalDistributionStatus>,
    #[serde(default)]
    recovered_source_actor_names_include: Vec<String>,
    #[serde(default)]
    distributed_to_actor_names_include: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedInfrastructureTopology {
    signal_name: String,
    topology_kind: InfrastructureTopologyKind,
    #[serde(default)]
    component_name: Option<String>,
    #[serde(default)]
    stage_count: Option<u32>,
    #[serde(default)]
    target_actor_names_include: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedTemporalRule {
    #[serde(default)]
    source_text: Option<String>,
    #[serde(default)]
    clock_signal: Option<String>,
    #[serde(default)]
    edge: Option<ClockEdge>,
    #[serde(default)]
    cycle_window: Option<CycleWindowRecord>,
    #[serde(default)]
    supporting_statement_ids_include: Vec<String>,
    #[serde(default)]
    antecedents_include: Vec<TemporalPredicateRecord>,
    #[serde(default)]
    consequents_include: Vec<TemporalPredicateRecord>,
}

#[derive(Debug, Deserialize)]
struct ExpectedTemporalConflict {
    signal_name: String,
    #[serde(default)]
    phase: Option<TickPhase>,
    #[serde(default)]
    clock_signal: Option<String>,
    #[serde(default)]
    edge: Option<ClockEdge>,
    #[serde(default)]
    cycle_window: Option<CycleWindowRecord>,
    #[serde(default)]
    antecedents_include: Vec<TemporalPredicateRecord>,
    #[serde(default)]
    conflicting_values_include: Vec<String>,
    #[serde(default)]
    supporting_rule_ids_include: Vec<String>,
    #[serde(default)]
    supporting_statement_ids_include: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSignalPolarityConflict {
    signal_name: String,
    #[serde(default)]
    conflict_id: Option<String>,
    #[serde(default)]
    observations_include: Vec<ExpectedSignalPolarityConflictObservation>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSignalPolarityConflictObservation {
    #[serde(default)]
    polarity: Option<SignalPolarity>,
    #[serde(default)]
    source_kind: Option<SignalPolarityEvidenceSourceKind>,
    #[serde(default)]
    supporting_statement_ids_include: Vec<String>,
    #[serde(default)]
    supporting_table_ids_include: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSignalSemanticConflict {
    signal_name: String,
    #[serde(default)]
    conflict_id: Option<String>,
    #[serde(default)]
    observations_include: Vec<ExpectedSignalSemanticConflictObservation>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSignalSemanticConflictObservation {
    #[serde(default)]
    semantic_tags_include: Vec<SignalSemanticTag>,
    #[serde(default)]
    source_kind: Option<SignalSemanticHintSourceKind>,
    #[serde(default)]
    source_text: Option<String>,
    #[serde(default)]
    supporting_statement_ids_include: Vec<String>,
    #[serde(default)]
    supporting_table_ids_include: Vec<String>,
    #[serde(default)]
    supporting_visual_evidence_ids_include: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSignalConnectivityConflict {
    signal_name: String,
    #[serde(default)]
    conflict_id: Option<String>,
    #[serde(default)]
    conflict_kind: Option<SignalConnectivityConflictKind>,
    #[serde(default)]
    conflicting_actor_ids_include: Vec<String>,
    #[serde(default)]
    conflicting_actor_names_include: Vec<String>,
    #[serde(default)]
    supporting_statement_ids_include: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedInterfaceSignalConflict {
    signal_name: String,
    #[serde(default)]
    conflict_id: Option<String>,
    #[serde(default)]
    conflict_kind: Option<InterfaceSignalConflictKind>,
    #[serde(default)]
    observations_include: Vec<ExpectedInterfaceSignalConflictObservation>,
}

#[derive(Debug, Deserialize)]
struct ExpectedInterfaceSignalConflictObservation {
    value_text: String,
    #[serde(default)]
    supporting_statement_ids_include: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedStateTransition {
    source_state: String,
    target_state: String,
}

#[derive(Debug, Default, Deserialize)]
struct ValidationExpectations {
    #[serde(default)]
    evidence: Option<ValidationStageExpectations>,
    #[serde(default)]
    semantic: Option<ValidationStageExpectations>,
    #[serde(default)]
    intent: Option<ValidationStageExpectations>,
}

#[derive(Debug, Default, Deserialize)]
struct ValidationStageExpectations {
    #[serde(default)]
    finding_ids_include: Vec<String>,
    #[serde(default)]
    finding_ids_exclude: Vec<String>,
    #[serde(default)]
    metric_values: BTreeMap<String, String>,
}

#[derive(Debug)]
pub(crate) struct KgBenchFixtureOutcome {
    pub(crate) name: String,
    pub(crate) fixture_path: PathBuf,
    pub(crate) failures: Vec<String>,
}

pub fn run(args: KgBenchArgs) -> Result<()> {
    let (fixtures_root, outcomes) = collect_fixture_outcomes(&args.fixtures_root, &args.fixtures)?;

    println!("command: kg-bench");
    println!("fixtures_root: {}", fixtures_root.display());
    println!("fixtures_requested: {}", outcomes.len());

    for outcome in &outcomes {
        println!(
            "fixture: {} [{}]",
            outcome.name,
            if outcome.failures.is_empty() {
                "pass"
            } else {
                "fail"
            }
        );
        for failure in &outcome.failures {
            println!("  - {failure}");
        }
    }

    let passed = outcomes
        .iter()
        .filter(|outcome| outcome.failures.is_empty())
        .count();
    let failed = outcomes.len().saturating_sub(passed);
    println!("fixtures_passed: {passed}");
    println!("fixtures_failed: {failed}");

    if failed > 0 {
        let mut lines = vec!["kg-bench detected fixture failures:".to_string()];
        for outcome in outcomes
            .iter()
            .filter(|outcome| !outcome.failures.is_empty())
        {
            lines.push(format!(
                "- {} ({})",
                outcome.name,
                outcome.fixture_path.display()
            ));
            for failure in &outcome.failures {
                lines.push(format!("  - {failure}"));
            }
        }
        return Err(AppError::InvalidStageArtifact(lines.join("\n")));
    }

    Ok(())
}

pub(crate) fn collect_fixture_outcomes(
    fixtures_root: &Path,
    requested: &[PathBuf],
) -> Result<(PathBuf, Vec<KgBenchFixtureOutcome>)> {
    let fixtures_root = canonicalize_existing_path(fixtures_root)?;
    let fixture_paths = resolve_fixture_paths(&fixtures_root, requested)?;
    let mut outcomes = Vec::new();
    for fixture_path in fixture_paths {
        outcomes.push(run_fixture(&fixture_path)?);
    }
    Ok((fixtures_root, outcomes))
}

fn run_fixture(fixture_path: &Path) -> Result<KgBenchFixtureOutcome> {
    let fixture = load_fixture(fixture_path)?;
    let fixture_dir = fixture_path.parent().ok_or_else(|| {
        AppError::InvalidStageArtifact(format!(
            "fixture path {} has no parent directory",
            fixture_path.display()
        ))
    })?;
    let source_path = normalize_input_path(&fixture.source, fixture_dir);
    if !source_path.exists() {
        return Err(AppError::MissingPath(source_path));
    }

    let tempdir = tempdir()?;
    let generated_root = tempdir.path().join("generated");
    let source_ir_root = generated_root.join("source_ir");
    let evidence_ir_root = generated_root.join("evidence_ir");
    let semantic_ir_root = generated_root.join("semantic_ir");
    let intent_ir_root = generated_root.join("intent_ir");
    let prior_memory_path = fixture
        .prior_memory_patch
        .as_ref()
        .map(|patch| write_fixture_prior_memory(&generated_root, patch))
        .transpose()?;

    let mut source_ir = source::SourceIr::build(&source_path, &source_ir_root)?;
    source_ir.materialize()?;
    if let Some(patch) = fixture.source_ir_patch.as_ref() {
        source_ir
            .structured_tables
            .extend(patch.structured_tables.iter().cloned());
        source_ir
            .visual_assets
            .extend(patch.visual_assets.iter().cloned());
        source_ir
            .document_sections
            .extend(patch.document_sections.iter().cloned());
    }
    source_ir.write_to_disk()?;
    let mut evidence_ir = EvidenceIr::build_with_prior_memory(
        &source_ir.artifact_layout.source_ir_path,
        &evidence_ir_root,
        prior_memory_path.as_deref(),
    )?;
    if let Some(patch) = fixture.evidence_ir_patch.as_ref() {
        evidence_ir.signal_alias_map.extend(
            patch
                .signal_alias_map
                .iter()
                .map(|(alias, signal_name)| (alias.clone(), signal_name.clone())),
        );
        if patch.refresh_signal_semantic_hints || !patch.signal_alias_map.is_empty() {
            evidence_ir.refresh_signal_semantic_hints()?;
        }
        evidence_ir
            .signal_constraints
            .extend(patch.signal_constraints.iter().cloned());
    }
    evidence_ir.write_to_disk()?;
    let evidence_report = if fixture
        .expectations
        .validation
        .as_ref()
        .and_then(|expectations| expectations.evidence.as_ref())
        .is_some()
    {
        validate::run_quiet(ValidateArgs {
            artifact: evidence_ir.artifact_layout.evidence_ir_path.clone(),
        })?;
        let reloaded = EvidenceIr::load_from_path(&evidence_ir.artifact_layout.evidence_ir_path)?;
        Some(latest_validation_report(
            &reloaded.validation_reports,
            "EvidenceIR",
            &evidence_ir.artifact_layout.evidence_ir_path,
        )?)
    } else {
        None
    };
    let semantic_ir = semantic::SemanticIr::build(
        &evidence_ir.artifact_layout.evidence_ir_path,
        &semantic_ir_root,
    )?;
    semantic_ir.write_to_disk()?;
    let intent_ir = intent::IntentIr::build(
        &semantic_ir.artifact_layout.semantic_ir_path,
        &intent_ir_root,
    )?;
    intent_ir.write_to_disk()?;

    let semantic_report = if fixture
        .expectations
        .validation
        .as_ref()
        .and_then(|expectations| expectations.semantic.as_ref())
        .is_some()
    {
        validate::run_quiet(ValidateArgs {
            artifact: semantic_ir.artifact_layout.semantic_ir_path.clone(),
        })?;
        let reloaded = SemanticIr::load_from_path(&semantic_ir.artifact_layout.semantic_ir_path)?;
        Some(latest_validation_report(
            &reloaded.validation_reports,
            "SemanticIR",
            &semantic_ir.artifact_layout.semantic_ir_path,
        )?)
    } else {
        None
    };
    let intent_report = if fixture
        .expectations
        .validation
        .as_ref()
        .and_then(|expectations| expectations.intent.as_ref())
        .is_some()
    {
        validate::run_quiet(ValidateArgs {
            artifact: intent_ir.artifact_layout.intent_ir_path.clone(),
        })?;
        let reloaded = IntentIr::load_from_path(&intent_ir.artifact_layout.intent_ir_path)?;
        Some(latest_validation_report(
            &reloaded.validation_reports,
            "IntentIR",
            &intent_ir.artifact_layout.intent_ir_path,
        )?)
    } else {
        None
    };

    let mut failures = Vec::new();
    if let Some(expectations) = fixture.expectations.semantic.as_ref() {
        evaluate_canonical_expectations(
            "semantic",
            expectations,
            &semantic_ir.interfaces,
            &semantic_ir.actor_signal_relations,
            &semantic_ir.actor_ports,
            &semantic_ir.infrastructure_signals,
            &semantic_ir.regular_states,
            &semantic_ir.state_transitions,
            &semantic_ir.residual_decisions,
            &[],
            &semantic_ir.temporal_rules,
            &semantic_ir.signal_polarity_conflicts,
            &semantic_ir.signal_semantic_conflicts,
            &semantic_ir.signal_connectivity_conflicts,
            &semantic_ir.interface_signal_conflicts,
            &semantic_ir.temporal_conflicts,
            &mut failures,
        );
    }
    if let Some(expectations) = fixture.expectations.intent.as_ref() {
        evaluate_canonical_expectations(
            "intent",
            expectations,
            &intent_ir.interfaces,
            &intent_ir.actor_signal_relations,
            &intent_ir.actor_ports,
            &intent_ir.infrastructure_signals,
            &intent_ir.regular_states,
            &intent_ir.state_transitions,
            &intent_ir.residual_decisions,
            &intent_ir.assumptions,
            &intent_ir.temporal_rules,
            &intent_ir.signal_polarity_conflicts,
            &intent_ir.signal_semantic_conflicts,
            &intent_ir.signal_connectivity_conflicts,
            &intent_ir.interface_signal_conflicts,
            &intent_ir.temporal_conflicts,
            &mut failures,
        );
    }
    if let Some(expectations) = fixture.expectations.validation.as_ref() {
        if let Some(stage) = expectations.evidence.as_ref() {
            evaluate_validation_expectations(
                "evidence_validation",
                stage,
                evidence_report
                    .as_ref()
                    .expect("evidence report should exist"),
                &mut failures,
            );
        }
        if let Some(stage) = expectations.semantic.as_ref() {
            evaluate_validation_expectations(
                "semantic_validation",
                stage,
                semantic_report
                    .as_ref()
                    .expect("semantic report should exist"),
                &mut failures,
            );
        }
        if let Some(stage) = expectations.intent.as_ref() {
            evaluate_validation_expectations(
                "intent_validation",
                stage,
                intent_report.as_ref().expect("intent report should exist"),
                &mut failures,
            );
        }
    }

    Ok(KgBenchFixtureOutcome {
        name: fixture.name,
        fixture_path: fixture_path.to_path_buf(),
        failures,
    })
}

#[expect(
    clippy::too_many_arguments,
    reason = "KG fixture evaluation keeps each canonical stage surface explicit for clearer failure messages"
)]
fn evaluate_canonical_expectations(
    label: &str,
    expectations: &CanonicalStageExpectations,
    interfaces: &[InterfaceRecord],
    actor_signal_relations: &[ActorSignalRelation],
    actor_ports: &[ActorPortRecord],
    infrastructure_signals: &[InfrastructureSignalRecord],
    regular_states: &[RegularStateRecord],
    state_transitions: &[StateTransitionRecord],
    residual_decisions: &[ResidualDecisionPacket],
    assumptions: &[IntentAssumption],
    temporal_rules: &[TemporalRuleRecord],
    signal_polarity_conflicts: &[SignalPolarityConflictRecord],
    signal_semantic_conflicts: &[SignalSemanticConflictRecord],
    signal_connectivity_conflicts: &[SignalConnectivityConflictRecord],
    interface_signal_conflicts: &[InterfaceSignalConflictRecord],
    temporal_conflicts: &[TemporalConflictRecord],
    failures: &mut Vec<String>,
) {
    let signal_names = interface_signal_names(interfaces);
    assert_includes(
        label,
        "signal_names_include",
        &expectations.signal_names_include,
        &signal_names,
        failures,
    );

    let graph_direction_signal_names = graph_direction_signal_names(actor_ports);
    assert_includes(
        label,
        "graph_direction_signal_names_include",
        &expectations.graph_direction_signal_names_include,
        &graph_direction_signal_names,
        failures,
    );
    assert_excludes(
        label,
        "graph_direction_signal_names_exclude",
        &expectations.graph_direction_signal_names_exclude,
        &graph_direction_signal_names,
        failures,
    );

    for expected_direction in &expectations.signal_directions_include {
        let Some(signal) = find_interface_signal(interfaces, &expected_direction.signal_name)
        else {
            failures.push(format!(
                "{label}: missing signal `{}` while checking direction expectation",
                expected_direction.signal_name
            ));
            continue;
        };
        if signal.direction_hint != Some(expected_direction.direction) {
            failures.push(format!(
                "{label}: expected signal `{}` direction `{}`, got `{}`",
                expected_direction.signal_name,
                interface_signal_direction_label(expected_direction.direction),
                signal
                    .direction_hint
                    .map(interface_signal_direction_label)
                    .unwrap_or("none")
            ));
        }
    }

    for expected_polarity in &expectations.signal_polarities_include {
        let Some(signal) = find_interface_signal(interfaces, &expected_polarity.signal_name) else {
            failures.push(format!(
                "{label}: missing signal `{}` while checking polarity expectation",
                expected_polarity.signal_name
            ));
            continue;
        };
        if signal.resolved_polarity != Some(expected_polarity.polarity) {
            failures.push(format!(
                "{label}: expected signal `{}` polarity `{}`, got `{}`",
                expected_polarity.signal_name,
                expected_polarity.polarity.as_str(),
                signal
                    .resolved_polarity
                    .map(SignalPolarity::as_str)
                    .unwrap_or("none")
            ));
        }
    }

    let resolved_role_signals = interface_signal_names_matching(interfaces, |signal| {
        signal.resolved_semantic_role.is_some()
    });
    assert_includes(
        label,
        "resolved_semantic_role_signal_names_include",
        &expectations.resolved_semantic_role_signal_names_include,
        &resolved_role_signals,
        failures,
    );
    assert_excludes(
        label,
        "resolved_semantic_role_signal_names_exclude",
        &expectations.resolved_semantic_role_signal_names_exclude,
        &resolved_role_signals,
        failures,
    );

    for expected_role in &expectations.resolved_semantic_roles_include {
        let Some(signal) = find_interface_signal(interfaces, &expected_role.signal_name) else {
            failures.push(format!(
                "{label}: missing signal `{}` while checking resolved semantic role expectation",
                expected_role.signal_name
            ));
            continue;
        };
        if signal.resolved_semantic_role != Some(expected_role.role) {
            failures.push(format!(
                "{label}: expected signal `{}` resolved semantic role `{}`, got `{}`",
                expected_role.signal_name,
                expected_role.role.as_str(),
                signal
                    .resolved_semantic_role
                    .map(InterfaceSignalSemanticRole::as_str)
                    .unwrap_or("none")
            ));
        }
    }

    for expected_strength in &expectations.semantic_grounding_strengths_include {
        let Some(signal) = find_interface_signal(interfaces, &expected_strength.signal_name) else {
            failures.push(format!(
                "{label}: missing signal `{}` while checking semantic grounding strength expectation",
                expected_strength.signal_name
            ));
            continue;
        };
        if signal.semantic_grounding_strength != Some(expected_strength.strength) {
            failures.push(format!(
                "{label}: expected signal `{}` semantic grounding strength `{}`, got `{}`",
                expected_strength.signal_name,
                expected_strength.strength.as_str(),
                signal
                    .semantic_grounding_strength
                    .map(SemanticGroundingStrength::as_str)
                    .unwrap_or("none")
            ));
        }
    }

    let semantic_consensus_signals =
        interface_signal_names_matching(interfaces, |signal| signal.semantic_consensus.is_some());
    assert_includes(
        label,
        "semantic_consensus_signal_names_include",
        &expectations.semantic_consensus_signal_names_include,
        &semantic_consensus_signals,
        failures,
    );
    assert_excludes(
        label,
        "semantic_consensus_signal_names_exclude",
        &expectations.semantic_consensus_signal_names_exclude,
        &semantic_consensus_signals,
        failures,
    );

    let alias_dependent_semantic_consensus_signals =
        interface_signal_names_matching(interfaces, |signal| {
            signal
                .semantic_consensus
                .as_ref()
                .is_some_and(|consensus| consensus.alias_dependent)
        });
    assert_includes(
        label,
        "alias_dependent_semantic_consensus_signal_names_include",
        &expectations.alias_dependent_semantic_consensus_signal_names_include,
        &alias_dependent_semantic_consensus_signals,
        failures,
    );
    assert_excludes(
        label,
        "alias_dependent_semantic_consensus_signal_names_exclude",
        &expectations.alias_dependent_semantic_consensus_signal_names_exclude,
        &alias_dependent_semantic_consensus_signals,
        failures,
    );

    let alias_dependent_semantic_candidate_signals =
        interface_signal_names_matching(interfaces, |signal| {
            signal
                .semantic_candidates
                .iter()
                .any(|candidate| candidate.alias_dependent)
        });
    assert_includes(
        label,
        "alias_dependent_semantic_candidate_signal_names_include",
        &expectations.alias_dependent_semantic_candidate_signal_names_include,
        &alias_dependent_semantic_candidate_signals,
        failures,
    );
    assert_excludes(
        label,
        "alias_dependent_semantic_candidate_signal_names_exclude",
        &expectations.alias_dependent_semantic_candidate_signal_names_exclude,
        &alias_dependent_semantic_candidate_signals,
        failures,
    );

    let semantic_candidate_signals = interface_signal_names_matching(interfaces, |signal| {
        !signal.semantic_candidates.is_empty()
    });
    assert_includes(
        label,
        "semantic_candidate_signal_names_include",
        &expectations.semantic_candidate_signal_names_include,
        &semantic_candidate_signals,
        failures,
    );
    assert_excludes(
        label,
        "semantic_candidate_signal_names_exclude",
        &expectations.semantic_candidate_signal_names_exclude,
        &semantic_candidate_signals,
        failures,
    );

    let multiple_semantic_candidate_signals =
        interface_signal_names_matching(interfaces, |signal| signal.semantic_candidates.len() > 1);
    assert_includes(
        label,
        "multiple_semantic_candidate_signal_names_include",
        &expectations.multiple_semantic_candidate_signal_names_include,
        &multiple_semantic_candidate_signals,
        failures,
    );
    assert_excludes(
        label,
        "multiple_semantic_candidate_signal_names_exclude",
        &expectations.multiple_semantic_candidate_signal_names_exclude,
        &multiple_semantic_candidate_signals,
        failures,
    );

    let semantic_arbitration_signals =
        interface_signal_names_matching(interfaces, |signal| signal.semantic_arbitration.is_some());
    assert_includes(
        label,
        "semantic_arbitration_signal_names_include",
        &expectations.semantic_arbitration_signal_names_include,
        &semantic_arbitration_signals,
        failures,
    );
    assert_excludes(
        label,
        "semantic_arbitration_signal_names_exclude",
        &expectations.semantic_arbitration_signal_names_exclude,
        &semantic_arbitration_signals,
        failures,
    );

    let decisive_semantic_arbitration_signals =
        interface_signal_names_matching(interfaces, |signal| {
            signal
                .semantic_arbitration
                .as_ref()
                .is_some_and(|arbitration| arbitration.decisive)
        });
    assert_includes(
        label,
        "decisive_semantic_arbitration_signal_names_include",
        &expectations.decisive_semantic_arbitration_signal_names_include,
        &decisive_semantic_arbitration_signals,
        failures,
    );
    assert_excludes(
        label,
        "decisive_semantic_arbitration_signal_names_exclude",
        &expectations.decisive_semantic_arbitration_signal_names_exclude,
        &decisive_semantic_arbitration_signals,
        failures,
    );

    let non_decisive_semantic_arbitration_signals =
        interface_signal_names_matching(interfaces, |signal| {
            signal
                .semantic_arbitration
                .as_ref()
                .is_some_and(|arbitration| !arbitration.decisive)
        });
    assert_includes(
        label,
        "non_decisive_semantic_arbitration_signal_names_include",
        &expectations.non_decisive_semantic_arbitration_signal_names_include,
        &non_decisive_semantic_arbitration_signals,
        failures,
    );
    assert_excludes(
        label,
        "non_decisive_semantic_arbitration_signal_names_exclude",
        &expectations.non_decisive_semantic_arbitration_signal_names_exclude,
        &non_decisive_semantic_arbitration_signals,
        failures,
    );

    for expected_port in &expectations.actor_ports_include {
        let found = actor_ports.iter().any(|port| {
            port.actor_name == expected_port.actor_name
                && port.signal_name == expected_port.signal_name
                && port.direction == expected_port.direction
        });
        if !found {
            failures.push(format!(
                "{label}: missing actor port `{}`:`{}`:`{}`",
                expected_port.actor_name,
                expected_port.signal_name,
                actor_relative_direction_label(expected_port.direction)
            ));
        }
    }

    for expected_relation in &expectations.actor_signal_relations_include {
        let found = actor_signal_relations.iter().any(|relation| {
            relation.actor_name == expected_relation.actor_name
                && relation.signal_name == expected_relation.signal_name
                && relation.relation == expected_relation.relation
        });
        if !found {
            failures.push(format!(
                "{label}: missing actor-signal relation `{}`:`{}`:`{}`",
                expected_relation.actor_name,
                expected_relation.signal_name,
                relation_kind_label(expected_relation.relation)
            ));
        }
    }

    for expected_infrastructure_signal in &expectations.infrastructure_signals_include {
        evaluate_expected_infrastructure_signal(
            label,
            expected_infrastructure_signal,
            infrastructure_signals,
            failures,
        );
    }

    for expected_topology in &expectations.infrastructure_topologies_include {
        evaluate_expected_infrastructure_topology(
            label,
            expected_topology,
            infrastructure_signals,
            failures,
        );
    }

    let state_names = regular_state_names(regular_states);
    assert_includes(
        label,
        "state_names_include",
        &expectations.state_names_include,
        &state_names,
        failures,
    );
    assert_excludes(
        label,
        "state_names_exclude",
        &expectations.state_names_exclude,
        &state_names,
        failures,
    );

    let initial_state_names = initial_regular_state_names(regular_states);
    assert_includes(
        label,
        "initial_state_names_include",
        &expectations.initial_state_names_include,
        &initial_state_names,
        failures,
    );
    assert_excludes(
        label,
        "initial_state_names_exclude",
        &expectations.initial_state_names_exclude,
        &initial_state_names,
        failures,
    );

    let state_transition_keys = state_transition_endpoint_keys(state_transitions);
    assert_includes(
        label,
        "state_transitions_include",
        &expected_state_transition_keys(&expectations.state_transitions_include),
        &state_transition_keys,
        failures,
    );
    assert_excludes(
        label,
        "state_transitions_exclude",
        &expected_state_transition_keys(&expectations.state_transitions_exclude),
        &state_transition_keys,
        failures,
    );

    let residual_ids = residual_decision_ids(residual_decisions);
    assert_includes(
        label,
        "residual_decision_ids_include",
        &expectations.residual_decision_ids_include,
        &residual_ids,
        failures,
    );
    assert_excludes(
        label,
        "residual_decision_ids_exclude",
        &expectations.residual_decision_ids_exclude,
        &residual_ids,
        failures,
    );

    let assumption_ids = assumption_id_set(assumptions);
    assert_includes(
        label,
        "assumption_ids_include",
        &expectations.assumption_ids_include,
        &assumption_ids,
        failures,
    );
    assert_excludes(
        label,
        "assumption_ids_exclude",
        &expectations.assumption_ids_exclude,
        &assumption_ids,
        failures,
    );

    assert_optional_count(
        label,
        "temporal_rule_count",
        expectations.temporal_rule_count,
        temporal_rules.len(),
        failures,
    );
    assert_optional_count(
        label,
        "temporal_rules_with_handshake_completion",
        expectations.temporal_rules_with_handshake_completion,
        temporal_rules_with_handshake_completion_count(temporal_rules),
        failures,
    );
    assert_optional_count(
        label,
        "temporal_rules_with_alias_dependent_handshake_completion",
        expectations.temporal_rules_with_alias_dependent_handshake_completion,
        temporal_rules_with_alias_dependent_handshake_completion_count(interfaces, temporal_rules),
        failures,
    );
    for expected_temporal_rule in &expectations.temporal_rules_include {
        evaluate_expected_temporal_rule(label, expected_temporal_rule, temporal_rules, failures);
    }
    assert_optional_count(
        label,
        "signal_polarity_conflicts",
        expectations.signal_polarity_conflicts,
        signal_polarity_conflicts.len(),
        failures,
    );
    for expected_signal_polarity_conflict in &expectations.signal_polarity_conflicts_include {
        evaluate_expected_signal_polarity_conflict(
            label,
            expected_signal_polarity_conflict,
            signal_polarity_conflicts,
            failures,
        );
    }
    assert_optional_count(
        label,
        "signal_semantic_conflicts",
        expectations.signal_semantic_conflicts,
        signal_semantic_conflicts.len(),
        failures,
    );
    for expected_signal_semantic_conflict in &expectations.signal_semantic_conflicts_include {
        evaluate_expected_signal_semantic_conflict(
            label,
            expected_signal_semantic_conflict,
            signal_semantic_conflicts,
            failures,
        );
    }
    assert_optional_count(
        label,
        "signal_connectivity_conflicts",
        expectations.signal_connectivity_conflicts,
        signal_connectivity_conflicts.len(),
        failures,
    );
    for expected_signal_connectivity_conflict in &expectations.signal_connectivity_conflicts_include
    {
        evaluate_expected_signal_connectivity_conflict(
            label,
            expected_signal_connectivity_conflict,
            signal_connectivity_conflicts,
            failures,
        );
    }
    assert_optional_count(
        label,
        "interface_signal_conflicts",
        expectations.interface_signal_conflicts,
        interface_signal_conflicts.len(),
        failures,
    );
    for expected_interface_signal_conflict in &expectations.interface_signal_conflicts_include {
        evaluate_expected_interface_signal_conflict(
            label,
            expected_interface_signal_conflict,
            interface_signal_conflicts,
            failures,
        );
    }
    assert_optional_count(
        label,
        "temporal_conflicts",
        expectations.temporal_conflicts,
        temporal_conflicts.len(),
        failures,
    );
    for expected_temporal_conflict in &expectations.temporal_conflicts_include {
        evaluate_expected_temporal_conflict(
            label,
            expected_temporal_conflict,
            temporal_conflicts,
            failures,
        );
    }
}

fn evaluate_expected_temporal_rule(
    label: &str,
    expectation: &ExpectedTemporalRule,
    temporal_rules: &[TemporalRuleRecord],
    failures: &mut Vec<String>,
) {
    let found = temporal_rules
        .iter()
        .any(|rule| temporal_rule_matches_expectation(rule, expectation));

    if !found {
        failures.push(format!(
            "{label}: missing temporal rule matching source_text `{:?}`, clock `{:?}`, edge `{:?}`, cycle_window `{:?}`, supporting ids {:?}, antecedents {:?}, and consequents {:?}",
            expectation.source_text,
            expectation.clock_signal,
            expectation.edge,
            expectation.cycle_window,
            expectation.supporting_statement_ids_include,
            expectation.antecedents_include,
            expectation.consequents_include
        ));
    }
}

fn temporal_rule_matches_expectation(
    rule: &TemporalRuleRecord,
    expectation: &ExpectedTemporalRule,
) -> bool {
    expectation
        .source_text
        .as_ref()
        .is_none_or(|source_text| rule.source_text == *source_text)
        && expectation
            .clock_signal
            .as_ref()
            .is_none_or(|clock_signal| rule.clock_signal.as_deref() == Some(clock_signal.as_str()))
        && expectation.edge.is_none_or(|edge| rule.edge == edge)
        && expectation
            .cycle_window
            .as_ref()
            .is_none_or(|cycle_window| rule.cycle_window.as_ref() == Some(cycle_window))
        && expectation
            .supporting_statement_ids_include
            .iter()
            .all(|statement_id| rule.supporting_statement_ids.contains(statement_id))
        && expectation
            .antecedents_include
            .iter()
            .all(|predicate| rule.antecedents.contains(predicate))
        && expectation
            .consequents_include
            .iter()
            .all(|predicate| rule.consequents.contains(predicate))
}

fn evaluate_expected_temporal_conflict(
    label: &str,
    expectation: &ExpectedTemporalConflict,
    temporal_conflicts: &[TemporalConflictRecord],
    failures: &mut Vec<String>,
) {
    let found = temporal_conflicts
        .iter()
        .any(|conflict| temporal_conflict_matches_expectation(conflict, expectation));

    if !found {
        failures.push(format!(
            "{label}: missing temporal conflict matching signal `{}`, phase `{:?}`, clock `{:?}`, edge `{:?}`, cycle_window `{:?}`, antecedents {:?}, values {:?}, rule ids {:?}, and statement ids {:?}",
            expectation.signal_name,
            expectation.phase,
            expectation.clock_signal,
            expectation.edge,
            expectation.cycle_window,
            expectation.antecedents_include,
            expectation.conflicting_values_include,
            expectation.supporting_rule_ids_include,
            expectation.supporting_statement_ids_include
        ));
    }
}

fn temporal_conflict_matches_expectation(
    conflict: &TemporalConflictRecord,
    expectation: &ExpectedTemporalConflict,
) -> bool {
    conflict.signal_name == expectation.signal_name
        && expectation
            .phase
            .is_none_or(|phase| conflict.phase == phase)
        && expectation
            .clock_signal
            .as_ref()
            .is_none_or(|clock_signal| {
                conflict.clock_signal.as_deref() == Some(clock_signal.as_str())
            })
        && expectation.edge.is_none_or(|edge| conflict.edge == edge)
        && expectation
            .cycle_window
            .as_ref()
            .is_none_or(|cycle_window| conflict.cycle_window.as_ref() == Some(cycle_window))
        && expectation
            .antecedents_include
            .iter()
            .all(|predicate| conflict.antecedents.contains(predicate))
        && expectation
            .conflicting_values_include
            .iter()
            .all(|value| conflict.conflicting_values.contains(value))
        && expectation
            .supporting_rule_ids_include
            .iter()
            .all(|rule_id| conflict.supporting_rule_ids.contains(rule_id))
        && expectation
            .supporting_statement_ids_include
            .iter()
            .all(|statement_id| conflict.supporting_statement_ids.contains(statement_id))
}

fn evaluate_expected_signal_polarity_conflict(
    label: &str,
    expectation: &ExpectedSignalPolarityConflict,
    signal_polarity_conflicts: &[SignalPolarityConflictRecord],
    failures: &mut Vec<String>,
) {
    let found = signal_polarity_conflicts
        .iter()
        .any(|conflict| signal_polarity_conflict_matches_expectation(conflict, expectation));

    if !found {
        failures.push(format!(
            "{label}: missing signal polarity conflict matching signal `{}`, conflict id `{:?}`, and observations {:?}",
            expectation.signal_name,
            expectation.conflict_id,
            expectation.observations_include
        ));
    }
}

fn signal_polarity_conflict_matches_expectation(
    conflict: &SignalPolarityConflictRecord,
    expectation: &ExpectedSignalPolarityConflict,
) -> bool {
    conflict.signal_name == expectation.signal_name
        && expectation
            .conflict_id
            .as_ref()
            .is_none_or(|conflict_id| conflict.conflict_id == *conflict_id)
        && expectation
            .observations_include
            .iter()
            .all(|expected_observation| {
                conflict.observations.iter().any(|observation| {
                    signal_polarity_conflict_observation_matches_expectation(
                        observation,
                        expected_observation,
                    )
                })
            })
}

fn signal_polarity_conflict_observation_matches_expectation(
    observation: &SignalPolarityObservationRecord,
    expectation: &ExpectedSignalPolarityConflictObservation,
) -> bool {
    expectation
        .polarity
        .is_none_or(|polarity| observation.polarity == polarity)
        && expectation
            .source_kind
            .is_none_or(|source_kind| observation.source_kind == source_kind)
        && expectation
            .supporting_statement_ids_include
            .iter()
            .all(|statement_id| observation.supporting_statement_ids.contains(statement_id))
        && expectation
            .supporting_table_ids_include
            .iter()
            .all(|table_id| observation.supporting_table_ids.contains(table_id))
}

fn evaluate_expected_signal_semantic_conflict(
    label: &str,
    expectation: &ExpectedSignalSemanticConflict,
    signal_semantic_conflicts: &[SignalSemanticConflictRecord],
    failures: &mut Vec<String>,
) {
    let found = signal_semantic_conflicts
        .iter()
        .any(|conflict| signal_semantic_conflict_matches_expectation(conflict, expectation));

    if !found {
        failures.push(format!(
            "{label}: missing signal semantic conflict matching signal `{}`, conflict id `{:?}`, and observations {:?}",
            expectation.signal_name, expectation.conflict_id, expectation.observations_include
        ));
    }
}

fn signal_semantic_conflict_matches_expectation(
    conflict: &SignalSemanticConflictRecord,
    expectation: &ExpectedSignalSemanticConflict,
) -> bool {
    conflict.signal_name == expectation.signal_name
        && expectation
            .conflict_id
            .as_ref()
            .is_none_or(|conflict_id| conflict.conflict_id == *conflict_id)
        && expectation
            .observations_include
            .iter()
            .all(|expected_observation| {
                conflict.observations.iter().any(|observation| {
                    signal_semantic_conflict_observation_matches_expectation(
                        observation,
                        expected_observation,
                    )
                })
            })
}

fn signal_semantic_conflict_observation_matches_expectation(
    observation: &SignalSemanticConflictObservationRecord,
    expectation: &ExpectedSignalSemanticConflictObservation,
) -> bool {
    expectation
        .semantic_tags_include
        .iter()
        .all(|tag| observation.semantic_tags.contains(tag))
        && expectation
            .source_kind
            .is_none_or(|source_kind| observation.source_kind == source_kind)
        && expectation
            .source_text
            .as_ref()
            .is_none_or(|source_text| observation.source_text == *source_text)
        && expectation
            .supporting_statement_ids_include
            .iter()
            .all(|statement_id| observation.supporting_statement_ids.contains(statement_id))
        && expectation
            .supporting_table_ids_include
            .iter()
            .all(|table_id| observation.supporting_table_ids.contains(table_id))
        && expectation
            .supporting_visual_evidence_ids_include
            .iter()
            .all(|visual_evidence_id| {
                observation
                    .supporting_visual_evidence_ids
                    .contains(visual_evidence_id)
            })
}

fn evaluate_expected_signal_connectivity_conflict(
    label: &str,
    expectation: &ExpectedSignalConnectivityConflict,
    signal_connectivity_conflicts: &[SignalConnectivityConflictRecord],
    failures: &mut Vec<String>,
) {
    let found = signal_connectivity_conflicts
        .iter()
        .any(|conflict| signal_connectivity_conflict_matches_expectation(conflict, expectation));

    if !found {
        failures.push(format!(
            "{label}: missing signal connectivity conflict matching signal `{}`, conflict id `{:?}`, kind `{:?}`, actor ids {:?}, actor names {:?}, and statement ids {:?}",
            expectation.signal_name,
            expectation.conflict_id,
            expectation.conflict_kind,
            expectation.conflicting_actor_ids_include,
            expectation.conflicting_actor_names_include,
            expectation.supporting_statement_ids_include
        ));
    }
}

fn signal_connectivity_conflict_matches_expectation(
    conflict: &SignalConnectivityConflictRecord,
    expectation: &ExpectedSignalConnectivityConflict,
) -> bool {
    conflict.signal_name == expectation.signal_name
        && expectation
            .conflict_id
            .as_ref()
            .is_none_or(|conflict_id| conflict.conflict_id == *conflict_id)
        && expectation
            .conflict_kind
            .is_none_or(|conflict_kind| conflict.conflict_kind == conflict_kind)
        && expectation
            .conflicting_actor_ids_include
            .iter()
            .all(|actor_id| conflict.conflicting_actor_ids.contains(actor_id))
        && expectation
            .conflicting_actor_names_include
            .iter()
            .all(|actor_name| conflict.conflicting_actor_names.contains(actor_name))
        && expectation
            .supporting_statement_ids_include
            .iter()
            .all(|statement_id| conflict.supporting_statement_ids.contains(statement_id))
}

fn evaluate_expected_interface_signal_conflict(
    label: &str,
    expectation: &ExpectedInterfaceSignalConflict,
    interface_signal_conflicts: &[InterfaceSignalConflictRecord],
    failures: &mut Vec<String>,
) {
    let found = interface_signal_conflicts
        .iter()
        .any(|conflict| interface_signal_conflict_matches_expectation(conflict, expectation));

    if !found {
        failures.push(format!(
            "{label}: missing interface signal conflict matching signal `{}`, conflict id `{:?}`, kind `{:?}`, and observations {:?}",
            expectation.signal_name,
            expectation.conflict_id,
            expectation.conflict_kind,
            expectation.observations_include
        ));
    }
}

fn interface_signal_conflict_matches_expectation(
    conflict: &InterfaceSignalConflictRecord,
    expectation: &ExpectedInterfaceSignalConflict,
) -> bool {
    conflict.signal_name == expectation.signal_name
        && expectation
            .conflict_id
            .as_ref()
            .is_none_or(|conflict_id| conflict.conflict_id == *conflict_id)
        && expectation
            .conflict_kind
            .is_none_or(|conflict_kind| conflict.conflict_kind == conflict_kind)
        && expectation
            .observations_include
            .iter()
            .all(|expected_observation| {
                conflict.observations.iter().any(|observation| {
                    interface_signal_conflict_observation_matches_expectation(
                        observation,
                        expected_observation,
                    )
                })
            })
}

fn interface_signal_conflict_observation_matches_expectation(
    observation: &InterfaceSignalConflictObservationRecord,
    expectation: &ExpectedInterfaceSignalConflictObservation,
) -> bool {
    observation.value_text == expectation.value_text
        && expectation
            .supporting_statement_ids_include
            .iter()
            .all(|statement_id| observation.supporting_statement_ids.contains(statement_id))
}

fn evaluate_expected_infrastructure_signal(
    label: &str,
    expectation: &ExpectedInfrastructureSignal,
    infrastructure_signals: &[InfrastructureSignalRecord],
    failures: &mut Vec<String>,
) {
    let Some(record) = infrastructure_signals
        .iter()
        .find(|record| record.signal_name == expectation.signal_name)
    else {
        failures.push(format!(
            "{label}: missing infrastructure signal `{}`",
            expectation.signal_name
        ));
        return;
    };

    if let Some(kind) = expectation.kind
        && record.kind != kind
    {
        failures.push(format!(
            "{label}: expected infrastructure signal `{}` kind `{}`, got `{}`",
            expectation.signal_name,
            infrastructure_signal_kind_label(kind),
            infrastructure_signal_kind_label(record.kind)
        ));
    }
    if let Some(source_status) = expectation.source_status
        && record.source_status != source_status
    {
        failures.push(format!(
            "{label}: expected infrastructure signal `{}` source status `{}`, got `{}`",
            expectation.signal_name,
            infrastructure_source_status_label(source_status),
            infrastructure_source_status_label(record.source_status)
        ));
    }
    if let Some(distribution_status) = expectation.distribution_status
        && record.distribution_status != distribution_status
    {
        failures.push(format!(
            "{label}: expected infrastructure signal `{}` distribution status `{}`, got `{}`",
            expectation.signal_name,
            infrastructure_distribution_status_label(distribution_status),
            infrastructure_distribution_status_label(record.distribution_status)
        ));
    }

    let recovered_sources = record
        .recovered_source_actor_names
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_includes(
        label,
        "infrastructure_signal.recovered_source_actor_names_include",
        &expectation.recovered_source_actor_names_include,
        &recovered_sources,
        failures,
    );

    let distributed_targets = record
        .distributed_to_actor_names
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>();
    assert_includes(
        label,
        "infrastructure_signal.distributed_to_actor_names_include",
        &expectation.distributed_to_actor_names_include,
        &distributed_targets,
        failures,
    );
}

fn evaluate_expected_infrastructure_topology(
    label: &str,
    expectation: &ExpectedInfrastructureTopology,
    infrastructure_signals: &[InfrastructureSignalRecord],
    failures: &mut Vec<String>,
) {
    let matching_signal_records = infrastructure_signals
        .iter()
        .filter(|record| record.signal_name == expectation.signal_name)
        .collect::<Vec<_>>();
    if matching_signal_records.is_empty() {
        failures.push(format!(
            "{label}: missing infrastructure signal `{}` while checking topology `{}`",
            expectation.signal_name,
            infrastructure_topology_kind_label(expectation.topology_kind)
        ));
        return;
    }

    let found = matching_signal_records
        .iter()
        .flat_map(|record| &record.infrastructure_topology)
        .any(|topology| {
            topology.topology_kind == expectation.topology_kind
                && expectation
                    .component_name
                    .as_ref()
                    .is_none_or(|component_name| {
                        topology.component_name.as_deref() == Some(component_name.as_str())
                    })
                && expectation
                    .stage_count
                    .is_none_or(|stage_count| topology.stage_count == Some(stage_count))
                && expectation
                    .target_actor_names_include
                    .iter()
                    .all(|actor_name| topology.target_actor_names.contains(actor_name))
        });

    if !found {
        failures.push(format!(
            "{label}: missing infrastructure topology `{}` for signal `{}` with component `{:?}`, stage_count `{:?}`, and targets {:?}",
            infrastructure_topology_kind_label(expectation.topology_kind),
            expectation.signal_name,
            expectation.component_name,
            expectation.stage_count,
            expectation.target_actor_names_include
        ));
    }
}

fn evaluate_validation_expectations(
    label: &str,
    expectations: &ValidationStageExpectations,
    report: &ValidationReportRecord,
    failures: &mut Vec<String>,
) {
    let finding_ids = validation_finding_ids(&report.findings);
    assert_includes(
        label,
        "finding_ids_include",
        &expectations.finding_ids_include,
        &finding_ids,
        failures,
    );
    assert_excludes(
        label,
        "finding_ids_exclude",
        &expectations.finding_ids_exclude,
        &finding_ids,
        failures,
    );

    for (metric_name, expected_value) in &expectations.metric_values {
        let actual_value = validation_metric_value(report, metric_name);
        match actual_value {
            Some(actual_value) if actual_value == expected_value => {}
            Some(actual_value) => failures.push(format!(
                "{label}: expected metric `{metric_name}` to equal `{expected_value}`, but actual value was `{actual_value}`"
            )),
            None => failures.push(format!(
                "{label}: expected metric `{metric_name}` to equal `{expected_value}`, but the metric was absent"
            )),
        }
    }
}

fn load_fixture(path: &Path) -> Result<KgBenchFixture> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

fn resolve_fixture_paths(fixtures_root: &Path, requested: &[PathBuf]) -> Result<Vec<PathBuf>> {
    let mut paths = if requested.is_empty() {
        discover_fixture_paths(fixtures_root)?
    } else {
        requested
            .iter()
            .map(|path| normalize_fixture_path(path, fixtures_root))
            .collect::<Vec<_>>()
    };

    if paths.is_empty() {
        return Err(AppError::InvalidStageArtifact(format!(
            "no KG benchmark fixtures found under {}",
            fixtures_root.display()
        )));
    }

    paths.sort();
    paths.dedup();
    Ok(paths)
}

fn discover_fixture_paths(root: &Path) -> Result<Vec<PathBuf>> {
    let mut fixtures = Vec::new();
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        if path.is_dir() {
            let fixture = path.join(FIXTURE_FILE_NAME);
            if fixture.exists() {
                fixtures.push(fixture);
                continue;
            }
            fixtures.extend(discover_fixture_paths(&path)?);
        } else if path.file_name().and_then(|name| name.to_str()) == Some(FIXTURE_FILE_NAME) {
            fixtures.push(path);
        }
    }
    Ok(fixtures)
}

fn normalize_fixture_path(path: &Path, fixtures_root: &Path) -> PathBuf {
    let absolute = normalize_input_path(path, fixtures_root);
    if absolute.is_dir() {
        absolute.join(FIXTURE_FILE_NAME)
    } else {
        absolute
    }
}

fn latest_validation_report(
    reports: &[ValidationReportRecord],
    stage_label: &str,
    artifact_path: &Path,
) -> Result<ValidationReportRecord> {
    reports.first().cloned().ok_or_else(|| {
        AppError::InvalidStageArtifact(format!(
            "{stage_label} artifact at {} does not carry a persisted validation report",
            artifact_path.display()
        ))
    })
}

fn interface_signal_names(interfaces: &[InterfaceRecord]) -> BTreeSet<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .map(|signal| signal.signal_name.clone())
        .collect()
}

fn interface_signal_names_matching(
    interfaces: &[InterfaceRecord],
    predicate: impl Fn(&crate::ir::semantic::InterfaceSignalRecord) -> bool,
) -> BTreeSet<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| predicate(signal))
        .map(|signal| signal.signal_name.clone())
        .collect()
}

fn find_interface_signal<'a>(
    interfaces: &'a [InterfaceRecord],
    signal_name: &str,
) -> Option<&'a crate::ir::semantic::InterfaceSignalRecord> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .find(|signal| signal.signal_name == signal_name)
}

fn graph_direction_signal_names(actor_ports: &[ActorPortRecord]) -> BTreeSet<String> {
    actor_ports
        .iter()
        .filter(|port| !matches!(port.direction, ActorRelativeDirection::Unknown))
        .map(|port| port.signal_name.clone())
        .collect()
}

fn regular_state_names(regular_states: &[RegularStateRecord]) -> BTreeSet<String> {
    regular_states
        .iter()
        .map(|state| state.state_name.clone())
        .collect()
}

fn initial_regular_state_names(regular_states: &[RegularStateRecord]) -> BTreeSet<String> {
    regular_states
        .iter()
        .filter(|state| state.is_initial)
        .map(|state| state.state_name.clone())
        .collect()
}

fn state_transition_endpoint_keys(state_transitions: &[StateTransitionRecord]) -> BTreeSet<String> {
    state_transitions
        .iter()
        .map(|transition| {
            state_transition_endpoint_key(&transition.source_state, &transition.target_state)
        })
        .collect()
}

fn expected_state_transition_keys(expected_transitions: &[ExpectedStateTransition]) -> Vec<String> {
    expected_transitions
        .iter()
        .map(|transition| {
            state_transition_endpoint_key(&transition.source_state, &transition.target_state)
        })
        .collect()
}

fn state_transition_endpoint_key(source_state: &str, target_state: &str) -> String {
    format!("{source_state}->{target_state}")
}

fn residual_decision_ids(residual_decisions: &[ResidualDecisionPacket]) -> BTreeSet<String> {
    residual_decisions
        .iter()
        .map(|packet| packet.packet_id.clone())
        .collect()
}

fn assumption_id_set(assumptions: &[IntentAssumption]) -> BTreeSet<String> {
    assumptions
        .iter()
        .map(|assumption| assumption.assumption_id.clone())
        .collect()
}

fn validation_finding_ids(findings: &[ValidationFindingRecord]) -> BTreeSet<String> {
    findings
        .iter()
        .map(|finding| finding.finding_id.clone())
        .collect()
}

fn validation_metric_value<'a>(
    report: &'a ValidationReportRecord,
    metric_name: &str,
) -> Option<&'a str> {
    report
        .metrics
        .iter()
        .find(|metric| metric.name == metric_name)
        .map(|metric| metric.value.as_str())
}

fn temporal_rules_with_handshake_completion_count(temporal_rules: &[TemporalRuleRecord]) -> usize {
    temporal_rules
        .iter()
        .filter(|rule| {
            rule.antecedents
                .iter()
                .chain(rule.consequents.iter())
                .any(|predicate| {
                    matches!(
                        predicate,
                        crate::ir::semantic::TemporalPredicateRecord::HandshakeComplete { .. }
                    )
                })
        })
        .count()
}

fn temporal_rules_with_alias_dependent_handshake_completion_count(
    interfaces: &[InterfaceRecord],
    temporal_rules: &[TemporalRuleRecord],
) -> usize {
    let alias_dependent_signals = interface_signal_names_matching(interfaces, |signal| {
        signal
            .semantic_consensus
            .as_ref()
            .is_some_and(|consensus| consensus.alias_dependent)
    });

    if alias_dependent_signals.is_empty() {
        return 0;
    }

    temporal_rules
        .iter()
        .filter(|rule| {
            rule.antecedents
                .iter()
                .chain(rule.consequents.iter())
                .any(|predicate| {
                    matches!(
                        predicate,
                        crate::ir::semantic::TemporalPredicateRecord::HandshakeComplete {
                            valid_signal,
                            ready_signal,
                            ..
                        } if alias_dependent_signals.contains(valid_signal)
                            || alias_dependent_signals.contains(ready_signal)
                    )
                })
        })
        .count()
}

fn assert_includes(
    label: &str,
    field: &str,
    expected: &[String],
    actual: &BTreeSet<String>,
    failures: &mut Vec<String>,
) {
    for value in expected {
        if !actual.contains(value) {
            failures.push(format!(
                "{label}: expected `{field}` to include `{value}`, but actual set was {:?}",
                actual
            ));
        }
    }
}

fn assert_excludes(
    label: &str,
    field: &str,
    excluded: &[String],
    actual: &BTreeSet<String>,
    failures: &mut Vec<String>,
) {
    for value in excluded {
        if actual.contains(value) {
            failures.push(format!(
                "{label}: expected `{field}` to exclude `{value}`, but actual set was {:?}",
                actual
            ));
        }
    }
}

fn assert_optional_count(
    label: &str,
    field: &str,
    expected: Option<usize>,
    actual: usize,
    failures: &mut Vec<String>,
) {
    if let Some(expected) = expected
        && actual != expected
    {
        failures.push(format!(
            "{label}: expected `{field}` = {expected}, got {actual}"
        ));
    }
}

fn actor_relative_direction_label(direction: ActorRelativeDirection) -> &'static str {
    match direction {
        ActorRelativeDirection::Input => "input",
        ActorRelativeDirection::Output => "output",
        ActorRelativeDirection::InOut => "in_out",
        ActorRelativeDirection::Unknown => "unknown",
    }
}

fn interface_signal_direction_label(direction: InterfaceSignalDirection) -> &'static str {
    match direction {
        InterfaceSignalDirection::Input => "input",
        InterfaceSignalDirection::Output => "output",
        InterfaceSignalDirection::Internal => "internal",
    }
}

fn infrastructure_signal_kind_label(kind: InfrastructureSignalKind) -> &'static str {
    match kind {
        InfrastructureSignalKind::SystemClock => "system_clock",
        InfrastructureSignalKind::SystemReset => "system_reset",
    }
}

fn infrastructure_source_status_label(status: InfrastructureSignalSourceStatus) -> &'static str {
    match status {
        InfrastructureSignalSourceStatus::UnresolvedSource => "unresolved_source",
        InfrastructureSignalSourceStatus::RecoveredProducer => "recovered_producer",
        InfrastructureSignalSourceStatus::MultipleRecoveredProducers => {
            "multiple_recovered_producers"
        }
    }
}

fn infrastructure_distribution_status_label(
    status: InfrastructureSignalDistributionStatus,
) -> &'static str {
    match status {
        InfrastructureSignalDistributionStatus::NoRecoveredConsumers => "no_recovered_consumers",
        InfrastructureSignalDistributionStatus::SingleRecoveredConsumer => {
            "single_recovered_consumer"
        }
        InfrastructureSignalDistributionStatus::SharedRecoveredConsumers => {
            "shared_recovered_consumers"
        }
    }
}

fn infrastructure_topology_kind_label(kind: InfrastructureTopologyKind) -> &'static str {
    match kind {
        InfrastructureTopologyKind::ClockGatedBranch => "clock_gated_branch",
        InfrastructureTopologyKind::ResetSynchronizerStages => "reset_synchronizer_stages",
        InfrastructureTopologyKind::ResetTreeTargets => "reset_tree_targets",
    }
}

fn relation_kind_label(relation: RelationKind) -> &'static str {
    match relation {
        RelationKind::Drives => "drives",
        RelationKind::Reads => "reads",
    }
}

fn normalize_input_path(path: &Path, base_root: &Path) -> PathBuf {
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_root.join(path)
    }
}

fn write_fixture_prior_memory(generated_root: &Path, patch: &PriorMemoryPatch) -> Result<PathBuf> {
    let prior_memory_path = generated_root
        .join("prior_memory")
        .join("corpus_memory.json");
    if let Some(parent) = prior_memory_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let corpus_memory = CorpusMemory {
        schema_version: 5,
        update_policy: CorpusMemoryUpdatePolicyRecord {
            advisory_only: true,
            requires_validated_intent_ir: true,
            rejects_error_findings: true,
            excludes_alias_dependent_semantic_consensus: true,
            local_grounding_required_for_canonical_promotion: true,
        },
        source_artifacts: vec![PriorSourceArtifactRecord {
            artifact_path: generated_root.join("fixture_seed_intent_ir.json"),
            document_key: "fixture_seed".to_string(),
            display_name: "fixture_seed".to_string(),
            protocol_family: crate::ir::prior_memory::ProtocolFamily::Unknown,
            overall_score: Some(100),
            grade: Some("EXCELLENT".to_string()),
            accepted_for_learning: true,
            skip_reason: None,
        }],
        actor_taxonomy_priors: patch.actor_taxonomy_priors.clone(),
        semantic_phrase_priors: patch.semantic_phrase_priors.clone(),
        semantic_modality_reliability_priors: patch.semantic_modality_reliability_priors.clone(),
        temporal_phrase_priors: patch.temporal_phrase_priors.clone(),
        table_shape_priors: patch.table_shape_priors.clone(),
        visual_motif_priors: patch.visual_motif_priors.clone(),
        negative_knowledge_priors: patch.negative_knowledge_priors.clone(),
    };
    fs::write(
        &prior_memory_path,
        serde_json::to_string_pretty(&corpus_memory)?,
    )?;
    Ok(prior_memory_path)
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use super::run;
    use crate::cli::KgBenchArgs;
    use crate::error::AppError;

    #[test]
    fn kg_bench_runs_tracked_fixtures() -> crate::error::Result<()> {
        let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("kg_quality");
        run(KgBenchArgs {
            fixtures_root,
            fixtures: Vec::new(),
        })
    }

    #[test]
    fn kg_bench_reports_fixture_failure() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("broken_fixture");
        fs::create_dir_all(&fixture_dir)?;
        fs::write(
            fixture_dir.join("source.md"),
            "# Interface\nSignal AWVALID is input width 1.\n",
        )?;
        fs::write(
            fixture_dir.join("fixture.json"),
            serde_json::json!({
                "name": "broken_fixture",
                "source": "source.md",
                "expectations": {
                    "semantic": {
                        "resolved_semantic_role_signal_names_include": ["AWVALID"]
                    }
                }
            })
            .to_string(),
        )?;

        let error = run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })
        .expect_err("expected kg-bench to fail for mismatched fixture");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("broken_fixture"));
                assert!(message.contains("resolved_semantic_role_signal_names_include"));
            }
            other => panic!("unexpected error variant: {other}"),
        }

        Ok(())
    }
}
