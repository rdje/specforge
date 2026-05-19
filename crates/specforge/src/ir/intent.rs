use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::{
    SignalPolarityConflictRecord, SignalPolarityRecord, SignalSemanticConflictRecord,
};
use crate::ir::semantic::{
    ActorPortRecord, ConditionalRuleRecord, ControlActionRecord, ControlBinaryOperator,
    ControlBlockRecord, ControlCompoundUpdateOperation, ControlExpressionRecord,
    ExplicitModuleRecord, ExplicitTopRecord, InfrastructureSignalRecord, InterfaceRecord,
    InterfaceSignalConflictRecord, RegisterRecord, RegularStateRecord, SemanticIr,
    SignalConnectivityConflictRecord, SignalConnectivityRecord, SignalConstraintRecord,
    StateTransitionRecord, SymbolDefinitionRecord, SystemContractRecord, TemporalConflictRecord,
    TemporalPredicateRecord, TemporalRuleRecord, TimingConstraintRecord,
};
use crate::ir::source::{
    ActorSignalRelation, AutomationConfidence, CandidateInterpretation, RelationKind,
    ResidualDecisionPacket, SignalConstraintKind, ValidationReportRecord, document_key,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub semantic_ir_path: PathBuf,
    pub artifact_layout: IntentArtifactLayout,
    pub document_identity: IntentDocumentIdentity,
    pub intent_identity: IntentIdentity,
    pub actors: Vec<IntentActor>,
    #[serde(default)]
    pub actor_signal_relations: Vec<ActorSignalRelation>,
    #[serde(default)]
    pub actor_ports: Vec<ActorPortRecord>,
    #[serde(default)]
    pub signal_connectivity: Vec<SignalConnectivityRecord>,
    #[serde(default)]
    pub infrastructure_signals: Vec<InfrastructureSignalRecord>,
    #[serde(default)]
    pub interface_signal_conflicts: Vec<InterfaceSignalConflictRecord>,
    #[serde(default)]
    pub signal_connectivity_conflicts: Vec<SignalConnectivityConflictRecord>,
    #[serde(default)]
    pub signal_polarities: Vec<SignalPolarityRecord>,
    #[serde(default)]
    pub signal_polarity_conflicts: Vec<SignalPolarityConflictRecord>,
    #[serde(default)]
    pub signal_semantic_conflicts: Vec<SignalSemanticConflictRecord>,
    #[serde(default)]
    pub interfaces: Vec<InterfaceRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_contract: Option<SystemContractRecord>,
    pub behaviors: Vec<BehaviorIntent>,
    pub constraints: Vec<IntentConstraint>,
    pub assumptions: Vec<IntentAssumption>,
    #[serde(default)]
    pub regular_states: Vec<RegularStateRecord>,
    #[serde(default)]
    pub state_transitions: Vec<StateTransitionRecord>,
    #[serde(default)]
    pub symbol_definitions: Vec<SymbolDefinitionRecord>,
    #[serde(default)]
    pub control_blocks: Vec<ControlBlockRecord>,
    #[serde(default)]
    pub explicit_modules: Vec<ExplicitModuleRecord>,
    #[serde(default)]
    pub explicit_tops: Vec<ExplicitTopRecord>,
    /// Register map records carried forward from `SemanticIR`.
    #[serde(default)]
    pub register_records: Vec<RegisterRecord>,
    /// Timing constraint records carried forward from `SemanticIR`.
    #[serde(default)]
    pub timing_constraints: Vec<TimingConstraintRecord>,
    /// Clock-tick temporal rules carried forward from `SemanticIR`.
    #[serde(default)]
    pub temporal_rules: Vec<TemporalRuleRecord>,
    /// ContractIR — typed timed-contract projection (R16-CONTRACT-IR),
    /// carried forward from `SemanticIR`. Named `actor_contracts` to
    /// avoid collision with `SemanticIR.contracts` (protocol contracts).
    /// Additive and empty until `R16-CONTRACT-IR.3` (serde-skipped while
    /// empty).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_contracts: Vec<crate::ir::contract::ActorContract>,
    /// Explicit conflicts detected across contradictory temporal value obligations.
    #[serde(default)]
    pub temporal_conflicts: Vec<TemporalConflictRecord>,
    /// Level 2 NLP: signal constraint records carried forward from `SemanticIR`.
    #[serde(default)]
    pub signal_constraints: Vec<SignalConstraintRecord>,
    /// Level 2 NLP: conditional rule records carried forward from `SemanticIR`.
    #[serde(default)]
    pub conditional_rules: Vec<ConditionalRuleRecord>,
    /// Transaction intents — ordered behavioral steps extracted from PDF text.
    #[serde(default)]
    pub transactions: Vec<TransactionIntent>,
    /// Actor drive relations — which actor drives which signal to which consumer.
    #[serde(default)]
    pub actor_drive_relations: Vec<ActorDriveRelationRecord>,
    /// Actor sample relations — which actor samples which signal and when.
    #[serde(default)]
    pub actor_sample_relations: Vec<ActorSampleRelationRecord>,
    /// Actor trigger relations — activation paths between actors.
    #[serde(default)]
    pub actor_trigger_relations: Vec<ActorTriggerRelationRecord>,
    /// Actor temporal dependencies — wait-for relationships between actors.
    #[serde(default)]
    pub actor_temporal_dependencies: Vec<ActorTemporalDependencyRecord>,
    /// Temporal invariants — constraints that must always hold.
    #[serde(default)]
    pub temporal_invariants: Vec<TemporalInvariantRecord>,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
    #[serde(default)]
    pub validation_reports: Vec<ValidationReportRecord>,
}

impl IntentIr {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(AppError::MissingPath(path.to_path_buf()));
        }

        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn build(semantic_ir_path: &Path, artifact_base_root: &Path) -> Result<Self> {
        let semantic_ir_path = canonicalize_existing_path(semantic_ir_path)?;
        let semantic_ir = SemanticIr::load_from_path(&semantic_ir_path)?;

        if !matches!(semantic_ir.stage, IrStage::SemanticIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} must be a SemanticIR document before building IntentIR",
                semantic_ir_path.display()
            )));
        }

        let artifact_root = artifact_base_root.join(&semantic_ir.document_identity.document_key);
        let intent_ir_path = artifact_root.join("intent_ir.json");
        let artifact_layout = IntentArtifactLayout {
            artifact_root,
            intent_ir_path,
        };
        let document_identity = IntentDocumentIdentity {
            document_key: semantic_ir.document_identity.document_key.clone(),
            display_name: semantic_ir.document_identity.display_name.clone(),
        };

        let context = IntentContext::from_semantic_ir(&semantic_ir);
        let actor_signal_relations = semantic_ir.actor_signal_relations.clone();
        let actor_ports = semantic_ir.actor_ports.clone();
        let signal_connectivity = semantic_ir.signal_connectivity.clone();
        let infrastructure_signals = semantic_ir.infrastructure_signals.clone();
        let interface_signal_conflicts = semantic_ir.interface_signal_conflicts.clone();
        let signal_connectivity_conflicts = semantic_ir.signal_connectivity_conflicts.clone();
        let signal_polarities = semantic_ir.signal_polarities.clone();
        let signal_polarity_conflicts = semantic_ir.signal_polarity_conflicts.clone();
        let signal_semantic_conflicts = semantic_ir.signal_semantic_conflicts.clone();
        let interfaces = semantic_ir.interfaces.clone();
        let system_contract = semantic_ir.system_contract.clone();
        let actors = build_intent_actors(&context);
        let actor_ids = actors.iter().map(|actor| actor.actor_id.clone()).collect();
        let behaviors = build_behaviors(&context, actor_ids);
        let constraints = build_constraints(&context);
        let assumptions = build_assumptions(&context, &actors);
        let regular_states = semantic_ir.regular_states.clone();
        let state_transitions = semantic_ir.state_transitions.clone();
        let symbol_definitions = semantic_ir.symbol_definitions.clone();
        let control_blocks = semantic_ir.control_blocks.clone();
        let explicit_modules = semantic_ir.explicit_modules.clone();
        let explicit_tops = semantic_ir.explicit_tops.clone();
        let register_records = semantic_ir.register_records.clone();
        let timing_constraints = semantic_ir.timing_constraints.clone();
        let temporal_rules = semantic_ir.temporal_rules.clone();
        // R16-CONTRACT-IR.3: carry the typed ContractIR forward.
        let actor_contracts = semantic_ir.actor_contracts.clone();
        let temporal_conflicts = semantic_ir.temporal_conflicts.clone();
        let signal_constraints = semantic_ir.signal_constraints.clone();
        let conditional_rules = semantic_ir.conditional_rules.clone();
        let mut transactions = synthesize_transactions(&semantic_ir);
        let mut actor_drive_relations = synthesize_actor_drive_relations(&semantic_ir);
        let mut actor_sample_relations = synthesize_actor_sample_relations(&semantic_ir);
        let mut actor_trigger_relations = synthesize_actor_trigger_relations(&semantic_ir);
        let mut actor_temporal_dependencies = synthesize_actor_temporal_dependencies(&semantic_ir);
        let mut temporal_invariants = synthesize_temporal_invariants(&semantic_ir);
        recognize_digital_patterns(
            &mut transactions,
            &mut actor_drive_relations,
            &mut actor_sample_relations,
            &mut actor_trigger_relations,
            &mut actor_temporal_dependencies,
            &mut temporal_invariants,
            &semantic_ir,
        );
        let residual_decisions =
            build_residual_decisions(&context, &actors, &behaviors, &constraints);
        let intent_identity = build_intent_identity(
            &document_identity,
            &actors,
            &interfaces,
            system_contract.as_ref(),
            &behaviors,
            &constraints,
            &regular_states,
            &state_transitions,
            &symbol_definitions,
            &control_blocks,
            &explicit_modules,
            &explicit_tops,
        );

        Ok(Self {
            schema_version: 1,
            stage: IrStage::IntentIr,
            semantic_ir_path,
            artifact_layout,
            document_identity,
            intent_identity,
            actors,
            actor_signal_relations,
            actor_ports,
            signal_connectivity,
            infrastructure_signals,
            interface_signal_conflicts,
            signal_connectivity_conflicts,
            signal_polarities,
            signal_polarity_conflicts,
            signal_semantic_conflicts,
            interfaces,
            system_contract,
            behaviors,
            constraints,
            assumptions,
            regular_states,
            state_transitions,
            symbol_definitions,
            control_blocks,
            explicit_modules,
            explicit_tops,
            register_records,
            timing_constraints,
            temporal_rules,
            actor_contracts,
            temporal_conflicts,
            signal_constraints,
            conditional_rules,
            transactions,
            actor_drive_relations,
            actor_sample_relations,
            actor_trigger_relations,
            actor_temporal_dependencies,
            temporal_invariants,
            residual_decisions,
            validation_reports: Vec::new(),
        })
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn write_to_disk(&self) -> Result<()> {
        fs::create_dir_all(&self.artifact_layout.artifact_root)?;
        fs::write(&self.artifact_layout.intent_ir_path, self.to_pretty_json()?)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentArtifactLayout {
    pub artifact_root: PathBuf,
    pub intent_ir_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentDocumentIdentity {
    pub document_key: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentIdentity {
    pub intent_id: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentActor {
    pub actor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_name: Option<String>,
    pub responsibilities: Vec<String>,
    pub supporting_actor_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BehaviorIntent {
    pub behavior_id: String,
    pub statement: String,
    pub actor_ids: Vec<String>,
    pub supporting_semantic_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentConstraint {
    pub constraint_id: String,
    pub statement: String,
    pub related_interface_ids: Vec<String>,
    pub supporting_semantic_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IntentAssumption {
    pub assumption_id: String,
    pub statement: String,
    pub supporting_semantic_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct IntentContext {
    semantic_actors: Vec<SemanticActorContext>,
    phases: Vec<PhaseContext>,
    invariants: Vec<ConstraintSourceContext>,
    assertions: Vec<ConstraintSourceContext>,
    contracts: Vec<ContractContext>,
    gates: Vec<GateContext>,
    abstractions: Vec<AbstractionContext>,
    residual_decisions: Vec<ResidualDecisionPacket>,
}

impl IntentContext {
    fn from_semantic_ir(semantic_ir: &SemanticIr) -> Self {
        let semantic_actors = semantic_ir
            .actors
            .iter()
            .map(|actor| SemanticActorContext {
                actor_id: actor.actor_id.clone(),
                actor_name: actor.actor_name.clone(),
                role_summary: actor.role_summary.clone(),
                supporting_statement_ids: actor.supporting_statement_ids.clone(),
                supporting_section_ids: actor.supporting_section_ids.clone(),
            })
            .collect();
        let phases = semantic_ir
            .phases
            .iter()
            .map(|phase| PhaseContext {
                phase_id: phase.phase_id.clone(),
                summary: phase.summary.clone(),
                supporting_statement_ids: phase.supporting_statement_ids.clone(),
                supporting_section_ids: phase.supporting_section_ids.clone(),
            })
            .collect();
        let invariants = semantic_ir
            .invariants
            .iter()
            .map(|invariant| ConstraintSourceContext {
                source_id: invariant.invariant_id.clone(),
                statement: invariant.statement.clone(),
                related_interface_ids: invariant.related_interface_ids.clone(),
            })
            .collect();
        let assertions = semantic_ir
            .assertions
            .iter()
            .map(|assertion| ConstraintSourceContext {
                source_id: assertion.assertion_id.clone(),
                statement: assertion.statement.clone(),
                related_interface_ids: Vec::new(),
            })
            .collect();
        let contracts = semantic_ir
            .contracts
            .iter()
            .map(|contract| ContractContext {
                contract_id: contract.contract_id.clone(),
                statement: contract.statement.clone(),
                actor_ids: contract.actor_ids.clone(),
            })
            .collect();
        let gates = semantic_ir
            .gates
            .iter()
            .map(|gate| GateContext {
                gate_id: gate.gate_id.clone(),
                condition: gate.condition.clone(),
                related_interface_ids: gate.related_interface_ids.clone(),
            })
            .collect();
        let abstractions = semantic_ir
            .abstractions
            .iter()
            .map(|abstraction| AbstractionContext {
                abstraction_id: abstraction.abstraction_id.clone(),
                description: abstraction.description.clone(),
            })
            .collect();

        Self {
            semantic_actors,
            phases,
            invariants,
            assertions,
            contracts,
            gates,
            abstractions,
            residual_decisions: semantic_ir.residual_decisions.clone(),
        }
    }
}

#[derive(Debug, Clone)]
struct SemanticActorContext {
    actor_id: String,
    actor_name: Option<String>,
    role_summary: String,
    supporting_statement_ids: Vec<String>,
    supporting_section_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct PhaseContext {
    phase_id: String,
    summary: String,
    supporting_statement_ids: Vec<String>,
    supporting_section_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct ConstraintSourceContext {
    source_id: String,
    statement: String,
    related_interface_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct ContractContext {
    contract_id: String,
    statement: String,
    actor_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct GateContext {
    gate_id: String,
    condition: String,
    related_interface_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct AbstractionContext {
    abstraction_id: String,
    description: String,
}

#[expect(
    clippy::too_many_arguments,
    reason = "Intent identity summarizes all canonical slices explicitly for traceable reporting"
)]
fn build_intent_identity(
    document_identity: &IntentDocumentIdentity,
    actors: &[IntentActor],
    interfaces: &[InterfaceRecord],
    system_contract: Option<&SystemContractRecord>,
    behaviors: &[BehaviorIntent],
    constraints: &[IntentConstraint],
    regular_states: &[RegularStateRecord],
    state_transitions: &[StateTransitionRecord],
    symbol_definitions: &[SymbolDefinitionRecord],
    control_blocks: &[ControlBlockRecord],
    explicit_modules: &[ExplicitModuleRecord],
    explicit_tops: &[ExplicitTopRecord],
) -> IntentIdentity {
    IntentIdentity {
        intent_id: format!("intent_{}", document_identity.document_key),
        summary: format!(
            "backend-neutral intent for {} covering {} actors, {} interfaces, {} behaviors, {} constraints, {} regular states, {} state transitions, {} symbol definitions, {} structured control blocks, {} explicit modules, {} explicit tops, and {} explicit system contract",
            document_identity.display_name,
            actors.len(),
            interfaces.len(),
            behaviors.len(),
            constraints.len(),
            regular_states.len(),
            state_transitions.len(),
            symbol_definitions.len(),
            control_blocks.len(),
            explicit_modules.len(),
            explicit_tops.len(),
            if system_contract.is_some() { 1 } else { 0 }
        ),
    }
}

fn build_intent_actors(context: &IntentContext) -> Vec<IntentActor> {
    let mut actors = Vec::new();

    for actor in &context.semantic_actors {
        let mut responsibilities = BTreeSet::new();
        responsibilities.insert(actor.role_summary.clone());

        for contract in &context.contracts {
            if contract.actor_ids.contains(&actor.actor_id) {
                responsibilities.insert(normalize_sentence(&contract.statement));
            }
        }

        for phase in &context.phases {
            if overlaps(
                actor.supporting_statement_ids.as_slice(),
                phase.supporting_statement_ids.as_slice(),
            ) || overlaps(
                actor.supporting_section_ids.as_slice(),
                phase.supporting_section_ids.as_slice(),
            ) {
                responsibilities.insert(format!("participate in {}", phase.summary));
            }
        }

        if actor.actor_id.ends_with("_channel") {
            responsibilities.insert(
                "treat grouped interface semantics as a backend-neutral channel abstraction"
                    .to_string(),
            );
        }

        actors.push(IntentActor {
            actor_id: actor.actor_id.clone(),
            actor_name: actor.actor_name.clone(),
            responsibilities: responsibilities.into_iter().collect(),
            supporting_actor_ids: vec![actor.actor_id.clone()],
        });
    }

    actors
}

fn build_behaviors(context: &IntentContext, actor_ids: BTreeSet<String>) -> Vec<BehaviorIntent> {
    let mut behaviors = Vec::new();
    let mut seen = BTreeSet::new();

    for phase in &context.phases {
        let statement = normalize_sentence(&phase.summary);
        let dedupe_key = normalize_text_key(&statement);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        behaviors.push(BehaviorIntent {
            behavior_id: format!("behavior_{}", document_key(&phase.phase_id)),
            statement,
            actor_ids: actor_ids.iter().cloned().collect(),
            supporting_semantic_ids: vec![phase.phase_id.clone()],
        });
    }

    for contract in &context.contracts {
        let statement = normalize_sentence(&contract.statement);
        let dedupe_key = normalize_text_key(&statement);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        behaviors.push(BehaviorIntent {
            behavior_id: format!("behavior_{}", document_key(&contract.contract_id)),
            statement,
            actor_ids: contract.actor_ids.clone(),
            supporting_semantic_ids: vec![contract.contract_id.clone()],
        });
    }

    for gate in &context.gates {
        let statement = normalize_sentence(&gate.condition);
        let dedupe_key = normalize_text_key(&statement);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        behaviors.push(BehaviorIntent {
            behavior_id: format!("behavior_{}", document_key(&gate.gate_id)),
            statement,
            actor_ids: actor_ids.iter().cloned().collect(),
            supporting_semantic_ids: vec![gate.gate_id.clone()],
        });
    }

    behaviors
}

fn build_constraints(context: &IntentContext) -> Vec<IntentConstraint> {
    let mut constraints = Vec::new();
    let mut seen = BTreeSet::new();

    for invariant in &context.invariants {
        let statement = normalize_sentence(&invariant.statement);
        let dedupe_key = normalize_text_key(&statement);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        constraints.push(IntentConstraint {
            constraint_id: format!("constraint_{}", document_key(&invariant.source_id)),
            statement,
            related_interface_ids: invariant.related_interface_ids.clone(),
            supporting_semantic_ids: vec![invariant.source_id.clone()],
        });
    }

    for assertion in &context.assertions {
        let statement = normalize_sentence(&assertion.statement);
        let dedupe_key = normalize_text_key(&statement);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        constraints.push(IntentConstraint {
            constraint_id: format!("constraint_{}", document_key(&assertion.source_id)),
            statement,
            related_interface_ids: assertion.related_interface_ids.clone(),
            supporting_semantic_ids: vec![assertion.source_id.clone()],
        });
    }

    for gate in &context.gates {
        if gate.related_interface_ids.is_empty() {
            continue;
        }

        let statement = format!(
            "{} [interface-coupled rule]",
            normalize_sentence(&gate.condition)
        );
        let dedupe_key = normalize_text_key(&statement);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        constraints.push(IntentConstraint {
            constraint_id: format!("constraint_{}_gate", document_key(&gate.gate_id)),
            statement,
            related_interface_ids: gate.related_interface_ids.clone(),
            supporting_semantic_ids: vec![gate.gate_id.clone()],
        });
    }

    constraints
}

fn build_assumptions(context: &IntentContext, actors: &[IntentActor]) -> Vec<IntentAssumption> {
    let mut assumptions = Vec::new();
    let mut seen = BTreeSet::new();

    for abstraction in &context.abstractions {
        let statement = normalize_sentence(&abstraction.description);
        let dedupe_key = normalize_text_key(&statement);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        assumptions.push(IntentAssumption {
            assumption_id: format!("assumption_{}", document_key(&abstraction.abstraction_id)),
            statement,
            supporting_semantic_ids: vec![abstraction.abstraction_id.clone()],
        });
    }

    for actor in actors {
        if !actor.actor_id.ends_with("_channel") {
            continue;
        }

        let statement = format!(
            "{} is treated as a backend-neutral channel abstraction in this IntentIR pass.",
            actor.actor_id
        );
        let dedupe_key = normalize_text_key(&statement);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        assumptions.push(IntentAssumption {
            assumption_id: format!("assumption_{}", document_key(&actor.actor_id)),
            statement,
            supporting_semantic_ids: actor.supporting_actor_ids.clone(),
        });
    }

    if context
        .residual_decisions
        .iter()
        .any(|packet| packet.packet_id == "semantic_ambiguous_visual_grounding")
    {
        let statement =
            "Ambiguous visual evidence is preserved as unresolved context rather than promoted into stronger canonical constraints in this IntentIR pass.".to_string();
        assumptions.push(IntentAssumption {
            assumption_id: "assumption_ambiguous_visual_grounding".to_string(),
            statement,
            supporting_semantic_ids: vec!["semantic_ambiguous_visual_grounding".to_string()],
        });
    }

    if context
        .residual_decisions
        .iter()
        .any(|packet| packet.packet_id == "semantic_resolved_role_without_consensus")
    {
        let statement =
            "Some carried semantic roles remain provisional because they still lack preserved observation-backed consensus in this IntentIR pass.".to_string();
        assumptions.push(IntentAssumption {
            assumption_id: "assumption_semantic_role_without_consensus".to_string(),
            statement,
            supporting_semantic_ids: vec!["semantic_resolved_role_without_consensus".to_string()],
        });
    }

    if context
        .residual_decisions
        .iter()
        .any(|packet| packet.packet_id == "semantic_alias_dependent_handshake_completion")
    {
        let statement =
            "Some typed handshake-completion semantics remain provisional because they still depend on alias-grounded semantic role consensus in this IntentIR pass.".to_string();
        assumptions.push(IntentAssumption {
            assumption_id: "assumption_alias_dependent_handshake_completion".to_string(),
            statement,
            supporting_semantic_ids: vec![
                "semantic_alias_dependent_handshake_completion".to_string(),
            ],
        });
    }

    assumptions
}

fn build_residual_decisions(
    context: &IntentContext,
    actors: &[IntentActor],
    behaviors: &[BehaviorIntent],
    constraints: &[IntentConstraint],
) -> Vec<ResidualDecisionPacket> {
    let mut residual_decisions = context.residual_decisions.clone();

    if actors.is_empty() {
        residual_decisions.push(ResidualDecisionPacket {
            packet_id: "intent_actor_canonicalization".to_string(),
            question: "Which canonical actors should own the backend-neutral intent model?".to_string(),
            why_unresolved: "SemanticIR did not provide stable actor records that can be lowered into IntentIR actor responsibilities safely.".to_string(),
            automation_confidence: AutomationConfidence::Low,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "interface_owned_actors".to_string(),
                    description: "Infer IntentIR actors from interface ownership and coupling patterns.".to_string(),
                    downstream_impact: "IntentIR becomes executable sooner, but actor invention risk increases without stronger semantic support.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "defer_actor_identity".to_string(),
                    description: "Keep actor identity unresolved until richer semantic or user-guided input exists.".to_string(),
                    downstream_impact: "IntentIR remains conservative, but adapters may be blocked on missing ownership structure.".to_string(),
                },
            ],
        });
    }

    if behaviors.is_empty() {
        residual_decisions.push(ResidualDecisionPacket {
            packet_id: "intent_behavior_canonicalization".to_string(),
            question: "Which canonical behaviors should be emitted into IntentIR?".to_string(),
            why_unresolved: "The current SemanticIR artifact did not provide enough phase, contract, or gate structure to canonicalize stable behavior intents.".to_string(),
            automation_confidence: AutomationConfidence::Low,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "constraint_only_intent".to_string(),
                    description: "Emit IntentIR with constraints and assumptions only, deferring explicit behaviors.".to_string(),
                    downstream_impact: "The canonical model stays conservative, but adapter work may need later behavior reconstruction.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "synthesize_from_sections".to_string(),
                    description: "Synthesize behaviors from coarser semantic clustering even without explicit phase or gate evidence.".to_string(),
                    downstream_impact: "IntentIR becomes fuller, but semantic invention risk increases.".to_string(),
                },
            ],
        });
    }

    if constraints.is_empty() {
        residual_decisions.push(ResidualDecisionPacket {
            packet_id: "intent_constraint_canonicalization".to_string(),
            question: "Should IntentIR emit canonical constraints when SemanticIR has no explicit invariant or assertion records?".to_string(),
            why_unresolved: "The current semantic slice did not produce stable constraint candidates, so canonical lowering would require speculative inference.".to_string(),
            automation_confidence: AutomationConfidence::Low,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "emit_minimal_intent".to_string(),
                    description: "Keep IntentIR minimal and wait for richer upstream semantic extraction.".to_string(),
                    downstream_impact: "Canonical intent remains safe but incomplete for later adapters.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "promote_behavior_rules".to_string(),
                    description: "Promote some behaviors into constraints heuristically.".to_string(),
                    downstream_impact: "IntentIR becomes denser, but semantic categories may blur too early.".to_string(),
                },
            ],
        });
    }

    residual_decisions
}

fn overlaps(left: &[String], right: &[String]) -> bool {
    if left.is_empty() || right.is_empty() {
        return false;
    }

    let right_ids: BTreeSet<&String> = right.iter().collect();
    left.iter().any(|id| right_ids.contains(id))
}

fn normalize_sentence(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn normalize_text_key(text: &str) -> String {
    normalize_sentence(text).to_ascii_lowercase()
}

// ---------------------------------------------------------------------------
// Transaction intent — ordered behavioral steps for ISF lowering
// ---------------------------------------------------------------------------

/// An ordered sequence of behavioral steps that form a transaction body.
/// Maps directly to ISF `(transaction name clauses...)`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransactionIntent {
    pub transaction_id: String,
    /// Human-readable name, sanitized for ISF.
    pub transaction_name: String,
    /// The activation port that starts this transaction (e.g., "start").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_port: Option<String>,
    /// Ports declared on this transaction (for do/spawn bindings).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub ports: Vec<TransactionPortRecord>,
    /// Ordered body steps.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<TransactionStep>,
    /// The control_block_ids this transaction was built from.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_block_ids: Vec<String>,
    /// The temporal_rule_ids referenced by this transaction.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_temporal_rule_ids: Vec<String>,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// A single port declaration on a transaction, used for do/spawn bindings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransactionPortRecord {
    pub port_name: String,
    pub direction: TransactionPortDirection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<u32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TransactionPortDirection {
    Input,
    Output,
    InOut,
}

/// One step in a transaction body. Each variant maps to an ISF construct.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "step_kind", rename_all = "snake_case")]
pub enum TransactionStep {
    /// `(drive name args...)` — call a named drive with actual values.
    Drive {
        drive_name: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        actuals: Vec<String>,
    },
    /// `(when condition body...)` — inline conditional.
    When {
        condition: String,
        body: Vec<TransactionStep>,
    },
    /// `(switch selector (val body...)...)` — multi-way dispatch.
    Switch {
        selector: String,
        branches: Vec<SwitchBranch>,
    },
    /// `(while condition body...)` — loop while condition holds.
    While {
        condition: String,
        body: Vec<TransactionStep>,
    },
    /// `(until condition body...)` — loop until condition holds.
    Until {
        condition: String,
        body: Vec<TransactionStep>,
    },
    /// `(repeat count body...)` — repeat N times.
    Repeat {
        count: String,
        body: Vec<TransactionStep>,
    },
    /// `(await port)` / `(await port (watchdog N))` — wait for port.
    Await {
        port: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        watchdog: Option<u32>,
    },
    /// `(wait N)` — wait N cycles.
    Wait { count: String },
    /// `(sample port as name)` — capture port value.
    Sample { port: String, as_name: String },
    /// `(do child (bind ...))` — synchronous child activation.
    Do {
        child_transaction: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        bindings: Vec<PortBinding>,
    },
    /// `(spawn child as inst (bind ...))` — parallel child activation.
    Spawn {
        child_transaction: String,
        instance: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        bindings: Vec<PortBinding>,
    },
    /// `(set target expr)` — register assignment.
    Set { target: String, expr: String },
    /// `(update target expr)` — in-place update (increment/decrement).
    Update { target: String, expr: String },
    /// `(shift_left reg bit)` — shift left.
    ShiftLeft { reg: String, bit: String },
    /// `(shift_right reg bit)` — shift right.
    ShiftRight {
        reg: String,
        bit: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        width: Option<u32>,
    },
    /// `(complete port)` — transaction completion.
    Complete { port: String },
    /// `(await_all done_port)` — wait for all child transactions.
    AwaitAll { done_port: String },
    /// `(await_any done_port)` — wait for any child transaction.
    AwaitAny { done_port: String },
    /// `(latency (min N) (max M))` — latency bounds.
    Latency { min: u32, max: u32 },
}

/// A branch within a `(switch selector ...)` clause.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SwitchBranch {
    pub value: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub body: Vec<TransactionStep>,
}

/// A port binding for do/spawn activation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PortBinding {
    pub direction: TransactionPortDirection,
    pub child_port: String,
    pub parent_signal: String,
}

// ---------------------------------------------------------------------------
// Actor interaction records — multi-actor drive/sample/trigger/dependency
// ---------------------------------------------------------------------------

/// "Actor X drives signal Y to actor Z when condition C"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorDriveRelationRecord {
    pub relation_id: String,
    pub driver_actor: String,
    pub signal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consumer_actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    pub source_text: String,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// "Actor X samples signal Y from actor Z at phase P"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorSampleRelationRecord {
    pub relation_id: String,
    pub sampler_actor: String,
    pub signal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_actor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phase: Option<String>,
    pub source_text: String,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// "Actor X triggers/spawns actor Y on port P"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorTriggerRelationRecord {
    pub relation_id: String,
    pub source_actor: String,
    pub target_actor: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_port: Option<String>,
    /// Whether this is a do (synchronous) or spawn (parallel) activation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_kind: Option<ActivationKind>,
    pub source_text: String,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActivationKind {
    Do,
    Spawn,
}

/// "Actor X waits for actor Y to assert/deassert signal Z"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorTemporalDependencyRecord {
    pub dependency_id: String,
    pub waiting_actor: String,
    pub signal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_actor: Option<String>,
    /// Whether the dependency waits for assertion or deassertion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wait_for: Option<WaitForKind>,
    pub source_text: String,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum WaitForKind {
    Assertion,
    Deassertion,
}

/// A temporal invariant — a constraint that always holds across time.
/// "Signal X must not change when Y is low", "X must remain stable during Y"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalInvariantRecord {
    pub invariant_id: String,
    pub subject_signal: String,
    pub invariant_kind: TemporalInvariantKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_signal: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub condition_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_value: Option<String>,
    pub source_text: String,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TemporalInvariantKind {
    MustNotChange,
    MustRemainStable,
    MustBeAsserted,
    MustMatch,
    MustNotExceed,
    OnlyValidWhen,
}

/// Count total nested steps (including recursion into when/switch/while/until/repeat bodies).
pub fn count_nested_steps(steps: &[TransactionStep]) -> usize {
    let mut count = steps.len();
    for step in steps {
        match step {
            TransactionStep::When { body, .. }
            | TransactionStep::While { body, .. }
            | TransactionStep::Until { body, .. }
            | TransactionStep::Repeat { body, .. } => {
                count += count_nested_steps(body);
            }
            TransactionStep::Switch { branches, .. } => {
                for branch in branches {
                    count += count_nested_steps(&branch.body);
                }
            }
            _ => {}
        }
    }
    count
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
}

// ---------------------------------------------------------------------------
// Synthesis: populate new IntentIR temporal record types from SemanticIR data
// ---------------------------------------------------------------------------

fn recognize_digital_patterns(
    transactions: &mut Vec<TransactionIntent>,
    drive_relations: &mut Vec<ActorDriveRelationRecord>,
    _sample_relations: &mut Vec<ActorSampleRelationRecord>,
    trigger_relations: &mut Vec<ActorTriggerRelationRecord>,
    _temporal_deps: &mut Vec<ActorTemporalDependencyRecord>,
    invariants: &mut Vec<TemporalInvariantRecord>,
    semantic_ir: &SemanticIr,
) {
    // Collect all signal names across interfaces and actor relations
    let all_signals: BTreeSet<String> = semantic_ir
        .actor_signal_relations
        .iter()
        .map(|r| r.signal_name.clone())
        .chain(
            semantic_ir
                .interfaces
                .iter()
                .flat_map(|i| i.signal_records.iter().map(|s| s.signal_name.clone())),
        )
        .collect();

    let signal_list: Vec<&str> = all_signals.iter().map(|s| s.as_str()).collect();

    // --- Pattern 1: Valid/Ready handshake ---
    // Look for pairs like *_VALID + *_READY, or VALID + READY
    let valid_signals: Vec<&str> = signal_list
        .iter()
        .filter(|s| s.ends_with("VALID") || s.ends_with("_VALID") || s.starts_with("VALID_"))
        .copied()
        .collect();
    let ready_signals: Vec<&str> = signal_list
        .iter()
        .filter(|s| s.ends_with("READY") || s.ends_with("_READY") || s.starts_with("READY_"))
        .copied()
        .collect();

    for valid in &valid_signals {
        for ready in &ready_signals {
            // Pair signals that share a common prefix (e.g., AXI_AWVALID + AXI_AWREADY)
            let v_base = valid.trim_end_matches("_VALID").trim_end_matches("VALID");
            let r_base = ready.trim_end_matches("_READY").trim_end_matches("READY");
            if v_base == r_base
                || valid.contains(ready.trim_end_matches("_READY"))
                || ready.contains(valid.trim_end_matches("_VALID"))
            {
                let tx_name = format!("{}_handshake", sanitize_id(v_base));
                // Check if we already have a transaction for this handshake
                let already_has = transactions.iter().any(|t| t.transaction_name == tx_name);
                if !already_has {
                    let steps = vec![
                        TransactionStep::AwaitAll {
                            done_port: format!("{}_handshake_done", sanitize_id(v_base)),
                        },
                        TransactionStep::Sample {
                            port: valid.to_string(),
                            as_name: format!("{}_val", sanitize_id(v_base)),
                        },
                    ];
                    transactions.push(TransactionIntent {
                        transaction_id: format!("txn_hs_{}", sanitize_id(v_base)),
                        transaction_name: tx_name,
                        activation_port: Some(ready.to_string()),
                        ports: vec![
                            TransactionPortRecord {
                                port_name: valid.to_string(),
                                direction: TransactionPortDirection::Input,
                                width: None,
                            },
                            TransactionPortRecord {
                                port_name: ready.to_string(),
                                direction: TransactionPortDirection::Output,
                                width: None,
                            },
                        ],
                        steps,
                        source_block_ids: Vec::new(),
                        source_temporal_rule_ids: Vec::new(),
                        supporting_statement_ids: Vec::new(),
                        automation_confidence: AutomationConfidence::Medium,
                    });
                }
                // Actor drive relations for handshake
                let valid_driver = semantic_ir.actor_signal_relations.iter().find(|r| {
                    r.signal_name == *valid && matches!(r.relation, RelationKind::Drives)
                });
                if let Some(driver) = valid_driver {
                    let already_rel = drive_relations
                        .iter()
                        .any(|dr| dr.signal_name == *valid && dr.driver_actor == driver.actor_name);
                    if !already_rel {
                        drive_relations.push(ActorDriveRelationRecord {
                            relation_id: format!("adr_hs_{}", sanitize_id(valid)),
                            driver_actor: driver.actor_name.clone(),
                            signal_name: valid.to_string(),
                            consumer_actor: None,
                            condition: Some(format!("{} asserted", ready)),
                            value: None,
                            source_text: String::new(),
                            supporting_statement_ids: Vec::new(),
                            automation_confidence: AutomationConfidence::Medium,
                        });
                    }
                }
            }
        }
    }

    // --- Pattern 2: Request/Acknowledge/Grant ---
    let req_signals: Vec<&str> = signal_list
        .iter()
        .filter(|s| {
            s.ends_with("REQ")
                || s.ends_with("_REQ")
                || s.ends_with("REQUEST")
                || s.ends_with("_REQUEST")
        })
        .copied()
        .collect();
    let ack_signals: Vec<&str> = signal_list
        .iter()
        .filter(|s| {
            s.ends_with("ACK")
                || s.ends_with("_ACK")
                || s.ends_with("GNT")
                || s.ends_with("_GNT")
                || s.ends_with("GRANT")
        })
        .copied()
        .collect();

    for req in &req_signals {
        for ack in &ack_signals {
            let req_actor = semantic_ir
                .actor_signal_relations
                .iter()
                .find(|r| r.signal_name == *req && matches!(r.relation, RelationKind::Drives));
            let ack_actor = semantic_ir
                .actor_signal_relations
                .iter()
                .find(|r| r.signal_name == *ack && matches!(r.relation, RelationKind::Drives));
            if let (Some(req_a), Some(ack_a)) = (req_actor, ack_actor)
                && req_a.actor_name != ack_a.actor_name
            {
                let already = trigger_relations.iter().any(|tr| {
                    tr.source_actor == req_a.actor_name && tr.target_actor == ack_a.actor_name
                });
                if !already {
                    trigger_relations.push(ActorTriggerRelationRecord {
                        relation_id: format!(
                            "atr_reqack_{}_{}",
                            sanitize_id(&req_a.actor_name),
                            sanitize_id(&ack_a.actor_name)
                        ),
                        source_actor: req_a.actor_name.clone(),
                        target_actor: ack_a.actor_name.clone(),
                        trigger_port: Some(req.to_string()),
                        activation_kind: Some(ActivationKind::Do),
                        source_text: String::new(),
                        supporting_statement_ids: Vec::new(),
                        automation_confidence: AutomationConfidence::Medium,
                    });
                }
            }
        }
    }

    // --- Pattern 3: AHB bus protocol ---
    let has_htrans = signal_list.contains(&"HTRANS");
    let has_hready = signal_list.contains(&"HREADY");
    let has_haddr = signal_list.contains(&"HADDR");
    if has_htrans && has_hready {
        let steps = vec![
            TransactionStep::Await {
                port: "HREADY".to_string(),
                watchdog: Some(16),
            },
            TransactionStep::When {
                condition: "HTRANS == NONSEQ or HTRANS == SEQ".to_string(),
                body: vec![
                    TransactionStep::Drive {
                        drive_name: "HADDR".to_string(),
                        actuals: vec!["addr_value".to_string()],
                    },
                    TransactionStep::Drive {
                        drive_name: "HWRITE".to_string(),
                        actuals: vec!["write_value".to_string()],
                    },
                ],
            },
        ];
        let tx_id = "txn_ahb_transfer";
        let already = transactions.iter().any(|t| t.transaction_id == tx_id);
        if !already {
            let mut ports = vec![
                TransactionPortRecord {
                    port_name: "HTRANS".to_string(),
                    direction: TransactionPortDirection::Output,
                    width: Some(2),
                },
                TransactionPortRecord {
                    port_name: "HREADY".to_string(),
                    direction: TransactionPortDirection::Input,
                    width: Some(1),
                },
            ];
            if has_haddr {
                ports.push(TransactionPortRecord {
                    port_name: "HADDR".to_string(),
                    direction: TransactionPortDirection::Output,
                    width: Some(32),
                });
            }
            transactions.push(TransactionIntent {
                transaction_id: tx_id.to_string(),
                transaction_name: "ahb_transfer".to_string(),
                activation_port: Some("HREADY".to_string()),
                ports,
                steps,
                source_block_ids: Vec::new(),
                source_temporal_rule_ids: Vec::new(),
                supporting_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            });
        }

        // FIFO-like invariant: "HADDR must not change when HREADY is low"
        invariants.push(TemporalInvariantRecord {
            invariant_id: "tinv_ahb_addr_stable".to_string(),
            subject_signal: "HADDR".to_string(),
            invariant_kind: TemporalInvariantKind::MustNotChange,
            condition_signal: Some("HREADY".to_string()),
            condition_value: Some("LOW".to_string()),
            target_value: None,
            source_text: "AHB protocol: address must remain stable while slave is not ready"
                .to_string(),
            supporting_statement_ids: Vec::new(),
            automation_confidence: AutomationConfidence::Medium,
        });
    }

    // --- Pattern 4: APB bus protocol ---
    let has_psel = signal_list
        .iter()
        .any(|s| *s == "PSEL" || s.starts_with("PSEL"));
    let has_penable = signal_list.contains(&"PENABLE");
    let has_pready = signal_list.contains(&"PREADY");
    if has_psel && has_penable {
        let steps = vec![TransactionStep::When {
            condition: "PSEL asserted".to_string(),
            body: vec![
                TransactionStep::Drive {
                    drive_name: "PENABLE".to_string(),
                    actuals: vec!["1".to_string()],
                },
                TransactionStep::Await {
                    port: "PREADY".to_string(),
                    watchdog: Some(16),
                },
            ],
        }];
        let tx_id = "txn_apb_transfer";
        let already = transactions.iter().any(|t| t.transaction_id == tx_id);
        if !already {
            let mut ports = vec![
                TransactionPortRecord {
                    port_name: "PSEL".to_string(),
                    direction: TransactionPortDirection::Output,
                    width: Some(1),
                },
                TransactionPortRecord {
                    port_name: "PENABLE".to_string(),
                    direction: TransactionPortDirection::Output,
                    width: Some(1),
                },
            ];
            if has_pready {
                ports.push(TransactionPortRecord {
                    port_name: "PREADY".to_string(),
                    direction: TransactionPortDirection::Input,
                    width: Some(1),
                });
            }
            transactions.push(TransactionIntent {
                transaction_id: tx_id.to_string(),
                transaction_name: "apb_transfer".to_string(),
                activation_port: None,
                ports,
                steps,
                source_block_ids: Vec::new(),
                source_temporal_rule_ids: Vec::new(),
                supporting_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    // --- Pattern 5: SPI protocol ---
    let has_miso = signal_list.contains(&"MISO");
    let has_mosi = signal_list.contains(&"MOSI");
    let has_sclk = signal_list.iter().any(|s| *s == "SCLK" || *s == "SCK");
    if (has_miso || has_mosi) && has_sclk {
        let steps = vec![TransactionStep::Repeat {
            count: "8".to_string(),
            body: vec![
                TransactionStep::Drive {
                    drive_name: "SCLK".to_string(),
                    actuals: vec!["1".to_string()],
                },
                TransactionStep::ShiftLeft {
                    reg: "mosi_shift".to_string(),
                    bit: "1".to_string(),
                },
                TransactionStep::Sample {
                    port: "MISO".to_string(),
                    as_name: "miso_bit".to_string(),
                },
                TransactionStep::ShiftRight {
                    reg: "miso_shift".to_string(),
                    bit: "miso_bit".to_string(),
                    width: Some(8),
                },
                TransactionStep::Drive {
                    drive_name: "SCLK".to_string(),
                    actuals: vec!["0".to_string()],
                },
            ],
        }];
        let tx_id = "txn_spi_transfer";
        let already = transactions.iter().any(|t| t.transaction_id == tx_id);
        if !already {
            transactions.push(TransactionIntent {
                transaction_id: tx_id.to_string(),
                transaction_name: "spi_transfer".to_string(),
                activation_port: None,
                ports: vec![
                    TransactionPortRecord {
                        port_name: "SCLK".to_string(),
                        direction: TransactionPortDirection::Output,
                        width: Some(1),
                    },
                    TransactionPortRecord {
                        port_name: "MOSI".to_string(),
                        direction: TransactionPortDirection::Output,
                        width: Some(1),
                    },
                    TransactionPortRecord {
                        port_name: "MISO".to_string(),
                        direction: TransactionPortDirection::Input,
                        width: Some(1),
                    },
                ],
                steps,
                source_block_ids: Vec::new(),
                source_temporal_rule_ids: Vec::new(),
                supporting_statement_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    // --- Pattern 6: FIFO status ---
    let has_full = signal_list
        .iter()
        .any(|s| *s == "FULL" || s.ends_with("_FULL"));
    let has_empty = signal_list
        .iter()
        .any(|s| *s == "EMPTY" || s.ends_with("_EMPTY"));
    if has_full {
        invariants.push(TemporalInvariantRecord {
            invariant_id: "tinv_fifo_full_write_protect".to_string(),
            subject_signal: "WEN".to_string(),
            invariant_kind: TemporalInvariantKind::OnlyValidWhen,
            condition_signal: Some("FULL".to_string()),
            condition_value: Some("LOW".to_string()),
            target_value: None,
            source_text: "FIFO: write enable only valid when FIFO is not full".to_string(),
            supporting_statement_ids: Vec::new(),
            automation_confidence: AutomationConfidence::Medium,
        });
    }
    if has_empty {
        invariants.push(TemporalInvariantRecord {
            invariant_id: "tinv_fifo_empty_read_protect".to_string(),
            subject_signal: "REN".to_string(),
            invariant_kind: TemporalInvariantKind::OnlyValidWhen,
            condition_signal: Some("EMPTY".to_string()),
            condition_value: Some("LOW".to_string()),
            target_value: None,
            source_text: "FIFO: read enable only valid when FIFO is not empty".to_string(),
            supporting_statement_ids: Vec::new(),
            automation_confidence: AutomationConfidence::Medium,
        });
    }
}

fn synthesize_transactions(semantic_ir: &SemanticIr) -> Vec<TransactionIntent> {
    let mut transactions = Vec::new();

    // 1. Convert each control_block to a TransactionIntent.
    //    Control blocks already capture conditional behavioral logic —
    //    they map ~1:1 to ISF transactions.
    for cb in &semantic_ir.control_blocks {
        let mut steps = Vec::new();

        if let Some(ref selector) = cb.selector {
            // selector + branches → Switch step
            let selector_expr = render_control_expression(selector);
            let mut branches = Vec::new();
            for branch in &cb.branches {
                let value = branch
                    .predicate
                    .as_ref()
                    .map(|p| extract_case_value(p, selector))
                    .unwrap_or_else(|| "default".to_string());
                let body = branch
                    .actions
                    .iter()
                    .map(convert_control_action_to_step)
                    .collect();
                branches.push(SwitchBranch { value, body });
            }
            if !branches.is_empty() {
                steps.push(TransactionStep::Switch {
                    selector: selector_expr,
                    branches,
                });
            }
        } else {
            // No selector — flat sequence of When branches or direct actions
            for branch in &cb.branches {
                if let Some(predicate) = &branch.predicate {
                    let condition = render_control_expression(predicate);
                    let body: Vec<TransactionStep> = branch
                        .actions
                        .iter()
                        .map(convert_control_action_to_step)
                        .collect();
                    if !body.is_empty() {
                        steps.push(TransactionStep::When { condition, body });
                    }
                } else {
                    // Unconditional branch — direct actions
                    for action in &branch.actions {
                        steps.push(convert_control_action_to_step(action));
                    }
                }
            }
        }

        // Extract ports referenced in this control block
        let ports: Vec<TransactionPortRecord> = cb
            .referenced_signal_names
            .iter()
            .map(|sig| TransactionPortRecord {
                port_name: sig.clone(),
                direction: TransactionPortDirection::InOut,
                width: None,
            })
            .collect();

        transactions.push(TransactionIntent {
            transaction_id: format!("txn_{}", cb.block_id),
            transaction_name: cb.block_name.clone(),
            activation_port: None,
            ports,
            steps,
            source_block_ids: vec![cb.block_id.clone()],
            source_temporal_rule_ids: Vec::new(),
            supporting_statement_ids: cb.supporting_statement_ids.clone(),
            automation_confidence: cb.automation_confidence,
        });
    }

    // 2. Synthesize per-actor transactions from temporal_rules.
    //    Group temporal_rules by the actor that drives the consequent signal.
    let mut actor_txn_map: BTreeMap<String, Vec<&TemporalRuleRecord>> = BTreeMap::new();
    for rule in &semantic_ir.temporal_rules {
        for consequent in &rule.consequents {
            if let TemporalPredicateRecord::ActorDrivesSignal { actor_name, .. } = consequent {
                actor_txn_map
                    .entry(actor_name.clone())
                    .or_default()
                    .push(rule);
                break; // one actor per rule for grouping
            }
        }
    }

    for (actor_name, rules) in actor_txn_map.iter() {
        if rules.is_empty() {
            continue;
        }
        let mut steps = Vec::new();
        for rule in rules {
            // antecedents → When condition
            let conditions: Vec<String> = rule
                .antecedents
                .iter()
                .map(render_temporal_predicate)
                .collect();
            let condition = if conditions.is_empty() {
                format!("on_{:?}_edge", rule.edge)
            } else {
                conditions.join(" and ")
            };

            let mut body = Vec::new();
            for consequent in &rule.consequents {
                if let Some(step) = temporal_consequent_to_step(consequent) {
                    body.push(step);
                }
            }

            if !body.is_empty() {
                steps.push(TransactionStep::When { condition, body });
            }
        }

        if !steps.is_empty() {
            let all_rule_ids: Vec<String> = rules.iter().map(|r| r.rule_id.clone()).collect();
            let all_stmt_ids: Vec<String> = rules
                .iter()
                .flat_map(|r| r.supporting_statement_ids.clone())
                .collect();
            transactions.push(TransactionIntent {
                transaction_id: format!("txn_temporal_{}", sanitize_id(actor_name)),
                transaction_name: format!("{}_behavior", actor_name),
                activation_port: None,
                ports: Vec::new(),
                steps,
                source_block_ids: Vec::new(),
                source_temporal_rule_ids: all_rule_ids,
                supporting_statement_ids: all_stmt_ids,
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    transactions
}

fn synthesize_actor_drive_relations(semantic_ir: &SemanticIr) -> Vec<ActorDriveRelationRecord> {
    let mut relations = Vec::new();

    // From actor_signal_relations (Drives)
    for rel in &semantic_ir.actor_signal_relations {
        if matches!(rel.relation, RelationKind::Drives) {
            relations.push(ActorDriveRelationRecord {
                relation_id: format!("adr_{}", rel.relation_id),
                driver_actor: rel.actor_name.clone(),
                signal_name: rel.signal_name.clone(),
                consumer_actor: None,
                condition: None,
                value: None,
                source_text: String::new(),
                supporting_statement_ids: rel.source_statement_ids.clone(),
                automation_confidence: rel.automation_confidence,
            });
        }
    }

    // From temporal_rules — consequents with ActorDrivesSignal
    for rule in &semantic_ir.temporal_rules {
        for consequent in &rule.consequents {
            if let TemporalPredicateRecord::ActorDrivesSignal {
                actor_name,
                signal_name,
                ..
            } = consequent
            {
                // Check if we already have this relation
                let already_has = relations
                    .iter()
                    .any(|r| r.driver_actor == *actor_name && r.signal_name == *signal_name);
                if !already_has {
                    relations.push(ActorDriveRelationRecord {
                        relation_id: format!(
                            "adr_{}_{}",
                            sanitize_id(actor_name),
                            sanitize_id(signal_name)
                        ),
                        driver_actor: actor_name.clone(),
                        signal_name: signal_name.clone(),
                        consumer_actor: None,
                        condition: None,
                        value: None,
                        source_text: rule.source_text.clone(),
                        supporting_statement_ids: rule.supporting_statement_ids.clone(),
                        automation_confidence: rule.automation_confidence,
                    });
                }
            }
        }
    }

    relations
}

fn synthesize_actor_sample_relations(semantic_ir: &SemanticIr) -> Vec<ActorSampleRelationRecord> {
    let mut relations = Vec::new();

    // From actor_signal_relations (Reads)
    for rel in &semantic_ir.actor_signal_relations {
        if matches!(rel.relation, RelationKind::Reads) {
            relations.push(ActorSampleRelationRecord {
                relation_id: format!("asr_{}", rel.relation_id),
                sampler_actor: rel.actor_name.clone(),
                signal_name: rel.signal_name.clone(),
                source_actor: None,
                phase: None,
                source_text: String::new(),
                supporting_statement_ids: rel.source_statement_ids.clone(),
                automation_confidence: rel.automation_confidence,
            });
        }
    }

    // From temporal_rules — ActorSamplesSignal
    for rule in &semantic_ir.temporal_rules {
        for pred in rule.antecedents.iter().chain(rule.consequents.iter()) {
            if let TemporalPredicateRecord::ActorSamplesSignal {
                actor_name,
                signal_name,
                phase,
            } = pred
            {
                let already_has = relations
                    .iter()
                    .any(|r| r.sampler_actor == *actor_name && r.signal_name == *signal_name);
                if !already_has {
                    relations.push(ActorSampleRelationRecord {
                        relation_id: format!(
                            "asr_{}_{}",
                            sanitize_id(actor_name),
                            sanitize_id(signal_name)
                        ),
                        sampler_actor: actor_name.clone(),
                        signal_name: signal_name.clone(),
                        source_actor: None,
                        phase: Some(format!("{:?}", phase)),
                        source_text: rule.source_text.clone(),
                        supporting_statement_ids: rule.supporting_statement_ids.clone(),
                        automation_confidence: rule.automation_confidence,
                    });
                }
            }
        }
    }

    relations
}

fn synthesize_actor_trigger_relations(semantic_ir: &SemanticIr) -> Vec<ActorTriggerRelationRecord> {
    let mut relations = Vec::new();

    // Build actor→signals map from actor_signal_relations
    let mut actor_signals: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for rel in &semantic_ir.actor_signal_relations {
        actor_signals
            .entry(rel.actor_name.clone())
            .or_default()
            .insert(rel.signal_name.clone());
    }

    // Look for temporal_rules where antecedent involves one actor's signal
    // and consequent involves a different actor's signal → trigger relation
    for rule in &semantic_ir.temporal_rules {
        let mut antecedent_actors: BTreeSet<String> = BTreeSet::new();
        let mut consequent_actors: BTreeSet<String> = BTreeSet::new();

        for ant in &rule.antecedents {
            if let Some(actor) = predicate_actor(ant) {
                antecedent_actors.insert(actor);
            }
        }
        for cons in &rule.consequents {
            if let Some(actor) = predicate_actor(cons) {
                consequent_actors.insert(actor);
            }
        }

        // Trigger: when antecedent actor's action leads to consequent actor's action
        for source in &antecedent_actors {
            for target in &consequent_actors {
                if source != target {
                    // Find the trigger port — the signal in the consequent
                    let trigger_port = rule.consequents.iter().find_map(predicate_signal);
                    relations.push(ActorTriggerRelationRecord {
                        relation_id: format!(
                            "atr_{}_to_{}",
                            sanitize_id(source),
                            sanitize_id(target)
                        ),
                        source_actor: source.clone(),
                        target_actor: target.clone(),
                        trigger_port,
                        activation_kind: Some(ActivationKind::Do),
                        source_text: rule.source_text.clone(),
                        supporting_statement_ids: rule.supporting_statement_ids.clone(),
                        automation_confidence: rule.automation_confidence,
                    });
                }
            }
        }
    }

    relations
}

fn synthesize_actor_temporal_dependencies(
    semantic_ir: &SemanticIr,
) -> Vec<ActorTemporalDependencyRecord> {
    let mut dependencies = Vec::new();

    for rule in &semantic_ir.temporal_rules {
        // If antecedent waits for a signal (SignalValue, SignalStable, HandshakeComplete)
        // and consequent is driven by a specific actor → dependency
        let mut waiting_actor: Option<String> = None;
        let mut signal_name: Option<String> = None;

        for ant in &rule.antecedents {
            match ant {
                TemporalPredicateRecord::SignalValue {
                    signal_name: ant_signal,
                    ..
                } => {
                    // Find who drives this signal
                    let driver = semantic_ir
                        .actor_signal_relations
                        .iter()
                        .find(|r| {
                            r.signal_name == *ant_signal
                                && matches!(r.relation, RelationKind::Drives)
                        })
                        .map(|r| r.actor_name.clone());
                    waiting_actor = driver;
                    signal_name = Some(ant_signal.clone());
                }
                TemporalPredicateRecord::HandshakeComplete { .. } => {
                    waiting_actor = None;
                }
                _ => {}
            }
        }

        // Find the actor in the consequent
        let consequent_actor = rule.consequents.iter().find_map(predicate_actor);

        if let (Some(wait_for), Some(sig), Some(source)) =
            (&waiting_actor, &signal_name, &consequent_actor)
            && wait_for != source
        {
            dependencies.push(ActorTemporalDependencyRecord {
                dependency_id: format!(
                    "atd_{}_waits_{}_from_{}",
                    sanitize_id(source),
                    sanitize_id(sig),
                    sanitize_id(wait_for)
                ),
                waiting_actor: source.clone(),
                signal_name: sig.clone(),
                source_actor: Some(wait_for.clone()),
                wait_for: None,
                source_text: rule.source_text.clone(),
                supporting_statement_ids: rule.supporting_statement_ids.clone(),
                automation_confidence: rule.automation_confidence,
            });
        }
    }

    dependencies
}

fn synthesize_temporal_invariants(semantic_ir: &SemanticIr) -> Vec<TemporalInvariantRecord> {
    let mut invariants = Vec::new();

    // From InvariantRecord in SemanticIR
    for inv in &semantic_ir.invariants {
        let kind = classify_invariant_text(&inv.statement);
        invariants.push(TemporalInvariantRecord {
            invariant_id: format!("tinv_{}", inv.invariant_id),
            subject_signal: String::new(),
            invariant_kind: kind,
            condition_signal: None,
            condition_value: None,
            target_value: None,
            source_text: inv.statement.clone(),
            supporting_statement_ids: inv.supporting_statement_ids.clone(),
            automation_confidence: AutomationConfidence::Medium,
        });
    }

    // From signal_constraints (these are temporal by nature — "must not change when...")
    for sc in &semantic_ir.signal_constraints {
        let kind = signal_constraint_to_temporal_invariant_kind(&sc.constraint_kind);
        let (condition_signal, condition_value) = extract_condition_from_text(&sc.condition_text);
        invariants.push(TemporalInvariantRecord {
            invariant_id: format!("tinv_sc_{}", sc.constraint_id),
            subject_signal: sc.subject_signal.clone(),
            invariant_kind: kind,
            condition_signal,
            condition_value,
            target_value: sc.target_value.clone(),
            source_text: sc.source_text.clone(),
            supporting_statement_ids: sc.supporting_statement_ids.clone(),
            automation_confidence: sc.automation_confidence,
        });
    }

    invariants
}

// ---------------------------------------------------------------------------
// Synthesis helpers
// ---------------------------------------------------------------------------

fn render_control_expression(expr: &ControlExpressionRecord) -> String {
    match expr {
        ControlExpressionRecord::Reference { reference } => reference.base_name.clone(),
        ControlExpressionRecord::Literal { literal } => literal.clone(),
        ControlExpressionRecord::Unary { operator, operand } => {
            format!("{:?}({})", operator, render_control_expression(operand))
        }
        ControlExpressionRecord::Binary {
            operator,
            left,
            right,
        } => {
            format!(
                "({} {:?} {})",
                render_control_expression(left),
                operator,
                render_control_expression(right)
            )
        }
    }
}

fn convert_control_action_to_step(action: &ControlActionRecord) -> TransactionStep {
    match action {
        ControlActionRecord::Assign { target, value, .. } => TransactionStep::Drive {
            drive_name: target.signal_name.clone(),
            actuals: vec![render_control_expression(value)],
        },
        ControlActionRecord::Transition { target_state } => TransactionStep::Set {
            target: "state".to_string(),
            expr: target_state.clone(),
        },
        ControlActionRecord::DelayedPulse {
            target,
            delay,
            value,
        } => {
            let steps = vec![
                TransactionStep::Wait {
                    count: delay.to_string(),
                },
                TransactionStep::Drive {
                    drive_name: target.signal_name.clone(),
                    actuals: vec![render_control_expression(value)],
                },
            ];
            TransactionStep::When {
                condition: "delayed_pulse".to_string(),
                body: steps,
            }
        }
        ControlActionRecord::CompoundUpdate {
            target,
            operation,
            amount,
        } => {
            let expr = if let Some(amt) = amount {
                render_control_expression(amt)
            } else {
                "1".to_string()
            };
            match operation {
                ControlCompoundUpdateOperation::Increment => TransactionStep::Update {
                    target: target.signal_name.clone(),
                    expr: format!("+{}", expr),
                },
                ControlCompoundUpdateOperation::Decrement => TransactionStep::Update {
                    target: target.signal_name.clone(),
                    expr: format!("-{}", expr),
                },
            }
        }
    }
}

fn render_temporal_predicate(pred: &TemporalPredicateRecord) -> String {
    match pred {
        TemporalPredicateRecord::SignalValue {
            signal_name,
            value,
            phase,
        } => format!("{} == {} @{:?}", signal_name, value, phase),
        TemporalPredicateRecord::ActorDrivesSignal {
            actor_name,
            signal_name,
            phase,
        } => format!("{} drives {} @{:?}", actor_name, signal_name, phase),
        TemporalPredicateRecord::ActorMaintainsSignalStable {
            actor_name,
            signal_name,
            from_phase,
            to_phase,
        } => format!(
            "{} stable {} {:?}→{:?}",
            actor_name, signal_name, from_phase, to_phase
        ),
        TemporalPredicateRecord::SignalStable {
            signal_name,
            from_phase,
            to_phase,
        } => format!("{} stable {:?}→{:?}", signal_name, from_phase, to_phase),
        TemporalPredicateRecord::ActorSamplesSignal {
            actor_name,
            signal_name,
            phase,
        } => format!("{} samples {} @{:?}", actor_name, signal_name, phase),
        TemporalPredicateRecord::SignalSampled { signal_name, phase } => {
            format!("{} sampled @{:?}", signal_name, phase)
        }
        TemporalPredicateRecord::HandshakeComplete {
            valid_signal,
            ready_signal,
            phase,
        } => format!("{}/{} handshake @{:?}", valid_signal, ready_signal, phase),
    }
}

fn temporal_consequent_to_step(consequent: &TemporalPredicateRecord) -> Option<TransactionStep> {
    match consequent {
        TemporalPredicateRecord::ActorDrivesSignal {
            signal_name, phase, ..
        } => Some(TransactionStep::Drive {
            drive_name: signal_name.clone(),
            actuals: vec![format!("@{:?}", phase)],
        }),
        TemporalPredicateRecord::ActorSamplesSignal { signal_name, .. } => {
            Some(TransactionStep::Sample {
                port: signal_name.clone(),
                as_name: signal_name.clone(),
            })
        }
        TemporalPredicateRecord::HandshakeComplete {
            valid_signal,
            ready_signal,
            ..
        } => Some(TransactionStep::AwaitAll {
            done_port: format!("{}_{}_done", valid_signal, ready_signal),
        }),
        TemporalPredicateRecord::SignalValue {
            signal_name, value, ..
        } => Some(TransactionStep::Drive {
            drive_name: signal_name.clone(),
            actuals: vec![value.clone()],
        }),
        _ => None,
    }
}

fn predicate_actor(pred: &TemporalPredicateRecord) -> Option<String> {
    match pred {
        TemporalPredicateRecord::ActorDrivesSignal { actor_name, .. } => Some(actor_name.clone()),
        TemporalPredicateRecord::ActorMaintainsSignalStable { actor_name, .. } => {
            Some(actor_name.clone())
        }
        TemporalPredicateRecord::ActorSamplesSignal { actor_name, .. } => Some(actor_name.clone()),
        _ => None,
    }
}

fn predicate_signal(pred: &TemporalPredicateRecord) -> Option<String> {
    match pred {
        TemporalPredicateRecord::SignalValue { signal_name, .. } => Some(signal_name.clone()),
        TemporalPredicateRecord::ActorDrivesSignal { signal_name, .. } => Some(signal_name.clone()),
        TemporalPredicateRecord::ActorMaintainsSignalStable { signal_name, .. } => {
            Some(signal_name.clone())
        }
        TemporalPredicateRecord::SignalStable { signal_name, .. } => Some(signal_name.clone()),
        TemporalPredicateRecord::ActorSamplesSignal { signal_name, .. } => {
            Some(signal_name.clone())
        }
        TemporalPredicateRecord::SignalSampled { signal_name, .. } => Some(signal_name.clone()),
        TemporalPredicateRecord::HandshakeComplete { valid_signal, .. } => {
            Some(valid_signal.clone())
        }
    }
}

fn classify_invariant_text(text: &str) -> TemporalInvariantKind {
    let lower = text.to_lowercase();
    if lower.contains("must not change") || lower.contains("shall not change") {
        TemporalInvariantKind::MustNotChange
    } else if lower.contains("remain stable") || lower.contains("must be stable") {
        TemporalInvariantKind::MustRemainStable
    } else if lower.contains("must be asserted") || lower.contains("shall be asserted") {
        TemporalInvariantKind::MustBeAsserted
    } else if lower.contains("must match") || lower.contains("shall match") {
        TemporalInvariantKind::MustMatch
    } else if lower.contains("must not exceed") || lower.contains("shall not exceed") {
        TemporalInvariantKind::MustNotExceed
    } else if lower.contains("only valid when") || lower.contains("valid only when") {
        TemporalInvariantKind::OnlyValidWhen
    } else {
        TemporalInvariantKind::MustRemainStable // default for chip invariants
    }
}

fn signal_constraint_to_temporal_invariant_kind(
    kind: &SignalConstraintKind,
) -> TemporalInvariantKind {
    match kind {
        SignalConstraintKind::MustBeHigh
        | SignalConstraintKind::MustBeLow
        | SignalConstraintKind::MustBeAsserted
        | SignalConstraintKind::MustBeDeasserted => TemporalInvariantKind::MustBeAsserted,
        SignalConstraintKind::MustNotChange => TemporalInvariantKind::MustNotChange,
        SignalConstraintKind::MustBeStable => TemporalInvariantKind::MustRemainStable,
        SignalConstraintKind::MustHoldData => TemporalInvariantKind::MustMatch,
        SignalConstraintKind::MustBeValue { .. } => TemporalInvariantKind::MustBeAsserted,
    }
}

fn extract_condition_from_text(text: &Option<String>) -> (Option<String>, Option<String>) {
    let text = match text {
        Some(t) => t,
        None => return (None, None),
    };
    let lower = text.to_lowercase();
    if let Some(pos) = lower.find("when ") {
        let rest = &text[pos + 5..];
        let signal = rest
            .split_whitespace()
            .next()
            .map(|s| s.trim_matches(',').to_uppercase());
        (signal, None)
    } else {
        (None, None)
    }
}

fn sanitize_id(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_alphanumeric() || c == '_' {
                c
            } else {
                '_'
            }
        })
        .collect::<String>()
        .to_lowercase()
}

/// Extract the case value from a branch predicate for a switch statement.
/// E.g. for predicate `(HTRANS == NONSEQ)` with selector `HTRANS`, returns `NONSEQ`.
fn extract_case_value(
    predicate: &ControlExpressionRecord,
    _selector: &ControlExpressionRecord,
) -> String {
    if let ControlExpressionRecord::Binary {
        operator: ControlBinaryOperator::Eq,
        left,
        right,
    } = predicate
    {
        if let ControlExpressionRecord::Literal { literal } = right.as_ref() {
            return literal.clone();
        }
        if let ControlExpressionRecord::Literal { literal } = left.as_ref() {
            return literal.clone();
        }
    }
    render_control_expression(predicate)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::evidence::EvidenceIr;
    use crate::ir::semantic::{
        ControlBlockRole, SemanticIr, SymbolDefinitionKind, SystemResetKind, SystemResetPolarity,
        SystemResetTargetKind, SystemResetTimingRelation,
    };
    use crate::ir::source::{
        AutomationConfidence, CandidateInterpretation, ResidualDecisionPacket, SourceIr,
        StructuredTableCellRecord, StructuredTableRecord, TableKind,
    };

    use super::IntentIr;

    fn make_table_cell(text: &str, is_header: bool) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        }
    }

    #[test]
    fn builds_intent_ir_from_handshake_semantics() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("handshake.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Channel Operation\nThe transmitter must assert VALID when data is available.\n\nThe receiver may assert READY when it can accept data.\n\nVALID must remain asserted until READY is observed.\n\nThe channel is modeled as a backend-neutral transport abstraction.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(intent_ir.stage.as_str(), "intent_ir");
        assert!(intent_ir.intent_identity.intent_id.starts_with("intent_"));
        assert!(
            intent_ir
                .actors
                .iter()
                .any(|actor| actor.actor_id == "actor_transmitter"
                    && !actor.supporting_actor_ids.is_empty())
        );
        assert!(intent_ir.behaviors.iter().any(|behavior| {
            behavior
                .statement
                .contains("The transmitter must assert VALID")
                && !behavior.supporting_semantic_ids.is_empty()
        }));
        assert!(intent_ir.constraints.iter().any(|constraint| {
            constraint
                .statement
                .contains("VALID must remain asserted until READY is observed.")
                && !constraint.supporting_semantic_ids.is_empty()
        }));
        assert!(intent_ir.assumptions.iter().any(|assumption| {
            assumption
                .statement
                .contains("backend-neutral transport abstraction")
                && !assumption.supporting_semantic_ids.is_empty()
        }));
        assert!(intent_ir.residual_decisions.is_empty());

        Ok(())
    }

    #[test]
    fn preserves_semantic_residual_decisions_in_intent_ir() {
        let context = super::IntentContext {
            semantic_actors: Vec::new(),
            phases: Vec::new(),
            invariants: Vec::new(),
            assertions: Vec::new(),
            contracts: Vec::new(),
            gates: Vec::new(),
            abstractions: Vec::new(),
            residual_decisions: vec![ResidualDecisionPacket {
                packet_id: "semantic_ambiguous_visual_grounding".to_string(),
                question: "Do the ambiguous visual artifacts carry normative semantics that must be lifted into SemanticIR?".to_string(),
                why_unresolved: "EvidenceIR links the current semantic slice to ambiguous visual evidence items (visual_0001) whose role is not safely classifiable as purely illustrative.".to_string(),
                automation_confidence: AutomationConfidence::Low,
                candidate_interpretations: vec![
                    CandidateInterpretation {
                        interpretation_id: "normative_visual".to_string(),
                        description: "Treat the ambiguous visual evidence as normatively relevant and lift additional semantic structure from it.".to_string(),
                        downstream_impact: "Later stages may need richer figure parsing, OCR, or visual-sequence extraction before IntentIR is complete.".to_string(),
                    },
                    CandidateInterpretation {
                        interpretation_id: "illustrative_visual".to_string(),
                        description: "Treat the ambiguous visual evidence as explanatory context only and rely on the current text-derived semantics.".to_string(),
                        downstream_impact: "The pipeline remains deterministic now, but there is a risk of missing figure-only constraints or sequencing information.".to_string(),
                    },
                ],
            }],
        };
        let actors = vec![super::IntentActor {
            actor_id: "actor_controller".to_string(),
            actor_name: Some("Controller".to_string()),
            responsibilities: vec!["controls protocol sequencing".to_string()],
            supporting_actor_ids: vec!["actor_controller".to_string()],
        }];
        let behaviors = vec![super::BehaviorIntent {
            behavior_id: "behavior_controller".to_string(),
            statement: "Controller governs transfer behavior.".to_string(),
            actor_ids: vec!["actor_controller".to_string()],
            supporting_semantic_ids: vec!["contract_controller".to_string()],
        }];
        let constraints = vec![super::IntentConstraint {
            constraint_id: "constraint_controller".to_string(),
            statement: "XREQ must remain stable.".to_string(),
            related_interface_ids: vec!["if_controller".to_string()],
            supporting_semantic_ids: vec!["invariant_controller".to_string()],
        }];

        let residual_decisions =
            super::build_residual_decisions(&context, &actors, &behaviors, &constraints);
        assert!(
            residual_decisions
                .iter()
                .any(|packet| { packet.packet_id == "semantic_ambiguous_visual_grounding" })
        );

        let assumptions = super::build_assumptions(&context, &actors);
        assert!(assumptions.iter().any(|assumption| {
            assumption.assumption_id == "assumption_ambiguous_visual_grounding"
        }));
    }

    #[test]
    fn emits_assumption_for_semantic_roles_without_consensus() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("provisional_semantic_role.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Protocol\nSignal XREQ is input width 1.\n\nSignal XACK is output width 1.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let mut semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.residual_decisions.push(ResidualDecisionPacket {
            packet_id: "semantic_resolved_role_without_consensus".to_string(),
            question:
                "Should fallback-only semantic role resolutions remain canonical without observation-backed consensus?"
                    .to_string(),
            why_unresolved:
                "Signal XREQ currently resolves a semantic role without observation-backed consensus."
                    .to_string(),
            automation_confidence: AutomationConfidence::Medium,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "keep_provisional_role".to_string(),
                    description: "Keep the provisional semantic role.".to_string(),
                    downstream_impact:
                        "Downstream consumers keep the weaker semantic hint.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "clear_unbacked_role".to_string(),
                    description: "Clear the unbacked semantic role.".to_string(),
                    downstream_impact:
                        "Canonical meaning stays conservative until stronger evidence arrives."
                            .to_string(),
                },
            ],
        });
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert!(
            intent_ir
                .residual_decisions
                .iter()
                .any(|packet| { packet.packet_id == "semantic_resolved_role_without_consensus" })
        );
        assert!(intent_ir.assumptions.iter().any(|assumption| {
            assumption.assumption_id == "assumption_semantic_role_without_consensus"
                && !assumption.supporting_semantic_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn preserves_blocked_handshake_fallback_residual_decisions_in_intent_ir() -> Result<()> {
        use crate::ir::source::{
            ContentSectionRecord, SectionKind, SignalConstraintKind, SignalConstraintRecord,
        };

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("intent_contested_semantic_handshake_guard.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal XVALID is input width 1.\n\n",
                "Signal XACK is input width 1.\n\n",
                "Signal PAYLOAD is output width 32.\n\n",
                "Clock clk.\n\n",
                "XVALID indicates that the subordinate can accept the transfer.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.document_sections.push(ContentSectionRecord {
            section_id: "sec_0001_channel_signals".to_string(),
            title: "Channel signals".to_string(),
            heading_level: 2,
            page_id: None,
            source_ref: None,
            reading_order: 1,
            section_kind: SectionKind::SignalDescription,
        });
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_contested_semantic_handshake_desc".to_string(),
            asset_id: "table_contested_semantic_handshake_desc".to_string(),
            page_id: None,
            caption_text: Some("Handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Source", true),
                make_table_cell("Width", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("XVALID", false),
                    make_table_cell("Requester", false),
                    make_table_cell("1", false),
                    make_table_cell(
                        "Indicates that address and control information are valid for transfer.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell("Subordinate", false),
                    make_table_cell("1", false),
                    make_table_cell(
                        "Indicates that the subordinate can accept the transfer.",
                        false,
                    ),
                ],
            ],
            row_count: 2,
            col_count: 4,
        });
        source_ir.write_to_disk()?;

        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_payload_contested_semantic_handshake".to_string(),
            subject_signal: "PAYLOAD".to_string(),
            constraint_kind: SignalConstraintKind::MustNotChange,
            target_value: None,
            condition_text: Some("when XVALID is HIGH and XACK is HIGH".to_string()),
            negated: false,
            source_text: "PAYLOAD must not change when XVALID is HIGH and XACK is HIGH."
                .to_string(),
            supporting_statement_ids: vec![
                "stmt_temporal_contested_semantic_handshake".to_string(),
            ],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert!(
            intent_ir
                .residual_decisions
                .iter()
                .any(|packet| { packet.packet_id == "semantic_handshake_name_fallback_blocked" })
        );

        Ok(())
    }

    #[test]
    fn emits_assumption_for_alias_dependent_handshake_completion() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("intent_alias_dependent_handshake_completion.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal XREQ is input width 1.\n\n",
                "Signal XACK is input width 1.\n\n",
                "Signal PAYLOAD is output width 32.\n\n",
                "Clock clk.\n\n",
                "The request phase indicates that address and control information are valid for transfer.\n\n",
                "The accept phase indicates that the subordinate can accept the transfer.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir
            .signal_alias_map
            .insert("request phase".to_string(), "XREQ".to_string());
        evidence_ir
            .signal_alias_map
            .insert("accept phase".to_string(), "XACK".to_string());
        evidence_ir.refresh_signal_semantic_hints()?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_payload_alias_semantic_handshake".to_string(),
            subject_signal: "PAYLOAD".to_string(),
            constraint_kind: SignalConstraintKind::MustNotChange,
            target_value: None,
            condition_text: Some("when XREQ is HIGH and XACK is HIGH".to_string()),
            negated: false,
            source_text: "PAYLOAD must not change when XREQ is HIGH and XACK is HIGH.".to_string(),
            supporting_statement_ids: vec!["stmt_temporal_alias_semantic_handshake".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert!(
            intent_ir.residual_decisions.iter().any(|packet| {
                packet.packet_id == "semantic_alias_dependent_handshake_completion"
            })
        );
        assert!(intent_ir.assumptions.iter().any(|assumption| {
            assumption.assumption_id == "assumption_alias_dependent_handshake_completion"
                && !assumption.supporting_semantic_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn carries_typed_interface_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("comb_dt.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Explicit Control\nSignal DATA_IN is input width 8.\n\nSignal DATA_OUT is output width 8.\n\nSignal ZERO_FLAG is output width 1.\n\nBlock route_data: DATA_OUT = DATA_IN.\n\nBlock flag_zero when DATA_IN == 8'0: ZERO_FLAG = 1.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert!(intent_ir.interfaces.iter().any(|interface| {
            interface.signal_records.iter().any(|signal| {
                signal.signal_name == "DATA_IN"
                    && signal.width_hint.as_ref().and_then(|w| w.as_numeric()) == Some(8)
            })
        }));

        Ok(())
    }

    #[test]
    fn carries_symbol_definitions_and_control_blocks_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("rich_control.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Rich Explicit Control\nSignal MODE is input width 2.\n\nSignal GO is input width 1.\n\nSignal ACC is output width 8.\n\nSignal PULSE_OUT is output width 1.\n\nConstant STEP = 8'1.\n\nParam RESET_VALUE = 8'0.\n\nEnum mode_t idle = 0.\n\nEnum mode_t busy = 1.\n\nState idle is initial.\n\nState busy.\n\nBlock decode select MODE when MODE == mode_t.idle: public ACC = 8'0; transition idle.\n\nSyncReset clear_acc: ACC <- RESET_VALUE.\n\nAsyncReset clear_pulse: public PULSE_OUT = 0.\n\nBlock busy when GO: next ACC <- ACC + STEP; pulse public PULSE_OUT after 2 = 1; ACC += STEP; -> idle.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(intent_ir.symbol_definitions.len(), 3);
        assert!(intent_ir.symbol_definitions.iter().any(|definition| {
            definition.symbol_name == "STEP" && definition.kind == SymbolDefinitionKind::Constant
        }));
        assert_eq!(intent_ir.control_blocks.len(), 4);
        assert!(intent_ir.control_blocks.iter().any(|block| {
            block.block_name == "busy" && block.role == ControlBlockRole::StateBody
        }));
        assert!(intent_ir.control_blocks.iter().any(|block| {
            block.block_name == "clear_acc" && block.role == ControlBlockRole::ResetSynchronous
        }));
        assert!(
            intent_ir
                .intent_identity
                .summary
                .contains("symbol definitions")
        );
        assert!(
            intent_ir
                .intent_identity
                .summary
                .contains("structured control blocks")
        );

        Ok(())
    }

    #[test]
    fn carries_system_contract_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("seq_dt.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Explicit Sequential Control\nSignal clk is input width 1.\n\nSignal rst_n is input width 1.\n\nSignal DATA_IN is input width 8.\n\nSignal ACC is output width 8.\n\nClock clk.\n\nReset rst_n is asynchronous active low.\n\nInit ACC = 8'0.\n\nBlock accumulate: ACC <- DATA_IN.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let system_contract = intent_ir
            .system_contract
            .as_ref()
            .expect("explicit system contract should be present");
        assert_eq!(system_contract.clock_signal, "clk");
        assert_eq!(system_contract.reset_signal, "rst_n");
        assert_eq!(system_contract.reset_kind, SystemResetKind::Asynchronous);
        assert_eq!(
            system_contract.reset_polarity,
            SystemResetPolarity::ActiveLow
        );
        assert_eq!(
            system_contract.assertion_timing,
            SystemResetTimingRelation::AsynchronousToClock
        );
        assert_eq!(
            system_contract.release_timing,
            SystemResetTimingRelation::SynchronousToClock
        );
        assert_eq!(
            system_contract.target_kind,
            SystemResetTargetKind::DedicatedResetPin
        );
        assert_eq!(
            system_contract.automation_confidence,
            AutomationConfidence::High
        );
        assert!(
            intent_ir
                .intent_identity
                .summary
                .contains("explicit system contract")
        );

        Ok(())
    }

    #[test]
    fn carries_synchronous_active_high_reset_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("sync_dt.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Explicit Synchronous Control\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\nSignal DATA_IN is input width 8.\n\nSignal ACC is output width 8.\n\nClock clk.\n\nReset rst is synchronous active high.\n\nInit ACC = 8'0.\n\nBlock accumulate: ACC <- DATA_IN.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let system_contract = intent_ir
            .system_contract
            .as_ref()
            .expect("explicit system contract should be present");
        assert_eq!(system_contract.clock_signal, "clk");
        assert_eq!(system_contract.reset_signal, "rst");
        assert_eq!(system_contract.reset_kind, SystemResetKind::Synchronous);
        assert_eq!(
            system_contract.reset_polarity,
            SystemResetPolarity::ActiveHigh
        );
        assert_eq!(
            system_contract.assertion_timing,
            SystemResetTimingRelation::SynchronousToClock
        );
        assert_eq!(
            system_contract.release_timing,
            SystemResetTimingRelation::SynchronousToClock
        );
        assert_eq!(
            system_contract.target_kind,
            SystemResetTargetKind::DataInputPath
        );
        assert_eq!(
            system_contract.automation_confidence,
            AutomationConfidence::High
        );

        Ok(())
    }

    #[test]
    fn carries_inferred_reset_polarity_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("inferred_reset.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Explicit Sequential Control\nSignal clk is input width 1.\n\nSignal rst_n is input width 1.\n\nSignal DATA_IN is input width 8.\n\nSignal ACC is output width 8.\n\nClock clk.\n\nReset rst_n is asynchronous.\n\nInit ACC = 8'0.\n\nBlock accumulate: ACC <- DATA_IN.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let system_contract = intent_ir
            .system_contract
            .as_ref()
            .expect("explicit system contract should be present");
        assert_eq!(system_contract.reset_kind, SystemResetKind::Asynchronous);
        assert_eq!(
            system_contract.reset_polarity,
            SystemResetPolarity::ActiveLow
        );
        assert_eq!(
            system_contract.assertion_timing,
            SystemResetTimingRelation::AsynchronousToClock
        );
        assert_eq!(
            system_contract.release_timing,
            SystemResetTimingRelation::SynchronousToClock
        );
        assert_eq!(
            system_contract.target_kind,
            SystemResetTargetKind::DedicatedResetPin
        );
        assert_eq!(
            system_contract.automation_confidence,
            AutomationConfidence::Medium
        );

        Ok(())
    }

    #[test]
    fn carries_explicit_modules_and_tops_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("composition.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Explicit Composition\nTop datapath.\n\nTop datapath port result_data is output width 8.\n\nTop datapath child producer uses module producer_core.\n\nTop datapath child consumer uses module consumer_core.\n\nTop datapath link producer.output_data -> consumer.input_data.\n\nTop datapath link consumer.result_data -> result_data.\n\nModule producer_core signal output_data is output width 8.\n\nModule producer_core block produce: output_data = 8'3.\n\nModule consumer_core signal input_data is input width 8.\n\nModule consumer_core signal result_data is output width 8.\n\nModule consumer_core block route: result_data = input_data.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(intent_ir.explicit_modules.len(), 2);
        assert_eq!(intent_ir.explicit_tops.len(), 1);
        assert!(
            intent_ir
                .explicit_modules
                .iter()
                .any(|module| { module.module_name == "producer_core" })
        );
        assert!(intent_ir.explicit_tops.iter().any(|top| {
            top.top_name == "datapath" && top.children.len() == 2 && top.links.len() == 2
        }));
        assert!(
            intent_ir
                .intent_identity
                .summary
                .contains("explicit modules")
        );
        assert!(intent_ir.intent_identity.summary.contains("explicit tops"));

        Ok(())
    }

    #[test]
    fn carries_regular_states_and_state_transitions_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("explicit_fsm.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Explicit FSM Control\nSignal clk is input width 1.\n\nSignal rst_n is input width 1.\n\nSignal GO is input width 1.\n\nSignal DONE is input width 1.\n\nSignal DATA_IN is input width 8.\n\nSignal ACC is output width 8.\n\nClock clk.\n\nReset rst_n is asynchronous active low.\n\nInit ACC = 8'0.\n\nState idle is initial.\n\nState busy.\n\nBlock idle: ACC <- DATA_IN.\n\nTransition idle -> busy when GO.\n\nBlock busy: ACC <- ACC.\n\nTransition busy -> idle when DONE.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(intent_ir.regular_states.len(), 2);
        assert!(
            intent_ir
                .regular_states
                .iter()
                .any(|state| state.state_name == "idle" && state.is_initial)
        );
        assert_eq!(intent_ir.state_transitions.len(), 2);
        assert!(intent_ir.state_transitions.iter().any(|transition| {
            transition.source_state == "idle" && transition.target_state == "busy"
        }));
        assert!(
            intent_ir
                .intent_identity
                .summary
                .contains("state transitions")
        );

        Ok(())
    }

    #[test]
    fn carries_actor_relative_ports_and_connectivity_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("kg_intent.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Protocol\nSignal PREADY is output width 1.\n\nThe Completer drives PREADY.\n\nThe Requester reads PREADY.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(
            intent_ir.actor_signal_relations.len(),
            semantic_ir.actor_signal_relations.len()
        );
        assert_eq!(intent_ir.actor_ports.len(), semantic_ir.actor_ports.len());
        assert_eq!(
            intent_ir.signal_connectivity.len(),
            semantic_ir.signal_connectivity.len()
        );
        assert_eq!(
            intent_ir.interface_signal_conflicts.len(),
            semantic_ir.interface_signal_conflicts.len()
        );
        assert_eq!(
            intent_ir.signal_connectivity_conflicts.len(),
            semantic_ir.signal_connectivity_conflicts.len()
        );
        assert!(intent_ir.actors.iter().any(|actor| {
            actor
                .actor_name
                .as_deref()
                .map(|name| name.eq_ignore_ascii_case("Completer"))
                .unwrap_or(false)
                && !actor.supporting_actor_ids.is_empty()
        }));
        assert!(intent_ir.actor_ports.iter().any(|port| {
            port.actor_name.eq_ignore_ascii_case("Requester") && port.signal_name == "PREADY"
        }));
        assert!(intent_ir.signal_connectivity.iter().any(|record| {
            record.signal_name == "PREADY"
                && record
                    .producer_actor_names
                    .iter()
                    .any(|name| name.eq_ignore_ascii_case("Completer"))
        }));

        Ok(())
    }

    #[test]
    fn carries_infrastructure_signal_connectivity_class_into_intent_ir() -> Result<()> {
        use crate::ir::evidence::{EvidenceModality, ExtractedStatement, StatementClass};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_infra_connectivity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal ACLK is input width 1.\n",
                "Signal ARESETN is input width 1.\n",
                "Signal XREQ is output width 1.\n\n",
                "Clock ACLK.\n",
                "Reset ARESETN is asynchronous active low.\n",
                "The Requester drives XREQ.\n",
                "The Completer reads XREQ.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.extracted_statements.push(ExtractedStatement {
            statement_id: "statement_clock".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: "Clock ACLK.".to_string(),
            evidence_span_ids: Vec::new(),
            related_visual_evidence_ids: Vec::new(),
        });
        evidence_ir.extracted_statements.push(ExtractedStatement {
            statement_id: "statement_reset".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: "Reset ARESETN is asynchronous active low.".to_string(),
            evidence_span_ids: Vec::new(),
            related_visual_evidence_ids: Vec::new(),
        });
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert!(intent_ir.signal_connectivity.iter().any(|record| {
            record.signal_name == "ACLK"
                && record.connectivity_class
                    == crate::ir::semantic::SignalConnectivityClass::SystemClock
                && record.producer_actor_ids.is_empty()
        }));
        assert!(intent_ir.signal_connectivity.iter().any(|record| {
            record.signal_name == "ARESETN"
                && record.connectivity_class
                    == crate::ir::semantic::SignalConnectivityClass::SystemReset
                && record.producer_actor_ids.is_empty()
        }));
        assert!(intent_ir.infrastructure_signals.iter().any(|record| {
            record.signal_name == "ACLK"
                && record.kind == crate::ir::semantic::InfrastructureSignalKind::SystemClock
                && record.source_status
                    == crate::ir::semantic::InfrastructureSignalSourceStatus::UnresolvedSource
                && record.distribution_status
                    == crate::ir::semantic::InfrastructureSignalDistributionStatus::SharedRecoveredConsumers
        }));
        assert!(intent_ir.infrastructure_signals.iter().any(|record| {
            record.signal_name == "ARESETN"
                && record.kind == crate::ir::semantic::InfrastructureSignalKind::SystemReset
                && record.source_status
                    == crate::ir::semantic::InfrastructureSignalSourceStatus::UnresolvedSource
                && record.distribution_status
                    == crate::ir::semantic::InfrastructureSignalDistributionStatus::SharedRecoveredConsumers
        }));

        Ok(())
    }

    #[test]
    fn carries_interface_signal_conflicts_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_signal_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal DATA is input width 8.\n\n",
                "Signal DATA is output width 16.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(intent_ir.interface_signal_conflicts.len(), 2);
        assert!(intent_ir.interface_signal_conflicts.iter().any(|conflict| {
            conflict.signal_name == "DATA"
                && conflict
                    .observations
                    .iter()
                    .any(|observation| observation.value_text == "input")
                && conflict
                    .observations
                    .iter()
                    .any(|observation| observation.value_text == "output")
        }));
        assert!(intent_ir.interface_signal_conflicts.iter().any(|conflict| {
            conflict.signal_name == "DATA"
                && conflict
                    .observations
                    .iter()
                    .any(|observation| observation.value_text == "8")
                && conflict
                    .observations
                    .iter()
                    .any(|observation| observation.value_text == "16")
        }));

        Ok(())
    }

    #[test]
    fn carries_signal_connectivity_conflicts_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_kg_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n\n",
                "The Completer drives PREADY.\n\n",
                "The Monitor drives PREADY.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(intent_ir.signal_connectivity_conflicts.len(), 1);
        let conflict = &intent_ir.signal_connectivity_conflicts[0];
        assert_eq!(conflict.signal_name, "PREADY");
        assert!(
            conflict
                .conflicting_actor_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case("Completer"))
        );
        assert!(
            conflict
                .conflicting_actor_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case("Monitor"))
        );

        Ok(())
    }

    #[test]
    fn carries_signal_semantic_conflicts_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_semantic_role_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal XCTRL is input width 1.\n\n",
                "XCTRL indicates that the subordinate can accept the transfer.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_semantic_conflict".to_string(),
            asset_id: "asset_semantic_conflict".to_string(),
            page_id: None,
            caption_text: Some("Control signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("XCTRL", false),
                make_table_cell(
                    "Indicates that address and control information are valid for transfer.",
                    false,
                ),
            ]],
            row_count: 1,
            col_count: 2,
        });
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(intent_ir.signal_semantic_conflicts.len(), 1);
        let conflict = &intent_ir.signal_semantic_conflicts[0];
        assert_eq!(conflict.signal_name, "XCTRL");
        assert!(conflict.observations.iter().any(|observation| {
            observation
                .semantic_tags
                .contains(&crate::ir::evidence::SignalSemanticTag::HandshakeValidLike)
        }));
        assert!(conflict.observations.iter().any(|observation| {
            observation
                .semantic_tags
                .contains(&crate::ir::evidence::SignalSemanticTag::HandshakeReadyLike)
        }));
        let xctrl = intent_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XCTRL")
            .expect("expected XCTRL interface signal");
        assert_eq!(xctrl.semantic_candidates.len(), 2);
        assert!(xctrl.resolved_semantic_role.is_none());
        assert!(xctrl.semantic_consensus.is_none());
        let arbitration = xctrl
            .semantic_arbitration
            .as_ref()
            .expect("expected XCTRL semantic arbitration");
        assert_eq!(arbitration.candidate_count, 2);
        assert_eq!(
            arbitration.leading_role,
            crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeValidLike
        );
        assert_eq!(arbitration.leading_evidence_weight, 6);
        assert_eq!(
            arbitration.runner_up_role,
            Some(crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeReadyLike)
        );
        assert_eq!(arbitration.runner_up_evidence_weight, Some(3));
        assert_eq!(arbitration.margin_over_runner_up, Some(3));
        assert!(!arbitration.decisive);
        assert!(xctrl.semantic_candidates.iter().any(|candidate| {
            candidate.role == crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeValidLike
                && candidate.evidence_weight == 6
        }));
        assert!(xctrl.semantic_candidates.iter().any(|candidate| {
            candidate.role == crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeReadyLike
                && candidate.evidence_weight == 3
        }));

        Ok(())
    }

    #[test]
    fn carries_signal_polarity_conflicts_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_polarity_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Reset\n",
                "Signal PRESETN is input width 1.\n\n",
                "PRESETN is active HIGH.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_reset_desc_conflict".to_string(),
            asset_id: "asset_reset_desc_conflict".to_string(),
            page_id: None,
            caption_text: Some("Reset signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("PRESETN", false),
                make_table_cell("Active low reset.", false),
            ]],
            row_count: 1,
            col_count: 2,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(intent_ir.signal_polarity_conflicts.len(), 1);
        let conflict = &intent_ir.signal_polarity_conflicts[0];
        assert_eq!(conflict.signal_name, "PRESETN");
        assert_eq!(conflict.observations.len(), 2);

        Ok(())
    }

    #[test]
    fn carries_resolved_signal_polarity_into_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_resolved_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Reset\n",
                "Signal PRESETN is input width 1.\n\n",
                "PRESETN is an active low reset signal.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let presetn = intent_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "PRESETN")
            .expect("expected PRESETN interface signal");
        assert_eq!(
            presetn.resolved_polarity,
            Some(crate::ir::evidence::SignalPolarity::ActiveLow)
        );

        Ok(())
    }

    #[test]
    fn carries_temporal_rules_into_intent_ir() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::semantic::{TemporalPredicateRecord, TickPhase};
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_temporal.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal HREADY is input width 1.\n\n",
                "Signal HTRANS is input width 2.\n\n",
                "Clock clk.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_htrans_stable".to_string(),
            subject_signal: "HTRANS".to_string(),
            constraint_kind: SignalConstraintKind::MustNotChange,
            target_value: None,
            condition_text: Some("when HREADY is LOW".to_string()),
            negated: false,
            source_text: "HTRANS must not change when HREADY is LOW for 2 cycles.".to_string(),
            supporting_statement_ids: vec!["stmt_temporal".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(
            intent_ir.temporal_rules.len(),
            semantic_ir.temporal_rules.len()
        );
        assert!(intent_ir.temporal_rules.iter().any(|rule| {
            rule.clock_signal.as_deref() == Some("clk")
                && matches!(
                    rule.cycle_window.as_ref(),
                    Some(crate::ir::semantic::CycleWindowRecord {
                        min_cycles: Some(2),
                        max_cycles: Some(2),
                    })
                )
                && rule.consequents.iter().any(|predicate| {
                    matches!(
                        predicate,
                        TemporalPredicateRecord::SignalStable {
                            signal_name,
                            from_phase: TickPhase::PreTick,
                            to_phase: TickPhase::PostTick,
                        } if signal_name == "HTRANS"
                    )
                })
                && rule.automation_confidence == AutomationConfidence::Medium
                && !rule.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn carries_actor_grounded_temporal_rules_into_intent_ir() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::semantic::{TemporalPredicateRecord, TickPhase};
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_temporal_actor.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal PREADY is output width 1.\n\n",
                "Clock clk.\n\n",
                "The Completer drives PREADY.\n\n",
                "The Requester reads PREADY.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_asserted".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted within 2 cycles.".to_string(),
            supporting_statement_ids: vec!["stmt_actor_temporal".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert!(intent_ir.temporal_rules.iter().any(|rule| {
            rule.consequents.iter().any(|predicate| {
                matches!(
                    predicate,
                    TemporalPredicateRecord::ActorDrivesSignal {
                        actor_name,
                        signal_name,
                        phase: TickPhase::PostTick,
                    } if actor_name.eq_ignore_ascii_case("Completer") && signal_name == "PREADY"
                )
            }) && rule.automation_confidence == AutomationConfidence::Medium
                && !rule.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn carries_actor_grounded_stability_rules_into_intent_ir() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::semantic::{TemporalPredicateRecord, TickPhase};
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_temporal_actor_stable.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal PREADY is output width 1.\n\n",
                "Clock clk.\n\n",
                "The Completer drives PREADY.\n\n",
                "The Requester reads PREADY.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_stable".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeStable,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be stable for 2 cycles.".to_string(),
            supporting_statement_ids: vec!["stmt_actor_stable".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert!(intent_ir.temporal_rules.iter().any(|rule| {
            rule.consequents.iter().any(|predicate| {
                matches!(
                    predicate,
                    TemporalPredicateRecord::ActorMaintainsSignalStable {
                        actor_name,
                        signal_name,
                        from_phase: TickPhase::PreTick,
                        to_phase: TickPhase::PostTick,
                    } if actor_name.eq_ignore_ascii_case("Completer") && signal_name == "PREADY"
                )
            }) && rule.automation_confidence == AutomationConfidence::Medium
                && !rule.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn carries_multi_predicate_temporal_antecedents_into_intent_ir() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::semantic::{TemporalPredicateRecord, TickPhase};
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_temporal_compound_guard.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal HREADY is input width 1.\n\n",
                "Signal HSEL is input width 1.\n\n",
                "Signal HTRANS is output width 2.\n\n",
                "Clock clk.\n\n",
                "The Manager drives HTRANS.\n\n",
                "The Subordinate reads HTRANS.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_htrans_compound_guard".to_string(),
            subject_signal: "HTRANS".to_string(),
            constraint_kind: SignalConstraintKind::MustNotChange,
            target_value: None,
            condition_text: Some("when HREADY is LOW and HSEL is HIGH".to_string()),
            negated: false,
            source_text: "HTRANS must not change when HREADY is LOW and HSEL is HIGH.".to_string(),
            supporting_statement_ids: vec!["stmt_temporal_compound_guard".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert!(intent_ir.temporal_rules.iter().any(|rule| {
            rule.rule_id == "temporal_signal_constraint_sigcon_htrans_compound_guard"
                && rule.antecedents.len() == 2
                && rule.antecedents.iter().any(|predicate| {
                    matches!(
                        predicate,
                        TemporalPredicateRecord::SignalValue {
                            signal_name,
                            value,
                            phase: TickPhase::PreTick,
                        } if signal_name == "HREADY" && value == "LOW"
                    )
                })
                && rule.antecedents.iter().any(|predicate| {
                    matches!(
                        predicate,
                        TemporalPredicateRecord::SignalValue {
                            signal_name,
                            value,
                            phase: TickPhase::PreTick,
                        } if signal_name == "HSEL" && value == "HIGH"
                    )
                })
                && rule.automation_confidence == AutomationConfidence::Medium
                && !rule.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn carries_handshake_temporal_predicates_into_intent_ir() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::semantic::{TemporalPredicateRecord, TickPhase};
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_temporal_handshake.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal AWVALID is input width 1.\n\n",
                "Signal AWREADY is input width 1.\n\n",
                "Signal PAYLOAD is output width 32.\n\n",
                "Clock clk.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_payload_handshake".to_string(),
            subject_signal: "PAYLOAD".to_string(),
            constraint_kind: SignalConstraintKind::MustNotChange,
            target_value: None,
            condition_text: Some("when AWVALID is HIGH and AWREADY is HIGH".to_string()),
            negated: false,
            source_text: "PAYLOAD must not change when AWVALID is HIGH and AWREADY is HIGH."
                .to_string(),
            supporting_statement_ids: vec!["stmt_temporal_handshake".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert!(intent_ir.temporal_rules.iter().any(|rule| {
            rule.rule_id == "temporal_signal_constraint_sigcon_payload_handshake"
                && rule.antecedents.iter().any(|predicate| {
                    matches!(
                        predicate,
                        TemporalPredicateRecord::HandshakeComplete {
                            valid_signal,
                            ready_signal,
                            phase: TickPhase::PreTick,
                        } if valid_signal == "AWVALID" && ready_signal == "AWREADY"
                    )
                })
                && rule.automation_confidence == AutomationConfidence::Medium
                && !rule.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn carries_signal_semantic_tags_into_intent_ir() -> Result<()> {
        use crate::ir::evidence::{EvidenceIr, SignalSemanticTag};
        use crate::ir::source::{
            ContentSectionRecord, SectionKind, StructuredTableCellRecord, StructuredTableRecord,
            TableKind,
        };

        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_signal_semantic_tags.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal XREQ is output width 1.\n\n",
                "Signal XACK is input width 1.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.document_sections.push(ContentSectionRecord {
            section_id: "sec_0001_channel_signals".to_string(),
            title: "Channel signals".to_string(),
            heading_level: 2,
            page_id: None,
            source_ref: None,
            reading_order: 1,
            section_kind: SectionKind::SignalDescription,
        });
        let make_cell = |text: &str, is_header: bool| StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        };
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_signal_semantic_tags".to_string(),
            asset_id: "table_signal_semantic_tags".to_string(),
            page_id: None,
            caption_text: Some("Handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_cell("Signal", true),
                make_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_cell("XREQ", false),
                    make_cell(
                        "Indicates that address and control information are valid for transfer.",
                        false,
                    ),
                ],
                vec![
                    make_cell("XACK", false),
                    make_cell(
                        "Indicates that the subordinate can accept the transfer.",
                        false,
                    ),
                ],
            ],
            row_count: 2,
            col_count: 2,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let xreq = intent_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XREQ")
            .expect("expected XREQ interface signal");
        assert!(
            xreq.semantic_tags
                .contains(&SignalSemanticTag::HandshakeValidLike)
        );
        assert_eq!(
            xreq.resolved_semantic_role,
            Some(crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeValidLike)
        );
        assert_eq!(
            xreq.semantic_grounding_strength,
            Some(crate::ir::semantic::SemanticGroundingStrength::SingleSource)
        );
        let xreq_consensus = xreq
            .semantic_consensus
            .as_ref()
            .expect("expected XREQ semantic consensus");
        assert_eq!(
            xreq_consensus.role,
            crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeValidLike
        );
        assert_eq!(
            xreq_consensus.grounding_strength,
            crate::ir::semantic::SemanticGroundingStrength::SingleSource
        );
        assert_eq!(xreq_consensus.supporting_observation_count, 1);
        assert_eq!(
            xreq_consensus.supporting_source_kinds,
            vec![crate::ir::evidence::SignalSemanticHintSourceKind::SignalDescriptionTable]
        );
        assert_eq!(
            xreq_consensus.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xreq.semantic_candidates.len(), 1);
        let xreq_candidate = &xreq.semantic_candidates[0];
        assert_eq!(
            xreq_candidate.role,
            crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeValidLike
        );
        assert_eq!(
            xreq_candidate.grounding_strength,
            crate::ir::semantic::SemanticGroundingStrength::SingleSource
        );
        assert_eq!(xreq_candidate.supporting_observation_count, 1);
        assert_eq!(
            xreq_candidate.supporting_source_kinds,
            vec![crate::ir::evidence::SignalSemanticHintSourceKind::SignalDescriptionTable]
        );
        assert_eq!(
            xreq_candidate.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xreq_candidate.evidence_weight, 6);
        let xreq_arbitration = xreq
            .semantic_arbitration
            .as_ref()
            .expect("expected XREQ semantic arbitration");
        assert_eq!(xreq_arbitration.candidate_count, 1);
        assert_eq!(
            xreq_arbitration.leading_role,
            crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeValidLike
        );
        assert_eq!(xreq_arbitration.leading_evidence_weight, 6);
        assert_eq!(xreq_arbitration.runner_up_role, None);
        assert_eq!(xreq_arbitration.runner_up_evidence_weight, None);
        assert_eq!(xreq_arbitration.margin_over_runner_up, None);
        assert!(xreq_arbitration.decisive);
        assert_eq!(xreq.semantic_observations.len(), 1);
        assert!(xreq.semantic_observations.iter().any(|observation| {
            matches!(
                observation.source_kind,
                crate::ir::evidence::SignalSemanticHintSourceKind::SignalDescriptionTable
            )
        }));

        let xack = intent_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XACK")
            .expect("expected XACK interface signal");
        assert!(
            xack.semantic_tags
                .contains(&SignalSemanticTag::HandshakeReadyLike)
        );
        assert_eq!(
            xack.resolved_semantic_role,
            Some(crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeReadyLike)
        );
        assert_eq!(
            xack.semantic_grounding_strength,
            Some(crate::ir::semantic::SemanticGroundingStrength::SingleSource)
        );
        let xack_consensus = xack
            .semantic_consensus
            .as_ref()
            .expect("expected XACK semantic consensus");
        assert_eq!(
            xack_consensus.role,
            crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeReadyLike
        );
        assert_eq!(
            xack_consensus.grounding_strength,
            crate::ir::semantic::SemanticGroundingStrength::SingleSource
        );
        assert_eq!(xack_consensus.supporting_observation_count, 1);
        assert_eq!(
            xack_consensus.supporting_source_kinds,
            vec![crate::ir::evidence::SignalSemanticHintSourceKind::SignalDescriptionTable]
        );
        assert_eq!(
            xack_consensus.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xack.semantic_candidates.len(), 1);
        let xack_candidate = &xack.semantic_candidates[0];
        assert_eq!(
            xack_candidate.role,
            crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeReadyLike
        );
        assert_eq!(
            xack_candidate.grounding_strength,
            crate::ir::semantic::SemanticGroundingStrength::SingleSource
        );
        assert_eq!(xack_candidate.supporting_observation_count, 1);
        assert_eq!(
            xack_candidate.supporting_source_kinds,
            vec![crate::ir::evidence::SignalSemanticHintSourceKind::SignalDescriptionTable]
        );
        assert_eq!(
            xack_candidate.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xack_candidate.evidence_weight, 6);
        let xack_arbitration = xack
            .semantic_arbitration
            .as_ref()
            .expect("expected XACK semantic arbitration");
        assert_eq!(xack_arbitration.candidate_count, 1);
        assert_eq!(
            xack_arbitration.leading_role,
            crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeReadyLike
        );
        assert_eq!(xack_arbitration.leading_evidence_weight, 6);
        assert_eq!(xack_arbitration.runner_up_role, None);
        assert_eq!(xack_arbitration.runner_up_evidence_weight, None);
        assert_eq!(xack_arbitration.margin_over_runner_up, None);
        assert!(xack_arbitration.decisive);
        assert_eq!(xack.semantic_observations.len(), 1);
        assert!(xack.semantic_observations.iter().any(|observation| {
            matches!(
                observation.source_kind,
                crate::ir::evidence::SignalSemanticHintSourceKind::SignalDescriptionTable
            )
        }));

        Ok(())
    }

    #[test]
    fn carries_cross_modality_semantic_grounding_into_intent_ir() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{
            StructuredTableCellRecord, StructuredTableRecord, TableKind, VisualAsset,
            VisualAssetKind,
        };

        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_semantic_cross_modality.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(&source, "# Protocol\nSignal XREQ is output width 1.\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "figure_xreq".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 1: XREQ valid timing.".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: None,
            diagram_kind: crate::ir::source::DiagramKind::TimingDiagram,
        });
        let make_cell = |text: &str, is_header: bool| StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        };
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_xreq_roles".to_string(),
            asset_id: "table_xreq_roles".to_string(),
            page_id: None,
            caption_text: Some("Handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_cell("Signal", true),
                make_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_cell("XREQ", false),
                make_cell(
                    "Indicates that address and control information are valid for transfer.",
                    false,
                ),
            ]],
            row_count: 1,
            col_count: 2,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let xreq = intent_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XREQ")
            .expect("expected XREQ interface signal");
        assert_eq!(
            xreq.resolved_semantic_role,
            Some(crate::ir::semantic::InterfaceSignalSemanticRole::HandshakeValidLike)
        );
        assert_eq!(
            xreq.semantic_grounding_strength,
            Some(crate::ir::semantic::SemanticGroundingStrength::CrossModality)
        );
        let xreq_consensus = xreq
            .semantic_consensus
            .as_ref()
            .expect("expected XREQ semantic consensus");
        assert_eq!(
            xreq_consensus.grounding_strength,
            crate::ir::semantic::SemanticGroundingStrength::CrossModality
        );
        assert_eq!(xreq_consensus.supporting_observation_count, 2);
        assert_eq!(
            xreq_consensus.supporting_source_kinds,
            vec![
                crate::ir::evidence::SignalSemanticHintSourceKind::SignalDescriptionTable,
                crate::ir::evidence::SignalSemanticHintSourceKind::VisualCaption
            ]
        );
        assert_eq!(
            xreq_consensus.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xreq.semantic_candidates.len(), 1);
        let xreq_candidate = &xreq.semantic_candidates[0];
        assert_eq!(
            xreq_candidate.grounding_strength,
            crate::ir::semantic::SemanticGroundingStrength::CrossModality
        );
        assert_eq!(xreq_candidate.supporting_observation_count, 2);
        assert_eq!(
            xreq_candidate.supporting_source_kinds,
            vec![
                crate::ir::evidence::SignalSemanticHintSourceKind::SignalDescriptionTable,
                crate::ir::evidence::SignalSemanticHintSourceKind::VisualCaption
            ]
        );
        assert_eq!(
            xreq_candidate.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xreq_candidate.evidence_weight, 10);
        assert_eq!(xreq.semantic_observations.len(), 2);

        Ok(())
    }

    #[test]
    fn carries_typed_temporal_conflicts_into_intent_ir() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_temporal_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal HREADY is input width 1.\n\n",
                "Signal PREADY is output width 1.\n\n",
                "Clock clk.\n\n",
                "The Completer drives PREADY.\n\n",
                "The Requester reads PREADY.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_high".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeHigh,
            target_value: None,
            condition_text: Some("when HREADY is LOW".to_string()),
            negated: false,
            source_text: "PREADY must be HIGH when HREADY is LOW.".to_string(),
            supporting_statement_ids: vec!["stmt_pready_high".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_low".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeLow,
            target_value: None,
            condition_text: Some("when HREADY is LOW".to_string()),
            negated: false,
            source_text: "PREADY must be LOW when HREADY is LOW.".to_string(),
            supporting_statement_ids: vec!["stmt_pready_low".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        semantic_ir.write_to_disk()?;

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        assert_eq!(intent_ir.temporal_conflicts.len(), 1);
        let conflict = &intent_ir.temporal_conflicts[0];
        assert_eq!(conflict.signal_name, "PREADY");
        assert_eq!(
            conflict.conflicting_values,
            vec!["HIGH".to_string(), "LOW".to_string()]
        );
        assert_eq!(conflict.automation_confidence, AutomationConfidence::Medium);
        assert!(!conflict.supporting_statement_ids.is_empty());

        Ok(())
    }

    // ── overlaps unit tests ──────────────────────────────────────────────

    #[test]
    fn overlaps_empty_left_returns_false() {
        assert!(!super::overlaps(&[], &["a".to_string()]));
    }

    #[test]
    fn overlaps_empty_right_returns_false() {
        assert!(!super::overlaps(&["a".to_string()], &[]));
    }

    #[test]
    fn overlaps_returns_true_when_common_element_exists() {
        assert!(super::overlaps(
            &["a".to_string(), "b".to_string()],
            &["b".to_string(), "c".to_string()]
        ));
    }

    #[test]
    fn overlaps_returns_false_when_no_common_element() {
        assert!(!super::overlaps(&["a".to_string()], &["b".to_string()]));
    }

    #[test]
    fn overlaps_empty_both_returns_false() {
        assert!(!super::overlaps(&[], &[]));
    }

    // ── dedup tests ─────────────────────────────────────────────────────

    #[test]
    fn build_behaviors_deduplicates_phases() {
        use std::collections::BTreeSet;
        let context = super::IntentContext {
            semantic_actors: Vec::new(),
            phases: vec![
                super::PhaseContext {
                    phase_id: "phase_1".to_string(),
                    summary: "Address phase".to_string(),
                    supporting_statement_ids: vec![],
                    supporting_section_ids: vec![],
                },
                super::PhaseContext {
                    phase_id: "phase_2".to_string(),
                    summary: "Address phase".to_string(), // duplicate summary
                    supporting_statement_ids: vec![],
                    supporting_section_ids: vec![],
                },
            ],
            invariants: Vec::new(),
            assertions: Vec::new(),
            contracts: Vec::new(),
            gates: Vec::new(),
            abstractions: Vec::new(),
            residual_decisions: Vec::new(),
        };
        let actor_ids = BTreeSet::new();
        let behaviors = super::build_behaviors(&context, actor_ids);
        assert_eq!(behaviors.len(), 1, "duplicate phases must be deduped");
        assert!(behaviors[0].statement.contains("Address phase"));
        assert_eq!(
            behaviors[0].behavior_id, "behavior_phase_1",
            "must keep first phase, not second"
        );
    }

    #[test]
    fn build_constraints_deduplicates_invariants() {
        let context = super::IntentContext {
            semantic_actors: Vec::new(),
            phases: Vec::new(),
            invariants: vec![
                super::ConstraintSourceContext {
                    source_id: "inv_1".to_string(),
                    statement: "XREQ must remain stable".to_string(),
                    related_interface_ids: vec![],
                },
                super::ConstraintSourceContext {
                    source_id: "inv_2".to_string(),
                    statement: "XREQ must remain stable".to_string(), // duplicate
                    related_interface_ids: vec![],
                },
            ],
            assertions: Vec::new(),
            contracts: Vec::new(),
            gates: Vec::new(),
            abstractions: Vec::new(),
            residual_decisions: Vec::new(),
        };
        let constraints = super::build_constraints(&context);
        assert_eq!(constraints.len(), 1, "duplicate invariants must be deduped");
        assert!(constraints[0].statement.contains("XREQ"));
        assert_eq!(
            constraints[0].constraint_id, "constraint_inv_1",
            "must keep first invariant, not second"
        );
    }

    // ── build_assumptions missing packet_id tests ────────────────────────

    #[test]
    fn build_assumptions_emits_for_semantic_role_without_consensus() {
        let context = super::IntentContext {
            semantic_actors: Vec::new(),
            phases: Vec::new(),
            invariants: Vec::new(),
            assertions: Vec::new(),
            contracts: Vec::new(),
            gates: Vec::new(),
            abstractions: Vec::new(),
            residual_decisions: vec![ResidualDecisionPacket {
                packet_id: "semantic_resolved_role_without_consensus".to_string(),
                question: "Is the role consensus resolved?".to_string(),
                why_unresolved: "Still provisional.".to_string(),
                automation_confidence: AutomationConfidence::Low,
                candidate_interpretations: vec![],
            }],
        };
        let assumptions = super::build_assumptions(&context, &[]);
        assert!(
            assumptions
                .iter()
                .any(|a| { a.assumption_id == "assumption_semantic_role_without_consensus" })
        );
    }

    #[test]
    fn build_assumptions_emits_for_alias_dependent_handshake_completion() {
        let context = super::IntentContext {
            semantic_actors: Vec::new(),
            phases: Vec::new(),
            invariants: Vec::new(),
            assertions: Vec::new(),
            contracts: Vec::new(),
            gates: Vec::new(),
            abstractions: Vec::new(),
            residual_decisions: vec![ResidualDecisionPacket {
                packet_id: "semantic_alias_dependent_handshake_completion".to_string(),
                question: "Is handshake completion alias-dependent?".to_string(),
                why_unresolved: "Alias dependent.".to_string(),
                automation_confidence: AutomationConfidence::Low,
                candidate_interpretations: vec![],
            }],
        };
        let assumptions = super::build_assumptions(&context, &[]);
        assert!(
            assumptions
                .iter()
                .any(|a| { a.assumption_id == "assumption_alias_dependent_handshake_completion" })
        );
    }

    #[test]
    fn build_behaviors_deduplicates_gates() {
        use std::collections::BTreeSet;
        let context = super::IntentContext {
            semantic_actors: Vec::new(),
            phases: Vec::new(),
            invariants: Vec::new(),
            assertions: Vec::new(),
            contracts: Vec::new(),
            gates: vec![
                super::GateContext {
                    gate_id: "gate_1".to_string(),
                    condition: "HREADY is HIGH".to_string(),
                    related_interface_ids: vec![],
                },
                super::GateContext {
                    gate_id: "gate_2".to_string(),
                    condition: "HREADY is HIGH".to_string(), // duplicate
                    related_interface_ids: vec![],
                },
            ],
            abstractions: Vec::new(),
            residual_decisions: Vec::new(),
        };
        let actor_ids = BTreeSet::new();
        let behaviors = super::build_behaviors(&context, actor_ids);
        assert_eq!(behaviors.len(), 1, "duplicate gates must be deduped");
        assert_eq!(behaviors[0].behavior_id, "behavior_gate_1");
    }

    #[test]
    fn build_constraints_deduplicates_assertions() {
        let context = super::IntentContext {
            semantic_actors: Vec::new(),
            phases: Vec::new(),
            invariants: Vec::new(),
            assertions: vec![
                super::ConstraintSourceContext {
                    source_id: "asrt_1".to_string(),
                    statement: "XREQ is stable".to_string(),
                    related_interface_ids: vec![],
                },
                super::ConstraintSourceContext {
                    source_id: "asrt_2".to_string(),
                    statement: "XREQ is stable".to_string(), // duplicate
                    related_interface_ids: vec![],
                },
            ],
            contracts: Vec::new(),
            gates: Vec::new(),
            abstractions: Vec::new(),
            residual_decisions: Vec::new(),
        };
        let constraints = super::build_constraints(&context);
        assert_eq!(constraints.len(), 1, "duplicate assertions must be deduped");
        assert_eq!(constraints[0].constraint_id, "constraint_asrt_1");
    }

    #[test]
    fn build_constraints_deduplicates_gates() {
        let context = super::IntentContext {
            semantic_actors: Vec::new(),
            phases: Vec::new(),
            invariants: Vec::new(),
            assertions: Vec::new(),
            contracts: Vec::new(),
            gates: vec![
                super::GateContext {
                    gate_id: "gate_3".to_string(),
                    condition: "HREADY is HIGH".to_string(),
                    related_interface_ids: vec!["if_1".to_string()],
                },
                super::GateContext {
                    gate_id: "gate_4".to_string(),
                    condition: "HREADY is HIGH".to_string(), // duplicate
                    related_interface_ids: vec!["if_2".to_string()],
                },
            ],
            abstractions: Vec::new(),
            residual_decisions: Vec::new(),
        };
        let constraints = super::build_constraints(&context);
        assert_eq!(constraints.len(), 1, "duplicate gates must be deduped");
        assert_eq!(constraints[0].constraint_id, "constraint_gate_3_gate");
    }

    // ── build_intent_actors overlaps test ────────────────────────────────

    #[test]
    fn build_intent_actors_attaches_phase_when_statements_overlap() {
        let context = super::IntentContext {
            semantic_actors: vec![super::SemanticActorContext {
                actor_id: "actor_tx".to_string(),
                actor_name: Some("Transmitter".to_string()),
                role_summary: String::new(),
                supporting_statement_ids: vec!["stmt_1".to_string()],
                supporting_section_ids: vec![], // no section overlap
            }],
            phases: vec![super::PhaseContext {
                phase_id: "phase_addr".to_string(),
                summary: "Address phase".to_string(),
                supporting_statement_ids: vec!["stmt_1".to_string()], // overlaps via statement
                supporting_section_ids: vec!["sec_other".to_string()],
            }],
            invariants: Vec::new(),
            assertions: Vec::new(),
            contracts: Vec::new(),
            gates: Vec::new(),
            abstractions: Vec::new(),
            residual_decisions: Vec::new(),
        };
        let actors = super::build_intent_actors(&context);
        let tx = actors.iter().find(|a| a.actor_id == "actor_tx").unwrap();
        assert!(
            tx.responsibilities
                .iter()
                .any(|r| r.contains("Address phase")),
            "actor must participate in phase when supporting statements overlap (even with empty sections)"
        );
    }

    // -----------------------------------------------------------------------
    // Regression tests for new temporal record types (#53)
    // -----------------------------------------------------------------------

    #[test]
    fn transaction_intent_round_trips_through_json() {
        let tx = super::TransactionIntent {
            transaction_id: "txn_test".to_string(),
            transaction_name: "test_transfer".to_string(),
            activation_port: Some("HREADY".to_string()),
            ports: vec![super::TransactionPortRecord {
                port_name: "HADDR".to_string(),
                direction: super::TransactionPortDirection::Output,
                width: Some(32),
            }],
            steps: vec![
                super::TransactionStep::Await {
                    port: "HREADY".to_string(),
                    watchdog: Some(16),
                },
                super::TransactionStep::When {
                    condition: "HTRANS == NONSEQ".to_string(),
                    body: vec![super::TransactionStep::Drive {
                        drive_name: "HADDR".to_string(),
                        actuals: vec!["addr".to_string()],
                    }],
                },
            ],
            source_block_ids: vec!["cb_1".to_string()],
            source_temporal_rule_ids: vec!["tr_1".to_string()],
            supporting_statement_ids: vec!["stmt_1".to_string()],
            automation_confidence: super::AutomationConfidence::Medium,
        };

        let json = serde_json::to_string(&tx).unwrap();
        let round_tripped: super::TransactionIntent = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped.transaction_id, "txn_test");
        assert_eq!(round_tripped.transaction_name, "test_transfer");
        assert_eq!(round_tripped.activation_port, Some("HREADY".to_string()));
        assert_eq!(round_tripped.ports.len(), 1);
        assert_eq!(round_tripped.steps.len(), 2);
    }

    #[test]
    fn transaction_step_all_variants_serialize_with_step_kind_tag() {
        let steps = vec![
            super::TransactionStep::Drive {
                drive_name: "HADDR".to_string(),
                actuals: vec!["val".to_string()],
            },
            super::TransactionStep::When {
                condition: "x == 1".to_string(),
                body: vec![],
            },
            super::TransactionStep::Switch {
                selector: "HTRANS".to_string(),
                branches: vec![super::SwitchBranch {
                    value: "NONSEQ".to_string(),
                    body: vec![],
                }],
            },
            super::TransactionStep::While {
                condition: "count < 4".to_string(),
                body: vec![],
            },
            super::TransactionStep::Until {
                condition: "done".to_string(),
                body: vec![],
            },
            super::TransactionStep::Repeat {
                count: "8".to_string(),
                body: vec![],
            },
            super::TransactionStep::Await {
                port: "READY".to_string(),
                watchdog: Some(16),
            },
            super::TransactionStep::Wait {
                count: "2".to_string(),
            },
            super::TransactionStep::Sample {
                port: "DATA".to_string(),
                as_name: "val".to_string(),
            },
            super::TransactionStep::Do {
                child_transaction: "sub_tx".to_string(),
                bindings: vec![],
            },
            super::TransactionStep::Spawn {
                child_transaction: "sub_tx".to_string(),
                instance: "i0".to_string(),
                bindings: vec![],
            },
            super::TransactionStep::Set {
                target: "state".to_string(),
                expr: "IDLE".to_string(),
            },
            super::TransactionStep::Update {
                target: "counter".to_string(),
                expr: "+1".to_string(),
            },
            super::TransactionStep::ShiftLeft {
                reg: "shift_reg".to_string(),
                bit: "1".to_string(),
            },
            super::TransactionStep::ShiftRight {
                reg: "shift_reg".to_string(),
                bit: "0".to_string(),
                width: Some(8),
            },
            super::TransactionStep::Complete {
                port: "done".to_string(),
            },
            super::TransactionStep::AwaitAll {
                done_port: "all_done".to_string(),
            },
            super::TransactionStep::AwaitAny {
                done_port: "any_done".to_string(),
            },
            super::TransactionStep::Latency { min: 1, max: 3 },
        ];

        let json = serde_json::to_string_pretty(&steps).unwrap();
        assert!(json.contains("\"step_kind\""));
        assert!(json.contains("\"drive\""));
        assert!(json.contains("\"when\""));
        assert!(json.contains("\"switch\""));
        assert!(json.contains("\"while\""));
        assert!(json.contains("\"until\""));
        assert!(json.contains("\"repeat\""));
        assert!(json.contains("\"await\""));
        assert!(json.contains("\"wait\""));
        assert!(json.contains("\"sample\""));
        assert!(json.contains("\"do\""));
        assert!(json.contains("\"spawn\""));
        assert!(json.contains("\"set\""));
        assert!(json.contains("\"update\""));
        assert!(json.contains("\"shift_left\""));
        assert!(json.contains("\"shift_right\""));
        assert!(json.contains("\"complete\""));
        assert!(json.contains("\"await_all\""));
        assert!(json.contains("\"await_any\""));
        assert!(json.contains("\"latency\""));

        let round_tripped: Vec<super::TransactionStep> = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped.len(), 19);
    }

    #[test]
    fn actor_drive_relation_round_trips() {
        let rel = super::ActorDriveRelationRecord {
            relation_id: "adr_1".to_string(),
            driver_actor: "Manager".to_string(),
            signal_name: "HADDR".to_string(),
            consumer_actor: Some("Subordinate".to_string()),
            condition: Some("HTRANS == NONSEQ".to_string()),
            value: Some("addr_val".to_string()),
            source_text: "Manager drives HADDR".to_string(),
            supporting_statement_ids: vec!["stmt_1".to_string()],
            automation_confidence: super::AutomationConfidence::High,
        };

        let json = serde_json::to_string(&rel).unwrap();
        let round_tripped: super::ActorDriveRelationRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped.relation_id, "adr_1");
        assert_eq!(round_tripped.driver_actor, "Manager");
        assert_eq!(round_tripped.signal_name, "HADDR");
        assert_eq!(
            round_tripped.consumer_actor,
            Some("Subordinate".to_string())
        );
    }

    #[test]
    fn temporal_invariant_round_trips() {
        let inv = super::TemporalInvariantRecord {
            invariant_id: "tinv_1".to_string(),
            subject_signal: "HADDR".to_string(),
            invariant_kind: super::TemporalInvariantKind::MustNotChange,
            condition_signal: Some("HREADY".to_string()),
            condition_value: Some("LOW".to_string()),
            target_value: None,
            source_text: "HADDR must not change when HREADY is LOW".to_string(),
            supporting_statement_ids: vec!["stmt_1".to_string()],
            automation_confidence: super::AutomationConfidence::High,
        };

        let json = serde_json::to_string(&inv).unwrap();
        let round_tripped: super::TemporalInvariantRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped.invariant_id, "tinv_1");
        assert_eq!(
            round_tripped.invariant_kind,
            super::TemporalInvariantKind::MustNotChange
        );
        assert_eq!(round_tripped.condition_signal, Some("HREADY".to_string()));
    }

    #[test]
    fn actor_trigger_relation_round_trips() {
        let rel = super::ActorTriggerRelationRecord {
            relation_id: "atr_1".to_string(),
            source_actor: "Requester".to_string(),
            target_actor: "Arbiter".to_string(),
            trigger_port: Some("REQ".to_string()),
            activation_kind: Some(super::ActivationKind::Do),
            source_text: "Arbiter grants on REQ".to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: super::AutomationConfidence::Medium,
        };

        let json = serde_json::to_string(&rel).unwrap();
        let round_tripped: super::ActorTriggerRelationRecord = serde_json::from_str(&json).unwrap();
        assert_eq!(round_tripped.source_actor, "Requester");
        assert_eq!(round_tripped.target_actor, "Arbiter");
        assert_eq!(
            round_tripped.activation_kind,
            Some(super::ActivationKind::Do)
        );
    }

    #[test]
    fn intent_ir_serializes_new_fields() {
        // Build a minimal IntentIr and verify the new fields serialize
        let tempdir = tempdir().unwrap();
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Spec\nSignal VALID is output width 1.\nSignal READY is input width 1.\nSignal DATA is output width 8.\n\nThe transmitter drives VALID and DATA.\nThe receiver samples VALID and drives READY.\nWhen VALID and READY, data is transferred.\n",
        ).unwrap();

        let source_ir = SourceIr::build(&source, &source_artifact_base).unwrap();
        source_ir.write_to_disk().unwrap();
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )
        .unwrap();
        evidence_ir.write_to_disk().unwrap();
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )
        .unwrap();
        semantic_ir.write_to_disk().unwrap();
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )
        .unwrap();

        let json = intent_ir.to_pretty_json().unwrap();

        // New fields must appear in JSON output
        assert!(
            json.contains("\"transactions\""),
            "transactions field missing"
        );
        assert!(
            json.contains("\"actor_drive_relations\""),
            "actor_drive_relations field missing"
        );
        assert!(
            json.contains("\"actor_sample_relations\""),
            "actor_sample_relations field missing"
        );
        assert!(
            json.contains("\"actor_trigger_relations\""),
            "actor_trigger_relations field missing"
        );
        assert!(
            json.contains("\"actor_temporal_dependencies\""),
            "actor_temporal_dependencies field missing"
        );
        assert!(
            json.contains("\"temporal_invariants\""),
            "temporal_invariants field missing"
        );

        // Write to disk then load back and verify new fields are accessible
        intent_ir.write_to_disk().unwrap();
        let reloaded = IntentIr::load_from_path(&intent_ir.artifact_layout.intent_ir_path).unwrap();
        // New fields should exist (may be empty depending on NLP)
        let _ = reloaded.transactions.len();
        let _ = reloaded.actor_drive_relations.len();
        let _ = reloaded.temporal_invariants.len();
    }

    #[test]
    fn pattern_recognition_detects_valid_ready_handshake() {
        // Verify that when INTENT_IR is built from a spec with VALID/READY signals,
        // the digital pattern recognizer adds handshake transactions
        let tempdir = tempdir().unwrap();
        let source = tempdir.path().join("hs.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Handshake Protocol\nSignal VALID is output width 1.\nSignal READY is input width 1.\n\nThe transmitter drives VALID.\nThe receiver drives READY.\nWhen VALID and READY, data is transferred.\n",
        ).unwrap();

        let source_ir = SourceIr::build(&source, &source_artifact_base).unwrap();
        source_ir.write_to_disk().unwrap();
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )
        .unwrap();
        evidence_ir.write_to_disk().unwrap();
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )
        .unwrap();
        semantic_ir.write_to_disk().unwrap();
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )
        .unwrap();

        // Pattern recognition should have added a handshake transaction
        let has_handshake_tx = intent_ir
            .transactions
            .iter()
            .any(|t| t.transaction_name.contains("handshake"));
        assert!(
            has_handshake_tx,
            "expected pattern recognition to add a valid/ready handshake transaction"
        );

        // Should also have a drive relation for VALID
        let has_valid_drive = intent_ir
            .actor_drive_relations
            .iter()
            .any(|r| r.signal_name == "VALID");
        assert!(
            has_valid_drive,
            "expected a drive relation for VALID signal"
        );
    }

    #[test]
    fn temporal_invariant_kind_classification() {
        assert_eq!(
            super::classify_invariant_text("HADDR must not change when HREADY is LOW"),
            super::TemporalInvariantKind::MustNotChange
        );
        assert_eq!(
            super::classify_invariant_text("DATA shall remain stable during address phase"),
            super::TemporalInvariantKind::MustRemainStable
        );
        assert_eq!(
            super::classify_invariant_text("RESET must be asserted for 16 cycles"),
            super::TemporalInvariantKind::MustBeAsserted
        );
        assert_eq!(
            super::classify_invariant_text("HWDATA must match the value on the bus"),
            super::TemporalInvariantKind::MustMatch
        );
        assert_eq!(
            super::classify_invariant_text("BURST length must not exceed 16"),
            super::TemporalInvariantKind::MustNotExceed
        );
        assert_eq!(
            super::classify_invariant_text("WRITE is only valid when READY is HIGH"),
            super::TemporalInvariantKind::OnlyValidWhen
        );
    }

    #[test]
    fn sanitize_id_replaces_special_chars() {
        assert_eq!(super::sanitize_id("AHB Manager"), "ahb_manager");
        assert_eq!(super::sanitize_id("req/ack_protocol"), "req_ack_protocol");
        assert_eq!(super::sanitize_id("READY"), "ready");
    }

    #[test]
    fn count_nested_steps_handles_empty() {
        let steps: Vec<super::TransactionStep> = vec![];
        assert_eq!(super::count_nested_steps(&steps), 0);
    }

    #[test]
    fn count_nested_steps_sums_nested_bodies() {
        let steps = vec![
            super::TransactionStep::When {
                condition: "x".to_string(),
                body: vec![
                    super::TransactionStep::Drive {
                        drive_name: "A".to_string(),
                        actuals: vec![],
                    },
                    super::TransactionStep::Drive {
                        drive_name: "B".to_string(),
                        actuals: vec![],
                    },
                ],
            },
            super::TransactionStep::Drive {
                drive_name: "C".to_string(),
                actuals: vec![],
            },
        ];
        // 2 top-level steps + 2 nested = 4
        assert_eq!(super::count_nested_steps(&steps), 4);
    }

    #[test]
    fn extract_case_value_from_binary_equality() {
        use crate::ir::semantic::{ControlBinaryOperator, ControlExpressionRecord};
        let predicate = ControlExpressionRecord::Binary {
            operator: ControlBinaryOperator::Eq,
            left: Box::new(ControlExpressionRecord::Reference {
                reference: crate::ir::semantic::ControlReferenceRecord {
                    base_name: "HTRANS".to_string(),
                    kind_hint: crate::ir::semantic::ControlReferenceKind::Signal,
                    suffixes: vec![],
                    exposed_public_output: false,
                },
            }),
            right: Box::new(ControlExpressionRecord::Literal {
                literal: "NONSEQ".to_string(),
            }),
        };
        let selector = ControlExpressionRecord::Reference {
            reference: crate::ir::semantic::ControlReferenceRecord {
                base_name: "HTRANS".to_string(),
                kind_hint: crate::ir::semantic::ControlReferenceKind::Signal,
                suffixes: vec![],
                exposed_public_output: false,
            },
        };
        let value = super::extract_case_value(&predicate, &selector);
        assert_eq!(value, "NONSEQ");
    }
}
