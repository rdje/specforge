use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::intent::{IntentDocumentIdentity, IntentIr};
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
            (AdapterTarget::Fsm, Some(fsm)) if fsm.renderability.is_renderable => None,
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
    pub supporting_intent_ids: Vec<String>,
    pub mention_categories: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FsmDecisionTreeCandidate {
    pub candidate_id: String,
    pub summary: String,
    pub supporting_behavior_ids: Vec<String>,
    pub supporting_constraint_ids: Vec<String>,
    pub supporting_assumption_ids: Vec<String>,
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
    let renderability = build_renderability();
    let emitted_target_path = renderability
        .is_renderable
        .then(|| artifact_root.join(format!("{root_name}.fsm")));
    let residual_decisions = build_adapter_residual_decisions(
        intent_ir,
        &signal_inventory,
        &decision_tree_candidates,
        &root_kind_decision,
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
    let mut inventory = BTreeMap::<String, SignalEvidence>::new();

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

    inventory
        .into_iter()
        .map(|(signal_name, evidence)| {
            let support_count = evidence.supporting_intent_ids.len();

            FsmSignalCandidate {
                signal_name,
                supporting_intent_ids: evidence.supporting_intent_ids.into_iter().collect(),
                mention_categories: evidence.mention_categories.into_iter().collect(),
                automation_confidence: if support_count >= 2 {
                    AutomationConfidence::Medium
                } else {
                    AutomationConfidence::Low
                },
            }
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
        supporting_behavior_ids: intent_ir
            .behaviors
            .iter()
            .map(|behavior| behavior.behavior_id.clone())
            .collect(),
        supporting_constraint_ids: intent_ir
            .constraints
            .iter()
            .map(|constraint| constraint.constraint_id.clone())
            .collect(),
        supporting_assumption_ids: intent_ir
            .assumptions
            .iter()
            .map(|assumption| assumption.assumption_id.clone())
            .collect(),
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

fn build_renderability() -> FsmRenderability {
    FsmRenderability {
        is_renderable: false,
        blocking_reasons: vec![
            "IntentIR does not yet carry stable signal directions or widths, so `.fsm` port declarations would be invented.".to_string(),
            "IntentIR does not yet carry a backend-neutral DT predicate/action graph, so `.fsm` guards and assignments cannot be emitted safely.".to_string(),
        ],
        required_canonical_enrichments: vec![
            "promote a canonical interface inventory with stable signal names, directions, and widths".to_string(),
            "promote backend-neutral DT predicates and actions that can lower directly into `.fsm` decision trees".to_string(),
            "add explicit regular-state and transition records only when the canonical model truly warrants a `?fsm:name` root".to_string(),
        ],
    }
}

fn build_adapter_residual_decisions(
    intent_ir: &IntentIr,
    signal_inventory: &[FsmSignalCandidate],
    decision_tree_candidates: &[FsmDecisionTreeCandidate],
    root_kind_decision: &FsmRootKindDecision,
) -> Vec<ResidualDecisionPacket> {
    let mut residual_decisions = intent_ir.residual_decisions.clone();

    residual_decisions.push(ResidualDecisionPacket {
        packet_id: "fsm_adapter_signal_inventory".to_string(),
        question: "Which canonical signal inventory should the `.fsm` adapter lower into ports, predicates, and assignments?".to_string(),
        why_unresolved: format!(
            "The current IntentIR artifact only provides low-confidence signal mentions ({}) and does not include stable directions or widths.",
            signal_inventory.len()
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "enrich_intent_ir_interface_inventory".to_string(),
                description: "Add a backend-neutral interface and signal inventory to IntentIR before widening `.fsm` emission.".to_string(),
                downstream_impact: "The adapter can render real `.fsm` roots without inventing ports or widths.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "keep_signal_inventory_plan_only".to_string(),
                description: "Treat the current signal inventory as advisory only and keep the adapter artifact blocked.".to_string(),
                downstream_impact: "The adapter remains honest, but generated `.fsm` text stays unavailable until upstream canonical structure improves.".to_string(),
            },
        ],
    });

    residual_decisions.push(ResidualDecisionPacket {
        packet_id: "fsm_adapter_dt_action_graph".to_string(),
        question: "What backend-neutral DT predicate and action graph should be lowered into `.fsm` branches and assignments?".to_string(),
        why_unresolved: format!(
            "The current `.fsm` adapter slice can identify {} DT candidate(s), but their executable control/data structure is still implicit in natural-language IntentIR records.",
            decision_tree_candidates.len()
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "enrich_intent_ir_with_dt_fragments".to_string(),
                description: "Introduce canonical DT fragments in IntentIR so adapters lower typed predicates and actions instead of text summaries.".to_string(),
                downstream_impact: "DT-centric targets like `.fsm` can become directly renderable and later HDL adapters can share the same canonical control structure.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "defer_target_text_emission".to_string(),
                description: "Preserve the current typed lowering plan without emitting `.fsm` text until canonical DT structure exists.".to_string(),
                downstream_impact: "The adapter remains useful for roadmap validation and gap analysis, but target text stays blocked.".to_string(),
            },
        ],
    });

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

fn register_signal_mentions(
    inventory: &mut BTreeMap<String, SignalEvidence>,
    statement: &str,
    supporting_intent_id: &str,
    mention_category: &str,
) {
    for token in extract_signal_tokens(statement) {
        let evidence = inventory.entry(token).or_default();
        evidence
            .supporting_intent_ids
            .insert(supporting_intent_id.to_string());
        evidence
            .mention_categories
            .insert(mention_category.to_string());
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

#[derive(Debug, Default)]
struct SignalEvidence {
    supporting_intent_ids: BTreeSet<String>,
    mention_categories: BTreeSet<String>,
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
