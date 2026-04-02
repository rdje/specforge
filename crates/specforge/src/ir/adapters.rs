use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::intent::{IntentDocumentIdentity, IntentIr};
use crate::ir::semantic::{
    ControlActionRecord, ControlAssignmentTargetRecord, ControlBinaryOperator, ControlBlockRecord,
    ControlBlockRole, ControlBranchRecord, ControlCompoundUpdateOperation, ControlDualOutputKind,
    ControlExpressionRecord, ControlReferenceKind, ControlReferenceRecord, ControlReferenceSuffix,
    ControlUnaryOperator, DecisionTreeActionRecord, DecisionTreeAssignmentKind,
    DecisionTreeComparisonOperator, DecisionTreeFragmentRecord, DecisionTreeGuardRecord,
    DecisionTreeValueRecord, ExplicitModuleRecord, ExplicitTopLinkEndpoint, ExplicitTopLinkRecord,
    ExplicitTopPortRecord, ExplicitTopRecord, InitAssignmentRecord, InterfaceRecord,
    InterfaceSignalDirection, StateTransitionRecord, SymbolDefinitionKind, SymbolDefinitionRecord,
    SystemContractRecord, SystemResetKind, SystemResetPolarity, SystemResetTargetKind,
    SystemResetTimingRelation,
};
use crate::ir::source::{
    AutomationConfidence, CandidateInterpretation, ResidualDecisionPacket, document_key,
};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterTarget {
    Fsm,
    SystemVerilog,
    Verilog,
    Vhdl,
}

impl AdapterTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fsm => "fsm",
            Self::SystemVerilog => "system_verilog",
            Self::Verilog => "verilog",
            Self::Vhdl => "vhdl",
        }
    }
}

fn validate_system_reset_semantics_renderability(
    system_contract: &SystemContractRecord,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    let expected_assertion_timing = system_contract.reset_kind.assertion_timing();
    if system_contract.assertion_timing != expected_assertion_timing {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Canonical reset `{}` carries assertion timing `{}` that conflicts with reset kind `{}`.",
                system_contract.reset_signal,
                render_system_reset_timing_relation(system_contract.assertion_timing),
                render_system_reset_kind_name(system_contract.reset_kind)
            ),
        );
        required_canonical_enrichments.insert(
            "keep canonical reset assertion timing aligned with the selected reset kind before lowering `.fsm` system contracts".to_string(),
        );
    }

    let expected_release_timing = system_contract.reset_kind.release_timing();
    if system_contract.release_timing != expected_release_timing {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Canonical reset `{}` carries release timing `{}` that conflicts with reset kind `{}`.",
                system_contract.reset_signal,
                render_system_reset_timing_relation(system_contract.release_timing),
                render_system_reset_kind_name(system_contract.reset_kind)
            ),
        );
        required_canonical_enrichments.insert(
            "keep canonical reset release timing aligned with the selected reset kind before lowering `.fsm` system contracts".to_string(),
        );
    }

    let expected_target_kind = system_contract.reset_kind.target_kind();
    if system_contract.target_kind != expected_target_kind {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Canonical reset `{}` targets `{}` but reset kind `{}` expects `{}`.",
                system_contract.reset_signal,
                render_system_reset_target_kind(system_contract.target_kind),
                render_system_reset_kind_name(system_contract.reset_kind),
                render_system_reset_target_kind(expected_target_kind)
            ),
        );
        required_canonical_enrichments.insert(
            "keep canonical reset target semantics aligned with the selected reset kind before lowering `.fsm` system contracts".to_string(),
        );
    }

    let signal_looks_active_low = reset_signal_name_looks_active_low(&system_contract.reset_signal);
    match (system_contract.reset_polarity, signal_looks_active_low) {
        (SystemResetPolarity::ActiveLow, false) => {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical reset `{}` is active-low, but emitted `.fsm` text can only expose reset polarity through the reset signal name in the current slice.",
                    system_contract.reset_signal
                ),
            );
            required_canonical_enrichments.insert(
                "align active-low reset polarity with an `_n`/`_b` naming cue before lowering `.fsm` system contracts".to_string(),
            );
        }
        (SystemResetPolarity::ActiveHigh, true) => {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical reset `{}` is active-high, but its signal name currently implies an active-low reset in the current `.fsm` slice.",
                    system_contract.reset_signal
                ),
            );
            required_canonical_enrichments.insert(
                "align active-high reset polarity with a non-active-low reset signal name before lowering `.fsm` system contracts".to_string(),
            );
        }
        _ => {}
    }
}

fn validate_system_contract_renderability(
    system_contract: &SystemContractRecord,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    validate_system_signal_renderability(
        &system_contract.clock_signal,
        "clock",
        signals_by_name,
        blocking_reasons,
        required_canonical_enrichments,
    );
    validate_system_signal_renderability(
        &system_contract.reset_signal,
        "reset",
        signals_by_name,
        blocking_reasons,
        required_canonical_enrichments,
    );
    validate_system_reset_semantics_renderability(
        system_contract,
        blocking_reasons,
        required_canonical_enrichments,
    );
}

fn validate_system_signal_renderability(
    signal_name: &str,
    role_name: &str,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    let Some(signal) = signals_by_name.get(signal_name).copied() else {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Canonical system contract references undeclared {} signal `{}`.",
                role_name, signal_name
            ),
        );
        required_canonical_enrichments.insert(
            "promote a canonical interface inventory with stable signal names, directions, and widths"
                .to_string(),
        );
        return;
    };

    match signal.direction_hint {
        Some(InterfaceSignalDirection::Input) => {}
        Some(_) => {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical {} signal `{}` must be declared as an input for standalone `.fsm` lowering.",
                    role_name, signal_name
                ),
            );
            required_canonical_enrichments.insert(
                "keep first-slice system-contract signals aligned with explicit input roles"
                    .to_string(),
            );
        }
        None => {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical {} signal `{}` is missing a direction hint required for standalone `.fsm` lowering.",
                    role_name, signal_name
                ),
            );
            required_canonical_enrichments.insert(
                "promote a canonical interface inventory with stable signal names, directions, and widths"
                    .to_string(),
            );
        }
    }

    match signal.width_hint {
        Some(1) => {}
        Some(width) => {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical {} signal `{}` must be 1-bit for the first standalone `.fsm` system-contract slice, but width {} was provided.",
                    role_name, signal_name, width
                ),
            );
            required_canonical_enrichments
                .insert("keep first-slice system-contract signals explicitly 1-bit".to_string());
        }
        None => {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical {} signal `{}` is missing a width hint required for standalone `.fsm` lowering.",
                    role_name, signal_name
                ),
            );
            required_canonical_enrichments.insert(
                "promote a canonical interface inventory with stable signal names, directions, and widths"
                    .to_string(),
            );
        }
    }
}

fn validate_init_assignment_renderability(
    init_assignment: &InitAssignmentRecord,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    let target_direction = register_renderable_signal(
        &init_assignment.target_signal,
        signals_by_name,
        size_entries,
        blocking_reasons,
        required_canonical_enrichments,
    );
    if !matches!(target_direction, Some(InterfaceSignalDirection::Output)) {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Init assignment target `{}` is not declared as a canonical output signal.",
                init_assignment.target_signal
            ),
        );
        required_canonical_enrichments
            .insert("keep first-slice init targets aligned with explicit output roles".to_string());
    }

    if !matches!(
        init_assignment.value,
        DecisionTreeValueRecord::Literal { .. }
    ) {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Init assignment for `{}` must use an explicit literal in the first standalone `.fsm` slice.",
                init_assignment.target_signal
            ),
        );
        required_canonical_enrichments
            .insert("keep first-slice init assignments explicit and literal-valued".to_string());
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterStatus {
    Implemented,
    Planned,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterPlan {
    pub target: AdapterTarget,
    pub required_input_stage: IrStage,
    pub status: AdapterStatus,
    pub notes: Vec<String>,
}

pub fn default_adapter_plans() -> Vec<AdapterPlan> {
    vec![
        AdapterPlan {
            target: AdapterTarget::Fsm,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Implemented,
            notes: vec![
                "first DT-centric `.fsm` adapter artifact exists after IntentIR".to_string(),
                "real `.fsm` text is emitted only when canonical control/data structure is renderable without semantic invention"
                    .to_string(),
            ],
        },
        AdapterPlan {
            target: AdapterTarget::SystemVerilog,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Planned,
            notes: vec!["direct RTL adapter planned after IntentIR stabilization".to_string()],
        },
        AdapterPlan {
            target: AdapterTarget::Verilog,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Planned,
            notes: vec!["target-neutral lowering should support Verilog later".to_string()],
        },
        AdapterPlan {
            target: AdapterTarget::Vhdl,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Planned,
            notes: vec!["target-neutral lowering should support VHDL later".to_string()],
        },
    ]
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterLoweringStatus {
    Renderable,
    Blocked,
}

impl AdapterLoweringStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Renderable => "renderable",
            Self::Blocked => "blocked",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterArtifact {
    pub schema_version: u32,
    pub target: AdapterTarget,
    pub required_input_stage: IrStage,
    pub intent_ir_path: PathBuf,
    pub artifact_layout: AdapterArtifactLayout,
    pub adapter_identity: AdapterIdentity,
    pub document_identity: IntentDocumentIdentity,
    pub lowering_status: AdapterLoweringStatus,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fsm: Option<FsmAdapterArtifact>,
}

impl AdapterArtifact {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Err(AppError::MissingPath(path.to_path_buf()));
        }

        Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
    }

    pub fn build(
        intent_ir_path: &Path,
        target: AdapterTarget,
        artifact_base_root: &Path,
    ) -> Result<Self> {
        let intent_ir_path = canonicalize_existing_path(intent_ir_path)?;
        let raw_artifact = fs::read_to_string(&intent_ir_path)?;
        let stage_probe: StageProbe = serde_json::from_str(&raw_artifact)?;

        if !matches!(stage_probe.stage, IrStage::IntentIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} must be an IntentIR document before building an adapter artifact",
                intent_ir_path.display()
            )));
        }

        let intent_ir: IntentIr = serde_json::from_str(&raw_artifact)?;

        match target {
            AdapterTarget::Fsm => {
                build_fsm_adapter_artifact(&intent_ir, &intent_ir_path, artifact_base_root)
            }
            AdapterTarget::SystemVerilog => {
                Err(AppError::FeatureNotYetImplemented("SystemVerilog adapter"))
            }
            AdapterTarget::Verilog => Err(AppError::FeatureNotYetImplemented("Verilog adapter")),
            AdapterTarget::Vhdl => Err(AppError::FeatureNotYetImplemented("VHDL adapter")),
        }
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    pub fn write_to_disk(&self) -> Result<()> {
        fs::create_dir_all(&self.artifact_layout.artifact_root)?;
        fs::write(
            &self.artifact_layout.adapter_artifact_path,
            self.to_pretty_json()?,
        )?;

        if let (Some(path), Some(text)) = (
            self.artifact_layout.emitted_target_path.as_ref(),
            self.rendered_target_text(),
        ) {
            fs::write(path, text)?;
        }

        Ok(())
    }

    fn rendered_target_text(&self) -> Option<String> {
        match (&self.target, &self.fsm) {
            (AdapterTarget::Fsm, Some(fsm)) if fsm.renderability.is_renderable => {
                if let Some(document) = fsm.renderable_document.as_ref() {
                    return Some(render_fsm_source_document(document));
                }

                Some(render_fsm_module(
                    &fsm.root_name,
                    fsm.root_kind_decision.selected_root_kind,
                    fsm.renderable_module.as_ref()?,
                ))
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterArtifactLayout {
    pub artifact_root: PathBuf,
    pub adapter_artifact_path: PathBuf,
    pub emitted_target_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterIdentity {
    pub adapter_id: String,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmAdapterArtifact {
    pub root_name: String,
    pub root_kind_decision: FsmRootKindDecision,
    pub signal_inventory: Vec<FsmSignalCandidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_contract: Option<SystemContractRecord>,
    #[serde(default)]
    pub init_assignments: Vec<InitAssignmentRecord>,
    pub decision_tree_candidates: Vec<FsmDecisionTreeCandidate>,
    pub state_candidates: Vec<FsmStateCandidate>,
    #[serde(default)]
    pub transition_candidates: Vec<FsmTransitionCandidate>,
    #[serde(default)]
    pub module_candidates: Vec<FsmExplicitModuleCandidate>,
    #[serde(default)]
    pub top_candidates: Vec<FsmTopCandidate>,
    pub renderability: FsmRenderability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderable_module: Option<FsmRenderableModule>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderable_document: Option<FsmRenderableSourceDocument>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FsmRootKind {
    Dt,
    Fsm,
    Top,
}

impl FsmRootKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dt => "dt",
            Self::Fsm => "fsm",
            Self::Top => "top",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRootKindDecision {
    pub selected_root_kind: FsmRootKind,
    pub deferred_root_kinds: Vec<FsmRootKind>,
    pub automation_confidence: AutomationConfidence,
    pub rationale: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmSignalCandidate {
    pub signal_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_hint: Option<InterfaceSignalDirection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_hint: Option<u32>,
    pub supporting_canonical_ids: Vec<String>,
    pub mention_categories: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmDecisionTreeCandidate {
    pub candidate_id: String,
    pub summary: String,
    pub supporting_fragment_ids: Vec<String>,
    #[serde(default)]
    pub blocks: Vec<DecisionTreeFragmentRecord>,
    pub referenced_signal_names: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmStateCandidate {
    pub state_id: String,
    pub state_name: String,
    pub is_initial: bool,
    pub declaration_order: u32,
    pub supporting_canonical_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmTransitionCandidate {
    pub transition_id: String,
    pub source_state: String,
    pub target_state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guard: Option<DecisionTreeGuardRecord>,
    pub declaration_order: u32,
    pub supporting_canonical_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRenderability {
    pub is_renderable: bool,
    pub blocking_reasons: Vec<String>,
    pub required_canonical_enrichments: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRenderableModule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_contract: Option<SystemContractRecord>,
    pub size_entries: Vec<FsmRenderableSizeEntry>,
    #[serde(default)]
    pub symbol_definitions: Vec<SymbolDefinitionRecord>,
    #[serde(default)]
    pub init_assignments: Vec<InitAssignmentRecord>,
    #[serde(default)]
    pub states: Vec<FsmRenderableState>,
    #[serde(default)]
    pub blocks: Vec<ControlBlockRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRenderableSizeEntry {
    pub signal_name: String,
    pub direction_hint: InterfaceSignalDirection,
    pub width: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRenderableState {
    pub state_name: String,
    pub is_initial: bool,
    #[serde(default)]
    pub blocks: Vec<ControlBlockRecord>,
    #[serde(default)]
    pub transitions: Vec<StateTransitionRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmExplicitModuleCandidate {
    pub module_name: String,
    pub declaration_order: u32,
    pub root_kind_decision: FsmRootKindDecision,
    pub signal_inventory: Vec<FsmSignalCandidate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_contract: Option<SystemContractRecord>,
    #[serde(default)]
    pub init_assignments: Vec<InitAssignmentRecord>,
    pub decision_tree_candidates: Vec<FsmDecisionTreeCandidate>,
    pub state_candidates: Vec<FsmStateCandidate>,
    #[serde(default)]
    pub transition_candidates: Vec<FsmTransitionCandidate>,
    pub renderability: FsmRenderability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderable_module: Option<FsmRenderableModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmTopCandidate {
    pub top_name: String,
    pub declaration_order: u32,
    #[serde(default)]
    pub ports: Vec<ExplicitTopPortRecord>,
    #[serde(default)]
    pub children: Vec<FsmTopChildCandidate>,
    #[serde(default)]
    pub links: Vec<ExplicitTopLinkRecord>,
    pub renderability: FsmRenderability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderable_top: Option<FsmRenderableTopRoot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmTopChildCandidate {
    pub instance_name: String,
    pub source_module_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_root_kind: Option<FsmRootKind>,
    pub declaration_order: u32,
    pub supporting_canonical_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRenderableSourceDocument {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_root: Option<FsmRenderableTopRoot>,
    #[serde(default)]
    pub direct_roots: Vec<FsmRenderableNamedModule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRenderableNamedModule {
    pub module_name: String,
    pub root_kind: FsmRootKind,
    pub module: FsmRenderableModule,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRenderableTopRoot {
    pub top_name: String,
    #[serde(default)]
    pub ports: Vec<ExplicitTopPortRecord>,
    #[serde(default)]
    pub children: Vec<FsmRenderableTopChild>,
    #[serde(default)]
    pub links: Vec<ExplicitTopLinkRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRenderableTopChild {
    pub instance_name: String,
    pub source_module_name: String,
    pub child_root_kind: FsmRootKind,
}

#[derive(Debug, Clone)]
struct SelectedFsmSurface {
    root_name: String,
    root_kind_decision: FsmRootKindDecision,
    signal_inventory: Vec<FsmSignalCandidate>,
    system_contract: Option<SystemContractRecord>,
    init_assignments: Vec<InitAssignmentRecord>,
    decision_tree_candidates: Vec<FsmDecisionTreeCandidate>,
    state_candidates: Vec<FsmStateCandidate>,
    transition_candidates: Vec<FsmTransitionCandidate>,
    renderability: FsmRenderability,
    renderable_module: Option<FsmRenderableModule>,
    renderable_document: Option<FsmRenderableSourceDocument>,
}

#[derive(Debug, Clone, Copy)]
struct RenderableEndpointPort {
    direction_hint: InterfaceSignalDirection,
    width_hint: Option<u32>,
}

fn build_fsm_adapter_artifact(
    intent_ir: &IntentIr,
    intent_ir_path: &Path,
    artifact_base_root: &Path,
) -> Result<AdapterArtifact> {
    let artifact_root = artifact_base_root
        .join(AdapterTarget::Fsm.as_str())
        .join(&intent_ir.document_identity.document_key);
    let adapter_artifact_path = artifact_root.join("adapter.json");
    let default_root_name = fsm_root_name(&intent_ir.document_identity.document_key);
    let signal_inventory = build_signal_inventory(intent_ir);
    let system_contract = intent_ir.system_contract.clone();
    let init_assignments = intent_ir.init_assignments.clone();
    let state_candidates = build_state_candidates(intent_ir);
    let transition_candidates = build_transition_candidates(intent_ir);
    let root_kind_decision = build_root_kind_decision(&state_candidates);
    let direct_root_kind = root_kind_decision.selected_root_kind;
    let decision_tree_candidates = build_decision_tree_candidates(intent_ir, &signal_inventory);
    let (renderability, renderable_module) = analyze_renderability(
        &signal_inventory,
        system_contract.as_ref(),
        &init_assignments,
        &intent_ir.symbol_definitions,
        &intent_ir.control_blocks,
        &state_candidates,
        &transition_candidates,
        &decision_tree_candidates,
        &root_kind_decision,
    );
    let direct_surface = SelectedFsmSurface {
        root_name: default_root_name.clone(),
        root_kind_decision: root_kind_decision.clone(),
        signal_inventory,
        system_contract,
        init_assignments,
        decision_tree_candidates,
        state_candidates,
        transition_candidates,
        renderability,
        renderable_module: renderable_module.clone(),
        renderable_document: renderable_module
            .clone()
            .map(|module| FsmRenderableSourceDocument {
                top_root: None,
                direct_roots: vec![FsmRenderableNamedModule {
                    module_name: default_root_name.clone(),
                    root_kind: direct_root_kind,
                    module,
                }],
            }),
    };
    let module_candidates = build_module_candidates(intent_ir);
    let top_candidates = build_top_candidates(intent_ir, &module_candidates);
    let selected_surface = select_fsm_surface(
        intent_ir,
        direct_surface,
        &module_candidates,
        &top_candidates,
    );
    let emitted_target_path = selected_surface
        .renderability
        .is_renderable
        .then(|| artifact_root.join(format!("{}.fsm", selected_surface.root_name)));
    let residual_decisions = build_adapter_residual_decisions(
        intent_ir,
        &selected_surface.signal_inventory,
        &selected_surface.state_candidates,
        &selected_surface.transition_candidates,
        &selected_surface.decision_tree_candidates,
        &selected_surface.root_kind_decision,
        &selected_surface.renderability,
        &module_candidates,
        &top_candidates,
    );
    let lowering_status = if selected_surface.renderability.is_renderable {
        AdapterLoweringStatus::Renderable
    } else {
        AdapterLoweringStatus::Blocked
    };
    let adapter_identity = AdapterIdentity {
        adapter_id: format!(
            "adapter_{}_{}",
            AdapterTarget::Fsm.as_str(),
            intent_ir.document_identity.document_key
        ),
        summary: format!(
            "typed `.fsm` lowering artifact for {}",
            intent_ir.document_identity.display_name
        ),
    };
    let fsm = FsmAdapterArtifact {
        root_name: selected_surface.root_name,
        root_kind_decision: selected_surface.root_kind_decision,
        signal_inventory: selected_surface.signal_inventory,
        system_contract: selected_surface.system_contract,
        init_assignments: selected_surface.init_assignments,
        decision_tree_candidates: selected_surface.decision_tree_candidates,
        state_candidates: selected_surface.state_candidates,
        transition_candidates: selected_surface.transition_candidates,
        module_candidates,
        top_candidates,
        renderability: selected_surface.renderability,
        renderable_module: selected_surface.renderable_module,
        renderable_document: selected_surface.renderable_document,
    };

    Ok(AdapterArtifact {
        schema_version: 1,
        target: AdapterTarget::Fsm,
        required_input_stage: IrStage::IntentIr,
        intent_ir_path: intent_ir_path.to_path_buf(),
        artifact_layout: AdapterArtifactLayout {
            artifact_root,
            adapter_artifact_path,
            emitted_target_path,
        },
        adapter_identity,
        document_identity: intent_ir.document_identity.clone(),
        lowering_status,
        residual_decisions,
        fsm: Some(fsm),
    })
}

fn build_signal_inventory(intent_ir: &IntentIr) -> Vec<FsmSignalCandidate> {
    let mut inventory = build_signal_inventory_map_from_surface(
        &intent_ir.interfaces,
        &intent_ir.decision_tree_fragments,
        &intent_ir.control_blocks,
    );

    if inventory.is_empty() {
        for behavior in &intent_ir.behaviors {
            register_signal_mentions(
                &mut inventory,
                &behavior.statement,
                &behavior.behavior_id,
                "behavior",
            );
        }

        for constraint in &intent_ir.constraints {
            register_signal_mentions(
                &mut inventory,
                &constraint.statement,
                &constraint.constraint_id,
                "constraint",
            );
        }

        for assumption in &intent_ir.assumptions {
            register_signal_mentions(
                &mut inventory,
                &assumption.statement,
                &assumption.assumption_id,
                "assumption",
            );
        }
    }

    inventory_to_signal_candidates(inventory)
}

fn build_signal_inventory_map_from_surface(
    interfaces: &[InterfaceRecord],
    decision_tree_fragments: &[DecisionTreeFragmentRecord],
    control_blocks: &[ControlBlockRecord],
) -> BTreeMap<String, SignalInventoryEvidence> {
    let mut inventory = BTreeMap::<String, SignalInventoryEvidence>::new();

    for interface in interfaces {
        if interface.signal_records.is_empty() {
            for signal_name in &interface.signals {
                register_canonical_signal(
                    &mut inventory,
                    signal_name,
                    None,
                    None,
                    &interface.interface_id,
                    "interface",
                    AutomationConfidence::Low,
                );
            }
            continue;
        }

        for signal in &interface.signal_records {
            register_canonical_signal(
                &mut inventory,
                &signal.signal_name,
                signal.direction_hint,
                signal.width_hint,
                &interface.interface_id,
                "interface",
                signal.automation_confidence,
            );
        }
    }

    for fragment in decision_tree_fragments {
        for signal_name in &fragment.referenced_signal_names {
            register_canonical_signal(
                &mut inventory,
                signal_name,
                None,
                None,
                &fragment.fragment_id,
                "control_fragment",
                fragment.automation_confidence,
            );
        }
    }

    for block in control_blocks {
        for signal_name in &block.referenced_signal_names {
            register_canonical_signal(
                &mut inventory,
                signal_name,
                None,
                None,
                &block.block_id,
                "control_block",
                block.automation_confidence,
            );
        }
    }

    inventory
}

fn inventory_to_signal_candidates(
    inventory: BTreeMap<String, SignalInventoryEvidence>,
) -> Vec<FsmSignalCandidate> {
    inventory
        .into_iter()
        .map(|(signal_name, evidence)| FsmSignalCandidate {
            signal_name,
            direction_hint: evidence.direction_hint,
            width_hint: evidence.width_hint,
            supporting_canonical_ids: evidence.supporting_canonical_ids.into_iter().collect(),
            mention_categories: evidence.mention_categories.into_iter().collect(),
            automation_confidence: evidence.automation_confidence,
        })
        .collect()
}

fn build_module_candidates(intent_ir: &IntentIr) -> Vec<FsmExplicitModuleCandidate> {
    intent_ir
        .explicit_modules
        .iter()
        .map(build_module_candidate)
        .collect()
}

fn build_module_candidate(module: &ExplicitModuleRecord) -> FsmExplicitModuleCandidate {
    let signal_inventory = inventory_to_signal_candidates(build_signal_inventory_map_from_surface(
        &module.interfaces,
        &module.decision_tree_fragments,
        &module.control_blocks,
    ));
    let state_candidates = build_state_candidates_from_records(&module.regular_states);
    let transition_candidates = build_transition_candidates_from_records(&module.state_transitions);
    let root_kind_decision = build_root_kind_decision(&state_candidates);
    let decision_tree_candidates = build_explicit_decision_tree_candidates(
        &module.module_name,
        &module.decision_tree_fragments,
    );
    let (renderability, renderable_module) = analyze_renderability(
        &signal_inventory,
        module.system_contract.as_ref(),
        &module.init_assignments,
        &module.symbol_definitions,
        &module.control_blocks,
        &state_candidates,
        &transition_candidates,
        &decision_tree_candidates,
        &root_kind_decision,
    );

    FsmExplicitModuleCandidate {
        module_name: module.module_name.clone(),
        declaration_order: module.declaration_order,
        root_kind_decision,
        signal_inventory,
        system_contract: module.system_contract.clone(),
        init_assignments: module.init_assignments.clone(),
        decision_tree_candidates,
        state_candidates,
        transition_candidates,
        renderability,
        renderable_module,
    }
}

fn build_top_candidates(
    intent_ir: &IntentIr,
    module_candidates: &[FsmExplicitModuleCandidate],
) -> Vec<FsmTopCandidate> {
    let module_candidates_by_name = module_candidates
        .iter()
        .map(|candidate| (candidate.module_name.clone(), candidate))
        .collect::<BTreeMap<_, _>>();

    intent_ir
        .explicit_tops
        .iter()
        .map(|top| build_top_candidate(top, &module_candidates_by_name))
        .collect()
}

fn build_top_candidate(
    top: &ExplicitTopRecord,
    module_candidates_by_name: &BTreeMap<String, &FsmExplicitModuleCandidate>,
) -> FsmTopCandidate {
    let children = top
        .children
        .iter()
        .map(|child| {
            let resolved_root_kind = module_candidates_by_name
                .get(&child.source_module_name)
                .and_then(|candidate| {
                    candidate
                        .renderability
                        .is_renderable
                        .then_some(candidate.root_kind_decision.selected_root_kind)
                });
            FsmTopChildCandidate {
                instance_name: child.instance_name.clone(),
                source_module_name: child.source_module_name.clone(),
                resolved_root_kind,
                declaration_order: child.declaration_order,
                supporting_canonical_ids: child.supporting_statement_ids.clone(),
                automation_confidence: child.automation_confidence,
            }
        })
        .collect::<Vec<_>>();
    let (renderability, renderable_top) =
        analyze_top_renderability(top, &children, module_candidates_by_name);

    FsmTopCandidate {
        top_name: top.top_name.clone(),
        declaration_order: top.declaration_order,
        ports: top.ports.clone(),
        children,
        links: top.links.clone(),
        renderability,
        renderable_top,
    }
}

fn build_state_candidates(intent_ir: &IntentIr) -> Vec<FsmStateCandidate> {
    build_state_candidates_from_records(&intent_ir.regular_states)
}

fn build_transition_candidates(intent_ir: &IntentIr) -> Vec<FsmTransitionCandidate> {
    build_transition_candidates_from_records(&intent_ir.state_transitions)
}

fn build_state_candidates_from_records(
    regular_states: &[crate::ir::semantic::RegularStateRecord],
) -> Vec<FsmStateCandidate> {
    regular_states
        .iter()
        .map(|state| FsmStateCandidate {
            state_id: state.state_id.clone(),
            state_name: state.state_name.clone(),
            is_initial: state.is_initial,
            declaration_order: state.declaration_order,
            supporting_canonical_ids: state.supporting_statement_ids.clone(),
            automation_confidence: state.automation_confidence,
        })
        .collect()
}

fn build_transition_candidates_from_records(
    state_transitions: &[StateTransitionRecord],
) -> Vec<FsmTransitionCandidate> {
    state_transitions
        .iter()
        .map(|transition| FsmTransitionCandidate {
            transition_id: transition.transition_id.clone(),
            source_state: transition.source_state.clone(),
            target_state: transition.target_state.clone(),
            guard: transition.guard.clone(),
            declaration_order: transition.declaration_order,
            supporting_canonical_ids: transition.supporting_statement_ids.clone(),
            automation_confidence: transition.automation_confidence,
        })
        .collect()
}

fn build_root_kind_decision(state_candidates: &[FsmStateCandidate]) -> FsmRootKindDecision {
    if !state_candidates.is_empty() {
        return FsmRootKindDecision {
            selected_root_kind: FsmRootKind::Fsm,
            deferred_root_kinds: vec![FsmRootKind::Top],
            automation_confidence: fold_automation_confidence(
                state_candidates
                    .iter()
                    .map(|candidate| candidate.automation_confidence),
            ),
            rationale: "the current IntentIR carries explicit regular-state facts, so the adapter can target a true `?fsm:name` root without inventing state identity".to_string(),
        };
    }

    FsmRootKindDecision {
        selected_root_kind: FsmRootKind::Dt,
        deferred_root_kinds: vec![
            FsmRootKind::Fsm,
            FsmRootKind::Top,
        ],
        automation_confidence: AutomationConfidence::Medium,
        rationale: "the current IntentIR does not yet carry explicit regular-state facts, so the adapter stays on a standalone `?dt:name` plan rather than inventing FSM or composition semantics".to_string(),
    }
}

fn build_decision_tree_candidates(
    intent_ir: &IntentIr,
    signal_inventory: &[FsmSignalCandidate],
) -> Vec<FsmDecisionTreeCandidate> {
    if !intent_ir.decision_tree_fragments.is_empty() {
        return build_explicit_decision_tree_candidates(
            &intent_ir.document_identity.display_name,
            &intent_ir.decision_tree_fragments,
        );
    }
    if intent_ir.behaviors.is_empty()
        && intent_ir.constraints.is_empty()
        && intent_ir.assumptions.is_empty()
    {
        return Vec::new();
    }

    vec![FsmDecisionTreeCandidate {
        candidate_id: "dt_primary_intent_cone".to_string(),
        summary: format!(
            "primary DT candidate aggregates {} behaviors, {} constraints, and {} assumptions for {}",
            intent_ir.behaviors.len(),
            intent_ir.constraints.len(),
            intent_ir.assumptions.len(),
            intent_ir.document_identity.display_name
        ),
        supporting_fragment_ids: Vec::new(),
        blocks: Vec::new(),
        referenced_signal_names: signal_inventory
            .iter()
            .map(|signal| signal.signal_name.clone())
            .collect(),
        automation_confidence: if intent_ir.behaviors.is_empty() {
            AutomationConfidence::Low
        } else {
            AutomationConfidence::Medium
        },
    }]
}

fn build_explicit_decision_tree_candidates(
    scope_name: &str,
    decision_tree_fragments: &[DecisionTreeFragmentRecord],
) -> Vec<FsmDecisionTreeCandidate> {
    if decision_tree_fragments.is_empty() {
        return Vec::new();
    }

    let referenced_signal_names = decision_tree_fragments
        .iter()
        .flat_map(|fragment| fragment.referenced_signal_names.iter().cloned())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    vec![FsmDecisionTreeCandidate {
        candidate_id: "dt_primary_intent_cone".to_string(),
        summary: format!(
            "primary canonical control candidate aggregates {} typed block(s) for {}",
            decision_tree_fragments.len(),
            scope_name
        ),
        supporting_fragment_ids: decision_tree_fragments
            .iter()
            .map(|fragment| fragment.fragment_id.clone())
            .collect(),
        blocks: decision_tree_fragments.to_vec(),
        referenced_signal_names,
        automation_confidence: fold_automation_confidence(
            decision_tree_fragments
                .iter()
                .map(|fragment| fragment.automation_confidence),
        ),
    }]
}

fn select_fsm_surface(
    intent_ir: &IntentIr,
    direct_surface: SelectedFsmSurface,
    module_candidates: &[FsmExplicitModuleCandidate],
    top_candidates: &[FsmTopCandidate],
) -> SelectedFsmSurface {
    if top_candidates.len() == 1 {
        let top_candidate = &top_candidates[0];
        return SelectedFsmSurface {
            root_name: top_candidate.top_name.clone(),
            root_kind_decision: build_top_root_kind_decision(top_candidate),
            signal_inventory: build_top_signal_inventory(&top_candidate.ports),
            system_contract: None,
            init_assignments: Vec::new(),
            decision_tree_candidates: Vec::new(),
            state_candidates: Vec::new(),
            transition_candidates: Vec::new(),
            renderability: top_candidate.renderability.clone(),
            renderable_module: None,
            renderable_document: top_candidate.renderability.is_renderable.then(|| {
                FsmRenderableSourceDocument {
                    top_root: top_candidate.renderable_top.clone(),
                    direct_roots: renderable_modules_for_top(top_candidate, module_candidates),
                }
            }),
        };
    }

    if top_candidates.len() > 1 {
        let first_top = &top_candidates[0];
        return SelectedFsmSurface {
            root_name: first_top.top_name.clone(),
            root_kind_decision: FsmRootKindDecision {
                selected_root_kind: FsmRootKind::Top,
                deferred_root_kinds: vec![FsmRootKind::Dt, FsmRootKind::Fsm],
                automation_confidence: AutomationConfidence::Low,
                rationale: "the current IntentIR carries multiple explicit top candidates, so broader-root lowering stays blocked until one canonical top is selected".to_string(),
            },
            signal_inventory: build_top_signal_inventory(&first_top.ports),
            system_contract: None,
            init_assignments: Vec::new(),
            decision_tree_candidates: Vec::new(),
            state_candidates: Vec::new(),
            transition_candidates: Vec::new(),
            renderability: FsmRenderability {
                is_renderable: false,
                blocking_reasons: vec![
                    "The current `.fsm` composition slice supports exactly one explicit top candidate per document.".to_string(),
                ],
                required_canonical_enrichments: vec![
                    "select one explicit top composition per document before lowering `?top:name`"
                        .to_string(),
                ],
            },
            renderable_module: None,
            renderable_document: None,
        };
    }

    if !document_has_explicit_direct_surface(intent_ir) && module_candidates.len() == 1 {
        return module_candidate_to_surface(&module_candidates[0]);
    }

    direct_surface
}

fn document_has_explicit_direct_surface(intent_ir: &IntentIr) -> bool {
    !intent_ir.interfaces.is_empty()
        || intent_ir.system_contract.is_some()
        || !intent_ir.init_assignments.is_empty()
        || !intent_ir.regular_states.is_empty()
        || !intent_ir.state_transitions.is_empty()
        || !intent_ir.decision_tree_fragments.is_empty()
        || !intent_ir.symbol_definitions.is_empty()
        || !intent_ir.control_blocks.is_empty()
}

fn module_candidate_to_surface(candidate: &FsmExplicitModuleCandidate) -> SelectedFsmSurface {
    SelectedFsmSurface {
        root_name: candidate.module_name.clone(),
        root_kind_decision: candidate.root_kind_decision.clone(),
        signal_inventory: candidate.signal_inventory.clone(),
        system_contract: candidate.system_contract.clone(),
        init_assignments: candidate.init_assignments.clone(),
        decision_tree_candidates: candidate.decision_tree_candidates.clone(),
        state_candidates: candidate.state_candidates.clone(),
        transition_candidates: candidate.transition_candidates.clone(),
        renderability: candidate.renderability.clone(),
        renderable_module: candidate.renderable_module.clone(),
        renderable_document: candidate.renderable_module.clone().map(|module| {
            FsmRenderableSourceDocument {
                top_root: None,
                direct_roots: vec![FsmRenderableNamedModule {
                    module_name: candidate.module_name.clone(),
                    root_kind: candidate.root_kind_decision.selected_root_kind,
                    module,
                }],
            }
        }),
    }
}

fn build_top_root_kind_decision(top_candidate: &FsmTopCandidate) -> FsmRootKindDecision {
    let confidence = fold_automation_confidence(
        top_candidate
            .ports
            .iter()
            .map(|port| port.automation_confidence)
            .chain(
                top_candidate
                    .children
                    .iter()
                    .map(|child| child.automation_confidence),
            )
            .chain(
                top_candidate
                    .links
                    .iter()
                    .map(|link| link.automation_confidence),
            ),
    );

    FsmRootKindDecision {
        selected_root_kind: FsmRootKind::Top,
        deferred_root_kinds: vec![FsmRootKind::Dt, FsmRootKind::Fsm],
        automation_confidence: confidence,
        rationale: "the current IntentIR carries explicit top ports, child module references, and wiring links, so the adapter can target a true `?top:name` root without inventing composition structure".to_string(),
    }
}

fn build_top_signal_inventory(ports: &[ExplicitTopPortRecord]) -> Vec<FsmSignalCandidate> {
    ports
        .iter()
        .map(|port| FsmSignalCandidate {
            signal_name: port.port_name.clone(),
            direction_hint: Some(port.direction_hint),
            width_hint: port.width_hint,
            supporting_canonical_ids: port.supporting_statement_ids.clone(),
            mention_categories: vec!["top_port".to_string()],
            automation_confidence: port.automation_confidence,
        })
        .collect()
}

fn renderable_modules_for_top(
    top_candidate: &FsmTopCandidate,
    module_candidates: &[FsmExplicitModuleCandidate],
) -> Vec<FsmRenderableNamedModule> {
    let modules_by_name = module_candidates
        .iter()
        .map(|candidate| (candidate.module_name.clone(), candidate))
        .collect::<BTreeMap<_, _>>();
    let mut seen = BTreeSet::new();
    let mut renderable_modules = Vec::new();

    for child in &top_candidate.children {
        if !seen.insert(child.source_module_name.clone()) {
            continue;
        }
        let Some(candidate) = modules_by_name.get(&child.source_module_name) else {
            continue;
        };
        let Some(renderable_module) = candidate.renderable_module.clone() else {
            continue;
        };
        renderable_modules.push(FsmRenderableNamedModule {
            module_name: child.source_module_name.clone(),
            root_kind: candidate.root_kind_decision.selected_root_kind,
            module: renderable_module,
        });
    }

    renderable_modules
}

fn analyze_top_renderability(
    top: &ExplicitTopRecord,
    children: &[FsmTopChildCandidate],
    module_candidates_by_name: &BTreeMap<String, &FsmExplicitModuleCandidate>,
) -> (FsmRenderability, Option<FsmRenderableTopRoot>) {
    let mut blocking_reasons = Vec::new();
    let mut required_canonical_enrichments = BTreeSet::new();

    if top.ports.is_empty() {
        push_unique_message(
            &mut blocking_reasons,
            "Top-root lowering requires at least one explicit top port record.",
        );
        required_canonical_enrichments
            .insert("carry explicit top-port records before lowering `?top:name`".to_string());
    }

    if children.is_empty() {
        push_unique_message(
            &mut blocking_reasons,
            "Top-root lowering requires at least one explicit child module reference.",
        );
        required_canonical_enrichments.insert(
            "carry explicit child-module references before lowering `?top:name`".to_string(),
        );
    }

    if children.len() > 1 && top.links.is_empty() {
        push_unique_message(
            &mut blocking_reasons,
            "Multi-child top-root lowering requires explicit link records in the first composition slice.",
        );
        required_canonical_enrichments.insert(
            "carry explicit top-link records for multi-child compositions before lowering `?top:name`"
                .to_string(),
        );
    }

    let mut seen_top_ports = BTreeSet::new();
    let mut top_ports_by_name = BTreeMap::new();
    for port in &top.ports {
        if !seen_top_ports.insert(port.port_name.clone()) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top port `{}` is declared more than once in the current explicit composition slice.",
                    port.port_name
                ),
            );
            required_canonical_enrichments.insert(
                "deduplicate explicit top-port records before lowering `?top:name`".to_string(),
            );
        }
        top_ports_by_name.insert(
            port.port_name.clone(),
            RenderableEndpointPort {
                direction_hint: port.direction_hint,
                width_hint: port.width_hint,
            },
        );
    }

    let mut child_ports_by_instance =
        BTreeMap::<String, BTreeMap<String, RenderableEndpointPort>>::new();
    let mut seen_child_instances = BTreeSet::new();
    let mut renderable_children = Vec::new();
    for child in children {
        if !seen_child_instances.insert(child.instance_name.clone()) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top child instance `{}` is declared more than once in the current explicit composition slice.",
                    child.instance_name
                ),
            );
            required_canonical_enrichments.insert(
                "deduplicate explicit child-instance records before lowering `?top:name`"
                    .to_string(),
            );
        }

        let Some(module_candidate) = module_candidates_by_name.get(&child.source_module_name)
        else {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top child `{}` references missing explicit module `{}`.",
                    child.instance_name, child.source_module_name
                ),
            );
            required_canonical_enrichments.insert(
                "declare every top child source as an explicit module before lowering `?top:name`"
                    .to_string(),
            );
            continue;
        };

        if !module_candidate.renderability.is_renderable {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top child `{}` references explicit module `{}` whose direct DT/FSM lowering is still blocked.",
                    child.instance_name, child.source_module_name
                ),
            );
            required_canonical_enrichments.insert(
                "keep top children aligned with renderable explicit modules before lowering `?top:name`"
                    .to_string(),
            );
            continue;
        }

        let Some(resolved_root_kind) = child.resolved_root_kind else {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top child `{}` does not resolve to a renderable DT/FSM child kind in the first composition slice.",
                    child.instance_name
                ),
            );
            required_canonical_enrichments.insert(
                "keep first-slice top children aligned with renderable DT/FSM module roots"
                    .to_string(),
            );
            continue;
        };

        if !matches!(resolved_root_kind, FsmRootKind::Dt | FsmRootKind::Fsm) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top child `{}` resolves to unsupported root kind `?{}:name` for the first composition slice.",
                    child.instance_name,
                    resolved_root_kind.as_str()
                ),
            );
            required_canonical_enrichments.insert(
                "keep first-slice top children aligned with DT/FSM module roots".to_string(),
            );
            continue;
        }

        child_ports_by_instance.insert(
            child.instance_name.clone(),
            renderable_ports_for_module_candidate(module_candidate),
        );
        renderable_children.push(FsmRenderableTopChild {
            instance_name: child.instance_name.clone(),
            source_module_name: child.source_module_name.clone(),
            child_root_kind: resolved_root_kind,
        });
    }

    for link in &top.links {
        let Some((source_port, source_is_top)) =
            resolve_top_link_endpoint(&link.source, &top_ports_by_name, &child_ports_by_instance)
        else {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top link source `{}` does not resolve to an explicit top port or child interface signal.",
                    render_top_link_endpoint(&link.source)
                ),
            );
            required_canonical_enrichments.insert(
                "declare every top-link source endpoint explicitly before lowering `?top:name`"
                    .to_string(),
            );
            continue;
        };
        let Some((target_port, target_is_top)) =
            resolve_top_link_endpoint(&link.target, &top_ports_by_name, &child_ports_by_instance)
        else {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top link target `{}` does not resolve to an explicit top port or child interface signal.",
                    render_top_link_endpoint(&link.target)
                ),
            );
            required_canonical_enrichments.insert(
                "declare every top-link target endpoint explicitly before lowering `?top:name`"
                    .to_string(),
            );
            continue;
        };

        if source_is_top {
            if !matches!(source_port.direction_hint, InterfaceSignalDirection::Input) {
                push_unique_message(
                    &mut blocking_reasons,
                    &format!(
                        "Top link source `{}` must be an explicit top input when it originates from the top boundary.",
                        render_top_link_endpoint(&link.source)
                    ),
                );
            }
        } else if !matches!(source_port.direction_hint, InterfaceSignalDirection::Output) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top link source `{}` must resolve to an explicit child output.",
                    render_top_link_endpoint(&link.source)
                ),
            );
        }

        if target_is_top {
            if !matches!(target_port.direction_hint, InterfaceSignalDirection::Output) {
                push_unique_message(
                    &mut blocking_reasons,
                    &format!(
                        "Top link target `{}` must be an explicit top output when it terminates at the top boundary.",
                        render_top_link_endpoint(&link.target)
                    ),
                );
            }
        } else if !matches!(target_port.direction_hint, InterfaceSignalDirection::Input) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Top link target `{}` must resolve to an explicit child input.",
                    render_top_link_endpoint(&link.target)
                ),
            );
        }

        if let (Some(source_width), Some(target_width)) =
            (source_port.width_hint, target_port.width_hint)
        {
            if source_width != target_width {
                push_unique_message(
                    &mut blocking_reasons,
                    &format!(
                        "Top link `{}` -> `{}` connects width {} to width {}.",
                        render_top_link_endpoint(&link.source),
                        render_top_link_endpoint(&link.target),
                        source_width,
                        target_width
                    ),
                );
                required_canonical_enrichments
                    .insert("keep first-slice top-link endpoints width-compatible".to_string());
            }
        }
    }

    let renderability = FsmRenderability {
        is_renderable: blocking_reasons.is_empty(),
        blocking_reasons,
        required_canonical_enrichments: required_canonical_enrichments.into_iter().collect(),
    };
    let renderable_top = renderability.is_renderable.then_some(FsmRenderableTopRoot {
        top_name: top.top_name.clone(),
        ports: top.ports.clone(),
        children: renderable_children,
        links: top.links.clone(),
    });

    (renderability, renderable_top)
}

fn renderable_ports_for_module_candidate(
    candidate: &FsmExplicitModuleCandidate,
) -> BTreeMap<String, RenderableEndpointPort> {
    let mut ports = candidate
        .signal_inventory
        .iter()
        .filter_map(|signal| {
            signal.direction_hint.map(|direction_hint| {
                (
                    signal.signal_name.clone(),
                    RenderableEndpointPort {
                        direction_hint,
                        width_hint: signal.width_hint,
                    },
                )
            })
        })
        .collect::<BTreeMap<_, _>>();

    if let Some(system_contract) = candidate.system_contract.as_ref() {
        ports
            .entry(system_contract.clock_signal.clone())
            .or_insert(RenderableEndpointPort {
                direction_hint: InterfaceSignalDirection::Input,
                width_hint: Some(1),
            });
        ports
            .entry(system_contract.reset_signal.clone())
            .or_insert(RenderableEndpointPort {
                direction_hint: InterfaceSignalDirection::Input,
                width_hint: Some(1),
            });
    }

    ports
}

fn resolve_top_link_endpoint(
    endpoint: &ExplicitTopLinkEndpoint,
    top_ports_by_name: &BTreeMap<String, RenderableEndpointPort>,
    child_ports_by_instance: &BTreeMap<String, BTreeMap<String, RenderableEndpointPort>>,
) -> Option<(RenderableEndpointPort, bool)> {
    if let Some(instance_name) = endpoint.instance_name.as_deref() {
        return child_ports_by_instance
            .get(instance_name)?
            .get(&endpoint.signal_name)
            .copied()
            .map(|port| (port, false));
    }

    top_ports_by_name
        .get(&endpoint.signal_name)
        .copied()
        .map(|port| (port, true))
}

fn render_top_link_endpoint(endpoint: &ExplicitTopLinkEndpoint) -> String {
    match endpoint.instance_name.as_deref() {
        Some(instance_name) => format!("{instance_name}.{}", endpoint.signal_name),
        None => endpoint.signal_name.clone(),
    }
}

fn analyze_renderability(
    signal_inventory: &[FsmSignalCandidate],
    system_contract: Option<&SystemContractRecord>,
    init_assignments: &[InitAssignmentRecord],
    symbol_definitions: &[SymbolDefinitionRecord],
    control_blocks: &[ControlBlockRecord],
    state_candidates: &[FsmStateCandidate],
    transition_candidates: &[FsmTransitionCandidate],
    decision_tree_candidates: &[FsmDecisionTreeCandidate],
    root_kind_decision: &FsmRootKindDecision,
) -> (FsmRenderability, Option<FsmRenderableModule>) {
    match root_kind_decision.selected_root_kind {
        FsmRootKind::Dt => analyze_dt_root_renderability(
            signal_inventory,
            system_contract,
            init_assignments,
            symbol_definitions,
            control_blocks,
            decision_tree_candidates,
        ),
        FsmRootKind::Fsm => analyze_fsm_root_renderability(
            signal_inventory,
            system_contract,
            init_assignments,
            symbol_definitions,
            control_blocks,
            state_candidates,
            transition_candidates,
            decision_tree_candidates,
        ),
        _ => (
            FsmRenderability {
                is_renderable: false,
                blocking_reasons: vec![
                    "This adapter slice only supports explicit standalone `?dt:name` and `?fsm:name` roots.".to_string(),
                ],
                required_canonical_enrichments: vec![
                    "keep composition roots deferred until the canonical model carries explicit module/top structure".to_string(),
                ],
            },
            None,
        ),
    }
}

fn analyze_dt_root_renderability(
    signal_inventory: &[FsmSignalCandidate],
    system_contract: Option<&SystemContractRecord>,
    init_assignments: &[InitAssignmentRecord],
    symbol_definitions: &[SymbolDefinitionRecord],
    control_blocks: &[ControlBlockRecord],
    decision_tree_candidates: &[FsmDecisionTreeCandidate],
) -> (FsmRenderability, Option<FsmRenderableModule>) {
    let mut blocking_reasons = Vec::new();
    let mut required_canonical_enrichments = BTreeSet::new();

    let signals_by_name: BTreeMap<String, &FsmSignalCandidate> = signal_inventory
        .iter()
        .map(|signal| (signal.signal_name.clone(), signal))
        .collect();
    let mut size_entries = BTreeMap::<String, FsmRenderableSizeEntry>::new();
    let mut driven_outputs = BTreeSet::new();
    let mut sequential_targets = BTreeSet::new();

    validate_symbol_definitions_renderability(
        symbol_definitions,
        &signals_by_name,
        &mut size_entries,
        &mut blocking_reasons,
        &mut required_canonical_enrichments,
    );
    let effective_control_blocks = if control_blocks.is_empty() {
        fallback_control_blocks_from_candidates(decision_tree_candidates, &BTreeSet::new())
    } else {
        control_blocks.to_vec()
    };
    let standalone_blocks = collect_renderable_dt_blocks(
        &effective_control_blocks,
        &signals_by_name,
        &mut size_entries,
        &mut driven_outputs,
        &mut sequential_targets,
        &mut blocking_reasons,
        &mut required_canonical_enrichments,
    );
    if standalone_blocks.is_empty() {
        push_unique_message(
            &mut blocking_reasons,
            "IntentIR does not yet carry standalone decision-tree control blocks that can be lowered safely into `.fsm`.",
        );
        required_canonical_enrichments.insert(
            "promote canonical standalone control blocks before lowering a `?dt:name` root"
                .to_string(),
        );
    }

    let requires_sequential_support =
        !sequential_targets.is_empty() || !init_assignments.is_empty();
    if let Some(system_contract) = system_contract {
        validate_system_contract_renderability(
            system_contract,
            &signals_by_name,
            &mut blocking_reasons,
            &mut required_canonical_enrichments,
        );
    } else if requires_sequential_support {
        push_unique_message(
            &mut blocking_reasons,
            "Sequential standalone `.fsm` lowering requires an explicit canonical system contract with clock and reset facts.",
        );
        required_canonical_enrichments.insert(
            "promote backend-neutral clock/reset system-contract facts before lowering sequential DT control"
                .to_string(),
        );
    }

    let init_targets = init_assignments
        .iter()
        .map(|assignment| assignment.target_signal.clone())
        .collect::<BTreeSet<_>>();
    for target_signal in &sequential_targets {
        if !init_targets.contains(target_signal) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Sequential target `{}` is missing an explicit canonical init assignment required for standalone `.fsm` lowering.",
                    target_signal
                ),
            );
            required_canonical_enrichments.insert(
                "promote backend-neutral init assignments for every sequentially driven standalone DT target"
                    .to_string(),
            );
        }
    }

    for init_assignment in init_assignments {
        validate_init_assignment_renderability(
            init_assignment,
            &signals_by_name,
            &mut size_entries,
            &mut blocking_reasons,
            &mut required_canonical_enrichments,
        );

        if !sequential_targets.contains(&init_assignment.target_signal) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Init assignment target `{}` is not driven by any sequential DT action in the current standalone slice.",
                    init_assignment.target_signal
                ),
            );
            required_canonical_enrichments.insert(
                "keep first-slice init assignments aligned with explicit sequential DT targets"
                    .to_string(),
            );
        }
    }

    for size_entry in size_entries.values() {
        if matches!(size_entry.direction_hint, InterfaceSignalDirection::Output)
            && !driven_outputs.contains(&size_entry.signal_name)
        {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Declared output signal `{}` is not driven by any typed control action.",
                    size_entry.signal_name
                ),
            );
            required_canonical_enrichments.insert(
                "keep canonical output roles aligned with explicit driving actions".to_string(),
            );
        }
    }

    let renderability = FsmRenderability {
        is_renderable: blocking_reasons.is_empty(),
        blocking_reasons,
        required_canonical_enrichments: required_canonical_enrichments.into_iter().collect(),
    };

    let renderable_module = renderability.is_renderable.then_some(FsmRenderableModule {
        system_contract: system_contract.cloned(),
        size_entries: size_entries.into_values().collect(),
        symbol_definitions: symbol_definitions.to_vec(),
        init_assignments: init_assignments.to_vec(),
        states: Vec::new(),
        blocks: standalone_blocks,
    });

    (renderability, renderable_module)
}

fn analyze_fsm_root_renderability(
    signal_inventory: &[FsmSignalCandidate],
    system_contract: Option<&SystemContractRecord>,
    init_assignments: &[InitAssignmentRecord],
    symbol_definitions: &[SymbolDefinitionRecord],
    control_blocks: &[ControlBlockRecord],
    state_candidates: &[FsmStateCandidate],
    transition_candidates: &[FsmTransitionCandidate],
    decision_tree_candidates: &[FsmDecisionTreeCandidate],
) -> (FsmRenderability, Option<FsmRenderableModule>) {
    let mut blocking_reasons = Vec::new();
    let mut required_canonical_enrichments = BTreeSet::new();
    let state_names = state_candidates
        .iter()
        .map(|state| state.state_name.clone())
        .collect::<BTreeSet<_>>();

    if state_candidates.is_empty() {
        push_unique_message(
            &mut blocking_reasons,
            "FSM-root lowering requires explicit canonical regular-state records.",
        );
        required_canonical_enrichments.insert(
            "promote explicit regular-state records before lowering a true `?fsm:name` root"
                .to_string(),
        );
    }
    let effective_control_blocks = if control_blocks.is_empty() {
        fallback_control_blocks_from_candidates(decision_tree_candidates, &state_names)
    } else {
        control_blocks.to_vec()
    };
    if effective_control_blocks.is_empty() && transition_candidates.is_empty() {
        push_unique_message(
            &mut blocking_reasons,
            "IntentIR does not yet carry structured state-body or standalone control blocks, so a true `.fsm` root cannot be emitted safely.",
        );
        required_canonical_enrichments.insert(
            "promote canonical state-body and standalone control blocks before lowering a true `?fsm:name` root".to_string(),
        );
    }

    let signals_by_name: BTreeMap<String, &FsmSignalCandidate> = signal_inventory
        .iter()
        .map(|signal| (signal.signal_name.clone(), signal))
        .collect();
    let mut size_entries = BTreeMap::<String, FsmRenderableSizeEntry>::new();
    let mut driven_outputs = BTreeSet::new();
    let mut sequential_targets = BTreeSet::new();
    validate_symbol_definitions_renderability(
        symbol_definitions,
        &signals_by_name,
        &mut size_entries,
        &mut blocking_reasons,
        &mut required_canonical_enrichments,
    );
    let (mut state_blocks_by_name, standalone_blocks) = collect_renderable_fsm_blocks(
        &effective_control_blocks,
        &state_names,
        &signals_by_name,
        &mut size_entries,
        &mut driven_outputs,
        &mut sequential_targets,
        &mut blocking_reasons,
        &mut required_canonical_enrichments,
    );

    if let Some(system_contract) = system_contract {
        validate_system_contract_renderability(
            system_contract,
            &signals_by_name,
            &mut blocking_reasons,
            &mut required_canonical_enrichments,
        );
    } else {
        push_unique_message(
            &mut blocking_reasons,
            "FSM-root lowering requires an explicit canonical system contract with clock and reset facts.",
        );
        required_canonical_enrichments.insert(
            "promote backend-neutral clock/reset system-contract facts before lowering a true `?fsm:name` root".to_string(),
        );
    }

    let initial_state_count = state_candidates
        .iter()
        .filter(|state| state.is_initial)
        .count();
    if initial_state_count != 1 {
        push_unique_message(
            &mut blocking_reasons,
            "FSM-root lowering requires exactly one explicit initial regular state in the canonical state graph.",
        );
        required_canonical_enrichments.insert(
            "mark exactly one canonical regular state as initial before lowering a true `?fsm:name` root".to_string(),
        );
    }

    let mut transitions_by_source = BTreeMap::<String, Vec<StateTransitionRecord>>::new();
    for transition in transition_candidates {
        validate_transition_renderability(
            transition,
            &state_names,
            &signals_by_name,
            &mut size_entries,
            &mut blocking_reasons,
            &mut required_canonical_enrichments,
        );
        if state_names.contains(&transition.source_state)
            && state_names.contains(&transition.target_state)
        {
            transitions_by_source
                .entry(transition.source_state.clone())
                .or_default()
                .push(StateTransitionRecord {
                    transition_id: transition.transition_id.clone(),
                    source_state: transition.source_state.clone(),
                    target_state: transition.target_state.clone(),
                    guard: transition.guard.clone(),
                    declaration_order: transition.declaration_order,
                    supporting_statement_ids: transition.supporting_canonical_ids.clone(),
                    automation_confidence: transition.automation_confidence,
                });
        }
    }

    let init_targets = init_assignments
        .iter()
        .map(|assignment| assignment.target_signal.clone())
        .collect::<BTreeSet<_>>();
    for target_signal in &sequential_targets {
        if !init_targets.contains(target_signal) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Sequential target `{}` is missing an explicit canonical init assignment required for `.fsm` lowering.",
                    target_signal
                ),
            );
            required_canonical_enrichments.insert(
                "promote backend-neutral init assignments for every sequentially driven FSM output"
                    .to_string(),
            );
        }
    }

    for init_assignment in init_assignments {
        validate_init_assignment_renderability(
            init_assignment,
            &signals_by_name,
            &mut size_entries,
            &mut blocking_reasons,
            &mut required_canonical_enrichments,
        );

        if !sequential_targets.contains(&init_assignment.target_signal) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Init assignment target `{}` is not driven by any sequential FSM-state action in the current slice.",
                    init_assignment.target_signal
                ),
            );
            required_canonical_enrichments.insert(
                "keep first-slice init assignments aligned with explicit sequential FSM outputs"
                    .to_string(),
            );
        }
    }

    for size_entry in size_entries.values() {
        if matches!(size_entry.direction_hint, InterfaceSignalDirection::Output)
            && !driven_outputs.contains(&size_entry.signal_name)
        {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Declared output signal `{}` is not driven by any typed FSM-state action.",
                    size_entry.signal_name
                ),
            );
            required_canonical_enrichments.insert(
                "keep canonical output roles aligned with explicit FSM-state actions".to_string(),
            );
        }
    }

    let mut ordered_states = state_candidates.iter().collect::<Vec<_>>();
    ordered_states.sort_by_key(|state| (!state.is_initial, state.declaration_order));
    let mut renderable_states = Vec::new();
    for state in ordered_states {
        let state_blocks = state_blocks_by_name
            .remove(&state.state_name)
            .unwrap_or_default();
        let mut state_transitions = transitions_by_source
            .remove(&state.state_name)
            .unwrap_or_default();
        state_transitions.sort_by_key(|transition| transition.declaration_order);
        if state_blocks.is_empty() && state_transitions.is_empty() {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Regular state `{}` has no typed actions or transitions to lower into `.fsm`.",
                    state.state_name
                ),
            );
            required_canonical_enrichments.insert(
                "keep each canonical regular state anchored to typed actions or explicit transitions"
                    .to_string(),
            );
        }
        renderable_states.push(FsmRenderableState {
            state_name: state.state_name.clone(),
            is_initial: state.is_initial,
            blocks: state_blocks,
            transitions: state_transitions,
        });
    }

    let renderability = FsmRenderability {
        is_renderable: blocking_reasons.is_empty(),
        blocking_reasons,
        required_canonical_enrichments: required_canonical_enrichments.into_iter().collect(),
    };

    let renderable_module = renderability.is_renderable.then_some(FsmRenderableModule {
        system_contract: system_contract.cloned(),
        size_entries: size_entries.into_values().collect(),
        symbol_definitions: symbol_definitions.to_vec(),
        init_assignments: init_assignments.to_vec(),
        states: renderable_states,
        blocks: standalone_blocks,
    });

    (renderability, renderable_module)
}

fn fallback_control_blocks_from_candidates(
    decision_tree_candidates: &[FsmDecisionTreeCandidate],
    state_names: &BTreeSet<String>,
) -> Vec<ControlBlockRecord> {
    decision_tree_candidates
        .iter()
        .flat_map(|candidate| candidate.blocks.iter())
        .enumerate()
        .map(|(declaration_order, block)| {
            fallback_control_block_from_fragment(
                block,
                state_names,
                u32::try_from(declaration_order).expect("control block count should fit in u32"),
            )
        })
        .collect()
}

fn fallback_control_block_from_fragment(
    block: &DecisionTreeFragmentRecord,
    state_names: &BTreeSet<String>,
    declaration_order: u32,
) -> ControlBlockRecord {
    let role = if state_names.contains(&block.block_name) {
        ControlBlockRole::StateBody
    } else {
        ControlBlockRole::StandaloneDecisionTree
    };

    ControlBlockRecord {
        block_id: format!("fallback_{}", block.fragment_id),
        block_name: block.block_name.clone(),
        role,
        declaration_order,
        selector: None,
        branches: vec![super::semantic::ControlBranchRecord {
            branch_id: format!("fallback_branch_{}", block.fragment_id),
            declaration_order: 0,
            predicate: block
                .guard
                .as_ref()
                .map(decision_tree_guard_to_control_expression),
            actions: block
                .actions
                .iter()
                .map(decision_tree_action_to_control_action)
                .collect(),
            supporting_statement_ids: block.supporting_statement_ids.clone(),
            automation_confidence: block.automation_confidence,
        }],
        referenced_signal_names: block.referenced_signal_names.clone(),
        supporting_statement_ids: block.supporting_statement_ids.clone(),
        automation_confidence: block.automation_confidence,
    }
}

fn decision_tree_guard_to_control_expression(
    guard: &DecisionTreeGuardRecord,
) -> ControlExpressionRecord {
    match guard {
        DecisionTreeGuardRecord::SignalIsHigh { signal_name } => {
            ControlExpressionRecord::Reference {
                reference: simple_signal_reference(signal_name),
            }
        }
        DecisionTreeGuardRecord::Comparison {
            left_signal,
            operator,
            right,
        } => ControlExpressionRecord::Binary {
            operator: match operator {
                DecisionTreeComparisonOperator::Eq => ControlBinaryOperator::Eq,
                DecisionTreeComparisonOperator::NotEq => ControlBinaryOperator::NotEq,
            },
            left: Box::new(ControlExpressionRecord::Reference {
                reference: simple_signal_reference(left_signal),
            }),
            right: Box::new(decision_tree_value_to_control_expression(right)),
        },
    }
}

fn decision_tree_action_to_control_action(
    action: &DecisionTreeActionRecord,
) -> ControlActionRecord {
    match action {
        DecisionTreeActionRecord::Assign {
            target_signal,
            assignment_kind,
            value,
        } => ControlActionRecord::Assign {
            target: ControlAssignmentTargetRecord {
                signal_name: target_signal.clone(),
                exposed_public_output: false,
            },
            assignment_kind: *assignment_kind,
            dual_output: None,
            value: decision_tree_value_to_control_expression(value),
        },
    }
}

fn decision_tree_value_to_control_expression(
    value: &DecisionTreeValueRecord,
) -> ControlExpressionRecord {
    match value {
        DecisionTreeValueRecord::SignalRef { signal_name } => ControlExpressionRecord::Reference {
            reference: simple_signal_reference(signal_name),
        },
        DecisionTreeValueRecord::Literal { literal } => ControlExpressionRecord::Literal {
            literal: literal.clone(),
        },
    }
}

fn simple_signal_reference(signal_name: &str) -> crate::ir::semantic::ControlReferenceRecord {
    crate::ir::semantic::ControlReferenceRecord {
        base_name: signal_name.to_string(),
        kind_hint: ControlReferenceKind::Signal,
        suffixes: Vec::new(),
        exposed_public_output: false,
    }
}

fn validate_symbol_definitions_renderability(
    symbol_definitions: &[SymbolDefinitionRecord],
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    for definition in symbol_definitions {
        match definition.kind {
            SymbolDefinitionKind::Enum => {
                if definition.members.is_empty() {
                    push_unique_message(
                        blocking_reasons,
                        &format!(
                            "Enum symbol definition `{}` has no members to lower into `.fsm`.",
                            definition.symbol_name
                        ),
                    );
                    required_canonical_enrichments.insert(
                        "keep canonical enum symbol definitions anchored to at least one member"
                            .to_string(),
                    );
                    continue;
                }
                for member in &definition.members {
                    validate_control_expression_renderability(
                        &member.value,
                        false,
                        signals_by_name,
                        size_entries,
                        blocking_reasons,
                        required_canonical_enrichments,
                    );
                }
            }
            _ => {
                let Some(value) = definition.value.as_ref() else {
                    push_unique_message(
                        blocking_reasons,
                        &format!(
                            "Symbol definition `{}` is missing its canonical value.",
                            definition.symbol_name
                        ),
                    );
                    required_canonical_enrichments.insert(
                        "preserve canonical scalar values for non-enum symbol definitions"
                            .to_string(),
                    );
                    continue;
                };
                validate_control_expression_renderability(
                    value,
                    false,
                    signals_by_name,
                    size_entries,
                    blocking_reasons,
                    required_canonical_enrichments,
                );
            }
        }
    }
}

fn collect_renderable_dt_blocks(
    control_blocks: &[ControlBlockRecord],
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    driven_outputs: &mut BTreeSet<String>,
    sequential_targets: &mut BTreeSet<String>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) -> Vec<ControlBlockRecord> {
    let mut seen_block_names = BTreeSet::new();
    let mut renderable_blocks = Vec::new();

    for block in control_blocks {
        match block.role {
            ControlBlockRole::StandaloneDecisionTree => {}
            ControlBlockRole::StateBody => {
                push_unique_message(
                    blocking_reasons,
                    &format!(
                        "Standalone `?dt:name` lowering cannot emit regular FSM state body `{}`.",
                        block.block_name
                    ),
                );
                required_canonical_enrichments.insert(
                    "keep `?dt:name` lowering scoped to standalone decision-tree blocks"
                        .to_string(),
                );
                continue;
            }
            ControlBlockRole::ResetSynchronous | ControlBlockRole::ResetAsynchronous => {
                push_unique_message(
                    blocking_reasons,
                    "Standalone `?dt:name` lowering does not yet emit dedicated reset-state blocks.",
                );
                required_canonical_enrichments.insert(
                    "keep reset-state control in `?fsm:name` roots until standalone-DT reset lowering is designed"
                        .to_string(),
                );
                continue;
            }
        }

        let rendered_name = render_control_block_name(block);
        if !seen_block_names.insert(rendered_name.clone()) {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical standalone control block `{}` is duplicated in the active `.fsm` DT slice.",
                    rendered_name
                ),
            );
            required_canonical_enrichments.insert(
                "deduplicate canonical standalone control-block names before lowering `.fsm` text"
                    .to_string(),
            );
        }

        validate_control_block_branches(
            block,
            false,
            None,
            signals_by_name,
            size_entries,
            driven_outputs,
            sequential_targets,
            blocking_reasons,
            required_canonical_enrichments,
        );
        renderable_blocks.push(block.clone());
    }

    renderable_blocks.sort_by_key(|block| block.declaration_order);
    renderable_blocks
}

fn collect_renderable_fsm_blocks(
    control_blocks: &[ControlBlockRecord],
    state_names: &BTreeSet<String>,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    driven_outputs: &mut BTreeSet<String>,
    sequential_targets: &mut BTreeSet<String>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) -> (
    BTreeMap<String, Vec<ControlBlockRecord>>,
    Vec<ControlBlockRecord>,
) {
    let mut state_blocks = BTreeMap::<String, Vec<ControlBlockRecord>>::new();
    let mut top_level_blocks = Vec::new();
    let mut seen_top_level_block_names = BTreeSet::new();

    for block in control_blocks {
        match block.role {
            ControlBlockRole::StateBody => {
                if !state_names.contains(&block.block_name) {
                    push_unique_message(
                        blocking_reasons,
                        &format!(
                            "Canonical state-body block `{}` does not resolve to an explicit regular state.",
                            block.block_name
                        ),
                    );
                    required_canonical_enrichments.insert(
                        "align every canonical state-body block with a declared regular state"
                            .to_string(),
                    );
                    continue;
                }
                validate_control_block_branches(
                    block,
                    true,
                    Some(state_names),
                    signals_by_name,
                    size_entries,
                    driven_outputs,
                    sequential_targets,
                    blocking_reasons,
                    required_canonical_enrichments,
                );
                state_blocks
                    .entry(block.block_name.clone())
                    .or_default()
                    .push(block.clone());
            }
            ControlBlockRole::StandaloneDecisionTree
            | ControlBlockRole::ResetSynchronous
            | ControlBlockRole::ResetAsynchronous => {
                let rendered_name = render_control_block_name(block);
                if !seen_top_level_block_names.insert(rendered_name.clone()) {
                    push_unique_message(
                        blocking_reasons,
                        &format!(
                            "Canonical top-level control block `{}` is duplicated in the active `.fsm` FSM slice.",
                            rendered_name
                        ),
                    );
                    required_canonical_enrichments.insert(
                        "deduplicate canonical top-level control-block names before lowering a true `?fsm:name` root".to_string(),
                    );
                }
                validate_control_block_branches(
                    block,
                    false,
                    Some(state_names),
                    signals_by_name,
                    size_entries,
                    driven_outputs,
                    sequential_targets,
                    blocking_reasons,
                    required_canonical_enrichments,
                );
                top_level_blocks.push(block.clone());
            }
        }
    }

    for blocks in state_blocks.values_mut() {
        blocks.sort_by_key(|block| block.declaration_order);
    }
    top_level_blocks.sort_by_key(|block| block.declaration_order);
    (state_blocks, top_level_blocks)
}

fn validate_control_block_branches(
    block: &ControlBlockRecord,
    allow_transition_actions: bool,
    state_names: Option<&BTreeSet<String>>,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    driven_outputs: &mut BTreeSet<String>,
    sequential_targets: &mut BTreeSet<String>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    if let Some(selector) = block.selector.as_ref() {
        validate_control_expression_renderability(
            selector,
            true,
            signals_by_name,
            size_entries,
            blocking_reasons,
            required_canonical_enrichments,
        );
        validate_test_selector_head_renderability(
            block,
            selector,
            blocking_reasons,
            required_canonical_enrichments,
        );
    }
    if block.branches.is_empty() {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Canonical control block `{}` has no branches to lower into `.fsm`.",
                block.block_name
            ),
        );
        required_canonical_enrichments.insert(
            "keep each canonical control block anchored to at least one branch".to_string(),
        );
    }

    for branch in &block.branches {
        if let Some(predicate) = branch.predicate.as_ref() {
            validate_control_expression_renderability(
                predicate,
                true,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }
        if let Some(selector) = block.selector.as_ref() {
            validate_test_selector_branch_renderability(
                block,
                selector,
                branch,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }
        if branch.actions.is_empty() {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical control branch `{}` has no actions to lower into `.fsm`.",
                    branch.branch_id
                ),
            );
            required_canonical_enrichments.insert(
                "keep each canonical control branch anchored to at least one action".to_string(),
            );
        }
        for action in &branch.actions {
            validate_control_action_renderability(
                action,
                allow_transition_actions,
                state_names,
                signals_by_name,
                size_entries,
                driven_outputs,
                sequential_targets,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }
    }
}

fn validate_control_expression_renderability(
    expression: &ControlExpressionRecord,
    allow_signal_references: bool,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    match expression {
        ControlExpressionRecord::Reference { reference } => {
            if reference
                .suffixes
                .iter()
                .any(|suffix| matches!(suffix, ControlReferenceSuffix::WidthCast { .. }))
            {
                push_unique_message(
                    blocking_reasons,
                    "The active `.fsm` adapter slice does not yet lower width-cast signal references.",
                );
                required_canonical_enrichments.insert(
                    "add width-cast reference lowering from canonical control expressions"
                        .to_string(),
                );
            }
            if matches!(reference.kind_hint, ControlReferenceKind::Symbol) {
                return;
            }
            if !allow_signal_references {
                push_unique_message(
                    blocking_reasons,
                    &format!(
                        "Symbol-definition expression `{}` currently resolves through signal `{}`, which is outside the active `.fsm` symbol slice.",
                        render_control_expression(expression),
                        reference.base_name
                    ),
                );
                required_canonical_enrichments.insert(
                    "keep emitted `.fsm` symbol-definition values signal-free in the active slice"
                        .to_string(),
                );
                return;
            }
            register_renderable_signal(
                &reference.base_name,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }
        ControlExpressionRecord::Literal { .. } => {}
        ControlExpressionRecord::Unary { operand, .. } => {
            validate_control_expression_renderability(
                operand,
                allow_signal_references,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }
        ControlExpressionRecord::Binary { left, right, .. } => {
            validate_control_expression_renderability(
                left,
                allow_signal_references,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
            validate_control_expression_renderability(
                right,
                allow_signal_references,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }
    }
}

fn validate_control_action_renderability(
    action: &ControlActionRecord,
    allow_transition_actions: bool,
    state_names: Option<&BTreeSet<String>>,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    driven_outputs: &mut BTreeSet<String>,
    sequential_targets: &mut BTreeSet<String>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    match action {
        ControlActionRecord::Assign {
            target,
            assignment_kind,
            dual_output,
            value,
        } => {
            let target_direction = register_renderable_signal(
                &target.signal_name,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
            if !matches!(target_direction, Some(InterfaceSignalDirection::Output)) {
                push_unique_message(
                    blocking_reasons,
                    &format!(
                        "Assignment target `{}` is not declared as a canonical output signal.",
                        target.signal_name
                    ),
                );
                required_canonical_enrichments.insert(
                    "keep first-slice assignment targets aligned with explicit output roles"
                        .to_string(),
                );
            } else {
                driven_outputs.insert(target.signal_name.clone());
            }
            if dual_output.is_some()
                && !matches!(assignment_kind, DecisionTreeAssignmentKind::Sequential)
            {
                push_unique_message(
                    blocking_reasons,
                    "Dual-output assignment lowering currently requires canonical sequential assignment intent.",
                );
                required_canonical_enrichments.insert(
                    "keep dual-output canonical assignments explicitly sequential in the active `.fsm` slice".to_string(),
                );
            }
            if matches!(assignment_kind, DecisionTreeAssignmentKind::Sequential) {
                sequential_targets.insert(target.signal_name.clone());
            }
            validate_control_expression_renderability(
                value,
                true,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }
        ControlActionRecord::Transition { target_state } => {
            if !allow_transition_actions {
                push_unique_message(
                    blocking_reasons,
                    &format!(
                        "Transition action `-> {target_state}` is only supported inside canonical FSM state-body blocks."
                    ),
                );
                required_canonical_enrichments.insert(
                    "keep transition actions scoped to canonical FSM state-body control blocks"
                        .to_string(),
                );
                return;
            }
            if let Some(state_names) = state_names {
                if !state_names.contains(target_state) {
                    push_unique_message(
                        blocking_reasons,
                        &format!(
                            "Transition action target `{}` is not declared as a canonical regular state.",
                            target_state
                        ),
                    );
                    required_canonical_enrichments.insert(
                        "declare every transition target as an explicit canonical regular state"
                            .to_string(),
                    );
                }
            }
        }
        ControlActionRecord::DelayedPulse {
            target,
            delay,
            value,
        } => {
            let target_direction = register_renderable_signal(
                &target.signal_name,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
            if !matches!(target_direction, Some(InterfaceSignalDirection::Output)) {
                push_unique_message(
                    blocking_reasons,
                    &format!(
                        "Delayed-pulse target `{}` is not declared as a canonical output signal.",
                        target.signal_name
                    ),
                );
            } else {
                driven_outputs.insert(target.signal_name.clone());
            }
            sequential_targets.insert(target.signal_name.clone());
            if *delay == 0 {
                push_unique_message(
                    blocking_reasons,
                    "The active `.fsm` delayed-pulse lowering requires a strictly positive delay.",
                );
                required_canonical_enrichments.insert(
                    "keep canonical delayed-pulse actions on positive cycle delays".to_string(),
                );
            }
            match value {
                ControlExpressionRecord::Literal { literal }
                    if literal == "0" || literal == "1" => {}
                _ => {
                    push_unique_message(
                        blocking_reasons,
                        "The active `.fsm` delayed-pulse lowering only supports literal `0` or `1` pulse levels.",
                    );
                    required_canonical_enrichments.insert(
                        "keep the first delayed-pulse adapter slice limited to literal 0/1 pulse levels".to_string(),
                    );
                }
            }
        }
        ControlActionRecord::CompoundUpdate { target, amount, .. } => {
            let target_direction = register_renderable_signal(
                &target.signal_name,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
            if !matches!(target_direction, Some(InterfaceSignalDirection::Output)) {
                push_unique_message(
                    blocking_reasons,
                    &format!(
                        "Compound-update target `{}` is not declared as a canonical output signal.",
                        target.signal_name
                    ),
                );
                required_canonical_enrichments.insert(
                    "keep canonical compound-update targets aligned with explicit output roles"
                        .to_string(),
                );
            } else {
                driven_outputs.insert(target.signal_name.clone());
            }
            if target.exposed_public_output {
                push_unique_message(
                    blocking_reasons,
                    &format!(
                        "Compound-update target `{}` cannot currently carry explicit public-output exposure in the active `.fsm` shorthand slice.",
                        target.signal_name
                    ),
                );
                required_canonical_enrichments.insert(
                    "keep canonical compound-update targets on scalar signal names without explicit public-output exposure".to_string(),
                );
            }
            sequential_targets.insert(target.signal_name.clone());
            if let Some(amount) = amount.as_ref() {
                validate_control_expression_renderability(
                    amount,
                    true,
                    signals_by_name,
                    size_entries,
                    blocking_reasons,
                    required_canonical_enrichments,
                );
            }
        }
    }
}

fn validate_block_set_renderability(
    blocks: &[DecisionTreeFragmentRecord],
    duplicate_names_allowed: &BTreeSet<String>,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    driven_outputs: &mut BTreeSet<String>,
    sequential_targets: &mut BTreeSet<String>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    let mut seen_block_names = BTreeSet::new();

    for block in blocks {
        if !duplicate_names_allowed.contains(&block.block_name)
            && !seen_block_names.insert(block.block_name.clone())
        {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical control block `{}` is duplicated; the current `.fsm` slice expects one unique standalone block per non-state name.",
                    block.block_name
                ),
            );
            required_canonical_enrichments.insert(
                "normalize canonical standalone block names before lowering `.fsm` text"
                    .to_string(),
            );
        }

        if block.actions.is_empty() {
            push_unique_message(
                blocking_reasons,
                &format!(
                    "Canonical control block `{}` has no typed actions to lower into `.fsm`.",
                    block.block_name
                ),
            );
            required_canonical_enrichments.insert(
                "keep each canonical control block anchored to at least one typed assignment action"
                    .to_string(),
            );
        }

        if let Some(guard) = block.guard.as_ref() {
            validate_guard_renderability(
                guard,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }

        for action in &block.actions {
            validate_action_renderability(
                action,
                signals_by_name,
                size_entries,
                driven_outputs,
                sequential_targets,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }
    }
}

fn validate_transition_renderability(
    transition: &FsmTransitionCandidate,
    state_names: &BTreeSet<String>,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    if !state_names.contains(&transition.source_state) {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Transition source `{}` is not declared as a canonical regular state.",
                transition.source_state
            ),
        );
        required_canonical_enrichments.insert(
            "declare every transition source as an explicit canonical regular state".to_string(),
        );
    }

    if !state_names.contains(&transition.target_state) {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Transition target `{}` is not declared as a canonical regular state.",
                transition.target_state
            ),
        );
        required_canonical_enrichments.insert(
            "declare every transition target as an explicit canonical regular state".to_string(),
        );
    }

    if let Some(guard) = transition.guard.as_ref() {
        validate_guard_renderability(
            guard,
            signals_by_name,
            size_entries,
            blocking_reasons,
            required_canonical_enrichments,
        );
    }
}

fn build_adapter_residual_decisions(
    intent_ir: &IntentIr,
    signal_inventory: &[FsmSignalCandidate],
    state_candidates: &[FsmStateCandidate],
    transition_candidates: &[FsmTransitionCandidate],
    decision_tree_candidates: &[FsmDecisionTreeCandidate],
    root_kind_decision: &FsmRootKindDecision,
    renderability: &FsmRenderability,
    module_candidates: &[FsmExplicitModuleCandidate],
    top_candidates: &[FsmTopCandidate],
) -> Vec<ResidualDecisionPacket> {
    let mut residual_decisions = intent_ir.residual_decisions.clone();
    let advisory_signal_inventory_only = decision_tree_candidates
        .iter()
        .all(|candidate| candidate.blocks.is_empty())
        && signal_inventory
            .iter()
            .any(|signal| signal.direction_hint.is_none() || signal.width_hint.is_none());
    let signal_inventory_blocked = advisory_signal_inventory_only
        || renderability.blocking_reasons.iter().any(|reason| {
            reason.contains("signal")
                || reason.contains("direction")
                || reason.contains("width")
                || reason.contains("output")
                || reason.contains("Declared")
        });
    if signal_inventory_blocked {
        residual_decisions.push(ResidualDecisionPacket {
            packet_id: "fsm_adapter_signal_inventory".to_string(),
            question: "Which canonical signal inventory should the `.fsm` adapter lower into ports, predicates, and assignments?".to_string(),
            why_unresolved: format!(
                "The current IntentIR artifact still leaves some render-critical signal roles unresolved across {} inventoried signal(s).",
                signal_inventory.len()
            ),
            automation_confidence: AutomationConfidence::Low,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "enrich_intent_ir_interface_inventory".to_string(),
                    description: "Add or complete backend-neutral interface and signal facts before widening `.fsm` emission.".to_string(),
                    downstream_impact: "The adapter can render real `.fsm` roots without inventing ports or widths.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "keep_signal_inventory_plan_only".to_string(),
                    description: "Treat the current signal inventory as advisory only and keep the adapter artifact blocked.".to_string(),
                    downstream_impact: "The adapter remains honest, but generated `.fsm` text stays unavailable until upstream canonical structure improves.".to_string(),
                },
            ],
        });
    }

    let state_graph_blocked = !renderability.is_renderable
        && (!state_candidates.is_empty()
            || !transition_candidates.is_empty()
            || renderability.blocking_reasons.iter().any(|reason| {
                reason.contains("regular state")
                    || reason.contains("Transition")
                    || reason.contains("initial regular state")
                    || reason.contains("FSM-root")
            }));
    if state_graph_blocked {
        residual_decisions.push(ResidualDecisionPacket {
            packet_id: "fsm_adapter_state_graph".to_string(),
            question: "Which canonical regular states, initial-state designation, and transition targets are complete enough to lower a true `?fsm:name` root honestly?".to_string(),
            why_unresolved: format!(
                "The current adapter sees {} regular-state candidate(s) and {} transition candidate(s), but the canonical state graph is not yet complete enough to guarantee safe FSM-root lowering in every case.",
                state_candidates.len(),
                transition_candidates.len()
            ),
            automation_confidence: AutomationConfidence::Low,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "enrich_intent_ir_state_graph".to_string(),
                    description: "Carry explicit regular states, exactly one initial-state designation, and fully declared transition targets forward in canonical form.".to_string(),
                    downstream_impact: "The adapter can emit a true `?fsm:name` root without inventing state identity or target membership.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "keep_fsm_root_blocked".to_string(),
                    description: "Continue blocking true FSM-root emission until the canonical state graph is explicit and internally consistent.".to_string(),
                    downstream_impact: "The adapter stays honest, but stateful `.fsm` text remains unavailable for under-specified inputs.".to_string(),
                },
            ],
        });
    }

    let system_contract_blocked = renderability.blocking_reasons.iter().any(|reason| {
        reason.contains("system contract")
            || reason.contains("clock")
            || reason.contains("reset")
            || reason.contains("init assignment")
            || reason.contains("Sequential target")
            || reason.contains("Init assignment target")
    });
    if system_contract_blocked {
        residual_decisions.push(ResidualDecisionPacket {
            packet_id: "fsm_adapter_system_contract".to_string(),
            question: "Which backend-neutral system-contract and init facts are complete enough to lower sequential `.fsm` control honestly?".to_string(),
            why_unresolved: "The current `.fsm` slice still lacks some combination of explicit clock/reset facts, supported reset kind, or reset/init assignments needed for honest sequential lowering.".to_string(),
            automation_confidence: AutomationConfidence::Low,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "enrich_intent_ir_system_surface".to_string(),
                    description: "Carry explicit clock/reset/init facts forward in canonical form so the adapter can lower `(+system ...)` and `(:= ...)` directly.".to_string(),
                    downstream_impact: "Sequential DT and FSM cases become renderable without inventing implicit reset semantics inside the adapter.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "keep_sequential_lowering_blocked".to_string(),
                    description: "Continue treating sequential `.fsm` lowering as blocked until the canonical system/init surface is explicit.".to_string(),
                    downstream_impact: "The adapter stays honest, but sequential `.fsm` text remains unavailable for under-specified inputs.".to_string(),
                },
            ],
        });
    }

    let dt_surface_blocked = !renderability.is_renderable
        && matches!(
            root_kind_decision.selected_root_kind,
            FsmRootKind::Dt | FsmRootKind::Fsm
        )
        && (decision_tree_candidates
            .iter()
            .all(|candidate| candidate.blocks.is_empty())
            || renderability.blocking_reasons.iter().any(|reason| {
                reason.contains("control block")
                    || reason.contains("guard")
                    || reason.contains("assignment")
                    || reason.contains("selector/test")
                    || reason.contains("compound-update")
            }));
    if dt_surface_blocked {
        residual_decisions.push(ResidualDecisionPacket {
            packet_id: "fsm_adapter_dt_action_graph".to_string(),
            question: "What backend-neutral control fragments should be lowered into `.fsm` guards and assignments?".to_string(),
            why_unresolved: format!(
                "The current `.fsm` adapter slice can identify {} DT candidate(s), but their executable guarded/action surface is still incomplete for honest lowering.",
                decision_tree_candidates.len()
            ),
            automation_confidence: AutomationConfidence::Low,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "enrich_intent_ir_with_dt_fragments".to_string(),
                    description: "Introduce or complete canonical guarded/action fragments so adapters lower typed predicates and assignments instead of prose summaries.".to_string(),
                    downstream_impact: "DT-centric targets like `.fsm` can become directly renderable and later HDL adapters can share the same canonical control structure.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "defer_target_text_emission".to_string(),
                    description: "Preserve the current typed lowering plan without emitting `.fsm` text until canonical control structure exists.".to_string(),
                    downstream_impact: "The adapter remains useful for roadmap validation and gap analysis, but target text stays blocked.".to_string(),
                },
            ],
        });
    }

    let composition_blocked = !top_candidates.is_empty() && !renderability.is_renderable;
    if composition_blocked {
        residual_decisions.push(ResidualDecisionPacket {
            packet_id: "fsm_adapter_composition_topology".to_string(),
            question: "Which explicit module and top facts are complete enough to lower a true `?top:name` composition root honestly?".to_string(),
            why_unresolved: format!(
                "The current adapter sees {} explicit module candidate(s) and {} explicit top candidate(s), but at least one child source, top port, or top link detail is still incomplete for the first composition slice.",
                module_candidates.len(),
                top_candidates.len()
            ),
            automation_confidence: AutomationConfidence::Low,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "enrich_intent_ir_composition_topology".to_string(),
                    description: "Carry explicit top ports, child-module references, renderable child modules, and width-compatible top links forward in canonical form.".to_string(),
                    downstream_impact: "The adapter can emit a true `?top:name` root with embedded DT/FSM children instead of deferring composition semantics.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "keep_top_root_blocked".to_string(),
                    description: "Continue blocking broader composition lowering until module/top topology is fully explicit and internally consistent.".to_string(),
                    downstream_impact: "The adapter stays honest, but broader `.fsm` composition text remains unavailable for under-specified inputs.".to_string(),
                },
            ],
        });
    }

    residual_decisions.push(ResidualDecisionPacket {
        packet_id: "fsm_adapter_root_kind_expansion".to_string(),
        question: "When should the `.fsm` adapter escalate from a `?dt:name` root to `?fsm:name` or `?top:name`?".to_string(),
        why_unresolved: format!(
            "The adapter currently selects `?{}:name`; explicit top composition can now participate when module/top facts are present, while compatibility-level `?mod:name` / `?module:name` roots remain outside the current canonical root-kind model until a real backend-neutral direct-module distinction exists.",
            root_kind_decision.selected_root_kind.as_str()
        ),
        automation_confidence: AutomationConfidence::Medium,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "promote_explicit_state_model".to_string(),
                description: "Add explicit state and transition records only for genuinely sequential control so the adapter can choose `?fsm:name` honestly.".to_string(),
                downstream_impact: "Sequential intent can lower into true FSM roots without collapsing broader composition semantics into DTs.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "promote_explicit_module_topology".to_string(),
                description: "Carry explicit modules and one explicit top composition forward so the adapter can choose `?top:name` without inventing child roots or wiring.".to_string(),
                downstream_impact: "Broader composition intent can lower honestly while direct-module alias roots stay deferred until they have a backend-neutral semantic basis.".to_string(),
            },
        ],
    });

    residual_decisions
}

fn push_unique_message(messages: &mut Vec<String>, message: &str) {
    if !messages.iter().any(|existing| existing == message) {
        messages.push(message.to_string());
    }
}

fn register_canonical_signal(
    inventory: &mut BTreeMap<String, SignalInventoryEvidence>,
    signal_name: &str,
    direction_hint: Option<InterfaceSignalDirection>,
    width_hint: Option<u32>,
    supporting_canonical_id: &str,
    mention_category: &str,
    automation_confidence: AutomationConfidence,
) {
    let entry = inventory.entry(signal_name.to_string()).or_default();
    merge_signal_hint(&mut entry.direction_hint, direction_hint);
    merge_signal_hint(&mut entry.width_hint, width_hint);
    entry
        .supporting_canonical_ids
        .insert(supporting_canonical_id.to_string());
    entry
        .mention_categories
        .insert(mention_category.to_string());
    entry.automation_confidence =
        max_automation_confidence(entry.automation_confidence, automation_confidence);
}

fn register_signal_mentions(
    inventory: &mut BTreeMap<String, SignalInventoryEvidence>,
    statement: &str,
    supporting_canonical_id: &str,
    mention_category: &str,
) {
    for token in extract_signal_tokens(statement) {
        register_canonical_signal(
            inventory,
            &token,
            None,
            None,
            supporting_canonical_id,
            mention_category,
            AutomationConfidence::Low,
        );
    }
}

fn merge_signal_hint<T: Copy + Eq>(target: &mut Option<T>, incoming: Option<T>) {
    match (*target, incoming) {
        (None, Some(value)) => *target = Some(value),
        (Some(existing), Some(value)) if existing != value => *target = None,
        _ => {}
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

fn fold_automation_confidence<I>(confidences: I) -> AutomationConfidence
where
    I: IntoIterator<Item = AutomationConfidence>,
{
    confidences
        .into_iter()
        .fold(AutomationConfidence::Low, max_automation_confidence)
}

fn validate_guard_renderability(
    guard: &DecisionTreeGuardRecord,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    match guard {
        DecisionTreeGuardRecord::SignalIsHigh { signal_name } => {
            register_renderable_signal(
                signal_name,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
        }
        DecisionTreeGuardRecord::Comparison {
            left_signal,
            operator,
            right,
        } => {
            register_renderable_signal(
                left_signal,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
            if matches!(operator, DecisionTreeComparisonOperator::NotEq) {
                push_unique_message(
                    blocking_reasons,
                    "The first renderable `.fsm` slice only lowers equality or truthy guards; `!=` guards remain deferred.",
                );
                required_canonical_enrichments.insert(
                    "keep first-slice canonical guards to truthy checks or equality comparisons"
                        .to_string(),
                );
            }
            if let DecisionTreeValueRecord::SignalRef { signal_name } = right {
                register_renderable_signal(
                    signal_name,
                    signals_by_name,
                    size_entries,
                    blocking_reasons,
                    required_canonical_enrichments,
                );
            }
        }
    }
}

fn validate_action_renderability(
    action: &DecisionTreeActionRecord,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    driven_outputs: &mut BTreeSet<String>,
    sequential_targets: &mut BTreeSet<String>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    match action {
        DecisionTreeActionRecord::Assign {
            target_signal,
            assignment_kind,
            value,
        } => {
            let target_direction = register_renderable_signal(
                target_signal,
                signals_by_name,
                size_entries,
                blocking_reasons,
                required_canonical_enrichments,
            );
            if !matches!(target_direction, Some(InterfaceSignalDirection::Output)) {
                push_unique_message(
                    blocking_reasons,
                    &format!(
                        "Assignment target `{}` is not declared as a canonical output signal.",
                        target_signal
                    ),
                );
                required_canonical_enrichments.insert(
                    "keep first-slice assignment targets aligned with explicit output roles"
                        .to_string(),
                );
            } else {
                driven_outputs.insert(target_signal.clone());
            }

            if matches!(assignment_kind, DecisionTreeAssignmentKind::Sequential) {
                sequential_targets.insert(target_signal.clone());
            }

            if let DecisionTreeValueRecord::SignalRef { signal_name } = value {
                register_renderable_signal(
                    signal_name,
                    signals_by_name,
                    size_entries,
                    blocking_reasons,
                    required_canonical_enrichments,
                );
            }
        }
    }
}

fn register_renderable_signal(
    signal_name: &str,
    signals_by_name: &BTreeMap<String, &FsmSignalCandidate>,
    size_entries: &mut BTreeMap<String, FsmRenderableSizeEntry>,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) -> Option<InterfaceSignalDirection> {
    let Some(signal) = signals_by_name.get(signal_name).copied() else {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Canonical control references undeclared signal `{}`.",
                signal_name
            ),
        );
        required_canonical_enrichments.insert(
            "promote a canonical interface inventory with stable signal names, directions, and widths"
                .to_string(),
        );
        return None;
    };

    let Some(direction_hint) = signal.direction_hint else {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Signal `{}` is missing a canonical direction hint required for `.fsm` emission.",
                signal_name
            ),
        );
        required_canonical_enrichments.insert(
            "promote a canonical interface inventory with stable signal names, directions, and widths"
                .to_string(),
        );
        return None;
    };

    if matches!(direction_hint, InterfaceSignalDirection::Internal) {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Signal `{}` is marked internal; the first `.fsm` slice does not yet lower canonical local temporaries.",
                signal_name
            ),
        );
        required_canonical_enrichments.insert(
            "add a canonical local-signal surface before lowering internal temporaries into `.fsm`"
                .to_string(),
        );
        return None;
    }

    let Some(width_hint) = signal.width_hint else {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Signal `{}` is missing a canonical width hint required for `.fsm` emission.",
                signal_name
            ),
        );
        required_canonical_enrichments.insert(
            "promote a canonical interface inventory with stable signal names, directions, and widths"
                .to_string(),
        );
        return None;
    };

    size_entries
        .entry(signal_name.to_string())
        .or_insert_with(|| FsmRenderableSizeEntry {
            signal_name: signal_name.to_string(),
            direction_hint,
            width: width_hint,
        });

    Some(direction_hint)
}

fn render_fsm_module(
    root_name: &str,
    root_kind: FsmRootKind,
    module: &FsmRenderableModule,
) -> String {
    let mut lines = vec![format!("(?{}:{root_name}", root_kind.as_str())];
    if let Some(system_contract) = module.system_contract.as_ref() {
        lines.push("  (+system".to_string());
        lines.push(format!("    (clock {})", system_contract.clock_signal));
        lines.push(format!(
            "    ({} {})",
            render_system_reset_kind(system_contract.reset_kind),
            system_contract.reset_signal
        ));
        lines.push("  )".to_string());
    }

    if !module.size_entries.is_empty() {
        lines.push("  (+size".to_string());
        for entry in &module.size_entries {
            lines.push(format!("    ({} {})", entry.signal_name, entry.width));
        }
        lines.push("  )".to_string());
    }

    lines.extend(render_symbol_definition_sections(
        &module.symbol_definitions,
    ));

    for init_assignment in &module.init_assignments {
        lines.push(format!("  ({})", render_init_assignment(init_assignment)));
    }

    if matches!(root_kind, FsmRootKind::Fsm) {
        for state in &module.states {
            lines.extend(render_regular_state(state, root_kind));
        }
    }

    for block in &module.blocks {
        lines.extend(render_standalone_block(block, root_kind));
    }

    lines.push(")".to_string());
    format!("{}\n", lines.join("\n"))
}

fn render_symbol_definition_sections(symbol_definitions: &[SymbolDefinitionRecord]) -> Vec<String> {
    let mut constants = symbol_definitions
        .iter()
        .filter(|definition| matches!(definition.kind, SymbolDefinitionKind::Constant))
        .collect::<Vec<_>>();
    let mut defines = symbol_definitions
        .iter()
        .filter(|definition| matches!(definition.kind, SymbolDefinitionKind::Define))
        .collect::<Vec<_>>();
    let mut params = symbol_definitions
        .iter()
        .filter(|definition| matches!(definition.kind, SymbolDefinitionKind::Param))
        .collect::<Vec<_>>();
    let mut enums = symbol_definitions
        .iter()
        .filter(|definition| matches!(definition.kind, SymbolDefinitionKind::Enum))
        .collect::<Vec<_>>();

    constants.sort_by_key(|definition| definition.declaration_order);
    defines.sort_by_key(|definition| definition.declaration_order);
    params.sort_by_key(|definition| definition.declaration_order);
    enums.sort_by_key(|definition| definition.declaration_order);

    let mut lines = Vec::new();

    if !constants.is_empty() {
        lines.extend(render_named_symbol_group("+constants", &constants));
    }

    for definition in defines {
        let value = definition
            .value
            .as_ref()
            .expect("renderable +define entry should include a value");
        lines.push(format!(
            "  (+define ({} {}))",
            definition.symbol_name,
            render_control_expression(value)
        ));
    }

    if !params.is_empty() {
        lines.extend(render_named_symbol_group("+params", &params));
    }

    if !enums.is_empty() {
        lines.push("  (+enums".to_string());
        for definition in enums {
            lines.push(format!("    ({}", definition.symbol_name));
            let mut members = definition.members.iter().collect::<Vec<_>>();
            members.sort_by_key(|member| member.declaration_order);
            for member in members {
                lines.push(format!(
                    "      ({} {})",
                    member.member_name,
                    render_control_expression(&member.value)
                ));
            }
            lines.push("    )".to_string());
        }
        lines.push("  )".to_string());
    }

    lines
}

fn render_named_symbol_group(
    section_name: &str,
    definitions: &[&SymbolDefinitionRecord],
) -> Vec<String> {
    let mut lines = vec![format!("  ({section_name}")];
    for definition in definitions {
        let value = definition
            .value
            .as_ref()
            .expect("renderable symbol definition should include a value");
        lines.push(format!(
            "    ({} {})",
            definition.symbol_name,
            render_control_expression(value)
        ));
    }
    lines.push("  )".to_string());
    lines
}

fn render_fsm_source_document(document: &FsmRenderableSourceDocument) -> String {
    let mut roots = Vec::new();

    if let Some(top_root) = document.top_root.as_ref() {
        roots.push(render_top_root(top_root));
    }

    for direct_root in &document.direct_roots {
        roots.push(render_fsm_module(
            &direct_root.module_name,
            direct_root.root_kind,
            &direct_root.module,
        ));
    }

    roots.join("\n")
}

fn render_top_root(top_root: &FsmRenderableTopRoot) -> String {
    let mut lines = vec![format!("(?top:{}", top_root.top_name)];

    if !top_root.ports.is_empty() {
        lines.push("  (?ports:public_io".to_string());
        for port in &top_root.ports {
            lines.push(format!("    {}", render_top_port_token(port)));
        }
        lines.push("  )".to_string());
    }

    for child in &top_root.children {
        lines.push(format!(
            "  (?{}:{} {})",
            render_top_child_kind(child.child_root_kind),
            child.instance_name,
            child.source_module_name
        ));
    }

    if !top_root.links.is_empty() {
        lines.push("  (?toplink:wiring".to_string());
        for link in &top_root.links {
            lines.push(format!(
                "    /{}/{}/",
                render_top_link_endpoint(&link.source),
                render_top_link_endpoint(&link.target)
            ));
        }
        lines.push("  )".to_string());
    }

    lines.push(")".to_string());
    format!("{}\n", lines.join("\n"))
}

fn render_top_port_token(port: &ExplicitTopPortRecord) -> String {
    match (port.direction_hint, port.width_hint) {
        (InterfaceSignalDirection::Input, None | Some(1)) => port.port_name.clone(),
        (InterfaceSignalDirection::Input, Some(width)) => format!("{}<{width}", port.port_name),
        (InterfaceSignalDirection::Output, None | Some(1)) => format!("{}>", port.port_name),
        (InterfaceSignalDirection::Output, Some(width)) => format!("{}>{width}", port.port_name),
        (InterfaceSignalDirection::Internal, None) => port.port_name.clone(),
        (InterfaceSignalDirection::Internal, Some(width)) => format!("{}<{width}", port.port_name),
    }
}

fn render_top_child_kind(root_kind: FsmRootKind) -> &'static str {
    match root_kind {
        FsmRootKind::Dt => "dtc",
        FsmRootKind::Fsm => "fsmc",
        FsmRootKind::Top => "topc",
    }
}

fn render_regular_state(state: &FsmRenderableState, root_kind: FsmRootKind) -> Vec<String> {
    let mut lines = vec![format!("  ({}", state.state_name)];

    for block in &state.blocks {
        lines.extend(render_state_body_fragment(block, root_kind));
    }
    for transition in &state.transitions {
        lines.extend(render_state_transition(transition));
    }

    lines.push("  )".to_string());
    lines
}

fn render_state_body_fragment(block: &ControlBlockRecord, root_kind: FsmRootKind) -> Vec<String> {
    render_control_block_body_lines(block, root_kind, "    ")
}

fn render_state_transition(transition: &StateTransitionRecord) -> Vec<String> {
    if let Some(guard) = transition.guard.as_ref() {
        return vec![
            format!("    ({}", render_guard(guard)),
            format!("      (-> {})", transition.target_state),
            "    )".to_string(),
        ];
    }

    vec![format!("    (-> {})", transition.target_state)]
}

fn render_standalone_block(block: &ControlBlockRecord, root_kind: FsmRootKind) -> Vec<String> {
    let mut lines = vec![format!("  (-{}", render_control_block_name(block))];
    lines.extend(render_control_block_body_lines(block, root_kind, "    "));
    lines.push("  )".to_string());
    lines
}

fn render_control_block_body_lines(
    block: &ControlBlockRecord,
    root_kind: FsmRootKind,
    indent: &str,
) -> Vec<String> {
    let mut branches = block.branches.iter().collect::<Vec<_>>();
    branches.sort_by_key(|branch| branch.declaration_order);
    if let Some(selector) = block.selector.as_ref() {
        return render_test_selector_block_lines(selector, &branches, root_kind, indent);
    }

    let mut lines = Vec::new();
    for branch in branches {
        lines.extend(render_control_branch_lines(branch, root_kind, indent));
    }
    lines
}

fn render_test_selector_block_lines(
    selector: &ControlExpressionRecord,
    branches: &[&ControlBranchRecord],
    root_kind: FsmRootKind,
    indent: &str,
) -> Vec<String> {
    let selector_head = render_test_selector_head(selector).expect(
        "renderable selector-bearing control blocks should carry a supported test-selector head",
    );
    let branch_indent = format!("{indent}  ");
    let action_indent = format!("{indent}    ");
    let mut lines = vec![format!("{indent}(?{selector_head}")];

    for branch in branches {
        let selector_token = render_test_selector_branch_token(
            selector,
            branch
                .predicate
                .as_ref()
                .expect("renderable selector branches should carry explicit predicates"),
        )
        .expect("renderable selector branches should lower into explicit selector tokens");
        lines.push(format!("{branch_indent}({selector_token}"));
        for action in &branch.actions {
            lines.push(format!(
                "{action_indent}({})",
                render_control_action(action, root_kind)
            ));
        }
        lines.push(format!("{branch_indent})"));
    }

    lines.push(format!("{indent})"));
    lines
}

fn render_control_branch_lines(
    branch: &ControlBranchRecord,
    root_kind: FsmRootKind,
    indent: &str,
) -> Vec<String> {
    if let Some(predicate) = branch.predicate.as_ref() {
        let action_indent = format!("{indent}  ");
        let mut lines = vec![format!(
            "{indent}(<{}",
            render_control_guard_expression(predicate)
        )];
        for action in &branch.actions {
            lines.push(format!(
                "{action_indent}({})",
                render_control_action(action, root_kind)
            ));
        }
        lines.push(format!("{indent})"));
        return lines;
    }

    branch
        .actions
        .iter()
        .map(|action| format!("{indent}({})", render_control_action(action, root_kind)))
        .collect()
}

fn render_control_action(action: &ControlActionRecord, root_kind: FsmRootKind) -> String {
    match action {
        ControlActionRecord::Assign {
            target,
            assignment_kind,
            dual_output,
            value,
        } => format!(
            "{} {} {}",
            render_control_assignment_target(target),
            render_control_assignment_operator(*assignment_kind, *dual_output, root_kind),
            render_control_expression(value)
        ),
        ControlActionRecord::Transition { target_state } => format!("-> {target_state}"),
        ControlActionRecord::DelayedPulse {
            target,
            delay,
            value,
        } => format!(
            "{} <{} {}",
            render_control_assignment_target(target),
            delay,
            render_control_expression(value)
        ),
        ControlActionRecord::CompoundUpdate {
            target,
            operation,
            amount,
        } => {
            let operator = match operation {
                ControlCompoundUpdateOperation::Increment => "+=",
                ControlCompoundUpdateOperation::Decrement => "-=",
            };
            match amount.as_ref() {
                Some(amount) => format!(
                    "{operator} {} {}",
                    render_control_assignment_target(target),
                    render_control_expression(amount)
                ),
                None => format!("{operator} {}", render_control_assignment_target(target)),
            }
        }
    }
}

fn render_control_assignment_operator(
    assignment_kind: DecisionTreeAssignmentKind,
    dual_output: Option<ControlDualOutputKind>,
    root_kind: FsmRootKind,
) -> &'static str {
    match dual_output {
        Some(ControlDualOutputKind::NextSignal) => "<-=",
        Some(ControlDualOutputKind::RegisteredSignal) => "<=+",
        None => render_assignment_operator(assignment_kind, root_kind),
    }
}

fn render_control_assignment_target(target: &ControlAssignmentTargetRecord) -> String {
    if target.exposed_public_output {
        return format!("{}>", target.signal_name);
    }

    target.signal_name.clone()
}

fn render_test_selector_head(selector: &ControlExpressionRecord) -> Option<String> {
    if let Some(signal_name) = plain_test_selector_signal_name(selector) {
        return Some(signal_name.to_string());
    }
    if supports_computed_test_selector_head(selector) {
        return Some(render_control_expression_prefix(selector));
    }

    None
}

fn render_test_selector_branch_token(
    selector: &ControlExpressionRecord,
    predicate: &ControlExpressionRecord,
) -> Option<String> {
    let ControlExpressionRecord::Binary {
        operator,
        left,
        right,
    } = predicate
    else {
        return None;
    };

    if let Some(value) = selector_value_render(left, selector, right, *operator) {
        return Some(format!(
            "{}{}",
            render_test_selector_operator(*operator)?,
            value
        ));
    }

    let inverted_operator = invert_test_selector_operator(*operator)?;
    let value = selector_value_render(right, selector, left, inverted_operator)?;
    Some(format!(
        "{}{}",
        render_test_selector_operator(inverted_operator)?,
        value
    ))
}

fn selector_value_render(
    selector_side: &ControlExpressionRecord,
    selector: &ControlExpressionRecord,
    value_side: &ControlExpressionRecord,
    _operator: ControlBinaryOperator,
) -> Option<String> {
    if selector_side != selector || !is_test_selector_scalar_value(value_side) {
        return None;
    }

    Some(render_test_selector_scalar_value(value_side))
}

fn plain_test_selector_signal_name(selector: &ControlExpressionRecord) -> Option<&str> {
    let ControlExpressionRecord::Reference { reference } = selector else {
        return None;
    };
    if reference.kind_hint == ControlReferenceKind::Symbol
        || !reference.suffixes.is_empty()
        || reference.exposed_public_output
        || !is_hdl_identifier(&reference.base_name)
    {
        return None;
    }

    Some(reference.base_name.as_str())
}

fn supports_computed_test_selector_head(selector: &ControlExpressionRecord) -> bool {
    matches!(
        selector,
        ControlExpressionRecord::Unary { .. } | ControlExpressionRecord::Binary { .. }
    )
}

fn render_control_expression_prefix(expression: &ControlExpressionRecord) -> String {
    match expression {
        ControlExpressionRecord::Reference { reference } => render_control_reference(reference),
        ControlExpressionRecord::Literal { literal } => literal.clone(),
        ControlExpressionRecord::Unary {
            operator: ControlUnaryOperator::Not,
            operand,
        } => format!("(! {})", render_control_expression_prefix(operand)),
        ControlExpressionRecord::Binary {
            operator,
            left,
            right,
        } => format!(
            "({} {} {})",
            render_control_binary_operator(*operator),
            render_control_expression_prefix(left),
            render_control_expression_prefix(right)
        ),
    }
}

fn is_test_selector_scalar_value(expression: &ControlExpressionRecord) -> bool {
    matches!(
        expression,
        ControlExpressionRecord::Reference { .. } | ControlExpressionRecord::Literal { .. }
    )
}

fn render_test_selector_scalar_value(expression: &ControlExpressionRecord) -> String {
    match expression {
        ControlExpressionRecord::Reference { reference } => render_control_reference(reference),
        ControlExpressionRecord::Literal { literal } => literal.clone(),
        _ => unreachable!("selector scalar values are validated before rendering"),
    }
}

fn render_test_selector_operator(operator: ControlBinaryOperator) -> Option<&'static str> {
    match operator {
        ControlBinaryOperator::Eq => Some("="),
        ControlBinaryOperator::NotEq => Some("!="),
        ControlBinaryOperator::Lt => Some("<"),
        ControlBinaryOperator::Le => Some("<="),
        ControlBinaryOperator::Gt => Some(">"),
        ControlBinaryOperator::Ge => Some(">="),
        _ => None,
    }
}

fn invert_test_selector_operator(operator: ControlBinaryOperator) -> Option<ControlBinaryOperator> {
    match operator {
        ControlBinaryOperator::Eq => Some(ControlBinaryOperator::Eq),
        ControlBinaryOperator::NotEq => Some(ControlBinaryOperator::NotEq),
        ControlBinaryOperator::Lt => Some(ControlBinaryOperator::Gt),
        ControlBinaryOperator::Le => Some(ControlBinaryOperator::Ge),
        ControlBinaryOperator::Gt => Some(ControlBinaryOperator::Lt),
        ControlBinaryOperator::Ge => Some(ControlBinaryOperator::Le),
        _ => None,
    }
}

fn validate_test_selector_head_renderability(
    block: &ControlBlockRecord,
    selector: &ControlExpressionRecord,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    if render_test_selector_head(selector).is_some() {
        return;
    }

    push_unique_message(
        blocking_reasons,
        &format!(
            "Canonical control block `{}` carries selector `{}` that the active `.fsm` test-node slice cannot encode yet as either `?signal` or `?(expr)`.",
            block.block_name,
            render_control_expression(selector)
        ),
    );
    required_canonical_enrichments.insert(
        "keep selector-bearing canonical control blocks on plain signal selectors or explicit computed unary/binary selector expressions".to_string(),
    );
}

fn validate_test_selector_branch_renderability(
    block: &ControlBlockRecord,
    selector: &ControlExpressionRecord,
    branch: &ControlBranchRecord,
    blocking_reasons: &mut Vec<String>,
    required_canonical_enrichments: &mut BTreeSet<String>,
) {
    let Some(predicate) = branch.predicate.as_ref() else {
        push_unique_message(
            blocking_reasons,
            &format!(
                "Selector-bearing canonical control block `{}` requires every branch to carry an explicit selector predicate.",
                block.block_name
            ),
        );
        required_canonical_enrichments.insert(
            "keep selector-bearing canonical branches aligned with explicit operator-prefixed selector predicates".to_string(),
        );
        return;
    };

    if render_test_selector_branch_token(selector, predicate).is_some() {
        return;
    }

    push_unique_message(
        blocking_reasons,
        &format!(
            "Selector-bearing canonical control block `{}` has branch predicate `{}` that does not map to an explicit `.fsm` test selector token against selector `{}`.",
            block.block_name,
            render_control_expression(predicate),
            render_control_expression(selector)
        ),
    );
    required_canonical_enrichments.insert(
        "keep selector-bearing canonical branches as comparisons between the selected test expression and a scalar selector token".to_string(),
    );
}

fn is_hdl_identifier(token: &str) -> bool {
    let mut chars = token.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
        return false;
    }

    chars.all(|character| character.is_ascii_alphanumeric() || character == '_')
}

fn render_control_guard_expression(expression: &ControlExpressionRecord) -> String {
    match expression {
        ControlExpressionRecord::Unary {
            operator: ControlUnaryOperator::Not,
            operand,
        } if control_expression_is_inline_guard_operand(operand) => {
            format!("!{}", render_control_expression(operand))
        }
        ControlExpressionRecord::Binary {
            operator,
            left,
            right,
        } if matches!(
            operator,
            ControlBinaryOperator::Eq
                | ControlBinaryOperator::NotEq
                | ControlBinaryOperator::Lt
                | ControlBinaryOperator::Le
                | ControlBinaryOperator::Gt
                | ControlBinaryOperator::Ge
        ) && control_expression_is_inline_guard_operand(left)
            && control_expression_is_inline_guard_operand(right) =>
        {
            format!(
                "{}{}{}",
                render_control_expression(left),
                render_control_binary_operator(*operator),
                render_control_expression(right)
            )
        }
        _ => render_control_expression(expression),
    }
}

fn control_expression_is_inline_guard_operand(expression: &ControlExpressionRecord) -> bool {
    match expression {
        ControlExpressionRecord::Reference { .. } | ControlExpressionRecord::Literal { .. } => true,
        ControlExpressionRecord::Unary {
            operator: ControlUnaryOperator::Not,
            operand,
        } => matches!(
            operand.as_ref(),
            ControlExpressionRecord::Reference { .. } | ControlExpressionRecord::Literal { .. }
        ),
        ControlExpressionRecord::Binary { .. } => false,
    }
}

fn render_control_expression(expression: &ControlExpressionRecord) -> String {
    match expression {
        ControlExpressionRecord::Reference { reference } => render_control_reference(reference),
        ControlExpressionRecord::Literal { literal } => literal.clone(),
        ControlExpressionRecord::Unary {
            operator: ControlUnaryOperator::Not,
            operand,
        } => {
            if matches!(
                operand.as_ref(),
                ControlExpressionRecord::Reference { .. } | ControlExpressionRecord::Literal { .. }
            ) {
                return format!("!{}", render_control_expression(operand));
            }

            format!("(! {})", render_control_expression(operand))
        }
        ControlExpressionRecord::Binary {
            operator,
            left,
            right,
        } => format!(
            "({} {} {})",
            render_control_binary_operator(*operator),
            render_control_expression(left),
            render_control_expression(right)
        ),
    }
}

fn render_control_reference(reference: &ControlReferenceRecord) -> String {
    let mut rendered = reference.base_name.clone();

    for suffix in &reference.suffixes {
        match suffix {
            ControlReferenceSuffix::Member { member_name } => {
                rendered.push('.');
                rendered.push_str(member_name);
            }
            ControlReferenceSuffix::BitIndex { index } => {
                rendered.push('[');
                rendered.push_str(&index.to_string());
                rendered.push(']');
            }
            ControlReferenceSuffix::Slice { msb, lsb } => {
                rendered.push('[');
                rendered.push_str(&msb.to_string());
                rendered.push(':');
                rendered.push_str(&lsb.to_string());
                rendered.push(']');
            }
            ControlReferenceSuffix::WidthCast { width } => {
                rendered.push('\'');
                rendered.push_str(&width.to_string());
            }
        }
    }

    if reference.exposed_public_output {
        rendered.push('>');
    }

    rendered
}

fn render_control_binary_operator(operator: ControlBinaryOperator) -> &'static str {
    match operator {
        ControlBinaryOperator::Add => "+",
        ControlBinaryOperator::Sub => "-",
        ControlBinaryOperator::Mul => "*",
        ControlBinaryOperator::Div => "/",
        ControlBinaryOperator::Mod => "%",
        ControlBinaryOperator::BitAnd => "&",
        ControlBinaryOperator::BitOr => "|",
        ControlBinaryOperator::BitXor => "^",
        ControlBinaryOperator::Eq => "==",
        ControlBinaryOperator::NotEq => "!=",
        ControlBinaryOperator::Lt => "<",
        ControlBinaryOperator::Le => "<=",
        ControlBinaryOperator::Gt => ">",
        ControlBinaryOperator::Ge => ">=",
    }
}

fn render_control_block_name(block: &ControlBlockRecord) -> String {
    match block.role {
        ControlBlockRole::StandaloneDecisionTree | ControlBlockRole::StateBody => {
            block.block_name.clone()
        }
        ControlBlockRole::ResetSynchronous => "syncrst".to_string(),
        ControlBlockRole::ResetAsynchronous => "asyncreset".to_string(),
    }
}

fn render_guard(guard: &DecisionTreeGuardRecord) -> String {
    match guard {
        DecisionTreeGuardRecord::SignalIsHigh { signal_name } => format!("<{signal_name}"),
        DecisionTreeGuardRecord::Comparison {
            left_signal,
            operator,
            right,
        } => format!(
            "<{}{}{}",
            left_signal,
            render_comparison_operator(*operator),
            render_value(right)
        ),
    }
}

fn render_action(action: &DecisionTreeActionRecord, root_kind: FsmRootKind) -> String {
    match action {
        DecisionTreeActionRecord::Assign {
            target_signal,
            assignment_kind,
            value,
        } => format!(
            "{} {} {}",
            target_signal,
            render_assignment_operator(*assignment_kind, root_kind),
            render_value(value)
        ),
    }
}

fn render_assignment_operator(
    kind: DecisionTreeAssignmentKind,
    root_kind: FsmRootKind,
) -> &'static str {
    match kind {
        DecisionTreeAssignmentKind::Combinational => "=",
        DecisionTreeAssignmentKind::Sequential => match root_kind {
            FsmRootKind::Fsm => "<=",
            _ => "<-",
        },
    }
}

fn render_system_reset_kind(kind: SystemResetKind) -> &'static str {
    match kind {
        SystemResetKind::Synchronous => "sreset",
        SystemResetKind::Asynchronous => "asreset",
    }
}

fn render_system_reset_kind_name(kind: SystemResetKind) -> &'static str {
    match kind {
        SystemResetKind::Synchronous => "synchronous",
        SystemResetKind::Asynchronous => "asynchronous",
    }
}

fn render_system_reset_timing_relation(relation: SystemResetTimingRelation) -> &'static str {
    match relation {
        SystemResetTimingRelation::SynchronousToClock => "synchronous_to_clock",
        SystemResetTimingRelation::AsynchronousToClock => "asynchronous_to_clock",
    }
}

fn render_system_reset_target_kind(kind: SystemResetTargetKind) -> &'static str {
    match kind {
        SystemResetTargetKind::DataInputPath => "data_input_path",
        SystemResetTargetKind::DedicatedResetPin => "dedicated_reset_pin",
    }
}

fn reset_signal_name_looks_active_low(signal_name: &str) -> bool {
    let lowered = signal_name.to_ascii_lowercase();
    lowered.ends_with("_n")
        || lowered.ends_with("_b")
        || matches!(lowered.as_str(), "rstn" | "rstb" | "resetn" | "resetb")
}

fn render_comparison_operator(operator: DecisionTreeComparisonOperator) -> &'static str {
    match operator {
        DecisionTreeComparisonOperator::Eq => "==",
        DecisionTreeComparisonOperator::NotEq => "!=",
    }
}

fn render_value(value: &DecisionTreeValueRecord) -> String {
    match value {
        DecisionTreeValueRecord::SignalRef { signal_name } => signal_name.clone(),
        DecisionTreeValueRecord::Literal { literal } => literal.clone(),
    }
}

fn render_init_assignment(init_assignment: &InitAssignmentRecord) -> String {
    format!(
        ":= {}={}",
        init_assignment.target_signal,
        render_value(&init_assignment.value)
    )
}

fn extract_signal_tokens(statement: &str) -> BTreeSet<String> {
    let mut tokens = BTreeSet::new();
    let mut current = String::new();

    for ch in statement.chars() {
        if ch.is_ascii_alphanumeric() || ch == '_' {
            current.push(ch);
        } else if !current.is_empty() {
            maybe_record_signal_token(&current, &mut tokens);
            current.clear();
        }
    }

    if !current.is_empty() {
        maybe_record_signal_token(&current, &mut tokens);
    }

    tokens
}

fn maybe_record_signal_token(token: &str, tokens: &mut BTreeSet<String>) {
    if token.chars().all(|ch| ch.is_ascii_digit()) {
        return;
    }

    let lower = token.to_ascii_lowercase();
    if is_signal_stopword(&lower) {
        return;
    }

    let has_alpha = token.chars().any(|ch| ch.is_ascii_alphabetic());
    let all_uppercase_or_digits = token
        .chars()
        .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_');

    if has_alpha && (token.contains('_') || all_uppercase_or_digits) {
        tokens.insert(token.to_string());
    }
}

fn is_signal_stopword(token: &str) -> bool {
    matches!(
        token,
        "a" | "an"
            | "and"
            | "are"
            | "as"
            | "assert"
            | "available"
            | "backend"
            | "be"
            | "channel"
            | "constraint"
            | "data"
            | "for"
            | "in"
            | "is"
            | "it"
            | "may"
            | "modeled"
            | "must"
            | "observed"
            | "or"
            | "remain"
            | "remains"
            | "should"
            | "the"
            | "this"
            | "transport"
            | "until"
            | "when"
    )
}

fn fsm_root_name(document_key_input: &str) -> String {
    let mut root_name = document_key(document_key_input);

    if root_name.is_empty() {
        root_name = "specforge_intent".to_string();
    }

    if root_name
        .chars()
        .next()
        .is_some_and(|ch| ch.is_ascii_digit())
    {
        root_name = format!("specforge_{root_name}");
    }

    root_name
}

fn canonicalize_existing_path(path: &Path) -> Result<PathBuf> {
    if !path.exists() {
        return Err(AppError::MissingPath(path.to_path_buf()));
    }

    Ok(fs::canonicalize(path)?)
}

#[derive(Debug)]
struct SignalInventoryEvidence {
    direction_hint: Option<InterfaceSignalDirection>,
    width_hint: Option<u32>,
    supporting_canonical_ids: BTreeSet<String>,
    mention_categories: BTreeSet<String>,
    automation_confidence: AutomationConfidence,
}

impl Default for SignalInventoryEvidence {
    fn default() -> Self {
        Self {
            direction_hint: None,
            width_hint: None,
            supporting_canonical_ids: BTreeSet::new(),
            mention_categories: BTreeSet::new(),
            automation_confidence: AutomationConfidence::Low,
        }
    }
}

#[derive(Debug, Deserialize)]
struct StageProbe {
    stage: IrStage,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use crate::error::{AppError, Result};
    use crate::ir::adapters::{AdapterArtifact, AdapterTarget, FsmRootKind};
    use crate::ir::evidence::EvidenceIr;
    use crate::ir::intent::IntentIr;
    use crate::ir::semantic::{
        SemanticIr, SystemResetPolarity, SystemResetTargetKind, SystemResetTimingRelation,
    };
    use crate::ir::source::SourceIr;

    fn build_handshake_intent_ir(base: &Path) -> Result<IntentIr> {
        let source = base.join("handshake.md");
        let source_artifact_base = base.join("generated").join("source_ir");
        let evidence_artifact_base = base.join("generated").join("evidence_ir");
        let semantic_artifact_base = base.join("generated").join("semantic_ir");
        let intent_artifact_base = base.join("generated").join("intent_ir");

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
        intent_ir.write_to_disk()?;
        Ok(intent_ir)
    }

    fn build_reset_polarity_name_mismatch_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "reset_name_mismatch.md",
            "# Reset Name Mismatch\nSignal clk is input width 1.\n\nSignal rst is input width 1.\n\nSignal DATA_IN is input width 8.\n\nSignal ACC is output width 8.\n\nClock clk.\n\nReset rst is asynchronous active low.\n\nInit ACC = 8'0.\n\nBlock accumulate: ACC <- DATA_IN.\n",
        )
    }

    fn build_explicit_symbolic_dt_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "symbolic_dt.md",
            "# Explicit Symbolic DT\nSignal SEL is input width 8.\n\nSignal DATA_OUT is output width 8.\n\nSignal PARAM_OUT is output width 8.\n\nSignal ENUM_OUT is output width 1.\n\nConstant C0 = 8'3.\n\nDefine D0 = 8'4.\n\nParam P0 = 8.\n\nEnum mode_t idle = 0.\n\nEnum mode_t busy = 1.\n\nBlock route_constant: DATA_OUT = C0.\n\nBlock route_param: PARAM_OUT = P0.\n\nBlock route_enum when SEL == C0: ENUM_OUT = mode_t.busy.\n\nBlock route_define when SEL == D0: DATA_OUT = D0.\n",
        )
    }

    fn build_selector_control_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "selector_dt.md",
            "# Selector DT\nSignal MODE is input width 2.\n\nSignal OUT is output width 1.\n\nEnum mode_t idle = 0.\n\nEnum mode_t busy = 1.\n\nBlock decode select MODE when MODE == mode_t.idle: OUT = 1.\n",
        )
    }

    fn build_computed_selector_control_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "computed_selector_dt.md",
            "# Computed Selector DT\nSignal A is input width 1.\n\nSignal B is input width 1.\n\nSignal X is output width 1.\n\nSignal Y is output width 1.\n\nBlock choose select A | B when A | B == 0: X = 1.\n\nBlock choose select A | B when A | B == 1: Y = 1.\n",
        )
    }

    fn build_selector_predicate_mismatch_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "selector_predicate_mismatch_dt.md",
            "# Selector Predicate Mismatch DT\nSignal MODE is input width 2.\n\nSignal GO is input width 1.\n\nSignal OUT is output width 1.\n\nBlock decode select MODE when GO: OUT = 1.\n",
        )
    }

    fn build_compound_update_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "compound_update_dt.md",
            "# Compound Update DT\nSignal clk is input width 1.\n\nSignal rst_n is input width 1.\n\nSignal ACC is output width 8.\n\nClock clk.\n\nReset rst_n is asynchronous active low.\n\nInit ACC = 8'0.\n\nConstant STEP = 8'1.\n\nBlock bump: ACC += STEP.\n",
        )
    }

    fn build_explicit_sequential_control_intent_ir(base: &Path) -> Result<IntentIr> {
        let source = base.join("seq_dt.md");
        let source_artifact_base = base.join("generated").join("source_ir");
        let evidence_artifact_base = base.join("generated").join("evidence_ir");
        let semantic_artifact_base = base.join("generated").join("semantic_ir");
        let intent_artifact_base = base.join("generated").join("intent_ir");

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
        intent_ir.write_to_disk()?;
        Ok(intent_ir)
    }

    fn build_incomplete_sequential_control_intent_ir(base: &Path) -> Result<IntentIr> {
        let source = base.join("seq_dt_incomplete.md");
        let source_artifact_base = base.join("generated").join("source_ir");
        let evidence_artifact_base = base.join("generated").join("evidence_ir");
        let semantic_artifact_base = base.join("generated").join("semantic_ir");
        let intent_artifact_base = base.join("generated").join("intent_ir");

        fs::write(
            &source,
            "# Incomplete Sequential Control\nSignal DATA_IN is input width 8.\n\nSignal ACC is output width 8.\n\nBlock accumulate: ACC <- DATA_IN.\n",
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
        intent_ir.write_to_disk()?;
        Ok(intent_ir)
    }

    fn build_explicit_control_intent_ir(base: &Path) -> Result<IntentIr> {
        let source = base.join("comb_dt.md");
        let source_artifact_base = base.join("generated").join("source_ir");
        let evidence_artifact_base = base.join("generated").join("evidence_ir");
        let semantic_artifact_base = base.join("generated").join("semantic_ir");
        let intent_artifact_base = base.join("generated").join("intent_ir");

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
        intent_ir.write_to_disk()?;
        Ok(intent_ir)
    }

    fn build_intent_ir_from_markdown(
        base: &Path,
        source_name: &str,
        markdown: &str,
    ) -> Result<IntentIr> {
        let source = base.join(source_name);
        let source_artifact_base = base.join("generated").join("source_ir");
        let evidence_artifact_base = base.join("generated").join("evidence_ir");
        let semantic_artifact_base = base.join("generated").join("semantic_ir");
        let intent_artifact_base = base.join("generated").join("intent_ir");

        fs::write(&source, markdown)?;

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
        intent_ir.write_to_disk()?;
        Ok(intent_ir)
    }

    fn build_explicit_fsm_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "explicit_fsm.md",
            "# Explicit FSM Control\nSignal clk is input width 1.\n\nSignal rst_n is input width 1.\n\nSignal GO is input width 1.\n\nSignal DONE is input width 1.\n\nSignal DATA_IN is input width 8.\n\nSignal ACC is output width 8.\n\nSignal TRACE is output width 1.\n\nClock clk.\n\nReset rst_n is asynchronous active low.\n\nInit ACC = 8'0.\n\nState idle is initial.\n\nState busy.\n\nBlock idle: ACC <- DATA_IN.\n\nTransition idle -> busy when GO.\n\nBlock busy: ACC <- DATA_IN.\n\nTransition busy -> idle when DONE.\n\nBlock trace when DONE: TRACE = 1.\n",
        )
    }

    fn build_explicit_reset_fsm_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "reset_fsm.md",
            "# Explicit FSM Reset Control\nSignal clk is input width 1.\n\nSignal rst_n is input width 1.\n\nSignal GO is input width 1.\n\nSignal ACC is output width 8.\n\nSignal PULSE_OUT is output width 1.\n\nClock clk.\n\nReset rst_n is asynchronous active low.\n\nInit ACC = 8'0.\n\nState idle is initial.\n\nState busy.\n\nSyncReset clear_acc: ACC <- 8'0.\n\nAsyncReset clear_pulse: public PULSE_OUT = 0.\n\nBlock idle when GO: ACC <- 8'1; -> busy.\n\nBlock busy: ACC <- 8'2; -> idle.\n",
        )
    }

    fn build_explicit_top_composition_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "explicit_top.md",
            "# Explicit Composition\nTop datapath.\n\nTop datapath port result_data is output width 8.\n\nTop datapath child producer uses module producer_core.\n\nTop datapath child consumer uses module consumer_core.\n\nTop datapath link producer.output_data -> consumer.input_data.\n\nTop datapath link consumer.result_data -> result_data.\n\nModule producer_core signal output_data is output width 8.\n\nModule producer_core block produce: output_data = 8'3.\n\nModule consumer_core signal input_data is input width 8.\n\nModule consumer_core signal result_data is output width 8.\n\nModule consumer_core block route: result_data = input_data.\n",
        )
    }

    fn build_missing_child_module_top_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "missing_child_top.md",
            "# Missing Child Module Composition\nTop datapath.\n\nTop datapath port result_data is output width 8.\n\nTop datapath child producer uses module missing_module.\n",
        )
    }

    fn build_missing_initial_fsm_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "missing_initial_fsm.md",
            "# Missing Initial FSM Control\nSignal clk is input width 1.\n\nSignal rst_n is input width 1.\n\nSignal GO is input width 1.\n\nSignal DONE is input width 1.\n\nSignal DATA_IN is input width 8.\n\nSignal ACC is output width 8.\n\nClock clk.\n\nReset rst_n is asynchronous active low.\n\nInit ACC = 8'0.\n\nState idle.\n\nState busy.\n\nBlock idle: ACC <- DATA_IN.\n\nTransition idle -> busy when GO.\n\nBlock busy: ACC <- DATA_IN.\n\nTransition busy -> idle when DONE.\n",
        )
    }

    fn build_unknown_target_fsm_intent_ir(base: &Path) -> Result<IntentIr> {
        build_intent_ir_from_markdown(
            base,
            "unknown_target_fsm.md",
            "# Unknown Transition Target FSM Control\nSignal clk is input width 1.\n\nSignal rst_n is input width 1.\n\nSignal GO is input width 1.\n\nSignal DATA_IN is input width 8.\n\nSignal ACC is output width 8.\n\nClock clk.\n\nReset rst_n is asynchronous active low.\n\nInit ACC = 8'0.\n\nState idle is initial.\n\nState busy.\n\nBlock idle: ACC <- DATA_IN.\n\nTransition idle -> missing_state when GO.\n\nBlock busy: ACC <- DATA_IN.\n",
        )
    }

    #[test]
    fn builds_blocked_dt_centric_fsm_adapter_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_handshake_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;

        assert_eq!(adapter.target, AdapterTarget::Fsm);
        assert_eq!(adapter.required_input_stage.as_str(), "intent_ir");
        assert_eq!(adapter.lowering_status.as_str(), "blocked");
        assert!(
            adapter
                .artifact_layout
                .adapter_artifact_path
                .ends_with("generated/adapters/fsm/handshake/adapter.json")
        );
        assert!(adapter.artifact_layout.emitted_target_path.is_none());
        assert!(
            adapter
                .residual_decisions
                .iter()
                .any(|packet| { packet.packet_id == "fsm_adapter_signal_inventory" })
        );

        let fsm = adapter.fsm.expect("fsm artifact should be present");
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Dt);
        assert_eq!(
            fsm.root_kind_decision.deferred_root_kinds,
            vec![FsmRootKind::Fsm, FsmRootKind::Top]
        );
        assert!(!fsm.renderability.is_renderable);
        assert_eq!(fsm.decision_tree_candidates.len(), 1);
        assert!(
            fsm.signal_inventory
                .iter()
                .any(|signal| signal.signal_name == "READY")
        );
        assert!(
            fsm.signal_inventory
                .iter()
                .any(|signal| signal.signal_name == "VALID")
        );
        assert!(
            fs::metadata(
                artifact_base
                    .join("fsm")
                    .join("handshake")
                    .join("adapter.json")
            )
            .is_ok()
        );
        assert!(
            fs::metadata(
                artifact_base
                    .join("fsm")
                    .join("handshake")
                    .join("handshake.fsm")
            )
            .is_err()
        );

        Ok(())
    }

    #[test]
    fn builds_renderable_standalone_dt_fsm_adapter_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_explicit_control_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;

        assert_eq!(adapter.lowering_status.as_str(), "renderable");
        let emitted_target_path = adapter
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter should emit target text");
        let emitted_text = fs::read_to_string(emitted_target_path)?;
        let fsm = adapter.fsm.expect("fsm artifact should be present");

        assert!(fsm.renderability.is_renderable);
        assert!(fsm.renderable_module.is_some());
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Dt);
        assert_eq!(
            fsm.root_kind_decision.deferred_root_kinds,
            vec![FsmRootKind::Fsm, FsmRootKind::Top]
        );
        assert_eq!(fsm.decision_tree_candidates.len(), 1);
        assert!(fsm.signal_inventory.iter().any(|signal| {
            signal.signal_name == "DATA_IN"
                && signal.width_hint == Some(8)
                && signal.direction_hint.is_some()
        }));
        assert!(emitted_text.contains("(?dt:comb_dt"));
        assert!(emitted_text.contains("(+size"));
        assert!(emitted_text.contains("(DATA_IN 8)"));
        assert!(emitted_text.contains("(DATA_OUT 8)"));
        assert!(emitted_text.contains("(ZERO_FLAG 1)"));
        assert!(emitted_text.contains("(-route_data"));
        assert!(emitted_text.contains("(DATA_OUT = DATA_IN)"));
        assert!(emitted_text.contains("(-flag_zero"));
        assert!(emitted_text.contains("(<DATA_IN==8'0"));
        assert!(emitted_text.contains("(ZERO_FLAG = 1)"));
        assert!(
            adapter
                .residual_decisions
                .iter()
                .all(|packet| packet.packet_id != "fsm_adapter_signal_inventory")
        );
        assert!(
            adapter
                .residual_decisions
                .iter()
                .all(|packet| packet.packet_id != "fsm_adapter_dt_action_graph")
        );

        Ok(())
    }

    #[test]
    fn builds_renderable_structured_fsm_with_reset_blocks() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_explicit_reset_fsm_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;

        assert_eq!(adapter.lowering_status.as_str(), "renderable");
        let emitted_target_path = adapter
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter should emit target text");
        let emitted_text = fs::read_to_string(emitted_target_path)?;
        let fsm = adapter.fsm.expect("fsm artifact should be present");

        assert!(fsm.renderability.is_renderable);
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Fsm);
        assert_eq!(
            fsm.root_kind_decision.deferred_root_kinds,
            vec![FsmRootKind::Top]
        );
        let system_contract = fsm
            .system_contract
            .as_ref()
            .expect("renderable FSM should carry a system contract");
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
        assert!(emitted_text.contains("(?fsm:reset_fsm"));
        assert!(emitted_text.contains("(+system"));
        assert!(emitted_text.contains("(clock clk)"));
        assert!(emitted_text.contains("(asreset rst_n)"));
        assert!(emitted_text.contains("(:= ACC=8'0)"));
        assert!(emitted_text.contains("(-syncrst"));
        assert!(emitted_text.contains("(ACC <= 8'0)"));
        assert!(emitted_text.contains("(-asyncreset"));
        assert!(emitted_text.contains("(PULSE_OUT> = 0)"));
        assert!(emitted_text.contains("(idle"));
        assert!(emitted_text.contains("(busy"));
        assert!(emitted_text.contains("(<GO"));
        assert!(emitted_text.contains("(-> busy)"));
        assert!(emitted_text.contains("(-> idle)"));

        Ok(())
    }

    #[test]
    fn builds_renderable_standalone_sequential_dt_fsm_adapter_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_explicit_sequential_control_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;

        assert_eq!(adapter.lowering_status.as_str(), "renderable");
        let emitted_target_path = adapter
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter should emit target text");
        let emitted_text = fs::read_to_string(emitted_target_path)?;
        let fsm = adapter.fsm.expect("fsm artifact should be present");

        assert!(fsm.renderability.is_renderable);
        let system_contract = fsm
            .system_contract
            .as_ref()
            .expect("renderable sequential DT should carry a system contract");
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
        assert_eq!(fsm.init_assignments.len(), 1);
        assert!(emitted_text.contains("(?dt:seq_dt"));
        assert!(emitted_text.contains("(+system"));
        assert!(emitted_text.contains("(clock clk)"));
        assert!(emitted_text.contains("(asreset rst_n)"));
        assert!(emitted_text.contains("(+size"));
        assert!(emitted_text.contains("(ACC 8)"));
        assert!(emitted_text.contains("(DATA_IN 8)"));
        assert!(emitted_text.contains("(:= ACC=8'0)"));
        assert!(emitted_text.contains("(ACC <- DATA_IN)"));
        assert!(
            adapter
                .residual_decisions
                .iter()
                .all(|packet| packet.packet_id != "fsm_adapter_system_contract")
        );

        Ok(())
    }

    #[test]
    fn keeps_standalone_sequential_dt_blocked_without_system_contract() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_incomplete_sequential_control_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        assert_eq!(adapter.lowering_status.as_str(), "blocked");
        assert!(adapter.artifact_layout.emitted_target_path.is_none());
        assert!(
            adapter
                .residual_decisions
                .iter()
                .any(|packet| { packet.packet_id == "fsm_adapter_system_contract" })
        );
        let fsm = adapter.fsm.expect("fsm artifact should be present");
        assert!(!fsm.renderability.is_renderable);
        assert!(fsm.renderability.blocking_reasons.iter().any(|reason| {
            reason.contains("system contract") || reason.contains("init assignment")
        }));

        Ok(())
    }

    #[test]
    fn keeps_reset_polarity_blocked_when_signal_name_cannot_preserve_it() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_reset_polarity_name_mismatch_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;

        assert_eq!(adapter.lowering_status.as_str(), "blocked");
        let fsm = adapter.fsm.expect("fsm artifact should be present");
        assert!(!fsm.renderability.is_renderable);
        assert!(
            fsm.renderability
                .blocking_reasons
                .iter()
                .any(|reason| { reason.contains("reset polarity through the reset signal name") })
        );

        Ok(())
    }

    #[test]
    fn builds_renderable_symbolic_dt_fsm_adapter_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_explicit_symbolic_dt_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;

        assert_eq!(adapter.lowering_status.as_str(), "renderable");
        let emitted_target_path = adapter
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter should emit target text");
        let emitted_text = fs::read_to_string(emitted_target_path)?;
        let fsm = adapter.fsm.expect("fsm artifact should be present");

        assert!(fsm.renderability.is_renderable);
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Dt);
        assert!(emitted_text.contains("(?dt:symbolic_dt"));
        assert!(emitted_text.contains("(+constants"));
        assert!(emitted_text.contains("(C0 8'3)"));
        assert!(emitted_text.contains("(+define (D0 8'4))"));
        assert!(emitted_text.contains("(+params"));
        assert!(emitted_text.contains("(P0 8)"));
        assert!(emitted_text.contains("(+enums"));
        assert!(emitted_text.contains("(mode_t"));
        assert!(emitted_text.contains("(idle 0)"));
        assert!(emitted_text.contains("(busy 1)"));
        assert!(emitted_text.contains("(DATA_OUT = C0)"));
        assert!(emitted_text.contains("(PARAM_OUT = P0)"));
        assert!(emitted_text.contains("(ENUM_OUT = mode_t.busy)"));
        assert!(emitted_text.contains("(<SEL==C0"));
        assert!(emitted_text.contains("(<SEL==D0"));
        assert!(emitted_text.contains("(DATA_OUT = D0)"));

        Ok(())
    }

    #[test]
    fn builds_renderable_selector_based_dt_fsm_adapter_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_selector_control_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;
        assert_eq!(adapter.lowering_status.as_str(), "renderable");
        let emitted_target_path = adapter
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter should emit target text");
        let emitted_text = fs::read_to_string(emitted_target_path)?;
        let fsm = adapter.fsm.expect("fsm artifact should be present");
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Dt);
        assert!(fsm.renderability.is_renderable);
        assert!(emitted_text.contains("(?dt:selector_dt"));
        assert!(emitted_text.contains("(-decode"));
        assert!(emitted_text.contains("(?MODE"));
        assert!(emitted_text.contains("(=mode_t.idle"));
        assert!(emitted_text.contains("(OUT = 1)"));

        Ok(())
    }

    #[test]
    fn builds_renderable_computed_selector_dt_fsm_adapter_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_computed_selector_control_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;

        assert_eq!(adapter.lowering_status.as_str(), "renderable");
        let emitted_target_path = adapter
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter should emit target text");
        let emitted_text = fs::read_to_string(emitted_target_path)?;
        let fsm = adapter.fsm.expect("fsm artifact should be present");

        assert!(fsm.renderability.is_renderable);
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Dt);
        assert!(emitted_text.contains("(?dt:computed_selector_dt"));
        assert!(emitted_text.contains("(-choose"));
        assert!(emitted_text.contains("(?(| A B)"));
        assert!(emitted_text.contains("(=0"));
        assert!(emitted_text.contains("(=1"));
        assert!(emitted_text.contains("(X = 1)"));
        assert!(emitted_text.contains("(Y = 1)"));

        Ok(())
    }

    #[test]
    fn keeps_selector_based_dt_blocked_when_branch_predicate_is_not_relative_to_selector()
    -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_selector_predicate_mismatch_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;

        assert_eq!(adapter.lowering_status.as_str(), "blocked");
        assert!(adapter.artifact_layout.emitted_target_path.is_none());
        let fsm = adapter.fsm.expect("fsm artifact should be present");
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Dt);
        assert!(!fsm.renderability.is_renderable);
        assert!(fsm.renderability.blocking_reasons.iter().any(|reason| {
            reason.contains("does not map to an explicit `.fsm` test selector token")
        }));

        Ok(())
    }

    #[test]
    fn builds_renderable_compound_update_dt_fsm_adapter_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_compound_update_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;

        assert_eq!(adapter.lowering_status.as_str(), "renderable");
        let emitted_target_path = adapter
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter should emit target text");
        let emitted_text = fs::read_to_string(emitted_target_path)?;
        let fsm = adapter.fsm.expect("fsm artifact should be present");

        assert!(fsm.renderability.is_renderable);
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Dt);
        assert!(emitted_text.contains("(?dt:compound_update_dt"));
        assert!(emitted_text.contains("(asreset rst_n)"));
        assert!(emitted_text.contains("(:= ACC=8'0)"));
        assert!(emitted_text.contains("(+constants"));
        assert!(emitted_text.contains("(STEP 8'1)"));
        assert!(emitted_text.contains("(-bump"));
        assert!(emitted_text.contains("(+= ACC STEP)"));
        assert!(
            adapter
                .residual_decisions
                .iter()
                .all(|packet| packet.packet_id != "fsm_adapter_dt_action_graph")
        );

        Ok(())
    }

    #[test]
    fn builds_renderable_structured_fsm_adapter_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_explicit_fsm_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;

        assert_eq!(adapter.lowering_status.as_str(), "renderable");
        let emitted_target_path = adapter
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter should emit target text");
        let emitted_text = fs::read_to_string(emitted_target_path)?;
        let fsm = adapter.fsm.expect("fsm artifact should be present");

        assert!(fsm.renderability.is_renderable);
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Fsm);
        let system_contract = fsm
            .system_contract
            .as_ref()
            .expect("renderable FSM should carry a system contract");
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
        assert_eq!(fsm.state_candidates.len(), 2);
        assert_eq!(fsm.transition_candidates.len(), 2);
        assert!(fsm.renderable_module.is_some());
        assert!(emitted_text.contains("(?fsm:explicit_fsm"));
        assert!(emitted_text.contains("(+system"));
        assert!(emitted_text.contains("(clock clk)"));
        assert!(emitted_text.contains("(asreset rst_n)"));
        assert!(emitted_text.contains("(:= ACC=8'0)"));
        assert!(emitted_text.contains("\n  (idle\n"));
        assert!(emitted_text.contains("\n  (busy\n"));
        assert!(emitted_text.contains("(ACC <= DATA_IN)"));
        assert!(emitted_text.contains("(<GO"));
        assert!(emitted_text.contains("(-> busy)"));
        assert!(emitted_text.contains("(<DONE"));
        assert!(emitted_text.contains("(-> idle)"));
        assert!(emitted_text.contains("(-trace"));
        assert!(emitted_text.contains("(TRACE = 1)"));
        assert!(!emitted_text.contains("<-"));

        let idle_position = emitted_text
            .find("\n  (idle\n")
            .expect("idle state should be rendered");
        let busy_position = emitted_text
            .find("\n  (busy\n")
            .expect("busy state should be rendered");
        assert!(
            idle_position < busy_position,
            "initial state should render before later states"
        );
        assert!(
            adapter
                .residual_decisions
                .iter()
                .all(|packet| packet.packet_id != "fsm_adapter_signal_inventory")
        );
        assert!(
            adapter
                .residual_decisions
                .iter()
                .all(|packet| packet.packet_id != "fsm_adapter_state_graph")
        );
        assert!(
            adapter
                .residual_decisions
                .iter()
                .all(|packet| packet.packet_id != "fsm_adapter_system_contract")
        );
        assert!(
            adapter
                .residual_decisions
                .iter()
                .all(|packet| packet.packet_id != "fsm_adapter_dt_action_graph")
        );

        Ok(())
    }

    #[test]
    fn builds_renderable_top_composition_fsm_adapter_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_explicit_top_composition_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;
        adapter.write_to_disk()?;

        assert_eq!(adapter.lowering_status.as_str(), "renderable");
        let emitted_target_path = adapter
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter should emit target text");
        let emitted_text = fs::read_to_string(emitted_target_path)?;
        let fsm = adapter.fsm.expect("fsm artifact should be present");

        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Top);
        assert_eq!(
            fsm.root_kind_decision.deferred_root_kinds,
            vec![FsmRootKind::Dt, FsmRootKind::Fsm]
        );
        assert_eq!(fsm.top_candidates.len(), 1);
        assert_eq!(fsm.module_candidates.len(), 2);
        assert!(fsm.renderability.is_renderable);
        assert!(fsm.renderable_module.is_none());
        assert!(fsm.renderable_document.is_some());
        assert!(emitted_text.contains("(?top:datapath"));
        assert!(emitted_text.contains("(?ports:public_io"));
        assert!(emitted_text.contains("result_data>8"));
        assert!(emitted_text.contains("(?dtc:producer producer_core)"));
        assert!(emitted_text.contains("(?dtc:consumer consumer_core)"));
        assert!(emitted_text.contains("(?toplink:wiring"));
        assert!(emitted_text.contains("/producer.output_data/consumer.input_data/"));
        assert!(emitted_text.contains("/consumer.result_data/result_data/"));
        assert!(emitted_text.contains("(?dt:producer_core"));
        assert!(emitted_text.contains("(?dt:consumer_core"));
        assert!(
            adapter
                .residual_decisions
                .iter()
                .all(|packet| packet.packet_id != "fsm_adapter_composition_topology")
        );

        Ok(())
    }

    #[test]
    fn keeps_top_composition_blocked_when_child_module_is_missing() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_missing_child_module_top_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;

        assert_eq!(adapter.lowering_status.as_str(), "blocked");
        assert!(adapter.artifact_layout.emitted_target_path.is_none());
        let fsm = adapter.fsm.expect("fsm artifact should be present");
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Top);
        assert_eq!(fsm.top_candidates.len(), 1);
        assert!(!fsm.renderability.is_renderable);
        assert!(
            fsm.renderability
                .blocking_reasons
                .iter()
                .any(|reason| { reason.contains("missing explicit module `missing_module`") })
        );
        assert!(
            adapter
                .residual_decisions
                .iter()
                .any(|packet| packet.packet_id == "fsm_adapter_composition_topology")
        );

        Ok(())
    }

    #[test]
    fn keeps_structured_fsm_blocked_without_exactly_one_initial_state() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_missing_initial_fsm_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;

        assert_eq!(adapter.lowering_status.as_str(), "blocked");
        assert!(adapter.artifact_layout.emitted_target_path.is_none());
        let fsm = adapter.fsm.expect("fsm artifact should be present");
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Fsm);
        assert!(!fsm.renderability.is_renderable);
        assert!(
            fsm.renderability
                .blocking_reasons
                .iter()
                .any(|reason| { reason.contains("exactly one explicit initial regular state") })
        );
        assert!(
            adapter
                .residual_decisions
                .iter()
                .any(|packet| packet.packet_id == "fsm_adapter_state_graph")
        );

        Ok(())
    }

    #[test]
    fn keeps_structured_fsm_blocked_when_transition_target_is_undeclared() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_unknown_target_fsm_intent_ir(tempdir.path())?;
        let artifact_base = tempdir.path().join("generated").join("adapters");

        let adapter = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Fsm,
            &artifact_base,
        )?;

        assert_eq!(adapter.lowering_status.as_str(), "blocked");
        assert!(adapter.artifact_layout.emitted_target_path.is_none());
        let fsm = adapter.fsm.expect("fsm artifact should be present");
        assert_eq!(fsm.root_kind_decision.selected_root_kind, FsmRootKind::Fsm);
        assert_eq!(fsm.transition_candidates.len(), 1);
        assert!(!fsm.renderability.is_renderable);
        assert!(fsm.renderability.blocking_reasons.iter().any(|reason| {
            reason.contains("Transition target `missing_state` is not declared")
        }));
        assert!(
            adapter
                .residual_decisions
                .iter()
                .any(|packet| packet.packet_id == "fsm_adapter_state_graph")
        );

        Ok(())
    }

    #[test]
    fn rejects_non_intent_artifacts_for_adapter_build() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("README.md");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let adapter_artifact_base = tempdir.path().join("generated").join("adapters");

        fs::write(&source, "# Hello\nVALID must hold until READY.\n")?;

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

        let error = AdapterArtifact::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            AdapterTarget::Fsm,
            &adapter_artifact_base,
        )
        .expect_err("semantic input should be rejected");

        match error {
            AppError::InvalidStageArtifact(message) => {
                assert!(message.contains("must be an IntentIR document"));
            }
            other => panic!("unexpected error: {other}"),
        }

        Ok(())
    }
}
