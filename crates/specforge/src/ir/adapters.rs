use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::intent::{IntentDocumentIdentity, IntentIr};
use crate::ir::semantic::{
    DecisionTreeActionRecord, DecisionTreeAssignmentKind, DecisionTreeComparisonOperator,
    DecisionTreeFragmentRecord, DecisionTreeGuardRecord, DecisionTreeValueRecord,
    InterfaceSignalDirection,
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
            (AdapterTarget::Fsm, Some(fsm)) if fsm.renderability.is_renderable => Some(
                render_fsm_module(&fsm.root_name, fsm.renderable_module.as_ref()?),
            ),
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
    pub decision_tree_candidates: Vec<FsmDecisionTreeCandidate>,
    pub state_candidates: Vec<FsmStateCandidate>,
    pub renderability: FsmRenderability,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub renderable_module: Option<FsmRenderableModule>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum FsmRootKind {
    Dt,
    Fsm,
    Top,
    Mod,
    Module,
}

impl FsmRootKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dt => "dt",
            Self::Fsm => "fsm",
            Self::Top => "top",
            Self::Mod => "mod",
            Self::Module => "module",
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
    pub summary: String,
    pub supporting_behavior_ids: Vec<String>,
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
    pub size_entries: Vec<FsmRenderableSizeEntry>,
    pub blocks: Vec<DecisionTreeFragmentRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmRenderableSizeEntry {
    pub signal_name: String,
    pub direction_hint: InterfaceSignalDirection,
    pub width: u32,
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
    let root_name = fsm_root_name(&intent_ir.document_identity.document_key);
    let signal_inventory = build_signal_inventory(intent_ir);
    let state_candidates = build_state_candidates(intent_ir);
    let root_kind_decision = build_root_kind_decision(&state_candidates);
    let decision_tree_candidates = build_decision_tree_candidates(intent_ir, &signal_inventory);
    let (renderability, renderable_module) = analyze_renderability(
        &signal_inventory,
        &decision_tree_candidates,
        &root_kind_decision,
    );
    let emitted_target_path = renderability
        .is_renderable
        .then(|| artifact_root.join(format!("{root_name}.fsm")));
    let residual_decisions = build_adapter_residual_decisions(
        intent_ir,
        &signal_inventory,
        &decision_tree_candidates,
        &root_kind_decision,
        &renderability,
    );
    let lowering_status = if renderability.is_renderable {
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
            "typed DT-centric `.fsm` lowering artifact for {}",
            intent_ir.document_identity.display_name
        ),
    };
    let fsm = FsmAdapterArtifact {
        root_name,
        root_kind_decision,
        signal_inventory,
        decision_tree_candidates,
        state_candidates,
        renderability,
        renderable_module,
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
    let mut inventory = BTreeMap::<String, SignalInventoryEvidence>::new();

    for interface in &intent_ir.interfaces {
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

    for fragment in &intent_ir.decision_tree_fragments {
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

fn build_state_candidates(intent_ir: &IntentIr) -> Vec<FsmStateCandidate> {
    let mut state_candidates = Vec::new();
    let mut seen = BTreeSet::new();

    for behavior in &intent_ir.behaviors {
        let lower = behavior.statement.to_ascii_lowercase();
        if !lower.contains("state")
            && !lower.contains("transition")
            && !lower.contains("next_state")
        {
            continue;
        }

        let state_id = format!("state_hint_{}", document_key(&behavior.behavior_id));
        if !seen.insert(state_id.clone()) {
            continue;
        }

        state_candidates.push(FsmStateCandidate {
            state_id,
            summary: format!(
                "low-confidence state-sequencing hint derived from behavior intent: {}",
                normalize_sentence(&behavior.statement)
            ),
            supporting_behavior_ids: vec![behavior.behavior_id.clone()],
            automation_confidence: AutomationConfidence::Low,
        });
    }

    state_candidates
}

fn build_root_kind_decision(state_candidates: &[FsmStateCandidate]) -> FsmRootKindDecision {
    let has_confident_state_model = state_candidates.iter().any(|candidate| {
        matches!(
            candidate.automation_confidence,
            AutomationConfidence::High | AutomationConfidence::Medium
        )
    });

    if has_confident_state_model {
        return FsmRootKindDecision {
            selected_root_kind: FsmRootKind::Fsm,
            deferred_root_kinds: vec![FsmRootKind::Top, FsmRootKind::Mod, FsmRootKind::Module],
            automation_confidence: AutomationConfidence::Medium,
            rationale: "canonical sequencing evidence is strong enough to justify a true `?fsm:name` lowering root".to_string(),
        };
    }

    FsmRootKindDecision {
        selected_root_kind: FsmRootKind::Dt,
        deferred_root_kinds: vec![
            FsmRootKind::Fsm,
            FsmRootKind::Top,
            FsmRootKind::Mod,
            FsmRootKind::Module,
        ],
        automation_confidence: AutomationConfidence::Medium,
        rationale: "the current IntentIR does not yet carry explicit regular-state progression, so the first `.fsm` adapter slice defaults to a standalone `?dt:name` plan rather than inventing FSM or composition semantics".to_string(),
    }
}

fn build_decision_tree_candidates(
    intent_ir: &IntentIr,
    signal_inventory: &[FsmSignalCandidate],
) -> Vec<FsmDecisionTreeCandidate> {
    if !intent_ir.decision_tree_fragments.is_empty() {
        let referenced_signal_names = intent_ir
            .decision_tree_fragments
            .iter()
            .flat_map(|fragment| fragment.referenced_signal_names.iter().cloned())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect();

        return vec![FsmDecisionTreeCandidate {
            candidate_id: "dt_primary_intent_cone".to_string(),
            summary: format!(
                "primary canonical control candidate aggregates {} typed block(s) for {}",
                intent_ir.decision_tree_fragments.len(),
                intent_ir.document_identity.display_name
            ),
            supporting_fragment_ids: intent_ir
                .decision_tree_fragments
                .iter()
                .map(|fragment| fragment.fragment_id.clone())
                .collect(),
            blocks: intent_ir.decision_tree_fragments.clone(),
            referenced_signal_names,
            automation_confidence: fold_automation_confidence(
                intent_ir
                    .decision_tree_fragments
                    .iter()
                    .map(|fragment| fragment.automation_confidence),
            ),
        }];
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

fn analyze_renderability(
    signal_inventory: &[FsmSignalCandidate],
    decision_tree_candidates: &[FsmDecisionTreeCandidate],
    root_kind_decision: &FsmRootKindDecision,
) -> (FsmRenderability, Option<FsmRenderableModule>) {
    let mut blocking_reasons = Vec::new();
    let mut required_canonical_enrichments = BTreeSet::new();

    if !matches!(root_kind_decision.selected_root_kind, FsmRootKind::Dt) {
        push_unique_message(
            &mut blocking_reasons,
            "This adapter slice only emits standalone `?dt:name` roots; canonical sequencing or composition still points beyond the current lowering boundary.",
        );
        required_canonical_enrichments.insert(
            "add explicit regular-state and composition records before widening beyond standalone `?dt:name`".to_string(),
        );
    }

    let blocks = decision_tree_candidates
        .iter()
        .flat_map(|candidate| candidate.blocks.iter().cloned())
        .collect::<Vec<_>>();
    if blocks.is_empty() {
        push_unique_message(
            &mut blocking_reasons,
            "IntentIR does not yet carry typed guarded/action blocks, so `.fsm` control cannot be emitted safely.",
        );
        required_canonical_enrichments.insert(
            "promote backend-neutral control fragments with explicit guards and assignments"
                .to_string(),
        );
    }

    let signals_by_name: BTreeMap<String, &FsmSignalCandidate> = signal_inventory
        .iter()
        .map(|signal| (signal.signal_name.clone(), signal))
        .collect();
    let mut size_entries = BTreeMap::<String, FsmRenderableSizeEntry>::new();
    let mut driven_outputs = BTreeSet::new();
    let mut seen_block_names = BTreeSet::new();

    for block in &blocks {
        if !seen_block_names.insert(block.block_name.clone()) {
            push_unique_message(
                &mut blocking_reasons,
                &format!(
                    "Canonical control block `{}` is duplicated; the first renderable `.fsm` slice expects one unique top-level block per name.",
                    block.block_name
                ),
            );
            required_canonical_enrichments.insert(
                "normalize canonical control blocks to one stable top-level block per name"
                    .to_string(),
            );
        }

        if block.actions.is_empty() {
            push_unique_message(
                &mut blocking_reasons,
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
                &signals_by_name,
                &mut size_entries,
                &mut blocking_reasons,
                &mut required_canonical_enrichments,
            );
        }

        for action in &block.actions {
            validate_action_renderability(
                action,
                &signals_by_name,
                &mut size_entries,
                &mut driven_outputs,
                &mut blocking_reasons,
                &mut required_canonical_enrichments,
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
        size_entries: size_entries.into_values().collect(),
        blocks,
    });

    (renderability, renderable_module)
}

fn build_adapter_residual_decisions(
    intent_ir: &IntentIr,
    signal_inventory: &[FsmSignalCandidate],
    decision_tree_candidates: &[FsmDecisionTreeCandidate],
    root_kind_decision: &FsmRootKindDecision,
    renderability: &FsmRenderability,
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

    let dt_surface_blocked = decision_tree_candidates
        .iter()
        .all(|candidate| candidate.blocks.is_empty())
        || renderability.blocking_reasons.iter().any(|reason| {
            reason.contains("control block")
                || reason.contains("guard")
                || reason.contains("assignment")
                || reason.contains("sequential")
        });
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

    residual_decisions.push(ResidualDecisionPacket {
        packet_id: "fsm_adapter_root_kind_expansion".to_string(),
        question: "When should the `.fsm` adapter escalate from a `?dt:name` root to `?fsm:name`, `?top:name`, `?mod:name`, or `?module:name`?".to_string(),
        why_unresolved: format!(
            "The adapter currently selects `?{}:name` because IntentIR does not yet contain explicit regular-state or composition records strong enough for the broader root kinds.",
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
                interpretation_id: "defer_composition_roots".to_string(),
                description: "Keep composition-level roots out of the first slice until the canonical model carries module/top structure explicitly.".to_string(),
                downstream_impact: "The adapter remains DT-centric and aligned with the roadmap instead of overloading the first target slice.".to_string(),
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
                push_unique_message(
                    blocking_reasons,
                    "The first renderable `.fsm` slice stays combinational; sequential assignments still need canonical init/system-contract support.",
                );
                required_canonical_enrichments.insert(
                    "promote backend-neutral init/reset/system-contract facts before lowering sequential actions"
                        .to_string(),
                );
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

fn render_fsm_module(root_name: &str, module: &FsmRenderableModule) -> String {
    let mut lines = vec![format!("(?dt:{root_name}")];

    if !module.size_entries.is_empty() {
        lines.push("  (+size".to_string());
        for entry in &module.size_entries {
            lines.push(format!("    ({} {})", entry.signal_name, entry.width));
        }
        lines.push("  )".to_string());
    }

    for block in &module.blocks {
        lines.push(format!("  (-{}", block.block_name));
        if let Some(guard) = block.guard.as_ref() {
            lines.push(format!("    ({}", render_guard(guard)));
            for action in &block.actions {
                lines.push(format!("      ({})", render_action(action)));
            }
            lines.push("    )".to_string());
        } else {
            for action in &block.actions {
                lines.push(format!("    ({})", render_action(action)));
            }
        }
        lines.push("  )".to_string());
    }

    lines.push(")".to_string());
    format!("{}\n", lines.join("\n"))
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

fn render_action(action: &DecisionTreeActionRecord) -> String {
    match action {
        DecisionTreeActionRecord::Assign {
            target_signal,
            assignment_kind,
            value,
        } => format!(
            "{} {} {}",
            target_signal,
            render_assignment_operator(*assignment_kind),
            render_value(value)
        ),
    }
}

fn render_assignment_operator(kind: DecisionTreeAssignmentKind) -> &'static str {
    match kind {
        DecisionTreeAssignmentKind::Combinational => "=",
        DecisionTreeAssignmentKind::Sequential => "<-",
    }
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

fn normalize_sentence(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
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
    use crate::ir::semantic::SemanticIr;
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
