use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

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
    ExtractionProfilePriorRecord, NegativeKnowledgePriorRecord, PriorSourceArtifactRecord,
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
    ValidationFindingSeverity, ValidationReportRecord,
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
    semantic_ir_patch: Option<SemanticIrPatch>,
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
    /// EXTRACTION-QUALITY-GAUGE.0: stage a persisted NLI extraction-quality gauge so tracked
    /// fixtures can lock the provider-free `validate` reporting surface (metrics + findings)
    /// without any live LLM dependency.
    #[serde(default)]
    extraction_quality_gauge: Option<crate::ir::evidence::ExtractionQualityGaugeRecord>,
}

#[derive(Debug, Default, Deserialize)]
struct SemanticIrPatch {
    #[serde(default)]
    actor_ports_append: Vec<ActorPortRecord>,
    #[serde(default)]
    clear_actor_ports: bool,
    #[serde(default)]
    clear_signal_direction_hints: Vec<String>,
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
    #[serde(default)]
    extraction_profile_priors: Vec<ExtractionProfilePriorRecord>,
}

#[derive(Debug, Default, Deserialize)]
struct FixtureExpectations {
    #[serde(default)]
    evidence: Option<EvidenceStageExpectations>,
    #[serde(default)]
    semantic: Option<CanonicalStageExpectations>,
    #[serde(default)]
    intent: Option<CanonicalStageExpectations>,
    #[serde(default)]
    validation: Option<ValidationExpectations>,
}

#[derive(Debug, Default, Deserialize)]
struct EvidenceStageExpectations {
    table_signal_declaration_provenance_count: Option<usize>,
    #[serde(default)]
    table_signal_declaration_provenance_include: Vec<ExpectedTableSignalDeclarationProvenance>,
    /// EXTRACTION-QUALITY-GAUGE.FIELD.2 — lock the typed message-field inventory.
    message_field_count: Option<usize>,
    #[serde(default)]
    message_fields_include: Vec<ExpectedMessageField>,
    #[serde(default)]
    message_field_names_exclude: Vec<String>,
    /// PDF-VARIANT-DIGESTION.12b — lock the typed signal-presence inventory.
    signal_presence_count: Option<usize>,
    #[serde(default)]
    signal_presence_include: Vec<ExpectedSignalPresence>,
    #[serde(default)]
    signal_presence_signal_names_exclude: Vec<String>,
}

#[derive(Debug, Default, Deserialize)]
struct CanonicalStageExpectations {
    #[serde(default)]
    signal_names_include: Vec<String>,
    #[serde(default)]
    signal_names_exclude: Vec<String>,
    #[serde(default)]
    graph_direction_signal_names_include: Vec<String>,
    #[serde(default)]
    graph_direction_signal_names_exclude: Vec<String>,
    #[serde(default)]
    graph_direction_conflicted_signal_names_include: Vec<String>,
    #[serde(default)]
    graph_direction_conflicted_signal_names_exclude: Vec<String>,
    #[serde(default)]
    graph_direction_conflicts_include: Vec<ExpectedGraphDirectionConflict>,
    #[serde(default)]
    signal_directions_include: Vec<ExpectedSignalDirection>,
    #[serde(default)]
    signal_supporting_table_ids_include: Vec<ExpectedSignalTableSupport>,
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
struct ExpectedGraphDirectionConflict {
    signal_name: String,
    actor_name: String,
    #[serde(default)]
    actor_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedSignalTableSupport {
    signal_name: String,
    #[serde(default)]
    table_ids_include: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedTableSignalDeclarationProvenance {
    signal_name: String,
    table_id: String,
    #[serde(default)]
    statement_text: Option<String>,
}

/// EXTRACTION-QUALITY-GAUGE.FIELD.2 — one expected MESSAGE FIELD. Identity is (container, name);
/// `bit_width` locks a declared width when given, and `bit_width_absent` locks the honest-absence
/// behavior (an unstated or variant-dependent width must stay `None`, never guessed).
#[derive(Debug, Deserialize)]
struct ExpectedMessageField {
    name: String,
    container: String,
    #[serde(default)]
    bit_width: Option<u32>,
    #[serde(default)]
    bit_width_absent: bool,
}

/// PDF-VARIANT-DIGESTION.12b — one expected signal-presence row: the literal signal name, an
/// optional literal condition expression (or an explicit honest-absence assertion), and the
/// literal (variant label, code) entries the captured row must carry.
#[derive(Debug, Deserialize)]
struct ExpectedSignalPresence {
    signal: String,
    #[serde(default)]
    condition: Option<String>,
    #[serde(default)]
    condition_absent: bool,
    #[serde(default)]
    variants: Vec<ExpectedVariantPresence>,
}

#[derive(Debug, Deserialize)]
struct ExpectedVariantPresence {
    label: String,
    code: String,
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
    source: Option<ValidationStageExpectations>,
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
    findings_include: Vec<ExpectedValidationFinding>,
    #[serde(default)]
    metric_values: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize)]
struct ExpectedValidationFinding {
    finding_id: String,
    #[serde(default)]
    severity: Option<ValidationFindingSeverity>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    summary_contains: Option<String>,
    #[serde(default)]
    related_ids_include: Vec<String>,
    #[serde(default)]
    related_ids_exclude: Vec<String>,
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

    let tempdir = crate::project_data::tempdir()?;
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
    let source_report = if fixture
        .expectations
        .validation
        .as_ref()
        .and_then(|expectations| expectations.source.as_ref())
        .is_some()
    {
        validate::run_quiet(ValidateArgs {
            artifact: source_ir.artifact_layout.source_ir_path.clone(),
        })?;
        let reloaded = source::SourceIr::load_from_path(&source_ir.artifact_layout.source_ir_path)?;
        Some(latest_validation_report(
            &reloaded.validation_reports,
            "SourceIR",
            &source_ir.artifact_layout.source_ir_path,
        )?)
    } else {
        None
    };
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
        if let Some(gauge) = patch.extraction_quality_gauge.as_ref() {
            evidence_ir.extraction_quality_gauge = Some(gauge.clone());
        }
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
    let mut semantic_ir = semantic::SemanticIr::build(
        &evidence_ir.artifact_layout.evidence_ir_path,
        &semantic_ir_root,
    )?;
    if let Some(patch) = fixture.semantic_ir_patch.as_ref() {
        if patch.clear_actor_ports {
            semantic_ir.actor_ports.clear();
        }
        semantic_ir
            .actor_ports
            .extend(patch.actor_ports_append.iter().cloned());
        if !patch.clear_signal_direction_hints.is_empty() {
            let signal_names: BTreeSet<&str> = patch
                .clear_signal_direction_hints
                .iter()
                .map(String::as_str)
                .collect();
            for interface in &mut semantic_ir.interfaces {
                for signal in &mut interface.signal_records {
                    if signal_names.contains(signal.signal_name.as_str()) {
                        signal.direction_hint = None;
                    }
                }
            }
        }
    }
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
    if let Some(expectations) = fixture
        .expectations
        .validation
        .as_ref()
        .and_then(|expectations| expectations.source.as_ref())
        && let Some(report) = source_report.as_ref()
    {
        evaluate_validation_expectations("validation.source", expectations, report, &mut failures);
    }
    if let Some(expectations) = fixture.expectations.evidence.as_ref() {
        evaluate_evidence_expectations("evidence", expectations, &evidence_ir, &mut failures);
    }
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

fn evaluate_evidence_expectations(
    label: &str,
    expectations: &EvidenceStageExpectations,
    evidence_ir: &EvidenceIr,
    failures: &mut Vec<String>,
) {
    assert_optional_count(
        label,
        "table_signal_declaration_provenance_count",
        expectations.table_signal_declaration_provenance_count,
        evidence_ir.table_signal_declaration_provenance.len(),
        failures,
    );

    for expectation in &expectations.table_signal_declaration_provenance_include {
        let matching_records = evidence_ir
            .table_signal_declaration_provenance
            .iter()
            .filter(|record| {
                record.signal_name == expectation.signal_name
                    && record.table_id == expectation.table_id
            })
            .collect::<Vec<_>>();

        if matching_records.is_empty() {
            failures.push(format!(
                "{label}: expected `table_signal_declaration_provenance_include` to contain signal `{}` from table `{}`, but actual provenance was {:?}",
                expectation.signal_name,
                expectation.table_id,
                evidence_ir.table_signal_declaration_provenance
            ));
            continue;
        }

        if let Some(statement_text) = expectation.statement_text.as_deref() {
            let has_matching_statement_text = matching_records.iter().any(|record| {
                evidence_ir.extracted_statements.iter().any(|statement| {
                    statement.statement_id == record.statement_id
                        && statement.text == statement_text
                })
            });

            if !has_matching_statement_text {
                let actual_statement_texts = matching_records
                    .iter()
                    .filter_map(|record| {
                        evidence_ir
                            .extracted_statements
                            .iter()
                            .find(|statement| statement.statement_id == record.statement_id)
                            .map(|statement| statement.text.clone())
                    })
                    .collect::<Vec<_>>();
                failures.push(format!(
                    "{label}: expected `table_signal_declaration_provenance_include` for signal `{}` from table `{}` to reference synthesized statement text `{}`, but matching records referenced {:?}",
                    expectation.signal_name,
                    expectation.table_id,
                    statement_text,
                    actual_statement_texts
                ));
            }
        }
    }

    // EXTRACTION-QUALITY-GAUGE.FIELD.2 — the typed message-field inventory.
    assert_optional_count(
        label,
        "message_field_count",
        expectations.message_field_count,
        evidence_ir.message_field_records.len(),
        failures,
    );
    for expectation in &expectations.message_fields_include {
        let record = evidence_ir
            .message_field_records
            .iter()
            .find(|r| r.name == expectation.name && r.container == expectation.container);
        let Some(record) = record else {
            failures.push(format!(
                "{label}: expected `message_fields_include` to contain field `{}` in container `{}`, but actual fields were {:?}",
                expectation.name,
                expectation.container,
                evidence_ir
                    .message_field_records
                    .iter()
                    .map(|r| format!("{}::{}", r.container, r.name))
                    .collect::<Vec<_>>()
            ));
            continue;
        };
        if let Some(expected_width) = expectation.bit_width
            && record.bit_width != Some(expected_width)
        {
            failures.push(format!(
                "{label}: expected message field `{}` in `{}` to declare bit_width {expected_width}, got {:?}",
                expectation.name, expectation.container, record.bit_width
            ));
        }
        if expectation.bit_width_absent && record.bit_width.is_some() {
            failures.push(format!(
                "{label}: expected message field `{}` in `{}` to carry NO bit_width (honest absence), got {:?}",
                expectation.name, expectation.container, record.bit_width
            ));
        }
    }
    let message_field_names: BTreeSet<String> = evidence_ir
        .message_field_records
        .iter()
        .map(|r| r.name.clone())
        .collect();
    assert_excludes(
        label,
        "message_field_names_exclude",
        &expectations.message_field_names_exclude,
        &message_field_names,
        failures,
    );

    // PDF-VARIANT-DIGESTION.12b — the typed signal-presence inventory.
    assert_optional_count(
        label,
        "signal_presence_count",
        expectations.signal_presence_count,
        evidence_ir.signal_presence_records.len(),
        failures,
    );
    for expectation in &expectations.signal_presence_include {
        let candidates: Vec<_> = evidence_ir
            .signal_presence_records
            .iter()
            .filter(|r| r.signal_name == expectation.signal)
            .collect();
        let record = candidates.iter().find(|record| {
            let condition_ok = match (&expectation.condition, expectation.condition_absent) {
                (Some(expected), _) => record.presence_condition.as_deref() == Some(expected),
                (None, true) => record.presence_condition.is_none(),
                (None, false) => true,
            };
            condition_ok
                && expectation.variants.iter().all(|expected| {
                    record.variant_presence.iter().any(|entry| {
                        entry.variant_label == expected.label && entry.code == expected.code
                    })
                })
        });
        if record.is_none() {
            failures.push(format!(
                "{label}: expected `signal_presence_include` to contain signal `{}` (condition {:?}{}, variants {:?}), but actual rows for that signal were {:?}",
                expectation.signal,
                expectation.condition,
                if expectation.condition_absent {
                    ", required absent"
                } else {
                    ""
                },
                expectation
                    .variants
                    .iter()
                    .map(|v| format!("{}={}", v.label, v.code))
                    .collect::<Vec<_>>(),
                candidates
                    .iter()
                    .map(|r| format!(
                        "condition {:?}, variants {:?}",
                        r.presence_condition,
                        r.variant_presence
                            .iter()
                            .map(|v| format!("{}={}", v.variant_label, v.code))
                            .collect::<Vec<_>>()
                    ))
                    .collect::<Vec<_>>()
            ));
        }
    }
    let signal_presence_names: BTreeSet<String> = evidence_ir
        .signal_presence_records
        .iter()
        .map(|r| r.signal_name.clone())
        .collect();
    assert_excludes(
        label,
        "signal_presence_signal_names_exclude",
        &expectations.signal_presence_signal_names_exclude,
        &signal_presence_names,
        failures,
    );
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
    assert_excludes(
        label,
        "signal_names_exclude",
        &expectations.signal_names_exclude,
        &signal_names,
        failures,
    );

    let graph_direction_signal_names = validate::graph_direction_signal_names(actor_ports);
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
    let graph_direction_conflicted_signal_names =
        validate::graph_direction_conflicted_signal_names(actor_ports);
    assert_includes(
        label,
        "graph_direction_conflicted_signal_names_include",
        &expectations.graph_direction_conflicted_signal_names_include,
        &graph_direction_conflicted_signal_names,
        failures,
    );
    assert_excludes(
        label,
        "graph_direction_conflicted_signal_names_exclude",
        &expectations.graph_direction_conflicted_signal_names_exclude,
        &graph_direction_conflicted_signal_names,
        failures,
    );
    let graph_direction_conflicts = validate::graph_direction_conflicts(actor_ports);
    for expected_conflict in &expectations.graph_direction_conflicts_include {
        evaluate_expected_graph_direction_conflict(
            label,
            expected_conflict,
            &graph_direction_conflicts,
            failures,
        );
    }

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

    for expected_table_support in &expectations.signal_supporting_table_ids_include {
        let Some(signal) = find_interface_signal(interfaces, &expected_table_support.signal_name)
        else {
            failures.push(format!(
                "{label}: missing signal `{}` while checking `signal_supporting_table_ids_include[{}]`; actual signals were {:?}",
                expected_table_support.signal_name,
                expected_table_support.signal_name,
                signal_names
            ));
            continue;
        };
        let actual_table_ids: BTreeSet<String> =
            signal.supporting_table_ids.iter().cloned().collect();
        assert_includes(
            label,
            &format!(
                "signal_supporting_table_ids_include[{}]",
                expected_table_support.signal_name
            ),
            &expected_table_support.table_ids_include,
            &actual_table_ids,
            failures,
        );
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

fn evaluate_expected_graph_direction_conflict(
    label: &str,
    expectation: &ExpectedGraphDirectionConflict,
    conflicts: &[validate::GraphDirectionConflictRecord],
    failures: &mut Vec<String>,
) {
    let found = conflicts.iter().any(|conflict| {
        conflict.signal_name == expectation.signal_name
            && conflict.actor_name == expectation.actor_name
            && expectation
                .actor_id
                .as_ref()
                .is_none_or(|actor_id| conflict.actor_id == *actor_id)
    });

    if !found {
        failures.push(format!(
            "{label}: missing graph-direction conflict matching signal `{}`, actor `{}`, and actor_id `{:?}`",
            expectation.signal_name, expectation.actor_name, expectation.actor_id
        ));
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

    for expectation in &expectations.findings_include {
        let matching_findings = report
            .findings
            .iter()
            .filter(|finding| finding.finding_id == expectation.finding_id)
            .collect::<Vec<_>>();

        if matching_findings.is_empty() {
            failures.push(format!(
                "{label}: expected `findings_include` to contain finding `{}`, but actual finding ids were {:?}",
                expectation.finding_id, finding_ids
            ));
            continue;
        }

        if !matching_findings
            .iter()
            .any(|finding| validation_finding_matches_expectation(finding, expectation))
        {
            failures.push(format!(
                "{label}: expected `findings_include` finding `{}` to satisfy {:?}, but matching findings were {:?}",
                expectation.finding_id, expectation, matching_findings
            ));
        }
    }

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

fn validation_finding_matches_expectation(
    finding: &ValidationFindingRecord,
    expectation: &ExpectedValidationFinding,
) -> bool {
    if let Some(expected_severity) = expectation.severity
        && finding.severity != expected_severity
    {
        return false;
    }
    if let Some(expected_category) = expectation.category.as_deref()
        && finding.category != expected_category
    {
        return false;
    }
    if let Some(expected_summary) = expectation.summary_contains.as_deref()
        && !finding.summary.contains(expected_summary)
    {
        return false;
    }
    if expectation
        .related_ids_include
        .iter()
        .any(|expected| !finding.related_ids.contains(expected))
    {
        return false;
    }
    if expectation
        .related_ids_exclude
        .iter()
        .any(|excluded| finding.related_ids.contains(excluded))
    {
        return false;
    }

    true
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
        schema_version: 6,
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
        extraction_profile_priors: patch.extraction_profile_priors.clone(),
    };
    fs::write(&prior_memory_path, corpus_memory.to_pretty_json()?)?;
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

    use super::{
        CanonicalStageExpectations, ExpectedValidationFinding, ValidationStageExpectations,
        discover_fixture_paths, evaluate_canonical_expectations, evaluate_validation_expectations,
        load_fixture, run,
    };
    use crate::cli::KgBenchArgs;
    use crate::error::AppError;
    use crate::ir::IrStage;
    use crate::ir::semantic::{ActorPortRecord, ActorRelativeDirection};
    use crate::ir::source::{
        AutomationConfidence, ValidationFindingRecord, ValidationFindingSeverity,
        ValidationReportRecord,
    };

    fn actor_port(
        actor_name: &str,
        signal_name: &str,
        direction: ActorRelativeDirection,
    ) -> ActorPortRecord {
        ActorPortRecord {
            actor_id: format!("actor_{}", actor_name.to_ascii_lowercase()),
            actor_name: actor_name.to_string(),
            signal_name: signal_name.to_string(),
            direction,
            relation_basis: Vec::new(),
            width_hint: None,
            source_statement_ids: Vec::new(),
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    fn write_one_signal_table_fixture(
        fixture_dir: &Path,
        fixture_name: &str,
        evidence_expectations: serde_json::Value,
    ) -> crate::error::Result<()> {
        write_one_signal_table_fixture_with_expectations(
            fixture_dir,
            fixture_name,
            serde_json::json!({
                "evidence": evidence_expectations
            }),
        )
    }

    fn write_one_signal_table_fixture_with_expectations(
        fixture_dir: &Path,
        fixture_name: &str,
        expectations: serde_json::Value,
    ) -> crate::error::Result<()> {
        fs::create_dir_all(fixture_dir)?;
        fs::write(
            fixture_dir.join("source.md"),
            "# Interface\nThe protocol signals are defined by the signal table.\n",
        )?;

        let table_cell = |text: &str, is_header: bool| {
            serde_json::json!({
                "text": text,
                "row_span": 1,
                "col_span": 1,
                "is_header": is_header
            })
        };

        fs::write(
            fixture_dir.join("fixture.json"),
            serde_json::json!({
                "name": fixture_name,
                "source": "source.md",
                "source_ir_patch": {
                    "structured_tables": [
                        {
                            "table_id": "table_protocol_signal_description",
                            "asset_id": "table_protocol_signal_description",
                            "page_id": null,
                            "caption_text": "Protocol signal descriptions",
                            "source_ref": null,
                            "table_kind": "signal_description",
                            "header_rows": [[
                                table_cell("Signal", true),
                                table_cell("Direction", true),
                                table_cell("Width", true),
                                table_cell("Description", true)
                            ]],
                            "body_rows": [[
                                table_cell("XREQ", false),
                                table_cell("output", false),
                                table_cell("1", false),
                                table_cell("Request indication.", false)
                            ]],
                            "row_count": 1,
                            "col_count": 4
                        }
                    ]
                },
                "expectations": expectations
            })
            .to_string(),
        )?;

        Ok(())
    }

    #[test]
    fn canonical_expectations_exclude_conflicting_same_actor_graph_direction() {
        let expectations = CanonicalStageExpectations {
            graph_direction_signal_names_exclude: vec!["PREADY".to_string()],
            graph_direction_conflicted_signal_names_include: vec!["PREADY".to_string()],
            graph_direction_conflicted_signal_names_exclude: vec!["PADDR".to_string()],
            graph_direction_conflicts_include: vec![super::ExpectedGraphDirectionConflict {
                signal_name: "PREADY".to_string(),
                actor_name: "Completer".to_string(),
                actor_id: None,
            }],
            ..CanonicalStageExpectations::default()
        };
        let actor_ports = vec![
            actor_port("Completer", "PREADY", ActorRelativeDirection::Output),
            actor_port("Completer", "PREADY", ActorRelativeDirection::Input),
            actor_port("Requester", "PADDR", ActorRelativeDirection::Output),
            actor_port("Completer", "PADDR", ActorRelativeDirection::Input),
        ];
        let mut failures = Vec::new();

        evaluate_canonical_expectations(
            "semantic",
            &expectations,
            &[],
            &[],
            &actor_ports,
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &[],
            &mut failures,
        );

        assert!(
            failures.is_empty(),
            "conflicting same-actor graph direction should be reflected in graph-direction expectation surfaces: {failures:?}"
        );
    }

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
    fn tracked_count_expectations_are_locked_as_validation_metrics() -> crate::error::Result<()> {
        let fixtures_root = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("test_data")
            .join("kg_quality");
        let mut failures = Vec::new();

        for fixture_path in discover_fixture_paths(&fixtures_root)? {
            let fixture = load_fixture(&fixture_path)?;
            assert_count_metrics_for_stage(
                &fixture.name,
                &fixture_path,
                "semantic",
                fixture.expectations.semantic.as_ref(),
                fixture
                    .expectations
                    .validation
                    .as_ref()
                    .and_then(|validation| validation.semantic.as_ref()),
                &mut failures,
            );
            assert_count_metrics_for_stage(
                &fixture.name,
                &fixture_path,
                "intent",
                fixture.expectations.intent.as_ref(),
                fixture
                    .expectations
                    .validation
                    .as_ref()
                    .and_then(|validation| validation.intent.as_ref()),
                &mut failures,
            );
        }

        assert!(
            failures.is_empty(),
            "tracked KG fixtures with canonical count-style expectations must lock matching validation metric_values:\n{}",
            failures.join("\n")
        );
        Ok(())
    }

    fn assert_count_metrics_for_stage(
        fixture_name: &str,
        fixture_path: &Path,
        stage_label: &str,
        canonical: Option<&CanonicalStageExpectations>,
        validation: Option<&ValidationStageExpectations>,
        failures: &mut Vec<String>,
    ) {
        let Some(canonical) = canonical else {
            return;
        };
        let expected_metrics = count_style_metric_expectations(canonical);
        if expected_metrics.is_empty() {
            return;
        }

        let Some(validation) = validation else {
            failures.push(format!(
                "{} ({fixture_name}) {stage_label}: missing validation expectations for count-style metrics",
                fixture_path.display()
            ));
            return;
        };

        for (metric_name, expected_value) in expected_metrics {
            match validation.metric_values.get(metric_name) {
                Some(actual_value) if actual_value == &expected_value => {}
                Some(actual_value) => failures.push(format!(
                    "{} ({fixture_name}) {stage_label}: metric `{metric_name}` expected `{expected_value}` but fixture locks `{actual_value}`",
                    fixture_path.display()
                )),
                None => failures.push(format!(
                    "{} ({fixture_name}) {stage_label}: missing validation metric `{metric_name}` with value `{expected_value}`",
                    fixture_path.display()
                )),
            }
        }
    }

    fn count_style_metric_expectations(
        canonical: &CanonicalStageExpectations,
    ) -> Vec<(&'static str, String)> {
        let mut expected = Vec::new();
        push_count_metric(
            &mut expected,
            "temporal_rules",
            canonical.temporal_rule_count,
        );
        push_count_metric(
            &mut expected,
            "temporal_rules_with_handshake_completion",
            canonical.temporal_rules_with_handshake_completion,
        );
        push_count_metric(
            &mut expected,
            "temporal_rules_with_alias_dependent_handshake_completion",
            canonical.temporal_rules_with_alias_dependent_handshake_completion,
        );
        push_count_metric(
            &mut expected,
            "signal_polarity_conflicts",
            canonical.signal_polarity_conflicts,
        );
        push_count_metric(
            &mut expected,
            "signal_semantic_conflicts",
            canonical.signal_semantic_conflicts,
        );
        push_count_metric(
            &mut expected,
            "signal_connectivity_conflicts",
            canonical.signal_connectivity_conflicts,
        );
        push_count_metric(
            &mut expected,
            "interface_signal_conflicts",
            canonical.interface_signal_conflicts,
        );
        push_count_metric(
            &mut expected,
            "temporal_conflicts",
            canonical.temporal_conflicts,
        );
        expected
    }

    fn push_count_metric(
        expected: &mut Vec<(&'static str, String)>,
        metric_name: &'static str,
        value: Option<usize>,
    ) {
        if let Some(value) = value {
            expected.push((metric_name, value.to_string()));
        }
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

    #[test]
    fn kg_bench_reports_validation_finding_related_id_failure() {
        let report = ValidationReportRecord {
            report_id: "validation_report_test".to_string(),
            validated_stage: IrStage::SemanticIr,
            artifact_fingerprint: "fingerprint".to_string(),
            summary: "test report".to_string(),
            overall_score: Some(100),
            grade: Some("EXCELLENT".to_string()),
            metrics: Vec::new(),
            findings: vec![ValidationFindingRecord {
                finding_id: "semantic_negative_knowledge_prior_matches".to_string(),
                severity: ValidationFindingSeverity::Info,
                category: "negative_knowledge".to_string(),
                summary: "1 carried conflict/residual pattern(s) match prior negative knowledge"
                    .to_string(),
                related_ids: vec!["semantic_conflict_0001".to_string()],
            }],
        };
        let expectations = ValidationStageExpectations {
            findings_include: vec![ExpectedValidationFinding {
                finding_id: "semantic_negative_knowledge_prior_matches".to_string(),
                severity: Some(ValidationFindingSeverity::Info),
                category: Some("negative_knowledge".to_string()),
                summary_contains: Some("match prior negative knowledge".to_string()),
                related_ids_include: vec!["missing_conflict".to_string()],
                related_ids_exclude: Vec::new(),
            }],
            ..ValidationStageExpectations::default()
        };
        let mut failures = Vec::new();

        evaluate_validation_expectations(
            "semantic_validation",
            &expectations,
            &report,
            &mut failures,
        );

        assert_eq!(failures.len(), 1);
        assert!(failures[0].contains("findings_include"));
        assert!(failures[0].contains("missing_conflict"));
        assert!(failures[0].contains("semantic_conflict_0001"));
    }

    #[test]
    fn kg_bench_supports_semantic_actor_port_patch_for_graph_direction_conflicts()
    -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("graph_direction_conflict_fixture");
        fs::create_dir_all(&fixture_dir)?;
        fs::write(
            fixture_dir.join("source.md"),
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "Signal PADDR is input width 32.\n",
                "\n",
                "The Completer drives PREADY.\n",
                "\n",
                "The Requester reads PREADY.\n",
                "\n",
                "The Requester drives PADDR.\n",
                "\n",
                "The Completer samples PADDR.\n"
            ),
        )?;
        fs::write(
            fixture_dir.join("fixture.json"),
            serde_json::json!({
                "name": "graph_direction_conflict_fixture",
                "source": "source.md",
                "semantic_ir_patch": {
                    "actor_ports_append": [
                        {
                            "actor_id": "actor_completer",
                            "actor_name": "Completer",
                            "signal_name": "PREADY",
                            "direction": "input",
                            "relation_basis": [],
                            "source_statement_ids": [],
                            "automation_confidence": "medium"
                        }
                    ]
                },
                "expectations": {
                    "semantic": {
                        "graph_direction_signal_names_include": ["PADDR"],
                        "graph_direction_signal_names_exclude": ["PREADY"]
                    },
                    "intent": {
                        "graph_direction_signal_names_include": ["PADDR"],
                        "graph_direction_signal_names_exclude": ["PREADY"]
                    },
                    "validation": {
                        "semantic": {
                            "finding_ids_include": ["semantic_graph_direction_conflicts_present"],
                            "findings_include": [
                                {
                                    "finding_id": "semantic_graph_direction_conflicts_present",
                                    "severity": "warning",
                                    "category": "knowledge_graph",
                                    "summary_contains": "conflicting actor-relative directions from the same actor",
                                    "related_ids_include": ["graph_direction_conflict:actor_completer:PREADY"]
                                }
                            ],
                            "metric_values": {
                                "graph_direction_conflicts": "1"
                            }
                        },
                        "intent": {
                            "finding_ids_include": ["intent_graph_direction_conflicts_present"],
                            "findings_include": [
                                {
                                    "finding_id": "intent_graph_direction_conflicts_present",
                                    "severity": "warning",
                                    "category": "knowledge_graph",
                                    "summary_contains": "conflicting actor-relative directions from the same actor",
                                    "related_ids_include": ["graph_direction_conflict:actor_completer:PREADY"]
                                }
                            ],
                            "metric_values": {
                                "graph_direction_conflicts": "1"
                            }
                        }
                    }
                }
            })
            .to_string(),
        )?;

        run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })?;

        Ok(())
    }

    #[test]
    fn kg_bench_supports_semantic_direction_hint_clear_patch() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("compat_direction_hint_gap_fixture");
        fs::create_dir_all(&fixture_dir)?;
        fs::write(
            fixture_dir.join("source.md"),
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "Signal PADDR is input width 32.\n",
                "\n",
                "The Completer drives PREADY.\n",
                "\n",
                "The Requester reads PREADY.\n",
                "\n",
                "The Requester drives PADDR.\n",
                "\n",
                "The Completer samples PADDR.\n"
            ),
        )?;
        fs::write(
            fixture_dir.join("fixture.json"),
            serde_json::json!({
                "name": "compat_direction_hint_gap_fixture",
                "source": "source.md",
                "semantic_ir_patch": {
                    "clear_signal_direction_hints": ["PREADY", "PADDR"]
                },
                "expectations": {
                    "validation": {
                        "semantic": {
                            "finding_ids_include": ["semantic_compat_direction_hints_lag_graph"],
                            "findings_include": [
                                {
                                    "finding_id": "semantic_compat_direction_hints_lag_graph",
                                    "severity": "info",
                                    "category": "compatibility_surface",
                                    "summary_contains": "2 interface signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist",
                                    "related_ids_include": ["PADDR", "PREADY"]
                                }
                            ],
                            "metric_values": {
                                "with_graph_direction": "2",
                                "with_compat_direction_hint": "0"
                            }
                        },
                        "intent": {
                            "finding_ids_include": ["intent_compat_direction_hints_lag_graph"],
                            "findings_include": [
                                {
                                    "finding_id": "intent_compat_direction_hints_lag_graph",
                                    "severity": "info",
                                    "category": "compatibility_surface",
                                    "summary_contains": "2 declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist",
                                    "related_ids_include": ["PADDR", "PREADY"]
                                }
                            ],
                            "metric_values": {
                                "with_graph_direction": "2",
                                "with_compat_direction_hint": "0"
                            }
                        }
                    }
                }
            })
            .to_string(),
        )?;

        run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })?;

        Ok(())
    }

    #[test]
    fn kg_bench_reports_canonical_table_support_failure() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("canonical_table_support_fixture");
        write_one_signal_table_fixture_with_expectations(
            &fixture_dir,
            "canonical_table_support_fixture",
            serde_json::json!({
                "semantic": {
                    "signal_supporting_table_ids_include": [
                        {
                            "signal_name": "XREQ",
                            "table_ids_include": ["missing_signal_table"]
                        }
                    ]
                }
            }),
        )?;

        let error = run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })
        .expect_err("expected kg-bench to fail for missing canonical table support");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("canonical_table_support_fixture"));
                assert!(message.contains("signal_supporting_table_ids_include[XREQ]"));
                assert!(message.contains("missing_signal_table"));
                assert!(message.contains("table_protocol_signal_description"));
            }
            other => panic!("unexpected error variant: {other}"),
        }

        Ok(())
    }

    #[test]
    fn kg_bench_reports_missing_canonical_table_support_failure() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("missing_canonical_signal_fixture");
        write_one_signal_table_fixture_with_expectations(
            &fixture_dir,
            "missing_canonical_signal_fixture",
            serde_json::json!({
                "semantic": {
                    "signal_supporting_table_ids_include": [
                        {
                            "signal_name": "MISSING_SIGNAL",
                            "table_ids_include": ["table_protocol_signal_description"]
                        }
                    ]
                }
            }),
        )?;

        let error = run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })
        .expect_err("expected kg-bench to fail for missing canonical signal");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("missing_canonical_signal_fixture"));
                assert!(message.contains("semantic"));
                assert!(message.contains("signal_supporting_table_ids_include[MISSING_SIGNAL]"));
                assert!(message.contains("MISSING_SIGNAL"));
                assert!(message.contains("XREQ"));
            }
            other => panic!("unexpected error variant: {other}"),
        }

        Ok(())
    }

    #[test]
    fn kg_bench_reports_missing_intent_table_support_failure() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("missing_intent_signal_fixture");
        write_one_signal_table_fixture_with_expectations(
            &fixture_dir,
            "missing_intent_signal_fixture",
            serde_json::json!({
                "intent": {
                    "signal_supporting_table_ids_include": [
                        {
                            "signal_name": "MISSING_INTENT_SIGNAL",
                            "table_ids_include": ["table_protocol_signal_description"]
                        }
                    ]
                }
            }),
        )?;

        let error = run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })
        .expect_err("expected kg-bench to fail for missing IntentIR signal");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("missing_intent_signal_fixture"));
                assert!(message.contains("intent"));
                assert!(
                    message.contains("signal_supporting_table_ids_include[MISSING_INTENT_SIGNAL]")
                );
                assert!(message.contains("MISSING_INTENT_SIGNAL"));
                assert!(message.contains("XREQ"));
            }
            other => panic!("unexpected error variant: {other}"),
        }

        Ok(())
    }

    #[test]
    fn kg_bench_reports_intent_table_support_failure() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("intent_table_support_fixture");
        write_one_signal_table_fixture_with_expectations(
            &fixture_dir,
            "intent_table_support_fixture",
            serde_json::json!({
                "intent": {
                    "signal_supporting_table_ids_include": [
                        {
                            "signal_name": "XREQ",
                            "table_ids_include": ["missing_intent_signal_table"]
                        }
                    ]
                }
            }),
        )?;

        let error = run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })
        .expect_err("expected kg-bench to fail for missing IntentIR table support");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("intent_table_support_fixture"));
                assert!(message.contains("intent"));
                assert!(message.contains("signal_supporting_table_ids_include[XREQ]"));
                assert!(message.contains("missing_intent_signal_table"));
                assert!(message.contains("table_protocol_signal_description"));
            }
            other => panic!("unexpected error variant: {other}"),
        }

        Ok(())
    }

    #[test]
    fn kg_bench_reports_evidence_table_provenance_count_failure() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("count_evidence_fixture");
        write_one_signal_table_fixture(
            &fixture_dir,
            "count_evidence_fixture",
            serde_json::json!({
                "table_signal_declaration_provenance_count": 0
            }),
        )?;

        let error = run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })
        .expect_err("expected kg-bench to fail for mismatched EvidenceIR provenance count");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("count_evidence_fixture"));
                assert!(message.contains("table_signal_declaration_provenance_count"));
                assert!(
                    message.contains(
                        "expected `table_signal_declaration_provenance_count` = 0, got 1"
                    )
                );
            }
            other => panic!("unexpected error variant: {other}"),
        }

        Ok(())
    }

    #[test]
    fn kg_bench_reports_evidence_table_provenance_statement_failure() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("broken_evidence_fixture");
        write_one_signal_table_fixture(
            &fixture_dir,
            "broken_evidence_fixture",
            serde_json::json!({
                "table_signal_declaration_provenance_include": [
                    {
                        "signal_name": "XREQ",
                        "table_id": "table_protocol_signal_description",
                        "statement_text": "Signal XREQ is output width 8."
                    }
                ]
            }),
        )?;

        let error = run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })
        .expect_err("expected kg-bench to fail for mismatched EvidenceIR provenance");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("broken_evidence_fixture"));
                assert!(message.contains("table_signal_declaration_provenance_include"));
                assert!(message.contains("Signal XREQ is output width 8."));
                assert!(message.contains("Signal XREQ is output width 1."));
            }
            other => panic!("unexpected error variant: {other}"),
        }

        Ok(())
    }

    #[test]
    fn kg_bench_reports_missing_evidence_table_provenance_failure() -> crate::error::Result<()> {
        let tempdir = tempdir()?;
        let fixture_dir = tempdir.path().join("missing_evidence_fixture");
        write_one_signal_table_fixture(
            &fixture_dir,
            "missing_evidence_fixture",
            serde_json::json!({
                "table_signal_declaration_provenance_include": [
                    {
                        "signal_name": "XREQ",
                        "table_id": "missing_signal_table"
                    }
                ]
            }),
        )?;

        let error = run(KgBenchArgs {
            fixtures_root: tempdir.path().to_path_buf(),
            fixtures: Vec::new(),
        })
        .expect_err("expected kg-bench to fail for missing EvidenceIR provenance");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("missing_evidence_fixture"));
                assert!(message.contains("table_signal_declaration_provenance_include"));
                assert!(message.contains("missing_signal_table"));
                assert!(message.contains("table_protocol_signal_description"));
            }
            other => panic!("unexpected error variant: {other}"),
        }

        Ok(())
    }
}
