use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::{EvidenceIr, StatementClass, VisualEvidenceRole, VisualObservationKind};
use crate::ir::source::{
    AutomationConfidence, CandidateInterpretation, ResidualDecisionPacket, WidthHint, document_key,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub evidence_ir_path: PathBuf,
    pub artifact_layout: SemanticArtifactLayout,
    pub document_identity: SemanticDocumentIdentity,
    pub actors: Vec<ActorRecord>,
    pub interfaces: Vec<InterfaceRecord>,
    pub phases: Vec<PhaseRecord>,
    pub invariants: Vec<InvariantRecord>,
    pub contracts: Vec<ContractRecord>,
    pub gates: Vec<GateRecord>,
    pub assertions: Vec<AssertionRecord>,
    pub abstractions: Vec<AbstractionRecord>,
    pub decomposition_candidates: Vec<DecompositionCandidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_contract: Option<SystemContractRecord>,
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
    /// Register map records synthesized from `register_map` tables in `SourceIR`.
    #[serde(default)]
    pub register_records: Vec<RegisterRecord>,
    /// Timing constraint records synthesized from `timing_parameter` tables in `SourceIR`.
    #[serde(default)]
    pub timing_constraints: Vec<TimingConstraintRecord>,
    /// Level 2 NLP: structured signal constraint records from `SignalValueConstraint` sentences.
    #[serde(default)]
    pub signal_constraints: Vec<SignalConstraintRecord>,
    /// Level 2 NLP: structured conditional rule records from `ConditionalRule` sentences.
    #[serde(default)]
    pub conditional_rules: Vec<ConditionalRuleRecord>,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
}

impl SemanticIr {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(AppError::MissingPath(path.to_path_buf()));
        }

        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn build(evidence_ir_path: &Path, artifact_base_root: &Path) -> Result<Self> {
        let evidence_ir_path = canonicalize_existing_path(evidence_ir_path)?;
        let evidence_ir = EvidenceIr::load_from_path(&evidence_ir_path)?;

        if !matches!(evidence_ir.stage, IrStage::EvidenceIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} must be an EvidenceIR document before building SemanticIR",
                evidence_ir_path.display()
            )));
        }

        let artifact_root = artifact_base_root.join(&evidence_ir.document_identity.document_key);
        let semantic_ir_path = artifact_root.join("semantic_ir.json");
        let artifact_layout = SemanticArtifactLayout {
            artifact_root,
            semantic_ir_path,
        };
        let document_identity = SemanticDocumentIdentity {
            document_key: evidence_ir.document_identity.document_key.clone(),
            display_name: evidence_ir.document_identity.display_name.clone(),
        };

        let context = SemanticContext::from_evidence_ir(&evidence_ir);
        let interfaces = build_interfaces(&context);
        let actor_build = build_actors(&context, &interfaces);
        let phases = build_phases(&context);
        let interface_ids_by_signal = interface_ids_by_signal(&interfaces);
        let invariants = build_invariants(&context, &interface_ids_by_signal);
        let contracts = build_contracts(&context, &actor_build.actor_id_by_term);
        let gates = build_gates(&context, &interface_ids_by_signal);
        let assertions = build_assertions(&context);
        let abstractions = build_abstractions(&context);
        let decomposition_candidates = build_decomposition_candidates(&context);
        let system_contract = build_system_contract(&context);
        let init_assignments = build_init_assignments(&context);
        let regular_states = build_regular_states(&context);
        let state_transitions = build_state_transitions(&context);
        let decision_tree_fragments = build_decision_tree_fragments(&context);
        let symbol_definitions = build_symbol_definitions(&context);
        let control_blocks = build_control_blocks(
            &context,
            regular_states.as_slice(),
            symbol_definitions.as_slice(),
        );
        let explicit_modules = build_explicit_modules(&context);
        let explicit_tops = build_explicit_tops(&context);
        let residual_decisions =
            build_residual_decisions(&context, &interfaces, actor_build.explicit_actor_count);
        // Carry structured table records forward from EvidenceIR.
        let register_records = evidence_ir.register_records.clone();

        // Layer D: Declared-signal gating.
        // Build the set of authoritative signal names from High-confidence interface records
        // (those that came from formal `Signal X is input/output` declarations synthesized
        // from signal description tables).  NLP records for signals outside this set are
        // heuristic noise and are suppressed so they do not pollute downstream scoring.
        // If no explicit declarations exist (e.g. pure prose specs with no tables), the set
        // is empty and gating is disabled so we never drop records unnecessarily.
        // Declared signals = High confidence (from structured tables) OR
        // Medium confidence (from Tier 2 KG actor-signal relation extraction).
        // Low confidence = heuristic co-mention noise; still excluded.
        let declared_signal_names: std::collections::HashSet<String> = interfaces
            .iter()
            .flat_map(|iface| &iface.signal_records)
            .filter(|r| !matches!(r.automation_confidence, AutomationConfidence::Low))
            .map(|r| r.signal_name.clone())
            .collect();

        let signal_constraints = if declared_signal_names.is_empty() {
            evidence_ir.signal_constraints.clone()
        } else {
            evidence_ir
                .signal_constraints
                .iter()
                .filter(|r| declared_signal_names.contains(&r.subject_signal))
                .cloned()
                .collect()
        };
        let conditional_rules = if declared_signal_names.is_empty() {
            evidence_ir.conditional_rules.clone()
        } else {
            evidence_ir
                .conditional_rules
                .iter()
                .filter(|r| {
                    // Keep rules where the consequent signal is declared, or rules with
                    // no specific consequent signal (system-level behavioral rules).
                    r.consequent_signal
                        .as_ref()
                        .map(|s| declared_signal_names.contains(s))
                        .unwrap_or(true)
                })
                .cloned()
                .collect()
        };

        // Merge timing constraints: table-synthesized + VLM diagram observations.
        let mut timing_constraints = evidence_ir.timing_constraints.clone();
        let (vlm_timing, vlm_states, vlm_transitions) =
            extract_records_from_vlm_observations(&evidence_ir);
        timing_constraints.extend(vlm_timing);

        // Merge state/transition records: formal syntax + VLM diagram observations.
        // VLM-sourced records are appended so they don’t replace existing formal records.
        let mut regular_states_with_vlm = regular_states.clone();
        let mut state_transitions_with_vlm = state_transitions.clone();
        for vlm_state in vlm_states {
            // Only add if not already present by name.
            if !regular_states_with_vlm
                .iter()
                .any(|s| s.state_name == vlm_state.state_name)
            {
                regular_states_with_vlm.push(vlm_state);
            }
        }
        for vlm_transition in vlm_transitions {
            if !state_transitions_with_vlm.iter().any(|t| {
                t.source_state == vlm_transition.source_state
                    && t.target_state == vlm_transition.target_state
            }) {
                state_transitions_with_vlm.push(vlm_transition);
            }
        }

        Ok(Self {
            schema_version: 1,
            stage: IrStage::SemanticIr,
            evidence_ir_path,
            artifact_layout,
            document_identity,
            actors: actor_build.actors,
            interfaces,
            phases,
            invariants,
            contracts,
            gates,
            assertions,
            abstractions,
            decomposition_candidates,
            system_contract,
            init_assignments,
            regular_states: regular_states_with_vlm,
            state_transitions: state_transitions_with_vlm,
            decision_tree_fragments,
            symbol_definitions,
            control_blocks,
            explicit_modules,
            explicit_tops,
            register_records,
            timing_constraints,
            signal_constraints,
            conditional_rules,
            residual_decisions,
        })
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn write_to_disk(&self) -> Result<()> {
        fs::create_dir_all(&self.artifact_layout.artifact_root)?;
        fs::write(
            &self.artifact_layout.semantic_ir_path,
            self.to_pretty_json()?,
        )?;
        Ok(())
    }
}

fn min_automation_confidence(
    left: AutomationConfidence,
    right: AutomationConfidence,
) -> AutomationConfidence {
    if automation_confidence_rank(left) <= automation_confidence_rank(right) {
        left
    } else {
        right
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticArtifactLayout {
    pub artifact_root: PathBuf,
    pub semantic_ir_path: PathBuf,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticDocumentIdentity {
    pub document_key: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorRecord {
    pub actor_id: String,
    pub role_summary: String,
    pub supporting_statement_ids: Vec<String>,
    pub supporting_section_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterfaceRecord {
    pub interface_id: String,
    pub signals: Vec<String>,
    #[serde(default)]
    pub signal_records: Vec<InterfaceSignalRecord>,
    pub supporting_statement_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceSignalDirection {
    Input,
    Output,
    Internal,
}

impl InterfaceSignalDirection {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Input => "input",
            Self::Output => "output",
            Self::Internal => "internal",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterfaceSignalRecord {
    pub signal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_hint: Option<InterfaceSignalDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_hint: Option<WidthHint>,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PhaseRecord {
    pub phase_id: String,
    pub summary: String,
    pub supporting_statement_ids: Vec<String>,
    pub supporting_section_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InvariantRecord {
    pub invariant_id: String,
    pub statement: String,
    pub supporting_statement_ids: Vec<String>,
    pub related_interface_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContractRecord {
    pub contract_id: String,
    pub statement: String,
    pub supporting_statement_ids: Vec<String>,
    pub actor_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GateRecord {
    pub gate_id: String,
    pub condition: String,
    pub supporting_statement_ids: Vec<String>,
    pub related_interface_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AssertionRecord {
    pub assertion_id: String,
    pub statement: String,
    pub supporting_statement_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AbstractionRecord {
    pub abstraction_id: String,
    pub description: String,
    pub supporting_statement_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecompositionCandidate {
    pub candidate_id: String,
    pub summary: String,
    pub supporting_statement_ids: Vec<String>,
    pub supporting_section_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemContractRecord {
    pub clock_signal: String,
    pub reset_signal: String,
    pub reset_kind: SystemResetKind,
    pub reset_polarity: SystemResetPolarity,
    pub assertion_timing: SystemResetTimingRelation,
    pub release_timing: SystemResetTimingRelation,
    pub target_kind: SystemResetTargetKind,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SystemResetKind {
    Synchronous,
    Asynchronous,
}

impl SystemResetKind {
    pub fn assertion_timing(self) -> SystemResetTimingRelation {
        match self {
            Self::Synchronous => SystemResetTimingRelation::SynchronousToClock,
            Self::Asynchronous => SystemResetTimingRelation::AsynchronousToClock,
        }
    }

    pub fn release_timing(self) -> SystemResetTimingRelation {
        SystemResetTimingRelation::SynchronousToClock
    }

    pub fn target_kind(self) -> SystemResetTargetKind {
        match self {
            Self::Synchronous => SystemResetTargetKind::DataInputPath,
            Self::Asynchronous => SystemResetTargetKind::DedicatedResetPin,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SystemResetPolarity {
    ActiveHigh,
    ActiveLow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SystemResetTimingRelation {
    SynchronousToClock,
    AsynchronousToClock,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SystemResetTargetKind {
    DataInputPath,
    DedicatedResetPin,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InitAssignmentRecord {
    pub target_signal: String,
    pub value: DecisionTreeValueRecord,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegularStateRecord {
    pub state_id: String,
    pub state_name: String,
    pub is_initial: bool,
    pub declaration_order: u32,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StateTransitionRecord {
    pub transition_id: String,
    pub source_state: String,
    pub target_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<DecisionTreeGuardRecord>,
    pub declaration_order: u32,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DecisionTreeFragmentRecord {
    pub fragment_id: String,
    pub block_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<DecisionTreeGuardRecord>,
    pub actions: Vec<DecisionTreeActionRecord>,
    pub referenced_signal_names: Vec<String>,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DecisionTreeGuardRecord {
    SignalIsHigh {
        signal_name: String,
    },
    Comparison {
        left_signal: String,
        operator: DecisionTreeComparisonOperator,
        right: DecisionTreeValueRecord,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionTreeComparisonOperator {
    Eq,
    NotEq,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DecisionTreeActionRecord {
    Assign {
        target_signal: String,
        assignment_kind: DecisionTreeAssignmentKind,
        value: DecisionTreeValueRecord,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum DecisionTreeAssignmentKind {
    Combinational,
    Sequential,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum DecisionTreeValueRecord {
    SignalRef { signal_name: String },
    Literal { literal: String },
}
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SymbolDefinitionKind {
    Constant,
    Define,
    Param,
    Enum,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolEnumMemberRecord {
    pub member_name: String,
    pub value: ControlExpressionRecord,
    pub declaration_order: u32,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SymbolDefinitionRecord {
    pub symbol_id: String,
    pub symbol_name: String,
    pub kind: SymbolDefinitionKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ControlExpressionRecord>,
    #[serde(default)]
    pub members: Vec<SymbolEnumMemberRecord>,
    pub declaration_order: u32,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ControlReferenceKind {
    Unknown,
    Signal,
    Symbol,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ControlReferenceSuffix {
    Member { member_name: String },
    BitIndex { index: u32 },
    Slice { msb: u32, lsb: u32 },
    WidthCast { width: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ControlReferenceRecord {
    pub base_name: String,
    pub kind_hint: ControlReferenceKind,
    #[serde(default)]
    pub suffixes: Vec<ControlReferenceSuffix>,
    pub exposed_public_output: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ControlUnaryOperator {
    Not,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ControlBinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitOr,
    BitXor,
    Eq,
    NotEq,
    Lt,
    Le,
    Gt,
    Ge,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ControlExpressionRecord {
    Reference {
        reference: ControlReferenceRecord,
    },
    Literal {
        literal: String,
    },
    Unary {
        operator: ControlUnaryOperator,
        operand: Box<ControlExpressionRecord>,
    },
    Binary {
        operator: ControlBinaryOperator,
        left: Box<ControlExpressionRecord>,
        right: Box<ControlExpressionRecord>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ControlBlockRole {
    StateBody,
    ResetSynchronous,
    ResetAsynchronous,
    StandaloneDecisionTree,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ControlAssignmentTargetRecord {
    pub signal_name: String,
    pub exposed_public_output: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ControlDualOutputKind {
    NextSignal,
    RegisteredSignal,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ControlCompoundUpdateOperation {
    Increment,
    Decrement,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ControlActionRecord {
    Assign {
        target: ControlAssignmentTargetRecord,
        assignment_kind: DecisionTreeAssignmentKind,
        #[serde(skip_serializing_if = "Option::is_none")]
        dual_output: Option<ControlDualOutputKind>,
        value: ControlExpressionRecord,
    },
    Transition {
        target_state: String,
    },
    DelayedPulse {
        target: ControlAssignmentTargetRecord,
        delay: u32,
        value: ControlExpressionRecord,
    },
    CompoundUpdate {
        target: ControlAssignmentTargetRecord,
        operation: ControlCompoundUpdateOperation,
        #[serde(skip_serializing_if = "Option::is_none")]
        amount: Option<ControlExpressionRecord>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ControlBranchRecord {
    pub branch_id: String,
    pub declaration_order: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate: Option<ControlExpressionRecord>,
    #[serde(default)]
    pub actions: Vec<ControlActionRecord>,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ControlBlockRecord {
    pub block_id: String,
    pub block_name: String,
    pub role: ControlBlockRole,
    pub declaration_order: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selector: Option<ControlExpressionRecord>,
    #[serde(default)]
    pub branches: Vec<ControlBranchRecord>,
    pub referenced_signal_names: Vec<String>,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitModuleRecord {
    pub module_id: String,
    pub module_name: String,
    pub declaration_order: u32,
    #[serde(default)]
    pub interfaces: Vec<InterfaceRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_contract: Option<SystemContractRecord>,
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
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitTopRecord {
    pub top_id: String,
    pub top_name: String,
    pub declaration_order: u32,
    #[serde(default)]
    pub ports: Vec<ExplicitTopPortRecord>,
    #[serde(default)]
    pub children: Vec<ExplicitTopChildRecord>,
    #[serde(default)]
    pub links: Vec<ExplicitTopLinkRecord>,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitTopPortRecord {
    pub port_name: String,
    pub direction_hint: InterfaceSignalDirection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_hint: Option<WidthHint>,
    pub declaration_order: u32,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitTopChildRecord {
    pub instance_name: String,
    pub source_module_name: String,
    pub declaration_order: u32,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitTopLinkRecord {
    pub link_id: String,
    pub source: ExplicitTopLinkEndpoint,
    pub target: ExplicitTopLinkEndpoint,
    pub declaration_order: u32,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplicitTopLinkEndpoint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_name: Option<String>,
    pub signal_name: String,
}

// Structured extraction types are defined in `source.rs` to avoid circular imports.
// Re-exported here so IntentIR and adapters can import them from `semantic`.
pub use crate::ir::source::{
    ConditionalRuleRecord, RegisterFieldRecord, RegisterRecord, SignalConstraintKind,
    SignalConstraintRecord, TimingConstraintRecord,
};

#[derive(Debug, Clone)]
struct SemanticContext {
    statements: Vec<StatementContext>,
    section_anchors: Vec<SemanticSectionContext>,
    visual_roles_by_id: HashMap<String, VisualEvidenceRole>,
}

impl SemanticContext {
    fn from_evidence_ir(evidence_ir: &EvidenceIr) -> Self {
        let spans_by_id: HashMap<String, (Option<u32>, Option<u32>)> = evidence_ir
            .evidence_spans
            .iter()
            .map(|span| (span.span_id.clone(), (span.line_start, span.line_end)))
            .collect();

        // Build set of boilerplate section IDs to exclude legal/administrative content from
        // semantic extraction. Real chip specs typically open with license text, proprietary
        // notices, change history, and revision information that pollutes actor/interface/invariant
        // extraction if left in the statement stream.
        let boilerplate_section_ids: BTreeSet<String> = evidence_ir
            .section_anchors
            .iter()
            .filter(|anchor| is_boilerplate_section_title(&anchor.title))
            .map(|anchor| anchor.section_id.clone())
            .collect();

        let mut section_statement_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let statements = evidence_ir
            .extracted_statements
            .iter()
            .filter_map(|statement| {
                let section_ids = section_ids_for_statement(
                    statement.evidence_span_ids.as_slice(),
                    &spans_by_id,
                    &evidence_ir.section_anchors,
                );

                // Skip statements that belong exclusively to boilerplate sections.
                // A statement with no section membership is kept (it may be in the document
                // preamble before the first heading and should still be evaluated).
                if !section_ids.is_empty()
                    && section_ids
                        .iter()
                        .all(|id| boilerplate_section_ids.contains(id))
                {
                    return None;
                }

                for section_id in &section_ids {
                    section_statement_ids
                        .entry(section_id.clone())
                        .or_default()
                        .push(statement.statement_id.clone());
                }

                Some(StatementContext {
                    statement_id: statement.statement_id.clone(),
                    class: statement.class,
                    text: statement.text.clone(),
                    related_visual_evidence_ids: statement.related_visual_evidence_ids.clone(),
                    section_ids,
                    signals: extract_signal_tokens(&statement.text),
                })
            })
            .collect();

        let section_anchors = evidence_ir
            .section_anchors
            .iter()
            .map(|anchor| SemanticSectionContext {
                section_id: anchor.section_id.clone(),
                title: anchor.title.clone(),
                supporting_statement_ids: section_statement_ids
                    .get(&anchor.section_id)
                    .cloned()
                    .unwrap_or_default(),
            })
            .collect();

        let visual_roles_by_id = evidence_ir
            .visual_evidence
            .iter()
            .map(|visual| (visual.evidence_id.clone(), visual.role))
            .collect();

        Self {
            statements,
            section_anchors,
            visual_roles_by_id,
        }
    }
}

#[derive(Debug, Clone)]
struct StatementContext {
    statement_id: String,
    class: StatementClass,
    text: String,
    related_visual_evidence_ids: Vec<String>,
    section_ids: Vec<String>,
    signals: Vec<String>,
}

impl StatementContext {
    fn with_rewritten_text(&self, text: String) -> Self {
        Self {
            statement_id: self.statement_id.clone(),
            class: self.class,
            text: text.clone(),
            related_visual_evidence_ids: self.related_visual_evidence_ids.clone(),
            section_ids: self.section_ids.clone(),
            signals: extract_signal_tokens(&text),
        }
    }
}

#[derive(Debug, Clone)]
struct SemanticSectionContext {
    section_id: String,
    title: String,
    supporting_statement_ids: Vec<String>,
}

#[derive(Debug, Clone)]
struct ActorBuildResult {
    actors: Vec<ActorRecord>,
    actor_id_by_term: HashMap<String, String>,
    explicit_actor_count: usize,
}

#[derive(Debug, Clone)]
struct ActorAccumulator {
    role_summary: String,
    supporting_statement_ids: BTreeSet<String>,
    supporting_section_ids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct InterfaceAccumulator {
    signals: BTreeSet<String>,
    signal_records: BTreeMap<String, InterfaceSignalAccumulator>,
    supporting_statement_ids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct InterfaceSignalAccumulator {
    direction_hint: Option<InterfaceSignalDirection>,
    width_hint: Option<WidthHint>,
    supporting_statement_ids: BTreeSet<String>,
    automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone)]
struct ScalarSymbolAccumulator {
    kind: SymbolDefinitionKind,
    symbol_name: String,
    value: ControlExpressionRecord,
    declaration_order: u32,
    supporting_statement_ids: BTreeSet<String>,
    conflicting_value: bool,
}

#[derive(Debug, Clone)]
struct EnumMemberAccumulator {
    member_name: String,
    value: ControlExpressionRecord,
    declaration_order: u32,
    supporting_statement_ids: BTreeSet<String>,
    conflicting_value: bool,
}

#[derive(Debug, Clone)]
struct EnumSymbolAccumulator {
    enum_name: String,
    declaration_order: u32,
    members: BTreeMap<String, EnumMemberAccumulator>,
    supporting_statement_ids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct ControlBranchAccumulator {
    predicate: Option<ControlExpressionRecord>,
    declaration_order: u32,
    actions: Vec<ControlActionRecord>,
    referenced_signal_names: BTreeSet<String>,
    supporting_statement_ids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct ControlBlockAccumulator {
    block_name: String,
    role: ControlBlockRole,
    declaration_order: u32,
    selector: Option<ControlExpressionRecord>,
    branches: Vec<ControlBranchAccumulator>,
    referenced_signal_names: BTreeSet<String>,
    supporting_statement_ids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct InitAssignmentAccumulator {
    value: DecisionTreeValueRecord,
    supporting_statement_ids: BTreeSet<String>,
    conflicting_value: bool,
}

#[derive(Debug, Clone)]
struct ParsedInterfaceSignalDeclaration {
    signal_name: String,
    direction_hint: InterfaceSignalDirection,
    width_hint: Option<WidthHint>,
}

#[derive(Debug, Clone)]
struct ParsedSystemResetDeclaration {
    signal_name: String,
    reset_kind: SystemResetKind,
    reset_polarity: SystemResetPolarity,
    assertion_timing: SystemResetTimingRelation,
    release_timing: SystemResetTimingRelation,
    target_kind: SystemResetTargetKind,
    automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone)]
struct ParsedModuleScopedStatement {
    module_name: String,
    scoped_text: Option<String>,
}

#[derive(Debug, Clone)]
struct ParsedTopScopedStatement {
    top_name: String,
    scoped_text: Option<String>,
}

#[derive(Debug, Clone)]
struct ParsedInitAssignment {
    target_signal: String,
    value: DecisionTreeValueRecord,
}

#[derive(Debug, Clone)]
enum ParsedSymbolDefinition {
    Scalar {
        kind: SymbolDefinitionKind,
        symbol_name: String,
        value: ControlExpressionRecord,
    },
    EnumMember {
        enum_name: String,
        member_name: String,
        value: ControlExpressionRecord,
    },
}

#[derive(Debug, Clone)]
struct ParsedExplicitTopChild {
    instance_name: String,
    source_module_name: String,
}

#[derive(Debug, Clone)]
struct ParsedExplicitTopLink {
    source: ExplicitTopLinkEndpoint,
    target: ExplicitTopLinkEndpoint,
}

#[derive(Debug, Clone)]
struct ParsedRegularStateDeclaration {
    state_name: String,
    is_initial: bool,
}

#[derive(Debug, Clone)]
struct ParsedStateTransition {
    source_state: String,
    target_state: String,
    guard: Option<DecisionTreeGuardRecord>,
}

#[derive(Debug, Clone)]
struct ParsedDecisionTreeFragment {
    block_name: String,
    guard: Option<DecisionTreeGuardRecord>,
    action: DecisionTreeActionRecord,
    referenced_signal_names: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct ParsedControlClause {
    block_name: String,
    role: ControlBlockRole,
    selector: Option<ControlExpressionRecord>,
    predicate: Option<ControlExpressionRecord>,
    actions: Vec<ControlActionRecord>,
    referenced_signal_names: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct DecisionTreeFragmentAccumulator {
    block_name: String,
    guard: Option<DecisionTreeGuardRecord>,
    actions: Vec<DecisionTreeActionRecord>,
    referenced_signal_names: BTreeSet<String>,
    supporting_statement_ids: BTreeSet<String>,
    automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone)]
struct ExplicitModuleAccumulator {
    module_name: String,
    declaration_order: u32,
    statements: Vec<StatementContext>,
    supporting_statement_ids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct ExplicitTopAccumulator {
    top_name: String,
    declaration_order: u32,
    ports: Vec<ExplicitTopPortRecord>,
    children: Vec<ExplicitTopChildRecord>,
    links: Vec<ExplicitTopLinkRecord>,
    supporting_statement_ids: BTreeSet<String>,
}

fn build_interfaces(context: &SemanticContext) -> Vec<InterfaceRecord> {
    let mut accumulators: BTreeMap<String, InterfaceAccumulator> = BTreeMap::new();
    let empty_regular_state_names = BTreeSet::<String>::new();
    let empty_known_signal_names = BTreeSet::<String>::new();
    let empty_known_symbol_names = BTreeSet::<String>::new();

    for statement in &context.statements {
        if let Some(signal_declaration) = parse_explicit_signal_declaration(&statement.text) {
            // Signal declarations arrive from EvidenceIR via two paths:
            //   1. Formal `Signal X is input/output width N.` in source text
            //   2. Synthesized by EvidenceIR from structured table cell grids in SourceIR
            // Both flow through `parse_explicit_signal_declaration` here.
            let key = explicit_interface_key(statement.section_ids.as_slice());
            let entry = accumulators
                .entry(key)
                .or_insert_with(|| InterfaceAccumulator {
                    signals: BTreeSet::new(),
                    signal_records: BTreeMap::new(),
                    supporting_statement_ids: BTreeSet::new(),
                });
            register_interface_signal_record(
                entry,
                &signal_declaration.signal_name,
                Some(signal_declaration.direction_hint),
                signal_declaration.width_hint,
                &statement.statement_id,
                AutomationConfidence::High,
            );
            continue;
        }

        if parse_explicit_decision_tree_fragment(&statement.text).is_some()
            || parse_explicit_symbol_definition(
                &statement.text,
                &empty_known_signal_names,
                &empty_known_symbol_names,
            )
            .is_some()
            || parse_explicit_control_clause(
                &statement.text,
                &empty_regular_state_names,
                &empty_known_signal_names,
                &empty_known_symbol_names,
            )
            .is_some()
            || parse_explicit_system_clock(&statement.text).is_some()
            || parse_explicit_system_reset(&statement.text).is_some()
            || parse_explicit_init_assignment(&statement.text).is_some()
            || parse_explicit_regular_state_declaration(&statement.text).is_some()
            || parse_explicit_state_transition(&statement.text).is_some()
            || parse_module_scoped_statement(&statement.text).is_some()
            || parse_top_scoped_statement(&statement.text).is_some()
        {
            continue;
        }
        if !should_emit_interface_candidate(statement.signals.as_slice()) {
            continue;
        }

        let key = statement.signals.join("__");
        let entry = accumulators
            .entry(key)
            .or_insert_with(|| InterfaceAccumulator {
                signals: statement.signals.iter().cloned().collect(),
                signal_records: BTreeMap::new(),
                supporting_statement_ids: BTreeSet::new(),
            });
        for signal_name in &statement.signals {
            register_interface_signal_record(
                entry,
                signal_name,
                None,
                None,
                &statement.statement_id,
                AutomationConfidence::Low,
            );
        }
    }

    accumulators
        .into_iter()
        .filter(|(key, entry)| {
            // Always keep explicitly declared interfaces (from formal `Signal X is input/output`
            // declarations). These are identified by their explicit key prefix.
            if key.starts_with("explicit_interface__") || key == "explicit_document_interface" {
                return true;
            }
            // For heuristic interfaces built from co-mentioned signal tokens: large signal sets
            // (>8 signals) are typical of legal/boilerplate text where many unrelated words are
            // treated as signal tokens. Require them to appear in at least 2 separate statements
            // to confirm they represent a real recurring interface boundary rather than noise.
            // Small signal sets (real hardware interfaces have 2–8 signals) are always kept.
            if entry.signals.len() > 8 {
                return entry.supporting_statement_ids.len() >= 2;
            }
            true
        })
        .map(|(key, entry)| {
            let signals: Vec<String> = entry.signals.into_iter().collect();
            InterfaceRecord {
                interface_id: format!("interface_{}", document_key(&key)),
                signals,
                signal_records: entry
                    .signal_records
                    .into_iter()
                    .map(|(signal_name, signal)| InterfaceSignalRecord {
                        signal_name,
                        direction_hint: signal.direction_hint,
                        width_hint: signal.width_hint,
                        supporting_statement_ids: signal
                            .supporting_statement_ids
                            .into_iter()
                            .collect(),
                        automation_confidence: signal.automation_confidence,
                    })
                    .collect(),
                supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
            }
        })
        .collect()
}

fn build_system_contract(context: &SemanticContext) -> Option<SystemContractRecord> {
    let mut clock_signal = None::<String>;
    let mut reset_signal = None::<String>;
    let mut reset_kind = None::<SystemResetKind>;
    let mut reset_polarity = None::<SystemResetPolarity>;
    let mut assertion_timing = None::<SystemResetTimingRelation>;
    let mut release_timing = None::<SystemResetTimingRelation>;
    let mut target_kind = None::<SystemResetTargetKind>;
    let mut supporting_statement_ids = BTreeSet::new();
    let mut automation_confidence = AutomationConfidence::High;
    let mut conflicting = false;

    for statement in &context.statements {
        if let Some(parsed_clock) = parse_explicit_system_clock(&statement.text) {
            if !merge_named_hint(&mut clock_signal, &parsed_clock) {
                conflicting = true;
            }
            supporting_statement_ids.insert(statement.statement_id.clone());
        }

        if let Some(parsed_reset) = parse_explicit_system_reset(&statement.text) {
            if !merge_named_hint(&mut reset_signal, &parsed_reset.signal_name) {
                conflicting = true;
            }
            if !merge_copy_hint(&mut reset_kind, parsed_reset.reset_kind) {
                conflicting = true;
            }
            if !merge_copy_hint(&mut reset_polarity, parsed_reset.reset_polarity) {
                conflicting = true;
            }
            if !merge_copy_hint(&mut assertion_timing, parsed_reset.assertion_timing) {
                conflicting = true;
            }
            if !merge_copy_hint(&mut release_timing, parsed_reset.release_timing) {
                conflicting = true;
            }
            if !merge_copy_hint(&mut target_kind, parsed_reset.target_kind) {
                conflicting = true;
            }
            automation_confidence = min_automation_confidence(
                automation_confidence,
                parsed_reset.automation_confidence,
            );
            supporting_statement_ids.insert(statement.statement_id.clone());
        }
    }

    if conflicting {
        return None;
    }

    Some(SystemContractRecord {
        clock_signal: clock_signal?,
        reset_signal: reset_signal?,
        reset_kind: reset_kind?,
        reset_polarity: reset_polarity?,
        assertion_timing: assertion_timing?,
        release_timing: release_timing?,
        target_kind: target_kind?,
        supporting_statement_ids: supporting_statement_ids.into_iter().collect(),
        automation_confidence,
    })
}

fn build_init_assignments(context: &SemanticContext) -> Vec<InitAssignmentRecord> {
    let mut accumulators = BTreeMap::<String, InitAssignmentAccumulator>::new();

    for statement in &context.statements {
        let Some(parsed_init) = parse_explicit_init_assignment(&statement.text) else {
            continue;
        };

        let entry = accumulators
            .entry(parsed_init.target_signal.clone())
            .or_insert_with(|| InitAssignmentAccumulator {
                value: parsed_init.value.clone(),
                supporting_statement_ids: BTreeSet::new(),
                conflicting_value: false,
            });
        if entry.value != parsed_init.value {
            entry.conflicting_value = true;
        }
        entry
            .supporting_statement_ids
            .insert(statement.statement_id.clone());
    }

    accumulators
        .into_iter()
        .filter_map(|(target_signal, entry)| {
            (!entry.conflicting_value).then_some(InitAssignmentRecord {
                target_signal,
                value: entry.value,
                supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
                automation_confidence: AutomationConfidence::High,
            })
        })
        .collect()
}

fn build_regular_states(context: &SemanticContext) -> Vec<RegularStateRecord> {
    let mut regular_states = Vec::<RegularStateRecord>::new();
    let mut state_index_by_name = HashMap::<String, usize>::new();

    for statement in &context.statements {
        let Some(parsed_state) = parse_explicit_regular_state_declaration(&statement.text) else {
            continue;
        };

        if let Some(existing_index) = state_index_by_name.get(&parsed_state.state_name).copied() {
            let existing = &mut regular_states[existing_index];
            existing.is_initial |= parsed_state.is_initial;
            if !existing
                .supporting_statement_ids
                .iter()
                .any(|statement_id| statement_id == &statement.statement_id)
            {
                existing
                    .supporting_statement_ids
                    .push(statement.statement_id.clone());
            }
            continue;
        }

        let declaration_order =
            u32::try_from(regular_states.len()).expect("regular-state count should fit in u32");
        let state_id = format!(
            "regular_state_{}",
            document_key(&format!(
                "{}_{}",
                declaration_order, parsed_state.state_name
            ))
        );
        regular_states.push(RegularStateRecord {
            state_id,
            state_name: parsed_state.state_name.clone(),
            is_initial: parsed_state.is_initial,
            declaration_order,
            supporting_statement_ids: vec![statement.statement_id.clone()],
            automation_confidence: AutomationConfidence::High,
        });
        state_index_by_name.insert(parsed_state.state_name, regular_states.len() - 1);
    }

    regular_states
}

fn build_state_transitions(context: &SemanticContext) -> Vec<StateTransitionRecord> {
    let mut state_transitions = Vec::new();

    for statement in &context.statements {
        let Some(parsed_transition) = parse_explicit_state_transition(&statement.text) else {
            continue;
        };

        let declaration_order =
            u32::try_from(state_transitions.len()).expect("transition count should fit in u32");
        state_transitions.push(StateTransitionRecord {
            transition_id: format!(
                "transition_{}",
                document_key(&format!(
                    "{}_{}_{}_{}",
                    declaration_order,
                    parsed_transition.source_state,
                    parsed_transition.target_state,
                    guard_key(parsed_transition.guard.as_ref())
                ))
            ),
            source_state: parsed_transition.source_state,
            target_state: parsed_transition.target_state,
            guard: parsed_transition.guard,
            declaration_order,
            supporting_statement_ids: vec![statement.statement_id.clone()],
            automation_confidence: AutomationConfidence::High,
        });
    }

    state_transitions
}

fn build_decision_tree_fragments(context: &SemanticContext) -> Vec<DecisionTreeFragmentRecord> {
    let mut accumulators: BTreeMap<String, DecisionTreeFragmentAccumulator> = BTreeMap::new();

    for statement in &context.statements {
        let Some(parsed_fragment) = parse_explicit_decision_tree_fragment(&statement.text) else {
            continue;
        };

        let key =
            decision_tree_fragment_key(&parsed_fragment.block_name, parsed_fragment.guard.as_ref());
        let entry = accumulators
            .entry(key)
            .or_insert_with(|| DecisionTreeFragmentAccumulator {
                block_name: parsed_fragment.block_name.clone(),
                guard: parsed_fragment.guard.clone(),
                actions: Vec::new(),
                referenced_signal_names: BTreeSet::new(),
                supporting_statement_ids: BTreeSet::new(),
                automation_confidence: AutomationConfidence::High,
            });
        entry.actions.push(parsed_fragment.action);
        entry
            .referenced_signal_names
            .extend(parsed_fragment.referenced_signal_names);
        entry
            .supporting_statement_ids
            .insert(statement.statement_id.clone());
    }

    accumulators
        .into_values()
        .map(|entry| DecisionTreeFragmentRecord {
            fragment_id: format!(
                "dt_fragment_{}",
                document_key(&format!(
                    "{}_{}",
                    entry.block_name,
                    guard_key(entry.guard.as_ref())
                ))
            ),
            block_name: entry.block_name,
            guard: entry.guard,
            actions: entry.actions,
            referenced_signal_names: entry.referenced_signal_names.into_iter().collect(),
            supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
            automation_confidence: entry.automation_confidence,
        })
        .collect()
}

fn build_symbol_definitions(context: &SemanticContext) -> Vec<SymbolDefinitionRecord> {
    let known_signal_names = known_explicit_signal_names(context);
    let mut scalar_accumulators = BTreeMap::<String, ScalarSymbolAccumulator>::new();
    let mut enum_accumulators = BTreeMap::<String, EnumSymbolAccumulator>::new();

    for statement in &context.statements {
        let known_symbol_names = scalar_accumulators
            .keys()
            .cloned()
            .chain(enum_accumulators.keys().cloned())
            .collect::<BTreeSet<_>>();
        let Some(parsed_definition) = parse_explicit_symbol_definition(
            &statement.text,
            &known_signal_names,
            &known_symbol_names,
        ) else {
            continue;
        };

        match parsed_definition {
            ParsedSymbolDefinition::Scalar {
                kind,
                symbol_name,
                value,
            } => {
                if enum_accumulators.contains_key(&symbol_name) {
                    continue;
                }
                let declaration_order =
                    u32::try_from(scalar_accumulators.len() + enum_accumulators.len())
                        .expect("symbol definition count should fit in u32");
                let entry = scalar_accumulators
                    .entry(symbol_name.clone())
                    .or_insert_with(|| ScalarSymbolAccumulator {
                        kind,
                        symbol_name: symbol_name.clone(),
                        value: value.clone(),
                        declaration_order,
                        supporting_statement_ids: BTreeSet::new(),
                        conflicting_value: false,
                    });
                if entry.kind != kind || entry.value != value {
                    entry.conflicting_value = true;
                }
                entry
                    .supporting_statement_ids
                    .insert(statement.statement_id.clone());
            }
            ParsedSymbolDefinition::EnumMember {
                enum_name,
                member_name,
                value,
            } => {
                if scalar_accumulators.contains_key(&enum_name) {
                    continue;
                }
                let declaration_order =
                    u32::try_from(scalar_accumulators.len() + enum_accumulators.len())
                        .expect("symbol definition count should fit in u32");
                let entry = enum_accumulators
                    .entry(enum_name.clone())
                    .or_insert_with(|| EnumSymbolAccumulator {
                        enum_name: enum_name.clone(),
                        declaration_order,
                        members: BTreeMap::new(),
                        supporting_statement_ids: BTreeSet::new(),
                    });
                entry
                    .supporting_statement_ids
                    .insert(statement.statement_id.clone());
                let member_declaration_order = u32::try_from(entry.members.len())
                    .expect("enum member count should fit in u32");
                let member_entry = entry.members.entry(member_name.clone()).or_insert_with(|| {
                    EnumMemberAccumulator {
                        member_name: member_name.clone(),
                        value: value.clone(),
                        declaration_order: member_declaration_order,
                        supporting_statement_ids: BTreeSet::new(),
                        conflicting_value: false,
                    }
                });
                if member_entry.value != value {
                    member_entry.conflicting_value = true;
                }
                member_entry
                    .supporting_statement_ids
                    .insert(statement.statement_id.clone());
            }
        }
    }

    let known_symbol_names = scalar_accumulators
        .keys()
        .cloned()
        .chain(enum_accumulators.keys().cloned())
        .collect::<BTreeSet<_>>();
    let mut records = scalar_accumulators
        .into_values()
        .filter_map(|entry| {
            (!entry.conflicting_value).then_some(SymbolDefinitionRecord {
                symbol_id: format!(
                    "symbol_{}",
                    document_key(&format!(
                        "{}_{}",
                        entry.declaration_order, entry.symbol_name
                    ))
                ),
                symbol_name: entry.symbol_name,
                kind: entry.kind,
                value: Some(reclassify_control_expression(
                    &entry.value,
                    &known_signal_names,
                    &known_symbol_names,
                )),
                members: Vec::new(),
                declaration_order: entry.declaration_order,
                supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
                automation_confidence: AutomationConfidence::High,
            })
        })
        .collect::<Vec<_>>();

    records.extend(enum_accumulators.into_values().map(|entry| {
        let mut members = entry
            .members
            .into_values()
            .filter_map(|member| {
                (!member.conflicting_value).then_some(SymbolEnumMemberRecord {
                    member_name: member.member_name,
                    value: reclassify_control_expression(
                        &member.value,
                        &known_signal_names,
                        &known_symbol_names,
                    ),
                    declaration_order: member.declaration_order,
                    supporting_statement_ids: member.supporting_statement_ids.into_iter().collect(),
                    automation_confidence: AutomationConfidence::High,
                })
            })
            .collect::<Vec<_>>();
        members.sort_by_key(|member| member.declaration_order);

        SymbolDefinitionRecord {
            symbol_id: format!(
                "symbol_{}",
                document_key(&format!("{}_{}", entry.declaration_order, entry.enum_name))
            ),
            symbol_name: entry.enum_name,
            kind: SymbolDefinitionKind::Enum,
            value: None,
            members,
            declaration_order: entry.declaration_order,
            supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
            automation_confidence: AutomationConfidence::High,
        }
    }));
    records.sort_by_key(|record| record.declaration_order);
    records
}

fn build_control_blocks(
    context: &SemanticContext,
    regular_states: &[RegularStateRecord],
    symbol_definitions: &[SymbolDefinitionRecord],
) -> Vec<ControlBlockRecord> {
    let normalized_regular_state_names = regular_states
        .iter()
        .map(|state| document_key(&state.state_name))
        .collect::<BTreeSet<_>>();
    let known_signal_names = known_explicit_signal_names(context);
    let known_symbol_names = symbol_definitions
        .iter()
        .map(|definition| definition.symbol_name.clone())
        .collect::<BTreeSet<_>>();
    let mut accumulators = BTreeMap::<String, ControlBlockAccumulator>::new();

    for statement in &context.statements {
        let Some(parsed_clause) = parse_explicit_control_clause(
            &statement.text,
            &normalized_regular_state_names,
            &known_signal_names,
            &known_symbol_names,
        ) else {
            continue;
        };

        let block_key = control_block_key(
            &parsed_clause.block_name,
            parsed_clause.role,
            parsed_clause.selector.as_ref(),
        );
        let declaration_order =
            u32::try_from(accumulators.len()).expect("control block count should fit in u32");
        let entry = accumulators
            .entry(block_key)
            .or_insert_with(|| ControlBlockAccumulator {
                block_name: parsed_clause.block_name.clone(),
                role: parsed_clause.role,
                declaration_order,
                selector: parsed_clause.selector.clone(),
                branches: Vec::new(),
                referenced_signal_names: BTreeSet::new(),
                supporting_statement_ids: BTreeSet::new(),
            });
        entry
            .supporting_statement_ids
            .insert(statement.statement_id.clone());
        entry
            .referenced_signal_names
            .extend(parsed_clause.referenced_signal_names.iter().cloned());

        if let Some(existing_branch) = entry
            .branches
            .iter_mut()
            .find(|branch| branch.predicate == parsed_clause.predicate)
        {
            existing_branch
                .actions
                .extend(parsed_clause.actions.clone());
            existing_branch
                .referenced_signal_names
                .extend(parsed_clause.referenced_signal_names);
            existing_branch
                .supporting_statement_ids
                .insert(statement.statement_id.clone());
            continue;
        }

        let branch_declaration_order =
            u32::try_from(entry.branches.len()).expect("control branch count should fit in u32");
        entry.branches.push(ControlBranchAccumulator {
            predicate: parsed_clause.predicate,
            declaration_order: branch_declaration_order,
            actions: parsed_clause.actions,
            referenced_signal_names: parsed_clause.referenced_signal_names,
            supporting_statement_ids: BTreeSet::from([statement.statement_id.clone()]),
        });
    }

    let mut blocks = accumulators.into_values().collect::<Vec<_>>();
    blocks.sort_by_key(|block| block.declaration_order);
    blocks
        .into_iter()
        .map(|entry| ControlBlockRecord {
            block_id: format!(
                "control_block_{}",
                document_key(&format!(
                    "{}_{}_{}_{}",
                    entry.declaration_order,
                    entry.block_name,
                    control_block_role_key(entry.role),
                    control_expression_key(entry.selector.as_ref())
                ))
            ),
            block_name: entry.block_name,
            role: entry.role,
            declaration_order: entry.declaration_order,
            selector: entry.selector,
            branches: entry
                .branches
                .into_iter()
                .map(|branch| ControlBranchRecord {
                    branch_id: format!(
                        "control_branch_{}",
                        document_key(&format!(
                            "{}_{}_{}",
                            entry.declaration_order,
                            branch.declaration_order,
                            control_expression_key(branch.predicate.as_ref())
                        ))
                    ),
                    declaration_order: branch.declaration_order,
                    predicate: branch.predicate,
                    actions: branch.actions,
                    supporting_statement_ids: branch.supporting_statement_ids.into_iter().collect(),
                    automation_confidence: AutomationConfidence::High,
                })
                .collect(),
            referenced_signal_names: entry.referenced_signal_names.into_iter().collect(),
            supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
            automation_confidence: AutomationConfidence::High,
        })
        .collect()
}
fn build_explicit_modules(context: &SemanticContext) -> Vec<ExplicitModuleRecord> {
    let mut accumulators = BTreeMap::<String, ExplicitModuleAccumulator>::new();

    for statement in &context.statements {
        let Some(parsed) = parse_module_scoped_statement(&statement.text) else {
            continue;
        };

        let declaration_order =
            u32::try_from(accumulators.len()).expect("explicit module count should fit in u32");
        let entry = accumulators
            .entry(parsed.module_name.clone())
            .or_insert_with(|| ExplicitModuleAccumulator {
                module_name: parsed.module_name.clone(),
                declaration_order,
                statements: Vec::new(),
                supporting_statement_ids: BTreeSet::new(),
            });
        entry
            .supporting_statement_ids
            .insert(statement.statement_id.clone());
        if let Some(scoped_text) = parsed.scoped_text {
            entry
                .statements
                .push(statement.with_rewritten_text(scoped_text));
        }
    }

    let mut modules = accumulators.into_values().collect::<Vec<_>>();
    modules.sort_by_key(|module| module.declaration_order);
    modules
        .into_iter()
        .map(build_explicit_module_record)
        .collect()
}

fn build_explicit_module_record(accumulator: ExplicitModuleAccumulator) -> ExplicitModuleRecord {
    let scoped_context = SemanticContext {
        statements: accumulator.statements,
        section_anchors: Vec::new(),
        visual_roles_by_id: HashMap::new(),
    };
    let interfaces = build_interfaces(&scoped_context);
    let system_contract = build_system_contract(&scoped_context);
    let init_assignments = build_init_assignments(&scoped_context);
    let regular_states = build_regular_states(&scoped_context);
    let state_transitions = build_state_transitions(&scoped_context);
    let decision_tree_fragments = build_decision_tree_fragments(&scoped_context);
    let symbol_definitions = build_symbol_definitions(&scoped_context);
    let control_blocks = build_control_blocks(
        &scoped_context,
        regular_states.as_slice(),
        symbol_definitions.as_slice(),
    );

    ExplicitModuleRecord {
        module_id: format!(
            "explicit_module_{}",
            document_key(&format!(
                "{}_{}",
                accumulator.declaration_order, accumulator.module_name
            ))
        ),
        module_name: accumulator.module_name,
        declaration_order: accumulator.declaration_order,
        interfaces,
        system_contract,
        init_assignments,
        regular_states,
        state_transitions,
        decision_tree_fragments,
        symbol_definitions,
        control_blocks,
        supporting_statement_ids: accumulator.supporting_statement_ids.into_iter().collect(),
        automation_confidence: AutomationConfidence::High,
    }
}

fn build_explicit_tops(context: &SemanticContext) -> Vec<ExplicitTopRecord> {
    let mut accumulators = BTreeMap::<String, ExplicitTopAccumulator>::new();

    for statement in &context.statements {
        let Some(parsed) = parse_top_scoped_statement(&statement.text) else {
            continue;
        };

        let declaration_order =
            u32::try_from(accumulators.len()).expect("explicit top count should fit in u32");
        let entry = accumulators
            .entry(parsed.top_name.clone())
            .or_insert_with(|| ExplicitTopAccumulator {
                top_name: parsed.top_name.clone(),
                declaration_order,
                ports: Vec::new(),
                children: Vec::new(),
                links: Vec::new(),
                supporting_statement_ids: BTreeSet::new(),
            });
        entry
            .supporting_statement_ids
            .insert(statement.statement_id.clone());

        let Some(scoped_text) = parsed.scoped_text.as_deref() else {
            continue;
        };

        if let Some(parsed_port) = parse_explicit_top_port(scoped_text) {
            let declaration_order =
                u32::try_from(entry.ports.len()).expect("top port count should fit in u32");
            entry.ports.push(ExplicitTopPortRecord {
                port_name: parsed_port.signal_name,
                direction_hint: parsed_port.direction_hint,
                width_hint: parsed_port.width_hint,
                declaration_order,
                supporting_statement_ids: vec![statement.statement_id.clone()],
                automation_confidence: AutomationConfidence::High,
            });
            continue;
        }

        if let Some(parsed_child) = parse_explicit_top_child(scoped_text) {
            let declaration_order =
                u32::try_from(entry.children.len()).expect("top child count should fit in u32");
            entry.children.push(ExplicitTopChildRecord {
                instance_name: parsed_child.instance_name,
                source_module_name: parsed_child.source_module_name,
                declaration_order,
                supporting_statement_ids: vec![statement.statement_id.clone()],
                automation_confidence: AutomationConfidence::High,
            });
            continue;
        }

        if let Some(parsed_link) = parse_explicit_top_link(scoped_text) {
            let declaration_order =
                u32::try_from(entry.links.len()).expect("top link count should fit in u32");
            entry.links.push(ExplicitTopLinkRecord {
                link_id: format!(
                    "top_link_{}",
                    document_key(&format!(
                        "{}_{}_{}_{}",
                        entry.top_name,
                        declaration_order,
                        explicit_top_link_endpoint_key(&parsed_link.source),
                        explicit_top_link_endpoint_key(&parsed_link.target)
                    ))
                ),
                source: parsed_link.source,
                target: parsed_link.target,
                declaration_order,
                supporting_statement_ids: vec![statement.statement_id.clone()],
                automation_confidence: AutomationConfidence::High,
            });
        }
    }

    let mut tops = accumulators.into_values().collect::<Vec<_>>();
    tops.sort_by_key(|top| top.declaration_order);
    tops.into_iter()
        .map(|top| ExplicitTopRecord {
            top_id: format!(
                "explicit_top_{}",
                document_key(&format!("{}_{}", top.declaration_order, top.top_name))
            ),
            top_name: top.top_name,
            declaration_order: top.declaration_order,
            ports: top.ports,
            children: top.children,
            links: top.links,
            supporting_statement_ids: top.supporting_statement_ids.into_iter().collect(),
            automation_confidence: AutomationConfidence::High,
        })
        .collect()
}

fn build_actors(context: &SemanticContext, interfaces: &[InterfaceRecord]) -> ActorBuildResult {
    const ACTOR_TERMS: &[&str] = &[
        "transmitter",
        "receiver",
        "sender",
        "requester",
        "responder",
        "producer",
        "consumer",
        "controller",
        "manager",
        "device",
        "host",
        "client",
        "server",
        "initiator",
        "target",
        "source",
        "sink",
        "agent",
        "arbiter",
        "scheduler",
        "decoder",
        "encoder",
        "channel",
        "state machine",
    ];

    let mut accumulators: BTreeMap<String, ActorAccumulator> = BTreeMap::new();
    let mut actor_id_by_term = HashMap::new();

    for statement in &context.statements {
        let lowered_text = statement.text.to_ascii_lowercase();
        for term in ACTOR_TERMS {
            if !contains_phrase(&lowered_text, term) {
                continue;
            }

            let actor_id = format!("actor_{}", document_key(term));
            actor_id_by_term.insert((*term).to_string(), actor_id.clone());
            let entry = accumulators
                .entry(actor_id)
                .or_insert_with(|| ActorAccumulator {
                    role_summary: format!("semantic role inferred around `{term}` evidence"),
                    supporting_statement_ids: BTreeSet::new(),
                    supporting_section_ids: BTreeSet::new(),
                });
            entry
                .supporting_statement_ids
                .insert(statement.statement_id.clone());
            entry
                .supporting_section_ids
                .extend(statement.section_ids.iter().cloned());
        }
    }

    let explicit_actor_count = accumulators.len();
    if accumulators.is_empty() {
        for interface in interfaces {
            let actor_id = format!(
                "actor_{}_channel",
                document_key(&interface.signals.join("_"))
            );
            let supporting_section_ids = statement_ids_to_section_ids(
                context,
                interface.supporting_statement_ids.as_slice(),
            );
            accumulators.insert(
                actor_id,
                ActorAccumulator {
                    role_summary: format!(
                        "semantic channel inferred from grouped interface signals: {}",
                        interface.signals.join(", ")
                    ),
                    supporting_statement_ids: interface
                        .supporting_statement_ids
                        .iter()
                        .cloned()
                        .collect(),
                    supporting_section_ids: supporting_section_ids.into_iter().collect(),
                },
            );
        }
    }

    let actors = accumulators
        .into_iter()
        .map(|(actor_id, entry)| ActorRecord {
            actor_id,
            role_summary: entry.role_summary,
            supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
            supporting_section_ids: entry.supporting_section_ids.into_iter().collect(),
        })
        .collect();

    ActorBuildResult {
        actors,
        actor_id_by_term,
        explicit_actor_count,
    }
}

fn build_phases(context: &SemanticContext) -> Vec<PhaseRecord> {
    context
        .section_anchors
        .iter()
        .filter(|section| {
            !section.supporting_statement_ids.is_empty()
                && (phase_like_title(&section.title)
                    || section
                        .supporting_statement_ids
                        .iter()
                        .filter_map(|statement_id| statement_by_id(context, statement_id))
                        .any(|statement| sequencing_language(&statement.text)))
        })
        .map(|section| PhaseRecord {
            phase_id: format!("phase_{}", document_key(&section.title)),
            summary: format!("semantic phase derived from section `{}`", section.title),
            supporting_statement_ids: section.supporting_statement_ids.clone(),
            supporting_section_ids: vec![section.section_id.clone()],
        })
        .collect()
}

fn build_invariants(
    context: &SemanticContext,
    interface_ids_by_signal: &HashMap<String, BTreeSet<String>>,
) -> Vec<InvariantRecord> {
    let mut invariants = Vec::new();
    let mut seen = BTreeSet::new();

    for statement in &context.statements {
        if !is_invariant_like(statement, context) {
            continue;
        }

        let dedupe_key = normalize_text_key(&statement.text);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        invariants.push(InvariantRecord {
            invariant_id: format!("invariant_{}", document_key(&dedupe_key)),
            statement: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
            related_interface_ids: related_interface_ids(
                statement.signals.as_slice(),
                interface_ids_by_signal,
            ),
        });
    }

    invariants
}

fn build_contracts(
    context: &SemanticContext,
    actor_id_by_term: &HashMap<String, String>,
) -> Vec<ContractRecord> {
    let mut contracts = Vec::new();
    let mut seen = BTreeSet::new();

    for statement in &context.statements {
        let lowered_text = statement.text.to_ascii_lowercase();
        if !contains_any_phrase(
            &lowered_text,
            &[
                "must",
                "shall",
                "required",
                "responsible",
                "may",
                "allowed",
                "forbidden",
                "prohibited",
            ],
        ) {
            continue;
        }

        let actor_ids = actor_ids_for_text(&lowered_text, actor_id_by_term);
        if actor_ids.is_empty() {
            continue;
        }

        let dedupe_key = normalize_text_key(&statement.text);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        contracts.push(ContractRecord {
            contract_id: format!("contract_{}", document_key(&dedupe_key)),
            statement: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
            actor_ids,
        });
    }

    contracts
}

fn build_gates(
    context: &SemanticContext,
    interface_ids_by_signal: &HashMap<String, BTreeSet<String>>,
) -> Vec<GateRecord> {
    let mut gates = Vec::new();
    let mut seen = BTreeSet::new();

    for statement in &context.statements {
        let lowered_text = statement.text.to_ascii_lowercase();
        if !contains_any_phrase(
            &lowered_text,
            &[
                "if",
                "when",
                "unless",
                "only when",
                "while",
                "after",
                "before",
                "until",
            ],
        ) {
            continue;
        }

        let dedupe_key = normalize_text_key(&statement.text);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        gates.push(GateRecord {
            gate_id: format!("gate_{}", document_key(&dedupe_key)),
            condition: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
            related_interface_ids: related_interface_ids(
                statement.signals.as_slice(),
                interface_ids_by_signal,
            ),
        });
    }

    gates
}

fn build_assertions(context: &SemanticContext) -> Vec<AssertionRecord> {
    let mut assertions = Vec::new();
    let mut seen = BTreeSet::new();

    for statement in &context.statements {
        let lowered_text = statement.text.to_ascii_lowercase();
        if !contains_any_phrase(
            &lowered_text,
            &[
                "assert",
                "assertion",
                "must not",
                "shall not",
                "must never",
                "shall never",
                "illegal",
                "forbidden",
            ],
        ) {
            continue;
        }

        let dedupe_key = normalize_text_key(&statement.text);
        if !seen.insert(dedupe_key.clone()) {
            continue;
        }

        assertions.push(AssertionRecord {
            assertion_id: format!("assertion_{}", document_key(&dedupe_key)),
            statement: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
        });
    }

    assertions
}

fn build_abstractions(context: &SemanticContext) -> Vec<AbstractionRecord> {
    context
        .statements
        .iter()
        .filter(|statement| matches!(statement.class, StatementClass::ExplicitAbstraction))
        .map(|statement| AbstractionRecord {
            abstraction_id: format!("abstraction_{}", document_key(&statement.statement_id)),
            description: statement.text.clone(),
            supporting_statement_ids: vec![statement.statement_id.clone()],
        })
        .collect()
}

fn build_decomposition_candidates(context: &SemanticContext) -> Vec<DecompositionCandidate> {
    context
        .section_anchors
        .iter()
        .filter(|section| {
            section.supporting_statement_ids.len() >= 2 || decomposition_like_title(&section.title)
        })
        .map(|section| DecompositionCandidate {
            candidate_id: format!("candidate_{}", document_key(&section.title)),
            summary: format!("semantic cluster around section `{}`", section.title),
            supporting_statement_ids: section.supporting_statement_ids.clone(),
            supporting_section_ids: vec![section.section_id.clone()],
        })
        .collect()
}

fn build_residual_decisions(
    context: &SemanticContext,
    interfaces: &[InterfaceRecord],
    explicit_actor_count: usize,
) -> Vec<ResidualDecisionPacket> {
    let mut packets = Vec::new();

    if explicit_actor_count == 0 && !interfaces.is_empty() {
        packets.push(ResidualDecisionPacket {
            packet_id: "semantic_actor_boundary_inference".to_string(),
            question: "Which actor boundary should own the inferred interface semantics?".to_string(),
            why_unresolved: "The first-pass SemanticIR builder inferred interface-level structure from evidence statements, but the evidence does not name stable endpoint actors explicitly.".to_string(),
            automation_confidence: AutomationConfidence::Medium,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "channel_actor".to_string(),
                    description: "Represent the grouped signals as a channel-like actor owned by the interface semantics.".to_string(),
                    downstream_impact: "SemanticIR remains conservative and backend-neutral, but later IntentIR stages may need endpoint decomposition.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "endpoint_pair".to_string(),
                    description: "Split the semantics into two endpoint actors that exchange the inferred interface signals.".to_string(),
                    downstream_impact: "Later stages gain clearer endpoint responsibilities, but actor invention risk is higher without explicit textual support.".to_string(),
                },
            ],
        });
    }

    let ambiguous_visual_ids: Vec<String> = context
        .statements
        .iter()
        .flat_map(|statement| statement.related_visual_evidence_ids.iter())
        .filter(|visual_id| {
            context
                .visual_roles_by_id
                .get(*visual_id)
                .is_some_and(|role| {
                    matches!(
                        role,
                        VisualEvidenceRole::Ambiguous | VisualEvidenceRole::Unknown
                    )
                })
        })
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    if !ambiguous_visual_ids.is_empty() {
        packets.push(ResidualDecisionPacket {
            packet_id: "semantic_ambiguous_visual_grounding".to_string(),
            question: "Do the ambiguous visual artifacts carry normative semantics that must be lifted into SemanticIR?".to_string(),
            why_unresolved: format!(
                "EvidenceIR links the current semantic slice to ambiguous visual evidence items ({}) whose role is not safely classifiable as purely illustrative.",
                ambiguous_visual_ids.join(", ")
            ),
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
        });
    }

    let overlapping_signals = overlapping_interface_signals(interfaces);
    if !overlapping_signals.is_empty() {
        packets.push(ResidualDecisionPacket {
            packet_id: "semantic_interface_grouping".to_string(),
            question: "Should overlapping signal groups be merged into one interface or kept as coupled interfaces?".to_string(),
            why_unresolved: format!(
                "The first-pass interface extractor observed overlapping signal participation for {} across multiple grouped statements.",
                overlapping_signals.join(", ")
            ),
            automation_confidence: AutomationConfidence::Medium,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "single_bus_interface".to_string(),
                    description: "Merge the overlapping signals into a broader bus/interface abstraction.".to_string(),
                    downstream_impact: "SemanticIR becomes simpler, but protocol subchannels may be flattened too early.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "multiple_coupled_interfaces".to_string(),
                    description: "Keep the overlapping signals in multiple related interface records.".to_string(),
                    downstream_impact: "SemanticIR preserves local structure, but later IntentIR stages must model coupling explicitly.".to_string(),
                },
            ],
        });
    }

    packets
}

fn section_ids_for_statement(
    evidence_span_ids: &[String],
    spans_by_id: &HashMap<String, (Option<u32>, Option<u32>)>,
    section_anchors: &[crate::ir::evidence::SectionAnchor],
) -> Vec<String> {
    let mut section_ids = BTreeSet::new();

    for span_id in evidence_span_ids {
        let Some((line_start, line_end)) = spans_by_id.get(span_id) else {
            continue;
        };

        let resolved_line_start = line_start.or(*line_end);
        let resolved_line_end = line_end.or(resolved_line_start);
        let Some(line_start) = resolved_line_start else {
            continue;
        };
        let Some(line_end) = resolved_line_end else {
            continue;
        };

        for anchor in section_anchors {
            let Some(anchor_start) = anchor.line_start else {
                continue;
            };
            let Some(anchor_end) = anchor.line_end else {
                continue;
            };

            if line_start <= anchor_end && line_end >= anchor_start {
                section_ids.insert(anchor.section_id.clone());
            }
        }
    }

    section_ids.into_iter().collect()
}

fn parse_explicit_signal_declaration(text: &str) -> Option<ParsedInterfaceSignalDeclaration> {
    let normalized = normalize_sentence(text);
    let normalized = normalized
        .trim()
        .trim_end_matches('.')
        .trim_end_matches(':');
    let tokens: Vec<&str> = normalized.split_whitespace().collect();
    if tokens.len() < 3 || !tokens[0].eq_ignore_ascii_case("signal") {
        return None;
    }

    let signal_name = parse_identifier(tokens[1])?;
    let mut index = 2usize;
    if tokens
        .get(index)
        .is_some_and(|token| token.eq_ignore_ascii_case("is"))
    {
        index += 1;
    }

    let direction_hint = parse_interface_signal_direction(*tokens.get(index)?)?;
    index += 1;

    let width_hint = parse_optional_width_hint(tokens.as_slice(), &mut index);
    if index != tokens.len() {
        return None;
    }

    Some(ParsedInterfaceSignalDeclaration {
        signal_name,
        direction_hint,
        width_hint,
    })
}

fn parse_explicit_system_clock(text: &str) -> Option<String> {
    let normalized = normalize_sentence(text);
    let normalized = normalized
        .trim()
        .trim_end_matches('.')
        .trim_end_matches(':');
    let tokens: Vec<&str> = normalized.split_whitespace().collect();
    if tokens.is_empty() || !tokens[0].eq_ignore_ascii_case("clock") {
        return None;
    }

    match tokens.as_slice() {
        [_, signal_name] => parse_identifier(signal_name),
        [_, middle, signal_name]
            if middle.eq_ignore_ascii_case("signal") || middle.eq_ignore_ascii_case("is") =>
        {
            parse_identifier(signal_name)
        }
        _ => None,
    }
}

fn parse_explicit_system_reset(text: &str) -> Option<ParsedSystemResetDeclaration> {
    let normalized = normalize_sentence(text);
    let normalized = normalized
        .trim()
        .trim_end_matches('.')
        .trim_end_matches(':');
    let tokens: Vec<&str> = normalized.split_whitespace().collect();
    if tokens.len() < 4 || !tokens[0].eq_ignore_ascii_case("reset") {
        return None;
    }

    let (signal_name, is_token_index) =
        if tokens[1].eq_ignore_ascii_case("signal") && tokens.len() >= 5 {
            (parse_identifier(tokens[2])?, 3usize)
        } else {
            (parse_identifier(tokens[1])?, 2usize)
        };

    if !tokens
        .get(is_token_index)
        .is_some_and(|token| token.eq_ignore_ascii_case("is"))
    {
        return None;
    }

    let (reset_kind, reset_polarity, automation_confidence) =
        parse_system_reset_descriptor(&signal_name, &tokens[is_token_index + 1..])?;

    Some(ParsedSystemResetDeclaration {
        signal_name,
        reset_kind,
        reset_polarity,
        assertion_timing: reset_kind.assertion_timing(),
        release_timing: reset_kind.release_timing(),
        target_kind: reset_kind.target_kind(),
        automation_confidence,
    })
}

fn parse_system_reset_descriptor(
    signal_name: &str,
    tokens: &[&str],
) -> Option<(SystemResetKind, SystemResetPolarity, AutomationConfidence)> {
    if tokens.is_empty() {
        return None;
    }

    let normalized_tokens = tokens
        .iter()
        .map(|token| token.trim_end_matches('.').to_ascii_lowercase())
        .collect::<Vec<_>>();
    let mut reset_kind = None;
    let mut reset_polarity = None;
    let mut index = 0usize;

    while index < normalized_tokens.len() {
        match normalized_tokens[index].as_str() {
            "sync" | "synchronous" => {
                if reset_kind.replace(SystemResetKind::Synchronous).is_some() {
                    return None;
                }
                index += 1;
            }
            "async" | "asynchronous" => {
                if reset_kind.replace(SystemResetKind::Asynchronous).is_some() {
                    return None;
                }
                index += 1;
            }
            "active" => {
                let level = normalized_tokens.get(index + 1)?;
                let parsed_polarity = match level.as_str() {
                    "high" => SystemResetPolarity::ActiveHigh,
                    "low" => SystemResetPolarity::ActiveLow,
                    _ => return None,
                };
                if reset_polarity.replace(parsed_polarity).is_some() {
                    return None;
                }
                index += 2;
            }
            _ => return None,
        }
    }

    let reset_kind = reset_kind?;
    let (reset_polarity, automation_confidence) = match reset_polarity {
        Some(reset_polarity) => (reset_polarity, AutomationConfidence::High),
        None => (
            infer_system_reset_polarity(signal_name),
            AutomationConfidence::Medium,
        ),
    };

    Some((reset_kind, reset_polarity, automation_confidence))
}

fn infer_system_reset_polarity(signal_name: &str) -> SystemResetPolarity {
    if reset_signal_name_looks_active_low(signal_name) {
        SystemResetPolarity::ActiveLow
    } else {
        SystemResetPolarity::ActiveHigh
    }
}

fn reset_signal_name_looks_active_low(signal_name: &str) -> bool {
    let lowered = signal_name.to_ascii_lowercase();
    lowered.ends_with("_n")
        || lowered.ends_with("_b")
        || matches!(lowered.as_str(), "rstn" | "rstb" | "resetn" | "resetb")
}

fn parse_explicit_init_assignment(text: &str) -> Option<ParsedInitAssignment> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    if !normalized.to_ascii_lowercase().starts_with("init ") {
        return None;
    }

    let body = normalized[5..].trim();
    let (target_signal, value_text) = body.split_once('=')?;
    Some(ParsedInitAssignment {
        target_signal: parse_identifier(target_signal.trim())?,
        value: parse_decision_tree_value(value_text.trim())?,
    })
}

fn parse_explicit_symbol_definition(
    text: &str,
    known_signal_names: &BTreeSet<String>,
    known_symbol_names: &BTreeSet<String>,
) -> Option<ParsedSymbolDefinition> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    let lowered = normalized.to_ascii_lowercase();

    if lowered.starts_with("constant ") {
        return parse_scalar_symbol_definition(
            SymbolDefinitionKind::Constant,
            &normalized[9..],
            known_signal_names,
            known_symbol_names,
        );
    }
    if lowered.starts_with("define ") {
        return parse_scalar_symbol_definition(
            SymbolDefinitionKind::Define,
            &normalized[7..],
            known_signal_names,
            known_symbol_names,
        );
    }
    if lowered.starts_with("param ") {
        return parse_scalar_symbol_definition(
            SymbolDefinitionKind::Param,
            &normalized[6..],
            known_signal_names,
            known_symbol_names,
        );
    }
    if lowered.starts_with("parameter ") {
        return parse_scalar_symbol_definition(
            SymbolDefinitionKind::Param,
            &normalized[10..],
            known_signal_names,
            known_symbol_names,
        );
    }
    if lowered.starts_with("enum ") {
        return parse_enum_symbol_definition(
            &normalized[5..],
            known_signal_names,
            known_symbol_names,
        );
    }

    None
}

fn parse_scalar_symbol_definition(
    kind: SymbolDefinitionKind,
    body: &str,
    known_signal_names: &BTreeSet<String>,
    known_symbol_names: &BTreeSet<String>,
) -> Option<ParsedSymbolDefinition> {
    let (symbol_name, value_text) = body.split_once('=')?;
    Some(ParsedSymbolDefinition::Scalar {
        kind,
        symbol_name: parse_identifier(symbol_name.trim())?,
        value: parse_control_expression(value_text.trim(), known_signal_names, known_symbol_names)?,
    })
}

fn parse_enum_symbol_definition(
    body: &str,
    known_signal_names: &BTreeSet<String>,
    known_symbol_names: &BTreeSet<String>,
) -> Option<ParsedSymbolDefinition> {
    let (lhs, value_text) = body.split_once('=')?;
    let lhs_tokens = lhs.split_whitespace().collect::<Vec<_>>();
    if lhs_tokens.len() != 2 {
        return None;
    }

    Some(ParsedSymbolDefinition::EnumMember {
        enum_name: parse_identifier(lhs_tokens[0])?,
        member_name: parse_identifier(lhs_tokens[1])?,
        value: parse_control_expression(value_text.trim(), known_signal_names, known_symbol_names)?,
    })
}

fn parse_module_scoped_statement(text: &str) -> Option<ParsedModuleScopedStatement> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    if !normalized.to_ascii_lowercase().starts_with("module ") {
        return None;
    }

    let body = normalized[7..].trim();
    let (module_name, scoped_text) = split_scoped_statement_body(body)?;
    Some(ParsedModuleScopedStatement {
        module_name,
        scoped_text,
    })
}

fn parse_top_scoped_statement(text: &str) -> Option<ParsedTopScopedStatement> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    if !normalized.to_ascii_lowercase().starts_with("top ") {
        return None;
    }

    let body = normalized[4..].trim();
    let (top_name, scoped_text) = split_scoped_statement_body(body)?;
    Some(ParsedTopScopedStatement {
        top_name,
        scoped_text,
    })
}

fn split_scoped_statement_body(body: &str) -> Option<(String, Option<String>)> {
    let body = body.trim();
    if body.is_empty() {
        return None;
    }

    let split_index = body
        .char_indices()
        .find_map(|(index, character)| character.is_whitespace().then_some(index));
    let (name_text, scoped_text) = match split_index {
        Some(index) => (&body[..index], Some(body[index..].trim().to_string())),
        None => (body, None),
    };
    let name = parse_identifier(name_text)?;
    let scoped_text = scoped_text.filter(|text| !text.is_empty());
    Some((name, scoped_text))
}

fn parse_explicit_top_port(text: &str) -> Option<ParsedInterfaceSignalDeclaration> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    if !normalized.to_ascii_lowercase().starts_with("port ") {
        return None;
    }

    parse_explicit_signal_declaration(&format!("Signal {}", normalized[5..].trim()))
}

fn parse_explicit_top_child(text: &str) -> Option<ParsedExplicitTopChild> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    if !normalized.to_ascii_lowercase().starts_with("child ") {
        return None;
    }

    let tokens = normalized[6..].split_whitespace().collect::<Vec<_>>();
    if tokens.len() != 4
        || !tokens[1].eq_ignore_ascii_case("uses")
        || !tokens[2].eq_ignore_ascii_case("module")
    {
        return None;
    }

    Some(ParsedExplicitTopChild {
        instance_name: parse_identifier(tokens[0])?,
        source_module_name: parse_identifier(tokens[3])?,
    })
}

fn parse_explicit_top_link(text: &str) -> Option<ParsedExplicitTopLink> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    if !normalized.to_ascii_lowercase().starts_with("link ") {
        return None;
    }

    let body = normalized[5..].trim();
    let (source_text, target_text) = body.split_once("->")?;
    Some(ParsedExplicitTopLink {
        source: parse_explicit_top_link_endpoint(source_text.trim())?,
        target: parse_explicit_top_link_endpoint(target_text.trim())?,
    })
}

fn parse_explicit_top_link_endpoint(text: &str) -> Option<ExplicitTopLinkEndpoint> {
    if let Some((instance_name, signal_name)) = text.split_once('.') {
        return Some(ExplicitTopLinkEndpoint {
            instance_name: Some(parse_identifier(instance_name.trim())?),
            signal_name: parse_identifier(signal_name.trim())?,
        });
    }

    Some(ExplicitTopLinkEndpoint {
        instance_name: None,
        signal_name: parse_identifier(text.trim())?,
    })
}

fn parse_explicit_regular_state_declaration(text: &str) -> Option<ParsedRegularStateDeclaration> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    if !normalized.to_ascii_lowercase().starts_with("state ") {
        return None;
    }

    let body = normalized[6..].trim();
    let (state_name_text, suffix) = if let Some((state_name_text, suffix)) = body.split_once(" is ")
    {
        (state_name_text.trim(), Some(suffix.trim()))
    } else {
        (body, None)
    };
    let state_name = parse_identifier(state_name_text)?;
    let is_initial = match suffix {
        None => false,
        Some(suffix) if suffix.eq_ignore_ascii_case("initial") => true,
        _ => return None,
    };

    Some(ParsedRegularStateDeclaration {
        state_name,
        is_initial,
    })
}

fn parse_explicit_state_transition(text: &str) -> Option<ParsedStateTransition> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    if !normalized.to_ascii_lowercase().starts_with("transition ") {
        return None;
    }

    let body = normalized[11..].trim();
    let (source_state_text, remainder) = body.split_once("->")?;
    let source_state = parse_identifier(source_state_text.trim())?;
    let remainder = remainder.trim();
    let remainder_lower = remainder.to_ascii_lowercase();
    let (target_state_text, guard) = if let Some(index) = remainder_lower.find(" when ") {
        (
            remainder[..index].trim(),
            Some(parse_explicit_decision_tree_guard(
                remainder[index + 6..].trim(),
            )?),
        )
    } else {
        (remainder, None)
    };

    Some(ParsedStateTransition {
        source_state,
        target_state: parse_identifier(target_state_text)?,
        guard,
    })
}

fn parse_explicit_decision_tree_fragment(text: &str) -> Option<ParsedDecisionTreeFragment> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    let lower = normalized.to_ascii_lowercase();
    if !lower.starts_with("block ") {
        return None;
    }

    let body = normalized[6..].trim();
    let (header, action_clause) = body.split_once(':')?;
    let header = header.trim();
    let action_clause = action_clause.trim();
    if header.is_empty() || action_clause.is_empty() {
        return None;
    }

    let header_lower = header.to_ascii_lowercase();
    let (raw_block_name, raw_guard) = if let Some(index) = header_lower.find(" when ") {
        (&header[..index], Some(&header[index + 6..]))
    } else {
        (header, None)
    };

    let block_name = normalize_decision_tree_block_name(raw_block_name)?;
    let guard = match raw_guard {
        Some(guard_text) => Some(parse_explicit_decision_tree_guard(guard_text.trim())?),
        None => None,
    };
    let action = parse_explicit_decision_tree_action(action_clause)?;

    let mut referenced_signal_names = BTreeSet::new();
    if let Some(guard) = guard.as_ref() {
        referenced_signal_names.extend(referenced_signal_names_for_guard(guard));
    }
    referenced_signal_names.extend(referenced_signal_names_for_action(&action));

    Some(ParsedDecisionTreeFragment {
        block_name,
        guard,
        action,
        referenced_signal_names,
    })
}

fn parse_explicit_control_clause(
    text: &str,
    normalized_regular_state_names: &BTreeSet<String>,
    known_signal_names: &BTreeSet<String>,
    known_symbol_names: &BTreeSet<String>,
) -> Option<ParsedControlClause> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    let (explicit_role, body) = parse_control_role_prefix(normalized)?;
    let (header, action_clause) = body.split_once(':')?;
    let header = header.trim();
    let action_clause = action_clause.trim();
    if header.is_empty() || action_clause.is_empty() {
        return None;
    }

    let (header_without_predicate, predicate_text) = split_control_header_keyword(header, " when ");
    let (raw_block_name, selector_text) =
        split_control_header_keyword(header_without_predicate.trim(), " select ");
    let block_name = normalize_decision_tree_block_name(raw_block_name)?;
    let role = explicit_role.unwrap_or_else(|| {
        if normalized_regular_state_names.contains(&block_name) {
            ControlBlockRole::StateBody
        } else {
            ControlBlockRole::StandaloneDecisionTree
        }
    });
    let selector = match selector_text {
        Some(selector_text) => Some(parse_control_expression(
            selector_text.trim(),
            known_signal_names,
            known_symbol_names,
        )?),
        None => None,
    };
    let predicate = match predicate_text {
        Some(predicate_text) => Some(parse_control_expression(
            predicate_text.trim(),
            known_signal_names,
            known_symbol_names,
        )?),
        None => None,
    };
    let actions = split_control_actions(action_clause)
        .into_iter()
        .map(|action_text| {
            parse_explicit_control_action(action_text, known_signal_names, known_symbol_names)
        })
        .collect::<Option<Vec<_>>>()?;
    if actions.is_empty() {
        return None;
    }

    let mut referenced_signal_names = BTreeSet::new();
    if let Some(selector) = selector.as_ref() {
        referenced_signal_names.extend(referenced_signal_names_for_control_expression(selector));
    }
    if let Some(predicate) = predicate.as_ref() {
        referenced_signal_names.extend(referenced_signal_names_for_control_expression(predicate));
    }
    for action in &actions {
        referenced_signal_names.extend(referenced_signal_names_for_control_action(action));
    }

    Some(ParsedControlClause {
        block_name,
        role,
        selector,
        predicate,
        actions,
        referenced_signal_names,
    })
}

fn parse_control_role_prefix(normalized: &str) -> Option<(Option<ControlBlockRole>, &str)> {
    let lowered = normalized.to_ascii_lowercase();
    for (prefix, role) in [
        ("block ", None),
        ("syncreset ", Some(ControlBlockRole::ResetSynchronous)),
        ("sync reset ", Some(ControlBlockRole::ResetSynchronous)),
        (
            "synchronous reset ",
            Some(ControlBlockRole::ResetSynchronous),
        ),
        ("asyncreset ", Some(ControlBlockRole::ResetAsynchronous)),
        ("async reset ", Some(ControlBlockRole::ResetAsynchronous)),
        (
            "asynchronous reset ",
            Some(ControlBlockRole::ResetAsynchronous),
        ),
    ] {
        if lowered.starts_with(prefix) {
            return Some((role, normalized[prefix.len()..].trim()));
        }
    }

    None
}

fn split_control_header_keyword<'a>(text: &'a str, keyword: &str) -> (&'a str, Option<&'a str>) {
    let lowered = text.to_ascii_lowercase();
    if let Some(index) = lowered.find(keyword) {
        return (&text[..index], Some(&text[index + keyword.len()..]));
    }

    (text, None)
}

fn split_control_actions(action_clause: &str) -> Vec<&str> {
    action_clause
        .split(';')
        .map(str::trim)
        .filter(|action| !action.is_empty())
        .collect()
}

fn parse_explicit_control_action(
    text: &str,
    known_signal_names: &BTreeSet<String>,
    known_symbol_names: &BTreeSet<String>,
) -> Option<ControlActionRecord> {
    let normalized = normalize_sentence(text);
    let normalized = normalized.trim().trim_end_matches('.');
    let lowered = normalized.to_ascii_lowercase();

    if lowered.starts_with("transition ") {
        return Some(ControlActionRecord::Transition {
            target_state: parse_identifier(normalized[11..].trim())?,
        });
    }
    if normalized.starts_with("->") {
        return Some(ControlActionRecord::Transition {
            target_state: parse_identifier(normalized[2..].trim())?,
        });
    }
    if lowered.starts_with("pulse ") {
        let body = normalized[6..].trim();
        let body_lower = body.to_ascii_lowercase();
        let after_index = body_lower.find(" after ")?;
        let target_text = body[..after_index].trim();
        let remainder = body[after_index + 7..].trim();
        let delay_end = remainder
            .char_indices()
            .find_map(|(index, character)| character.is_whitespace().then_some(index))?;
        let delay = parse_u32_token(remainder[..delay_end].trim())?;
        let assignment_text = remainder[delay_end..].trim();
        let (_, _, value_text) = split_explicit_assignment(assignment_text)?;
        let (target, dual_output) = parse_control_assignment_target(target_text)?;
        if dual_output.is_some() {
            return None;
        }

        return Some(ControlActionRecord::DelayedPulse {
            target,
            delay,
            value: parse_control_expression(value_text, known_signal_names, known_symbol_names)?,
        });
    }
    if let Some((target_text, amount_text)) = normalized.split_once("+=") {
        let (target, dual_output) = parse_control_assignment_target(target_text.trim())?;
        if dual_output.is_some() {
            return None;
        }

        return Some(ControlActionRecord::CompoundUpdate {
            target,
            operation: ControlCompoundUpdateOperation::Increment,
            amount: Some(parse_control_expression(
                amount_text.trim(),
                known_signal_names,
                known_symbol_names,
            )?),
        });
    }
    if let Some((target_text, amount_text)) = normalized.split_once("-=") {
        let (target, dual_output) = parse_control_assignment_target(target_text.trim())?;
        if dual_output.is_some() {
            return None;
        }

        return Some(ControlActionRecord::CompoundUpdate {
            target,
            operation: ControlCompoundUpdateOperation::Decrement,
            amount: Some(parse_control_expression(
                amount_text.trim(),
                known_signal_names,
                known_symbol_names,
            )?),
        });
    }

    let (target_text, assignment_kind, value_text) = split_explicit_assignment(normalized)?;
    let (target, dual_output) = parse_control_assignment_target(target_text)?;
    Some(ControlActionRecord::Assign {
        target,
        assignment_kind,
        dual_output,
        value: parse_control_expression(value_text, known_signal_names, known_symbol_names)?,
    })
}

fn split_explicit_assignment(text: &str) -> Option<(&str, DecisionTreeAssignmentKind, &str)> {
    if let Some((left, right)) = text.split_once("<-") {
        return Some((
            left.trim(),
            DecisionTreeAssignmentKind::Sequential,
            right.trim(),
        ));
    }

    for (index, character) in text.char_indices() {
        if character != '=' {
            continue;
        }

        let prefix = text[..index].chars().next_back();
        let suffix = text[index + character.len_utf8()..].chars().next();
        if matches!(prefix, Some('!' | '<' | '>' | '=')) || matches!(suffix, Some('=')) {
            continue;
        }

        return Some((
            text[..index].trim(),
            DecisionTreeAssignmentKind::Combinational,
            text[index + character.len_utf8()..].trim(),
        ));
    }

    None
}

fn parse_control_assignment_target(
    text: &str,
) -> Option<(ControlAssignmentTargetRecord, Option<ControlDualOutputKind>)> {
    let mut exposed_public_output = false;
    let mut dual_output = None::<ControlDualOutputKind>;
    let mut signal_name = None::<String>;

    for token in text.split_whitespace() {
        if token.eq_ignore_ascii_case("public") {
            exposed_public_output = true;
            continue;
        }
        if token.eq_ignore_ascii_case("next") {
            if dual_output.is_some() {
                return None;
            }
            dual_output = Some(ControlDualOutputKind::NextSignal);
            continue;
        }
        if token.eq_ignore_ascii_case("registered") || token.eq_ignore_ascii_case("reg") {
            if dual_output.is_some() {
                return None;
            }
            dual_output = Some(ControlDualOutputKind::RegisteredSignal);
            continue;
        }
        if signal_name.is_some() {
            return None;
        }
        signal_name = Some(parse_identifier(token)?);
    }

    Some((
        ControlAssignmentTargetRecord {
            signal_name: signal_name?,
            exposed_public_output,
        },
        dual_output,
    ))
}

fn parse_explicit_decision_tree_guard(text: &str) -> Option<DecisionTreeGuardRecord> {
    if let Some((left_signal, right_text)) = text.split_once("==") {
        return Some(DecisionTreeGuardRecord::Comparison {
            left_signal: parse_identifier(left_signal.trim())?,
            operator: DecisionTreeComparisonOperator::Eq,
            right: parse_decision_tree_value(right_text.trim())?,
        });
    }
    if let Some((left_signal, right_text)) = text.split_once("!=") {
        return Some(DecisionTreeGuardRecord::Comparison {
            left_signal: parse_identifier(left_signal.trim())?,
            operator: DecisionTreeComparisonOperator::NotEq,
            right: parse_decision_tree_value(right_text.trim())?,
        });
    }

    Some(DecisionTreeGuardRecord::SignalIsHigh {
        signal_name: parse_identifier(text.trim())?,
    })
}

fn parse_explicit_decision_tree_action(text: &str) -> Option<DecisionTreeActionRecord> {
    if let Some((target_signal, value_text)) = text.split_once("<-") {
        return Some(DecisionTreeActionRecord::Assign {
            target_signal: parse_identifier(target_signal.trim())?,
            assignment_kind: DecisionTreeAssignmentKind::Sequential,
            value: parse_decision_tree_value(value_text.trim())?,
        });
    }
    if let Some((target_signal, value_text)) = text.split_once('=') {
        return Some(DecisionTreeActionRecord::Assign {
            target_signal: parse_identifier(target_signal.trim())?,
            assignment_kind: DecisionTreeAssignmentKind::Combinational,
            value: parse_decision_tree_value(value_text.trim())?,
        });
    }

    None
}

fn parse_decision_tree_value(text: &str) -> Option<DecisionTreeValueRecord> {
    let trimmed = text.trim().trim_end_matches('.');
    if trimmed.is_empty() {
        return None;
    }

    if let Some(signal_name) = parse_identifier(trimmed) {
        return Some(DecisionTreeValueRecord::SignalRef { signal_name });
    }

    Some(DecisionTreeValueRecord::Literal {
        literal: trimmed.to_string(),
    })
}

fn parse_control_expression(
    text: &str,
    known_signal_names: &BTreeSet<String>,
    known_symbol_names: &BTreeSet<String>,
) -> Option<ControlExpressionRecord> {
    let tokens = tokenize_control_expression(text)?;
    let mut parser = ControlExpressionParser::new(tokens, known_signal_names, known_symbol_names);
    parser.parse()
}

fn tokenize_control_expression(text: &str) -> Option<Vec<String>> {
    let trimmed = text.trim().trim_end_matches('.');
    if trimmed.is_empty() {
        return None;
    }

    let characters = trimmed.chars().collect::<Vec<_>>();
    let mut tokens = Vec::new();
    let mut index = 0usize;
    while index < characters.len() {
        let character = characters[index];
        if character.is_whitespace() {
            index += 1;
            continue;
        }

        if index + 1 < characters.len() {
            let pair = [character, characters[index + 1]];
            if matches!(pair, ['=', '='] | ['!', '='] | ['<', '='] | ['>', '=']) {
                tokens.push(pair.iter().collect());
                index += 2;
                continue;
            }
        }

        if matches!(
            character,
            '(' | ')'
                | '['
                | ']'
                | ':'
                | '.'
                | '@'
                | '+'
                | '-'
                | '*'
                | '/'
                | '%'
                | '&'
                | '|'
                | '^'
                | '!'
                | '<'
                | '>'
        ) {
            tokens.push(character.to_string());
            index += 1;
            continue;
        }

        if character.is_ascii_alphanumeric() || character == '_' || character == '\'' {
            let start = index;
            index += 1;
            while index < characters.len()
                && (characters[index].is_ascii_alphanumeric()
                    || characters[index] == '_'
                    || characters[index] == '\'')
            {
                index += 1;
            }
            tokens.push(trimmed[start..index].to_string());
            continue;
        }

        return None;
    }

    Some(tokens)
}

struct ControlExpressionParser<'a> {
    tokens: Vec<String>,
    index: usize,
    known_signal_names: &'a BTreeSet<String>,
    known_symbol_names: &'a BTreeSet<String>,
}

impl<'a> ControlExpressionParser<'a> {
    fn new(
        tokens: Vec<String>,
        known_signal_names: &'a BTreeSet<String>,
        known_symbol_names: &'a BTreeSet<String>,
    ) -> Self {
        Self {
            tokens,
            index: 0,
            known_signal_names,
            known_symbol_names,
        }
    }

    fn parse(&mut self) -> Option<ControlExpressionRecord> {
        let expression = self.parse_comparison()?;
        (self.index == self.tokens.len()).then_some(expression)
    }

    fn parse_comparison(&mut self) -> Option<ControlExpressionRecord> {
        let mut expression = self.parse_bit_or()?;
        loop {
            let operator = match self.peek() {
                Some("==") => ControlBinaryOperator::Eq,
                Some("!=") => ControlBinaryOperator::NotEq,
                Some("<") => ControlBinaryOperator::Lt,
                Some("<=") => ControlBinaryOperator::Le,
                Some(">") => ControlBinaryOperator::Gt,
                Some(">=") => ControlBinaryOperator::Ge,
                _ => break,
            };
            self.index += 1;
            let right = self.parse_bit_or()?;
            expression = ControlExpressionRecord::Binary {
                operator,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }
        Some(expression)
    }

    fn parse_bit_or(&mut self) -> Option<ControlExpressionRecord> {
        self.parse_left_associative(Self::parse_bit_xor, &[("|", ControlBinaryOperator::BitOr)])
    }

    fn parse_bit_xor(&mut self) -> Option<ControlExpressionRecord> {
        self.parse_left_associative(Self::parse_bit_and, &[("^", ControlBinaryOperator::BitXor)])
    }

    fn parse_bit_and(&mut self) -> Option<ControlExpressionRecord> {
        self.parse_left_associative(Self::parse_add_sub, &[("&", ControlBinaryOperator::BitAnd)])
    }

    fn parse_add_sub(&mut self) -> Option<ControlExpressionRecord> {
        self.parse_left_associative(
            Self::parse_mul_div_mod,
            &[
                ("+", ControlBinaryOperator::Add),
                ("-", ControlBinaryOperator::Sub),
            ],
        )
    }

    fn parse_mul_div_mod(&mut self) -> Option<ControlExpressionRecord> {
        self.parse_left_associative(
            Self::parse_unary,
            &[
                ("*", ControlBinaryOperator::Mul),
                ("/", ControlBinaryOperator::Div),
                ("%", ControlBinaryOperator::Mod),
            ],
        )
    }

    fn parse_left_associative(
        &mut self,
        next_parser: fn(&mut Self) -> Option<ControlExpressionRecord>,
        operators: &[(&str, ControlBinaryOperator)],
    ) -> Option<ControlExpressionRecord> {
        let mut expression = next_parser(self)?;
        loop {
            let Some((_, operator)) = operators
                .iter()
                .find(|(token, _)| self.peek().is_some_and(|next| next == *token))
            else {
                break;
            };
            self.index += 1;
            let right = next_parser(self)?;
            expression = ControlExpressionRecord::Binary {
                operator: *operator,
                left: Box::new(expression),
                right: Box::new(right),
            };
        }
        Some(expression)
    }

    fn parse_unary(&mut self) -> Option<ControlExpressionRecord> {
        if self.consume("!") {
            return Some(ControlExpressionRecord::Unary {
                operator: ControlUnaryOperator::Not,
                operand: Box::new(self.parse_unary()?),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Option<ControlExpressionRecord> {
        if self.consume("(") {
            let expression = self.parse_comparison()?;
            self.expect(")")?;
            return Some(expression);
        }

        let token = self.next_owned()?;
        if token.eq_ignore_ascii_case("true")
            || token.eq_ignore_ascii_case("false")
            || token
                .chars()
                .next()
                .is_some_and(|character| character.is_ascii_digit())
        {
            return Some(ControlExpressionRecord::Literal { literal: token });
        }

        let base_name = parse_identifier(&token)?;
        let mut suffixes = Vec::new();
        loop {
            if self.consume(".") {
                suffixes.push(ControlReferenceSuffix::Member {
                    member_name: parse_identifier(&self.next_owned()?)?,
                });
                continue;
            }
            if self.consume("[") {
                let first = self.next_owned()?;
                if self.consume(":") {
                    let second = self.next_owned()?;
                    self.expect("]")?;
                    suffixes.push(ControlReferenceSuffix::Slice {
                        msb: parse_u32_token(&first)?,
                        lsb: parse_u32_token(&second)?,
                    });
                } else {
                    self.expect("]")?;
                    suffixes.push(ControlReferenceSuffix::BitIndex {
                        index: parse_u32_token(&first)?,
                    });
                }
                continue;
            }
            if self.consume("@") {
                // WidthCast uses a literal numeric width (e.g. signal@32),
                // not a parametric expression — use parse_u32_token here.
                suffixes.push(ControlReferenceSuffix::WidthCast {
                    width: parse_u32_token(&self.next_owned()?)?,
                });
                continue;
            }
            break;
        }

        Some(ControlExpressionRecord::Reference {
            reference: ControlReferenceRecord {
                base_name: base_name.clone(),
                kind_hint: classify_control_reference_kind(
                    &base_name,
                    self.known_signal_names,
                    self.known_symbol_names,
                ),
                suffixes,
                exposed_public_output: false,
            },
        })
    }

    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.index).map(String::as_str)
    }

    fn consume(&mut self, token: &str) -> bool {
        if self.peek().is_some_and(|next| next == token) {
            self.index += 1;
            true
        } else {
            false
        }
    }

    fn expect(&mut self, token: &str) -> Option<()> {
        self.consume(token).then_some(())
    }

    fn next_owned(&mut self) -> Option<String> {
        let token = self.tokens.get(self.index)?.clone();
        self.index += 1;
        Some(token)
    }
}

fn classify_control_reference_kind(
    base_name: &str,
    known_signal_names: &BTreeSet<String>,
    known_symbol_names: &BTreeSet<String>,
) -> ControlReferenceKind {
    if known_symbol_names.contains(base_name) {
        return ControlReferenceKind::Symbol;
    }
    if known_signal_names.contains(base_name) {
        return ControlReferenceKind::Signal;
    }

    ControlReferenceKind::Unknown
}

fn reclassify_control_expression(
    expression: &ControlExpressionRecord,
    known_signal_names: &BTreeSet<String>,
    known_symbol_names: &BTreeSet<String>,
) -> ControlExpressionRecord {
    match expression {
        ControlExpressionRecord::Reference { reference } => ControlExpressionRecord::Reference {
            reference: ControlReferenceRecord {
                base_name: reference.base_name.clone(),
                kind_hint: classify_control_reference_kind(
                    &reference.base_name,
                    known_signal_names,
                    known_symbol_names,
                ),
                suffixes: reference.suffixes.clone(),
                exposed_public_output: reference.exposed_public_output,
            },
        },
        ControlExpressionRecord::Literal { literal } => ControlExpressionRecord::Literal {
            literal: literal.clone(),
        },
        ControlExpressionRecord::Unary { operator, operand } => ControlExpressionRecord::Unary {
            operator: *operator,
            operand: Box::new(reclassify_control_expression(
                operand,
                known_signal_names,
                known_symbol_names,
            )),
        },
        ControlExpressionRecord::Binary {
            operator,
            left,
            right,
        } => ControlExpressionRecord::Binary {
            operator: *operator,
            left: Box::new(reclassify_control_expression(
                left,
                known_signal_names,
                known_symbol_names,
            )),
            right: Box::new(reclassify_control_expression(
                right,
                known_signal_names,
                known_symbol_names,
            )),
        },
    }
}

fn parse_u32_token(token: &str) -> Option<u32> {
    token
        .trim()
        .trim_end_matches('.')
        .trim_end_matches(',')
        .parse::<u32>()
        .ok()
}

fn parse_identifier(token: &str) -> Option<String> {
    let trimmed = token
        .trim()
        .trim_end_matches('.')
        .trim_end_matches(':')
        .trim_end_matches(',');
    let mut chars = trimmed.chars();
    let first = chars.next()?;
    if !(first.is_ascii_alphabetic() || first == '_') {
        return None;
    }
    if !chars.all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
        return None;
    }

    Some(trimmed.to_string())
}

fn parse_interface_signal_direction(token: &str) -> Option<InterfaceSignalDirection> {
    if token.eq_ignore_ascii_case("input") {
        return Some(InterfaceSignalDirection::Input);
    }
    if token.eq_ignore_ascii_case("output") {
        return Some(InterfaceSignalDirection::Output);
    }
    if token.eq_ignore_ascii_case("internal") || token.eq_ignore_ascii_case("local") {
        return Some(InterfaceSignalDirection::Internal);
    }

    None
}

fn parse_optional_width_hint(tokens: &[&str], index: &mut usize) -> Option<WidthHint> {
    let Some(token) = tokens.get(*index).copied() else {
        return None;
    };

    if token.eq_ignore_ascii_case("width") {
        let width = parse_width_token(*tokens.get(*index + 1)?)?;
        *index += 2;
        return Some(width);
    }

    let width = parse_width_token(token)?;
    *index += 1;
    Some(width)
}

/// Parse a single width token into a `WidthHint`.
/// Handles:
/// - Numeric: `"32"`, `"1"`, `"4-bit"`, `"8-bits"` → `Numeric(n)`
/// - Parametric: `"ADDR_WIDTH"`, `"DATA_WIDTH/8"` → `Parametric(expr)`
fn parse_width_token(token: &str) -> Option<WidthHint> {
    let trimmed = token
        .trim()
        .trim_end_matches('.')
        .trim_end_matches(',')
        .trim_end_matches(':');
    let trimmed = trimmed
        .strip_suffix("-bit")
        .or_else(|| trimmed.strip_suffix("-bits"))
        .unwrap_or(trimmed);

    // Try numeric first
    if let Ok(n) = trimmed.parse::<u32>() {
        return (n > 0).then_some(WidthHint::Numeric(n));
    }

    // Non-numeric but contains alphabetic chars → parametric expression
    if !trimmed.is_empty() && trimmed.chars().any(|c| c.is_ascii_alphabetic()) {
        return Some(WidthHint::Parametric(trimmed.to_string()));
    }

    None
}

fn known_explicit_signal_names(context: &SemanticContext) -> BTreeSet<String> {
    let mut signal_names = BTreeSet::new();

    for statement in &context.statements {
        if let Some(signal_declaration) = parse_explicit_signal_declaration(&statement.text) {
            signal_names.insert(signal_declaration.signal_name);
        }
        if let Some(clock_signal) = parse_explicit_system_clock(&statement.text) {
            signal_names.insert(clock_signal);
        }
        if let Some(reset_declaration) = parse_explicit_system_reset(&statement.text) {
            signal_names.insert(reset_declaration.signal_name);
        }
        if let Some(init_assignment) = parse_explicit_init_assignment(&statement.text) {
            signal_names.insert(init_assignment.target_signal);
            if let DecisionTreeValueRecord::SignalRef { signal_name } = init_assignment.value {
                signal_names.insert(signal_name);
            }
        }
        if let Some(transition) = parse_explicit_state_transition(&statement.text) {
            if let Some(guard) = transition.guard.as_ref() {
                signal_names.extend(referenced_signal_names_for_guard(guard));
            }
        }
        if let Some(fragment) = parse_explicit_decision_tree_fragment(&statement.text) {
            signal_names.extend(fragment.referenced_signal_names);
        }
    }

    signal_names
}

fn normalize_decision_tree_block_name(raw_name: &str) -> Option<String> {
    let normalized = document_key(raw_name.trim());
    (!normalized.is_empty()).then_some(normalized)
}

fn explicit_interface_key(section_ids: &[String]) -> String {
    if section_ids.is_empty() {
        return "explicit_document_interface".to_string();
    }

    format!("explicit_interface__{}", section_ids.join("__"))
}

fn register_interface_signal_record(
    accumulator: &mut InterfaceAccumulator,
    signal_name: &str,
    direction_hint: Option<InterfaceSignalDirection>,
    width_hint: Option<WidthHint>,
    supporting_statement_id: &str,
    automation_confidence: AutomationConfidence,
) {
    accumulator.signals.insert(signal_name.to_string());
    accumulator
        .supporting_statement_ids
        .insert(supporting_statement_id.to_string());
    let entry = accumulator
        .signal_records
        .entry(signal_name.to_string())
        .or_insert_with(|| InterfaceSignalAccumulator {
            direction_hint: None,
            width_hint: None,
            supporting_statement_ids: BTreeSet::new(),
            automation_confidence,
        });
    merge_signal_hint(&mut entry.direction_hint, direction_hint);
    merge_signal_hint(&mut entry.width_hint, width_hint);
    entry
        .supporting_statement_ids
        .insert(supporting_statement_id.to_string());
    entry.automation_confidence =
        max_automation_confidence(entry.automation_confidence, automation_confidence);
}

fn merge_signal_hint<T: Clone + Eq>(target: &mut Option<T>, incoming: Option<T>) {
    match (target.as_ref(), incoming.as_ref()) {
        (None, Some(value)) => *target = Some(value.clone()),
        (Some(existing), Some(value)) if existing != value => *target = None,
        _ => {}
    }
}

fn merge_named_hint(target: &mut Option<String>, incoming: &str) -> bool {
    match target {
        None => {
            *target = Some(incoming.to_string());
            true
        }
        Some(existing) => existing == incoming,
    }
}

fn merge_copy_hint<T: Copy + Eq>(target: &mut Option<T>, incoming: T) -> bool {
    match *target {
        None => {
            *target = Some(incoming);
            true
        }
        Some(existing) => existing == incoming,
    }
}

fn max_automation_confidence(
    left: AutomationConfidence,
    right: AutomationConfidence,
) -> AutomationConfidence {
    if automation_confidence_rank(left) >= automation_confidence_rank(right) {
        left
    } else {
        right
    }
}

fn automation_confidence_rank(confidence: AutomationConfidence) -> u8 {
    match confidence {
        AutomationConfidence::High => 3,
        AutomationConfidence::Medium => 2,
        AutomationConfidence::Low => 1,
    }
}

fn decision_tree_fragment_key(block_name: &str, guard: Option<&DecisionTreeGuardRecord>) -> String {
    format!("{block_name}::{}", guard_key(guard))
}

fn control_block_key(
    block_name: &str,
    role: ControlBlockRole,
    selector: Option<&ControlExpressionRecord>,
) -> String {
    format!(
        "{block_name}::{}::{}",
        control_block_role_key(role),
        control_expression_key(selector)
    )
}

fn control_block_role_key(role: ControlBlockRole) -> &'static str {
    match role {
        ControlBlockRole::StateBody => "state_body",
        ControlBlockRole::ResetSynchronous => "reset_synchronous",
        ControlBlockRole::ResetAsynchronous => "reset_asynchronous",
        ControlBlockRole::StandaloneDecisionTree => "standalone_decision_tree",
    }
}

fn control_expression_key(expression: Option<&ControlExpressionRecord>) -> String {
    match expression {
        None => "none".to_string(),
        Some(expression) => control_expression_record_key(expression),
    }
}

fn control_expression_record_key(expression: &ControlExpressionRecord) -> String {
    match expression {
        ControlExpressionRecord::Reference { reference } => format!(
            "ref:{}:{}:{}",
            control_reference_kind_key(reference.kind_hint),
            reference.base_name,
            reference
                .suffixes
                .iter()
                .map(control_reference_suffix_key)
                .collect::<Vec<_>>()
                .join("|")
        ),
        ControlExpressionRecord::Literal { literal } => format!("lit:{literal}"),
        ControlExpressionRecord::Unary { operator, operand } => format!(
            "unary:{}:{}",
            control_unary_operator_key(*operator),
            control_expression_record_key(operand)
        ),
        ControlExpressionRecord::Binary {
            operator,
            left,
            right,
        } => format!(
            "bin:{}:{}:{}",
            control_binary_operator_key(*operator),
            control_expression_record_key(left),
            control_expression_record_key(right)
        ),
    }
}

fn control_reference_kind_key(kind: ControlReferenceKind) -> &'static str {
    match kind {
        ControlReferenceKind::Unknown => "unknown",
        ControlReferenceKind::Signal => "signal",
        ControlReferenceKind::Symbol => "symbol",
    }
}

fn control_reference_suffix_key(suffix: &ControlReferenceSuffix) -> String {
    match suffix {
        ControlReferenceSuffix::Member { member_name } => format!("member:{member_name}"),
        ControlReferenceSuffix::BitIndex { index } => format!("bit:{index}"),
        ControlReferenceSuffix::Slice { msb, lsb } => format!("slice:{msb}:{lsb}"),
        ControlReferenceSuffix::WidthCast { width } => format!("width:{width}"),
    }
}

fn control_unary_operator_key(operator: ControlUnaryOperator) -> &'static str {
    match operator {
        ControlUnaryOperator::Not => "not",
    }
}

fn control_binary_operator_key(operator: ControlBinaryOperator) -> &'static str {
    match operator {
        ControlBinaryOperator::Add => "add",
        ControlBinaryOperator::Sub => "sub",
        ControlBinaryOperator::Mul => "mul",
        ControlBinaryOperator::Div => "div",
        ControlBinaryOperator::Mod => "mod",
        ControlBinaryOperator::BitAnd => "bit_and",
        ControlBinaryOperator::BitOr => "bit_or",
        ControlBinaryOperator::BitXor => "bit_xor",
        ControlBinaryOperator::Eq => "eq",
        ControlBinaryOperator::NotEq => "not_eq",
        ControlBinaryOperator::Lt => "lt",
        ControlBinaryOperator::Le => "le",
        ControlBinaryOperator::Gt => "gt",
        ControlBinaryOperator::Ge => "ge",
    }
}

fn explicit_top_link_endpoint_key(endpoint: &ExplicitTopLinkEndpoint) -> String {
    match endpoint.instance_name.as_deref() {
        Some(instance_name) => format!("{instance_name}.{}", endpoint.signal_name),
        None => endpoint.signal_name.clone(),
    }
}

fn guard_key(guard: Option<&DecisionTreeGuardRecord>) -> String {
    match guard {
        None => "unguarded".to_string(),
        Some(DecisionTreeGuardRecord::SignalIsHigh { signal_name }) => {
            format!("signal_high:{signal_name}")
        }
        Some(DecisionTreeGuardRecord::Comparison {
            left_signal,
            operator,
            right,
        }) => format!(
            "comparison:{}:{}:{}",
            left_signal,
            decision_tree_comparison_operator_key(*operator),
            decision_tree_value_key(right)
        ),
    }
}

fn decision_tree_comparison_operator_key(operator: DecisionTreeComparisonOperator) -> &'static str {
    match operator {
        DecisionTreeComparisonOperator::Eq => "eq",
        DecisionTreeComparisonOperator::NotEq => "not_eq",
    }
}

fn decision_tree_value_key(value: &DecisionTreeValueRecord) -> String {
    match value {
        DecisionTreeValueRecord::SignalRef { signal_name } => format!("signal:{signal_name}"),
        DecisionTreeValueRecord::Literal { literal } => format!("literal:{literal}"),
    }
}

fn referenced_signal_names_for_guard(guard: &DecisionTreeGuardRecord) -> BTreeSet<String> {
    let mut signal_names = BTreeSet::new();
    match guard {
        DecisionTreeGuardRecord::SignalIsHigh { signal_name } => {
            signal_names.insert(signal_name.clone());
        }
        DecisionTreeGuardRecord::Comparison {
            left_signal, right, ..
        } => {
            signal_names.insert(left_signal.clone());
            if let DecisionTreeValueRecord::SignalRef { signal_name } = right {
                signal_names.insert(signal_name.clone());
            }
        }
    }
    signal_names
}

fn referenced_signal_names_for_action(action: &DecisionTreeActionRecord) -> BTreeSet<String> {
    let mut signal_names = BTreeSet::new();
    match action {
        DecisionTreeActionRecord::Assign {
            target_signal,
            value,
            ..
        } => {
            signal_names.insert(target_signal.clone());
            if let DecisionTreeValueRecord::SignalRef { signal_name } = value {
                signal_names.insert(signal_name.clone());
            }
        }
    }
    signal_names
}

fn referenced_signal_names_for_control_expression(
    expression: &ControlExpressionRecord,
) -> BTreeSet<String> {
    match expression {
        ControlExpressionRecord::Reference { reference } => {
            if matches!(reference.kind_hint, ControlReferenceKind::Symbol) {
                return BTreeSet::new();
            }

            BTreeSet::from([reference.base_name.clone()])
        }
        ControlExpressionRecord::Literal { .. } => BTreeSet::new(),
        ControlExpressionRecord::Unary { operand, .. } => {
            referenced_signal_names_for_control_expression(operand)
        }
        ControlExpressionRecord::Binary { left, right, .. } => {
            let mut signal_names = referenced_signal_names_for_control_expression(left);
            signal_names.extend(referenced_signal_names_for_control_expression(right));
            signal_names
        }
    }
}

fn referenced_signal_names_for_control_action(action: &ControlActionRecord) -> BTreeSet<String> {
    let mut signal_names = BTreeSet::new();
    match action {
        ControlActionRecord::Assign { target, value, .. } => {
            signal_names.insert(target.signal_name.clone());
            signal_names.extend(referenced_signal_names_for_control_expression(value));
        }
        ControlActionRecord::Transition { .. } => {}
        ControlActionRecord::DelayedPulse { target, value, .. } => {
            signal_names.insert(target.signal_name.clone());
            signal_names.extend(referenced_signal_names_for_control_expression(value));
        }
        ControlActionRecord::CompoundUpdate { target, amount, .. } => {
            signal_names.insert(target.signal_name.clone());
            if let Some(amount) = amount.as_ref() {
                signal_names.extend(referenced_signal_names_for_control_expression(amount));
            }
        }
    }
    signal_names
}

fn extract_signal_tokens(text: &str) -> Vec<String> {
    let mut signals = BTreeSet::new();
    let mut current = String::new();

    for character in text.chars() {
        if character.is_ascii_alphanumeric() || character == '_' {
            current.push(character);
        } else {
            maybe_add_signal_token(&mut signals, &current);
            current.clear();
        }
    }
    maybe_add_signal_token(&mut signals, &current);

    signals.into_iter().collect()
}

fn maybe_add_signal_token(signals: &mut BTreeSet<String>, token: &str) {
    if !looks_like_signal_token(token) {
        return;
    }

    let upper_token = token.to_ascii_uppercase();
    if signal_stop_words().contains(&upper_token.as_str()) {
        return;
    }

    signals.insert(upper_token);
}

fn looks_like_signal_token(token: &str) -> bool {
    if token.len() < 2 {
        return false;
    }

    let has_alpha = token
        .chars()
        .any(|character| character.is_ascii_alphabetic());
    if !has_alpha {
        return false;
    }

    let is_upper = token.chars().all(|character| {
        character.is_ascii_uppercase() || character.is_ascii_digit() || character == '_'
    });
    if is_upper
        && token
            .chars()
            .any(|character| character.is_ascii_uppercase())
    {
        return true;
    }

    let lowered_token = token.to_ascii_lowercase();
    lowered_token.ends_with("_n") || lowered_token.ends_with("_b")
}

fn signal_stop_words() -> BTreeSet<&'static str> {
    [
        // --- Tool-internal terms ---
        "IR",
        "PDF",
        "JSON",
        "CLI",
        "README",
        "DOCS",
        "LLM",
        "RTL",
        "FSM",
        "VHDL",
        "DOCLING",
        "SOURCEIR",
        "EVIDENCEIR",
        "SEMANTICIR",
        "INTENTIR",
        "API",
        "URL",
        // --- Common English function words appearing in ALL CAPS in technical specs ---
        "A",
        "AN",
        "THE",
        "AND",
        "OR",
        "NOT",
        "NOR",
        "BUT",
        "YET",
        "SO",
        "IF",
        "AS",
        "AT",
        "BY",
        "IN",
        "ON",
        "OF",
        "TO",
        "UP",
        "OUT",
        "IS",
        "ARE",
        "WAS",
        "WERE",
        "BE",
        "BEEN",
        "BEING",
        "DO",
        "DOES",
        "DID",
        "HAS",
        "HAVE",
        "HAD",
        "CAN",
        "MAY",
        "MUST",
        "SHALL",
        "WILL",
        "WOULD",
        "SHOULD",
        "COULD",
        "NO",
        "YES",
        "OK",
        "THIS",
        "THAT",
        "THESE",
        "THOSE",
        "WITH",
        "FROM",
        "INTO",
        "ONTO",
        "UPON",
        "OVER",
        "UNDER",
        "ABOUT",
        "ABOVE",
        "BELOW",
        "BEFORE",
        "AFTER",
        "DURING",
        "SINCE",
        "UNTIL",
        "WHEN",
        "WHERE",
        "WHO",
        "WHAT",
        "HOW",
        "WHY",
        "WHICH",
        "EACH",
        "BOTH",
        "ALL",
        "ANY",
        "SOME",
        "NONE",
        "MORE",
        "LESS",
        "SUCH",
        "SAME",
        "ONLY",
        "ALSO",
        "EVEN",
        "JUST",
        "THEN",
        "THAN",
        "FOR",
        "EITHER",
        "NEITHER",
        "HOWEVER",
        "THEREFORE",
        "THUS",
        "HENCE",
        "ONE",
        "TWO",
        "THREE",
        "FOUR",
        "FIVE",
        "SIX",
        "ONCE",
        "TWICE",
        "II",
        "III",
        "IV",
        "VI",
        "VII",
        "VIII",
        "USE",
        "USED",
        "USING",
        "USER",
        "USERS",
        "NEW",
        "OLD",
        "SAME",
        "NEXT",
        "LAST",
        "FIRST",
        "PRIOR",
        // --- Common English words that appear ALL CAPS in formal/technical documents ---
        // Note: do NOT add words that are legitimate hardware signal names such as VALID, READY,
        // ACTIVE, IDLE, DONE, FULL, EMPTY, READ, WRITE, BUSY, GRANT, REQ — those ARE real signals.
        "HIGH",
        "LOW",
        "TRUE",
        "FALSE",
        "NULL",
        "VOID",
        "OPTIONAL",
        "MANDATORY",
        "RECOMMENDED",
        "PROHIBITED",
        "RESERVED",
        "IMPLEMENTATION",
        "DEFINED",
        "DEFAULT",
        "NOTE",
        "NOTES",
        "WARNING",
        "CAUTION",
        "IMPORTANT",
        // --- Legal and contractual vocabulary (common in chip spec front matter) ---
        "LICENSE",
        "LICENCE",
        "LICENSEE",
        "LICENSOR",
        "LICENSED",
        "LICENSES",
        "COPYRIGHT",
        "COPYRIGHTED",
        "COPYRIGHTS",
        "AGREEMENT",
        "AGREED",
        "AGREES",
        "DISCLAIMER",
        "DISCLAIMED",
        "WARRANTY",
        "WARRANTIES",
        "WARRANTED",
        "PATENT",
        "PATENTS",
        "PATENTED",
        "CLAIM",
        "CLAIMS",
        "CLAIMED",
        "LIABILITY",
        "LIABILITIES",
        "LIABLE",
        "INDEMNIFY",
        "INDEMNIFIED",
        "INDEMNIFICATION",
        "TERMINATE",
        "TERMINATION",
        "TERMINATED",
        "SUBLICENSE",
        "SUBLICENSED",
        "ROYALTY",
        "ROYALTIES",
        "TRADEMARK",
        "TRADEMARKS",
        "CONFIDENTIAL",
        "NON",
        "PROPRIETARY",
        "INTELLECTUAL",
        "PROPERTY",
        "RIGHTS",
        "RIGHTSHOLDER",
        "EXPRESS",
        "IMPLIED",
        "STATUTORY",
        "LIMITATION",
        "LIMITED",
        "UNLIMITED",
        "NOTWITHSTANDING",
        "REGARDLESS",
        "IRRESPECTIVE",
        "PROVISION",
        "PROVISIONS",
        "CLAUSE",
        "ARTICLE",
        "SUBSECTION",
        "CONTRACT",
        "TERMS",
        "CONDITIONS",
        "CONDITION",
        "ACCEPT",
        "ACCEPTANCE",
        "ACCOMPANYING",
        "AGREE",
        "BOUND",
        "CLICKING",
        "COPYING",
        "END",
        "ENTITY",
        "INCLUDING",
        "INDICATE",
        "INDIVIDUAL",
        "LEGAL",
        "OBLIGATION",
        "OBLIGATIONS",
        "OTHERWISE",
        "RELEVANT",
        "SINGLE",
        "SPECIFICATION",
        "WITHOUT",
        "YOU",
        "YOUR",
        "CLICKING",
        "EXCEPT",
        "SUBJECT",
        "TORT",
        "LAW",
        "LAWS",
        "CONSEQUENTIAL",
        "INCIDENTAL",
        "PUNITIVE",
        "INDIRECT",
        "DIRECT",
        "SPECIAL",
        "EXEMPLARY",
        "AGGREGATE",
        "MAXIMUM",
        "DAMAGES",
        "DAMAGE",
        "LOSS",
        "LOSSES",
        "ARISING",
        "CAUSED",
        "THEORY",
        "POSSIBILITY",
        "ADVISED",
        "EXTENT",
        "EVENT",
        "DOCUMENT",
        "FULLEST",
        "PETMITTED",
        "RELEASES",
        "DEMANDS",
        "CONTAIN",
        "CONTAINED",
        "CREATED",
        "EXCEED",
        "EXCESS",
        "ENLARGE",
        "EXTEND",
        "EXISTENCE",
        "FEES",
        "MADE",
        "MATTER",
        "OBLIGATIONS",
        "PAID",
        "PRODUCT",
        "SUIT",
        "TECHNOLOGY",
        "UNDER",
        "CONTRARY",
        "CONNECT",
        "CONNECTION",
        "WAIVER",
        // --- AMBA/ARM protocol family names (not hardware signal names) ---
        "AMBA",
        "AHB",
        "AHB5",
        "APB",
        "APB3",
        "APB4",
        "AXI",
        "AXI4",
        "AXI5",
        "ACE",
        "ACE5",
        "CHI",
        "ATB",
        "DTI",
        "LTI",
        "CXS",
        "GFB",
        "LPI",
        "ASB",
        "ASH",
        "ACP",
        // --- Company, organization, and standard body names ---
        "ARM",
        "AMD",
        "INTEL",
        "NVIDIA",
        "QUALCOMM",
        "SAMSUNG",
        "TSMC",
        "SIFIVE",
        "RISC",
        "MIPS",
        "SYNOPSYS",
        "CADENCE",
        "MENTOR",
        "SIEMENS",
        "XILINX",
        "ALTERA",
        "LATTICE",
        "MICROCHIP",
        "IEEE",
        "JEDEC",
        "IETF",
        "ISO",
        "IEC",
        "ANSI",
        "NIST",
        // --- Document structure and publication metadata ---
        "CHAPTER",
        "SECTION",
        "TABLE",
        "FIGURE",
        "APPENDIX",
        "ANNEX",
        "SCHEDULE",
        "EXHIBIT",
        "EXAMPLE",
        "REFERENCE",
        "REFERENCES",
        "REVISION",
        "VERSION",
        "RELEASE",
        "ISSUE",
        "HISTORY",
        "OVERVIEW",
        "INTRODUCTION",
        "SUMMARY",
        "ABSTRACT",
        "PREFACE",
        "GLOSSARY",
        "ACRONYM",
        "ABBREVIATION",
        "DEFINITION",
        "DESCRIPTION",
        // --- Publication ID prefixes common in ARM/AMBA specifications ---
        "IHI",
        "DDI",
        "DEN",
        "DVI",
        "AEI",
        // --- Timing diagram cycle/slot labels (T0–T9 are clock cycle markers, not signal names) ---
        "T0",
        "T1",
        "T2",
        "T3",
        "T4",
        "T5",
        "T6",
        "T7",
        "T8",
        "T9",
        // --- Bit-position descriptors (Least/Most Significant; these describe field positions) ---
        "LS",
        "MS",
        "LSB",
        "MSB",
        // --- AMBA AHB HTRANS encoding values (these are register-field values, not signal names) ---
        "NONSEQ",
        "NONSEQUENTIAL",
        // --- AMBA AHB HBURST encoding values (burst type names, not signal names) ---
        "INCR4",
        "INCR8",
        "INCR16",
        "WRAP4",
        "WRAP8",
        "WRAP16",
        // --- Interface/connection type names that appear as ALL CAPS context words ---
        "OC",
        // --- Memory technology type names (DRAM, SRAM etc. are memory arrays, not port signals) ---
        "DRAM",
        "SRAM",
        // --- Timing diagram notation words (appear in legend explanations, not signal names) ---
        "CAPITALS",
        "SMALL",
        // --- Common technology abbreviations that are never hardware signal names ---
        "CPU",
        "GPU",
        "DMA",
        "ROM",
        "RAM",
        "BIOS",
        "USB",
        "UART",
        "FIFO",
        "LIFO",
        "CRC",
        "ECC",
        "SOC",
        "NOC",
        "NIC",
        "PHY",
        "PLL",
        "DLL",
        "ADC",
        "DAC",
        "FPGA",
        "ASIC",
        "EDA",
        "ISA",
        "ABI",
        "MMU",
        "TLB",
        "PCIE",
        "DDR",
        "LPDDR",
        "SDRAM",
        "HDMI",
        "MIPI",
        "LVDS",
        "SMP",
        "AMP",
        "NUMA",
        "SIMD",
        "SUBORDINATE",
        "MANAGER",
        "INITIATOR",
        "MASTER",
        "SLAVE",
    ]
    .into_iter()
    .collect()
}

fn should_emit_interface_candidate(signals: &[String]) -> bool {
    if signals.len() >= 2 {
        return true;
    }

    signals.first().is_some_and(|signal| {
        signal.ends_with("_N")
            || signal.ends_with("_B")
            || signal.contains("RST")
            || signal.contains("RESET")
            || signal.contains("CLK")
            || signal.contains("CLOCK")
    })
}

fn interface_ids_by_signal(interfaces: &[InterfaceRecord]) -> HashMap<String, BTreeSet<String>> {
    let mut ids_by_signal = HashMap::new();

    for interface in interfaces {
        for signal in &interface.signals {
            ids_by_signal
                .entry(signal.clone())
                .or_insert_with(BTreeSet::new)
                .insert(interface.interface_id.clone());
        }
    }

    ids_by_signal
}

fn related_interface_ids(
    signals: &[String],
    interface_ids_by_signal: &HashMap<String, BTreeSet<String>>,
) -> Vec<String> {
    let mut related_ids = BTreeSet::new();

    for signal in signals {
        if let Some(interface_ids) = interface_ids_by_signal.get(signal) {
            related_ids.extend(interface_ids.iter().cloned());
        }
    }

    related_ids.into_iter().collect()
}

fn phase_like_title(title: &str) -> bool {
    contains_any_phrase(
        &title.to_ascii_lowercase(),
        &[
            "phase",
            "sequence",
            "flow",
            "timing",
            "transaction",
            "handshake",
            "operation",
            "mode",
            "startup",
            "shutdown",
            "reset",
            "request",
            "response",
            "transport",
            "state",
            "write",
            "read",
        ],
    )
}

fn decomposition_like_title(title: &str) -> bool {
    contains_any_phrase(
        &title.to_ascii_lowercase(),
        &[
            "channel",
            "interface",
            "control",
            "data",
            "read",
            "write",
            "timing",
            "state",
            "reset",
            "transaction",
            "arbiter",
            "decoder",
            "encoder",
            "path",
        ],
    )
}

fn sequencing_language(text: &str) -> bool {
    contains_any_phrase(
        &text.to_ascii_lowercase(),
        &[
            "before", "after", "until", "during", "then", "next", "once", "when", "while",
        ],
    )
}

fn is_invariant_like(statement: &StatementContext, context: &SemanticContext) -> bool {
    if matches!(statement.class, StatementClass::ExplicitAbstraction) {
        return false;
    }

    let lowered_text = statement.text.to_ascii_lowercase();
    if contains_any_phrase(
        &lowered_text,
        &[
            "must",
            "shall",
            "always",
            "never",
            "required",
            "remains",
            "remain",
            "until",
            "only when",
            "cannot",
            "must not",
            "shall not",
        ],
    ) {
        return true;
    }

    if !statement.signals.is_empty()
        && contains_any_phrase(
            &lowered_text,
            &[
                "handshake",
                "asserted",
                "deasserted",
                "transition",
                "state",
                "timing",
                "observed",
            ],
        )
    {
        return true;
    }

    statement
        .related_visual_evidence_ids
        .iter()
        .any(|visual_id| {
            context
                .visual_roles_by_id
                .get(visual_id)
                .is_some_and(|role| {
                    matches!(
                        role,
                        VisualEvidenceRole::Normative | VisualEvidenceRole::Ambiguous
                    )
                })
        })
}

fn actor_ids_for_text(
    lowered_text: &str,
    actor_id_by_term: &HashMap<String, String>,
) -> Vec<String> {
    actor_id_by_term
        .iter()
        .filter_map(|(term, actor_id)| {
            contains_phrase(lowered_text, term).then_some(actor_id.clone())
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn overlapping_interface_signals(interfaces: &[InterfaceRecord]) -> Vec<String> {
    let mut counts: HashMap<String, usize> = HashMap::new();

    for interface in interfaces {
        for signal in &interface.signals {
            *counts.entry(signal.clone()).or_insert(0) += 1;
        }
    }

    counts
        .into_iter()
        .filter_map(|(signal, count)| (count > 1).then_some(signal))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn statement_ids_to_section_ids(
    context: &SemanticContext,
    statement_ids: &[String],
) -> Vec<String> {
    let mut section_ids = BTreeSet::new();

    for statement_id in statement_ids {
        if let Some(statement) = statement_by_id(context, statement_id) {
            section_ids.extend(statement.section_ids.iter().cloned());
        }
    }

    section_ids.into_iter().collect()
}

fn statement_by_id<'a>(
    context: &'a SemanticContext,
    statement_id: &str,
) -> Option<&'a StatementContext> {
    context
        .statements
        .iter()
        .find(|statement| statement.statement_id == statement_id)
}

fn normalize_text_key(text: &str) -> String {
    normalize_sentence(text).to_ascii_lowercase()
}

fn normalize_sentence(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn contains_any_phrase(text: &str, phrases: &[&str]) -> bool {
    phrases.iter().any(|phrase| contains_phrase(text, phrase))
}

fn contains_phrase(text: &str, phrase: &str) -> bool {
    let phrase = phrase.to_ascii_lowercase();

    for (index, _) in text.match_indices(&phrase) {
        let prefix_ok = text[..index]
            .chars()
            .next_back()
            .map(|character| !character.is_ascii_alphanumeric())
            .unwrap_or(true);
        let suffix_index = index + phrase.len();
        let suffix_ok = text[suffix_index..]
            .chars()
            .next()
            .map(|character| !character.is_ascii_alphanumeric())
            .unwrap_or(true);

        if prefix_ok && suffix_ok {
            return true;
        }
    }

    false
}

fn build_register_records(context: &SemanticContext) -> Vec<RegisterRecord> {
    // Currently carry-through from EvidenceIR. Direct semantic enrichment
    // (cross-referencing registers against signal declarations, etc.) is future work.
    // The context does not yet expose EvidenceIR register_records directly, so we
    // return an empty Vec here; the IntentIR builder reads them from SemanticIR
    // which receives them via EvidenceIR.load_from_path already.//
    // NOTE: SemanticIR.register_records is populated directly in SemanticIr::build()
    // from evidence_ir.register_records; this stub exists to satisfy the call site.
    let _ = context;
    Vec::new()
}

fn build_timing_constraints(context: &SemanticContext) -> Vec<TimingConstraintRecord> {
    let _ = context;
    Vec::new()
}

/// Parse VLM-derived `VisualObservation` entries from `EvidenceIR` into typed
/// `SemanticIR` records.
///
/// Returns `(timing_constraints, regular_states, state_transitions)` extracted from:
/// - `VisualObservationKind::TimingDiagramExtraction` → timing annotation → `TimingConstraintRecord`
/// - `VisualObservationKind::StateMachineExtraction` → states/transitions → typed records
fn extract_records_from_vlm_observations(
    evidence_ir: &EvidenceIr,
) -> (
    Vec<TimingConstraintRecord>,
    Vec<RegularStateRecord>,
    Vec<StateTransitionRecord>,
) {
    let mut timing_records = Vec::new();
    let mut state_records = Vec::new();
    let mut transition_records = Vec::new();

    for visual_item in &evidence_ir.visual_evidence {
        for obs in &visual_item.observations {
            match obs.kind {
                VisualObservationKind::TimingDiagramExtraction => {
                    parse_timing_diagram_observation(
                        &obs.text,
                        &visual_item.evidence_id,
                        &mut timing_records,
                    );
                }
                VisualObservationKind::StateMachineExtraction => {
                    parse_state_machine_observation(
                        &obs.text,
                        &visual_item.evidence_id,
                        &mut state_records,
                        &mut transition_records,
                    );
                }
                _ => {}
            }
        }
    }

    (timing_records, state_records, transition_records)
}

/// Parse a `TimingDiagramExtraction` JSON observation into `TimingConstraintRecord` entries.
/// Expected format:
/// `{"signals":[{"name":str,"values":[{"cycle":str,"state":str}]}],"annotations":[str]}`
fn parse_timing_diagram_observation(
    json_text: &str,
    evidence_id: &str,
    records: &mut Vec<TimingConstraintRecord>,
) {
    // Use serde_json for robust parsing.
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json_text) else {
        return;
    };

    // Each annotation string becomes a TimingConstraintRecord.
    if let Some(annotations) = value.get("annotations").and_then(|a| a.as_array()) {
        for (idx, annotation) in annotations.iter().enumerate() {
            let text = annotation.as_str().unwrap_or_default().trim();
            if text.is_empty() {
                continue;
            }
            records.push(TimingConstraintRecord {
                constraint_id: format!("vlm_timing_{}_{idx:03}", document_key(evidence_id)),
                parameter_name: format!("vlm_annotation_{idx:03}"),
                min_value: None,
                typ_value: None,
                max_value: None,
                unit: None,
                description: Some(text.to_string()),
                supporting_statement_ids: vec![evidence_id.to_string()],
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    // Each signal cycle pair adds context but no single-value record for now.
    // Future: extract "HCLK stays HIGH for 3 cycles" patterns into TimingConstraintRecord.
}

/// Parse a `StateMachineExtraction` JSON observation into state and transition records.
/// Expected format:
/// `{"states":[{"name":str,"is_initial":bool}],"transitions":[{"from":str,"to":str,"guard":str}]}`
fn parse_state_machine_observation(
    json_text: &str,
    evidence_id: &str,
    state_records: &mut Vec<RegularStateRecord>,
    transition_records: &mut Vec<StateTransitionRecord>,
) {
    let Ok(value) = serde_json::from_str::<serde_json::Value>(json_text) else {
        return;
    };

    // Parse states.
    if let Some(states) = value.get("states").and_then(|s| s.as_array()) {
        for (idx, state_val) in states.iter().enumerate() {
            let name = state_val
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or_default()
                .trim();
            if name.is_empty() {
                continue;
            }
            let is_initial = state_val
                .get("is_initial")
                .and_then(|b| b.as_bool())
                .unwrap_or(false);
            state_records.push(RegularStateRecord {
                state_id: format!(
                    "vlm_state_{}_{}",
                    document_key(evidence_id),
                    document_key(name)
                ),
                state_name: name.to_string(),
                is_initial,
                declaration_order: u32::try_from(idx).unwrap_or(0),
                supporting_statement_ids: vec![evidence_id.to_string()],
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    // Parse transitions.
    if let Some(transitions) = value.get("transitions").and_then(|t| t.as_array()) {
        for (idx, trans_val) in transitions.iter().enumerate() {
            let from = trans_val
                .get("from")
                .and_then(|f| f.as_str())
                .unwrap_or_default()
                .trim()
                .to_string();
            let to = trans_val
                .get("to")
                .and_then(|t| t.as_str())
                .unwrap_or_default()
                .trim()
                .to_string();
            if from.is_empty() || to.is_empty() {
                continue;
            }
            let guard_text = trans_val
                .get("guard")
                .and_then(|g| g.as_str())
                .unwrap_or_default()
                .trim();
            // Convert guard string to DecisionTreeGuardRecord heuristically.
            let guard = if guard_text.is_empty() {
                None
            } else {
                Some(DecisionTreeGuardRecord::SignalIsHigh {
                    signal_name: guard_text.to_string(),
                })
            };
            transition_records.push(StateTransitionRecord {
                transition_id: format!(
                    "vlm_trans_{}_{}_{idx:03}",
                    document_key(evidence_id),
                    document_key(&from)
                ),
                source_state: from,
                target_state: to,
                guard,
                declaration_order: u32::try_from(idx).unwrap_or(0),
                supporting_statement_ids: vec![evidence_id.to_string()],
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }
}

fn is_boilerplate_section_title(title: &str) -> bool {
    let lowered = title.to_ascii_lowercase();
    // Legal sections common to ARM/chip specifications
    if lowered.contains("licence") || lowered.contains("license") {
        return true;
    }
    contains_any_phrase(
        &lowered,
        &[
            "proprietary notice",
            "proprietary information",
            "change history",
            "revision history",
            "release information",
            "release note",
            "release history",
            "preface",
            "about this document",
            "how to use this",
            "how to read",
            "intended audience",
            "feedback",
            "contact",
            "non-confidential",
            "confidentiality",
            "legal notice",
            "legal information",
            "terms of use",
            "terms and conditions",
            "end user",
            "specification licence",
            "specification license",
            "trademark",
            "copyright notice",
            "disclaimer",
        ],
    )
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
    use crate::ir::source::{
        AutomationConfidence, SourceIr, VisualAsset, VisualAssetKind, WidthHint,
    };

    use super::{
        ControlActionRecord, ControlBinaryOperator, ControlBlockRole,
        ControlCompoundUpdateOperation, ControlDualOutputKind, ControlExpressionRecord,
        ControlReferenceKind, ControlReferenceSuffix, DecisionTreeActionRecord,
        DecisionTreeAssignmentKind, DecisionTreeComparisonOperator, DecisionTreeGuardRecord,
        DecisionTreeValueRecord, InterfaceSignalDirection, SemanticIr, SymbolDefinitionKind,
        SystemResetKind, SystemResetPolarity, SystemResetTargetKind, SystemResetTimingRelation,
    };

    #[test]
    fn builds_semantic_ir_from_handshake_evidence() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("handshake.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# Channel Operation\nThe transmitter asserts VALID when data is available.\n\nThe receiver asserts READY when it can accept data.\n\nVALID must remain asserted until READY is observed.\n",
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

        assert_eq!(semantic_ir.stage.as_str(), "semantic_ir");
        assert!(
            semantic_ir
                .actors
                .iter()
                .any(|actor| actor.actor_id == "actor_transmitter")
        );
        assert!(
            semantic_ir
                .actors
                .iter()
                .any(|actor| actor.actor_id == "actor_receiver")
        );
        assert!(semantic_ir.interfaces.iter().any(|interface| {
            interface.signals.contains(&"VALID".to_string())
                && interface.signals.contains(&"READY".to_string())
        }));
        assert!(semantic_ir.invariants.iter().any(|invariant| {
            invariant
                .statement
                .contains("VALID must remain asserted until READY is observed.")
        }));
        assert!(!semantic_ir.gates.is_empty());
        assert!(semantic_ir.residual_decisions.is_empty());

        Ok(())
    }

    #[test]
    fn emits_residual_decision_for_ambiguous_visual_grounding() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("control.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
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

        assert!(
            semantic_ir
                .actors
                .iter()
                .any(|actor| actor.actor_id == "actor_controller")
        );
        assert!(
            semantic_ir
                .residual_decisions
                .iter()
                .any(|packet| { packet.packet_id == "semantic_ambiguous_visual_grounding" })
        );
        assert!(
            semantic_ir
                .artifact_layout
                .semantic_ir_path
                .ends_with("generated/semantic_ir/control/semantic_ir.json")
        );

        Ok(())
    }

    #[test]
    fn extracts_typed_interface_signals_and_dt_fragments_from_explicit_markdown() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("comb_dt.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let explicit_interface = semantic_ir
            .interfaces
            .iter()
            .find(|interface| interface.signals.contains(&"DATA_IN".to_string()))
            .expect("explicit interface should be present");
        assert!(explicit_interface.signal_records.iter().any(|signal| {
            signal.signal_name == "DATA_IN"
                && signal.direction_hint == Some(InterfaceSignalDirection::Input)
                && signal.width_hint == Some(WidthHint::Numeric(8))
        }));
        assert!(explicit_interface.signal_records.iter().any(|signal| {
            signal.signal_name == "ZERO_FLAG"
                && signal.direction_hint == Some(InterfaceSignalDirection::Output)
                && signal.width_hint == Some(WidthHint::Numeric(1))
        }));
        assert_eq!(semantic_ir.decision_tree_fragments.len(), 2);
        assert!(semantic_ir.decision_tree_fragments.iter().any(|fragment| {
            fragment.block_name == "route_data"
                && matches!(
                    fragment.actions.first(),
                    Some(DecisionTreeActionRecord::Assign {
                        target_signal,
                        assignment_kind: DecisionTreeAssignmentKind::Combinational,
                        value: DecisionTreeValueRecord::SignalRef { signal_name },
                    }) if target_signal == "DATA_OUT" && signal_name == "DATA_IN"
                )
        }));
        assert!(semantic_ir.decision_tree_fragments.iter().any(|fragment| {
            fragment.block_name == "flag_zero"
                && matches!(
                    fragment.guard.as_ref(),
                    Some(DecisionTreeGuardRecord::Comparison {
                        left_signal,
                        operator: DecisionTreeComparisonOperator::Eq,
                        right: DecisionTreeValueRecord::Literal { literal },
                    }) if left_signal == "DATA_IN" && literal == "8'0"
                )
        }));

        Ok(())
    }

    #[test]
    fn extracts_system_contract_and_init_assignments_from_explicit_markdown() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("seq_dt.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let system_contract = semantic_ir
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
        assert_eq!(semantic_ir.init_assignments.len(), 1);
        assert!(matches!(
            semantic_ir.init_assignments.first(),
            Some(super::InitAssignmentRecord {
                target_signal,
                value: DecisionTreeValueRecord::Literal { literal },
                ..
            }) if target_signal == "ACC" && literal == "8'0"
        ));
        assert!(semantic_ir.decision_tree_fragments.iter().any(|fragment| {
            fragment.block_name == "accumulate"
                && matches!(
                    fragment.actions.first(),
                    Some(DecisionTreeActionRecord::Assign {
                        target_signal,
                        assignment_kind: DecisionTreeAssignmentKind::Sequential,
                        value: DecisionTreeValueRecord::SignalRef { signal_name },
                    }) if target_signal == "ACC" && signal_name == "DATA_IN"
                )
        }));

        Ok(())
    }

    #[test]
    fn extracts_synchronous_active_high_reset_from_explicit_markdown() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("sync_dt.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let system_contract = semantic_ir
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
        assert_eq!(semantic_ir.init_assignments.len(), 1);

        Ok(())
    }

    #[test]
    fn infers_reset_polarity_when_explicit_level_is_omitted() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("inferred_reset.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let system_contract = semantic_ir
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
    fn extracts_symbol_definitions_and_rich_control_blocks_from_explicit_markdown() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("rich_control.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        assert_eq!(semantic_ir.symbol_definitions.len(), 3);
        assert!(semantic_ir.symbol_definitions.iter().any(|definition| {
            definition.symbol_name == "STEP"
                && definition.kind == SymbolDefinitionKind::Constant
                && matches!(
                    definition.value.as_ref(),
                    Some(ControlExpressionRecord::Literal { literal }) if literal == "8'1"
                )
        }));
        assert!(semantic_ir.symbol_definitions.iter().any(|definition| {
            definition.symbol_name == "mode_t"
                && definition.kind == SymbolDefinitionKind::Enum
                && definition.members.len() == 2
                && definition
                    .members
                    .iter()
                    .any(|member| member.member_name == "idle")
        }));

        let decode_block = semantic_ir
            .control_blocks
            .iter()
            .find(|block| block.block_name == "decode")
            .expect("decode control block should be present");
        assert_eq!(decode_block.role, ControlBlockRole::StandaloneDecisionTree);
        assert!(matches!(
            decode_block.selector.as_ref(),
            Some(ControlExpressionRecord::Reference { reference })
                if reference.base_name == "MODE"
                    && reference.kind_hint == ControlReferenceKind::Signal
        ));
        assert!(matches!(
            decode_block
                .branches
                .first()
                .and_then(|branch| branch.predicate.as_ref()),
            Some(ControlExpressionRecord::Binary {
                operator: ControlBinaryOperator::Eq,
                left,
                right,
            }) if matches!(
                left.as_ref(),
                ControlExpressionRecord::Reference { reference }
                    if reference.base_name == "MODE"
                        && reference.kind_hint == ControlReferenceKind::Signal
            ) && matches!(
                right.as_ref(),
                ControlExpressionRecord::Reference { reference }
                    if reference.base_name == "mode_t"
                        && reference.kind_hint == ControlReferenceKind::Symbol
                        && matches!(
                            reference.suffixes.first(),
                            Some(ControlReferenceSuffix::Member { member_name })
                                if member_name == "idle"
                        )
            )
        ));

        let busy_block = semantic_ir
            .control_blocks
            .iter()
            .find(|block| block.block_name == "busy")
            .expect("state body control block should be present");
        assert_eq!(busy_block.role, ControlBlockRole::StateBody);
        assert!(busy_block.branches.iter().any(|branch| {
            branch.actions.iter().any(|action| {
                matches!(
                    action,
                    ControlActionRecord::Assign {
                        target,
                        dual_output: Some(ControlDualOutputKind::NextSignal),
                        value:
                            ControlExpressionRecord::Binary {
                                operator: ControlBinaryOperator::Add,
                                ..
                            },
                        ..
                    } if target.signal_name == "ACC"
                )
            })
        }));
        assert!(busy_block.branches.iter().any(|branch| {
            branch.actions.iter().any(|action| {
                matches!(
                    action,
                    ControlActionRecord::DelayedPulse {
                        target,
                        delay,
                        value: ControlExpressionRecord::Literal { literal },
                    } if target.signal_name == "PULSE_OUT"
                        && target.exposed_public_output
                        && *delay == 2
                        && literal == "1"
                )
            })
        }));
        assert!(busy_block.branches.iter().any(|branch| {
            branch.actions.iter().any(|action| {
                matches!(
                    action,
                    ControlActionRecord::CompoundUpdate {
                        target,
                        operation: ControlCompoundUpdateOperation::Increment,
                        amount:
                            Some(ControlExpressionRecord::Reference { reference }),
                    } if target.signal_name == "ACC"
                        && reference.base_name == "STEP"
                        && reference.kind_hint == ControlReferenceKind::Symbol
                )
            })
        }));
        assert!(busy_block.branches.iter().any(|branch| {
            branch.actions.iter().any(|action| {
                matches!(
                    action,
                    ControlActionRecord::Transition { target_state }
                        if target_state == "idle"
                )
            })
        }));
        assert!(semantic_ir.control_blocks.iter().any(|block| {
            block.block_name == "clear_acc" && block.role == ControlBlockRole::ResetSynchronous
        }));
        assert!(semantic_ir.control_blocks.iter().any(|block| {
            block.block_name == "clear_pulse" && block.role == ControlBlockRole::ResetAsynchronous
        }));

        Ok(())
    }

    #[test]
    fn extracts_explicit_modules_and_tops_from_markdown() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("composition.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        assert_eq!(semantic_ir.explicit_modules.len(), 2);
        assert_eq!(semantic_ir.explicit_tops.len(), 1);
        assert!(semantic_ir.explicit_modules.iter().any(|module| {
            module.module_name == "producer_core"
                && module
                    .decision_tree_fragments
                    .iter()
                    .any(|fragment| fragment.block_name == "produce")
                && module
                    .control_blocks
                    .iter()
                    .any(|block| block.block_name == "produce")
        }));
        assert!(semantic_ir.explicit_modules.iter().any(|module| {
            module.module_name == "consumer_core"
                && module.interfaces.iter().any(|interface| {
                    interface.signal_records.iter().any(|signal| {
                        signal.signal_name == "input_data"
                            && signal.direction_hint == Some(InterfaceSignalDirection::Input)
                            && signal.width_hint == Some(WidthHint::Numeric(8))
                    })
                })
        }));

        let explicit_top = semantic_ir
            .explicit_tops
            .iter()
            .find(|top| top.top_name == "datapath")
            .expect("explicit top should be present");
        assert_eq!(explicit_top.ports.len(), 1);
        assert_eq!(explicit_top.children.len(), 2);
        assert_eq!(explicit_top.links.len(), 2);
        assert!(explicit_top.ports.iter().any(|port| {
            port.port_name == "result_data"
                && port.direction_hint == InterfaceSignalDirection::Output
                && port.width_hint == Some(WidthHint::Numeric(8))
        }));
        assert!(explicit_top.children.iter().any(|child| {
            child.instance_name == "producer" && child.source_module_name == "producer_core"
        }));
        assert!(explicit_top.links.iter().any(|link| {
            link.source.instance_name.as_deref() == Some("consumer")
                && link.source.signal_name == "result_data"
                && link.target.instance_name.is_none()
                && link.target.signal_name == "result_data"
        }));

        Ok(())
    }

    #[test]
    fn extracts_regular_states_and_state_transitions_from_explicit_markdown() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("explicit_fsm.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        assert_eq!(semantic_ir.regular_states.len(), 2);
        assert!(semantic_ir.regular_states.iter().any(|state| {
            state.state_name == "idle" && state.is_initial && state.declaration_order == 0
        }));
        assert!(semantic_ir.regular_states.iter().any(|state| {
            state.state_name == "busy" && !state.is_initial && state.declaration_order == 1
        }));
        assert_eq!(semantic_ir.state_transitions.len(), 2);
        assert!(semantic_ir.state_transitions.iter().any(|transition| {
            transition.source_state == "idle"
                && transition.target_state == "busy"
                && matches!(
                    transition.guard.as_ref(),
                    Some(DecisionTreeGuardRecord::SignalIsHigh { signal_name })
                        if signal_name == "GO"
                )
        }));
        assert!(semantic_ir.state_transitions.iter().any(|transition| {
            transition.source_state == "busy"
                && transition.target_state == "idle"
                && matches!(
                    transition.guard.as_ref(),
                    Some(DecisionTreeGuardRecord::SignalIsHigh { signal_name })
                        if signal_name == "DONE"
                )
        }));
        assert!(
            semantic_ir
                .decision_tree_fragments
                .iter()
                .any(|fragment| fragment.block_name == "idle")
        );
        assert!(
            semantic_ir
                .decision_tree_fragments
                .iter()
                .any(|fragment| fragment.block_name == "busy")
        );

        Ok(())
    }

    #[test]
    fn vlm_timing_diagram_observation_produces_timing_constraint_records() -> Result<()> {
        // Tests the VLM wiring chain in SemanticIR:
        //   SourceIR.visual_assets[i].note = "vlm_timing_diagram_extraction: {json}"
        //     → EvidenceIR injects TimingDiagramExtraction observation
        //     → SemanticIR parses annotations → TimingConstraintRecord entries
        let tempdir = tempdir()?;
        let source = tempdir.path().join("timing_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# Timing\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0001".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 3-1 Read transfer timing".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_timing_diagram_extraction: {\"signals\":[{\"name\":\"HCLK\",\"values\":[{\"cycle\":\"T1\",\"state\":\"HIGH\"}]}],\"annotations\":[\"tSU = 2 ns\",\"tHD = 1 ns\"]}"
                    .to_string(),
            ),
            diagram_kind: crate::ir::source::DiagramKind::TimingDiagram,
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

        // VLM annotations should produce TimingConstraintRecord entries.
        assert!(
            semantic_ir.timing_constraints.iter().any(|tc| {
                tc.description
                    .as_deref()
                    .map(|d| d.contains("tSU"))
                    .unwrap_or(false)
            }),
            "expected timing constraint from VLM annotation 'tSU = 2 ns'"
        );
        assert!(
            semantic_ir.timing_constraints.iter().any(|tc| {
                tc.description
                    .as_deref()
                    .map(|d| d.contains("tHD"))
                    .unwrap_or(false)
            }),
            "expected timing constraint from VLM annotation 'tHD = 1 ns'"
        );

        Ok(())
    }

    #[test]
    fn vlm_state_machine_observation_produces_state_and_transition_records() -> Result<()> {
        // Tests the VLM wiring chain for state machine diagrams:
        //   SourceIR.visual_assets[i].note = "vlm_state_machine_extraction: {json}"
        //     → EvidenceIR injects StateMachineExtraction observation
        //     → SemanticIR appends RegularStateRecord + StateTransitionRecord entries
        let tempdir = tempdir()?;
        let source = tempdir.path().join("sm_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# State Machine\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0002".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 5-1 Transfer state machine".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_state_machine_extraction: {\"states\":[{\"name\":\"IDLE\",\"is_initial\":true},{\"name\":\"BUSY\",\"is_initial\":false}],\"transitions\":[{\"from\":\"IDLE\",\"to\":\"BUSY\",\"guard\":\"HTRANS_NONSEQ\"}]}"
                    .to_string(),
            ),
            diagram_kind: crate::ir::source::DiagramKind::StateMachineDiagram,
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

        // VLM states should appear in regular_states.
        assert!(
            semantic_ir
                .regular_states
                .iter()
                .any(|s| s.state_name == "IDLE" && s.is_initial),
            "expected IDLE initial state from VLM extraction"
        );
        assert!(
            semantic_ir
                .regular_states
                .iter()
                .any(|s| s.state_name == "BUSY"),
            "expected BUSY state from VLM extraction"
        );
        // VLM transitions should appear in state_transitions.
        assert!(
            semantic_ir
                .state_transitions
                .iter()
                .any(|t| t.source_state == "IDLE" && t.target_state == "BUSY"),
            "expected IDLE→BUSY transition from VLM extraction"
        );

        Ok(())
    }

    #[test]
    fn extracts_signal_direction_and_width_from_structured_table_in_source_ir() -> Result<()> {
        // Tests the new SOTA architecture:
        //   SourceIR.structured_tables (Docling cell grids)
        //     → EvidenceIR synthesizes Signal declarations
        //     → SemanticIR parses them via parse_explicit_signal_declaration
        //
        // We populate structured_tables directly to simulate what Docling would
        // produce for a PDF with Manager and Subordinate signal tables.
        use crate::ir::source::{
            ContentSectionRecord, SectionKind, StructuredTableCellRecord, StructuredTableRecord,
            TableKind,
        };

        let tempdir = tempdir()?;
        let source = tempdir.path().join("stub.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# Stub\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;

        // Simulate Docling extracting a "Manager signals" section on page 1.
        source_ir.document_sections.push(ContentSectionRecord {
            section_id: "sec_0001_manager_signals".to_string(),
            title: "Manager signals".to_string(),
            heading_level: 2,
            page_id: Some("page_0001".to_string()),
            source_ref: None,
            reading_order: 1,
            section_kind: SectionKind::SignalDescription,
        });
        source_ir.document_sections.push(ContentSectionRecord {
            section_id: "sec_0002_subordinate_signals".to_string(),
            title: "Subordinate signals".to_string(),
            heading_level: 2,
            page_id: Some("page_0002".to_string()),
            source_ref: None,
            reading_order: 10,
            section_kind: SectionKind::SignalDescription,
        });

        // Manager signal table (page 1).
        let make_cell = |text: &str, is_header: bool| StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        };
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_0001".to_string(),
            asset_id: "table_0001".to_string(),
            page_id: Some("page_0001".to_string()),
            caption_text: None,
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_cell("Name", true),
                make_cell("Destination", true),
                make_cell("Width", true),
            ]],
            body_rows: vec![
                vec![
                    make_cell("HADDR", false),
                    make_cell("Subordinate", false),
                    make_cell("32", false),
                ],
                vec![
                    make_cell("HWRITE", false),
                    make_cell("Subordinate", false),
                    make_cell("1", false),
                ],
                vec![
                    make_cell("HTRANS", false),
                    make_cell("Subordinate", false),
                    make_cell("2", false),
                ],
            ],
            row_count: 3,
            col_count: 3,
        });

        // Subordinate signal table (page 2).
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_0002".to_string(),
            asset_id: "table_0002".to_string(),
            page_id: Some("page_0002".to_string()),
            caption_text: None,
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_cell("Name", true),
                make_cell("Destination", true),
                make_cell("Width", true),
            ]],
            body_rows: vec![
                vec![
                    make_cell("HREADYOUT", false),
                    make_cell("Manager", false),
                    make_cell("1", false),
                ],
                vec![
                    make_cell("HRESP", false),
                    make_cell("Manager", false),
                    make_cell("1", false),
                ],
            ],
            row_count: 2,
            col_count: 3,
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

        // Manager signals should be extracted as Output with explicit widths.
        let find_signal = |name: &str| -> Option<super::InterfaceSignalRecord> {
            semantic_ir.interfaces.iter().find_map(|iface| {
                iface
                    .signal_records
                    .iter()
                    .find(|sig| sig.signal_name == name)
                    .cloned()
            })
        };

        let haddr = find_signal("HADDR").expect("HADDR should be extracted from table");
        assert_eq!(haddr.direction_hint, Some(InterfaceSignalDirection::Output));
        assert_eq!(haddr.width_hint, Some(WidthHint::Numeric(32)));

        let hwrite = find_signal("HWRITE").expect("HWRITE should be extracted from table");
        assert_eq!(
            hwrite.direction_hint,
            Some(InterfaceSignalDirection::Output)
        );
        assert_eq!(hwrite.width_hint, Some(WidthHint::Numeric(1)));

        let htrans = find_signal("HTRANS").expect("HTRANS should be extracted from table");
        assert_eq!(
            htrans.direction_hint,
            Some(InterfaceSignalDirection::Output)
        );
        assert_eq!(htrans.width_hint, Some(WidthHint::Numeric(2)));

        // Subordinate signals should be extracted as Input with explicit widths.
        let hreadyout = find_signal("HREADYOUT").expect("HREADYOUT should be extracted from table");
        assert_eq!(
            hreadyout.direction_hint,
            Some(InterfaceSignalDirection::Input)
        );
        assert_eq!(hreadyout.width_hint, Some(WidthHint::Numeric(1)));

        let hresp = find_signal("HRESP").expect("HRESP should be extracted from table");
        assert_eq!(hresp.direction_hint, Some(InterfaceSignalDirection::Input));
        assert_eq!(hresp.width_hint, Some(WidthHint::Numeric(1)));

        Ok(())
    }

    // ── Layer D: declared-signal gating ────────────────────────────────

    #[test]
    fn signal_constraints_for_undeclared_signals_are_filtered_by_layer_d() -> Result<()> {
        // Build an EvidenceIR with:
        //   • A synthesized "Signal HREADY is input width 1." declaration  (declared signal)
        //   • A signal_constraint for HREADY  (should survive gating)
        //   • A signal_constraint for NOTSIG  (undeclared, should be removed)
        // After SemanticIR.build(), only the HREADY constraint must appear.
        use crate::ir::evidence::{
            EvidenceIr, EvidenceModality, ExtractedStatement, StatementClass,
        };
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# Protocol\nHREADY shall be asserted when the transfer completes.\n",
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;

        // Inject a formal signal declaration (as synthesized from a signal description table).
        evidence_ir.extracted_statements.push(ExtractedStatement {
            statement_id: "stmt_decl_hready".to_string(),
            text: "Signal HREADY is input width 1.".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            evidence_span_ids: vec![],
            related_visual_evidence_ids: vec![],
        });

        // Add a constraint for the declared signal HREADY.
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_hready".to_string(),
            subject_signal: "HREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: Some("transfer completes".to_string()),
            negated: false,
            source_text: "HREADY shall be asserted when the transfer completes.".to_string(),
            supporting_statement_ids: vec!["stmt_001".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });

        // Add a constraint for an undeclared signal NOTSIG (heuristic noise).
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_notsig".to_string(),
            subject_signal: "NOTSIG".to_string(),
            constraint_kind: SignalConstraintKind::MustBeStable,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "NOTSIG must be stable.".to_string(),
            supporting_statement_ids: vec!["stmt_002".to_string()],
            automation_confidence: AutomationConfidence::Low,
        });

        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        // HREADY (declared) must pass the gate.
        assert!(
            semantic_ir
                .signal_constraints
                .iter()
                .any(|r| r.subject_signal == "HREADY"),
            "HREADY is a declared signal and its constraint must survive Layer D gating"
        );
        // NOTSIG (undeclared) must be filtered out.
        assert!(
            !semantic_ir
                .signal_constraints
                .iter()
                .any(|r| r.subject_signal == "NOTSIG"),
            "NOTSIG is not declared and its constraint must be removed by Layer D gating"
        );

        Ok(())
    }
}
