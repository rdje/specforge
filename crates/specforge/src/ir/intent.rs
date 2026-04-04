use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::SignalSemanticConflictRecord;
use crate::ir::semantic::{
    ActorPortRecord, ConditionalRuleRecord, ControlBlockRecord, DecisionTreeFragmentRecord,
    ExplicitModuleRecord, ExplicitTopRecord, InitAssignmentRecord, InterfaceRecord,
    InterfaceSignalConflictRecord, RegisterRecord, RegularStateRecord, SemanticIr,
    SignalConnectivityConflictRecord, SignalConnectivityRecord, SignalConstraintRecord,
    StateTransitionRecord, SymbolDefinitionRecord, SystemContractRecord, TemporalConflictRecord,
    TemporalRuleRecord, TimingConstraintRecord,
};
use crate::ir::source::{
    ActorSignalRelation, AutomationConfidence, CandidateInterpretation, ResidualDecisionPacket,
    ValidationReportRecord, document_key,
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
    pub interface_signal_conflicts: Vec<InterfaceSignalConflictRecord>,
    #[serde(default)]
    pub signal_connectivity_conflicts: Vec<SignalConnectivityConflictRecord>,
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
    pub init_assignments: Vec<InitAssignmentRecord>,
    #[serde(default)]
    pub regular_states: Vec<RegularStateRecord>,
    #[serde(default)]
    pub state_transitions: Vec<StateTransitionRecord>,
    #[serde(default)]
    pub decision_tree_fragments: Vec<DecisionTreeFragmentRecord>,
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
    /// Explicit conflicts detected across contradictory temporal value obligations.
    #[serde(default)]
    pub temporal_conflicts: Vec<TemporalConflictRecord>,
    /// Level 2 NLP: signal constraint records carried forward from `SemanticIR`.
    #[serde(default)]
    pub signal_constraints: Vec<SignalConstraintRecord>,
    /// Level 2 NLP: conditional rule records carried forward from `SemanticIR`.
    #[serde(default)]
    pub conditional_rules: Vec<ConditionalRuleRecord>,
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
        let interface_signal_conflicts = semantic_ir.interface_signal_conflicts.clone();
        let signal_connectivity_conflicts = semantic_ir.signal_connectivity_conflicts.clone();
        let signal_semantic_conflicts = semantic_ir.signal_semantic_conflicts.clone();
        let interfaces = semantic_ir.interfaces.clone();
        let system_contract = semantic_ir.system_contract.clone();
        let actors = build_intent_actors(&context);
        let actor_ids = actors.iter().map(|actor| actor.actor_id.clone()).collect();
        let behaviors = build_behaviors(&context, actor_ids);
        let constraints = build_constraints(&context);
        let assumptions = build_assumptions(&context, &actors);
        let init_assignments = semantic_ir.init_assignments.clone();
        let regular_states = semantic_ir.regular_states.clone();
        let state_transitions = semantic_ir.state_transitions.clone();
        let decision_tree_fragments = semantic_ir.decision_tree_fragments.clone();
        let symbol_definitions = semantic_ir.symbol_definitions.clone();
        let control_blocks = semantic_ir.control_blocks.clone();
        let explicit_modules = semantic_ir.explicit_modules.clone();
        let explicit_tops = semantic_ir.explicit_tops.clone();
        let register_records = semantic_ir.register_records.clone();
        let timing_constraints = semantic_ir.timing_constraints.clone();
        let temporal_rules = semantic_ir.temporal_rules.clone();
        let temporal_conflicts = semantic_ir.temporal_conflicts.clone();
        let signal_constraints = semantic_ir.signal_constraints.clone();
        let conditional_rules = semantic_ir.conditional_rules.clone();
        let residual_decisions =
            build_residual_decisions(&context, &actors, &behaviors, &constraints);
        let intent_identity = build_intent_identity(
            &document_identity,
            &actors,
            &interfaces,
            system_contract.as_ref(),
            &behaviors,
            &constraints,
            &init_assignments,
            &regular_states,
            &state_transitions,
            &decision_tree_fragments,
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
            interface_signal_conflicts,
            signal_connectivity_conflicts,
            signal_semantic_conflicts,
            interfaces,
            system_contract,
            behaviors,
            constraints,
            assumptions,
            init_assignments,
            regular_states,
            state_transitions,
            decision_tree_fragments,
            symbol_definitions,
            control_blocks,
            explicit_modules,
            explicit_tops,
            register_records,
            timing_constraints,
            temporal_rules,
            temporal_conflicts,
            signal_constraints,
            conditional_rules,
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

fn build_intent_identity(
    document_identity: &IntentDocumentIdentity,
    actors: &[IntentActor],
    interfaces: &[InterfaceRecord],
    system_contract: Option<&SystemContractRecord>,
    behaviors: &[BehaviorIntent],
    constraints: &[IntentConstraint],
    init_assignments: &[InitAssignmentRecord],
    regular_states: &[RegularStateRecord],
    state_transitions: &[StateTransitionRecord],
    decision_tree_fragments: &[DecisionTreeFragmentRecord],
    symbol_definitions: &[SymbolDefinitionRecord],
    control_blocks: &[ControlBlockRecord],
    explicit_modules: &[ExplicitModuleRecord],
    explicit_tops: &[ExplicitTopRecord],
) -> IntentIdentity {
    IntentIdentity {
        intent_id: format!("intent_{}", document_identity.document_key),
        summary: format!(
            "backend-neutral intent for {} covering {} actors, {} interfaces, {} behaviors, {} constraints, {} init assignments, {} regular states, {} state transitions, {} control fragments, {} symbol definitions, {} structured control blocks, {} explicit modules, {} explicit tops, and {} explicit system contract",
            document_identity.display_name,
            actors.len(),
            interfaces.len(),
            behaviors.len(),
            constraints.len(),
            init_assignments.len(),
            regular_states.len(),
            state_transitions.len(),
            decision_tree_fragments.len(),
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

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
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
        AutomationConfidence, SourceIr, StructuredTableCellRecord, StructuredTableRecord,
        TableKind, VisualAsset, VisualAssetKind,
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
                .any(|actor| actor.actor_id == "actor_transmitter")
        );
        assert!(intent_ir.behaviors.iter().any(|behavior| {
            behavior
                .statement
                .contains("The transmitter must assert VALID")
        }));
        assert!(intent_ir.constraints.iter().any(|constraint| {
            constraint
                .statement
                .contains("VALID must remain asserted until READY is observed.")
        }));
        assert!(intent_ir.assumptions.iter().any(|assumption| {
            assumption
                .statement
                .contains("backend-neutral transport abstraction")
        }));
        assert!(intent_ir.residual_decisions.is_empty());

        Ok(())
    }

    #[test]
    fn preserves_semantic_residual_decisions_in_intent_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("control.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        let asset_path = tempdir.path().join("assets").join("figure-0001.png");

        fs::create_dir_all(asset_path.parent().expect("asset parent should exist"))?;
        fs::write(&asset_path, b"png")?;
        fs::write(
            &source,
            "# Control Path\nFigure 1: Controller block diagram.\n\n![Image](assets/figure-0001.png)\n\nThe controller behavior is shown in Figure 1.\n",
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "figure_0001".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: Some(asset_path),
            caption_text: Some("Figure 1: Controller block diagram.".to_string()),
            caption_source_path: None,
            source_ref: Some("#/pictures/0".to_string()),
            placeholder_text: None,
            note: None,
            diagram_kind: crate::ir::source::DiagramKind::BlockDiagram,
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
        intent_ir.write_to_disk()?;

        assert!(
            intent_ir
                .residual_decisions
                .iter()
                .any(|packet| { packet.packet_id == "semantic_ambiguous_visual_grounding" })
        );
        assert!(intent_ir.assumptions.iter().any(|assumption| {
            assumption.assumption_id == "assumption_ambiguous_visual_grounding"
        }));
        assert!(
            intent_ir
                .artifact_layout
                .intent_ir_path
                .ends_with("generated/intent_ir/control/intent_ir.json")
        );

        Ok(())
    }

    #[test]
    fn carries_typed_interface_and_control_fragments_into_intent_ir() -> Result<()> {
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
        assert_eq!(intent_ir.decision_tree_fragments.len(), 2);
        assert!(
            intent_ir
                .decision_tree_fragments
                .iter()
                .any(|fragment| fragment.block_name == "route_data")
        );
        assert!(
            intent_ir
                .intent_identity
                .summary
                .contains("control fragments")
        );

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
    fn carries_system_contract_and_init_assignments_into_intent_ir() -> Result<()> {
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
        assert_eq!(intent_ir.init_assignments.len(), 1);
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
        assert_eq!(intent_ir.init_assignments.len(), 1);

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
        assert!(intent_ir.explicit_modules.iter().any(|module| {
            module.module_name == "producer_core"
                && module
                    .decision_tree_fragments
                    .iter()
                    .any(|fragment| fragment.block_name == "produce")
        }));
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
            })
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
            })
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

        Ok(())
    }
}
