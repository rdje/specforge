use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::{EvidenceIr, StatementClass, VisualEvidenceRole};
use crate::ir::source::{
    AutomationConfidence, CandidateInterpretation, ResidualDecisionPacket, document_key,
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
        let residual_decisions =
            build_residual_decisions(&context, &interfaces, actor_build.explicit_actor_count);

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
            regular_states,
            state_transitions,
            decision_tree_fragments,
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
    pub width_hint: Option<u32>,
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
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SystemResetKind {
    Synchronous,
    Asynchronous,
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

        let mut section_statement_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let statements = evidence_ir
            .extracted_statements
            .iter()
            .map(|statement| {
                let section_ids = section_ids_for_statement(
                    statement.evidence_span_ids.as_slice(),
                    &spans_by_id,
                    &evidence_ir.section_anchors,
                );
                for section_id in &section_ids {
                    section_statement_ids
                        .entry(section_id.clone())
                        .or_default()
                        .push(statement.statement_id.clone());
                }

                StatementContext {
                    statement_id: statement.statement_id.clone(),
                    class: statement.class,
                    text: statement.text.clone(),
                    related_visual_evidence_ids: statement.related_visual_evidence_ids.clone(),
                    section_ids,
                    signals: extract_signal_tokens(&statement.text),
                }
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
    width_hint: Option<u32>,
    supporting_statement_ids: BTreeSet<String>,
    automation_confidence: AutomationConfidence,
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
    width_hint: Option<u32>,
}

#[derive(Debug, Clone)]
struct ParsedSystemResetDeclaration {
    signal_name: String,
    reset_kind: SystemResetKind,
}

#[derive(Debug, Clone)]
struct ParsedInitAssignment {
    target_signal: String,
    value: DecisionTreeValueRecord,
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
struct DecisionTreeFragmentAccumulator {
    block_name: String,
    guard: Option<DecisionTreeGuardRecord>,
    actions: Vec<DecisionTreeActionRecord>,
    referenced_signal_names: BTreeSet<String>,
    supporting_statement_ids: BTreeSet<String>,
    automation_confidence: AutomationConfidence,
}

fn build_interfaces(context: &SemanticContext) -> Vec<InterfaceRecord> {
    let mut accumulators: BTreeMap<String, InterfaceAccumulator> = BTreeMap::new();

    for statement in &context.statements {
        if let Some(signal_declaration) = parse_explicit_signal_declaration(&statement.text) {
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
            || parse_explicit_system_clock(&statement.text).is_some()
            || parse_explicit_system_reset(&statement.text).is_some()
            || parse_explicit_init_assignment(&statement.text).is_some()
            || parse_explicit_regular_state_declaration(&statement.text).is_some()
            || parse_explicit_state_transition(&statement.text).is_some()
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
    let mut supporting_statement_ids = BTreeSet::new();
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
        supporting_statement_ids: supporting_statement_ids.into_iter().collect(),
        automation_confidence: AutomationConfidence::High,
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

    Some(ParsedSystemResetDeclaration {
        signal_name,
        reset_kind: parse_system_reset_kind(&tokens[is_token_index + 1..])?,
    })
}

fn parse_system_reset_kind(tokens: &[&str]) -> Option<SystemResetKind> {
    if tokens.is_empty() {
        return None;
    }

    let filtered_tokens = tokens
        .iter()
        .map(|token| token.trim_end_matches('.').to_ascii_lowercase())
        .filter(|token| token != "active" && token != "low")
        .collect::<Vec<_>>();

    match filtered_tokens.as_slice() {
        [kind] if kind == "sync" || kind == "synchronous" => Some(SystemResetKind::Synchronous),
        [kind] if kind == "async" || kind == "asynchronous" => Some(SystemResetKind::Asynchronous),
        _ => None,
    }
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

fn parse_optional_width_hint(tokens: &[&str], index: &mut usize) -> Option<u32> {
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

fn parse_width_token(token: &str) -> Option<u32> {
    let trimmed = token
        .trim()
        .trim_end_matches('.')
        .trim_end_matches(',')
        .trim_end_matches(':');
    let trimmed = trimmed
        .strip_suffix("-bit")
        .or_else(|| trimmed.strip_suffix("-bits"))
        .unwrap_or(trimmed);
    let width = trimmed.parse::<u32>().ok()?;
    (width > 0).then_some(width)
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
    width_hint: Option<u32>,
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

fn merge_signal_hint<T: Copy + Eq>(target: &mut Option<T>, incoming: Option<T>) {
    match (*target, incoming) {
        (None, Some(value)) => *target = Some(value),
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
    use crate::ir::source::{SourceIr, VisualAsset, VisualAssetKind};

    use super::{
        DecisionTreeActionRecord, DecisionTreeAssignmentKind, DecisionTreeComparisonOperator,
        DecisionTreeGuardRecord, DecisionTreeValueRecord, InterfaceSignalDirection, SemanticIr,
        SystemResetKind,
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
                && signal.width_hint == Some(8)
        }));
        assert!(explicit_interface.signal_records.iter().any(|signal| {
            signal.signal_name == "ZERO_FLAG"
                && signal.direction_hint == Some(InterfaceSignalDirection::Output)
                && signal.width_hint == Some(1)
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

        assert_eq!(
            semantic_ir.system_contract.as_ref().map(|contract| (
                contract.clock_signal.as_str(),
                contract.reset_signal.as_str(),
                contract.reset_kind,
            )),
            Some(("clk", "rst_n", SystemResetKind::Asynchronous))
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
}
