use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::{
    EvidenceIr, InterfaceEdgeTimingRecord, ProtocolStateRecord, SerialFrameField, SignalPolarity,
    SignalPolarityConflictRecord, SignalPolarityRecord, SignalSemanticConflictRecord,
    SignalSemanticHintRecord, SignalSemanticHintSourceKind, SignalSemanticTag, StatementClass,
    SwdOperation, VisualEvidenceRole, VisualObservationKind, parse_visual_observation_json,
};
use crate::ir::prior_memory::{
    CorpusMemory, ProtocolFamily, is_meaningful_actor_term, normalize_actor_term,
};
use crate::ir::source::{
    ActorSignalRelation, AutomationConfidence, CandidateInterpretation, RelationKind,
    ResidualDecisionPacket, ValidationReportRecord, WidthHint, document_key,
};
use crate::persisted_path::{
    PersistedPathOrigin, normalize_for_storage, resolve_existing, resolve_repository_output,
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticIr {
    pub schema_version: u32,
    pub stage: IrStage,
    pub evidence_ir_path: PathBuf,
    pub artifact_layout: SemanticArtifactLayout,
    pub document_identity: SemanticDocumentIdentity,
    pub actors: Vec<ActorRecord>,
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
    pub regular_states: Vec<RegularStateRecord>,
    #[serde(default)]
    pub state_transitions: Vec<StateTransitionRecord>,
    #[serde(default)]
    pub symbol_definitions: Vec<SymbolDefinitionRecord>,
    /// KG-ISF-TRANSACTIONS.2a: the transactions the document NAMES in its
    /// section headings (Cue A — `<qualifier> transfer/transaction/operation`),
    /// recovered by universal English structural grammar (no chip-spec name
    /// list; ADR 0006). This is the recognition substrate the IntentIR
    /// transaction recognizer mints typed transactions from. Serde-skipped
    /// while empty ⇒ zero artifact churn on docs that name no transactions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transaction_anchors: Vec<TransactionAnchorRecord>,
    /// KG-ISF-TRANSACTIONS.2g: the protocol PHASES the document NAMES in its
    /// prose (Cue — the document's own `<qualifier> phase` vocabulary:
    /// "address phase", "data phase", "setup phase", "access phase",
    /// "response phase", "turnaround phase", …), recovered by universal English
    /// structural grammar (no chip-spec name list; ADR 0006). This is the
    /// recognition substrate the later ordered multi-phase transaction body
    /// (`.2h` ordering, `.2i` membership-by-phase + composition) sequences a
    /// transaction's signal-set membership through; it is recognition-only and
    /// not yet lowered to `.isf`. Serde-skipped while empty ⇒ zero artifact
    /// churn on docs that name no phases.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transaction_phases: Vec<TransactionPhaseRecord>,
    /// KG-ISF-TRANSACTIONS.2m: each declared signal's document-grounded CHANNEL (from the
    /// `<role> channel signals` table captions), carried verbatim from EvidenceIR so the
    /// IntentIR transaction recognizer can group a transaction's signal-set membership by
    /// channel. Serde-skipped while empty ⇒ zero artifact churn on docs without channels.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signal_channel_memberships: Vec<crate::ir::evidence::SignalChannelMembershipRecord>,
    /// SWD-SERIAL-EXTRACTION.7b: serial-frame facts projected losslessly from EvidenceIR. These
    /// remain extraction records, including original ids/order/provenance; SemanticIR does not
    /// reinterpret incomplete frame bindings as executable transactions (ADR 0016).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub serial_frame_fields: Vec<SerialFrameField>,
    /// Protocol operation branches projected losslessly from EvidenceIR (ADR 0016).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub swd_operations: Vec<SwdOperation>,
    /// Protocol state observations projected losslessly from EvidenceIR (ADR 0016). These records
    /// do not imply transitions, guards, initial state, or encoding when those facts are absent.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub protocol_states: Vec<ProtocolStateRecord>,
    /// Interface-edge timing observations projected losslessly from EvidenceIR (ADR 0016).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interface_edge_timings: Vec<InterfaceEdgeTimingRecord>,
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
    /// Clock-tick temporal rules derived from timing, constraint, and conditional evidence.
    #[serde(default)]
    pub temporal_rules: Vec<TemporalRuleRecord>,
    /// ContractIR — typed timed-contract projection (R16-CONTRACT-IR).
    /// Named `actor_contracts` to avoid collision with the pre-existing
    /// `contracts: Vec<ContractRecord>` (semantic protocol contracts).
    /// Additive and empty until `R16-CONTRACT-IR.3` wires producers and
    /// re-points `.isf` lowering (serde-skipped while empty: zero
    /// artifact/fixture change in `.2`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actor_contracts: Vec<crate::ir::contract::ActorContract>,
    /// CVE-PROSE-EXTRACTION: producer-time constrained-extraction stats
    /// (carried from `EvidenceIR`). `None` until `extract-contracts` runs;
    /// the `validate` `constrained:` block reads `schema_rejects` from it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub constrained_extraction_stats: Option<crate::ir::cve::ConstrainedExtractionStats>,
    /// Protocol-structure KG (R16-KG-PROTOCOL-ONTOLOGY): channels /
    /// protocol phases / transactions / handshake pairs. Additive and
    /// empty until extraction (R16 #3/#4/#6) populates it; serde-skipped
    /// while empty ⇒ zero artifact churn.
    #[serde(
        default,
        skip_serializing_if = "crate::ir::protocol_graph::ProtocolGraph::is_empty"
    )]
    pub protocol_graph: crate::ir::protocol_graph::ProtocolGraph,
    /// Capture-fidelity findings (R16-CAPTURE-FIDELITY-GATES). Additive
    /// and empty until `R16-CAPTURE-FIDELITY-GATES.3` wires the
    /// producer (serde-skipped while empty ⇒ zero artifact churn).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fidelity_findings: Vec<crate::ir::fidelity::FidelityFinding>,
    /// Explicit conflicts detected across contradictory temporal value obligations.
    #[serde(default)]
    pub temporal_conflicts: Vec<TemporalConflictRecord>,
    /// Level 2 NLP: structured signal constraint records from `SignalValueConstraint` sentences.
    #[serde(default)]
    pub signal_constraints: Vec<SignalConstraintRecord>,
    /// Level 2 NLP: structured conditional rule records from `ConditionalRule` sentences.
    #[serde(default)]
    pub conditional_rules: Vec<ConditionalRuleRecord>,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
    #[serde(default)]
    pub validation_reports: Vec<ValidationReportRecord>,
}

impl SemanticIr {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let path = resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
        let semantic_ir = serde_json::from_str::<Self>(&fs::read_to_string(path)?)?;
        if !matches!(semantic_ir.stage, IrStage::SemanticIr) {
            return Err(AppError::InvalidStageArtifact(
                "artifact must be a SemanticIR document before loading SemanticIR".to_string(),
            ));
        }
        semantic_ir.runtime_clone()
    }

    pub fn build(evidence_ir_path: &Path, artifact_base_root: &Path) -> Result<Self> {
        let evidence_ir_path =
            resolve_existing(evidence_ir_path, PersistedPathOrigin::RepositoryOwned)?;
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
        }
        .runtime_layout()?;
        let document_identity = SemanticDocumentIdentity {
            document_key: evidence_ir.document_identity.document_key.clone(),
            display_name: evidence_ir.document_identity.display_name.clone(),
        };

        let prior_guidance = load_semantic_prior_guidance(
            evidence_ir.prior_memory_path.as_deref(),
            &document_identity.document_key,
            &document_identity.display_name,
        )?;
        let context = SemanticContext::from_evidence_ir(&evidence_ir);
        let system_contract = build_system_contract(&context);
        let (interfaces, interface_signal_conflicts) = build_interfaces(
            &context,
            prior_guidance.as_ref(),
            evidence_ir.signal_polarities.as_slice(),
            system_contract.as_ref(),
        );
        let actor_build = build_actors(&context, &interfaces);
        let actor_ports = build_actor_ports(&context, &interfaces, system_contract.as_ref());
        let signal_connectivity = build_signal_connectivity(&actor_ports, system_contract.as_ref());
        let infrastructure_signals =
            build_infrastructure_signals(&context, &signal_connectivity, system_contract.as_ref());
        let signal_connectivity_conflicts =
            build_signal_connectivity_conflicts(signal_connectivity.as_slice());
        let signal_polarities = evidence_ir.signal_polarities.clone();
        let signal_polarity_conflicts = evidence_ir.signal_polarity_conflicts.clone();
        let signal_semantic_conflicts = evidence_ir.signal_semantic_conflicts.clone();
        let phases = build_phases(&context);
        let interface_ids_by_signal = interface_ids_by_signal(&interfaces);
        let invariants = build_invariants(&context, &interface_ids_by_signal);
        let contracts = build_contracts(&context, &actor_build.actor_id_by_term);
        let gates = build_gates(&context, &interface_ids_by_signal);
        let assertions = build_assertions(&context);
        let abstractions = build_abstractions(&context);
        let decomposition_candidates = build_decomposition_candidates(&context);
        let regular_states = build_regular_states(&context);
        let state_transitions = build_state_transitions(&context);
        let symbol_definitions = build_symbol_definitions(&context);
        let control_blocks = build_control_blocks(
            &context,
            regular_states.as_slice(),
            symbol_definitions.as_slice(),
        );
        let explicit_modules = build_explicit_modules(&context);
        let explicit_tops = build_explicit_tops(&context);
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

        let mut signal_constraints = if declared_signal_names.is_empty() {
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
        let known_actor_names =
            collect_known_actor_names(actor_build.actors.as_slice(), actor_ports.as_slice());
        let mut vlm_known_signal_names: HashSet<String> = interfaces
            .iter()
            .flat_map(|iface| &iface.signal_records)
            .map(|record| record.signal_name.clone())
            .collect();
        vlm_known_signal_names.extend(
            context
                .statements
                .iter()
                .flat_map(|statement| statement.signals.iter().cloned()),
        );

        // Merge timing constraints: table-synthesized + VLM diagram observations.
        let mut timing_constraints = evidence_ir.timing_constraints.clone();
        let (vlm_timing, vlm_signal_constraints, vlm_states, vlm_transitions) =
            extract_records_from_vlm_observations(&evidence_ir, &vlm_known_signal_names);
        timing_constraints.extend(vlm_timing);
        signal_constraints.extend(vlm_signal_constraints);
        let temporal_rules = build_temporal_rules(
            &context,
            interfaces.as_slice(),
            system_contract.as_ref(),
            signal_connectivity.as_slice(),
            signal_constraints.as_slice(),
            conditional_rules.as_slice(),
            timing_constraints.as_slice(),
            &known_actor_names,
            prior_guidance.as_ref(),
        );
        // R16-CONTRACT-IR.3: project the typed ContractIR alongside
        // `temporal_rules` (lossless 1:1; `.isf` lowering consumes this).
        let mut actor_contracts = temporal_rules
            .iter()
            .map(crate::ir::contract::contract_from_temporal_rule)
            .collect::<Vec<_>>();
        // CVE-PROSE-EXTRACTION.2: fold prose-extracted contracts (from the
        // `extract-contracts` command — already fails-closed parsed +
        // entailment-gated when written) into `actor_contracts` BEFORE fusion
        // / fidelity, so they flow through the same honesty pipeline as the
        // temporal-rule projection.
        crate::ir::cve::fold_extracted_contracts(
            &mut actor_contracts,
            &evidence_ir.extracted_contracts,
        );
        let constrained_extraction_stats = evidence_ir.constrained_extraction_stats.clone();
        // R16-MULTIMODAL-CONTRACT-FUSION.3: cluster `actor_contracts`
        // by `fusion_key` and merge each multi-element cluster
        // deterministically; disagreements route to `Residual` (the
        // honesty doctrine, mechanically enforced — never a silent
        // pick across contradicting sources). Runs BEFORE the
        // fidelity gates so they evaluate the fused contracts.
        crate::ir::fusion::apply_fusion(&mut actor_contracts);
        // R16-CAPTURE-FIDELITY-GATES.3: evaluate the typed fidelity
        // gates over `actor_contracts` and **mechanically enforce the
        // residual-honesty doctrine** — any `Fail` on a `Lowerable`
        // contract is rerouted to `Residual{reason}` BEFORE the `.isf`
        // adapter consumes it (the doctrine becomes structural, not
        // only authorial). The recorded findings reflect what the gates
        // saw pre-routing (a `Fail` finding paired with a now-Residual
        // contract = the doctrine working).
        let fidelity_findings = apply_fidelity_gates(&mut actor_contracts, &actor_ports);
        let temporal_conflicts = build_temporal_conflicts(
            &temporal_rules,
            signal_polarities.as_slice(),
            system_contract.as_ref(),
        );
        let residual_decisions = build_residual_decisions(
            &context,
            &interfaces,
            actor_build.explicit_actor_count,
            temporal_rules.as_slice(),
        );

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

        // KG-ISF-TRANSACTIONS.2a: recover the document's NAMED transactions from
        // its section headings (Cue A). Structural English grammar only; ADR 0006.
        // .2c: the declared-signal inventory grounds each transaction's signal set.
        let transaction_anchors = build_transaction_anchors(&context, &declared_signal_names);

        // KG-ISF-TRANSACTIONS.2g: recover the protocol PHASES the document names
        // in its prose (`<qualifier> phase`). Recognition only — no body
        // composition (`.2i`); universal English grammar, no name list (ADR 0006).
        let transaction_phases = build_transaction_phases(&context, &declared_signal_names);

        Ok(Self {
            schema_version: 1,
            stage: IrStage::SemanticIr,
            evidence_ir_path,
            artifact_layout,
            document_identity,
            actors: actor_build.actors,
            actor_signal_relations: context.actor_signal_relations.clone(),
            actor_ports,
            signal_connectivity,
            infrastructure_signals,
            interface_signal_conflicts,
            signal_connectivity_conflicts,
            signal_polarities,
            signal_polarity_conflicts,
            signal_semantic_conflicts,
            interfaces,
            phases,
            invariants,
            contracts,
            gates,
            assertions,
            abstractions,
            decomposition_candidates,
            system_contract,
            regular_states: regular_states_with_vlm,
            state_transitions: state_transitions_with_vlm,
            symbol_definitions,
            transaction_anchors,
            transaction_phases,
            signal_channel_memberships: evidence_ir.signal_channel_memberships.clone(),
            serial_frame_fields: evidence_ir.serial_frame_fields.clone(),
            swd_operations: evidence_ir.swd_operations.clone(),
            protocol_states: evidence_ir.protocol_states.clone(),
            interface_edge_timings: evidence_ir.interface_edge_timings.clone(),
            control_blocks,
            explicit_modules,
            explicit_tops,
            register_records,
            timing_constraints,
            temporal_rules,
            protocol_graph: crate::ir::protocol_graph::ProtocolGraph {
                handshakes: crate::ir::protocol_graph::project_handshake_pairs(&actor_contracts),
                ..Default::default()
            },
            actor_contracts,
            constrained_extraction_stats,
            fidelity_findings,
            temporal_conflicts,
            signal_constraints,
            conditional_rules,
            residual_decisions,
            validation_reports: Vec::new(),
        })
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.persisted_clone()?)?)
    }

    pub fn write_to_disk(&self) -> Result<()> {
        let persisted = self.persisted_clone()?;
        let runtime_layout = persisted.artifact_layout.runtime_layout()?;
        fs::create_dir_all(&runtime_layout.artifact_root)?;
        fs::write(
            &runtime_layout.semantic_ir_path,
            serde_json::to_string_pretty(&persisted)?,
        )?;
        Ok(())
    }

    fn persisted_clone(&self) -> Result<Self> {
        let mut persisted = self.clone();
        persisted.evidence_ir_path = normalize_for_storage(
            &persisted.evidence_ir_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        persisted.artifact_layout.normalize_for_storage()?;
        Ok(persisted)
    }

    fn runtime_clone(&self) -> Result<Self> {
        let mut runtime = self.clone();
        runtime.evidence_ir_path = resolve_existing(
            &runtime.evidence_ir_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        runtime.artifact_layout = runtime.artifact_layout.runtime_layout()?;
        Ok(runtime)
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

fn is_false(value: &bool) -> bool {
    !*value
}

fn is_zero(value: &u32) -> bool {
    *value == 0
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticArtifactLayout {
    pub artifact_root: PathBuf,
    pub semantic_ir_path: PathBuf,
}

impl SemanticArtifactLayout {
    fn normalize_for_storage(&mut self) -> Result<()> {
        self.artifact_root =
            normalize_for_storage(&self.artifact_root, PersistedPathOrigin::RepositoryOwned)?;
        self.semantic_ir_path =
            normalize_for_storage(&self.semantic_ir_path, PersistedPathOrigin::RepositoryOwned)?;
        Ok(())
    }

    fn runtime_layout(&self) -> Result<Self> {
        Ok(Self {
            artifact_root: resolve_repository_output(&self.artifact_root)?,
            semantic_ir_path: resolve_repository_output(&self.semantic_ir_path)?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticDocumentIdentity {
    pub document_key: String,
    pub display_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorRecord {
    pub actor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor_name: Option<String>,
    pub role_summary: String,
    pub supporting_statement_ids: Vec<String>,
    pub supporting_section_ids: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ActorRelativeDirection {
    Input,
    Output,
    InOut,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ActorPortRecord {
    pub actor_id: String,
    pub actor_name: String,
    pub signal_name: String,
    pub direction: ActorRelativeDirection,
    #[serde(default)]
    pub relation_basis: Vec<RelationKind>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_hint: Option<WidthHint>,
    #[serde(default)]
    pub source_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SignalConnectivityClass {
    #[default]
    Protocol,
    SystemClock,
    SystemReset,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalConnectivityRecord {
    pub signal_name: String,
    #[serde(default)]
    pub connectivity_class: SignalConnectivityClass,
    #[serde(default)]
    pub producer_actor_ids: Vec<String>,
    #[serde(default)]
    pub producer_actor_names: Vec<String>,
    #[serde(default)]
    pub consumer_actor_ids: Vec<String>,
    #[serde(default)]
    pub consumer_actor_names: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width_hint: Option<WidthHint>,
    #[serde(default)]
    pub source_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureSignalKind {
    SystemClock,
    SystemReset,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureSignalSourceStatus {
    UnresolvedSource,
    RecoveredProducer,
    MultipleRecoveredProducers,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureSignalDistributionStatus {
    NoRecoveredConsumers,
    SingleRecoveredConsumer,
    SharedRecoveredConsumers,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InfrastructureTopologyKind {
    ClockGatedBranch,
    ResetSynchronizerStages,
    ResetTreeTargets,
}

impl InfrastructureTopologyKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::ClockGatedBranch => "clock_gated_branch",
            Self::ResetSynchronizerStages => "reset_synchronizer_stages",
            Self::ResetTreeTargets => "reset_tree_targets",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InfrastructureTopologyRecord {
    pub topology_id: String,
    pub topology_kind: InfrastructureTopologyKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub component_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub stage_count: Option<u32>,
    #[serde(default)]
    pub target_actor_ids: Vec<String>,
    #[serde(default)]
    pub target_actor_names: Vec<String>,
    pub supporting_statement_id: String,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InfrastructureSignalRecord {
    pub signal_name: String,
    pub kind: InfrastructureSignalKind,
    pub source_status: InfrastructureSignalSourceStatus,
    #[serde(default)]
    pub recovered_source_actor_ids: Vec<String>,
    #[serde(default)]
    pub recovered_source_actor_names: Vec<String>,
    pub distribution_status: InfrastructureSignalDistributionStatus,
    #[serde(default)]
    pub distributed_to_actor_ids: Vec<String>,
    #[serde(default)]
    pub distributed_to_actor_names: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub infrastructure_topology: Vec<InfrastructureTopologyRecord>,
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SignalConnectivityConflictKind {
    MultipleProducers,
}

impl SignalConnectivityConflictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MultipleProducers => "multiple_producers",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SignalConnectivityConflictRecord {
    pub conflict_id: String,
    pub signal_name: String,
    pub conflict_kind: SignalConnectivityConflictKind,
    #[serde(default)]
    pub conflicting_actor_ids: Vec<String>,
    #[serde(default)]
    pub conflicting_actor_names: Vec<String>,
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceSignalConflictKind {
    DirectionMismatch,
    WidthMismatch,
}

impl InterfaceSignalConflictKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DirectionMismatch => "direction_mismatch",
            Self::WidthMismatch => "width_mismatch",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterfaceSignalConflictObservationRecord {
    pub value_text: String,
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterfaceSignalConflictRecord {
    pub conflict_id: String,
    pub signal_name: String,
    pub conflict_kind: InterfaceSignalConflictKind,
    #[serde(default)]
    pub observations: Vec<InterfaceSignalConflictObservationRecord>,
    pub automation_confidence: AutomationConfidence,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_polarity: Option<SignalPolarity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic_tags: Vec<SignalSemanticTag>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic_candidates: Vec<InterfaceSignalSemanticCandidateRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_arbitration: Option<InterfaceSignalSemanticArbitrationRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolved_semantic_role: Option<InterfaceSignalSemanticRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_grounding_strength: Option<SemanticGroundingStrength>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic_consensus: Option<InterfaceSignalSemanticConsensusRecord>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic_observations: Vec<InterfaceSignalSemanticObservationRecord>,
    pub supporting_statement_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_table_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum InterfaceSignalSemanticRole {
    HandshakeValidLike,
    HandshakeReadyLike,
}

impl InterfaceSignalSemanticRole {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HandshakeValidLike => "handshake_valid_like",
            Self::HandshakeReadyLike => "handshake_ready_like",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SemanticGroundingStrength {
    SingleSource,
    MultiSource,
    CrossModality,
}

impl SemanticGroundingStrength {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SingleSource => "single_source",
            Self::MultiSource => "multi_source",
            Self::CrossModality => "cross_modality",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterfaceSignalSemanticCandidateRecord {
    pub role: InterfaceSignalSemanticRole,
    pub grounding_strength: SemanticGroundingStrength,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_source_kinds: Vec<SignalSemanticHintSourceKind>,
    pub supporting_observation_count: usize,
    pub automation_confidence: AutomationConfidence,
    pub evidence_weight: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub prior_reliability_adjustment: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub arbitration_weight: u32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub alias_dependent: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SemanticArbitrationDecisionBasis {
    SingleCandidate,
    PriorGuidedMargin,
    Contested,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterfaceSignalSemanticArbitrationRecord {
    pub candidate_count: usize,
    pub leading_role: InterfaceSignalSemanticRole,
    pub leading_evidence_weight: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub leading_prior_reliability_adjustment: u32,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub leading_arbitration_weight: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_up_role: Option<InterfaceSignalSemanticRole>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_up_evidence_weight: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_up_prior_reliability_adjustment: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub runner_up_arbitration_weight: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub margin_over_runner_up: Option<u32>,
    pub decisive: bool,
    pub decision_basis: SemanticArbitrationDecisionBasis,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterfaceSignalSemanticConsensusRecord {
    pub role: InterfaceSignalSemanticRole,
    pub grounding_strength: SemanticGroundingStrength,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub supporting_source_kinds: Vec<SignalSemanticHintSourceKind>,
    pub supporting_observation_count: usize,
    pub automation_confidence: AutomationConfidence,
    #[serde(default, skip_serializing_if = "is_zero")]
    pub prior_reliability_adjustment: u32,
    #[serde(default, skip_serializing_if = "is_false")]
    pub prior_guided: bool,
    #[serde(default, skip_serializing_if = "is_false")]
    pub alias_dependent: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InterfaceSignalSemanticObservationRecord {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub semantic_tags: Vec<SignalSemanticTag>,
    pub source_kind: SignalSemanticHintSourceKind,
    pub source_text: String,
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
    #[serde(default)]
    pub supporting_table_ids: Vec<String>,
    #[serde(default)]
    pub supporting_visual_evidence_ids: Vec<String>,
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
pub enum ClockEdge {
    Rising,
    Falling,
    Unknown,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TickPhase {
    PreTick,
    PostTick,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CycleWindowRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_cycles: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_cycles: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case", tag = "kind")]
pub enum TemporalPredicateRecord {
    SignalValue {
        signal_name: String,
        value: String,
        phase: TickPhase,
    },
    ActorDrivesSignal {
        actor_name: String,
        signal_name: String,
        phase: TickPhase,
    },
    ActorMaintainsSignalStable {
        actor_name: String,
        signal_name: String,
        from_phase: TickPhase,
        to_phase: TickPhase,
    },
    SignalStable {
        signal_name: String,
        from_phase: TickPhase,
        to_phase: TickPhase,
    },
    ActorSamplesSignal {
        actor_name: String,
        signal_name: String,
        phase: TickPhase,
    },
    SignalSampled {
        signal_name: String,
        phase: TickPhase,
    },
    HandshakeComplete {
        valid_signal: String,
        ready_signal: String,
        phase: TickPhase,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalRuleRecord {
    pub rule_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_signal: Option<String>,
    pub edge: ClockEdge,
    #[serde(default)]
    pub antecedents: Vec<TemporalPredicateRecord>,
    #[serde(default)]
    pub consequents: Vec<TemporalPredicateRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_window: Option<CycleWindowRecord>,
    pub source_text: String,
    pub supporting_statement_ids: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TemporalConflictRecord {
    pub conflict_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clock_signal: Option<String>,
    pub edge: ClockEdge,
    #[serde(default)]
    pub antecedents: Vec<TemporalPredicateRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cycle_window: Option<CycleWindowRecord>,
    pub signal_name: String,
    pub phase: TickPhase,
    #[serde(default)]
    pub conflicting_values: Vec<String>,
    #[serde(default)]
    pub supporting_rule_ids: Vec<String>,
    #[serde(default)]
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
    pub regular_states: Vec<RegularStateRecord>,
    #[serde(default)]
    pub state_transitions: Vec<StateTransitionRecord>,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub direction_hint: Option<InterfaceSignalDirection>,
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
    actor_signal_relations: Vec<ActorSignalRelation>,
    signal_semantic_hints: Vec<SignalSemanticHintRecord>,
}

#[derive(Debug, Clone)]
struct SemanticPriorGuidance {
    corpus_memory: CorpusMemory,
    protocol_family: ProtocolFamily,
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
            .filter(|anchor| {
                is_boilerplate_section_title(&anchor.title)
                    || is_administrative_section_title(&anchor.title)
            })
            .map(|anchor| anchor.section_id.clone())
            .collect();

        let mut section_statement_ids: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut table_ids_by_statement_id: BTreeMap<String, Vec<String>> = BTreeMap::new();
        for provenance in &evidence_ir.table_signal_declaration_provenance {
            table_ids_by_statement_id
                .entry(provenance.statement_id.clone())
                .or_default()
                .push(provenance.table_id.clone());
        }
        for table_ids in table_ids_by_statement_id.values_mut() {
            table_ids.sort();
            table_ids.dedup();
        }
        let statements = evidence_ir
            .extracted_statements
            .iter()
            .filter_map(|statement| {
                // Section headings are not a sufficient legal-content boundary: PDF front matter
                // is often nested under generic headings such as `Approved`. Exclude only prose
                // carrying a compound legal/administrative signature so rights, licensing, and
                // warranty conditions cannot become gates, phases, invariants, or actors. A single
                // technical word such as permission, rights, version, or Copyright remains usable.
                if is_legal_or_administrative_statement(&statement.text) {
                    return None;
                }

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
                    supporting_table_ids: table_ids_by_statement_id
                        .get(&statement.statement_id)
                        .cloned()
                        .unwrap_or_default(),
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
            actor_signal_relations: evidence_ir.actor_signal_relations.clone(),
            signal_semantic_hints: evidence_ir.signal_semantic_hints.clone(),
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
    supporting_table_ids: Vec<String>,
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
            supporting_table_ids: self.supporting_table_ids.clone(),
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
    actor_name: Option<String>,
    role_summary: String,
    supporting_statement_ids: BTreeSet<String>,
    supporting_section_ids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct ActorPortAccumulator {
    actor_id: String,
    actor_name: String,
    drives: bool,
    reads: bool,
    width_hint: Option<WidthHint>,
    source_statement_ids: BTreeSet<String>,
    automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone)]
struct InfrastructureSourceEvidence {
    actor_id: String,
    actor_name: String,
    supporting_statement_id: String,
    automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone)]
struct InfrastructureDistributionEvidence {
    actor_id: String,
    actor_name: String,
    supporting_statement_id: String,
    automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone)]
struct InfrastructureTopologyEvidence {
    topology_kind: InfrastructureTopologyKind,
    component_name: Option<String>,
    stage_count: Option<u32>,
    target_actor_names: Vec<String>,
    supporting_statement_id: String,
    automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone)]
struct ParsedInfrastructureTopology {
    topology_kind: InfrastructureTopologyKind,
    component_name: Option<String>,
    stage_count: Option<u32>,
    target_actor_names: Vec<String>,
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
    direction_hint_conflicted: bool,
    width_hint: Option<WidthHint>,
    width_hint_conflicted: bool,
    semantic_tags: BTreeSet<SignalSemanticTag>,
    semantic_observations: Vec<InterfaceSignalSemanticObservationRecord>,
    direction_observations: BTreeMap<String, BTreeSet<String>>,
    width_observations: BTreeMap<String, BTreeSet<String>>,
    supporting_statement_ids: BTreeSet<String>,
    supporting_table_ids: BTreeSet<String>,
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
struct TemporalConflictAccumulator {
    clock_signal: Option<String>,
    edge: ClockEdge,
    antecedents: Vec<TemporalPredicateRecord>,
    cycle_window: Option<CycleWindowRecord>,
    signal_name: String,
    phase: TickPhase,
    observed_values: BTreeMap<TemporalConflictComparableValue, TemporalConflictValueSupport>,
    automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone)]
struct TemporalConflictEmission {
    clock_signal: Option<String>,
    edge: ClockEdge,
    antecedents: Vec<TemporalPredicateRecord>,
    cycle_window: Option<CycleWindowRecord>,
    signal_name: String,
    phase: TickPhase,
    conflicting_values: BTreeSet<String>,
    supporting_rule_ids: BTreeSet<String>,
    supporting_statement_ids: BTreeSet<String>,
    automation_confidence: AutomationConfidence,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum TemporalConflictComparableValue {
    Level(String),
    Assertion(String),
    Symbolic(String),
}

impl TemporalConflictComparableValue {
    fn domain(&self) -> TemporalConflictValueDomain {
        match self {
            Self::Level(_) => TemporalConflictValueDomain::Level,
            Self::Assertion(_) => TemporalConflictValueDomain::Assertion,
            Self::Symbolic(_) => TemporalConflictValueDomain::Symbolic,
        }
    }

    fn display_value(&self) -> String {
        match self {
            Self::Level(value) | Self::Assertion(value) | Self::Symbolic(value) => value.clone(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TemporalConflictValueDomain {
    Level,
    Assertion,
    Symbolic,
}

#[derive(Debug, Clone, Default)]
struct TemporalConflictValueSupport {
    supporting_rule_ids: BTreeSet<String>,
    supporting_statement_ids: BTreeSet<String>,
}

#[derive(Debug, Clone)]
struct ParsedInterfaceSignalDeclaration {
    signal_name: String,
    direction_hint: Option<InterfaceSignalDirection>,
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

// Retained only for signal-connectivity extraction (`referenced_signal_names`)
// after ISF-ONLY-IR-PRUNE.2 removed the `decision_tree_fragments` IR surface.
// `block_name`/`guard`/`action` are still computed locally inside
// `parse_explicit_decision_tree_fragment` (for parse-validity `?` guards and
// to derive the referenced signals) but no consumer reads them, so they are
// no longer stored on this struct.
#[derive(Debug, Clone)]
struct ParsedDecisionTreeFragment {
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

fn build_interfaces(
    context: &SemanticContext,
    prior_guidance: Option<&SemanticPriorGuidance>,
    signal_polarities: &[SignalPolarityRecord],
    system_contract: Option<&SystemContractRecord>,
) -> (Vec<InterfaceRecord>, Vec<InterfaceSignalConflictRecord>) {
    let mut accumulators: BTreeMap<String, InterfaceAccumulator> = BTreeMap::new();
    let mut authoritative_signal_names = BTreeSet::new();
    let empty_regular_state_names = BTreeSet::<String>::new();
    let empty_known_signal_names = BTreeSet::<String>::new();
    let empty_known_symbol_names = BTreeSet::<String>::new();
    let signal_polarity_by_signal =
        build_signal_polarity_lookup(signal_polarities, system_contract);

    for statement in &context.statements {
        if let Some(signal_declaration) = parse_explicit_signal_declaration(&statement.text)
            && !interface_signal_declaration_looks_like_width_symbol(&signal_declaration)
        {
            authoritative_signal_names.insert(signal_declaration.signal_name);
        }
    }
    if let Some(system_contract) = system_contract {
        authoritative_signal_names.insert(system_contract.clock_signal.clone());
        authoritative_signal_names.insert(system_contract.reset_signal.clone());
    }

    for statement in &context.statements {
        if let Some(signal_declaration) = parse_explicit_signal_declaration(&statement.text) {
            if interface_signal_declaration_looks_like_width_symbol(&signal_declaration) {
                continue;
            }
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
                signal_declaration.direction_hint,
                signal_declaration.width_hint,
                &statement.statement_id,
                &statement.supporting_table_ids,
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
        let candidate_signals = retain_authoritative_interface_candidate_signals(
            filtered_interface_candidate_signals(statement.signals.as_slice()).as_slice(),
            &authoritative_signal_names,
            &statement.text,
        );
        // Single-signal control prose should enrich an explicit declaration, not mint a duplicate
        // low-confidence interface for the same canonical signal.
        if heuristic_interface_candidate_is_redundant_with_authoritative_surface(
            candidate_signals.as_slice(),
            &authoritative_signal_names,
            &statement.text,
        ) {
            continue;
        }
        if !should_emit_interface_candidate(candidate_signals.as_slice()) {
            continue;
        }

        let key = candidate_signals.join("__");
        let entry = accumulators
            .entry(key)
            .or_insert_with(|| InterfaceAccumulator {
                signals: candidate_signals.iter().cloned().collect(),
                signal_records: BTreeMap::new(),
                supporting_statement_ids: BTreeSet::new(),
            });
        for signal_name in &candidate_signals {
            register_interface_signal_record(
                entry,
                signal_name,
                None,
                None,
                &statement.statement_id,
                &[],
                AutomationConfidence::Low,
            );
        }
    }

    if let Some(system_contract) = system_contract {
        let supporting_statement_ids =
            supporting_statement_ids_for_system_contract(context, system_contract);
        if !supporting_statement_ids.is_empty() {
            let entry = accumulators
                .entry("explicit_document_interface".to_string())
                .or_insert_with(|| InterfaceAccumulator {
                    signals: BTreeSet::new(),
                    signal_records: BTreeMap::new(),
                    supporting_statement_ids: BTreeSet::new(),
                });
            for supporting_statement_id in &supporting_statement_ids {
                register_interface_signal_record(
                    entry,
                    &system_contract.clock_signal,
                    Some(InterfaceSignalDirection::Input),
                    Some(WidthHint::Numeric(1)),
                    supporting_statement_id,
                    &[],
                    system_contract.automation_confidence,
                );
                register_interface_signal_record(
                    entry,
                    &system_contract.reset_signal,
                    Some(InterfaceSignalDirection::Input),
                    Some(WidthHint::Numeric(1)),
                    supporting_statement_id,
                    &[],
                    system_contract.automation_confidence,
                );
            }
        }
    }

    for hint in &context.signal_semantic_hints {
        for accumulator in accumulators.values_mut() {
            if accumulator.signal_records.contains_key(&hint.signal_name) {
                register_interface_signal_semantic_hint(accumulator, &hint.signal_name, hint);
            }
        }
    }

    let mut interface_conflicts = Vec::new();
    let interfaces = accumulators
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
            for (signal_name, signal) in &entry.signal_records {
                if signal.direction_observations.len() > 1 {
                    interface_conflicts.push(InterfaceSignalConflictRecord {
                        conflict_id: format!(
                            "interface_signal_conflict_{:04}",
                            interface_conflicts.len() + 1
                        ),
                        signal_name: signal_name.clone(),
                        conflict_kind: InterfaceSignalConflictKind::DirectionMismatch,
                        observations: build_interface_signal_conflict_observations(
                            &signal.direction_observations,
                        ),
                        automation_confidence: signal.automation_confidence,
                    });
                }
                if signal.width_observations.len() > 1 {
                    interface_conflicts.push(InterfaceSignalConflictRecord {
                        conflict_id: format!(
                            "interface_signal_conflict_{:04}",
                            interface_conflicts.len() + 1
                        ),
                        signal_name: signal_name.clone(),
                        conflict_kind: InterfaceSignalConflictKind::WidthMismatch,
                        observations: build_interface_signal_conflict_observations(
                            &signal.width_observations,
                        ),
                        automation_confidence: signal.automation_confidence,
                    });
                }
            }
            InterfaceRecord {
                interface_id: format!("interface_{}", document_key(&key)),
                signals,
                signal_records: entry
                    .signal_records
                    .into_iter()
                    .map(|(signal_name, signal)| {
                        let semantic_tags: Vec<SignalSemanticTag> =
                            signal.semantic_tags.into_iter().collect();
                        let semantic_observations = signal.semantic_observations;
                        let (
                            semantic_candidates,
                            semantic_arbitration,
                            resolved_semantic_role,
                            semantic_grounding_strength,
                            semantic_consensus,
                        ) = resolve_interface_signal_semantic_role(
                            &semantic_tags,
                            &semantic_observations,
                            prior_guidance,
                        );
                        let resolved_polarity =
                            signal_polarity_by_signal.get(&signal_name).copied();
                        InterfaceSignalRecord {
                            signal_name,
                            direction_hint: signal.direction_hint,
                            width_hint: signal.width_hint,
                            resolved_polarity,
                            semantic_tags,
                            semantic_candidates,
                            semantic_arbitration,
                            resolved_semantic_role,
                            semantic_grounding_strength,
                            semantic_consensus,
                            semantic_observations,
                            supporting_statement_ids: signal
                                .supporting_statement_ids
                                .into_iter()
                                .collect(),
                            supporting_table_ids: signal.supporting_table_ids.into_iter().collect(),
                            automation_confidence: signal.automation_confidence,
                        }
                    })
                    .collect(),
                supporting_statement_ids: entry.supporting_statement_ids.into_iter().collect(),
            }
        })
        .collect();

    (interfaces, interface_conflicts)
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

// ---------------------------------------------------------------------------
// Transaction anchors (KG-ISF-TRANSACTIONS.2a — Cue A)
// ---------------------------------------------------------------------------

/// A transaction the document NAMES in a section heading (Cue A). The IntentIR
/// transaction recognizer mints a typed `TransactionIntent` from each. Recovered
/// by universal English structural grammar — no chip-spec name list (ADR 0006).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransactionAnchorRecord {
    pub transaction_anchor_id: String,
    /// Sanitized, document-derived transaction name (e.g. `write_transfer`).
    pub transaction_name: String,
    /// The raw section-heading title the name was derived from (provenance).
    pub source_title: String,
    pub section_id: String,
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
    /// KG-ISF-TRANSACTIONS.2c: the transaction's grounded signal-set membership —
    /// the declared signals referenced by the statements in its defining section
    /// (the union of `StatementContext.signals` over `supporting_statement_ids`).
    /// This is the document's OWN scoping of which signals belong to the
    /// transaction (bar #3/#4): a signal is a member iff the transaction's section
    /// text references it, so shared signals (e.g. AHB `HREADY` in several
    /// transfers) are correctly attributed to each — universal, no name list
    /// (ADR 0006). Sorted + deduped for determinism; empty when the section
    /// references no declared signal (honest absence, not a miss).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signal_set: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// Universal structural stoplist: function words / cardinals whose presence in a
/// candidate transaction-name phrase means the heading is ABOUT an aspect of a
/// transaction (e.g. "Write transaction dependencies", "ID use for Atomic
/// transactions"), not a transaction-defining noun phrase. Generic English
/// grammar, never spec vocabulary (ADR 0006).
const TXN_NAME_STOPWORDS: &[&str] = &[
    "after", "before", "for", "of", "to", "with", "during", "per", "on", "in", "by", "as", "and",
    "or", "that", "which", "when", "if", "from", "into", "between", "about", "than", "via",
    "without", "one", "two", "three", "four", "five", "six", "seven", "eight",
];

/// Heading furniture words that precede a section label (e.g. "Chapter 10",
/// "Appendix: ...", "Section A11"). Generic document structure, not spec names.
const HEADING_FURNITURE: &[&str] = &["chapter", "appendix", "section", "part", "annex"];

/// Recover the document's NAMED transactions (Cue A) from its section headings.
/// Deterministic (heading order, first-wins dedup by derived name).
fn build_transaction_anchors(
    context: &SemanticContext,
    declared_signals: &std::collections::HashSet<String>,
) -> Vec<TransactionAnchorRecord> {
    // KG-ISF-TRANSACTIONS.2c: the per-statement signal-shaped tokens the
    // evidence/semantic pipeline already extracted — used to ground each
    // transaction's signal-set membership from its own section's statements.
    let signals_by_statement: HashMap<&str, &Vec<String>> = context
        .statements
        .iter()
        .map(|s| (s.statement_id.as_str(), &s.signals))
        .collect();

    let mut seen: BTreeSet<String> = BTreeSet::new();
    let mut records = Vec::new();
    for anchor in &context.section_anchors {
        let Some(name) = derive_transaction_name(&anchor.title) else {
            continue;
        };
        if !seen.insert(name.clone()) {
            continue;
        }
        // KG-ISF-TRANSACTIONS.2k: a transaction defined by a PARENT section (APB
        // `3.1 Write transfers`) genuinely SPANS its subsections (`3.1.1 With no
        // wait states` / `3.1.2 With wait states`), where the PDF files the
        // signal-rich phase prose (`PADDR`/`PWRITE`/`PENABLE`/…). But section anchors
        // are line-range and non-overlapping, so the parent carries only its own
        // intro statements — leaving the membership thin (APB `write_transfer`
        // would otherwise see only `PCLK`). Broaden the transaction's statement
        // scope to also include every section whose dotted number is a strict
        // DESCENDANT of this one. The rule is universal document-structure grammar
        // keyed off the section number — no name list (ADR 0006) — and
        // boundary-precise (bar #3): descendant subtrees are disjoint, so a write
        // transaction (`3.1.x`) and a read transaction (`3.3.x`) never cross. When
        // the section has no subsections the set is byte-identical to its own.
        let mut supporting_statement_ids: Vec<String> = anchor.supporting_statement_ids.clone();
        if let Some(section_number) = leading_section_number(&anchor.title) {
            let mut seen_statement: BTreeSet<String> =
                supporting_statement_ids.iter().cloned().collect();
            for descendant in &context.section_anchors {
                let Some(descendant_number) = leading_section_number(&descendant.title) else {
                    continue;
                };
                if !is_descendant_section_number(&section_number, &descendant_number) {
                    continue;
                }
                for id in &descendant.supporting_statement_ids {
                    if seen_statement.insert(id.clone()) {
                        supporting_statement_ids.push(id.clone());
                    }
                }
            }
        }
        // The transaction's grounded signal set: the DECLARED signals referenced by
        // the statements in its (descendant-expanded) section scope (deduped +
        // sorted). Intersecting with `declared_signals` is essential — the raw
        // statement tokens over-capture enum VALUES (`IDLE`, `INCR4`) and prose
        // abbreviations (`MPMC`, `AHB5`) that are not interface signals; keeping only
        // declared signals makes the set faithful (a signal is a member iff the
        // transaction's section subtree references it AND the document declares it as
        // a signal — bar #3/#4). No name list (ADR 0006) — `declared_signals` is the
        // document's own inventory.
        let signal_set: Vec<String> = supporting_statement_ids
            .iter()
            .filter_map(|id| signals_by_statement.get(id.as_str()))
            .flat_map(|sigs| sigs.iter().cloned())
            .filter(|sig| declared_signals.contains(sig))
            .collect::<BTreeSet<String>>()
            .into_iter()
            .collect();
        records.push(TransactionAnchorRecord {
            transaction_anchor_id: format!("txnanchor_{}", name),
            transaction_name: name,
            source_title: anchor.title.clone(),
            section_id: anchor.section_id.clone(),
            supporting_statement_ids,
            signal_set,
            automation_confidence: AutomationConfidence::Medium,
        });
    }
    records
}

/// Derive a sanitized, document-named transaction identity from a section-heading
/// title, or `None` if the title is not a transaction-defining noun phrase.
///
/// Cue A of KG-ISF-TRANSACTIONS.2a. Universal English structural grammar ONLY —
/// the transaction head nouns live in `normative_vocab` (the single authority)
/// and the discriminators are generic function-word / cardinal / gerund grammar;
/// there is NO chip-spec name list (ADR 0006), so this recognizes transactions in
/// ANY protocol or platform PDF, keyed entirely off the document's own headings.
fn derive_transaction_name(title: &str) -> Option<String> {
    let trimmed = title.trim();
    // Examples / figures / tables ILLUSTRATE a transaction; they do not define one.
    let lead = trimmed.split_whitespace().next().unwrap_or("");
    if matches!(
        lead.to_ascii_lowercase().as_str(),
        "example" | "figure" | "table"
    ) {
        return None;
    }
    let stripped = strip_trailing_parenthetical(strip_heading_prefix(trimmed)).trim();
    if stripped.is_empty() || stripped.contains(',') {
        return None;
    }
    let tokens: Vec<&str> = stripped.split_whitespace().collect();
    if tokens.len() < 2 || tokens.len() > 5 {
        return None;
    }
    // The head noun must be the FINAL token (rejects "transfer restrictions",
    // "transaction dependencies", where the head noun sits mid-phrase).
    let head = crate::ir::normative_vocab::transaction_head_singular(tokens[tokens.len() - 1])?;
    let qualifiers = &tokens[..tokens.len() - 1];
    for q in qualifiers {
        let ql = q.to_ascii_lowercase();
        // function words / cardinals / a mid-phrase head noun / a gerund-led verb
        // ("Transmitting ...", "Deallocating ...") all signal a sub-topic phrasing.
        if TXN_NAME_STOPWORDS.contains(&ql.as_str())
            || crate::ir::normative_vocab::transaction_head_singular(q).is_some()
            || (ql.len() > 5 && ql.ends_with("ing"))
        {
            return None;
        }
    }
    let mut parts: Vec<String> = qualifiers.iter().map(|q| q.to_ascii_lowercase()).collect();
    parts.push(head.to_string());
    let name = sanitize_transaction_name(&parts.join("_"));
    if name.is_empty() { None } else { Some(name) }
}

/// True for a leading section-label token such as `3.1`, `B4.2.1`, `10.4`,
/// `5.5.3.` (trailing dot), `12.`, or `A11` — an optional single leading letter
/// followed by a dotted digit run.
fn is_section_number_token(tok: &str) -> bool {
    let t = tok.trim_end_matches('.');
    let rest = match t.chars().next() {
        Some(c) if c.is_ascii_alphabetic() => &t[c.len_utf8()..],
        Some(_) => t,
        None => return false,
    };
    rest.starts_with(|c: char| c.is_ascii_digit())
        && rest.chars().all(|c| c.is_ascii_digit() || c == '.')
}

/// The normalized leading dotted section number of a heading title (`3.1 Write
/// transfers` → `3.1`, `B4.2.1 Successful write operation` → `B4.2.1`), or `None`
/// when the title carries no leading section-number token (heading furniture like
/// `Chapter 10`, or an unnumbered heading). Trailing dots are trimmed so `5.5.3.`
/// and `5.5.3` compare equal. Universal document-structure grammar — no chip-spec
/// name list (ADR 0006).
fn leading_section_number(title: &str) -> Option<String> {
    let first = title.split_whitespace().next()?;
    if is_section_number_token(first) {
        Some(first.trim_end_matches('.').to_string())
    } else {
        None
    }
}

/// True when `candidate`'s section number is a strict DESCENDANT of `ancestor`'s —
/// a dotted-number prefix test (`3.1.1`/`3.1.2` under `3.1`, `B4.2.1.3` under
/// `B4.2.1`). Equality is not descent, and `3.10` is NOT a descendant of `3.1`
/// (the boundary must be a literal `.`). Universal grammar, no name list (ADR 0006).
fn is_descendant_section_number(ancestor: &str, candidate: &str) -> bool {
    candidate.len() > ancestor.len()
        && candidate.starts_with(ancestor)
        && candidate.as_bytes()[ancestor.len()] == b'.'
}

/// Strip leading section-number tokens and heading furniture (`3.1 `,
/// `Chapter 10 `, `Appendix: `) so only the heading's content phrase remains.
fn strip_heading_prefix(title: &str) -> &str {
    let mut s = title.trim_start();
    loop {
        let mut it = s.splitn(2, char::is_whitespace);
        let first = it.next().unwrap_or("");
        let rest = it.next().unwrap_or("").trim_start();
        if first.is_empty() {
            break;
        }
        if is_section_number_token(first) {
            s = rest;
            continue;
        }
        // A furniture word (optionally colon-suffixed) consumes a following
        // section-number-like label, but never a content word.
        let fl = first.trim_end_matches(':').to_ascii_lowercase();
        if HEADING_FURNITURE.contains(&fl.as_str()) {
            let mut rit = rest.splitn(2, char::is_whitespace);
            let label = rit.next().unwrap_or("");
            if is_section_number_token(label) {
                s = rit.next().unwrap_or("").trim_start();
            } else {
                s = rest;
            }
            continue;
        }
        break;
    }
    s
}

/// Drop a trailing parenthetical clause (`Successful write operation (OK
/// response)` → `Successful write operation`).
fn strip_trailing_parenthetical(s: &str) -> &str {
    let t = s.trim_end();
    if t.ends_with(')')
        && let Some(open) = t.rfind('(')
    {
        return t[..open].trim_end();
    }
    t
}

/// Sanitize a derived transaction name to lowercase `[a-z0-9_]`, collapsing
/// runs of non-alphanumerics to a single underscore.
fn sanitize_transaction_name(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut prev_underscore = false;
    for c in s.chars() {
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_lowercase());
            prev_underscore = false;
        } else if !prev_underscore && !out.is_empty() {
            out.push('_');
            prev_underscore = true;
        }
    }
    out.trim_end_matches('_').to_string()
}

// ---------------------------------------------------------------------------
// Transaction phases (KG-ISF-TRANSACTIONS.2g — the document's own `<qualifier>
// phase` vocabulary)
// ---------------------------------------------------------------------------

/// A protocol transaction PHASE the document NAMES in its prose — the
/// single-word qualifier before a `phase`/`phases` head ("address phase",
/// "data phase", "setup phase", "access phase", "response phase",
/// "turnaround phase"). Recovered by universal English structural grammar, no
/// chip-spec name list (ADR 0006), so it runs on ANY protocol/platform PDF.
///
/// Recognition-only (KG-ISF-TRANSACTIONS.2g): it records WHICH phases the
/// document names and WHERE (provenance), so the later slices can order them
/// (`.2h`) and group each transaction's signal-set membership through them into
/// the ordered multi-phase `(transaction …)` body (`.2i`). It is NOT yet lowered
/// to `.isf`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TransactionPhaseRecord {
    pub transaction_phase_id: String,
    /// The document-named phase, lowercased + sanitized (e.g. `address`, `data`,
    /// `setup`, `access`, `response`, `turnaround`).
    pub phase_name: String,
    /// Every statement whose prose names this phase (provenance) — accumulated
    /// across the document, sorted + deduped for determinism.
    #[serde(default)]
    pub supporting_statement_ids: Vec<String>,
    /// KG-ISF-TRANSACTIONS.2i: the phase's grounded signal set — the declared
    /// signals referenced by the statements that name this phase (the union of
    /// `StatementContext.signals` over `supporting_statement_ids`, intersected with
    /// the declared-signal inventory). This is the SAME intersection technique as the
    /// transaction-anchor `signal_set` (`.2c`): the raw statement tokens over-capture
    /// enum VALUES / prose abbreviations that are not signals, so keeping only declared
    /// signals makes the set faithful. It lets the IntentIR group each transaction's
    /// signal-set membership BY phase (a member belongs to phase P iff P's prose
    /// references it). Universal English grammar over the document's own prose, no name
    /// list (ADR 0006); sorted + deduped; empty when the phase's prose references no
    /// declared signal (honest absence — e.g. AXI/SWD, where the `<qualifier> phase`
    /// prose names no declared signal).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signal_set: Vec<String>,
    pub automation_confidence: AutomationConfidence,
}

/// Qualifier stoplist for the prose `<qualifier> phase` recogniser. Prose is
/// noisier than the section headings the transaction-anchor recogniser reads
/// (`derive_transaction_name`), so beyond the shared function-word / cardinal
/// grammar this also rejects determiners, demonstratives, ordinals, and the
/// position/quantity adjectives that *modify* a phase rather than *name* one
/// ("the/this/each/next/first/separate phase"). Universal English grammar,
/// never chip-spec vocabulary (ADR 0006) — the same kind of curated
/// function-word list `TXN_NAME_STOPWORDS` already is. Tuned against a
/// corpus-wide before/after measurement (`docs/research/transaction-capture-census.md`
/// §4.5): on the wire docs it keeps exactly the protocol phases
/// (APB setup/access, AHB/AXI address/data, SWD address/data/response/turnaround/
/// acknowledge) and rejects the determiner/cardinal/ordinal/head-noun/boundary
/// noise.
#[rustfmt::skip]
const PHASE_NAME_STOPWORDS: &[&str] = &[
    // determiners / demonstratives / quantifiers
    "the", "a", "an", "this", "that", "these", "those", "each", "its", "their", "his", "her",
    "our", "your", "my", "any", "some", "no", "none", "every", "all", "both", "either", "neither",
    "another", "such", "same",
    // prepositions / conjunctions
    "after", "before", "for", "of", "to", "with", "during", "per", "on", "in", "by", "as", "and",
    "or", "which", "when", "if", "from", "into", "between", "about", "than", "via", "without",
    "while", "where", "then", "so", "but", "nor", "yet",
    // cardinals
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    "eleven", "twelve",
    // ordinals
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    // position / quantity adjectives that modify (not name) a phase
    "next", "previous", "last", "current", "final", "initial", "prior", "preceding", "single",
    "multiple", "separate", "other", "given", "various", "several", "additional", "further",
    "main", "only", "new", "certain", "following",
];

/// Trim a raw token's leading/trailing non-alphanumeric punctuation and
/// lowercase it (`"phase,"` → `phase`, `"(address)"` → `address`).
fn normalize_phase_token(raw: &str) -> String {
    raw.trim_matches(|c: char| !c.is_ascii_alphanumeric())
        .to_ascii_lowercase()
}

/// True when a raw token is the `phase` / `phases` head noun (after trimming
/// edge punctuation). Exact match — never a substring, so `phaseshift` /
/// `multiphase` do not trip it.
fn is_phase_head_token(raw: &str) -> bool {
    matches!(normalize_phase_token(raw).as_str(), "phase" | "phases")
}

/// Derive a document-named phase from the raw token immediately before a
/// `phase`/`phases` head, or `None` if it is not a phase-naming qualifier.
///
/// Cue of KG-ISF-TRANSACTIONS.2g. Universal English structural grammar ONLY —
/// no chip-spec name list (ADR 0006), so it recognises the phases of ANY
/// protocol/platform PDF keyed entirely off the document's own prose.
fn derive_phase_name(raw_prev: &str) -> Option<String> {
    // A phase qualifier and its `phase` head are one noun phrase: if the
    // previous token ends a sentence/clause, the `phase` token actually starts a
    // NEW clause, so this is not a `<qualifier> phase` name (e.g. "… data on the
    // lanes. Phase 2 …" must not yield a `lanes` phase).
    if raw_prev.trim_end().ends_with(['.', ':', ';', '!', '?']) {
        return None;
    }
    let q = normalize_phase_token(raw_prev);
    // All-alphabetic and ≥3 chars: drops symbols (`|`, `-`), hyphenated/numeric
    // tokens (`link-up`, `3-1`, `pre-boot`), and 1–2-letter noise (`aw`, `rx`).
    if q.len() < 3 || !q.chars().all(|c| c.is_ascii_alphabetic()) {
        return None;
    }
    // function-word / determiner / cardinal / ordinal / position-adjective noise,
    // a transaction head noun used as a modifier ("data TRANSFER phase" → drop
    // `transfer`), or a gerund-led verb ("processing phase") all describe rather
    // than name a phase.
    if PHASE_NAME_STOPWORDS.contains(&q.as_str())
        || crate::ir::normative_vocab::transaction_head_singular(&q).is_some()
        || (q.len() > 5 && q.ends_with("ing"))
    {
        return None;
    }
    let name = sanitize_transaction_name(&q);
    if name.is_empty() { None } else { Some(name) }
}

/// Recover the protocol PHASES the document NAMES in its prose (the document's
/// own `<qualifier> phase` vocabulary). Deterministic: first statement-order
/// occurrence sets the phase order; every naming statement is recorded as
/// provenance (sorted + deduped). KG-ISF-TRANSACTIONS.2g — recognition only,
/// no body composition. Universal English grammar, no chip-spec name list
/// (ADR 0006).
fn build_transaction_phases(
    context: &SemanticContext,
    declared_signals: &std::collections::HashSet<String>,
) -> Vec<TransactionPhaseRecord> {
    // The per-statement signal-shaped tokens the pipeline already extracted — the
    // same source the transaction-anchor membership uses (`.2c`), reused here to
    // ground each phase's signal set.
    let signals_by_statement: HashMap<&str, &Vec<String>> = context
        .statements
        .iter()
        .map(|s| (s.statement_id.as_str(), &s.signals))
        .collect();
    let mut order: Vec<String> = Vec::new();
    let mut provenance: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for statement in &context.statements {
        let tokens: Vec<&str> = statement.text.split_whitespace().collect();
        for (i, raw) in tokens.iter().enumerate() {
            if i == 0 || !is_phase_head_token(raw) {
                continue;
            }
            let Some(name) = derive_phase_name(tokens[i - 1]) else {
                continue;
            };
            if !provenance.contains_key(&name) {
                order.push(name.clone());
            }
            provenance
                .entry(name)
                .or_default()
                .insert(statement.statement_id.clone());
        }
    }
    order
        .into_iter()
        .map(|name| {
            let supporting_statement_ids: Vec<String> = provenance
                .get(&name)
                .map(|ids| ids.iter().cloned().collect())
                .unwrap_or_default();
            // KG-ISF-TRANSACTIONS.2i: the phase's grounded signal set — the DECLARED
            // signals referenced by the statements that name this phase, deduped +
            // sorted. Intersecting with `declared_signals` is essential for the same
            // reason as the anchor signal_set (`.2c`): the raw statement tokens
            // over-capture enum values / abbreviations that are not signals. No name
            // list (ADR 0006) — `declared_signals` is the document's own inventory.
            let signal_set: Vec<String> = supporting_statement_ids
                .iter()
                .filter_map(|id| signals_by_statement.get(id.as_str()))
                .flat_map(|sigs| sigs.iter().cloned())
                .filter(|sig| declared_signals.contains(sig))
                .collect::<BTreeSet<String>>()
                .into_iter()
                .collect();
            TransactionPhaseRecord {
                transaction_phase_id: format!("txnphase_{name}"),
                phase_name: name,
                supporting_statement_ids,
                signal_set,
                automation_confidence: AutomationConfidence::Medium,
            }
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
        actor_signal_relations: Vec::new(),
        signal_semantic_hints: Vec::new(),
    };
    let system_contract = build_system_contract(&scoped_context);
    let (interfaces, _interface_signal_conflicts) =
        build_interfaces(&scoped_context, None, &[], system_contract.as_ref());
    let regular_states = build_regular_states(&scoped_context);
    let state_transitions = build_state_transitions(&scoped_context);
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
        regular_states,
        state_transitions,
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

    for relation in &context.actor_signal_relations {
        let actor_id = actor_id_for_name(&relation.actor_name);
        actor_id_by_term.insert(relation.actor_name.to_ascii_lowercase(), actor_id.clone());
        let entry = accumulators
            .entry(actor_id)
            .or_insert_with(|| ActorAccumulator {
                actor_name: Some(relation.actor_name.clone()),
                role_summary: format!(
                    "semantic role inferred from actor-signal relation evidence around `{}`",
                    relation.actor_name
                ),
                supporting_statement_ids: BTreeSet::new(),
                supporting_section_ids: BTreeSet::new(),
            });
        if entry.actor_name.is_none() {
            entry.actor_name = Some(relation.actor_name.clone());
        }
        entry
            .supporting_statement_ids
            .extend(relation.source_statement_ids.iter().cloned());
        entry
            .supporting_section_ids
            .extend(statement_ids_to_section_ids(
                context,
                relation.source_statement_ids.as_slice(),
            ));
    }

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
                    actor_name: Some((*term).to_string()),
                    role_summary: format!("semantic role inferred around `{term}` evidence"),
                    supporting_statement_ids: BTreeSet::new(),
                    supporting_section_ids: BTreeSet::new(),
                });
            if entry.actor_name.is_none() {
                entry.actor_name = Some((*term).to_string());
            }
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
                    actor_name: None,
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
            actor_name: entry.actor_name,
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

/// Evaluate the typed fidelity gates over every `ActorContract` and
/// mechanically enforce the residual-honesty doctrine: any `Fail` on a
/// `Lowerable` contract is rerouted to `Residual{reason}` BEFORE the
/// `.isf` adapter consumes it. The reason carries the gate name and
/// its message so downstream consumers can attribute the routing.
/// Returns the full per-gate findings the gates produced (a `Fail`
/// finding paired with a now-Residual contract is the doctrine
/// working, not a bug).
///
/// Boundary-direction context comes from `actor_ports`: per-actor
/// `(declared, inputs, outputs)` sets, with an actor-agnostic fallback
/// of the global declared union when a contract's `actor_name` is
/// `None` or unknown (direction-bearing gates run `NotEvaluated` in
/// that case, honestly).
fn apply_fidelity_gates(
    contracts: &mut [crate::ir::contract::ActorContract],
    actor_ports: &[ActorPortRecord],
) -> Vec<crate::ir::fidelity::FidelityFinding> {
    use crate::ir::contract::LoweringDisposition;
    use crate::ir::fidelity::{
        FindingStatus, evaluate_figure_conformance, evaluate_no_strict_invalid,
        evaluate_realizable_boundary, evaluate_realizable_direction, evaluate_realizable_handshake,
        evaluate_residual_honesty,
    };

    // (declared, inputs, outputs) per actor.
    type ActorSigSets = (BTreeSet<String>, BTreeSet<String>, BTreeSet<String>);
    let mut per_actor: BTreeMap<String, ActorSigSets> = BTreeMap::new();
    let mut global: BTreeSet<String> = BTreeSet::new();
    for p in actor_ports {
        let entry = per_actor.entry(p.actor_name.clone()).or_default();
        entry.0.insert(p.signal_name.clone());
        global.insert(p.signal_name.clone());
        match p.direction {
            ActorRelativeDirection::Input => {
                entry.1.insert(p.signal_name.clone());
            }
            ActorRelativeDirection::Output => {
                entry.2.insert(p.signal_name.clone());
            }
            ActorRelativeDirection::InOut => {
                entry.1.insert(p.signal_name.clone());
                entry.2.insert(p.signal_name.clone());
            }
            ActorRelativeDirection::Unknown => {}
        }
    }
    let empty_actor: ActorSigSets = (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());

    let mut all_findings = Vec::with_capacity(contracts.len() * 6);
    for c in contracts.iter_mut() {
        let resolved = c.actor_name.as_deref().and_then(|n| per_actor.get(n));
        let (declared, inputs, outputs) = match resolved {
            Some(sigs) => (&sigs.0, &sigs.1, &sigs.2),
            None => (&global, &empty_actor.1, &empty_actor.2),
        };

        let here = [
            evaluate_realizable_boundary(c, declared),
            evaluate_realizable_direction(c, inputs, outputs),
            evaluate_realizable_handshake(c, inputs, outputs),
            evaluate_residual_honesty(c),
            evaluate_no_strict_invalid(c),
            evaluate_figure_conformance(c, None),
        ];

        // Honesty doctrine: a Lowerable contract that fails any gate
        // is rerouted to Residual with the gate finding as the reason.
        if matches!(c.lowering, LoweringDisposition::Lowerable)
            && let Some(failing) = here.iter().find(|f| f.status == FindingStatus::Fail)
        {
            c.lowering = LoweringDisposition::Residual {
                reason: format!("fidelity:{:?}: {}", failing.gate, failing.message),
            };
        }

        all_findings.extend(here);
    }
    all_findings
}

fn build_actor_ports(
    context: &SemanticContext,
    interfaces: &[InterfaceRecord],
    system_contract: Option<&SystemContractRecord>,
) -> Vec<ActorPortRecord> {
    let mut accumulators: BTreeMap<(String, String), ActorPortAccumulator> = BTreeMap::new();
    let signal_widths = signal_width_hints_by_name(interfaces);

    for relation in &context.actor_signal_relations {
        let key = (relation.actor_name.clone(), relation.signal_name.clone());
        let entry = accumulators
            .entry(key)
            .or_insert_with(|| ActorPortAccumulator {
                actor_id: actor_id_for_name(&relation.actor_name),
                actor_name: relation.actor_name.clone(),
                drives: false,
                reads: false,
                width_hint: signal_widths.get(&relation.signal_name).cloned().flatten(),
                source_statement_ids: BTreeSet::new(),
                automation_confidence: relation.automation_confidence,
            });
        match relation.relation {
            RelationKind::Drives => entry.drives = true,
            RelationKind::Reads => entry.reads = true,
        }
        if entry.width_hint.is_none() {
            entry.width_hint = signal_widths.get(&relation.signal_name).cloned().flatten();
        }
        entry
            .source_statement_ids
            .extend(relation.source_statement_ids.iter().cloned());
        entry.automation_confidence =
            max_automation_confidence(entry.automation_confidence, relation.automation_confidence);
    }

    if let Some(system_contract) = system_contract {
        let actor_names_with_protocol_relations = context
            .actor_signal_relations
            .iter()
            .filter(|relation| {
                relation.signal_name != system_contract.clock_signal
                    && relation.signal_name != system_contract.reset_signal
            })
            .map(|relation| relation.actor_name.clone())
            .collect::<BTreeSet<_>>();
        let supporting_statement_ids =
            supporting_statement_ids_for_system_contract(context, system_contract);

        for actor_name in actor_names_with_protocol_relations {
            for signal_name in [&system_contract.clock_signal, &system_contract.reset_signal] {
                let key = (actor_name.clone(), signal_name.clone());
                let entry = accumulators
                    .entry(key)
                    .or_insert_with(|| ActorPortAccumulator {
                        actor_id: actor_id_for_name(&actor_name),
                        actor_name: actor_name.clone(),
                        drives: false,
                        reads: false,
                        width_hint: signal_widths.get(signal_name).cloned().flatten(),
                        source_statement_ids: BTreeSet::new(),
                        automation_confidence: system_contract.automation_confidence,
                    });
                entry.reads = true;
                if entry.width_hint.is_none() {
                    entry.width_hint = signal_widths.get(signal_name).cloned().flatten();
                }
                entry
                    .source_statement_ids
                    .extend(supporting_statement_ids.iter().cloned());
                entry.automation_confidence = max_automation_confidence(
                    entry.automation_confidence,
                    system_contract.automation_confidence,
                );
            }
        }
    }

    accumulators
        .into_iter()
        .map(|((_actor_name, signal_name), entry)| {
            let (direction, relation_basis) =
                actor_relative_direction_and_basis(entry.drives, entry.reads);
            ActorPortRecord {
                actor_id: entry.actor_id,
                actor_name: entry.actor_name,
                signal_name,
                direction,
                relation_basis,
                width_hint: entry.width_hint,
                source_statement_ids: entry.source_statement_ids.into_iter().collect(),
                automation_confidence: entry.automation_confidence,
            }
        })
        .collect()
}

fn supporting_statement_ids_for_system_contract(
    context: &SemanticContext,
    system_contract: &SystemContractRecord,
) -> BTreeSet<String> {
    let mut statement_ids = BTreeSet::new();

    for statement in &context.statements {
        if parse_explicit_system_clock(&statement.text)
            .as_deref()
            .is_some_and(|signal_name| signal_name == system_contract.clock_signal)
        {
            statement_ids.insert(statement.statement_id.clone());
        }
        if parse_explicit_system_reset(&statement.text)
            .as_ref()
            .is_some_and(|reset| reset.signal_name == system_contract.reset_signal)
        {
            statement_ids.insert(statement.statement_id.clone());
        }
    }

    statement_ids
}

fn build_signal_connectivity(
    actor_ports: &[ActorPortRecord],
    system_contract: Option<&SystemContractRecord>,
) -> Vec<SignalConnectivityRecord> {
    let mut accumulators: BTreeMap<String, SignalConnectivityRecord> = BTreeMap::new();

    for port in actor_ports {
        let connectivity_class = classify_signal_connectivity(&port.signal_name, system_contract);
        let entry = accumulators
            .entry(port.signal_name.clone())
            .or_insert_with(|| SignalConnectivityRecord {
                signal_name: port.signal_name.clone(),
                connectivity_class,
                producer_actor_ids: Vec::new(),
                producer_actor_names: Vec::new(),
                consumer_actor_ids: Vec::new(),
                consumer_actor_names: Vec::new(),
                width_hint: port.width_hint.clone(),
                source_statement_ids: Vec::new(),
                automation_confidence: port.automation_confidence,
            });
        entry.connectivity_class = connectivity_class;

        merge_signal_hint(&mut entry.width_hint, port.width_hint.clone());
        entry.automation_confidence =
            min_automation_confidence(entry.automation_confidence, port.automation_confidence);

        for statement_id in &port.source_statement_ids {
            if !entry.source_statement_ids.contains(statement_id) {
                entry.source_statement_ids.push(statement_id.clone());
            }
        }

        if matches!(
            port.direction,
            ActorRelativeDirection::Output | ActorRelativeDirection::InOut
        ) {
            if !entry.producer_actor_ids.contains(&port.actor_id) {
                entry.producer_actor_ids.push(port.actor_id.clone());
            }
            if !entry.producer_actor_names.contains(&port.actor_name) {
                entry.producer_actor_names.push(port.actor_name.clone());
            }
        }

        if matches!(
            port.direction,
            ActorRelativeDirection::Input | ActorRelativeDirection::InOut
        ) {
            if !entry.consumer_actor_ids.contains(&port.actor_id) {
                entry.consumer_actor_ids.push(port.actor_id.clone());
            }
            if !entry.consumer_actor_names.contains(&port.actor_name) {
                entry.consumer_actor_names.push(port.actor_name.clone());
            }
        }
    }

    accumulators.into_values().collect()
}

fn build_infrastructure_signals(
    context: &SemanticContext,
    signal_connectivity: &[SignalConnectivityRecord],
    system_contract: Option<&SystemContractRecord>,
) -> Vec<InfrastructureSignalRecord> {
    let Some(system_contract) = system_contract else {
        return Vec::new();
    };

    [
        (
            system_contract.clock_signal.as_str(),
            InfrastructureSignalKind::SystemClock,
        ),
        (
            system_contract.reset_signal.as_str(),
            InfrastructureSignalKind::SystemReset,
        ),
    ]
    .into_iter()
    .map(|(signal_name, kind)| {
        let connectivity = signal_connectivity
            .iter()
            .find(|record| record.signal_name == signal_name);
        let recovered_source_actor_ids = connectivity
            .map(|record| record.producer_actor_ids.clone())
            .unwrap_or_default();
        let recovered_source_actor_names = connectivity
            .map(|record| record.producer_actor_names.clone())
            .unwrap_or_default();
        let distributed_to_actor_ids = connectivity
            .map(|record| record.consumer_actor_ids.clone())
            .unwrap_or_default();
        let distributed_to_actor_names = connectivity
            .map(|record| record.consumer_actor_names.clone())
            .unwrap_or_default();
        let supporting_statement_ids = connectivity
            .map(|record| record.source_statement_ids.clone())
            .filter(|ids| !ids.is_empty())
            .unwrap_or_else(|| system_contract.supporting_statement_ids.clone());
        let automation_confidence = connectivity
            .map(|record| record.automation_confidence)
            .unwrap_or(system_contract.automation_confidence);
        let explicit_sources = explicit_infrastructure_source_evidence(context, signal_name);
        let explicit_distribution =
            explicit_infrastructure_distribution_evidence(context, signal_name);
        let explicit_topology =
            explicit_infrastructure_topology_evidence(context, signal_name, kind);

        let mut recovered_source_actor_ids = recovered_source_actor_ids;
        let mut recovered_source_actor_names = recovered_source_actor_names;
        let mut distributed_to_actor_ids = distributed_to_actor_ids;
        let mut distributed_to_actor_names = distributed_to_actor_names;
        let mut supporting_statement_ids = supporting_statement_ids;
        let mut automation_confidence = automation_confidence;
        let mut infrastructure_topology = Vec::new();
        for source in explicit_sources {
            if !recovered_source_actor_ids.contains(&source.actor_id) {
                recovered_source_actor_ids.push(source.actor_id);
            }
            if !recovered_source_actor_names.contains(&source.actor_name) {
                recovered_source_actor_names.push(source.actor_name);
            }
            if !supporting_statement_ids.contains(&source.supporting_statement_id) {
                supporting_statement_ids.push(source.supporting_statement_id);
            }
            automation_confidence =
                min_automation_confidence(automation_confidence, source.automation_confidence);
        }
        for distribution in explicit_distribution {
            if !distributed_to_actor_ids.contains(&distribution.actor_id) {
                distributed_to_actor_ids.push(distribution.actor_id);
            }
            if !distributed_to_actor_names.contains(&distribution.actor_name) {
                distributed_to_actor_names.push(distribution.actor_name);
            }
            if !supporting_statement_ids.contains(&distribution.supporting_statement_id) {
                supporting_statement_ids.push(distribution.supporting_statement_id);
            }
            automation_confidence = min_automation_confidence(
                automation_confidence,
                distribution.automation_confidence,
            );
        }
        for topology in explicit_topology {
            let target_actor_ids = topology
                .target_actor_names
                .iter()
                .map(|actor_name| actor_id_for_name(actor_name))
                .collect::<Vec<_>>();
            if !supporting_statement_ids.contains(&topology.supporting_statement_id) {
                supporting_statement_ids.push(topology.supporting_statement_id.clone());
            }
            automation_confidence =
                min_automation_confidence(automation_confidence, topology.automation_confidence);
            infrastructure_topology.push(InfrastructureTopologyRecord {
                topology_id: infrastructure_topology_id(
                    signal_name,
                    topology.topology_kind,
                    &topology.supporting_statement_id,
                    infrastructure_topology.len(),
                ),
                topology_kind: topology.topology_kind,
                component_name: topology.component_name,
                stage_count: topology.stage_count,
                target_actor_ids,
                target_actor_names: topology.target_actor_names,
                supporting_statement_id: topology.supporting_statement_id,
                automation_confidence: topology.automation_confidence,
            });
        }

        InfrastructureSignalRecord {
            signal_name: signal_name.to_string(),
            kind,
            source_status: infrastructure_source_status(recovered_source_actor_ids.len()),
            recovered_source_actor_ids,
            recovered_source_actor_names,
            distribution_status: infrastructure_distribution_status(distributed_to_actor_ids.len()),
            distributed_to_actor_ids,
            distributed_to_actor_names,
            infrastructure_topology,
            supporting_statement_ids,
            automation_confidence,
        }
    })
    .collect()
}

fn explicit_infrastructure_source_evidence(
    context: &SemanticContext,
    signal_name: &str,
) -> Vec<InfrastructureSourceEvidence> {
    let mut sources = Vec::new();
    let mut seen = BTreeSet::<String>::new();

    for statement in &context.statements {
        let Some(actor_name) =
            parse_explicit_infrastructure_source_actor(&statement.text, signal_name)
        else {
            continue;
        };
        let actor_id = actor_id_for_name(&actor_name);
        if !seen.insert(format!("{}:{}", actor_id, statement.statement_id.as_str())) {
            continue;
        }
        sources.push(InfrastructureSourceEvidence {
            actor_id,
            actor_name,
            supporting_statement_id: statement.statement_id.clone(),
            automation_confidence: AutomationConfidence::Medium,
        });
    }

    sources
}

fn parse_explicit_infrastructure_source_actor(text: &str, signal_name: &str) -> Option<String> {
    let lowered = text.to_ascii_lowercase();
    let signal_lower = signal_name.to_ascii_lowercase();

    const ACTIVE_SOURCE_VERBS: &[&str] = &[
        "drives",
        "generates",
        "provides",
        "sources",
        "supplies",
        "outputs",
        "produces",
        "feeds",
        "drives the",
        "generates the",
        "provides the",
        "sources the",
        "supplies the",
        "outputs the",
        "produces the",
        "feeds the",
    ];
    const PASSIVE_SOURCE_VERBS: &[&str] = &[
        "driven",
        "generated",
        "provided",
        "sourced",
        "supplied",
        "output",
        "produced",
        "fed",
    ];

    for verb in ACTIVE_SOURCE_VERBS {
        let active_pat = format!(" {verb} {signal_lower}");
        if let Some(verb_pos) = lowered.find(&active_pat)
            && let Some(actor) = extract_infrastructure_subject_phrase(&text[..verb_pos])
        {
            return Some(actor);
        }
    }

    for verb in PASSIVE_SOURCE_VERBS {
        for prep in ["by", "from"] {
            let passive_pat = format!("{signal_lower} is {verb} {prep} ");
            if let Some(pattern_pos) = lowered.find(&passive_pat) {
                let actor_start = pattern_pos + passive_pat.len();
                if actor_start <= text.len()
                    && let Some(actor) =
                        extract_infrastructure_component_phrase(&text[actor_start..])
                {
                    return Some(actor);
                }
            }
        }
    }

    None
}

fn explicit_infrastructure_distribution_evidence(
    context: &SemanticContext,
    signal_name: &str,
) -> Vec<InfrastructureDistributionEvidence> {
    let mut distribution = Vec::new();
    let mut seen = BTreeSet::<String>::new();

    for statement in &context.statements {
        for actor_name in
            parse_explicit_infrastructure_distribution_actors(&statement.text, signal_name)
        {
            let actor_id = actor_id_for_name(&actor_name);
            if !seen.insert(format!("{}:{}", actor_id, statement.statement_id.as_str())) {
                continue;
            }
            distribution.push(InfrastructureDistributionEvidence {
                actor_id,
                actor_name,
                supporting_statement_id: statement.statement_id.clone(),
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    distribution
}

fn explicit_infrastructure_topology_evidence(
    context: &SemanticContext,
    signal_name: &str,
    kind: InfrastructureSignalKind,
) -> Vec<InfrastructureTopologyEvidence> {
    let mut topology = Vec::new();
    let mut seen = BTreeSet::<String>::new();

    for statement in &context.statements {
        for parsed in parse_explicit_infrastructure_topology(&statement.text, signal_name, kind) {
            let seen_key = format!(
                "{}:{:?}:{:?}:{}",
                parsed.topology_kind.as_str(),
                parsed.component_name,
                parsed.stage_count,
                parsed.target_actor_names.join(",")
            );
            if !seen.insert(seen_key) {
                continue;
            }
            topology.push(InfrastructureTopologyEvidence {
                topology_kind: parsed.topology_kind,
                component_name: parsed.component_name,
                stage_count: parsed.stage_count,
                target_actor_names: parsed.target_actor_names,
                supporting_statement_id: statement.statement_id.clone(),
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }

    topology
}

fn parse_explicit_infrastructure_topology(
    text: &str,
    signal_name: &str,
    kind: InfrastructureSignalKind,
) -> Vec<ParsedInfrastructureTopology> {
    let mut topology = Vec::new();
    match kind {
        InfrastructureSignalKind::SystemClock => {
            if let Some(parsed) = parse_explicit_clock_gated_branch(text, signal_name) {
                topology.push(parsed);
            }
        }
        InfrastructureSignalKind::SystemReset => {
            if let Some(parsed) = parse_explicit_reset_synchronizer_stages(text, signal_name) {
                topology.push(parsed);
            }
            if let Some(parsed) = parse_explicit_reset_tree_targets(text, signal_name) {
                topology.push(parsed);
            }
        }
    }

    topology
}

fn parse_explicit_infrastructure_distribution_actors(text: &str, signal_name: &str) -> Vec<String> {
    let lowered = text.to_ascii_lowercase();
    let signal_lower = signal_name.to_ascii_lowercase();
    let mut actors = Vec::new();

    const ACTIVE_DISTRIBUTION_VERBS: &[&str] = &[
        "feeds",
        "routes",
        "distributes",
        "delivers",
        "provides",
        "drives",
        "sends",
        "propagates",
    ];
    const PASSIVE_DISTRIBUTION_VERBS: &[&str] = &[
        "distributed",
        "routed",
        "delivered",
        "fed",
        "sent",
        "propagated",
        "provided",
        "driven",
    ];
    const SIGNAL_SUBJECT_VERBS: &[&str] =
        &["feeds", "drives", "clocks", "resets", "reaches", "serves"];

    for verb in ACTIVE_DISTRIBUTION_VERBS {
        for object in [
            format!(" {verb} {signal_lower} to "),
            format!(" {verb} the {signal_lower} to "),
        ] {
            if let Some(pattern_pos) = lowered.find(&object) {
                let target_start = pattern_pos + object.len();
                if target_start <= text.len() {
                    append_unique_infrastructure_actors(
                        &mut actors,
                        extract_infrastructure_target_phrases(&text[target_start..]),
                    );
                }
            }
        }
    }

    for verb in PASSIVE_DISTRIBUTION_VERBS {
        let passive_pat = format!("{signal_lower} is {verb} to ");
        if let Some(pattern_pos) = lowered.find(&passive_pat) {
            let target_start = pattern_pos + passive_pat.len();
            if target_start <= text.len() {
                append_unique_infrastructure_actors(
                    &mut actors,
                    extract_infrastructure_target_phrases(&text[target_start..]),
                );
            }
        }
    }

    for verb in SIGNAL_SUBJECT_VERBS {
        let active_pat = format!("{signal_lower} {verb} ");
        if let Some(pattern_pos) = lowered.find(&active_pat) {
            let target_start = pattern_pos + active_pat.len();
            if target_start <= text.len() {
                append_unique_infrastructure_actors(
                    &mut actors,
                    extract_infrastructure_target_phrases(&text[target_start..]),
                );
            }
        }
    }

    actors
}

fn parse_explicit_clock_gated_branch(
    text: &str,
    signal_name: &str,
) -> Option<ParsedInfrastructureTopology> {
    let lowered = text.to_ascii_lowercase();
    let signal_lower = signal_name.to_ascii_lowercase();
    if !lowered.contains(&signal_lower)
        || !(lowered.contains("clock gate")
            || lowered.contains("clock-gated")
            || lowered.contains("clock gated")
            || lowered.contains("gated branch")
            || lowered.contains("gated by"))
    {
        return None;
    }

    let component_name = extract_infrastructure_component_after_markers(
        text,
        &[
            "clock gate ",
            "clock-gated by ",
            "clock gated by ",
            "gated by ",
        ],
    );
    let target_actor_names = extract_infrastructure_targets_after_markers(
        text,
        &[
            " before reaching ",
            " reaching ",
            " feeds ",
            " drives ",
            " clocks ",
            " routes to ",
            " distributed to ",
            " to ",
            " for ",
        ],
    );

    if target_actor_names.is_empty() {
        return None;
    }

    Some(ParsedInfrastructureTopology {
        topology_kind: InfrastructureTopologyKind::ClockGatedBranch,
        component_name,
        stage_count: None,
        target_actor_names,
    })
}

fn parse_explicit_reset_synchronizer_stages(
    text: &str,
    signal_name: &str,
) -> Option<ParsedInfrastructureTopology> {
    let lowered = text.to_ascii_lowercase();
    let signal_lower = signal_name.to_ascii_lowercase();
    if !lowered.contains(&signal_lower)
        || !(lowered.contains("synchronizer") || lowered.contains("synchroniser"))
    {
        return None;
    }

    let stage_count = parse_infrastructure_stage_count(&lowered)?;
    let component_name = extract_infrastructure_component_after_markers(
        text,
        &[
            "reset synchronizer ",
            "reset synchroniser ",
            "synchronizer ",
            "synchroniser ",
        ],
    )
    .or_else(|| Some("reset synchronizer".to_string()));
    let mut target_actor_names =
        parse_explicit_infrastructure_distribution_actors(text, signal_name);
    dedup_actor_names(&mut target_actor_names);

    Some(ParsedInfrastructureTopology {
        topology_kind: InfrastructureTopologyKind::ResetSynchronizerStages,
        component_name,
        stage_count: Some(stage_count),
        target_actor_names,
    })
}

fn parse_explicit_reset_tree_targets(
    text: &str,
    signal_name: &str,
) -> Option<ParsedInfrastructureTopology> {
    let lowered = text.to_ascii_lowercase();
    let signal_lower = signal_name.to_ascii_lowercase();
    if !lowered.contains(&signal_lower)
        || !(lowered.contains("reset tree") || lowered.contains("reset-tree"))
    {
        return None;
    }

    let target_actor_names = extract_infrastructure_targets_after_markers(
        text,
        &[
            " targets ",
            " target ",
            " fans out to ",
            " fanout to ",
            " fan-out to ",
            " routes to ",
            " distributes to ",
            " is distributed to ",
            " reaches ",
            " resets ",
            " serves ",
            " to ",
        ],
    );
    if target_actor_names.is_empty() {
        return None;
    }

    Some(ParsedInfrastructureTopology {
        topology_kind: InfrastructureTopologyKind::ResetTreeTargets,
        component_name: Some("reset tree".to_string()),
        stage_count: None,
        target_actor_names,
    })
}

fn extract_infrastructure_component_after_markers(text: &str, markers: &[&str]) -> Option<String> {
    let lowered = text.to_ascii_lowercase();
    for marker in markers {
        if let Some(marker_pos) = lowered.find(marker) {
            let start = marker_pos + marker.len();
            if start <= text.len()
                && let Some(component_name) =
                    extract_infrastructure_component_phrase(&text[start..])
            {
                return Some(component_name);
            }
        }
    }

    None
}

fn extract_infrastructure_targets_after_markers(text: &str, markers: &[&str]) -> Vec<String> {
    let lowered = text.to_ascii_lowercase();
    let mut actors = Vec::new();
    for marker in markers {
        if let Some(marker_pos) = lowered.find(marker) {
            let start = marker_pos + marker.len();
            if start <= text.len() {
                append_unique_infrastructure_actors(
                    &mut actors,
                    extract_infrastructure_target_phrases(&text[start..]),
                );
            }
        }
    }

    actors
}

fn parse_infrastructure_stage_count(lowered: &str) -> Option<u32> {
    const STAGE_WORDS: &[(u32, &str)] = &[
        (1, "one"),
        (2, "two"),
        (3, "three"),
        (4, "four"),
        (5, "five"),
        (6, "six"),
        (7, "seven"),
        (8, "eight"),
    ];

    for (count, word) in STAGE_WORDS {
        let numeric = count.to_string();
        for token in [numeric.as_str(), *word] {
            for pattern in [
                format!("{token}-stage"),
                format!("{token} stage"),
                format!("{token} stages"),
            ] {
                if lowered.contains(&pattern) {
                    return Some(*count);
                }
            }
        }
    }

    None
}

fn dedup_actor_names(actor_names: &mut Vec<String>) {
    let mut seen = BTreeSet::new();
    actor_names.retain(|actor_name| seen.insert(actor_name.clone()));
}

fn append_unique_infrastructure_actors(target: &mut Vec<String>, candidates: Vec<String>) {
    for candidate in candidates {
        if !target.contains(&candidate) {
            target.push(candidate);
        }
    }
}

fn extract_infrastructure_target_phrases(text: &str) -> Vec<String> {
    let mut segment = text.trim_start();
    for prefix in [
        "the ", "a ", "an ", "this ", "that ", "its ", "each ", "all ", "every ", "any ",
    ] {
        if segment.to_ascii_lowercase().starts_with(prefix) {
            segment = &segment[prefix.len()..];
            break;
        }
    }

    const STOP_CHARS: &[char] = &['.', ';', '(', ')', ':'];
    let mut end = segment
        .char_indices()
        .find_map(|(idx, character)| STOP_CHARS.contains(&character).then_some(idx))
        .unwrap_or(segment.len());
    let lowered = segment.to_ascii_lowercase();
    for boundary in [
        " when ", " if ", " while ", " during ", " after ", " before ", " with ", " using ",
        " on ", " at ", " for ",
    ] {
        if let Some(idx) = lowered.find(boundary) {
            end = end.min(idx);
        }
    }
    let segment = segment[..end].trim();
    if segment.is_empty() {
        return Vec::new();
    }

    let normalized_delimiters = segment
        .replace(", and ", ",")
        .replace(", or ", ",")
        .replace(" and ", ",")
        .replace(" or ", ",");

    normalized_delimiters
        .split(',')
        .filter_map(|candidate| {
            let candidate = candidate.trim();
            let candidate = strip_leading_infrastructure_target_prefix(candidate);
            let candidate = strip_trailing_infrastructure_target_suffix(candidate);
            let candidate = candidate
                .split_whitespace()
                .take(4)
                .collect::<Vec<_>>()
                .join(" ");
            normalize_infrastructure_component_name(&candidate)
        })
        .collect()
}

fn strip_leading_infrastructure_target_prefix(candidate: &str) -> &str {
    let lowered = candidate.to_ascii_lowercase();
    for prefix in [
        "the ", "a ", "an ", "this ", "that ", "its ", "each ", "all ", "every ", "any ",
    ] {
        if lowered.starts_with(prefix) {
            return &candidate[prefix.len()..];
        }
    }

    candidate
}

fn strip_trailing_infrastructure_target_suffix(candidate: &str) -> &str {
    let lowered = candidate.to_ascii_lowercase();
    for suffix in [
        " clock branch",
        " reset branch",
        " branch",
        " clock domain",
        " reset domain",
        " domain",
        " register bank",
        " registers",
        " register",
        " flip flops",
        " flops",
        " flop",
    ] {
        if lowered.ends_with(suffix) {
            return candidate[..candidate.len() - suffix.len()].trim_end();
        }
    }

    candidate
}

fn extract_infrastructure_component_phrase(text: &str) -> Option<String> {
    let trimmed = text.trim_start();
    let stripped = {
        let lowered = trimmed.to_ascii_lowercase();
        let mut result = trimmed;
        for prefix in [
            "the ", "a ", "an ", "this ", "that ", "its ", "each ", "all ", "every ", "any ",
        ] {
            if lowered.starts_with(prefix) {
                result = &trimmed[prefix.len()..];
                break;
            }
        }
        result
    };

    const STOP_DELIMITERS: &[char] = &['.', ',', ';', '(', ')', ':'];
    const STOP_WORDS: &[&str] = &[
        "to", "for", "and", "or", "in", "at", "on", "with", "when", "if", "by", "from", "that",
        "which", "where", "as", "is", "are", "has", "have", "will", "shall", "can", "may", "might",
        "must", "should", "could", "would", "feeds", "feed", "drives", "drive", "routes", "route",
        "clocks", "clock", "resets", "reset", "gates", "gate", "reaches", "reach",
    ];

    let mut words = Vec::new();
    for word in stripped.split_whitespace() {
        let clean = word.trim_matches(|character: char| {
            !character.is_ascii_alphanumeric() && character != '_' && character != '-'
        });
        if clean.is_empty() {
            break;
        }
        let lowered = clean.to_ascii_lowercase();
        if !words.is_empty() && STOP_WORDS.contains(&lowered.as_str()) {
            break;
        }
        if clean
            .chars()
            .any(|character| STOP_DELIMITERS.contains(&character))
        {
            let clean = clean.trim_end_matches(STOP_DELIMITERS);
            if !clean.is_empty() {
                words.push(clean);
            }
            break;
        }
        words.push(clean);
        if words.len() >= 4 {
            break;
        }
    }

    normalize_infrastructure_component_name(&words.join(" "))
}

fn extract_infrastructure_subject_phrase(text: &str) -> Option<String> {
    const DETERMINERS: &[&str] = &[
        "the", "a", "an", "this", "that", "these", "those", "each", "every", "any",
    ];
    const SKIP_WORDS: &[&str] = &[
        "the", "a", "an", "this", "that", "and", "or", "when", "if", "can", "may", "might", "must",
        "should", "could", "would", "will", "shall", "once", "before", "after", "while",
    ];

    let words = text.split_whitespace().collect::<Vec<_>>();
    for index in (0..words.len()).rev() {
        let determiner =
            words[index].trim_matches(|character: char| !character.is_ascii_alphabetic());
        if !DETERMINERS.contains(&determiner.to_ascii_lowercase().as_str()) {
            continue;
        }

        let mut actor_words = Vec::new();
        for word in &words[index + 1..] {
            let clean = word.trim_matches(|character: char| {
                !character.is_ascii_alphanumeric() && character != '_' && character != '-'
            });
            if clean.is_empty() {
                break;
            }
            if SKIP_WORDS.contains(&clean.to_ascii_lowercase().as_str()) {
                break;
            }
            actor_words.push(clean);
            if actor_words.len() >= 4 {
                break;
            }
        }

        if let Some(actor) = normalize_infrastructure_component_name(&actor_words.join(" ")) {
            return Some(actor);
        }
    }

    let mut actor_words = Vec::new();
    for word in words.iter().rev() {
        let clean = word.trim_matches(|character: char| {
            !character.is_ascii_alphanumeric() && character != '_' && character != '-'
        });
        if clean.is_empty() {
            break;
        }
        let lowered = clean.to_ascii_lowercase();
        if SKIP_WORDS.contains(&lowered.as_str()) {
            if !actor_words.is_empty() {
                break;
            }
            continue;
        }
        actor_words.push(clean);
        if actor_words.len() >= 4 {
            break;
        }
    }
    actor_words.reverse();

    normalize_infrastructure_component_name(&actor_words.join(" "))
}

fn normalize_infrastructure_component_name(text: &str) -> Option<String> {
    let trimmed = text.trim();
    if trimmed.len() < 2 {
        return None;
    }
    let normalized = normalize_actor_term(trimmed);
    if normalized.is_empty() {
        return None;
    }
    if matches!(
        normalized.as_str(),
        "clock"
            | "reset"
            | "global"
            | "external"
            | "external clock"
            | "external reset"
            | "input"
            | "output"
            | "source"
            | "driver"
            | "signal"
    ) {
        return None;
    }
    if is_meaningful_actor_term(trimmed) || is_explicit_infrastructure_component_term(&normalized) {
        return Some(trimmed.to_string());
    }

    None
}

fn is_explicit_infrastructure_component_term(normalized: &str) -> bool {
    matches!(
        normalized,
        "pll" | "dll" | "oscillator" | "clkgen" | "rstgen"
    ) || normalized.contains("generator")
        || normalized.contains("controller")
        || normalized.contains("synchronizer")
        || normalized.contains("synchroniser")
        || normalized.contains("oscillator")
        || normalized.contains("divider")
        || normalized.contains("clock gate")
        || normalized.contains("clock mux")
        || normalized.contains("reset mux")
        || normalized.contains("reset bridge")
}

fn infrastructure_source_status(source_count: usize) -> InfrastructureSignalSourceStatus {
    match source_count {
        0 => InfrastructureSignalSourceStatus::UnresolvedSource,
        1 => InfrastructureSignalSourceStatus::RecoveredProducer,
        _ => InfrastructureSignalSourceStatus::MultipleRecoveredProducers,
    }
}

fn infrastructure_distribution_status(
    consumer_count: usize,
) -> InfrastructureSignalDistributionStatus {
    match consumer_count {
        0 => InfrastructureSignalDistributionStatus::NoRecoveredConsumers,
        1 => InfrastructureSignalDistributionStatus::SingleRecoveredConsumer,
        _ => InfrastructureSignalDistributionStatus::SharedRecoveredConsumers,
    }
}

fn classify_signal_connectivity(
    signal_name: &str,
    system_contract: Option<&SystemContractRecord>,
) -> SignalConnectivityClass {
    let Some(system_contract) = system_contract else {
        return SignalConnectivityClass::Protocol;
    };

    if signal_name == system_contract.clock_signal {
        return SignalConnectivityClass::SystemClock;
    }
    if signal_name == system_contract.reset_signal {
        return SignalConnectivityClass::SystemReset;
    }

    SignalConnectivityClass::Protocol
}

fn build_signal_connectivity_conflicts(
    signal_connectivity: &[SignalConnectivityRecord],
) -> Vec<SignalConnectivityConflictRecord> {
    let mut conflicts = Vec::new();

    for record in signal_connectivity {
        if record.producer_actor_ids.len() <= 1 {
            continue;
        }

        let mut conflicting_actor_ids = record.producer_actor_ids.clone();
        conflicting_actor_ids.sort();
        conflicting_actor_ids.dedup();
        let mut conflicting_actor_names = record.producer_actor_names.clone();
        conflicting_actor_names.sort();
        conflicting_actor_names.dedup();
        let mut supporting_statement_ids = record.source_statement_ids.clone();
        supporting_statement_ids.sort();
        supporting_statement_ids.dedup();

        conflicts.push(SignalConnectivityConflictRecord {
            conflict_id: format!("signal_connectivity_conflict_{:04}", conflicts.len() + 1),
            signal_name: record.signal_name.clone(),
            conflict_kind: SignalConnectivityConflictKind::MultipleProducers,
            conflicting_actor_ids,
            conflicting_actor_names,
            supporting_statement_ids,
            automation_confidence: record.automation_confidence,
        });
    }

    conflicts
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
    temporal_rules: &[TemporalRuleRecord],
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

    let semantically_grounded_visual_ids: BTreeSet<String> = interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .flat_map(|signal| signal.semantic_observations.iter())
        .flat_map(|observation| observation.supporting_visual_evidence_ids.iter().cloned())
        .collect();
    let ambiguous_visual_ids: Vec<String> = context
        .statements
        .iter()
        .flat_map(|statement| statement.related_visual_evidence_ids.iter())
        .filter(|visual_id| {
            semantically_grounded_visual_ids.contains(*visual_id)
                && context
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

    let blocked_handshake_fallback_signals =
        handshake_name_fallback_blocked_signal_names(interfaces);
    if !blocked_handshake_fallback_signals.is_empty() {
        packets.push(ResidualDecisionPacket {
            packet_id: "semantic_handshake_name_fallback_blocked".to_string(),
            question:
                "Should handshake-shaped signal names override contested or provisional semantic role evidence?"
                    .to_string(),
            why_unresolved: format!(
                "Signals {} look handshake-shaped by name, but their preserved semantic role state is still contested or only provisional, so SemanticIR blocks literal VALID/READY fallback instead of promoting a potentially wrong role.",
                blocked_handshake_fallback_signals.join(", ")
            ),
            automation_confidence: AutomationConfidence::Medium,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "preserve_contested_semantics".to_string(),
                    description: "Keep the role unresolved until stronger multimodal evidence or user guidance turns the semantic meaning into a grounded consensus.".to_string(),
                    downstream_impact: "Typed temporal handshake predicates stay conservative, but some protocol progress semantics remain deferred while provisional meaning stays explicit.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "trust_name_heuristic".to_string(),
                    description: "Let the handshake-shaped signal name override the contested or provisional semantic evidence.".to_string(),
                    downstream_impact: "More temporal handshake structure appears immediately, but semantic invention risk increases because spelling outranks preserved disagreement or weaker fallback-only meaning.".to_string(),
                },
            ],
        });
    }

    let fallback_only_semantic_roles =
        resolved_semantic_roles_without_consensus_signal_names(interfaces);
    if !fallback_only_semantic_roles.is_empty() {
        packets.push(ResidualDecisionPacket {
            packet_id: "semantic_resolved_role_without_consensus".to_string(),
            question:
                "Should fallback-only semantic role resolutions remain canonical without observation-backed consensus?"
                    .to_string(),
            why_unresolved: format!(
                "Signals {} currently resolve a semantic role, but that role still lacks preserved observation-backed consensus, so SemanticIR is carrying a provisional meaning rather than a fully grounded canonical one.",
                fallback_only_semantic_roles.join(", ")
            ),
            automation_confidence: AutomationConfidence::Medium,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "keep_provisional_role".to_string(),
                    description: "Keep the fallback semantic role as a provisional canonical hint until stronger grounding arrives.".to_string(),
                    downstream_impact: "Downstream consumers keep some useful role structure, but they must treat the meaning as weaker than observation-backed consensus.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "clear_unbacked_role".to_string(),
                    description: "Clear the fallback semantic role until preserved observations support a true consensus.".to_string(),
                    downstream_impact: "The canonical model becomes more conservative, but some useful protocol role structure disappears until later passes recover it safely.".to_string(),
                },
            ],
        });
    }

    let alias_dependent_handshake_signals =
        alias_dependent_handshake_completion_signal_names(temporal_rules, interfaces);
    if !alias_dependent_handshake_signals.is_empty() {
        packets.push(ResidualDecisionPacket {
            packet_id: "semantic_alias_dependent_handshake_completion".to_string(),
            question:
                "Should alias-grounded semantic role consensus be enough to keep typed handshake-completion semantics canonical?"
                    .to_string(),
            why_unresolved: format!(
                "Typed HandshakeComplete predicates currently depend on alias-grounded semantic role consensus for signals {}. SemanticIR keeps that transfer-progress structure, but the current grounding is weaker than direct or corroborating non-alias evidence.",
                alias_dependent_handshake_signals.join(", ")
            ),
            automation_confidence: AutomationConfidence::Medium,
            candidate_interpretations: vec![
                CandidateInterpretation {
                    interpretation_id: "keep_alias_grounded_handshake".to_string(),
                    description: "Keep the typed handshake-completion semantics, but treat them as provisional until direct or corroborating non-alias evidence arrives.".to_string(),
                    downstream_impact: "Useful temporal progress structure stays available, but downstream consumers must preserve the weaker alias-dependent grounding explicitly.".to_string(),
                },
                CandidateInterpretation {
                    interpretation_id: "require_stronger_handshake_grounding".to_string(),
                    description: "Defer canonical handshake-completion semantics until direct signal mentions or corroborating non-alias modalities support the role meaning.".to_string(),
                    downstream_impact: "The canonical temporal model becomes more conservative, but alias-only role recovery no longer contributes protocol-progress structure immediately.".to_string(),
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

    let direction_hint = tokens
        .get(index)
        .and_then(|token| parse_interface_signal_direction(token))
        .inspect(|_| index += 1);

    let width_hint = if direction_hint.is_some()
        || tokens
            .get(index)
            .is_some_and(|token| token.eq_ignore_ascii_case("width"))
    {
        parse_optional_width_hint(tokens.as_slice(), &mut index)
    } else {
        None
    };

    if direction_hint.is_none() && width_hint.is_none() {
        return None;
    }
    if index != tokens.len() {
        return None;
    }

    Some(ParsedInterfaceSignalDeclaration {
        signal_name,
        direction_hint,
        width_hint,
    })
}

fn interface_signal_declaration_looks_like_width_symbol(
    declaration: &ParsedInterfaceSignalDeclaration,
) -> bool {
    declaration.direction_hint.is_none()
        && declaration.width_hint.is_some()
        && declaration.signal_name.ends_with("_WIDTH")
}

fn heuristic_interface_signal_looks_like_metadata(signal_name: &str) -> bool {
    let normalized = signal_name.trim().to_ascii_uppercase();

    normalized == "MIN"
        || normalized == "MAX"
        || normalized == "_WIDTH"
        || normalized.ends_with("_WIDTH")
}

fn filtered_interface_candidate_signals(signals: &[String]) -> Vec<String> {
    let mut filtered = Vec::new();
    let mut seen = BTreeSet::new();

    for signal in signals {
        if heuristic_interface_signal_looks_like_metadata(signal) {
            continue;
        }
        if seen.insert(signal.clone()) {
            filtered.push(signal.clone());
        }
    }

    filtered
}

fn retain_authoritative_interface_candidate_signals(
    signals: &[String],
    authoritative_signal_names: &BTreeSet<String>,
    statement_text: &str,
) -> Vec<String> {
    // Co-mention normally enriches only a formal declaration or the document system contract.
    // A declaration-free document has one narrow positive path: a signal-led deontic behavior
    // statement can ground its own multi-signal interface (for example, an asserted-until-
    // observed handshake). Ordinary uppercase prose and table tokens still fail closed.
    if authoritative_signal_names.is_empty() {
        return if statement_establishes_signal_behavior(signals, statement_text) {
            signals.to_vec()
        } else {
            Vec::new()
        };
    }

    signals
        .iter()
        .filter(|signal| authoritative_signal_names.contains(*signal))
        .cloned()
        .collect()
}

fn statement_establishes_signal_behavior(signals: &[String], statement_text: &str) -> bool {
    if signals.len() < 2 {
        return false;
    }

    let trimmed = statement_text.trim();
    let split_at = trimmed.find(char::is_whitespace).unwrap_or(trimmed.len());
    let subject = &trimmed[..split_at];
    if !signals.iter().any(|signal| signal == subject) {
        return false;
    }

    let behavior = normalize_sentence(trimmed[split_at..].trim()).to_ascii_lowercase();
    let deontic = behavior.starts_with("must ") || behavior.starts_with("shall ");
    let signal_action = behavior.contains("assert")
        || behavior.contains("deassert")
        || behavior.contains("remain stable")
        || behavior.contains("held stable");

    deontic && signal_action
}

fn heuristic_interface_candidate_is_redundant_with_authoritative_surface(
    signals: &[String],
    authoritative_signal_names: &BTreeSet<String>,
    statement_text: &str,
) -> bool {
    if authoritative_signal_names.is_empty() || signals.is_empty() {
        return false;
    }

    if !signals
        .iter()
        .all(|signal| authoritative_signal_names.contains(signal))
    {
        return false;
    }

    signals.len() == 1 || statement_text_describes_signal_polarity(statement_text)
}

fn statement_text_describes_signal_polarity(text: &str) -> bool {
    let lowered = normalize_sentence(text).to_ascii_lowercase();
    [
        "active low",
        "active-low",
        "asserted low",
        "low asserted",
        "asserted when low",
        "low when asserted",
        "asserted by driving low",
        "driven low to assert",
        "active high",
        "active-high",
        "asserted high",
        "high asserted",
        "asserted when high",
        "high when asserted",
        "asserted by driving high",
        "driven high to assert",
    ]
    .iter()
    .any(|phrase| lowered.contains(phrase))
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

    // `?` keeps the parse-validity guard even though the value is unused now.
    normalize_decision_tree_block_name(raw_block_name)?;
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
    if let Some(stripped) = normalized.strip_prefix("->") {
        return Some(ControlActionRecord::Transition {
            target_state: parse_identifier(stripped.trim())?,
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
        while let Some((_, operator)) = operators
            .iter()
            .find(|(token, _)| self.peek().is_some_and(|next| next == *token))
        {
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
    let token = tokens.get(*index).copied()?;

    if token.eq_ignore_ascii_case("width") {
        let width = parse_width_token(tokens.get(*index + 1).copied()?)?;
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
        if let Some(signal_declaration) = parse_explicit_signal_declaration(&statement.text)
            && !interface_signal_declaration_looks_like_width_symbol(&signal_declaration)
        {
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
        if let Some(transition) = parse_explicit_state_transition(&statement.text)
            && let Some(guard) = transition.guard.as_ref()
        {
            signal_names.extend(referenced_signal_names_for_guard(guard));
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

fn actor_id_for_name(actor_name: &str) -> String {
    format!("actor_{}", document_key(actor_name))
}

fn infrastructure_topology_id(
    signal_name: &str,
    topology_kind: InfrastructureTopologyKind,
    statement_id: &str,
    ordinal: usize,
) -> String {
    format!(
        "infrastructure_topology_{}_{}_{}_{}",
        document_key(signal_name),
        topology_kind.as_str(),
        document_key(statement_id),
        ordinal
    )
}

fn actor_relative_direction_and_basis(
    drives: bool,
    reads: bool,
) -> (ActorRelativeDirection, Vec<RelationKind>) {
    match (drives, reads) {
        (true, true) => (
            ActorRelativeDirection::InOut,
            vec![RelationKind::Drives, RelationKind::Reads],
        ),
        (true, false) => (ActorRelativeDirection::Output, vec![RelationKind::Drives]),
        (false, true) => (ActorRelativeDirection::Input, vec![RelationKind::Reads]),
        (false, false) => (ActorRelativeDirection::Unknown, Vec::new()),
    }
}

fn signal_width_hints_by_name(
    interfaces: &[InterfaceRecord],
) -> HashMap<String, Option<WidthHint>> {
    let mut widths = HashMap::new();

    for signal in interfaces
        .iter()
        .flat_map(|interface| &interface.signal_records)
    {
        let entry = widths.entry(signal.signal_name.clone()).or_insert(None);
        merge_signal_hint(entry, signal.width_hint.clone());
    }

    widths
}

fn build_interface_signal_conflict_observations(
    observations: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<InterfaceSignalConflictObservationRecord> {
    observations
        .iter()
        .map(
            |(value_text, supporting_statement_ids)| InterfaceSignalConflictObservationRecord {
                value_text: value_text.clone(),
                supporting_statement_ids: supporting_statement_ids.iter().cloned().collect(),
            },
        )
        .collect()
}

fn register_interface_signal_record(
    accumulator: &mut InterfaceAccumulator,
    signal_name: &str,
    direction_hint: Option<InterfaceSignalDirection>,
    width_hint: Option<WidthHint>,
    supporting_statement_id: &str,
    supporting_table_ids: &[String],
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
            direction_hint_conflicted: false,
            width_hint: None,
            width_hint_conflicted: false,
            semantic_tags: BTreeSet::new(),
            semantic_observations: Vec::new(),
            direction_observations: BTreeMap::new(),
            width_observations: BTreeMap::new(),
            supporting_statement_ids: BTreeSet::new(),
            supporting_table_ids: BTreeSet::new(),
            automation_confidence,
        });
    if let Some(direction_hint) = direction_hint {
        entry
            .direction_observations
            .entry(direction_hint.as_str().to_string())
            .or_default()
            .insert(supporting_statement_id.to_string());
    }
    if let Some(width_hint) = width_hint.clone() {
        entry
            .width_observations
            .entry(width_hint_key(&width_hint))
            .or_default()
            .insert(supporting_statement_id.to_string());
    }
    merge_sticky_signal_hint(
        &mut entry.direction_hint,
        &mut entry.direction_hint_conflicted,
        direction_hint,
    );
    merge_sticky_signal_hint(
        &mut entry.width_hint,
        &mut entry.width_hint_conflicted,
        width_hint,
    );
    entry
        .supporting_statement_ids
        .insert(supporting_statement_id.to_string());
    entry
        .supporting_table_ids
        .extend(supporting_table_ids.iter().cloned());
    entry.automation_confidence =
        max_automation_confidence(entry.automation_confidence, automation_confidence);
}

fn register_interface_signal_semantic_hint(
    accumulator: &mut InterfaceAccumulator,
    signal_name: &str,
    hint: &SignalSemanticHintRecord,
) {
    if hint.semantic_tags.is_empty() {
        return;
    }
    let Some(entry) = accumulator.signal_records.get_mut(signal_name) else {
        return;
    };
    entry
        .semantic_tags
        .extend(hint.semantic_tags.iter().copied());
    let observation = InterfaceSignalSemanticObservationRecord {
        semantic_tags: hint.semantic_tags.clone(),
        source_kind: hint.source_kind,
        source_text: hint.source_text.clone(),
        supporting_statement_ids: hint.supporting_statement_ids.clone(),
        supporting_table_ids: hint.supporting_table_ids.clone(),
        supporting_visual_evidence_ids: hint.supporting_visual_evidence_ids.clone(),
        automation_confidence: hint.automation_confidence,
    };
    if !entry.semantic_observations.iter().any(|existing| {
        existing.semantic_tags == observation.semantic_tags
            && existing.source_kind == observation.source_kind
            && existing.source_text == observation.source_text
            && existing.supporting_statement_ids == observation.supporting_statement_ids
            && existing.supporting_table_ids == observation.supporting_table_ids
            && existing.supporting_visual_evidence_ids == observation.supporting_visual_evidence_ids
    }) {
        entry.semantic_observations.push(observation);
    }
}

fn width_hint_key(width_hint: &WidthHint) -> String {
    match width_hint {
        WidthHint::Numeric(bits) => bits.to_string(),
        WidthHint::Parametric(expr) => expr.clone(),
    }
}

fn merge_signal_hint<T: Clone + Eq>(target: &mut Option<T>, incoming: Option<T>) {
    match (target.as_ref(), incoming.as_ref()) {
        (None, Some(value)) => *target = Some(value.clone()),
        (Some(existing), Some(value)) if existing != value => *target = None,
        _ => {}
    }
}

fn merge_sticky_signal_hint<T: Clone + Eq>(
    target: &mut Option<T>,
    target_conflicted: &mut bool,
    incoming: Option<T>,
) {
    match (target.as_ref(), *target_conflicted, incoming.as_ref()) {
        (_, true, Some(_)) => {}
        (None, false, Some(value)) => *target = Some(value.clone()),
        (Some(existing), false, Some(value)) if existing != value => {
            *target = None;
            *target_conflicted = true;
        }
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

    let mut chars = token.chars();
    let Some(first) = chars.next() else {
        return false;
    };
    if !(first.is_ascii_alphabetic() || first == '_') {
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
        // NOTE: protocol family names (AMBA/AHB/AXI/CHI/…) and vendor names
        // (ARM/AMD/INTEL/…) were REMOVED here — they are chip-spec-specific and
        // hardcoding them violates ADR 0006 (PDF-AGNOSTIC-EXTRACTION.3). A token
        // mis-discovered as a signal is filtered downstream against the document's
        // own declared signals (`declared_signal_names`). Only document-independent
        // standards/structure/digital tokens remain below.
        // --- Standards bodies (universal, document-independent) ---
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
        // (ARM/AMBA publication-ID prefixes removed — chip-spec-specific, ADR 0006.)
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
        // (AMBA HTRANS/HBURST encoding values removed — chip-spec-specific, ADR 0006.)
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
    let explicit_signal_sets: Vec<BTreeSet<String>> = interfaces
        .iter()
        .filter(|interface| interface.interface_id.starts_with("interface_explicit_"))
        .map(|interface| interface.signals.iter().cloned().collect())
        .collect();

    let mut counts: HashMap<String, usize> = HashMap::new();

    for interface in interfaces {
        if !interface.interface_id.starts_with("interface_explicit_")
            && explicit_signal_sets.iter().any(|explicit_signals| {
                interface
                    .signals
                    .iter()
                    .all(|signal| explicit_signals.contains(signal))
            })
        {
            continue;
        }
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

#[expect(
    clippy::too_many_arguments,
    reason = "temporal rule synthesis intentionally consumes the explicit semantic evidence planes"
)]
fn build_temporal_rules(
    context: &SemanticContext,
    interfaces: &[InterfaceRecord],
    system_contract: Option<&SystemContractRecord>,
    signal_connectivity: &[SignalConnectivityRecord],
    signal_constraints: &[SignalConstraintRecord],
    conditional_rules: &[ConditionalRuleRecord],
    timing_constraints: &[TimingConstraintRecord],
    actor_names: &BTreeSet<String>,
    prior_guidance: Option<&SemanticPriorGuidance>,
) -> Vec<TemporalRuleRecord> {
    let known_signals = interfaces
        .iter()
        .flat_map(|interface| {
            interface
                .signal_records
                .iter()
                .map(|signal| signal.signal_name.clone())
        })
        .collect::<BTreeSet<_>>();
    let default_clock = temporal_clock_signal(context, system_contract);
    let default_edge = if default_clock.is_some() {
        ClockEdge::Rising
    } else {
        ClockEdge::Unknown
    };
    let handshake_role_context = handshake_role_context(interfaces);
    let unique_producer_by_signal = unique_producer_by_signal(signal_connectivity);
    let mut rules = Vec::new();

    for constraint in signal_constraints {
        let antecedents = constraint
            .condition_text
            .as_deref()
            .map(|text| {
                parse_temporal_condition_predicates(
                    text,
                    &known_signals,
                    TickPhase::PreTick,
                    &handshake_role_context,
                )
            })
            .unwrap_or_default();
        let consequents =
            temporal_consequents_from_signal_constraint(constraint, &unique_producer_by_signal);
        if consequents.is_empty() {
            continue;
        }
        let actor_grounded = temporal_predicates_are_actor_grounded(&antecedents, &consequents);
        let handshake_completion =
            temporal_predicates_have_handshake_completion(&antecedents, &consequents);
        let explicit_clock_signal =
            explicit_clock_signal_from_text(&constraint.source_text, &known_signals);
        rules.push(TemporalRuleRecord {
            rule_id: format!("temporal_signal_constraint_{}", constraint.constraint_id),
            clock_signal: explicit_clock_signal
                .clone()
                .or_else(|| default_clock.clone()),
            edge: grounded_temporal_edge(
                &constraint.source_text,
                explicit_clock_signal.as_deref(),
                default_edge,
            ),
            antecedents,
            consequents,
            cycle_window: resolve_cycle_window_from_text(
                &constraint.source_text,
                &known_signals,
                actor_names,
                actor_grounded,
                handshake_completion,
                prior_guidance,
            ),
            source_text: constraint.source_text.clone(),
            supporting_statement_ids: constraint.supporting_statement_ids.clone(),
            automation_confidence: constraint.automation_confidence,
        });
    }

    for rule in conditional_rules {
        let consequents = temporal_consequents_from_conditional_rule(
            rule,
            &known_signals,
            &unique_producer_by_signal,
            &handshake_role_context,
        );
        if consequents.is_empty() {
            continue;
        }
        let antecedents = parse_temporal_condition_predicates(
            &rule.antecedent_text,
            &known_signals,
            TickPhase::PreTick,
            &handshake_role_context,
        );
        let actor_grounded = temporal_predicates_are_actor_grounded(&antecedents, &consequents);
        let handshake_completion =
            temporal_predicates_have_handshake_completion(&antecedents, &consequents);
        let explicit_clock_signal =
            explicit_clock_signal_from_text(&rule.source_text, &known_signals);
        rules.push(TemporalRuleRecord {
            rule_id: format!("temporal_conditional_rule_{}", rule.rule_id),
            clock_signal: explicit_clock_signal
                .clone()
                .or_else(|| default_clock.clone()),
            edge: grounded_temporal_edge(
                &rule.source_text,
                explicit_clock_signal.as_deref(),
                default_edge,
            ),
            antecedents,
            consequents,
            cycle_window: resolve_cycle_window_from_text(
                &rule.source_text,
                &known_signals,
                actor_names,
                actor_grounded,
                handshake_completion,
                prior_guidance,
            ),
            source_text: rule.source_text.clone(),
            supporting_statement_ids: rule.supporting_statement_ids.clone(),
            automation_confidence: rule.automation_confidence,
        });
    }

    for timing in timing_constraints {
        let Some(description) = timing.description.as_deref() else {
            continue;
        };
        let Some(rule) = temporal_rule_from_timing_constraint(
            timing,
            description,
            &known_signals,
            default_clock.as_deref(),
            actor_names,
            prior_guidance,
        ) else {
            continue;
        };
        rules.push(rule);
    }

    dedup_temporal_rules(rules)
}

fn temporal_clock_signal(
    context: &SemanticContext,
    system_contract: Option<&SystemContractRecord>,
) -> Option<String> {
    let mut clock_signal = system_contract.map(|contract| contract.clock_signal.clone());
    for statement in &context.statements {
        if let Some(parsed_clock) = parse_explicit_system_clock(&statement.text) {
            let _ = merge_named_hint(&mut clock_signal, &parsed_clock);
        }
    }
    clock_signal
}

fn parse_temporal_condition_predicates(
    text: &str,
    known_signals: &BTreeSet<String>,
    phase: TickPhase,
    handshake_role_context: &HandshakeRoleContext,
) -> Vec<TemporalPredicateRecord> {
    let normalized = text
        .trim()
        .trim_start_matches("when ")
        .trim_start_matches("When ")
        .trim_start_matches("if ")
        .trim_start_matches("If ")
        .trim_start_matches("while ")
        .trim_start_matches("While ")
        .trim();
    // Drop a trailing EXCEPTION clause ("… unless HRESP is ERROR", "… except when …"): an exception
    // is a negative caveat, NOT a positive antecedent, so it must not be conjoined into the
    // condition (e.g. "valid when HREADY is HIGH, unless HRESP is ERROR" → antecedent HREADY=HIGH
    // only, not HREADY=HIGH AND HRESP=ERROR). WIRE-BASED-100.5h.
    let normalized = {
        let lowered = normalized.to_ascii_lowercase();
        let cut = [" unless ", " except "]
            .iter()
            .filter_map(|m| lowered.find(m))
            .min();
        match cut {
            Some(pos) => normalized[..pos].trim(),
            None => normalized,
        }
    };
    // Parse each clause into (signal, optional value). A clause with a known signal but no
    // value of its own (a bare mention in a coordinated list, e.g. "PSEL" in
    // "PSEL, PENABLE, and PREADY are asserted") is kept with value=None so a single shared
    // trailing value can be distributed across the list (TEMPORAL-ANTECEDENT-RECALL).
    let mut parts: Vec<(String, Option<String>)> =
        split_temporal_condition_clauses(normalized, known_signals)
            .into_iter()
            .filter_map(|clause| {
                // Resolve the clause's signal: an exactly-declared name first, else an
                // un-indexed prose reference to a declared indexed family member
                // (e.g. prose "PSEL" → declared "PSELx"/"PSELX"). The fallback fires
                // only when the bare token is not itself declared, so it is purely
                // additive and uses the canonical declared identity (WIRE-BASED-100.4).
                find_known_signal_name(&clause, known_signals)
                    .or_else(|| resolve_indexed_signal_family(&clause, known_signals))
                    .map(|signal| {
                        let value = temporal_clause_value(&clause, &signal, known_signals);
                        (signal, value)
                    })
            })
            .collect();

    // Distribute a single shared trailing value across the coordinated list: only when there
    // are >=2 signals, exactly one distinct value is present, and at least one signal has no
    // value of its own. A genuinely mixed list ("PSEL HIGH, PREADY LOW") is untouched, and a
    // bare signal with no shared value is still dropped below -- no fabrication.
    let distinct_values: BTreeSet<&String> = parts.iter().filter_map(|(_, v)| v.as_ref()).collect();
    if parts.len() >= 2 && distinct_values.len() == 1 && parts.iter().any(|(_, v)| v.is_none()) {
        let shared = distinct_values
            .iter()
            .next()
            .expect("one distinct value")
            .to_string();
        for (_, value) in parts.iter_mut() {
            if value.is_none() {
                *value = Some(shared.clone());
            }
        }
    }

    let mut seen = BTreeSet::new();
    let predicates = parts
        .into_iter()
        .filter_map(|(signal_name, value)| {
            value.map(|value| TemporalPredicateRecord::SignalValue {
                signal_name,
                value,
                phase,
            })
        })
        .filter(|predicate| {
            serde_json::to_string(predicate)
                .map(|key| seen.insert(key))
                .unwrap_or(true)
        })
        .collect::<Vec<_>>();
    enrich_handshake_completion_predicates(predicates, handshake_role_context)
}

fn split_temporal_condition_clauses(text: &str, known_signals: &BTreeSet<String>) -> Vec<String> {
    let mut segments = text
        .split(',')
        .flat_map(|segment| segment.split("&&"))
        .map(str::trim)
        .filter(|segment| !segment.is_empty())
        .map(ToOwned::to_owned)
        .collect::<Vec<_>>();

    if segments.is_empty() {
        return Vec::new();
    }

    let mut clauses = Vec::new();
    for segment in segments.drain(..) {
        clauses.extend(split_temporal_condition_segment_on_and(
            &segment,
            known_signals,
        ));
    }
    clauses
}

fn split_temporal_condition_segment_on_and(
    text: &str,
    known_signals: &BTreeSet<String>,
) -> Vec<String> {
    let lowered = text.to_ascii_lowercase();
    if !lowered.contains(" and ") {
        return vec![text.trim().to_string()];
    }

    let candidate_parts = lowered.match_indices(" and ").map(|(index, _)| index).fold(
        (Vec::new(), 0usize),
        |(mut parts, start), index| {
            parts.push(text[start..index].trim().to_string());
            (parts, index + 5)
        },
    );
    let (mut parts, last_start) = candidate_parts;
    parts.push(text[last_start..].trim().to_string());

    let all_parts_are_signal_anchored = parts.len() > 1
        && parts
            .iter()
            .all(|part| !part.is_empty() && find_known_signal_name(part, known_signals).is_some());
    if all_parts_are_signal_anchored {
        parts
    } else {
        vec![text.trim().to_string()]
    }
}

/// The value keyword carried by a single condition clause (LOW / HIGH / ASSERTED /
/// DEASSERTED, or a symbolic value), or `None` if the clause is a bare signal mention with no
/// value of its own. Check order is preserved from the original clause parser so a clause's
/// own value is unchanged; the caller distributes a shared value to value-less clauses.
///
/// A token that is itself a signal — declared, or an un-indexed reference to a declared
/// indexed family member (e.g. "PSEL" → "PSELx") — is never a value: a bare list member like
/// "PSEL" must stay value-less so the shared list value distributes to it, not leak the prose
/// token as a fake value (`WIRE-BASED-100.4`).
fn temporal_clause_value(
    text: &str,
    signal_name: &str,
    known_signals: &BTreeSet<String>,
) -> Option<String> {
    if contains_phrase_case_insensitive(text, "LOW") {
        Some("LOW".to_string())
    } else if contains_phrase_case_insensitive(text, "HIGH") {
        Some("HIGH".to_string())
    } else if contains_phrase_case_insensitive(text, "asserted") {
        Some("ASSERTED".to_string())
    } else if contains_phrase_case_insensitive(text, "deasserted") {
        Some("DEASSERTED".to_string())
    } else {
        extract_symbolic_value(text, Some(signal_name)).filter(|candidate| {
            let upper = candidate.to_ascii_uppercase();
            !known_signals.contains(&upper)
                && resolve_indexed_signal_family(candidate, known_signals).is_none()
        })
    }
}

fn enrich_handshake_completion_predicates(
    mut predicates: Vec<TemporalPredicateRecord>,
    handshake_role_context: &HandshakeRoleContext,
) -> Vec<TemporalPredicateRecord> {
    let mut existing = predicates
        .iter()
        .filter_map(|predicate| match predicate {
            TemporalPredicateRecord::HandshakeComplete {
                valid_signal,
                ready_signal,
                phase,
            } => serde_json::to_string(&(valid_signal, ready_signal, phase)).ok(),
            _ => None,
        })
        .collect::<BTreeSet<_>>();

    let mut valid_assertions = Vec::<(String, TickPhase)>::new();
    let mut ready_assertions = Vec::<(String, TickPhase)>::new();
    for predicate in &predicates {
        let TemporalPredicateRecord::SignalValue {
            signal_name,
            value,
            phase,
        } = predicate
        else {
            continue;
        };
        if !is_handshake_asserted_value(value) {
            continue;
        }
        match classify_handshake_signal(signal_name, handshake_role_context) {
            Some(HandshakeSignalRole::Valid) => {
                valid_assertions.push((signal_name.clone(), *phase))
            }
            Some(HandshakeSignalRole::Ready) => {
                ready_assertions.push((signal_name.clone(), *phase))
            }
            None => {}
        }
    }

    for (valid_signal, phase) in &valid_assertions {
        for (ready_signal, ready_phase) in &ready_assertions {
            if phase != ready_phase || valid_signal == ready_signal {
                continue;
            }
            let signature = serde_json::to_string(&(valid_signal, ready_signal, phase))
                .unwrap_or_else(|_| format!("{valid_signal}:{ready_signal}:{phase:?}"));
            if existing.insert(signature) {
                predicates.push(TemporalPredicateRecord::HandshakeComplete {
                    valid_signal: valid_signal.clone(),
                    ready_signal: ready_signal.clone(),
                    phase: *phase,
                });
            }
        }
    }

    predicates
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HandshakeSignalRole {
    Valid,
    Ready,
}

#[derive(Debug, Default)]
struct HandshakeRoleContext {
    resolved_roles_by_signal: BTreeMap<String, HandshakeSignalRole>,
    heuristic_blocked_signals: BTreeSet<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum SemanticObservationModality {
    Table,
    Prose,
    Visual,
}

#[expect(
    clippy::type_complexity,
    reason = "semantic role resolution returns the coupled candidate, arbitration, role, strength, and consensus surfaces"
)]
fn resolve_interface_signal_semantic_role(
    semantic_tags: &[SignalSemanticTag],
    semantic_observations: &[InterfaceSignalSemanticObservationRecord],
    prior_guidance: Option<&SemanticPriorGuidance>,
) -> (
    Vec<InterfaceSignalSemanticCandidateRecord>,
    Option<InterfaceSignalSemanticArbitrationRecord>,
    Option<InterfaceSignalSemanticRole>,
    Option<SemanticGroundingStrength>,
    Option<InterfaceSignalSemanticConsensusRecord>,
) {
    let semantic_candidates = build_semantic_candidates(semantic_observations, prior_guidance);
    let semantic_arbitration = build_semantic_arbitration(&semantic_candidates);

    match semantic_candidates.as_slice() {
        [candidate] => (
            semantic_candidates.clone(),
            semantic_arbitration,
            Some(candidate.role),
            Some(candidate.grounding_strength),
            Some(build_semantic_consensus(candidate)),
        ),
        [_, ..] => {
            if semantic_arbitration
                .as_ref()
                .is_some_and(|arbitration| arbitration.decisive)
            {
                let leading_candidate = semantic_candidates.first().expect("checked non-empty");
                let leading_role = leading_candidate.role;
                let leading_grounding_strength = leading_candidate.grounding_strength;
                let consensus = build_semantic_consensus(leading_candidate);
                (
                    semantic_candidates,
                    semantic_arbitration,
                    Some(leading_role),
                    Some(leading_grounding_strength),
                    Some(consensus),
                )
            } else {
                (semantic_candidates, semantic_arbitration, None, None, None)
            }
        }
        [] => {
            let has_valid_tag = semantic_tags_support_semantic_role(
                semantic_tags,
                InterfaceSignalSemanticRole::HandshakeValidLike,
            );
            let has_ready_tag = semantic_tags_support_semantic_role(
                semantic_tags,
                InterfaceSignalSemanticRole::HandshakeReadyLike,
            );
            match (has_valid_tag, has_ready_tag) {
                (true, false) => (
                    Vec::new(),
                    None,
                    Some(InterfaceSignalSemanticRole::HandshakeValidLike),
                    None,
                    None,
                ),
                (false, true) => (
                    Vec::new(),
                    None,
                    Some(InterfaceSignalSemanticRole::HandshakeReadyLike),
                    None,
                    None,
                ),
                _ => (Vec::new(), None, None, None, None),
            }
        }
    }
}

fn build_semantic_candidates(
    semantic_observations: &[InterfaceSignalSemanticObservationRecord],
    prior_guidance: Option<&SemanticPriorGuidance>,
) -> Vec<InterfaceSignalSemanticCandidateRecord> {
    let mut candidates: Vec<InterfaceSignalSemanticCandidateRecord> = [
        InterfaceSignalSemanticRole::HandshakeValidLike,
        InterfaceSignalSemanticRole::HandshakeReadyLike,
    ]
    .into_iter()
    .filter_map(|role| {
        let supporting_observations: Vec<&InterfaceSignalSemanticObservationRecord> =
            semantic_observations
                .iter()
                .filter(|observation| observation_supports_semantic_role(observation, role))
                .collect();
        if supporting_observations.is_empty() {
            None
        } else {
            Some(build_semantic_candidate(
                role,
                &supporting_observations,
                prior_guidance,
            ))
        }
    })
    .collect();
    candidates.sort_by(|left, right| {
        right
            .arbitration_weight
            .cmp(&left.arbitration_weight)
            .then_with(|| {
                right
                    .prior_reliability_adjustment
                    .cmp(&left.prior_reliability_adjustment)
            })
            .then_with(|| right.evidence_weight.cmp(&left.evidence_weight))
            .then_with(|| {
                right
                    .supporting_observation_count
                    .cmp(&left.supporting_observation_count)
            })
            .then_with(|| {
                automation_confidence_rank(right.automation_confidence)
                    .cmp(&automation_confidence_rank(left.automation_confidence))
            })
            .then_with(|| left.role.as_str().cmp(right.role.as_str()))
    });
    candidates
}

fn observation_supports_semantic_role(
    observation: &InterfaceSignalSemanticObservationRecord,
    role: InterfaceSignalSemanticRole,
) -> bool {
    semantic_tags_support_semantic_role(&observation.semantic_tags, role)
}

fn semantic_tags_support_semantic_role(
    semantic_tags: &[SignalSemanticTag],
    role: InterfaceSignalSemanticRole,
) -> bool {
    semantic_tags.iter().any(|tag| match role {
        InterfaceSignalSemanticRole::HandshakeValidLike => {
            matches!(tag, SignalSemanticTag::HandshakeValidLike)
        }
        InterfaceSignalSemanticRole::HandshakeReadyLike => {
            matches!(tag, SignalSemanticTag::HandshakeReadyLike)
        }
    })
}

fn grounding_strength_for_supporting_observations(
    observations: &[&InterfaceSignalSemanticObservationRecord],
) -> SemanticGroundingStrength {
    if observations.len() <= 1 {
        SemanticGroundingStrength::SingleSource
    } else if observations
        .iter()
        .map(|observation| semantic_observation_modality(observation.source_kind))
        .collect::<BTreeSet<_>>()
        .len()
        > 1
    {
        SemanticGroundingStrength::CrossModality
    } else {
        SemanticGroundingStrength::MultiSource
    }
}

fn build_semantic_candidate(
    role: InterfaceSignalSemanticRole,
    observations: &[&InterfaceSignalSemanticObservationRecord],
    prior_guidance: Option<&SemanticPriorGuidance>,
) -> InterfaceSignalSemanticCandidateRecord {
    let grounding_strength = grounding_strength_for_supporting_observations(observations);
    let supporting_source_kinds = observations
        .iter()
        .map(|observation| observation.source_kind)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let automation_confidence =
        observations
            .iter()
            .fold(AutomationConfidence::Low, |current, observation| {
                max_automation_confidence(current, observation.automation_confidence)
            });
    let evidence_weight: u32 = observations
        .iter()
        .map(|observation| semantic_observation_weight(observation))
        .sum();
    let prior_reliability_adjustment =
        semantic_prior_reliability_adjustment(role, observations, prior_guidance);
    let arbitration_weight = evidence_weight.saturating_add(prior_reliability_adjustment);
    InterfaceSignalSemanticCandidateRecord {
        role,
        grounding_strength,
        supporting_source_kinds,
        supporting_observation_count: observations.len(),
        automation_confidence,
        evidence_weight,
        prior_reliability_adjustment,
        arbitration_weight,
        alias_dependent: semantic_observations_are_alias_dependent(observations),
    }
}

fn semantic_prior_reliability_adjustment(
    role: InterfaceSignalSemanticRole,
    observations: &[&InterfaceSignalSemanticObservationRecord],
    prior_guidance: Option<&SemanticPriorGuidance>,
) -> u32 {
    let Some(prior_guidance) = prior_guidance else {
        return 0;
    };

    observations
        .iter()
        .map(|observation| observation.source_kind)
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .map(|source_kind| {
            prior_guidance
                .corpus_memory
                .semantic_modality_reliability_bonus(
                    Some(prior_guidance.protocol_family),
                    role,
                    source_kind,
                )
        })
        .sum()
}

fn build_semantic_consensus(
    candidate: &InterfaceSignalSemanticCandidateRecord,
) -> InterfaceSignalSemanticConsensusRecord {
    InterfaceSignalSemanticConsensusRecord {
        role: candidate.role,
        grounding_strength: candidate.grounding_strength,
        supporting_source_kinds: candidate.supporting_source_kinds.clone(),
        supporting_observation_count: candidate.supporting_observation_count,
        automation_confidence: candidate.automation_confidence,
        prior_reliability_adjustment: candidate.prior_reliability_adjustment,
        prior_guided: candidate.prior_reliability_adjustment > 0,
        alias_dependent: candidate.alias_dependent,
    }
}

fn build_semantic_arbitration(
    candidates: &[InterfaceSignalSemanticCandidateRecord],
) -> Option<InterfaceSignalSemanticArbitrationRecord> {
    let leading_candidate = candidates.first()?;
    let runner_up_candidate = candidates.get(1);
    let margin_over_runner_up = runner_up_candidate.map(|candidate| {
        leading_candidate
            .arbitration_weight
            .saturating_sub(candidate.arbitration_weight)
    });
    let prior_guided_margin = runner_up_candidate.is_some_and(|runner_up| {
        leading_candidate.prior_reliability_adjustment > runner_up.prior_reliability_adjustment
            && leading_candidate.prior_reliability_adjustment >= 2
            && margin_over_runner_up.unwrap_or_default() >= 2
            && !leading_candidate.alias_dependent
            && !matches!(
                leading_candidate.automation_confidence,
                AutomationConfidence::Low
            )
    });
    let (decisive, decision_basis) = if candidates.len() == 1 {
        (true, SemanticArbitrationDecisionBasis::SingleCandidate)
    } else if prior_guided_margin {
        (true, SemanticArbitrationDecisionBasis::PriorGuidedMargin)
    } else {
        (false, SemanticArbitrationDecisionBasis::Contested)
    };
    Some(InterfaceSignalSemanticArbitrationRecord {
        candidate_count: candidates.len(),
        leading_role: leading_candidate.role,
        leading_evidence_weight: leading_candidate.evidence_weight,
        leading_prior_reliability_adjustment: leading_candidate.prior_reliability_adjustment,
        leading_arbitration_weight: leading_candidate.arbitration_weight,
        runner_up_role: runner_up_candidate.map(|candidate| candidate.role),
        runner_up_evidence_weight: runner_up_candidate.map(|candidate| candidate.evidence_weight),
        runner_up_prior_reliability_adjustment: runner_up_candidate
            .map(|candidate| candidate.prior_reliability_adjustment),
        runner_up_arbitration_weight: runner_up_candidate
            .map(|candidate| candidate.arbitration_weight),
        margin_over_runner_up,
        decisive,
        decision_basis,
    })
}

fn semantic_observation_weight(observation: &InterfaceSignalSemanticObservationRecord) -> u32 {
    semantic_source_kind_weight(observation.source_kind)
        + automation_confidence_weight(observation.automation_confidence)
}

fn semantic_source_kind_weight(source_kind: SignalSemanticHintSourceKind) -> u32 {
    match source_kind {
        SignalSemanticHintSourceKind::SignalDescriptionTable => 4,
        SignalSemanticHintSourceKind::VisualCaption => 3,
        SignalSemanticHintSourceKind::ProseStatement => 2,
        SignalSemanticHintSourceKind::VlmTimingDiagramAnnotation => 2,
        SignalSemanticHintSourceKind::AliasGroundedProseStatement => 1,
    }
}

fn automation_confidence_weight(confidence: AutomationConfidence) -> u32 {
    match confidence {
        AutomationConfidence::High => 3,
        AutomationConfidence::Medium => 2,
        AutomationConfidence::Low => 1,
    }
}

fn semantic_observation_modality(
    source_kind: SignalSemanticHintSourceKind,
) -> SemanticObservationModality {
    match source_kind {
        SignalSemanticHintSourceKind::SignalDescriptionTable => SemanticObservationModality::Table,
        SignalSemanticHintSourceKind::ProseStatement
        | SignalSemanticHintSourceKind::AliasGroundedProseStatement => {
            SemanticObservationModality::Prose
        }
        SignalSemanticHintSourceKind::VisualCaption
        | SignalSemanticHintSourceKind::VlmTimingDiagramAnnotation => {
            SemanticObservationModality::Visual
        }
    }
}

fn semantic_observations_are_alias_dependent(
    observations: &[&InterfaceSignalSemanticObservationRecord],
) -> bool {
    !observations.is_empty()
        && observations.iter().all(|observation| {
            matches!(
                observation.source_kind,
                SignalSemanticHintSourceKind::AliasGroundedProseStatement
            )
        })
}

fn handshake_role_context(interfaces: &[InterfaceRecord]) -> HandshakeRoleContext {
    let mut context = HandshakeRoleContext::default();
    for signal in interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
    {
        if handshake_name_fallback_is_blocked(signal) {
            context
                .heuristic_blocked_signals
                .insert(signal.signal_name.clone());
        }
        if let Some(role) = classify_handshake_signal_from_interface_signal(signal) {
            context
                .resolved_roles_by_signal
                .insert(signal.signal_name.clone(), role);
        }
    }
    context
}

fn handshake_name_fallback_is_blocked(signal: &InterfaceSignalRecord) -> bool {
    signal
        .semantic_arbitration
        .as_ref()
        .is_some_and(|arbitration| !arbitration.decisive)
        || (signal.resolved_semantic_role.is_some() && signal.semantic_consensus.is_none())
}

fn handshake_name_fallback_blocked_signal_names(interfaces: &[InterfaceRecord]) -> Vec<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            handshake_name_fallback_is_blocked(signal)
                && classify_handshake_signal_from_name(&signal.signal_name).is_some()
        })
        .map(|signal| signal.signal_name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn resolved_semantic_roles_without_consensus_signal_names(
    interfaces: &[InterfaceRecord],
) -> Vec<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal.resolved_semantic_role.is_some() && signal.semantic_consensus.is_none()
        })
        .map(|signal| signal.signal_name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn alias_dependent_semantic_consensus_signal_names(
    interfaces: &[InterfaceRecord],
) -> BTreeSet<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal
                .semantic_consensus
                .as_ref()
                .is_some_and(|consensus| consensus.alias_dependent)
        })
        .map(|signal| signal.signal_name.clone())
        .collect()
}

fn alias_dependent_handshake_completion_signal_names(
    temporal_rules: &[TemporalRuleRecord],
    interfaces: &[InterfaceRecord],
) -> Vec<String> {
    let alias_dependent_signals = alias_dependent_semantic_consensus_signal_names(interfaces);
    if alias_dependent_signals.is_empty() {
        return Vec::new();
    }

    temporal_rules
        .iter()
        .flat_map(|rule| rule.antecedents.iter().chain(rule.consequents.iter()))
        .filter_map(|predicate| match predicate {
            TemporalPredicateRecord::HandshakeComplete {
                valid_signal,
                ready_signal,
                ..
            } => Some([valid_signal, ready_signal]),
            _ => None,
        })
        .flat_map(|signals| signals.into_iter())
        .filter(|signal_name| alias_dependent_signals.contains(*signal_name))
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn classify_handshake_signal_from_interface_signal(
    signal: &InterfaceSignalRecord,
) -> Option<HandshakeSignalRole> {
    signal.semantic_consensus.as_ref().and_then(|_| {
        signal
            .resolved_semantic_role
            .map(handshake_role_from_resolved_semantic_role)
    })
}

fn handshake_role_from_resolved_semantic_role(
    role: InterfaceSignalSemanticRole,
) -> HandshakeSignalRole {
    match role {
        InterfaceSignalSemanticRole::HandshakeValidLike => HandshakeSignalRole::Valid,
        InterfaceSignalSemanticRole::HandshakeReadyLike => HandshakeSignalRole::Ready,
    }
}

fn classify_handshake_signal(
    signal_name: &str,
    handshake_role_context: &HandshakeRoleContext,
) -> Option<HandshakeSignalRole> {
    if let Some(role) = handshake_role_context
        .resolved_roles_by_signal
        .get(signal_name)
        .copied()
    {
        return Some(role);
    }
    if handshake_role_context
        .heuristic_blocked_signals
        .contains(signal_name)
    {
        return None;
    }
    classify_handshake_signal_from_name(signal_name)
}

fn classify_handshake_signal_from_name(signal_name: &str) -> Option<HandshakeSignalRole> {
    let normalized = signal_name.to_ascii_lowercase();
    if normalized.contains("valid") {
        Some(HandshakeSignalRole::Valid)
    } else if normalized.contains("ready") {
        Some(HandshakeSignalRole::Ready)
    } else {
        None
    }
}

fn is_handshake_asserted_value(value: &str) -> bool {
    matches!(
        value.to_ascii_uppercase().as_str(),
        "HIGH" | "ASSERTED" | "1" | "TRUE"
    )
}

fn temporal_consequents_from_signal_constraint(
    constraint: &SignalConstraintRecord,
    unique_producer_by_signal: &BTreeMap<String, String>,
) -> Vec<TemporalPredicateRecord> {
    let actor_drive_predicate = unique_producer_by_signal
        .get(&constraint.subject_signal)
        .map(|actor_name| TemporalPredicateRecord::ActorDrivesSignal {
            actor_name: actor_name.clone(),
            signal_name: constraint.subject_signal.clone(),
            phase: TickPhase::PostTick,
        });
    let actor_maintains_predicate = unique_producer_by_signal
        .get(&constraint.subject_signal)
        .map(
            |actor_name| TemporalPredicateRecord::ActorMaintainsSignalStable {
                actor_name: actor_name.clone(),
                signal_name: constraint.subject_signal.clone(),
                from_phase: TickPhase::PreTick,
                to_phase: TickPhase::PostTick,
            },
        );
    match &constraint.constraint_kind {
        SignalConstraintKind::MustNotChange
        | SignalConstraintKind::MustBeStable
        | SignalConstraintKind::MustHoldData => actor_maintains_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalStable {
                signal_name: constraint.subject_signal.clone(),
                from_phase: TickPhase::PreTick,
                to_phase: TickPhase::PostTick,
            }))
            .collect(),
        SignalConstraintKind::MustBeHigh => actor_drive_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                signal_name: constraint.subject_signal.clone(),
                value: "HIGH".to_string(),
                phase: TickPhase::PostTick,
            }))
            .collect(),
        SignalConstraintKind::MustBeLow => actor_drive_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                signal_name: constraint.subject_signal.clone(),
                value: "LOW".to_string(),
                phase: TickPhase::PostTick,
            }))
            .collect(),
        SignalConstraintKind::MustBeAsserted => actor_drive_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                signal_name: constraint.subject_signal.clone(),
                value: "ASSERTED".to_string(),
                phase: TickPhase::PostTick,
            }))
            .collect(),
        SignalConstraintKind::MustBeDeasserted => actor_drive_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                signal_name: constraint.subject_signal.clone(),
                value: "DEASSERTED".to_string(),
                phase: TickPhase::PostTick,
            }))
            .collect(),
        SignalConstraintKind::MustBeValue { value } => actor_drive_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                signal_name: constraint.subject_signal.clone(),
                value: constraint
                    .target_value
                    .clone()
                    .unwrap_or_else(|| value.clone()),
                phase: TickPhase::PostTick,
            }))
            .collect(),
    }
}

fn temporal_consequents_from_conditional_rule(
    rule: &ConditionalRuleRecord,
    known_signals: &BTreeSet<String>,
    unique_producer_by_signal: &BTreeMap<String, String>,
    handshake_role_context: &HandshakeRoleContext,
) -> Vec<TemporalPredicateRecord> {
    let Some(signal_name) = rule.consequent_signal.clone() else {
        return Vec::new();
    };
    let actor_drive_predicate = unique_producer_by_signal
        .get(&signal_name)
        .map(|actor_name| TemporalPredicateRecord::ActorDrivesSignal {
            actor_name: actor_name.clone(),
            signal_name: signal_name.clone(),
            phase: TickPhase::PostTick,
        });
    let actor_maintains_predicate = unique_producer_by_signal
        .get(&signal_name)
        .map(
            |actor_name| TemporalPredicateRecord::ActorMaintainsSignalStable {
                actor_name: actor_name.clone(),
                signal_name: signal_name.clone(),
                from_phase: TickPhase::PreTick,
                to_phase: TickPhase::PostTick,
            },
        );
    let action = rule.consequent_action.trim();
    let action_lower = action.to_ascii_lowercase();
    if action_lower.contains("not change")
        || action_lower.contains("be stable")
        || action_lower.contains("remain stable")
        || action_lower.contains("hold")
    {
        return actor_maintains_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalStable {
                signal_name,
                from_phase: TickPhase::PreTick,
                to_phase: TickPhase::PostTick,
            }))
            .collect();
    }
    if action_lower.contains("deasserted") {
        return actor_drive_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                signal_name,
                value: "DEASSERTED".to_string(),
                phase: TickPhase::PostTick,
            }))
            .collect();
    }
    if action_lower.contains("asserted") {
        return actor_drive_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                signal_name,
                value: "ASSERTED".to_string(),
                phase: TickPhase::PostTick,
            }))
            .collect();
    }
    if action_lower.contains(" low") || action_lower == "low" {
        return actor_drive_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                signal_name,
                value: "LOW".to_string(),
                phase: TickPhase::PostTick,
            }))
            .collect();
    }
    if action_lower.contains(" high") || action_lower == "high" {
        return actor_drive_predicate
            .into_iter()
            .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                signal_name,
                value: "HIGH".to_string(),
                phase: TickPhase::PostTick,
            }))
            .collect();
    }

    extract_symbolic_value(action, Some(&signal_name))
        .map(|value| {
            actor_drive_predicate
                .clone()
                .into_iter()
                .chain(std::iter::once(TemporalPredicateRecord::SignalValue {
                    signal_name: signal_name.clone(),
                    value,
                    phase: TickPhase::PostTick,
                }))
                .collect()
        })
        .unwrap_or_else(|| {
            parse_temporal_condition_predicates(
                action,
                known_signals,
                TickPhase::PostTick,
                handshake_role_context,
            )
            .into_iter()
            .map(|predicate| match predicate {
                TemporalPredicateRecord::SignalValue { value, .. } => {
                    TemporalPredicateRecord::SignalValue {
                        signal_name: signal_name.clone(),
                        value,
                        phase: TickPhase::PostTick,
                    }
                }
                TemporalPredicateRecord::HandshakeComplete {
                    valid_signal,
                    ready_signal,
                    ..
                } => TemporalPredicateRecord::HandshakeComplete {
                    valid_signal,
                    ready_signal,
                    phase: TickPhase::PostTick,
                },
                other => other,
            })
            .collect()
        })
}

fn temporal_predicates_are_actor_grounded(
    antecedents: &[TemporalPredicateRecord],
    consequents: &[TemporalPredicateRecord],
) -> bool {
    antecedents
        .iter()
        .chain(consequents.iter())
        .any(|predicate| {
            matches!(
                predicate,
                TemporalPredicateRecord::ActorDrivesSignal { .. }
                    | TemporalPredicateRecord::ActorMaintainsSignalStable { .. }
                    | TemporalPredicateRecord::ActorSamplesSignal { .. }
            )
        })
}

fn temporal_predicates_have_handshake_completion(
    antecedents: &[TemporalPredicateRecord],
    consequents: &[TemporalPredicateRecord],
) -> bool {
    antecedents
        .iter()
        .chain(consequents.iter())
        .any(|predicate| matches!(predicate, TemporalPredicateRecord::HandshakeComplete { .. }))
}

fn temporal_rule_from_timing_constraint(
    timing: &TimingConstraintRecord,
    description: &str,
    known_signals: &BTreeSet<String>,
    default_clock: Option<&str>,
    actor_names: &BTreeSet<String>,
    prior_guidance: Option<&SemanticPriorGuidance>,
) -> Option<TemporalRuleRecord> {
    let signal_name = find_known_signal_name(description, known_signals)?;
    let description_lower = description.to_ascii_lowercase();
    let explicit_clock_signal = explicit_clock_signal_from_text(description, known_signals);
    let default_edge = if default_clock.is_some() {
        ClockEdge::Rising
    } else {
        ClockEdge::Unknown
    };
    let edge = grounded_temporal_edge(description, explicit_clock_signal.as_deref(), default_edge);
    if !description_lower.contains("sampled") && !description_lower.contains("captured") {
        return None;
    }

    let actor_name = extract_actor_after_by(description);
    let consequent = actor_name
        .map(|actor_name| TemporalPredicateRecord::ActorSamplesSignal {
            actor_name,
            signal_name: signal_name.clone(),
            phase: TickPhase::PreTick,
        })
        .unwrap_or_else(|| TemporalPredicateRecord::SignalSampled {
            signal_name: signal_name.clone(),
            phase: TickPhase::PreTick,
        });
    let consequents = vec![consequent];
    let actor_grounded = temporal_predicates_are_actor_grounded(&[], &consequents);
    let handshake_completion = temporal_predicates_have_handshake_completion(&[], &consequents);

    Some(TemporalRuleRecord {
        rule_id: format!("temporal_timing_{}", timing.constraint_id),
        clock_signal: explicit_clock_signal.or_else(|| default_clock.map(str::to_string)),
        edge,
        antecedents: Vec::new(),
        consequents,
        cycle_window: extract_cycle_window_from_timing_constraint(
            timing,
            description,
            known_signals,
            actor_names,
            actor_grounded,
            handshake_completion,
            prior_guidance,
        ),
        source_text: description.to_string(),
        supporting_statement_ids: timing.supporting_statement_ids.clone(),
        automation_confidence: timing.automation_confidence,
    })
}

fn extract_cycle_window_from_timing_constraint(
    timing: &TimingConstraintRecord,
    description: &str,
    known_signals: &BTreeSet<String>,
    actor_names: &BTreeSet<String>,
    actor_grounded: bool,
    handshake_completion: bool,
    prior_guidance: Option<&SemanticPriorGuidance>,
) -> Option<CycleWindowRecord> {
    extract_cycle_window_from_text_with_known_signals(description, known_signals)
        .or_else(|| {
            let unit_mentions_cycle_like_units = timing
                .unit
                .as_deref()
                .map(unit_mentions_cycle_like_unit)
                .unwrap_or(false);
            if !unit_mentions_cycle_like_units {
                return None;
            }

            let min_cycles = timing
                .min_value
                .as_deref()
                .and_then(parse_cycle_count_value);
            let max_cycles = timing
                .max_value
                .as_deref()
                .and_then(parse_cycle_count_value);
            let typ_cycles = timing
                .typ_value
                .as_deref()
                .and_then(parse_cycle_count_value);

            if min_cycles.is_none() && max_cycles.is_none() && typ_cycles.is_none() {
                return None;
            }

            Some(CycleWindowRecord {
                min_cycles: min_cycles.or(typ_cycles),
                max_cycles: max_cycles.or(typ_cycles),
            })
        })
        .or_else(|| {
            prior_guidance.and_then(|guidance| {
                guidance.corpus_memory.temporal_cycle_window_in_text(
                    Some(guidance.protocol_family),
                    description,
                    known_signals,
                    actor_names,
                    actor_grounded.then_some(true),
                    handshake_completion.then_some(true),
                )
            })
        })
}

fn explicit_clock_edge_from_text(text: &str) -> Option<ClockEdge> {
    let lowered = text.to_ascii_lowercase();
    if contains_phrase(&lowered, "falling edge")
        || contains_phrase(&lowered, "falling edges")
        || contains_phrase(&lowered, "negedge")
        || contains_phrase(&lowered, "negedges")
    {
        Some(ClockEdge::Falling)
    } else if contains_phrase(&lowered, "rising edge")
        || contains_phrase(&lowered, "rising edges")
        || contains_phrase(&lowered, "posedge")
        || contains_phrase(&lowered, "posedges")
    {
        Some(ClockEdge::Rising)
    } else {
        None
    }
}

fn grounded_temporal_edge(
    text: &str,
    explicit_clock_signal: Option<&str>,
    default_edge: ClockEdge,
) -> ClockEdge {
    explicit_clock_edge_from_text(text).unwrap_or_else(|| {
        if explicit_clock_signal.is_some() {
            ClockEdge::Rising
        } else {
            default_edge
        }
    })
}

fn explicit_clock_signal_from_text(text: &str, known_signals: &BTreeSet<String>) -> Option<String> {
    let lowered = text.to_ascii_lowercase();
    let tokens = lowered
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    let known_signal_tokens = known_signals
        .iter()
        .map(|signal| signal.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();

    for index in 0..tokens.len() {
        if let Some((_, signal_token)) =
            parse_named_cycle_like_diagram_position(&tokens, index, &known_signal_tokens).or_else(
                || parse_named_generic_edge_diagram_position(&tokens, index, &known_signal_tokens),
            )
            && let Some(signal_name) = known_signals
                .iter()
                .find(|signal| signal.eq_ignore_ascii_case(signal_token))
        {
            return Some(signal_name.clone());
        }
    }

    for signal in known_signals {
        let signal_lower = signal.to_ascii_lowercase();
        let explicit_patterns = [
            format!("posedge {signal_lower}"),
            format!("posedge of {signal_lower}"),
            format!("posedge of the {signal_lower}"),
            format!("posedges of {signal_lower}"),
            format!("posedges of the {signal_lower}"),
            format!("negedge {signal_lower}"),
            format!("negedge of {signal_lower}"),
            format!("negedge of the {signal_lower}"),
            format!("negedges of {signal_lower}"),
            format!("negedges of the {signal_lower}"),
            format!("{signal_lower} posedge"),
            format!("{signal_lower} negedge"),
            format!("rising edge of {signal_lower}"),
            format!("rising edge of the {signal_lower}"),
            format!("rising edges of {signal_lower}"),
            format!("rising edges of the {signal_lower}"),
            format!("rising edge {signal_lower}"),
            format!("falling edge of {signal_lower}"),
            format!("falling edge of the {signal_lower}"),
            format!("falling edges of {signal_lower}"),
            format!("falling edges of the {signal_lower}"),
            format!("falling edge {signal_lower}"),
            format!("{signal_lower} rising edge"),
            format!("{signal_lower} falling edge"),
            format!("edge of {signal_lower}"),
            format!("edges of {signal_lower}"),
            format!("edge of the {signal_lower}"),
            format!("edges of the {signal_lower}"),
            format!("{signal_lower} edge"),
            format!("{signal_lower} edges"),
            format!("clock edge of {signal_lower}"),
            format!("clock edges of {signal_lower}"),
            format!("clock edge of the {signal_lower}"),
            format!("clock edges of the {signal_lower}"),
            format!("{signal_lower} clock edge"),
            format!("{signal_lower} clock edges"),
            format!("{signal_lower} cycle"),
            format!("{signal_lower} cycles"),
            format!("{signal_lower} tick"),
            format!("{signal_lower} ticks"),
        ];

        if explicit_patterns
            .iter()
            .any(|pattern| contains_phrase(&lowered, pattern))
        {
            return Some(signal.clone());
        }
    }

    None
}

fn unit_mentions_cycle_like_unit(unit: &str) -> bool {
    let normalized = unit.to_ascii_lowercase();
    let tokens = normalized
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    contains_cycle_like_unit(&tokens)
}

fn dedup_temporal_rules(rules: Vec<TemporalRuleRecord>) -> Vec<TemporalRuleRecord> {
    let mut seen = BTreeSet::new();
    let mut deduped = Vec::new();
    for rule in rules {
        let key = serde_json::to_string(&(
            &rule.clock_signal,
            rule.edge,
            &rule.antecedents,
            &rule.consequents,
            &rule.cycle_window,
            &rule.source_text,
        ))
        .unwrap_or_else(|_| rule.rule_id.clone());
        if seen.insert(key) {
            deduped.push(rule);
        }
    }
    deduped
}

fn build_temporal_conflicts(
    temporal_rules: &[TemporalRuleRecord],
    signal_polarities: &[SignalPolarityRecord],
    system_contract: Option<&SystemContractRecord>,
) -> Vec<TemporalConflictRecord> {
    let mut accumulators = BTreeMap::<String, TemporalConflictAccumulator>::new();
    let signal_polarity_by_signal =
        build_signal_polarity_lookup(signal_polarities, system_contract);

    for rule in temporal_rules {
        for predicate in &rule.consequents {
            let TemporalPredicateRecord::SignalValue {
                signal_name,
                value,
                phase,
            } = predicate
            else {
                continue;
            };

            let key = temporal_conflict_group_key(rule, signal_name, *phase);
            let entry = accumulators
                .entry(key)
                .or_insert_with(|| TemporalConflictAccumulator {
                    clock_signal: rule.clock_signal.clone(),
                    edge: rule.edge,
                    antecedents: rule.antecedents.clone(),
                    cycle_window: rule.cycle_window.clone(),
                    signal_name: signal_name.clone(),
                    phase: *phase,
                    observed_values: BTreeMap::new(),
                    automation_confidence: rule.automation_confidence,
                });
            let normalized_value =
                normalize_temporal_conflict_value(value, signal_name, &signal_polarity_by_signal);
            let support = entry.observed_values.entry(normalized_value).or_default();
            support.supporting_rule_ids.insert(rule.rule_id.clone());
            support
                .supporting_statement_ids
                .extend(rule.supporting_statement_ids.iter().cloned());
            entry.automation_confidence =
                min_automation_confidence(entry.automation_confidence, rule.automation_confidence);
        }
    }

    accumulators
        .into_values()
        .flat_map(temporal_conflict_records_from_accumulator)
        .enumerate()
        .map(|(index, emission)| TemporalConflictRecord {
            conflict_id: format!("temporal_conflict_{:04}", index + 1),
            clock_signal: emission.clock_signal,
            edge: emission.edge,
            antecedents: emission.antecedents,
            cycle_window: emission.cycle_window,
            signal_name: emission.signal_name,
            phase: emission.phase,
            conflicting_values: emission.conflicting_values.into_iter().collect(),
            supporting_rule_ids: emission.supporting_rule_ids.into_iter().collect(),
            supporting_statement_ids: emission.supporting_statement_ids.into_iter().collect(),
            automation_confidence: emission.automation_confidence,
        })
        .collect()
}

fn temporal_conflict_group_key(
    rule: &TemporalRuleRecord,
    signal_name: &str,
    phase: TickPhase,
) -> String {
    let mut antecedent_signatures = rule
        .antecedents
        .iter()
        .map(|predicate| serde_json::to_string(predicate).unwrap_or_default())
        .collect::<Vec<_>>();
    antecedent_signatures.sort();

    serde_json::to_string(&(
        &rule.clock_signal,
        rule.edge,
        antecedent_signatures,
        &rule.cycle_window,
        signal_name,
        phase,
    ))
    .unwrap_or_else(|_| {
        format!(
            "{:?}:{:?}:{signal_name}:{phase:?}",
            rule.clock_signal, rule.edge
        )
    })
}

fn build_signal_polarity_lookup(
    signal_polarities: &[SignalPolarityRecord],
    system_contract: Option<&SystemContractRecord>,
) -> HashMap<String, SignalPolarity> {
    let mut polarity_by_signal = signal_polarities
        .iter()
        .map(|record| (record.signal_name.clone(), record.polarity))
        .collect::<HashMap<_, _>>();

    if let Some(system_contract) = system_contract {
        polarity_by_signal
            .entry(system_contract.reset_signal.clone())
            .or_insert_with(|| match system_contract.reset_polarity {
                SystemResetPolarity::ActiveHigh => SignalPolarity::ActiveHigh,
                SystemResetPolarity::ActiveLow => SignalPolarity::ActiveLow,
            });
    }

    polarity_by_signal
}

fn normalize_temporal_conflict_value(
    value: &str,
    signal_name: &str,
    signal_polarity_by_signal: &HashMap<String, SignalPolarity>,
) -> TemporalConflictComparableValue {
    match value.trim().to_ascii_uppercase().as_str() {
        "HIGH" | "1" | "TRUE" => TemporalConflictComparableValue::Level("HIGH".to_string()),
        "LOW" | "0" | "FALSE" => TemporalConflictComparableValue::Level("LOW".to_string()),
        "ASSERTED" => match signal_polarity_by_signal.get(signal_name) {
            Some(SignalPolarity::ActiveHigh) => {
                TemporalConflictComparableValue::Level("HIGH".to_string())
            }
            Some(SignalPolarity::ActiveLow) => {
                TemporalConflictComparableValue::Level("LOW".to_string())
            }
            None => TemporalConflictComparableValue::Assertion("ASSERTED".to_string()),
        },
        "DEASSERTED" => match signal_polarity_by_signal.get(signal_name) {
            Some(SignalPolarity::ActiveHigh) => {
                TemporalConflictComparableValue::Level("LOW".to_string())
            }
            Some(SignalPolarity::ActiveLow) => {
                TemporalConflictComparableValue::Level("HIGH".to_string())
            }
            None => TemporalConflictComparableValue::Assertion("DEASSERTED".to_string()),
        },
        other => TemporalConflictComparableValue::Symbolic(other.to_string()),
    }
}

fn temporal_conflict_records_from_accumulator(
    entry: TemporalConflictAccumulator,
) -> Vec<TemporalConflictEmission> {
    let mut emitted = Vec::new();
    for domain in [
        TemporalConflictValueDomain::Level,
        TemporalConflictValueDomain::Assertion,
        TemporalConflictValueDomain::Symbolic,
    ] {
        let matching_values = entry
            .observed_values
            .iter()
            .filter(|(value, _)| value.domain() == domain)
            .collect::<Vec<_>>();
        if matching_values.len() <= 1 {
            continue;
        }

        let conflicting_values = matching_values
            .iter()
            .map(|(value, _)| value.display_value())
            .collect::<BTreeSet<_>>();
        let supporting_rule_ids = matching_values
            .iter()
            .flat_map(|(_, support)| support.supporting_rule_ids.iter().cloned())
            .collect::<BTreeSet<_>>();
        let supporting_statement_ids = matching_values
            .iter()
            .flat_map(|(_, support)| support.supporting_statement_ids.iter().cloned())
            .collect::<BTreeSet<_>>();

        emitted.push(TemporalConflictEmission {
            clock_signal: entry.clock_signal.clone(),
            edge: entry.edge,
            antecedents: entry.antecedents.clone(),
            cycle_window: entry.cycle_window.clone(),
            signal_name: entry.signal_name.clone(),
            phase: entry.phase,
            conflicting_values,
            supporting_rule_ids,
            supporting_statement_ids,
            automation_confidence: entry.automation_confidence,
        });
    }

    emitted
}

fn unique_producer_by_signal(
    signal_connectivity: &[SignalConnectivityRecord],
) -> BTreeMap<String, String> {
    signal_connectivity
        .iter()
        .filter_map(|record| {
            if record.producer_actor_names.len() == 1 {
                Some((
                    record.signal_name.clone(),
                    record.producer_actor_names[0].clone(),
                ))
            } else {
                None
            }
        })
        .collect()
}

fn extract_cycle_window_from_text(text: &str) -> Option<CycleWindowRecord> {
    let normalized = text.to_ascii_lowercase();
    let tokens = normalized
        .split(|ch: char| !ch.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();

    if tokens.is_empty() {
        return None;
    }

    for index in 0..tokens.len() {
        if let Some(unit_token_len) = diagram_position_cycle_like_unit_len(&tokens, index)
            && let Some(count) = tokens
                .get(index + unit_token_len)
                .copied()
                .and_then(parse_diagram_cycle_count_value)
        {
            return Some(CycleWindowRecord {
                min_cycles: Some(count),
                max_cycles: Some(count),
            });
        }

        if matches!(tokens[index], "at" | "during" | "on")
            && let Some(count) = tokens
                .get(index + 1)
                .copied()
                .and_then(parse_diagram_cycle_count_value)
        {
            return Some(CycleWindowRecord {
                min_cycles: Some(count),
                max_cycles: Some(count),
            });
        }

        if matches!(tokens[index], "at" | "during" | "on")
            && let Some(count) = parse_explicit_cycle_position_count(&tokens, index + 1)
        {
            return Some(CycleWindowRecord {
                min_cycles: Some(count),
                max_cycles: Some(count),
            });
        }
    }

    for index in 0..tokens.len() {
        if tokens[index] != "between" {
            continue;
        }
        let Some(min_cycles) = tokens
            .get(index + 1)
            .copied()
            .and_then(parse_cycle_count_value)
        else {
            continue;
        };
        let and_index = if tokens.get(index + 2) == Some(&"and") {
            index + 2
        } else {
            continue;
        };
        let Some(max_cycles) = tokens
            .get(and_index + 1)
            .copied()
            .and_then(parse_cycle_count_value)
        else {
            continue;
        };
        if contains_cycle_like_unit(&tokens[and_index + 2..tokens.len().min(and_index + 6)]) {
            return Some(CycleWindowRecord {
                min_cycles: Some(min_cycles),
                max_cycles: Some(max_cycles),
            });
        }
    }

    for index in 0..tokens.len() {
        let Some(count) = tokens
            .get(index + 1)
            .copied()
            .and_then(parse_cycle_count_value)
        else {
            continue;
        };
        if !contains_cycle_like_unit(&tokens[index + 2..tokens.len().min(index + 6)]) {
            continue;
        }

        match tokens[index] {
            "within" => {
                return Some(CycleWindowRecord {
                    min_cycles: None,
                    max_cycles: Some(count),
                });
            }
            "after" => {
                return Some(CycleWindowRecord {
                    min_cycles: Some(count),
                    max_cycles: Some(count),
                });
            }
            "for" => {
                return Some(CycleWindowRecord {
                    min_cycles: Some(count),
                    max_cycles: Some(count),
                });
            }
            _ => {}
        }
    }

    for index in 0..tokens.len().saturating_sub(2) {
        if tokens[index] == "at" && tokens[index + 1] == "least" {
            let Some(count) = parse_cycle_count_value(tokens[index + 2]) else {
                continue;
            };
            if contains_cycle_like_unit(&tokens[index + 3..tokens.len().min(index + 7)]) {
                return Some(CycleWindowRecord {
                    min_cycles: Some(count),
                    max_cycles: None,
                });
            }
        }
        if tokens[index] == "at" && tokens[index + 1] == "most" {
            let Some(count) = parse_cycle_count_value(tokens[index + 2]) else {
                continue;
            };
            if contains_cycle_like_unit(&tokens[index + 3..tokens.len().min(index + 7)]) {
                return Some(CycleWindowRecord {
                    min_cycles: None,
                    max_cycles: Some(count),
                });
            }
        }
        if tokens[index] == "no"
            && tokens[index + 1] == "more"
            && tokens.get(index + 2) == Some(&"than")
        {
            let Some(count) = tokens
                .get(index + 3)
                .copied()
                .and_then(parse_cycle_count_value)
            else {
                continue;
            };
            if contains_cycle_like_unit(&tokens[index + 4..tokens.len().min(index + 8)]) {
                return Some(CycleWindowRecord {
                    min_cycles: None,
                    max_cycles: Some(count),
                });
            }
        }
    }

    for index in 0..tokens.len() {
        let Some(count) = tokens.get(index).copied().and_then(parse_cycle_count_value) else {
            continue;
        };
        let lookahead = &tokens[index + 1..tokens.len().min(index + 5)];
        if contains_cycle_like_unit(lookahead) && lookahead.contains(&"later") {
            return Some(CycleWindowRecord {
                min_cycles: Some(count),
                max_cycles: Some(count),
            });
        }
    }

    for index in 0..tokens.len() {
        if !matches!(tokens[index], "same" | "this" | "current") {
            continue;
        }

        let lookahead = &tokens[index + 1..tokens.len().min(index + 4)];
        if contains_cycle_like_unit(lookahead) {
            return Some(CycleWindowRecord {
                min_cycles: Some(0),
                max_cycles: Some(0),
            });
        }
    }

    let single_cycle_phrases = [
        ["next", "cycle"].as_slice(),
        ["next", "clock", "cycle"].as_slice(),
        ["following", "cycle"].as_slice(),
        ["subsequent", "cycle"].as_slice(),
        ["next", "tick"].as_slice(),
        ["following", "tick"].as_slice(),
        ["subsequent", "tick"].as_slice(),
        ["next", "clock", "edge"].as_slice(),
        ["following", "clock", "edge"].as_slice(),
        ["subsequent", "clock", "edge"].as_slice(),
        ["next", "rising", "edge"].as_slice(),
        ["following", "rising", "edge"].as_slice(),
        ["subsequent", "rising", "edge"].as_slice(),
        ["next", "falling", "edge"].as_slice(),
        ["following", "falling", "edge"].as_slice(),
        ["subsequent", "falling", "edge"].as_slice(),
        ["next", "posedge"].as_slice(),
        ["following", "posedge"].as_slice(),
        ["subsequent", "posedge"].as_slice(),
        ["next", "negedge"].as_slice(),
        ["following", "negedge"].as_slice(),
        ["subsequent", "negedge"].as_slice(),
    ];
    if single_cycle_phrases
        .iter()
        .any(|phrase| contains_token_phrase(&tokens, phrase))
    {
        return Some(CycleWindowRecord {
            min_cycles: Some(1),
            max_cycles: Some(1),
        });
    }

    None
}

fn extract_cycle_window_from_text_with_known_signals(
    text: &str,
    known_signals: &BTreeSet<String>,
) -> Option<CycleWindowRecord> {
    extract_cycle_window_from_text(text).or_else(|| {
        let normalized = text.to_ascii_lowercase();
        let tokens = normalized
            .split(|ch: char| !ch.is_ascii_alphanumeric())
            .filter(|token| !token.is_empty())
            .collect::<Vec<_>>();
        if tokens.len() < 3 {
            return None;
        }

        let known_signal_tokens = known_signals
            .iter()
            .map(|signal| signal.to_ascii_lowercase())
            .collect::<BTreeSet<_>>();

        for index in 0..tokens.len() {
            if let Some((count, _)) =
                parse_named_generic_edge_diagram_position(&tokens, index, &known_signal_tokens)
            {
                return Some(CycleWindowRecord {
                    min_cycles: Some(count),
                    max_cycles: Some(count),
                });
            }
        }

        for index in 0..tokens.len() {
            if matches!(tokens[index], "same" | "this" | "current")
                && (named_cycle_like_unit_len(&tokens, index + 1, &known_signal_tokens).is_some()
                    || edge_of_known_signal_unit_len(&tokens, index + 1, &known_signal_tokens)
                        .is_some())
            {
                return Some(CycleWindowRecord {
                    min_cycles: Some(0),
                    max_cycles: Some(0),
                });
            }

            if !matches!(tokens[index], "next" | "following" | "subsequent") {
                continue;
            }
            if named_cycle_like_unit_len(&tokens, index + 1, &known_signal_tokens).is_some()
                || edge_of_known_signal_unit_len(&tokens, index + 1, &known_signal_tokens).is_some()
            {
                return Some(CycleWindowRecord {
                    min_cycles: Some(1),
                    max_cycles: Some(1),
                });
            }
        }

        for index in 0..tokens.len() {
            if tokens[index] != "between" {
                continue;
            }
            let Some(min_cycles) = tokens
                .get(index + 1)
                .copied()
                .and_then(parse_cycle_count_value)
            else {
                continue;
            };
            let and_index = if tokens.get(index + 2) == Some(&"and") {
                index + 2
            } else {
                continue;
            };
            let Some(max_cycles) = tokens
                .get(and_index + 1)
                .copied()
                .and_then(parse_cycle_count_value)
            else {
                continue;
            };
            if contains_named_generic_edge_unit(
                &tokens[and_index + 2..tokens.len().min(and_index + 8)],
                &known_signal_tokens,
            ) {
                return Some(CycleWindowRecord {
                    min_cycles: Some(min_cycles),
                    max_cycles: Some(max_cycles),
                });
            }
        }

        for index in 0..tokens.len() {
            let Some(count) = tokens
                .get(index + 1)
                .copied()
                .and_then(parse_cycle_count_value)
            else {
                continue;
            };
            if !contains_named_generic_edge_unit(
                &tokens[index + 2..tokens.len().min(index + 8)],
                &known_signal_tokens,
            ) {
                continue;
            }

            match tokens[index] {
                "within" => {
                    return Some(CycleWindowRecord {
                        min_cycles: None,
                        max_cycles: Some(count),
                    });
                }
                "after" | "for" => {
                    return Some(CycleWindowRecord {
                        min_cycles: Some(count),
                        max_cycles: Some(count),
                    });
                }
                _ => {}
            }
        }

        for index in 0..tokens.len().saturating_sub(2) {
            if tokens[index] == "at" && tokens[index + 1] == "least" {
                let Some(count) = parse_cycle_count_value(tokens[index + 2]) else {
                    continue;
                };
                if contains_named_generic_edge_unit(
                    &tokens[index + 3..tokens.len().min(index + 9)],
                    &known_signal_tokens,
                ) {
                    return Some(CycleWindowRecord {
                        min_cycles: Some(count),
                        max_cycles: None,
                    });
                }
            }
            if tokens[index] == "at" && tokens[index + 1] == "most" {
                let Some(count) = parse_cycle_count_value(tokens[index + 2]) else {
                    continue;
                };
                if contains_named_generic_edge_unit(
                    &tokens[index + 3..tokens.len().min(index + 9)],
                    &known_signal_tokens,
                ) {
                    return Some(CycleWindowRecord {
                        min_cycles: None,
                        max_cycles: Some(count),
                    });
                }
            }
            if tokens[index] == "no"
                && tokens[index + 1] == "more"
                && tokens.get(index + 2) == Some(&"than")
            {
                let Some(count) = tokens
                    .get(index + 3)
                    .copied()
                    .and_then(parse_cycle_count_value)
                else {
                    continue;
                };
                if contains_named_generic_edge_unit(
                    &tokens[index + 4..tokens.len().min(index + 10)],
                    &known_signal_tokens,
                ) {
                    return Some(CycleWindowRecord {
                        min_cycles: None,
                        max_cycles: Some(count),
                    });
                }
            }
        }

        for index in 0..tokens.len() {
            if !matches!(tokens[index], "at" | "during" | "on") {
                continue;
            }
            if let Some(count) = parse_explicit_named_generic_edge_position_count(
                &tokens,
                index + 1,
                &known_signal_tokens,
            ) {
                return Some(CycleWindowRecord {
                    min_cycles: Some(count),
                    max_cycles: Some(count),
                });
            }
        }

        None
    })
}

fn named_cycle_like_unit_len(
    tokens: &[&str],
    start_index: usize,
    known_signal_tokens: &BTreeSet<String>,
) -> Option<usize> {
    let signal_token = tokens.get(start_index).copied()?;
    if !known_signal_tokens.contains(signal_token) {
        return None;
    }

    if let Some(unit_len) = diagram_position_cycle_like_unit_len(tokens, start_index + 1) {
        return Some(1 + unit_len);
    }

    if tokens.get(start_index + 1) == Some(&"edge") {
        return Some(2);
    }

    None
}

fn edge_of_known_signal_unit_len(
    tokens: &[&str],
    start_index: usize,
    known_signal_tokens: &BTreeSet<String>,
) -> Option<usize> {
    if !matches!(tokens.get(start_index), Some(&"edge") | Some(&"edges"))
        || tokens.get(start_index + 1) != Some(&"of")
    {
        return None;
    }

    let signal_index = if tokens.get(start_index + 2) == Some(&"the") {
        start_index + 3
    } else {
        start_index + 2
    };
    let signal_token = tokens.get(signal_index).copied()?;
    known_signal_tokens
        .contains(signal_token)
        .then_some(signal_index - start_index + 1)
}

fn contains_named_generic_edge_unit(
    tokens: &[&str],
    known_signal_tokens: &BTreeSet<String>,
) -> bool {
    for index in 0..tokens.len() {
        if named_generic_edge_unit_len(tokens, index, known_signal_tokens).is_some()
            || edge_of_known_signal_unit_len(tokens, index, known_signal_tokens).is_some()
        {
            return true;
        }
    }

    false
}

fn generic_edge_unit_len(tokens: &[&str], start_index: usize) -> Option<usize> {
    match tokens.get(start_index).copied()? {
        "edge" | "edges" => Some(1),
        "clock" if matches!(tokens.get(start_index + 1), Some(&"edge") | Some(&"edges")) => Some(2),
        _ => None,
    }
}

fn named_generic_edge_unit_len(
    tokens: &[&str],
    start_index: usize,
    known_signal_tokens: &BTreeSet<String>,
) -> Option<usize> {
    let signal_token = tokens.get(start_index).copied()?;
    if !known_signal_tokens.contains(signal_token) {
        return None;
    }

    if matches!(tokens.get(start_index + 1), Some(&"edge") | Some(&"edges")) {
        return Some(2);
    }

    None
}

fn parse_named_generic_edge_diagram_position<'a>(
    tokens: &'a [&'a str],
    start_index: usize,
    known_signal_tokens: &BTreeSet<String>,
) -> Option<(u32, &'a str)> {
    let signal_token = tokens.get(start_index).copied()?;
    if known_signal_tokens.contains(signal_token)
        && let Some(unit_len) = generic_edge_unit_len(tokens, start_index + 1)
        && let Some(count) = tokens
            .get(start_index + 1 + unit_len)
            .copied()
            .and_then(parse_diagram_cycle_count_value)
    {
        return Some((count, signal_token));
    }

    let unit_len = generic_edge_unit_len(tokens, start_index)?;
    let count_index = start_index + unit_len;
    let count = tokens
        .get(count_index)
        .copied()
        .and_then(parse_diagram_cycle_count_value)?;
    if tokens.get(count_index + 1) != Some(&"of") {
        return None;
    }
    let signal_index = if tokens.get(count_index + 2) == Some(&"the") {
        count_index + 3
    } else {
        count_index + 2
    };
    let signal_token = tokens.get(signal_index).copied()?;
    known_signal_tokens
        .contains(signal_token)
        .then_some((count, signal_token))
}

fn parse_named_cycle_like_diagram_position<'a>(
    tokens: &'a [&'a str],
    start_index: usize,
    known_signal_tokens: &BTreeSet<String>,
) -> Option<(u32, &'a str)> {
    let unit_len = diagram_position_cycle_like_unit_len(tokens, start_index)?;
    let count_index = start_index + unit_len;
    let count = tokens
        .get(count_index)
        .copied()
        .and_then(parse_diagram_cycle_count_value)?;
    if tokens.get(count_index + 1) != Some(&"of") {
        return None;
    }
    let signal_index = if tokens.get(count_index + 2) == Some(&"the") {
        count_index + 3
    } else {
        count_index + 2
    };
    let signal_token = tokens.get(signal_index).copied()?;
    known_signal_tokens
        .contains(signal_token)
        .then_some((count, signal_token))
}

fn parse_explicit_named_generic_edge_position_count(
    tokens: &[&str],
    start_index: usize,
    known_signal_tokens: &BTreeSet<String>,
) -> Option<u32> {
    let count_index = match tokens.get(start_index).copied() {
        Some("the" | "a" | "an") => start_index + 1,
        Some(_) => start_index,
        None => return None,
    };
    let count = tokens
        .get(count_index)
        .copied()
        .and_then(parse_cycle_count_value)?;
    let lookahead = &tokens[count_index + 1..tokens.len().min(count_index + 7)];
    contains_named_generic_edge_unit(lookahead, known_signal_tokens).then_some(count)
}

fn contains_cycle_like_unit(tokens: &[&str]) -> bool {
    tokens.iter().any(|token| {
        matches!(
            *token,
            "cycle" | "cycles" | "tick" | "ticks" | "posedge" | "posedges" | "negedge" | "negedges"
        )
    }) || contains_token_phrase(tokens, &["rising", "edge"])
        || contains_token_phrase(tokens, &["rising", "edges"])
        || contains_token_phrase(tokens, &["falling", "edge"])
        || contains_token_phrase(tokens, &["falling", "edges"])
        || contains_token_phrase(tokens, &["clock", "edge"])
        || contains_token_phrase(tokens, &["clock", "edges"])
}

fn diagram_position_cycle_like_unit_len(tokens: &[&str], index: usize) -> Option<usize> {
    match tokens.get(index).copied()? {
        "cycle" | "tick" | "posedge" | "negedge" => Some(1),
        "clock" if matches!(tokens.get(index + 1), Some(&"edge") | Some(&"edges")) => Some(2),
        "rising" | "falling" if tokens.get(index + 1) == Some(&"edge") => Some(2),
        _ => None,
    }
}

fn parse_explicit_cycle_position_count(tokens: &[&str], start_index: usize) -> Option<u32> {
    let count_index = match tokens.get(start_index).copied() {
        Some("the" | "a" | "an") => start_index + 1,
        Some(_) => start_index,
        None => return None,
    };
    let count = tokens
        .get(count_index)
        .copied()
        .and_then(parse_cycle_count_value)?;
    let lookahead = &tokens[count_index + 1..tokens.len().min(count_index + 5)];
    contains_cycle_like_unit(lookahead).then_some(count)
}

fn resolve_cycle_window_from_text(
    text: &str,
    signal_names: &BTreeSet<String>,
    actor_names: &BTreeSet<String>,
    actor_grounded: bool,
    handshake_completion: bool,
    prior_guidance: Option<&SemanticPriorGuidance>,
) -> Option<CycleWindowRecord> {
    extract_cycle_window_from_text_with_known_signals(text, signal_names).or_else(|| {
        prior_guidance.and_then(|guidance| {
            guidance.corpus_memory.temporal_cycle_window_in_text(
                Some(guidance.protocol_family),
                text,
                signal_names,
                actor_names,
                actor_grounded.then_some(true),
                handshake_completion.then_some(true),
            )
        })
    })
}

fn collect_known_actor_names(
    actors: &[ActorRecord],
    actor_ports: &[ActorPortRecord],
) -> BTreeSet<String> {
    let mut actor_names = actors
        .iter()
        .filter_map(|actor| actor.actor_name.clone())
        .collect::<BTreeSet<_>>();
    actor_names.extend(actor_ports.iter().map(|port| port.actor_name.clone()));
    actor_names
}

fn load_semantic_prior_guidance(
    prior_memory_path: Option<&Path>,
    document_key: &str,
    display_name: &str,
) -> Result<Option<SemanticPriorGuidance>> {
    let Some(prior_memory_path) = prior_memory_path else {
        return Ok(None);
    };
    let prior_memory_path =
        match resolve_existing(prior_memory_path, PersistedPathOrigin::RepositoryOwned) {
            Ok(path) => path,
            Err(AppError::MissingPath(_)) => return Ok(None),
            Err(error) => return Err(error),
        };
    let corpus_memory =
        serde_json::from_str::<CorpusMemory>(&fs::read_to_string(&prior_memory_path)?)?;
    Ok(Some(SemanticPriorGuidance {
        corpus_memory,
        protocol_family: ProtocolFamily::infer(document_key, display_name),
    }))
}

fn contains_token_phrase(tokens: &[&str], phrase: &[&str]) -> bool {
    if phrase.is_empty() || tokens.len() < phrase.len() {
        return false;
    }
    tokens.windows(phrase.len()).any(|window| {
        window
            .iter()
            .zip(phrase.iter())
            .all(|(lhs, rhs)| lhs == rhs)
    })
}

fn parse_cycle_count_value(token: &str) -> Option<u32> {
    parse_cardinal_cycle_count_value(token).or_else(|| parse_ordinal_cycle_count_value(token))
}

fn parse_cardinal_cycle_count_value(token: &str) -> Option<u32> {
    parse_u32_token(token).or(match token {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        "ten" => Some(10),
        _ => None,
    })
}

fn parse_ordinal_cycle_count_value(token: &str) -> Option<u32> {
    for suffix in ["st", "nd", "rd", "th"] {
        if let Some(prefix) = token.strip_suffix(suffix)
            && !prefix.is_empty()
            && let Some(value) = parse_u32_token(prefix)
        {
            return Some(value);
        }
    }

    match token {
        "first" => Some(1),
        "second" => Some(2),
        "third" => Some(3),
        "fourth" => Some(4),
        "fifth" => Some(5),
        "sixth" => Some(6),
        "seventh" => Some(7),
        "eighth" => Some(8),
        "ninth" => Some(9),
        "tenth" => Some(10),
        _ => None,
    }
}

fn parse_diagram_cycle_count_value(token: &str) -> Option<u32> {
    parse_cardinal_cycle_count_value(token).or_else(|| {
        token
            .strip_prefix('t')
            .filter(|suffix| !suffix.is_empty())
            .and_then(parse_cardinal_cycle_count_value)
    })
}

fn find_known_signal_name(text: &str, known_signals: &BTreeSet<String>) -> Option<String> {
    let mut best_match = None::<String>;
    for signal_name in known_signals {
        if contains_phrase_case_insensitive(text, signal_name) {
            match best_match.as_ref() {
                Some(existing) if existing.len() >= signal_name.len() => {}
                _ => best_match = Some(signal_name.clone()),
            }
        }
    }
    best_match
}

fn contains_phrase_case_insensitive(text: &str, phrase: &str) -> bool {
    let text_lower = text.to_ascii_lowercase();
    let phrase_lower = phrase.to_ascii_lowercase();
    contains_text_phrase(&text_lower, &phrase_lower)
}

/// Resolve an un-indexed prose signal reference to its DECLARED indexed family
/// member, using the universal index-suffix convention: a per-instance signal is
/// written `PSELx` / `HSELx` (or numerically, `FOO0`/`FOO1`) in the declaration but
/// referenced bare as `PSEL` / `HSEL` / `FOO` in prose. Returns the declared signal
/// (e.g. `PSEL` → `PSELX`) so the IR uses one canonical identity everywhere — this is
/// grammar (the `x`/digit index convention), never a hardcoded chip-spec name
/// (ADR 0006). `WIRE-BASED-100.4`.
///
/// Fires ONLY for a candidate token that is not itself a known signal, so it is
/// purely additive and can never override a real declaration. The trailing `X`
/// marker matches the uppercased declared form of the lowercase-`x` instance index.
fn resolve_indexed_signal_family(text: &str, known_signals: &BTreeSet<String>) -> Option<String> {
    for token in text.split(|c: char| !(c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')) {
        if token.len() < 3
            || !token
                .chars()
                .next()
                .map(|c| c.is_ascii_uppercase())
                .unwrap_or(false)
        {
            continue;
        }
        let upper = token.to_ascii_uppercase();
        if known_signals.contains(&upper) {
            continue; // already a declared signal — not an un-indexed family reference
        }
        for declared in known_signals {
            if let Some(suffix) = declared.strip_prefix(upper.as_str()) {
                let is_index_suffix = suffix == "X"
                    || (!suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()));
                if is_index_suffix {
                    return Some(declared.clone());
                }
            }
        }
    }
    None
}

fn contains_text_phrase(text: &str, phrase: &str) -> bool {
    for (index, _) in text.match_indices(phrase) {
        let prefix_ok = text[..index]
            .chars()
            .next_back()
            .map(|character| !character.is_ascii_alphanumeric() && character != '_')
            .unwrap_or(true);
        let suffix_index = index + phrase.len();
        let suffix_ok = text[suffix_index..]
            .chars()
            .next()
            .map(|character| !character.is_ascii_alphanumeric() && character != '_')
            .unwrap_or(true);
        if prefix_ok && suffix_ok {
            return true;
        }
    }
    false
}

fn extract_symbolic_value(text: &str, excluded_signal: Option<&str>) -> Option<String> {
    text.split(|ch: char| !ch.is_ascii_alphanumeric() && ch != '_')
        .filter(|token| !token.is_empty())
        .find_map(|token| {
            let uppercase = token
                .chars()
                .any(|character| character.is_ascii_uppercase());
            if !uppercase {
                return None;
            }
            if matches!(
                token,
                "HIGH" | "LOW" | "ASSERTED" | "DEASSERTED" | "WHEN" | "IF" | "WHILE"
            ) {
                return None;
            }
            if excluded_signal.is_some_and(|signal| token.eq_ignore_ascii_case(signal)) {
                return None;
            }
            Some(token.to_string())
        })
}

fn extract_actor_after_by(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();
    let by_index = lower.find(" by ")?;
    let actor_text = text[by_index + 4..]
        .split(['.', ',', ';'])
        .next()
        .unwrap_or("")
        .trim();
    let actor_text = actor_text
        .strip_prefix("the ")
        .or_else(|| actor_text.strip_prefix("The "))
        .unwrap_or(actor_text)
        .trim();
    if actor_text.is_empty() {
        None
    } else {
        Some(actor_text.to_string())
    }
}

/// Parse VLM-derived `VisualObservation` entries from `EvidenceIR` into typed
/// `SemanticIR` records.
///
/// Returns `(timing_constraints, regular_states, state_transitions)` extracted from:
/// - `VisualObservationKind::TimingDiagramExtraction` → timing annotation → `TimingConstraintRecord`
/// - `VisualObservationKind::StateMachineExtraction` → states/transitions → typed records
fn extract_records_from_vlm_observations(
    evidence_ir: &EvidenceIr,
    known_signal_names: &HashSet<String>,
) -> (
    Vec<TimingConstraintRecord>,
    Vec<SignalConstraintRecord>,
    Vec<RegularStateRecord>,
    Vec<StateTransitionRecord>,
) {
    let mut timing_records = Vec::new();
    let mut signal_constraint_records = Vec::new();
    let mut state_records = Vec::new();
    let mut transition_records = Vec::new();

    for visual_item in &evidence_ir.visual_evidence {
        for obs in &visual_item.observations {
            match obs.kind {
                VisualObservationKind::TimingDiagramExtraction => {
                    parse_timing_diagram_observation(
                        &obs.text,
                        &visual_item.evidence_id,
                        known_signal_names,
                        &mut timing_records,
                        &mut signal_constraint_records,
                    );
                }
                VisualObservationKind::StateMachineExtraction => {
                    parse_state_machine_observation(
                        &obs.text,
                        &visual_item.evidence_id,
                        known_signal_names,
                        &mut state_records,
                        &mut transition_records,
                    );
                }
                _ => {}
            }
        }
    }

    (
        timing_records,
        signal_constraint_records,
        state_records,
        transition_records,
    )
}

/// Parse a `TimingDiagramExtraction` JSON observation into `TimingConstraintRecord` entries.
/// Expected format:
/// `{"signals":[{"name":str,"values":[{"cycle":str,"state":str}]}],"annotations":[str]}`
fn parse_timing_diagram_observation(
    json_text: &str,
    evidence_id: &str,
    known_signal_names: &HashSet<String>,
    records: &mut Vec<TimingConstraintRecord>,
    signal_constraints: &mut Vec<SignalConstraintRecord>,
) {
    let Some(value) = parse_visual_observation_json(json_text) else {
        return;
    };

    // Each annotation string becomes a TimingConstraintRecord.
    if let Some(annotations) = value.get("annotations").and_then(|a| a.as_array()) {
        for (idx, annotation) in annotations.iter().enumerate() {
            let text = annotation.as_str().unwrap_or_default().trim();
            if text.is_empty()
                || is_spurious_timing_annotation_label(text)
                || is_name_only_signal_annotation_label(text, known_signal_names)
                || is_signal_value_annotation_label(text, known_signal_names)
            {
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

    push_signal_constraints_from_timing_diagram_observation(
        &value,
        evidence_id,
        known_signal_names,
        signal_constraints,
    );
}

fn is_name_only_signal_annotation_label(text: &str, known_signal_names: &HashSet<String>) -> bool {
    let Some(signal_name) = parse_identifier(text) else {
        return false;
    };

    known_signal_names
        .iter()
        .any(|known_name| known_name.eq_ignore_ascii_case(&signal_name))
}

fn is_signal_value_annotation_label(text: &str, known_signal_names: &HashSet<String>) -> bool {
    let tokens = text.split_whitespace().collect::<Vec<_>>();
    let Some(signal_token) = tokens.first().copied() else {
        return false;
    };
    let Some(value_token) = tokens.get(1).copied() else {
        return false;
    };

    let signal_token = signal_token
        .trim_end_matches('.')
        .trim_end_matches(':')
        .trim_end_matches(',');
    let Some(signal_name) = parse_identifier(signal_token)
        .or_else(|| parse_indexed_signal_annotation_base(signal_token))
    else {
        return false;
    };
    if !known_signal_names
        .iter()
        .any(|known_name| known_name.eq_ignore_ascii_case(&signal_name))
    {
        return false;
    }

    let normalized_value = value_token
        .trim()
        .trim_end_matches('.')
        .trim_end_matches(':')
        .trim_end_matches(',')
        .trim_matches(|character| matches!(character, '"' | '`'));
    matches!(
        signal_constraint_kind_from_vlm_state(normalized_value),
        Some((
            SignalConstraintKind::MustBeHigh
                | SignalConstraintKind::MustBeLow
                | SignalConstraintKind::MustBeAsserted
                | SignalConstraintKind::MustBeDeasserted,
            None
        ))
    ) && trailing_tokens_form_only_cycle_marker_label(&tokens[2..])
}

fn parse_indexed_signal_annotation_base(signal_token: &str) -> Option<String> {
    for (open_delimiter, close_delimiter) in [('[', ']'), ('<', '>')] {
        let Some((prefix_text, index_with_delimiter)) = signal_token.split_once(open_delimiter)
        else {
            continue;
        };
        let Some(index_text) = index_with_delimiter.strip_suffix(close_delimiter) else {
            continue;
        };
        let prefix = prefix_text.trim();
        let index = index_text.trim();
        if prefix.is_empty() || index.is_empty() || index.contains(open_delimiter) {
            continue;
        }
        if !is_waveform_index_token(index) {
            continue;
        }
        if let Some(signal_name) = parse_identifier(prefix) {
            return Some(signal_name);
        }
    }

    None
}

fn trailing_tokens_form_only_cycle_marker_label(tokens: &[&str]) -> bool {
    if tokens.is_empty() {
        return true;
    }

    let normalized_tokens = tokens
        .iter()
        .map(|token| {
            token
                .trim()
                .trim_end_matches('.')
                .trim_end_matches(':')
                .trim_end_matches(',')
                .trim_matches(|character| matches!(character, '"' | '`'))
                .to_ascii_lowercase()
        })
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();

    match normalized_tokens.as_slice() {
        [cycle] => is_timing_cycle_marker_token(cycle),
        [prefix, cycle] => {
            matches!(prefix.as_str(), "at" | "on" | "during" | "in")
                && is_timing_cycle_marker_token(cycle)
        }
        _ => false,
    }
}

fn push_signal_constraints_from_timing_diagram_observation(
    value: &serde_json::Value,
    evidence_id: &str,
    known_signal_names: &HashSet<String>,
    records: &mut Vec<SignalConstraintRecord>,
) {
    let Some(signals) = value.get("signals").and_then(|signals| signals.as_array()) else {
        return;
    };

    for (signal_index, signal_value) in signals.iter().enumerate() {
        let Some(signal_name) = signal_value
            .get("name")
            .and_then(|name| name.as_str())
            .and_then(|name| parse_allowed_vlm_observation_signal(name, known_signal_names))
        else {
            continue;
        };
        let Some(values) = signal_value
            .get("values")
            .and_then(|values| values.as_array())
        else {
            continue;
        };

        for (value_index, value) in values.iter().enumerate() {
            let Some(raw_state) = value.get("state").and_then(|state| state.as_str()) else {
                continue;
            };
            let Some((constraint_kind, target_value)) =
                signal_constraint_kind_from_vlm_state(raw_state)
            else {
                continue;
            };
            let cycle = value
                .get("cycle")
                .and_then(|cycle| cycle.as_str())
                .map(str::trim)
                .filter(|cycle| !cycle.is_empty());
            let source_text = if let Some(cycle) = cycle {
                format!(
                    "VLM timing diagram observation: {signal_name} is {} at cycle {cycle}.",
                    raw_state.trim()
                )
            } else {
                format!(
                    "VLM timing diagram observation: {signal_name} is {}.",
                    raw_state.trim()
                )
            };

            records.push(SignalConstraintRecord {
                constraint_id: format!(
                    "vlm_signal_value_{}_{}_{signal_index:03}_{value_index:03}",
                    document_key(evidence_id),
                    document_key(&signal_name)
                ),
                subject_signal: signal_name.clone(),
                constraint_kind,
                target_value,
                condition_text: None,
                negated: false,
                source_text,
                supporting_statement_ids: vec![evidence_id.to_string()],
                automation_confidence: AutomationConfidence::Medium,
            });
        }
    }
}

fn parse_allowed_vlm_observation_signal(
    signal_text: &str,
    known_signal_names: &HashSet<String>,
) -> Option<String> {
    let signal_name = parse_identifier(trim_vlm_guard_clause(signal_text))?;
    if known_signal_names.contains(&signal_name)
        || (known_signal_names.is_empty() && !is_generic_vlm_signal_term(&signal_name))
    {
        Some(signal_name)
    } else {
        None
    }
}

fn signal_constraint_kind_from_vlm_state(
    raw_state: &str,
) -> Option<(SignalConstraintKind, Option<String>)> {
    let state = raw_state
        .trim()
        .trim_matches(|character| matches!(character, '"' | '`'));
    if state.is_empty() {
        return None;
    }

    let normalized = state.to_ascii_lowercase();
    let value = normalized.as_str();
    // Logic levels are universal binary-logic vocabulary (the "how", ADR 0006) — centralized
    // in `normative_vocab` as the single authority, NOT a chip-spec name (see that module).
    if crate::ir::normative_vocab::LOGIC_HIGH_VALUES.contains(&value) {
        return Some((SignalConstraintKind::MustBeHigh, None));
    }
    if crate::ir::normative_vocab::LOGIC_LOW_VALUES.contains(&value) {
        return Some((SignalConstraintKind::MustBeLow, None));
    }
    match value {
        "asserted" | "assert" => Some((SignalConstraintKind::MustBeAsserted, None)),
        "deasserted" | "deassert" => Some((SignalConstraintKind::MustBeDeasserted, None)),
        "x" | "z" | "unknown" | "don't care" | "dont care" => None,
        _ => {
            if is_vlm_waveform_motion_state(state) {
                return None;
            }
            let symbolic_value = parse_identifier(state)?;
            Some((
                SignalConstraintKind::MustBeValue {
                    value: symbolic_value.clone(),
                },
                Some(symbolic_value),
            ))
        }
    }
}

fn is_vlm_waveform_motion_state(state: &str) -> bool {
    let normalized = normalize_vlm_waveform_motion_state(state);
    matches!(
        normalized.as_str(),
        "rise"
            | "rising"
            | "fall"
            | "falling"
            | "rising edge"
            | "falling edge"
            | "rise edge"
            | "fall edge"
            | "positive edge"
            | "negative edge"
            | "pos edge"
            | "neg edge"
            | "risingedge"
            | "fallingedge"
            | "positiveedge"
            | "negativeedge"
            | "posedge"
            | "negedge"
            | "edge"
            | "transition"
            | "transient"
            | "toggle"
            | "toggling"
            | "stable"
            | "steady"
            | "unchanged"
            | "held"
            | "hold"
            | "low to high"
            | "high to low"
            | "lowtohigh"
            | "hightolow"
            | "low2high"
            | "high2low"
            | "0 to 1"
            | "1 to 0"
            | "zero to one"
            | "one to zero"
    )
}

fn normalize_vlm_waveform_motion_state(state: &str) -> String {
    state
        .trim()
        .to_ascii_lowercase()
        .split(|character: char| {
            character == '_' || character == '-' || character.is_ascii_whitespace()
        })
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>()
        .join(" ")
}

fn is_spurious_timing_annotation_label(text: &str) -> bool {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        return true;
    }

    if is_timing_cycle_marker_token(trimmed) {
        return true;
    }
    if is_compact_waveform_index_label(trimmed) {
        return true;
    }
    if is_non_quantitative_waveform_motion_annotation(trimmed) {
        return true;
    }

    let tokens = trimmed
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|token| !token.is_empty())
        .collect::<Vec<_>>();
    if tokens.is_empty() {
        return true;
    }

    tokens.len() <= 4
        && tokens
            .iter()
            .all(|token| is_generic_waveform_label_token(token))
}

fn is_non_quantitative_waveform_motion_annotation(text: &str) -> bool {
    let tokens = text
        .split(|character: char| {
            !(character.is_ascii_alphanumeric() || character == '_' || character == '\'')
        })
        .filter(|token| !token.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();
    if tokens.is_empty()
        || !tokens
            .iter()
            .any(|token| is_waveform_motion_annotation_token(token))
    {
        return false;
    }
    if tokens
        .iter()
        .any(|token| is_timing_annotation_constraint_token(token))
    {
        return false;
    }
    if tokens
        .iter()
        .any(|token| token.chars().any(|character| character.is_ascii_digit()))
    {
        return false;
    }

    true
}

fn is_waveform_motion_annotation_token(token: &str) -> bool {
    let normalized = normalize_vlm_waveform_motion_state(token);
    is_vlm_waveform_motion_state(&normalized)
        || matches!(
            normalized.as_str(),
            "rises"
                | "falls"
                | "rose"
                | "fell"
                | "stays"
                | "stay"
                | "remains"
                | "remain"
                | "keeps"
                | "kept"
                | "settles"
                | "settled"
        )
}

fn is_timing_annotation_constraint_token(token: &str) -> bool {
    matches!(
        token,
        "setup"
            | "hold"
            | "time"
            | "timing"
            | "delay"
            | "delayed"
            | "latency"
            | "period"
            | "pulse"
            | "width"
            | "skew"
            | "window"
            | "minimum"
            | "maximum"
            | "min"
            | "max"
            | "within"
            | "before"
            | "after"
            | "until"
            | "during"
            | "while"
            | "when"
            | "if"
            | "must"
            | "shall"
            | "require"
            | "requires"
            | "required"
            | "cycle"
            | "cycles"
            | "tick"
            | "ticks"
    )
}

fn is_timing_cycle_marker_token(token: &str) -> bool {
    let token = token.trim();
    token.len() >= 2
        && token
            .strip_prefix('T')
            .or_else(|| token.strip_prefix('t'))
            .map(|suffix| !suffix.is_empty() && suffix.chars().all(|c| c.is_ascii_digit()))
            .unwrap_or(false)
}

fn is_generic_waveform_label_token(token: &str) -> bool {
    let lowered = token.trim().to_ascii_lowercase();
    if lowered.is_empty() {
        return false;
    }

    lowered.chars().all(|character| character.is_ascii_digit())
        || is_timing_cycle_marker_token(&lowered)
        || is_compact_waveform_sample_label(&lowered)
        || matches!(
            lowered.as_str(),
            "addr"
                | "address"
                | "data"
                | "burst"
                | "frame"
                | "packet"
                | "transaction"
                | "txn"
                | "phase"
                | "transfer"
                | "beat"
                | "cycle"
                | "slot"
                | "wait"
                | "lane"
                | "channel"
        )
}

fn is_compact_waveform_sample_label(token: &str) -> bool {
    let token = token.trim().to_ascii_lowercase();
    if token.is_empty() {
        return false;
    }

    if let Some(hex_digits) = token.strip_prefix("0x") {
        return !hex_digits.is_empty()
            && hex_digits
                .chars()
                .all(|character| character.is_ascii_hexdigit());
    }
    if let Some(binary_digits) = token.strip_prefix("0b") {
        return !binary_digits.is_empty()
            && binary_digits
                .chars()
                .all(|character| matches!(character, '0' | '1'));
    }

    for prefix in [
        "addr",
        "address",
        "data",
        "burst",
        "frame",
        "packet",
        "transaction",
        "txn",
        "phase",
        "transfer",
        "beat",
        "cycle",
        "slot",
        "lane",
        "channel",
        "wait",
    ] {
        if let Some(suffix) = token.strip_prefix(prefix) {
            return !suffix.is_empty()
                && suffix.chars().all(|character| character.is_ascii_digit());
        }
    }

    let mut chars = token.chars();
    let Some(prefix) = chars.next() else {
        return false;
    };
    let suffix = chars.as_str();
    matches!(prefix, 'a' | 'd')
        && !suffix.is_empty()
        && suffix.chars().all(|character| character.is_ascii_digit())
}

fn is_compact_waveform_index_label(text: &str) -> bool {
    let text = text.trim().to_ascii_lowercase();
    if text.is_empty() {
        return false;
    }

    for (open_delimiter, close_delimiter) in [('[', ']'), ('<', '>')] {
        let Some((prefix_text, index_with_delimiter)) = text.split_once(open_delimiter) else {
            continue;
        };
        let Some(index_text) = index_with_delimiter.strip_suffix(close_delimiter) else {
            continue;
        };
        let prefix = prefix_text.trim();
        let index = index_text.trim();
        if prefix.is_empty() || index.is_empty() || index.contains(open_delimiter) {
            continue;
        }
        if !matches!(
            prefix,
            "a" | "d"
                | "addr"
                | "address"
                | "data"
                | "beat"
                | "cycle"
                | "slot"
                | "lane"
                | "channel"
                | "wait"
        ) && parse_identifier(prefix).is_none()
        {
            continue;
        }
        if is_waveform_index_token(index) {
            return true;
        }
    }

    false
}

fn is_waveform_index_token(index: &str) -> bool {
    let index = index.trim();
    if index.is_empty() {
        return false;
    }
    if index.chars().all(|character| character.is_ascii_digit()) {
        return true;
    }
    if let Some(hex_digits) = index.strip_prefix("0x") {
        return !hex_digits.is_empty()
            && hex_digits
                .chars()
                .all(|character| character.is_ascii_hexdigit());
    }
    if let Some((left, right)) = index.split_once(':') {
        return [left, right].into_iter().all(is_waveform_index_token);
    }

    false
}

/// Parse a `StateMachineExtraction` JSON observation into state and transition records.
/// Expected format:
/// `{"states":[{"name":str,"is_initial":bool}],"transitions":[{"from":str,"to":str,"guard":str}]}`
fn parse_state_machine_observation(
    json_text: &str,
    evidence_id: &str,
    known_signal_names: &HashSet<String>,
    state_records: &mut Vec<RegularStateRecord>,
    transition_records: &mut Vec<StateTransitionRecord>,
) {
    let Some(value) = parse_visual_observation_json(json_text) else {
        return;
    };

    let mut observation_state_names = BTreeSet::new();

    // Parse states.
    if let Some(states) = value.get("states").and_then(|s| s.as_array()) {
        for (idx, state_val) in states.iter().enumerate() {
            let Some(name) = state_val
                .get("name")
                .and_then(|n| n.as_str())
                .and_then(parse_identifier)
            else {
                continue;
            };
            let is_initial = state_val
                .get("is_initial")
                .and_then(|b| b.as_bool())
                .unwrap_or(false);
            observation_state_names.insert(name.clone());
            if let Some(existing_state) = state_records
                .iter_mut()
                .find(|state| state.state_name == name)
            {
                existing_state.is_initial |= is_initial;
                if !existing_state
                    .supporting_statement_ids
                    .iter()
                    .any(|statement_id| statement_id == evidence_id)
                {
                    existing_state
                        .supporting_statement_ids
                        .push(evidence_id.to_string());
                }
                continue;
            }
            state_records.push(RegularStateRecord {
                state_id: format!(
                    "vlm_state_{}_{}",
                    document_key(evidence_id),
                    document_key(&name)
                ),
                state_name: name,
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
            let Some(from) = trans_val
                .get("from")
                .and_then(|f| f.as_str())
                .and_then(parse_identifier)
            else {
                continue;
            };
            let Some(to) = trans_val
                .get("to")
                .and_then(|t| t.as_str())
                .and_then(parse_identifier)
            else {
                continue;
            };
            if !observation_state_names.contains(&from) || !observation_state_names.contains(&to) {
                continue;
            }
            let guard_text = trans_val
                .get("guard")
                .and_then(|g| g.as_str())
                .unwrap_or_default()
                .trim();
            let guard = if guard_text.is_empty() {
                None
            } else {
                parse_vlm_state_machine_guard(guard_text, known_signal_names)
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

fn parse_vlm_state_machine_guard(
    guard_text: &str,
    known_signal_names: &HashSet<String>,
) -> Option<DecisionTreeGuardRecord> {
    let clauses = split_vlm_guard_clauses(guard_text);
    if clauses.is_empty() {
        return None;
    }

    for clause in clauses
        .iter()
        .filter(|clause| vlm_guard_clause_has_comparison(clause))
    {
        if let Some(guard) = parse_vlm_state_machine_guard_clause(clause, known_signal_names) {
            return Some(guard);
        }
    }

    clauses
        .iter()
        .find_map(|clause| parse_vlm_state_machine_guard_clause(clause, known_signal_names))
}

fn split_vlm_guard_clauses(guard_text: &str) -> Vec<&str> {
    let mut clauses = Vec::new();
    let mut remainder = guard_text.trim();
    while !remainder.is_empty() {
        let Some((delimiter_index, delimiter_len)) = find_vlm_guard_clause_delimiter(remainder)
        else {
            clauses.push(remainder.trim());
            break;
        };

        let clause = remainder[..delimiter_index].trim();
        if !clause.is_empty() {
            clauses.push(clause);
        }
        remainder = remainder[delimiter_index + delimiter_len..].trim();
    }

    clauses
}

fn find_vlm_guard_clause_delimiter(text: &str) -> Option<(usize, usize)> {
    let lowered = text.to_ascii_lowercase();
    ["&&", " and ", ",", ";", "\n"]
        .iter()
        .filter_map(|delimiter| {
            lowered
                .find(delimiter)
                .map(|index| (index, delimiter.len()))
        })
        .min_by_key(|(index, _)| *index)
}

fn vlm_guard_clause_has_comparison(clause: &str) -> bool {
    clause.contains("==")
        || clause.contains("!=")
        || clause.contains('=')
        || find_ascii_case_insensitive(clause, " is ").is_some()
}

fn parse_vlm_state_machine_guard_clause(
    clause: &str,
    known_signal_names: &HashSet<String>,
) -> Option<DecisionTreeGuardRecord> {
    let clause = trim_vlm_guard_clause(clause);
    if clause.is_empty() {
        return None;
    }

    if let Some((left_signal, right_text)) = clause.split_once("==") {
        return Some(DecisionTreeGuardRecord::Comparison {
            left_signal: parse_allowed_vlm_guard_signal(left_signal, known_signal_names)?,
            operator: DecisionTreeComparisonOperator::Eq,
            right: parse_vlm_decision_tree_value(right_text, known_signal_names)?,
        });
    }
    if let Some((left_signal, right_text)) = clause.split_once("!=") {
        return Some(DecisionTreeGuardRecord::Comparison {
            left_signal: parse_allowed_vlm_guard_signal(left_signal, known_signal_names)?,
            operator: DecisionTreeComparisonOperator::NotEq,
            right: parse_vlm_decision_tree_value(right_text, known_signal_names)?,
        });
    }
    if let Some((left_signal, right_text)) = clause.split_once('=') {
        if left_signal.trim_end().ends_with(['!', '<', '>']) {
            return None;
        }
        return Some(DecisionTreeGuardRecord::Comparison {
            left_signal: parse_allowed_vlm_guard_signal(left_signal, known_signal_names)?,
            operator: DecisionTreeComparisonOperator::Eq,
            right: parse_vlm_decision_tree_value(right_text, known_signal_names)?,
        });
    }
    if let Some(index) = find_ascii_case_insensitive(clause, " is ") {
        let (left_signal, right_text_with_is) = clause.split_at(index);
        let right_text =
            right_text_with_is.trim_start_matches(|c: char| c.is_ascii_whitespace())[2..].trim();
        return Some(DecisionTreeGuardRecord::Comparison {
            left_signal: parse_allowed_vlm_guard_signal(left_signal, known_signal_names)?,
            operator: DecisionTreeComparisonOperator::Eq,
            right: parse_vlm_decision_tree_value(right_text, known_signal_names)?,
        });
    }

    Some(DecisionTreeGuardRecord::SignalIsHigh {
        signal_name: parse_allowed_vlm_guard_signal(clause, known_signal_names)?,
    })
}

fn trim_vlm_guard_clause(clause: &str) -> &str {
    clause
        .trim()
        .trim_matches(|character| matches!(character, '(' | ')' | '[' | ']'))
        .trim()
}

fn parse_allowed_vlm_guard_signal(
    signal_text: &str,
    known_signal_names: &HashSet<String>,
) -> Option<String> {
    let signal_name = parse_identifier(trim_vlm_guard_clause(signal_text))?;
    if known_signal_names.contains(&signal_name)
        || (known_signal_names.is_empty() && !is_generic_vlm_signal_term(&signal_name))
    {
        Some(signal_name)
    } else {
        None
    }
}

fn is_generic_vlm_signal_term(signal_name: &str) -> bool {
    matches!(
        signal_name.to_ascii_lowercase().as_str(),
        "transfer"
            | "transaction"
            | "request"
            | "response"
            | "beat"
            | "cycle"
            | "phase"
            | "state"
            | "condition"
            | "event"
    )
}

fn parse_vlm_decision_tree_value(
    value_text: &str,
    known_signal_names: &HashSet<String>,
) -> Option<DecisionTreeValueRecord> {
    let value_text = trim_vlm_guard_clause(
        value_text
            .trim()
            .trim_end_matches('.')
            .trim_end_matches(','),
    )
    .trim_matches(|character| matches!(character, '"' | '`'));
    if value_text.is_empty() {
        return None;
    }

    let lowered = value_text.to_ascii_lowercase();
    if matches!(
        lowered.as_str(),
        "0" | "1" | "1'b0" | "1'b1" | "low" | "high" | "false" | "true" | "asserted" | "deasserted"
    ) {
        return Some(DecisionTreeValueRecord::Literal {
            literal: value_text.to_string(),
        });
    }

    if let Some(signal_name) = parse_identifier(value_text)
        && (known_signal_names.is_empty() || known_signal_names.contains(&signal_name))
    {
        return Some(DecisionTreeValueRecord::SignalRef { signal_name });
    }

    Some(DecisionTreeValueRecord::Literal {
        literal: value_text.to_string(),
    })
}

fn find_ascii_case_insensitive(haystack: &str, needle: &str) -> Option<usize> {
    haystack
        .to_ascii_lowercase()
        .find(&needle.to_ascii_lowercase())
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

fn is_administrative_section_title(title: &str) -> bool {
    let trimmed = title.trim();
    let title_without_number = trimmed
        .find(char::is_whitespace)
        .and_then(|index| {
            let prefix = &trimmed[..index];
            (prefix.chars().any(|character| character.is_ascii_digit())
                && prefix
                    .chars()
                    .all(|character| character.is_ascii_digit() || matches!(character, '.' | ')')))
            .then_some(trimmed[index..].trim_start())
        })
        .unwrap_or(trimmed);
    let lowered = title_without_number.to_ascii_lowercase();

    if matches!(
        lowered.as_str(),
        "references" | "normative references" | "informative references"
    ) {
        return true;
    }

    let certification_context = contains_any_phrase(&lowered, &["certified", "certification"]);
    let certification_administration = certification_context
        && contains_any_phrase(
            &lowered,
            &[
                "mark",
                "logo",
                "list",
                "listing",
                "publication",
                "demonstrating",
                "requesting",
                "program",
            ],
        );
    let submission_administration = contains_phrase(&lowered, "requesting")
        && contains_any_phrase(
            &lowered,
            &[
                "product", "license", "licence", "mark", "logo", "list", "listing",
            ],
        );
    let internal_request_processing = contains_any_phrase(&lowered, &["request", "requested"])
        && contains_phrase(&lowered, "information")
        && contains_phrase(&lowered, "internal processing");
    let institutional_test_lab =
        contains_any_phrase(&lowered, &["independent test lab", "independent test labs"]);

    certification_administration
        || submission_administration
        || internal_request_processing
        || institutional_test_lab
}

fn is_legal_or_administrative_statement(text: &str) -> bool {
    let lowered = text.to_ascii_lowercase();
    let has_any = |phrases: &[&str]| contains_any_phrase(&lowered, phrases);

    let copyright_notice = contains_phrase(&lowered, "copyright")
        && has_any(&[
            "document",
            "notice",
            "reserved",
            "permission",
            "license",
            "licence",
        ]);
    let inherently_legal = has_any(&[
        "intellectual property",
        "trademark",
        "trademarks",
        "service mark",
        "service marks",
        "derivative work",
        "derivative works",
        "applicable law",
        "successors or assigns",
        "adopters agreement",
    ]);
    let warranty_or_liability = has_any(&[
        "warranty",
        "warranties",
        "liability",
        "liabilities",
        "liable",
    ]) && has_any(&[
        "document",
        "information",
        "product",
        "products",
        "customer",
        "commercial sale",
        "specification",
    ]);
    let patent_rights = has_any(&["patent", "patents"])
        && has_any(&[
            "right",
            "rights",
            "infringe",
            "infringement",
            "public domain",
        ]);
    let document_license = has_any(&[
        "license",
        "licenses",
        "licence",
        "licences",
        "licensee",
        "licensees",
    ]) && has_any(&[
        "document",
        "specification",
        "agreement",
        "intellectual property",
        "royalty-free",
        "confidential",
    ]);
    let revocable_permission = has_any(&["permission granted", "permissions granted"])
        && has_any(&["revoked", "right", "rights", "successors", "assigns"]);
    let commercial_terms = contains_phrase(&lowered, "terms and conditions")
        && has_any(&["sale", "agreement", "customer"]);
    let ipr_declaration = has_any(&["ipr", "iprs"])
        && has_any(&["declared", "rights policy", "deliverable", "deliverables"]);

    // Administrative process prose needs the same evidence/authority separation as legal prose.
    // Require compound organizational context: request/response is ubiquitous in protocols, an
    // electrical contact is not a contact address, and certification requirements can still be
    // real product constraints. These combinations identify document-reading, submission,
    // listing, institutional review, mark licensing, and test-program administration without a
    // vendor, document, organization, section, or exact-sentence exception.
    let document_reading_guidance = has_any(&["document", "documents", "specification", "manual"])
        && has_any(&["when reading", "helpful when reading", "helpful to read"]);
    let submission = has_any(&[
        "request",
        "requests",
        "requesting",
        "submitted",
        "submitting",
        "submission",
        "application form",
        "apply for",
        "resubmit",
    ]);
    let administrative_channel = has_any(&[
        "email",
        "contact name",
        "contact email",
        "contact phone",
        "contact information",
        "contact address",
        "online form",
        "online request",
        "product for approval",
        "approved product",
        "certified mark",
        "certified list",
        "listing request",
        "product listing",
        "logo usage",
        "trademark license",
        "trademark licence",
        "work group",
        "committee",
        "board of directors",
    ]);
    let submission_or_listing_workflow = submission && administrative_channel;
    let institutional_conflict_workflow = has_any(&[
        "conflict resolution",
        "mutual agreement",
        "recommended resolution",
    ]) && has_any(&[
        "email",
        "work group",
        "committee",
        "board of directors",
        "chair",
        "submitted",
        "escalated",
        "final determination",
        "certification program",
        "certified program",
        "logo",
        "test lab",
    ]);
    let derivative_listing_workflow = has_any(&[
        "derivative product",
        "derivative products",
        "as a derivative",
    ]) && has_any(&[
        "apply",
        "listing",
        "listed",
        "request",
        "re-test",
        "re-testing",
    ]);
    let certification_attestation_workflow = has_any(&["certified", "certification"])
        && has_any(&["manufacturer", "manufacturers", "supplier", "applicant"])
        && has_any(&["shall assert", "must attest", "shall attest"])
        && has_any(&["requirements", "criteria"]);
    let certification_mark_administration = has_any(&["mark", "marks", "marking"])
        && has_any(&[
            "badge",
            "logo",
            "trademark",
            "license to display",
            "licence to display",
            "display the mark",
            "display the certified",
            "product listing",
            "certified list",
            "ecosystem participants",
            "organizations wishing",
        ]);
    let institutional_testing_administration = has_any(&["test lab", "test labs", "auditor"])
        && has_any(&[
            "certification program",
            "contracted",
            "selecting a lab",
            "single site",
            "administer third party testing",
        ]);
    let product_qualification_testing = has_any(&["product", "products"])
        && has_any(&["tested", "testing", "submitted for test"])
        && has_any(&["out-of-box", "end user", "offered in the market"]);

    copyright_notice
        || inherently_legal
        || warranty_or_liability
        || patent_rights
        || document_license
        || revocable_permission
        || commercial_terms
        || ipr_declaration
        || document_reading_guidance
        || submission_or_listing_workflow
        || institutional_conflict_workflow
        || derivative_listing_workflow
        || certification_attestation_workflow
        || certification_mark_administration
        || institutional_testing_administration
        || product_qualification_testing
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, BTreeSet, HashMap, HashSet},
        fs,
    };

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::evidence::{
        EvidenceIr, EvidenceModality, ExtractedStatement, InterfaceClockEdge,
        InterfaceEdgeTimingRecord, ProtocolStateRecord, SerialFrameField, SerialFramePhase,
        SignalPolarity, StatementClass, SwdOperation, SwdioDirection, VisualEvidenceRole,
    };
    use crate::ir::intent::IntentIr;
    use crate::ir::prior_memory::{
        CorpusMemory, CorpusMemoryUpdatePolicyRecord, PriorSourceArtifactRecord, ProtocolFamily,
        SemanticModalityReliabilityPriorRecord, TemporalPhrasePriorRecord,
    };
    use crate::ir::source::{
        AutomationConfidence, SignalConstraintKind, SourceIr, StructuredTableCellRecord,
        StructuredTableRecord, TableKind, VisualAsset, VisualAssetKind, WidthHint,
    };

    use super::{
        ActorRelativeDirection, ControlActionRecord, ControlBinaryOperator, ControlBlockRole,
        ControlCompoundUpdateOperation, ControlDualOutputKind, ControlExpressionRecord,
        ControlReferenceKind, ControlReferenceSuffix, ControlUnaryOperator, CycleWindowRecord,
        DecisionTreeComparisonOperator, DecisionTreeGuardRecord, DecisionTreeValueRecord,
        InfrastructureTopologyKind, InterfaceSignalDirection, InterfaceSignalSemanticRole,
        SemanticArbitrationDecisionBasis, SemanticContext, SemanticGroundingStrength, SemanticIr,
        SemanticSectionContext, SignalSemanticHintSourceKind, SignalSemanticTag,
        SymbolDefinitionKind, SystemResetKind, SystemResetPolarity, SystemResetTargetKind,
        SystemResetTimingRelation, TransactionAnchorRecord, TransactionPhaseRecord,
        build_transaction_anchors, build_transaction_phases, control_binary_operator_key,
        control_block_role_key, control_reference_kind_key, control_reference_suffix_key,
        control_unary_operator_key, decision_tree_comparison_operator_key, derive_phase_name,
        derive_transaction_name, is_descendant_section_number,
        is_explicit_infrastructure_component_term, is_false, is_zero, leading_section_number,
        split_control_header_keyword, width_hint_key,
    };

    fn make_table_cell(text: &str, is_header: bool) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        }
    }

    fn write_temporal_phrase_prior_memory(
        root: &std::path::Path,
        normalized_phrase: &str,
        protocol_family: ProtocolFamily,
        cycle_window: CycleWindowRecord,
    ) -> Result<std::path::PathBuf> {
        let prior_memory_path = root
            .join("generated")
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
                artifact_path: root.join("seed_intent_ir.json"),
                document_key: "seed_doc".to_string(),
                display_name: "Seed Doc".to_string(),
                protocol_family,
                overall_score: Some(100),
                grade: Some("EXCELLENT".to_string()),
                accepted_for_learning: true,
                skip_reason: None,
            }],
            actor_taxonomy_priors: Vec::new(),
            semantic_phrase_priors: Vec::new(),
            semantic_modality_reliability_priors: Vec::new(),
            temporal_phrase_priors: vec![TemporalPhrasePriorRecord {
                prior_id: "temporal_phrase_prior_0001".to_string(),
                normalized_phrase: normalized_phrase.to_string(),
                protocol_family,
                cycle_window: Some(cycle_window),
                actor_grounded: false,
                handshake_completion: false,
                support_count: 1,
                supporting_document_keys: vec!["seed_doc".to_string()],
                strongest_automation_confidence: AutomationConfidence::High,
            }],
            table_shape_priors: Vec::new(),
            visual_motif_priors: Vec::new(),
            negative_knowledge_priors: Vec::new(),
            extraction_profile_priors: Vec::new(),
        };
        fs::write(
            &prior_memory_path,
            serde_json::to_string_pretty(&corpus_memory)?,
        )?;
        Ok(prior_memory_path)
    }

    fn write_semantic_modality_reliability_prior_memory(
        root: &std::path::Path,
        protocol_family: ProtocolFamily,
        role: InterfaceSignalSemanticRole,
        source_kind: SignalSemanticHintSourceKind,
        support_count: usize,
        strongest_grounding_strength: SemanticGroundingStrength,
    ) -> Result<std::path::PathBuf> {
        let prior_memory_path = root
            .join("generated")
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
                artifact_path: root.join("seed_intent_ir.json"),
                document_key: "seed_doc".to_string(),
                display_name: "Seed Doc".to_string(),
                protocol_family,
                overall_score: Some(100),
                grade: Some("EXCELLENT".to_string()),
                accepted_for_learning: true,
                skip_reason: None,
            }],
            actor_taxonomy_priors: Vec::new(),
            semantic_phrase_priors: Vec::new(),
            semantic_modality_reliability_priors: vec![SemanticModalityReliabilityPriorRecord {
                prior_id: "semantic_modality_reliability_prior_0001".to_string(),
                role,
                protocol_family,
                source_kind,
                support_count,
                supporting_document_keys: vec!["seed_doc".to_string()],
                strongest_automation_confidence: AutomationConfidence::High,
                strongest_grounding_strength,
            }],
            temporal_phrase_priors: Vec::new(),
            table_shape_priors: Vec::new(),
            visual_motif_priors: Vec::new(),
            negative_knowledge_priors: Vec::new(),
            extraction_profile_priors: Vec::new(),
        };
        fs::write(
            &prior_memory_path,
            serde_json::to_string_pretty(&corpus_memory)?,
        )?;
        Ok(prior_memory_path)
    }

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
                .any(|actor| actor.actor_id == "actor_transmitter"
                    && !actor.supporting_statement_ids.is_empty())
        );
        assert!(
            semantic_ir
                .actors
                .iter()
                .any(|actor| actor.actor_id == "actor_receiver"
                    && !actor.supporting_statement_ids.is_empty())
        );
        assert!(semantic_ir.interfaces.iter().any(|interface| {
            interface.signals.contains(&"VALID".to_string())
                && interface.signals.contains(&"READY".to_string())
        }));
        assert!(semantic_ir.invariants.iter().any(|invariant| {
            invariant
                .statement
                .contains("VALID must remain asserted until READY is observed.")
                && !invariant.supporting_statement_ids.is_empty()
        }));
        assert!(!semantic_ir.gates.is_empty());
        assert!(
            semantic_ir
                .gates
                .iter()
                .all(|gate| !gate.supporting_statement_ids.is_empty())
        );
        assert!(semantic_ir.residual_decisions.is_empty());

        Ok(())
    }

    #[test]
    fn passive_ambiguous_visual_links_do_not_emit_residual_decision() -> Result<()> {
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
            !semantic_ir
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
    fn emits_residual_decision_for_ambiguous_visual_semantic_grounding() {
        let context = super::SemanticContext {
            statements: vec![super::StatementContext {
                statement_id: "stmt_visual".to_string(),
                class: StatementClass::NormativeStatement,
                text: "Figure 1 shows the transfer acceptance behavior.".to_string(),
                related_visual_evidence_ids: vec!["visual_0001".to_string()],
                section_ids: Vec::new(),
                signals: vec!["XACK".to_string()],
                supporting_table_ids: Vec::new(),
            }],
            section_anchors: Vec::new(),
            visual_roles_by_id: HashMap::from([(
                "visual_0001".to_string(),
                VisualEvidenceRole::Ambiguous,
            )]),
            actor_signal_relations: Vec::new(),
            signal_semantic_hints: Vec::new(),
        };
        let interfaces = vec![super::InterfaceRecord {
            interface_id: "if_req".to_string(),
            signals: vec!["XACK".to_string()],
            signal_records: vec![super::InterfaceSignalRecord {
                signal_name: "XACK".to_string(),
                direction_hint: None,
                width_hint: None,
                resolved_polarity: None,
                semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                semantic_candidates: Vec::new(),
                semantic_arbitration: None,
                resolved_semantic_role: Some(
                    super::InterfaceSignalSemanticRole::HandshakeReadyLike,
                ),
                semantic_grounding_strength: Some(super::SemanticGroundingStrength::SingleSource),
                semantic_consensus: None,
                semantic_observations: vec![super::InterfaceSignalSemanticObservationRecord {
                    semantic_tags: vec![SignalSemanticTag::HandshakeReadyLike],
                    source_kind: SignalSemanticHintSourceKind::VisualCaption,
                    source_text: "XACK can sink the transfer".to_string(),
                    supporting_statement_ids: vec!["stmt_visual".to_string()],
                    supporting_table_ids: Vec::new(),
                    supporting_visual_evidence_ids: vec!["visual_0001".to_string()],
                    automation_confidence: AutomationConfidence::Low,
                }],
                supporting_statement_ids: vec!["stmt_visual".to_string()],
                supporting_table_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Low,
            }],
            supporting_statement_ids: vec!["stmt_visual".to_string()],
        }];

        let packets = super::build_residual_decisions(&context, &interfaces, 1, &[]);
        assert!(
            packets
                .iter()
                .any(|packet| { packet.packet_id == "semantic_ambiguous_visual_grounding" })
        );
    }

    #[test]
    fn emits_residual_decision_for_resolved_semantic_roles_without_consensus() {
        let context = super::SemanticContext {
            statements: Vec::new(),
            section_anchors: Vec::new(),
            visual_roles_by_id: HashMap::new(),
            actor_signal_relations: Vec::new(),
            signal_semantic_hints: Vec::new(),
        };
        let interfaces = vec![super::InterfaceRecord {
            interface_id: "if_req".to_string(),
            signals: vec!["XREQ".to_string()],
            signal_records: vec![super::InterfaceSignalRecord {
                signal_name: "XREQ".to_string(),
                direction_hint: None,
                width_hint: None,
                resolved_polarity: None,
                semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                semantic_candidates: Vec::new(),
                semantic_arbitration: None,
                resolved_semantic_role: Some(
                    super::InterfaceSignalSemanticRole::HandshakeValidLike,
                ),
                semantic_grounding_strength: None,
                semantic_consensus: None,
                semantic_observations: Vec::new(),
                supporting_statement_ids: Vec::new(),
                supporting_table_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            }],
            supporting_statement_ids: Vec::new(),
        }];

        let packets = super::build_residual_decisions(&context, &interfaces, 1, &[]);
        assert!(
            packets
                .iter()
                .any(|packet| { packet.packet_id == "semantic_resolved_role_without_consensus" })
        );
    }

    #[test]
    fn provisional_semantic_roles_do_not_drive_handshake_role_context() {
        let interfaces = vec![super::InterfaceRecord {
            interface_id: "if_req".to_string(),
            signals: vec!["XVALID".to_string()],
            signal_records: vec![super::InterfaceSignalRecord {
                signal_name: "XVALID".to_string(),
                direction_hint: None,
                width_hint: None,
                resolved_polarity: None,
                semantic_tags: vec![SignalSemanticTag::HandshakeValidLike],
                semantic_candidates: Vec::new(),
                semantic_arbitration: None,
                resolved_semantic_role: Some(
                    super::InterfaceSignalSemanticRole::HandshakeValidLike,
                ),
                semantic_grounding_strength: None,
                semantic_consensus: None,
                semantic_observations: Vec::new(),
                supporting_statement_ids: Vec::new(),
                supporting_table_ids: Vec::new(),
                automation_confidence: AutomationConfidence::Medium,
            }],
            supporting_statement_ids: Vec::new(),
        }];

        let context = super::handshake_role_context(&interfaces);

        assert_eq!(
            super::classify_handshake_signal("XVALID", &context),
            None,
            "fallback-only semantic roles should not drive typed handshake classification"
        );
        assert!(
            context.heuristic_blocked_signals.contains("XVALID"),
            "handshake-shaped signals with provisional semantic roles should block raw name fallback"
        );
    }

    #[test]
    fn extracts_typed_interface_signals_from_explicit_markdown() -> Result<()> {
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
        assert!(!explicit_interface.supporting_statement_ids.is_empty());
        assert!(explicit_interface.signal_records.iter().any(|signal| {
            signal.signal_name == "DATA_IN"
                && signal.direction_hint == Some(InterfaceSignalDirection::Input)
                && signal.width_hint == Some(WidthHint::Numeric(8))
                && !signal.supporting_statement_ids.is_empty()
        }));
        assert!(explicit_interface.signal_records.iter().any(|signal| {
            signal.signal_name == "ZERO_FLAG"
                && signal.direction_hint == Some(InterfaceSignalDirection::Output)
                && signal.width_hint == Some(WidthHint::Numeric(1))
                && !signal.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn projects_protocol_surfaces_losslessly_and_keeps_empty_schema_additive() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("protocol_projection.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        fs::write(&source, "# Protocol projection fixture\n")?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.serial_frame_fields = vec![
            SerialFrameField {
                field_id: "serial_field_0001".to_string(),
                name: "REQUEST".to_string(),
                bit_width: Some(1),
                bit_range: Some((0, 0)),
                phase: Some(SerialFramePhase::Request),
                swdio_direction: Some(SwdioDirection::HostDrives),
                order: Some(0),
                response_values: Vec::new(),
                supporting_statement_ids: vec!["statement_request".to_string()],
            },
            SerialFrameField {
                field_id: "serial_field_0002".to_string(),
                name: "ACK".to_string(),
                bit_width: Some(3),
                bit_range: Some((2, 0)),
                phase: Some(SerialFramePhase::Acknowledge),
                swdio_direction: Some(SwdioDirection::TargetDrives),
                order: Some(1),
                response_values: vec!["OK".to_string(), "WAIT".to_string()],
                supporting_statement_ids: vec!["statement_ack".to_string()],
            },
        ];
        evidence_ir.swd_operations = vec![SwdOperation {
            operation_id: "swd_operation_0001".to_string(),
            response: "OK".to_string(),
            access: Some("write".to_string()),
            phase_count: 3,
            has_data_phase: true,
            turnaround_before_data: Some(true),
            supporting_statement_ids: vec!["statement_operation".to_string()],
        }];
        evidence_ir.protocol_states = vec![ProtocolStateRecord {
            state_id: "protocol_state_0001".to_string(),
            machine_name: Some("serial machine".to_string()),
            state_name: "Operating".to_string(),
            action: Some("transfers data".to_string()),
            supporting_statement_ids: vec!["statement_state".to_string()],
        }];
        evidence_ir.interface_edge_timings = vec![InterfaceEdgeTimingRecord {
            timing_id: "interface_edge_timing_0001".to_string(),
            actor_name: "target".to_string(),
            signal_name: "DATA".to_string(),
            clock_signal: "CLK".to_string(),
            edge: InterfaceClockEdge::Rising,
            samples_on_edge: true,
            drive_changes_on_edge: true,
            supporting_statement_ids: vec!["statement_edge".to_string()],
        }];
        let expected_frame_fields = evidence_ir.serial_frame_fields.clone();
        let expected_operations = evidence_ir.swd_operations.clone();
        let expected_states = evidence_ir.protocol_states.clone();
        let expected_edge_timings = evidence_ir.interface_edge_timings.clone();
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        assert_eq!(semantic_ir.serial_frame_fields, expected_frame_fields);
        assert_eq!(semantic_ir.swd_operations, expected_operations);
        assert_eq!(semantic_ir.protocol_states, expected_states);
        assert_eq!(semantic_ir.interface_edge_timings, expected_edge_timings);

        let encoded = serde_json::to_value(&semantic_ir)?;
        assert_eq!(
            encoded.get("serial_frame_fields"),
            Some(&serde_json::to_value(&expected_frame_fields)?)
        );
        assert_eq!(
            encoded.get("swd_operations"),
            Some(&serde_json::to_value(&expected_operations)?)
        );
        assert_eq!(
            encoded.get("protocol_states"),
            Some(&serde_json::to_value(&expected_states)?)
        );
        assert_eq!(
            encoded.get("interface_edge_timings"),
            Some(&serde_json::to_value(&expected_edge_timings)?)
        );

        let mut empty = semantic_ir;
        empty.serial_frame_fields.clear();
        empty.swd_operations.clear();
        empty.protocol_states.clear();
        empty.interface_edge_timings.clear();
        let legacy_shape = serde_json::to_value(&empty)?;
        for field in [
            "serial_frame_fields",
            "swd_operations",
            "protocol_states",
            "interface_edge_timings",
        ] {
            assert!(
                legacy_shape.get(field).is_none(),
                "{field} must omit while empty"
            );
        }
        let decoded: SemanticIr = serde_json::from_value(legacy_shape)?;
        assert!(decoded.serial_frame_fields.is_empty());
        assert!(decoded.swd_operations.is_empty());
        assert!(decoded.protocol_states.is_empty());
        assert!(decoded.interface_edge_timings.is_empty());

        Ok(())
    }

    #[test]
    fn extracts_system_contract_from_explicit_markdown() -> Result<()> {
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
        assert!(!system_contract.supporting_statement_ids.is_empty());

        Ok(())
    }

    #[test]
    fn system_contract_signals_become_explicit_interface_records() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("system_contract_interface.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# Contract\nClock HCLK.\n\nReset HRESETN is asynchronous active low.\n",
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

        let interface = semantic_ir
            .interfaces
            .iter()
            .find(|interface| interface.interface_id == "interface_explicit_document_interface")
            .expect("expected explicit document interface");
        assert!(!interface.supporting_statement_ids.is_empty());
        let hclk = interface
            .signal_records
            .iter()
            .find(|signal| signal.signal_name == "HCLK")
            .expect("expected HCLK interface record");
        assert_eq!(hclk.direction_hint, Some(InterfaceSignalDirection::Input));
        assert_eq!(hclk.width_hint, Some(WidthHint::Numeric(1)));
        assert_eq!(hclk.automation_confidence, AutomationConfidence::High);
        assert!(!hclk.supporting_statement_ids.is_empty());
        let hresetn = interface
            .signal_records
            .iter()
            .find(|signal| signal.signal_name == "HRESETN")
            .expect("expected HRESETN interface record");
        assert_eq!(
            hresetn.direction_hint,
            Some(InterfaceSignalDirection::Input)
        );
        assert_eq!(hresetn.width_hint, Some(WidthHint::Numeric(1)));
        assert_eq!(hresetn.resolved_polarity, Some(SignalPolarity::ActiveLow));
        assert_eq!(hresetn.automation_confidence, AutomationConfidence::High);
        assert!(!hresetn.supporting_statement_ids.is_empty());

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
        assert!(!system_contract.supporting_statement_ids.is_empty());

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
            "# Rich Explicit Control\nSignal MODE is input width 2.\n\nSignal GO is input width 1.\n\nSignal ACC is output width 8.\n\nSignal PULSE_OUT is output width 1.\n\nConstant STEP = 8'1.\n\nParam RESET_VALUE = 8'0.\n\nEnum mode_t idle = 0.\n\nEnum mode_t busy = 1.\n\nDefine LATE_VAL = 1.\n\nState idle is initial.\n\nState busy.\n\nBlock decode select MODE when MODE == mode_t.idle: public ACC = 8'0; transition idle.\n\nSyncReset clear_acc: ACC <- RESET_VALUE.\n\nAsyncReset clear_pulse: public PULSE_OUT = 0.\n\nBlock busy when GO: next ACC <- ACC + STEP; pulse public PULSE_OUT after 2 = 1; ACC += STEP; -> idle.\n",
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

        assert_eq!(semantic_ir.symbol_definitions.len(), 4);
        assert!(semantic_ir.symbol_definitions.iter().any(|definition| {
            definition.symbol_name == "STEP"
                && definition.kind == SymbolDefinitionKind::Constant
                && definition.declaration_order == 0
                && matches!(
                    definition.value.as_ref(),
                    Some(ControlExpressionRecord::Literal { literal }) if literal == "8'1"
                )
                && definition.automation_confidence == AutomationConfidence::High
                && !definition.supporting_statement_ids.is_empty()
        }));
        assert!(semantic_ir.symbol_definitions.iter().any(|definition| {
            definition.symbol_name == "RESET_VALUE"
                && definition.kind == SymbolDefinitionKind::Param
                && definition.declaration_order == 1
                && definition.automation_confidence == AutomationConfidence::High
        }));
        assert!(semantic_ir.symbol_definitions.iter().any(|definition| {
            definition.symbol_name == "LATE_VAL"
                && definition.kind == SymbolDefinitionKind::Define
                && definition.declaration_order == 3
                && definition.automation_confidence == AutomationConfidence::High
        }));
        assert!(semantic_ir.symbol_definitions.iter().any(|definition| {
            definition.symbol_name == "mode_t"
                && definition.kind == SymbolDefinitionKind::Enum
                && definition.declaration_order == 2
                && definition.members.len() == 2
                && definition.members.iter().any(|member| {
                    member.member_name == "idle"
                        && member.automation_confidence == AutomationConfidence::High
                        && !member.supporting_statement_ids.is_empty()
                })
                && definition.automation_confidence == AutomationConfidence::High
                && !definition.supporting_statement_ids.is_empty()
        }));

        let decode_block = semantic_ir
            .control_blocks
            .iter()
            .find(|block| block.block_name == "decode")
            .expect("decode control block should be present");
        assert_eq!(decode_block.role, ControlBlockRole::StandaloneDecisionTree);
        assert_eq!(
            decode_block.automation_confidence,
            AutomationConfidence::High
        );
        assert!(!decode_block.supporting_statement_ids.is_empty());
        assert!(matches!(
            decode_block.selector.as_ref(),
            Some(ControlExpressionRecord::Reference { reference })
                if reference.base_name == "MODE"
                    && reference.kind_hint == ControlReferenceKind::Signal
        ));
        let decode_branch = decode_block
            .branches
            .first()
            .expect("decode block should have a first branch");
        assert_eq!(
            decode_branch.automation_confidence,
            AutomationConfidence::High
        );
        assert!(!decode_branch.supporting_statement_ids.is_empty());
        assert!(matches!(
            decode_branch.predicate.as_ref(),
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
    fn conflicting_symbol_definitions_are_excluded() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("conflict_symbols.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# Symbols\nSignal CLK is input width 1.\n\nConstant STEP = 8'1.\n\nConstant STEP = 8'2.\n",
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

        assert!(
            !semantic_ir
                .symbol_definitions
                .iter()
                .any(|d| d.symbol_name == "STEP"),
            "conflicting STEP definitions must be excluded"
        );

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
                    .control_blocks
                    .iter()
                    .any(|block| block.block_name == "produce")
                && module.automation_confidence == AutomationConfidence::High
                && !module.supporting_statement_ids.is_empty()
        }));
        assert!(semantic_ir.explicit_modules.iter().any(|module| {
            module.module_name == "consumer_core"
                && module.interfaces.iter().any(|interface| {
                    interface.signal_records.iter().any(|signal| {
                        signal.signal_name == "input_data"
                            && signal.direction_hint == Some(InterfaceSignalDirection::Input)
                            && signal.width_hint == Some(WidthHint::Numeric(8))
                            && !signal.supporting_statement_ids.is_empty()
                    })
                })
                && module.automation_confidence == AutomationConfidence::High
                && !module.supporting_statement_ids.is_empty()
        }));

        let explicit_top = semantic_ir
            .explicit_tops
            .iter()
            .find(|top| top.top_name == "datapath")
            .expect("explicit top should be present");
        assert_eq!(explicit_top.ports.len(), 1);
        assert_eq!(explicit_top.children.len(), 2);
        assert_eq!(explicit_top.links.len(), 2);
        assert_eq!(
            explicit_top.automation_confidence,
            AutomationConfidence::High
        );
        assert!(!explicit_top.supporting_statement_ids.is_empty());
        assert!(explicit_top.ports.iter().any(|port| {
            port.port_name == "result_data"
                && port.direction_hint == Some(InterfaceSignalDirection::Output)
                && port.width_hint == Some(WidthHint::Numeric(8))
                && port.automation_confidence == AutomationConfidence::High
                && !port.supporting_statement_ids.is_empty()
        }));
        assert!(explicit_top.children.iter().any(|child| {
            child.instance_name == "producer"
                && child.source_module_name == "producer_core"
                && child.automation_confidence == AutomationConfidence::High
                && !child.supporting_statement_ids.is_empty()
        }));
        assert!(explicit_top.links.iter().any(|link| {
            link.source.instance_name.as_deref() == Some("consumer")
                && link.source.signal_name == "result_data"
                && link.target.instance_name.is_none()
                && link.target.signal_name == "result_data"
                && link.automation_confidence == AutomationConfidence::High
                && !link.supporting_statement_ids.is_empty()
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
            state.state_name == "idle"
                && state.is_initial
                && state.declaration_order == 0
                && state.automation_confidence == AutomationConfidence::High
                && !state.supporting_statement_ids.is_empty()
        }));
        assert!(semantic_ir.regular_states.iter().any(|state| {
            state.state_name == "busy"
                && !state.is_initial
                && state.declaration_order == 1
                && state.automation_confidence == AutomationConfidence::High
                && !state.supporting_statement_ids.is_empty()
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
                && transition.automation_confidence == AutomationConfidence::High
                && !transition.supporting_statement_ids.is_empty()
        }));
        assert!(semantic_ir.state_transitions.iter().any(|transition| {
            transition.source_state == "busy"
                && transition.target_state == "idle"
                && matches!(
                    transition.guard.as_ref(),
                    Some(DecisionTreeGuardRecord::SignalIsHigh { signal_name })
                        if signal_name == "DONE"
                )
                && transition.automation_confidence == AutomationConfidence::High
                && !transition.supporting_statement_ids.is_empty()
        }));

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

        fs::write(
            &source,
            concat!(
                "# Timing\n",
                "Signal XREQ is input width 1.\n\n",
                "Signal clk is input width 1.\n\n",
                "Clock clk.\n",
            ),
        )?;

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
                "vlm_timing_diagram_extraction: {\"signals\":[{\"name\":\"XREQ\",\"values\":[{\"cycle\":\"T1\",\"state\":\"HIGH\"}]},{\"name\":\"transfer\",\"values\":[{\"cycle\":\"T1\",\"state\":\"HIGH\"}]}],\"annotations\":[\"tSU = 2 ns\",\"tHD = 1 ns\"]}"
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
                    && tc.automation_confidence == AutomationConfidence::Medium
            }),
            "expected timing constraint from VLM annotation 'tSU = 2 ns'"
        );
        assert!(
            semantic_ir.timing_constraints.iter().any(|tc| {
                tc.description
                    .as_deref()
                    .map(|d| d.contains("tHD"))
                    .unwrap_or(false)
                    && tc.automation_confidence == AutomationConfidence::Medium
            }),
            "expected timing constraint from VLM annotation 'tHD = 1 ns'"
        );
        assert!(
            semantic_ir.signal_constraints.iter().any(|constraint| {
                constraint.constraint_id.starts_with("vlm_signal_value_")
                    && constraint.subject_signal == "XREQ"
                    && matches!(constraint.constraint_kind, SignalConstraintKind::MustBeHigh)
                    && constraint.source_text.contains("cycle T1")
                    && constraint.automation_confidence == AutomationConfidence::Medium
            }),
            "expected VLM timing signal/value tuple to become a grounded signal constraint"
        );
        assert!(
            !semantic_ir
                .signal_constraints
                .iter()
                .any(|constraint| constraint.subject_signal == "transfer"),
            "generic timing-diagram words must not become VLM-authored signal constraints"
        );
        assert!(
            semantic_ir.temporal_rules.iter().any(|rule| {
                rule.supporting_statement_ids
                    .iter()
                    .any(|support| support == "visual_0001")
                    && rule.consequents.iter().any(|predicate| {
                        matches!(
                            predicate,
                            super::TemporalPredicateRecord::SignalValue {
                                signal_name,
                                value,
                                phase: super::TickPhase::PostTick,
                            } if signal_name == "XREQ" && value == "HIGH"
                        )
                    })
            }),
            "expected VLM timing signal/value tuple to feed a temporal SignalValue predicate"
        );
        let vlm_signal_value_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.supporting_statement_ids
                    .iter()
                    .any(|support| support == "visual_0001")
                    && rule.consequents.iter().any(|predicate| {
                        matches!(
                            predicate,
                            super::TemporalPredicateRecord::SignalValue {
                                signal_name,
                                value,
                                ..
                            } if signal_name == "XREQ" && value == "HIGH"
                        )
                    })
            })
            .expect("expected VLM timing tuple temporal rule");
        assert_eq!(
            vlm_signal_value_rule.cycle_window,
            Some(CycleWindowRecord {
                min_cycles: Some(1),
                max_cycles: Some(1),
            }),
            "diagram cycle label T1 should survive as a bounded cycle window"
        );
        assert_eq!(
            vlm_signal_value_rule.automation_confidence,
            AutomationConfidence::Medium
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

        fs::write(
            &source,
            "# State Machine\nSignal PREADY is input width 1.\n",
        )?;

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
    fn vlm_state_machine_observation_merges_duplicate_state_initial_markers() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("sm_duplicate_state_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# State Machine\nSignal PREADY is input width 1.\n",
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0005".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 5-4 Duplicate state marker".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_state_machine_extraction: {\"states\":[{\"name\":\"IDLE\",\"is_initial\":false},{\"name\":\"BUSY\",\"is_initial\":false},{\"name\":\"IDLE\",\"is_initial\":true}],\"transitions\":[{\"from\":\"IDLE\",\"to\":\"BUSY\",\"guard\":\"PREADY = 1\"}]}"
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

        let idle_states = semantic_ir
            .regular_states
            .iter()
            .filter(|state| state.state_name == "IDLE")
            .collect::<Vec<_>>();
        assert_eq!(
            idle_states.len(),
            1,
            "duplicate VLM state labels must collapse into one canonical state"
        );
        assert!(
            idle_states[0].is_initial,
            "duplicate VLM state labels must preserve an initial marker if any duplicate carries it"
        );
        assert_eq!(
            semantic_ir
                .regular_states
                .iter()
                .filter(|state| state.state_name == "BUSY")
                .count(),
            1,
            "expected BUSY to survive once"
        );
        assert!(
            semantic_ir
                .state_transitions
                .iter()
                .any(|transition| transition.source_state == "IDLE"
                    && transition.target_state == "BUSY"),
            "expected transition between deduped states to survive"
        );

        Ok(())
    }

    #[test]
    fn vlm_state_machine_observation_rejects_undeclared_transition_endpoints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("sm_undeclared_endpoint_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# State Machine\nSignal PREADY is input width 1.\n",
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0004".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 5-3 Partial transfer state machine".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_state_machine_extraction: {\"states\":[{\"name\":\"IDLE\",\"is_initial\":true},{\"name\":\"BUSY\",\"is_initial\":false}],\"transitions\":[{\"from\":\"IDLE\",\"to\":\"BUSY\",\"guard\":\"PREADY = 1\"},{\"from\":\"BUSY\",\"to\":\"DONE\",\"guard\":\"PREADY = 1\"},{\"from\":\"RESET\",\"to\":\"IDLE\",\"guard\":\"PREADY = 1\"}]}"
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

        assert!(
            semantic_ir
                .regular_states
                .iter()
                .any(|state| state.state_name == "IDLE" && state.is_initial),
            "expected valid IDLE state to survive"
        );
        assert!(
            semantic_ir
                .regular_states
                .iter()
                .any(|state| state.state_name == "BUSY"),
            "expected valid BUSY state to survive"
        );
        assert!(
            semantic_ir
                .state_transitions
                .iter()
                .any(|transition| transition.source_state == "IDLE"
                    && transition.target_state == "BUSY"),
            "expected declared IDLE to BUSY transition to survive"
        );
        assert!(
            !semantic_ir.state_transitions.iter().any(|transition| {
                transition.target_state == "DONE" || transition.source_state == "RESET"
            }),
            "VLM transitions with undeclared endpoints must not survive: {:?}",
            semantic_ir.state_transitions
        );
        assert_eq!(
            semantic_ir.state_transitions.len(),
            1,
            "only the transition whose endpoints were declared in the VLM state list should survive"
        );

        Ok(())
    }

    #[test]
    fn vlm_state_machine_observation_rejects_non_identifier_state_labels() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("sm_label_noise_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# State Machine\nSignal PREADY is input width 1.\n",
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0003".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 5-2 Noisy transfer state machine".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_state_machine_extraction: {\"states\":[{\"name\":\"IDLE\",\"is_initial\":true},{\"name\":\"IDLE state\",\"is_initial\":false},{\"name\":\"BUSY\",\"is_initial\":false},{\"name\":\"ACCESS phase\",\"is_initial\":false}],\"transitions\":[{\"from\":\"IDLE\",\"to\":\"BUSY\",\"guard\":\"PREADY = 1\"},{\"from\":\"IDLE state\",\"to\":\"BUSY\",\"guard\":\"PREADY = 1\"},{\"from\":\"BUSY\",\"to\":\"ACCESS phase\",\"guard\":\"PREADY = 1\"}]}"
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

        assert!(
            semantic_ir
                .regular_states
                .iter()
                .any(|state| state.state_name == "IDLE" && state.is_initial),
            "expected valid IDLE state to survive"
        );
        assert!(
            semantic_ir
                .regular_states
                .iter()
                .any(|state| state.state_name == "BUSY"),
            "expected valid BUSY state to survive"
        );
        assert!(
            !semantic_ir
                .regular_states
                .iter()
                .any(|state| state.state_name == "IDLE state"
                    || state.state_name == "ACCESS phase"),
            "VLM prose state labels must not become canonical states: {:?}",
            semantic_ir.regular_states
        );
        assert!(
            semantic_ir
                .state_transitions
                .iter()
                .any(|transition| transition.source_state == "IDLE"
                    && transition.target_state == "BUSY"),
            "expected valid IDLE to BUSY transition to survive"
        );
        assert!(
            !semantic_ir.state_transitions.iter().any(|transition| {
                transition.source_state == "IDLE state" || transition.target_state == "ACCESS phase"
            }),
            "VLM prose transition endpoints must not become canonical transitions: {:?}",
            semantic_ir.state_transitions
        );

        Ok(())
    }

    #[test]
    fn vlm_timing_diagram_observation_accepts_fenced_json_with_trailing_prose() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("timing_fenced_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# Timing\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0003".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 3-2 Write transfer with wait states".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_timing_diagram_extraction: ```json\n{\n  \"signals\": [\n    {\n      \"name\": \"PCLK\",\n      \"values\": [\n        {\"cycle\": \"T4\", \"state\": \"HIGH\"}\n      ]\n    }\n  ],\n  \"annotations\": [\n    \"setup time of data signal during T4\",\n    \"hold time of data signal during T4\"\n  ]\n}\n```\nThe waveform also highlights the transfer boundary around T4."
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

        assert!(
            semantic_ir.timing_constraints.iter().any(|tc| {
                tc.description
                    .as_deref()
                    .map(|description| description.contains("setup time"))
                    .unwrap_or(false)
                    && tc.automation_confidence == AutomationConfidence::Medium
            }),
            "expected timing constraint from fenced VLM annotation"
        );
        assert!(
            semantic_ir.timing_constraints.iter().any(|tc| {
                tc.description
                    .as_deref()
                    .map(|description| description.contains("hold time"))
                    .unwrap_or(false)
                    && tc.automation_confidence == AutomationConfidence::Medium
            }),
            "expected timing constraint from fenced VLM annotation"
        );

        Ok(())
    }

    #[test]
    fn vlm_timing_diagram_observation_rejects_label_only_noise() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("timing_noise_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# Timing\nSignal XREQ is input width 1.\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0004".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 3-3 Transfer timing".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_timing_diagram_extraction: {\"signals\":[{\"name\":\"XREQ\",\"values\":[{\"cycle\":\"T0\",\"state\":\"LOW\"},{\"cycle\":\"T1\",\"state\":\"HIGH\"}]}],\"annotations\":[\"XREQ\",\"XREQ HIGH\",\"XREQ LOW\",\"XREQ asserted\",\"XREQ deasserted\",\"T0\",\"Addr 1\",\"Cycle 2\",\"Burst 1\",\"Packet 2\",\"Frame 3\",\"Transaction 4\",\"Txn 5\",\"Burst1\",\"Packet2\",\"Frame3\",\"Transaction4\",\"Txn5\",\"Phase1\",\"Transfer2\",\"Channel 1 Phase 2\",\"Lane 0 Slot 1\",\"D0\",\"A1\",\"DATA0\",\"0xAA\",\"D[0]\",\"A[1]\",\"DATA[3]\",\"ADDR[7]\",\"XREQ[0]\",\"XREQ<1>\",\"XREQ[3:0]\"]}"
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

        assert!(
            semantic_ir.timing_constraints.is_empty(),
            "label-only timing annotations must not become timing constraints: {:?}",
            semantic_ir.timing_constraints
        );

        Ok(())
    }

    #[test]
    fn vlm_timing_diagram_observation_rejects_indexed_signal_value_labels() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("timing_indexed_value_noise_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# Timing\nSignal XREQ is input width 1.\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0004b".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 3-3b Transfer timing".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_timing_diagram_extraction: {\"signals\":[{\"name\":\"XREQ\",\"values\":[{\"cycle\":\"T0\",\"state\":\"LOW\"},{\"cycle\":\"T1\",\"state\":\"HIGH\"}]}],\"annotations\":[\"XREQ[0] HIGH\",\"XREQ<1> LOW\",\"XREQ[3:0] asserted\",\"XREQ[7:4] deasserted\"]}"
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

        assert!(
            semantic_ir.timing_constraints.is_empty(),
            "indexed signal-value timing annotations must not become timing constraints: {:?}",
            semantic_ir.timing_constraints
        );
        let vlm_constraints = semantic_ir
            .signal_constraints
            .iter()
            .filter(|constraint| constraint.constraint_id.starts_with("vlm_signal_value_"))
            .collect::<Vec<_>>();
        assert_eq!(
            vlm_constraints.len(),
            2,
            "only the concrete LOW/HIGH waveform samples should survive as VLM-authored signal constraints: {vlm_constraints:?}"
        );
        assert!(
            vlm_constraints.iter().any(|constraint| {
                constraint.subject_signal == "XREQ"
                    && matches!(constraint.constraint_kind, SignalConstraintKind::MustBeLow)
                    && constraint.source_text.contains("cycle T0")
            }),
            "expected LOW sample at T0 to survive"
        );
        assert!(
            vlm_constraints.iter().any(|constraint| {
                constraint.subject_signal == "XREQ"
                    && matches!(constraint.constraint_kind, SignalConstraintKind::MustBeHigh)
                    && constraint.source_text.contains("cycle T1")
            }),
            "expected HIGH sample at T1 to survive"
        );

        Ok(())
    }

    #[test]
    fn vlm_timing_diagram_observation_rejects_cycle_qualified_signal_value_labels() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("timing_cycle_qualified_value_noise_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# Timing\nSignal XREQ is input width 1.\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0004c".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 3-3c Transfer timing".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_timing_diagram_extraction: {\"signals\":[{\"name\":\"XREQ\",\"values\":[{\"cycle\":\"T0\",\"state\":\"LOW\"},{\"cycle\":\"T1\",\"state\":\"HIGH\"}]}],\"annotations\":[\"XREQ HIGH at T1\",\"XREQ LOW during T0\",\"XREQ asserted on T1\",\"XREQ deasserted in T0\"]}"
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

        assert!(
            semantic_ir.timing_constraints.is_empty(),
            "cycle-qualified signal-value timing annotations must not become timing constraints: {:?}",
            semantic_ir.timing_constraints
        );
        let vlm_constraints = semantic_ir
            .signal_constraints
            .iter()
            .filter(|constraint| constraint.constraint_id.starts_with("vlm_signal_value_"))
            .collect::<Vec<_>>();
        assert_eq!(
            vlm_constraints.len(),
            2,
            "only the concrete LOW/HIGH waveform samples should survive as VLM-authored signal constraints: {vlm_constraints:?}"
        );
        assert!(
            vlm_constraints.iter().any(|constraint| {
                constraint.subject_signal == "XREQ"
                    && matches!(constraint.constraint_kind, SignalConstraintKind::MustBeLow)
                    && constraint.source_text.contains("cycle T0")
            }),
            "expected LOW sample at T0 to survive"
        );
        assert!(
            vlm_constraints.iter().any(|constraint| {
                constraint.subject_signal == "XREQ"
                    && matches!(constraint.constraint_kind, SignalConstraintKind::MustBeHigh)
                    && constraint.source_text.contains("cycle T1")
            }),
            "expected HIGH sample at T1 to survive"
        );

        Ok(())
    }

    #[test]
    fn vlm_timing_diagram_observation_rejects_waveform_motion_states() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("timing_motion_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# Timing\nSignal XREQ is input width 1.\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0005".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 3-4 Request waveform motion".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_timing_diagram_extraction: {\"signals\":[{\"name\":\"XREQ\",\"values\":[{\"cycle\":\"T0\",\"state\":\"rising\"},{\"cycle\":\"T1\",\"state\":\"HIGH\"},{\"cycle\":\"T2\",\"state\":\"stable\"},{\"cycle\":\"T3\",\"state\":\"falling\"},{\"cycle\":\"T4\",\"state\":\"UNCHANGED\"},{\"cycle\":\"T5\",\"state\":\"RISING_EDGE\"},{\"cycle\":\"T6\",\"state\":\"LOW_TO_HIGH\"},{\"cycle\":\"T7\",\"state\":\"HIGH_TO_LOW\"},{\"cycle\":\"T8\",\"state\":\"POS_EDGE\"},{\"cycle\":\"T9\",\"state\":\"NEG_EDGE\"},{\"cycle\":\"T10\",\"state\":\"risingedge\"},{\"cycle\":\"T11\",\"state\":\"LOW2HIGH\"},{\"cycle\":\"T12\",\"state\":\"HIGH2LOW\"}]}],\"annotations\":[\"XREQ rises, stays stable, then falls\"]}"
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

        assert!(
            semantic_ir.timing_constraints.is_empty(),
            "motion-only timing annotations must not become timing constraints: {:?}",
            semantic_ir.timing_constraints
        );
        let vlm_constraints = semantic_ir
            .signal_constraints
            .iter()
            .filter(|constraint| constraint.constraint_id.starts_with("vlm_signal_value_"))
            .collect::<Vec<_>>();
        assert_eq!(
            vlm_constraints.len(),
            1,
            "only the concrete HIGH sample should become a VLM-authored signal constraint: {vlm_constraints:?}"
        );
        assert!(
            vlm_constraints.iter().any(|constraint| {
                constraint.subject_signal == "XREQ"
                    && matches!(constraint.constraint_kind, SignalConstraintKind::MustBeHigh)
                    && constraint.source_text.contains("cycle T1")
            }),
            "expected the concrete HIGH sample to survive as timing evidence"
        );

        Ok(())
    }

    #[test]
    fn vlm_state_machine_observation_accepts_fenced_json_with_trailing_prose() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("sm_fenced_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# State Machine\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_0004".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure 4-1 State diagram".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_state_machine_extraction: ```json\n{\n  \"states\": [\n    {\"name\": \"IDLE\", \"is_initial\": true},\n    {\"name\": \"SETUP\"},\n    {\"name\": \"ACCESS\"}\n  ],\n  \"transitions\": [\n    {\"from\": \"IDLE\", \"to\": \"SETUP\", \"guard\": \"Transfer\"},\n    {\"from\": \"SETUP\", \"to\": \"ACCESS\", \"guard\": \"PREADY = 1 and transfer\"}\n  ]\n}\n```\nThis state diagram highlights the ACCESS phase."
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

        assert!(
            semantic_ir
                .regular_states
                .iter()
                .any(|state| state.state_name == "IDLE" && state.is_initial),
            "expected IDLE initial state from fenced VLM extraction"
        );
        assert!(
            semantic_ir.state_transitions.iter().any(|transition| {
                transition.source_state == "SETUP" && transition.target_state == "ACCESS"
            }),
            "expected SETUP→ACCESS transition from fenced VLM extraction"
        );
        let idle_setup = semantic_ir
            .state_transitions
            .iter()
            .find(|transition| {
                transition.source_state == "IDLE" && transition.target_state == "SETUP"
            })
            .expect("expected IDLE to SETUP transition from fenced VLM extraction");
        assert_eq!(
            idle_setup.guard, None,
            "generic VLM guard prose must not become a fake signal when declared signals are known"
        );
        let setup_access = semantic_ir
            .state_transitions
            .iter()
            .find(|transition| {
                transition.source_state == "SETUP" && transition.target_state == "ACCESS"
            })
            .expect("expected SETUP to ACCESS transition from fenced VLM extraction");
        assert_eq!(
            setup_access.guard,
            Some(DecisionTreeGuardRecord::Comparison {
                left_signal: "PREADY".to_string(),
                operator: DecisionTreeComparisonOperator::Eq,
                right: DecisionTreeValueRecord::Literal {
                    literal: "1".to_string(),
                },
            }),
            "VLM compound guard should keep the declared signal comparison instead of the whole prose fragment"
        );

        Ok(())
    }

    #[test]
    fn surfaces_interface_signal_conflicts_for_conflicting_explicit_declarations() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("signal_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal DATA is input width 8.\n\n",
                "Signal DATA is output width 16.\n\n",
                "Signal DATA is input width 8.\n",
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

        let data_signal = semantic_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "DATA")
            .expect("expected DATA interface signal");
        assert_eq!(data_signal.direction_hint, None);
        assert_eq!(data_signal.width_hint, None);
        assert_eq!(semantic_ir.interface_signal_conflicts.len(), 2);
        assert!(
            semantic_ir
                .interface_signal_conflicts
                .iter()
                .any(|conflict| {
                    conflict.signal_name == "DATA"
                        && matches!(
                            conflict.conflict_kind,
                            super::InterfaceSignalConflictKind::DirectionMismatch
                        )
                        && conflict
                            .observations
                            .iter()
                            .any(|observation| observation.value_text == "input")
                        && conflict
                            .observations
                            .iter()
                            .any(|observation| observation.value_text == "output")
                        && conflict.automation_confidence == AutomationConfidence::High
                })
        );
        assert!(
            semantic_ir
                .interface_signal_conflicts
                .iter()
                .any(|conflict| {
                    conflict.signal_name == "DATA"
                        && matches!(
                            conflict.conflict_kind,
                            super::InterfaceSignalConflictKind::WidthMismatch
                        )
                        && conflict
                            .observations
                            .iter()
                            .any(|observation| observation.value_text == "8")
                        && conflict
                            .observations
                            .iter()
                            .any(|observation| observation.value_text == "16")
                        && conflict.automation_confidence == AutomationConfidence::High
                })
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
        assert_eq!(haddr.automation_confidence, AutomationConfidence::High);
        assert!(!haddr.supporting_table_ids.is_empty());

        let hwrite = find_signal("HWRITE").expect("HWRITE should be extracted from table");
        assert_eq!(
            hwrite.direction_hint,
            Some(InterfaceSignalDirection::Output)
        );
        assert_eq!(hwrite.width_hint, Some(WidthHint::Numeric(1)));
        assert_eq!(hwrite.automation_confidence, AutomationConfidence::High);
        assert!(!hwrite.supporting_table_ids.is_empty());

        let htrans = find_signal("HTRANS").expect("HTRANS should be extracted from table");
        assert_eq!(
            htrans.direction_hint,
            Some(InterfaceSignalDirection::Output)
        );
        assert_eq!(htrans.width_hint, Some(WidthHint::Numeric(2)));
        assert_eq!(htrans.automation_confidence, AutomationConfidence::High);
        assert!(!htrans.supporting_table_ids.is_empty());

        // Subordinate signals should be extracted as Input with explicit widths.
        let hreadyout = find_signal("HREADYOUT").expect("HREADYOUT should be extracted from table");
        assert_eq!(
            hreadyout.direction_hint,
            Some(InterfaceSignalDirection::Input)
        );
        assert_eq!(hreadyout.width_hint, Some(WidthHint::Numeric(1)));
        assert_eq!(hreadyout.automation_confidence, AutomationConfidence::High);
        assert!(!hreadyout.supporting_table_ids.is_empty());

        let hresp = find_signal("HRESP").expect("HRESP should be extracted from table");
        assert_eq!(hresp.direction_hint, Some(InterfaceSignalDirection::Input));
        assert_eq!(hresp.width_hint, Some(WidthHint::Numeric(1)));
        assert_eq!(hresp.automation_confidence, AutomationConfidence::High);
        assert!(!hresp.supporting_table_ids.is_empty());

        Ok(())
    }

    #[test]
    fn carries_actor_relative_ports_and_signal_connectivity() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("kg_ports.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# Protocol\nSignal PREADY is output width 1.\n\nSignal PADDR is input width 32.\n\nThe Completer drives PREADY.\n\nThe Requester reads PREADY.\n\nThe Requester drives PADDR.\n\nThe Completer samples PADDR.\n",
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

        assert!(
            semantic_ir
                .actors
                .iter()
                .filter_map(|actor| actor.actor_name.as_deref())
                .any(|name| name.eq_ignore_ascii_case("Completer"))
        );
        assert!(
            semantic_ir
                .actors
                .iter()
                .filter_map(|actor| actor.actor_name.as_deref())
                .any(|name| name.eq_ignore_ascii_case("Requester"))
        );
        assert!(
            semantic_ir
                .actors
                .iter()
                .all(|actor| !actor.supporting_statement_ids.is_empty())
        );
        assert!(
            semantic_ir
                .actors
                .iter()
                .all(|actor| !actor.supporting_section_ids.is_empty())
        );

        let completer_pready = semantic_ir
            .actor_ports
            .iter()
            .find(|port| {
                port.actor_name.eq_ignore_ascii_case("Completer") && port.signal_name == "PREADY"
            })
            .expect("expected Completer/PREADY actor-relative port");
        assert_eq!(completer_pready.direction, ActorRelativeDirection::Output);
        assert_eq!(completer_pready.width_hint, Some(WidthHint::Numeric(1)));
        assert_eq!(
            completer_pready.automation_confidence,
            AutomationConfidence::Medium
        );

        let requester_pready = semantic_ir
            .actor_ports
            .iter()
            .find(|port| {
                port.actor_name.eq_ignore_ascii_case("Requester") && port.signal_name == "PREADY"
            })
            .expect("expected Requester/PREADY actor-relative port");
        assert_eq!(requester_pready.direction, ActorRelativeDirection::Input);
        assert_eq!(
            requester_pready.automation_confidence,
            AutomationConfidence::Medium
        );

        let pready_connectivity = semantic_ir
            .signal_connectivity
            .iter()
            .find(|record| record.signal_name == "PREADY")
            .expect("expected signal connectivity for PREADY");
        assert_eq!(pready_connectivity.width_hint, Some(WidthHint::Numeric(1)));
        assert_eq!(
            pready_connectivity.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(
            pready_connectivity
                .producer_actor_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case("Completer"))
        );
        assert!(
            pready_connectivity
                .consumer_actor_names
                .iter()
                .any(|name| name.eq_ignore_ascii_case("Requester"))
        );

        Ok(())
    }

    #[test]
    fn surfaces_signal_connectivity_conflicts_for_multiple_producers() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("kg_producer_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        assert_eq!(semantic_ir.signal_connectivity_conflicts.len(), 1);
        let conflict = &semantic_ir.signal_connectivity_conflicts[0];
        assert_eq!(conflict.signal_name, "PREADY");
        assert!(matches!(
            conflict.conflict_kind,
            super::SignalConnectivityConflictKind::MultipleProducers
        ));
        assert_eq!(conflict.automation_confidence, AutomationConfidence::Medium);
        assert!(!conflict.supporting_statement_ids.is_empty());
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
    fn carries_signal_semantic_conflicts_into_semantic_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_role_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        assert_eq!(semantic_ir.signal_semantic_conflicts.len(), 1);
        let conflict = &semantic_ir.signal_semantic_conflicts[0];
        assert_eq!(conflict.signal_name, "XCTRL");
        assert!(conflict.observations.iter().any(|observation| {
            observation
                .semantic_tags
                .contains(&SignalSemanticTag::HandshakeValidLike)
        }));
        assert!(conflict.observations.iter().any(|observation| {
            observation
                .semantic_tags
                .contains(&SignalSemanticTag::HandshakeReadyLike)
                && !observation.supporting_statement_ids.is_empty()
        }));
        let xctrl = semantic_ir
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
            super::InterfaceSignalSemanticRole::HandshakeValidLike
        );
        assert_eq!(arbitration.leading_evidence_weight, 6);
        assert_eq!(
            arbitration.runner_up_role,
            Some(super::InterfaceSignalSemanticRole::HandshakeReadyLike)
        );
        assert_eq!(arbitration.runner_up_evidence_weight, Some(3));
        assert_eq!(arbitration.margin_over_runner_up, Some(3));
        assert!(!arbitration.decisive);
        assert!(xctrl.semantic_candidates.iter().any(|candidate| {
            candidate.role == super::InterfaceSignalSemanticRole::HandshakeValidLike
                && candidate.evidence_weight == 6
                && candidate.automation_confidence == AutomationConfidence::Medium
        }));
        assert!(xctrl.semantic_candidates.iter().any(|candidate| {
            candidate.role == super::InterfaceSignalSemanticRole::HandshakeReadyLike
                && candidate.evidence_weight == 3
                && candidate.automation_confidence == AutomationConfidence::Low
        }));

        Ok(())
    }

    #[test]
    fn modality_reliability_priors_can_resolve_local_semantic_conflicts() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_role_conflict_with_prior.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let prior_memory_path = write_semantic_modality_reliability_prior_memory(
            tempdir.path(),
            ProtocolFamily::Unknown,
            InterfaceSignalSemanticRole::HandshakeValidLike,
            SignalSemanticHintSourceKind::SignalDescriptionTable,
            3,
            SemanticGroundingStrength::CrossModality,
        )?;

        let evidence_ir = EvidenceIr::build_with_prior_memory(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
            Some(&prior_memory_path),
        )?;
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        assert_eq!(semantic_ir.signal_semantic_conflicts.len(), 1);
        let xctrl = semantic_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XCTRL")
            .expect("expected XCTRL interface signal");
        assert_eq!(
            xctrl.resolved_semantic_role,
            Some(InterfaceSignalSemanticRole::HandshakeValidLike)
        );
        let consensus = xctrl
            .semantic_consensus
            .as_ref()
            .expect("expected prior-guided semantic consensus");
        assert!(consensus.prior_guided);
        assert_eq!(consensus.prior_reliability_adjustment, 2);
        let arbitration = xctrl
            .semantic_arbitration
            .as_ref()
            .expect("expected XCTRL semantic arbitration");
        assert!(arbitration.decisive);
        assert!(matches!(
            arbitration.decision_basis,
            SemanticArbitrationDecisionBasis::PriorGuidedMargin
        ));
        assert_eq!(arbitration.leading_evidence_weight, 6);
        assert_eq!(arbitration.leading_prior_reliability_adjustment, 2);
        assert_eq!(arbitration.leading_arbitration_weight, 8);
        assert_eq!(arbitration.runner_up_evidence_weight, Some(3));
        assert_eq!(arbitration.runner_up_prior_reliability_adjustment, Some(0));
        assert_eq!(arbitration.runner_up_arbitration_weight, Some(3));
        assert_eq!(arbitration.margin_over_runner_up, Some(5));
        assert!(xctrl.semantic_candidates.iter().any(|candidate| {
            candidate.role == InterfaceSignalSemanticRole::HandshakeValidLike
                && candidate.prior_reliability_adjustment == 2
                && candidate.arbitration_weight == 8
        }));

        Ok(())
    }

    #[test]
    fn carries_signal_polarity_conflicts_into_semantic_ir() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_polarity_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        assert_eq!(semantic_ir.signal_polarity_conflicts.len(), 1);
        let conflict = &semantic_ir.signal_polarity_conflicts[0];
        assert_eq!(conflict.signal_name, "PRESETN");
        assert_eq!(conflict.observations.len(), 2);
        assert!(conflict.observations.iter().any(|observation| {
            matches!(
                observation.polarity,
                crate::ir::evidence::SignalPolarity::ActiveHigh
            )
        }));
        assert!(conflict.observations.iter().any(|observation| {
            matches!(
                observation.polarity,
                crate::ir::evidence::SignalPolarity::ActiveLow
            )
        }));

        Ok(())
    }

    #[test]
    fn carries_resolved_signal_polarity_into_interface_records() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_resolved_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let presetn = semantic_ir
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
    fn explicit_asserted_when_level_polarity_does_not_duplicate_interface_records() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_control_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Control\n",
                "Signal CS_N is input width 1.\n",
                "\n",
                "CS_N is asserted when LOW.\n",
                "\n",
                "CS_N must be asserted.\n",
                "\n",
                "CS_N must be deasserted.\n",
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

        let cs_n_records: Vec<_> = semantic_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .filter(|signal| signal.signal_name == "CS_N")
            .collect();
        assert_eq!(
            cs_n_records.len(),
            1,
            "explicit single-signal polarity prose should enrich CS_N, not mint a duplicate heuristic interface record"
        );
        assert_eq!(
            cs_n_records[0].resolved_polarity,
            Some(crate::ir::evidence::SignalPolarity::ActiveLow)
        );

        Ok(())
    }

    #[test]
    fn collective_polarity_prose_does_not_duplicate_interface_records() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("semantic_collective_control_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Control\n",
                "Signal CS_N is input width 1.\n",
                "\n",
                "Signal WE_N is input width 1.\n",
                "\n",
                "CS_N and WE_N are active LOW signals.\n",
                "\n",
                "CS_N must be asserted.\n",
                "\n",
                "WE_N must be deasserted.\n",
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

        for signal_name in ["CS_N", "WE_N"] {
            let records: Vec<_> = semantic_ir
                .interfaces
                .iter()
                .flat_map(|interface| interface.signal_records.iter())
                .filter(|signal| signal.signal_name == signal_name)
                .collect();
            assert_eq!(
                records.len(),
                1,
                "collective polarity prose should enrich {signal_name}, not mint a duplicate heuristic interface record"
            );
            assert_eq!(
                records[0].resolved_polarity,
                Some(crate::ir::evidence::SignalPolarity::ActiveLow)
            );
        }

        Ok(())
    }

    #[test]
    fn carries_semantic_observations_into_interface_records() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_role_observations.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal XREQ is input width 1.\n\n",
                "Signal XACK is input width 1.\n",
            ),
        )?;

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
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_signal_semantic_tags".to_string(),
            asset_id: "table_signal_semantic_tags".to_string(),
            page_id: None,
            caption_text: Some("Handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("XACK", false),
                make_table_cell(
                    "Indicates that the subordinate can accept the transfer.",
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

        let xreq = semantic_ir
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
            Some(super::InterfaceSignalSemanticRole::HandshakeValidLike)
        );
        assert_eq!(
            xreq.semantic_grounding_strength,
            Some(super::SemanticGroundingStrength::SingleSource)
        );
        assert_eq!(xreq.semantic_candidates.len(), 1);
        let xreq_candidate = &xreq.semantic_candidates[0];
        assert_eq!(
            xreq_candidate.role,
            super::InterfaceSignalSemanticRole::HandshakeValidLike
        );
        assert_eq!(
            xreq_candidate.grounding_strength,
            super::SemanticGroundingStrength::SingleSource
        );
        assert_eq!(
            xreq_candidate.automation_confidence,
            AutomationConfidence::Low
        );
        assert_eq!(xreq_candidate.supporting_observation_count, 1);
        assert_eq!(
            xreq_candidate.supporting_source_kinds,
            vec![SignalSemanticHintSourceKind::VisualCaption]
        );
        assert_eq!(
            xreq_candidate.automation_confidence,
            AutomationConfidence::Low
        );
        assert_eq!(xreq_candidate.evidence_weight, 4);
        assert!(!xreq_candidate.alias_dependent);
        let xreq_arbitration = xreq
            .semantic_arbitration
            .as_ref()
            .expect("expected XREQ semantic arbitration");
        assert_eq!(xreq_arbitration.candidate_count, 1);
        assert_eq!(
            xreq_arbitration.leading_role,
            super::InterfaceSignalSemanticRole::HandshakeValidLike
        );
        assert_eq!(xreq_arbitration.leading_evidence_weight, 4);
        assert_eq!(xreq_arbitration.runner_up_role, None);
        assert_eq!(xreq_arbitration.runner_up_evidence_weight, None);
        assert_eq!(xreq_arbitration.margin_over_runner_up, None);
        assert!(xreq_arbitration.decisive);
        let xreq_consensus = xreq
            .semantic_consensus
            .as_ref()
            .expect("expected XREQ semantic consensus");
        assert_eq!(
            xreq_consensus.role,
            super::InterfaceSignalSemanticRole::HandshakeValidLike
        );
        assert_eq!(
            xreq_consensus.grounding_strength,
            super::SemanticGroundingStrength::SingleSource
        );
        assert_eq!(xreq_consensus.supporting_observation_count, 1);
        assert_eq!(
            xreq_consensus.supporting_source_kinds,
            vec![SignalSemanticHintSourceKind::VisualCaption]
        );
        assert_eq!(
            xreq_consensus.automation_confidence,
            AutomationConfidence::Low
        );
        assert!(!xreq_consensus.alias_dependent);
        assert!(xreq.semantic_observations.iter().any(|observation| {
            matches!(
                observation.source_kind,
                SignalSemanticHintSourceKind::VisualCaption
            ) && !observation.supporting_visual_evidence_ids.is_empty()
                && observation.automation_confidence == AutomationConfidence::Low
        }));

        let xack = semantic_ir
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
            Some(super::InterfaceSignalSemanticRole::HandshakeReadyLike)
        );
        assert_eq!(
            xack.semantic_grounding_strength,
            Some(super::SemanticGroundingStrength::SingleSource)
        );
        assert_eq!(xack.semantic_candidates.len(), 1);
        let xack_candidate = &xack.semantic_candidates[0];
        assert_eq!(
            xack_candidate.role,
            super::InterfaceSignalSemanticRole::HandshakeReadyLike
        );
        assert_eq!(
            xack_candidate.grounding_strength,
            super::SemanticGroundingStrength::SingleSource
        );
        assert_eq!(
            xack_candidate.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xack_candidate.supporting_observation_count, 1);
        assert_eq!(
            xack_candidate.supporting_source_kinds,
            vec![SignalSemanticHintSourceKind::SignalDescriptionTable]
        );
        assert_eq!(
            xack_candidate.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xack_candidate.evidence_weight, 6);
        assert!(!xack_candidate.alias_dependent);
        let xack_arbitration = xack
            .semantic_arbitration
            .as_ref()
            .expect("expected XACK semantic arbitration");
        assert_eq!(xack_arbitration.candidate_count, 1);
        assert_eq!(
            xack_arbitration.leading_role,
            super::InterfaceSignalSemanticRole::HandshakeReadyLike
        );
        assert_eq!(xack_arbitration.leading_evidence_weight, 6);
        assert_eq!(xack_arbitration.runner_up_role, None);
        assert_eq!(xack_arbitration.runner_up_evidence_weight, None);
        assert_eq!(xack_arbitration.margin_over_runner_up, None);
        assert!(xack_arbitration.decisive);
        let xack_consensus = xack
            .semantic_consensus
            .as_ref()
            .expect("expected XACK semantic consensus");
        assert_eq!(
            xack_consensus.role,
            super::InterfaceSignalSemanticRole::HandshakeReadyLike
        );
        assert_eq!(
            xack_consensus.grounding_strength,
            super::SemanticGroundingStrength::SingleSource
        );
        assert_eq!(xack_consensus.supporting_observation_count, 1);
        assert_eq!(
            xack_consensus.supporting_source_kinds,
            vec![SignalSemanticHintSourceKind::SignalDescriptionTable]
        );
        assert_eq!(
            xack_consensus.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(!xack_consensus.alias_dependent);
        assert!(xack.semantic_observations.iter().any(|observation| {
            matches!(
                observation.source_kind,
                SignalSemanticHintSourceKind::SignalDescriptionTable
            ) && observation
                .supporting_table_ids
                .contains(&"table_signal_semantic_tags".to_string())
                && observation.automation_confidence == AutomationConfidence::Medium
        }));

        Ok(())
    }

    #[test]
    fn marks_cross_modality_semantic_grounding_on_interface_signals() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_role_multi_source.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_xreq_roles".to_string(),
            asset_id: "table_xreq_roles".to_string(),
            page_id: None,
            caption_text: Some("Handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("XREQ", false),
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

        let xreq = semantic_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XREQ")
            .expect("expected XREQ interface signal");
        assert_eq!(xreq.semantic_observations.len(), 2);
        assert_eq!(
            xreq.resolved_semantic_role,
            Some(super::InterfaceSignalSemanticRole::HandshakeValidLike)
        );
        assert_eq!(
            xreq.semantic_grounding_strength,
            Some(super::SemanticGroundingStrength::CrossModality)
        );
        assert_eq!(xreq.semantic_candidates.len(), 1);
        let xreq_candidate = &xreq.semantic_candidates[0];
        assert_eq!(
            xreq_candidate.grounding_strength,
            super::SemanticGroundingStrength::CrossModality
        );
        assert_eq!(xreq_candidate.supporting_observation_count, 2);
        assert_eq!(
            xreq_candidate.supporting_source_kinds,
            vec![
                SignalSemanticHintSourceKind::SignalDescriptionTable,
                SignalSemanticHintSourceKind::VisualCaption
            ]
        );
        assert_eq!(
            xreq_candidate.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xreq_candidate.evidence_weight, 10);
        let xreq_consensus = xreq
            .semantic_consensus
            .as_ref()
            .expect("expected XREQ semantic consensus");
        assert_eq!(
            xreq_consensus.grounding_strength,
            super::SemanticGroundingStrength::CrossModality
        );
        assert_eq!(xreq_consensus.supporting_observation_count, 2);
        assert_eq!(
            xreq_consensus.supporting_source_kinds,
            vec![
                SignalSemanticHintSourceKind::SignalDescriptionTable,
                SignalSemanticHintSourceKind::VisualCaption
            ]
        );
        assert_eq!(
            xreq_consensus.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(xreq.semantic_observations.iter().any(|observation| {
            matches!(
                observation.source_kind,
                SignalSemanticHintSourceKind::SignalDescriptionTable
            ) && !observation.supporting_table_ids.is_empty()
        }));
        assert!(xreq.semantic_observations.iter().any(|observation| {
            matches!(
                observation.source_kind,
                SignalSemanticHintSourceKind::VisualCaption
            ) && !observation.supporting_visual_evidence_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn marks_same_modality_repetition_as_multi_source_grounding() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_role_same_modality.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# Protocol\nSignal XREQ is output width 1.\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_xreq_roles_1".to_string(),
            asset_id: "table_xreq_roles_1".to_string(),
            page_id: None,
            caption_text: Some("Primary handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("XREQ", false),
                make_table_cell(
                    "Indicates that address and control information are valid for transfer.",
                    false,
                ),
            ]],
            row_count: 1,
            col_count: 2,
        });
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_xreq_roles_2".to_string(),
            asset_id: "table_xreq_roles_2".to_string(),
            page_id: None,
            caption_text: Some("Secondary handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![vec![
                make_table_cell("XREQ", false),
                make_table_cell(
                    "Asserted when transfer information is valid on the channel.",
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

        let xreq = semantic_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XREQ")
            .expect("expected XREQ interface signal");
        assert_eq!(xreq.semantic_observations.len(), 2);
        assert_eq!(
            xreq.resolved_semantic_role,
            Some(super::InterfaceSignalSemanticRole::HandshakeValidLike)
        );
        assert_eq!(
            xreq.semantic_grounding_strength,
            Some(super::SemanticGroundingStrength::MultiSource)
        );
        assert_eq!(xreq.semantic_candidates.len(), 1);
        let xreq_candidate = &xreq.semantic_candidates[0];
        assert_eq!(
            xreq_candidate.grounding_strength,
            super::SemanticGroundingStrength::MultiSource
        );
        assert_eq!(xreq_candidate.supporting_observation_count, 2);
        assert_eq!(
            xreq_candidate.supporting_source_kinds,
            vec![SignalSemanticHintSourceKind::SignalDescriptionTable]
        );
        assert_eq!(
            xreq_candidate.automation_confidence,
            AutomationConfidence::Medium
        );
        assert_eq!(xreq_candidate.evidence_weight, 12);
        let xreq_consensus = xreq
            .semantic_consensus
            .as_ref()
            .expect("expected XREQ semantic consensus");
        assert_eq!(
            xreq_consensus.grounding_strength,
            super::SemanticGroundingStrength::MultiSource
        );
        assert_eq!(xreq_consensus.supporting_observation_count, 2);
        assert_eq!(
            xreq_consensus.supporting_source_kinds,
            vec![SignalSemanticHintSourceKind::SignalDescriptionTable]
        );
        assert_eq!(
            xreq_consensus.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(
            xreq.semantic_observations
                .iter()
                .all(|observation| { !observation.supporting_table_ids.is_empty() })
        );

        Ok(())
    }

    #[test]
    fn derives_temporal_rules_from_constraints_and_clock_context() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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
            source_text: "HTRANS must not change when HREADY is LOW.".to_string(),
            supporting_statement_ids: vec!["stmt_temporal".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_htrans_stable")
            .expect("expected temporal rule derived from signal constraint");
        assert_eq!(rule.clock_signal.as_deref(), Some("clk"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);
        assert!(rule.antecedents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::SignalValue {
                    signal_name,
                    value,
                    phase: super::TickPhase::PreTick,
                } if signal_name == "HREADY" && value == "LOW"
            )
        }));
        assert!(rule.consequents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::SignalStable {
                    signal_name,
                    from_phase: super::TickPhase::PreTick,
                    to_phase: super::TickPhase::PostTick,
                } if signal_name == "HTRANS"
            )
        }));
        assert_eq!(rule.automation_confidence, AutomationConfidence::Medium);
        assert!(!rule.supporting_statement_ids.is_empty());

        Ok(())
    }

    #[test]
    fn derives_cycle_window_from_temporal_constraint_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_cycles.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
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
            constraint_id: "sigcon_pready_latency".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted within 2 cycles.".to_string(),
            supporting_statement_ids: vec!["stmt_cycle_window".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_latency")
            .expect("expected temporal rule derived from cycle-bounded constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'within 2 cycles'");
        assert_eq!(cycle_window.min_cycles, None);
        assert_eq!(cycle_window.max_cycles, Some(2));

        Ok(())
    }

    #[test]
    fn extracts_single_cycle_window_from_idiomatic_clock_tick_phrases() {
        let next_cycle =
            super::extract_cycle_window_from_text("The response must arrive on the next cycle.")
                .expect("expected cycle window from 'next cycle'");
        assert_eq!(next_cycle.min_cycles, Some(1));
        assert_eq!(next_cycle.max_cycles, Some(1));

        let next_clock_cycle = super::extract_cycle_window_from_text(
            "The response must arrive on the next clock cycle.",
        )
        .expect("expected cycle window from 'next clock cycle'");
        assert_eq!(next_clock_cycle.min_cycles, Some(1));
        assert_eq!(next_clock_cycle.max_cycles, Some(1));

        let following_cycle = super::extract_cycle_window_from_text(
            "The response must arrive on the following cycle.",
        )
        .expect("expected cycle window from 'following cycle'");
        assert_eq!(following_cycle.min_cycles, Some(1));
        assert_eq!(following_cycle.max_cycles, Some(1));

        let subsequent_cycle = super::extract_cycle_window_from_text(
            "The response must arrive on the subsequent cycle.",
        )
        .expect("expected cycle window from 'subsequent cycle'");
        assert_eq!(subsequent_cycle.min_cycles, Some(1));
        assert_eq!(subsequent_cycle.max_cycles, Some(1));

        let next_tick =
            super::extract_cycle_window_from_text("The receiver samples DATA on the next tick.")
                .expect("expected cycle window from 'next tick'");
        assert_eq!(next_tick.min_cycles, Some(1));
        assert_eq!(next_tick.max_cycles, Some(1));

        let following_tick = super::extract_cycle_window_from_text(
            "The receiver samples DATA on the following tick.",
        )
        .expect("expected cycle window from 'following tick'");
        assert_eq!(following_tick.min_cycles, Some(1));
        assert_eq!(following_tick.max_cycles, Some(1));

        let subsequent_tick = super::extract_cycle_window_from_text(
            "The receiver samples DATA on the subsequent tick.",
        )
        .expect("expected cycle window from 'subsequent tick'");
        assert_eq!(subsequent_tick.min_cycles, Some(1));
        assert_eq!(subsequent_tick.max_cycles, Some(1));

        let next_edge =
            super::extract_cycle_window_from_text("VALID is sampled on the next rising edge.")
                .expect("expected cycle window from 'next rising edge'");
        assert_eq!(next_edge.min_cycles, Some(1));
        assert_eq!(next_edge.max_cycles, Some(1));
    }

    #[test]
    fn extracts_exact_cycle_window_from_later_and_ordinal_edge_phrases() {
        let later_phrase =
            super::extract_cycle_window_from_text("The response must arrive two cycles later.")
                .expect("expected cycle window from 'two cycles later'");
        assert_eq!(later_phrase.min_cycles, Some(2));
        assert_eq!(later_phrase.max_cycles, Some(2));

        let ordinal_edge = super::extract_cycle_window_from_text(
            "DATA is sampled on the third rising edge of HCLK.",
        )
        .expect("expected cycle window from 'on the third rising edge'");
        assert_eq!(ordinal_edge.min_cycles, Some(3));
        assert_eq!(ordinal_edge.max_cycles, Some(3));
    }

    #[test]
    fn extracts_quantified_tick_and_edge_cycle_window_phrases() {
        let bounded_ticks =
            super::extract_cycle_window_from_text("The receiver must respond within 2 ticks.")
                .expect("expected cycle window from 'within 2 ticks'");
        assert_eq!(bounded_ticks.min_cycles, None);
        assert_eq!(bounded_ticks.max_cycles, Some(2));

        let falling_edges =
            super::extract_cycle_window_from_text("DATA is sampled after 3 falling edges.")
                .expect("expected cycle window from 'after 3 falling edges'");
        assert_eq!(falling_edges.min_cycles, Some(3));
        assert_eq!(falling_edges.max_cycles, Some(3));

        let next_falling =
            super::extract_cycle_window_from_text("DATA is sampled on the next falling edge.")
                .expect("expected cycle window from 'next falling edge'");
        assert_eq!(next_falling.min_cycles, Some(1));
        assert_eq!(next_falling.max_cycles, Some(1));
    }

    #[test]
    fn extracts_posedge_negedge_and_diagram_position_cycle_window_phrases() {
        let next_posedge =
            super::extract_cycle_window_from_text("DATA is sampled on the next posedge.")
                .expect("expected cycle window from 'next posedge'");
        assert_eq!(next_posedge.min_cycles, Some(1));
        assert_eq!(next_posedge.max_cycles, Some(1));

        let next_negedge =
            super::extract_cycle_window_from_text("DATA is sampled on the next negedge.")
                .expect("expected cycle window from 'next negedge'");
        assert_eq!(next_negedge.min_cycles, Some(1));
        assert_eq!(next_negedge.max_cycles, Some(1));

        let tick_t3 =
            super::extract_cycle_window_from_text("The receiver samples DATA at tick T3.")
                .expect("expected cycle window from 'tick T3'");
        assert_eq!(tick_t3.min_cycles, Some(3));
        assert_eq!(tick_t3.max_cycles, Some(3));

        let posedge_t4 = super::extract_cycle_window_from_text("DATA is sampled on posedge T4.")
            .expect("expected cycle window from 'posedge T4'");
        assert_eq!(posedge_t4.min_cycles, Some(4));
        assert_eq!(posedge_t4.max_cycles, Some(4));
    }

    #[test]
    fn extracts_named_generic_edge_diagram_position_phrases() {
        let known_signals = BTreeSet::from(["HCLK".to_string()]);

        let hclk_edge_t3 = super::extract_cycle_window_from_text_with_known_signals(
            "DATA is sampled on HCLK edge T3.",
            &known_signals,
        )
        .expect("expected cycle window from 'HCLK edge T3'");
        assert_eq!(hclk_edge_t3.min_cycles, Some(3));
        assert_eq!(hclk_edge_t3.max_cycles, Some(3));

        let edge_t4_of_hclk = super::extract_cycle_window_from_text_with_known_signals(
            "DATA is sampled on edge T4 of HCLK.",
            &known_signals,
        )
        .expect("expected cycle window from 'edge T4 of HCLK'");
        assert_eq!(edge_t4_of_hclk.min_cycles, Some(4));
        assert_eq!(edge_t4_of_hclk.max_cycles, Some(4));

        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled on HCLK edge T3.",
                &known_signals
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled on edge T4 of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
    }

    #[test]
    fn extracts_unit_first_diagram_position_of_clock_phrases() {
        let known_signals = BTreeSet::from(["HCLK".to_string()]);

        let tick_t3_of_hclk =
            super::extract_cycle_window_from_text("DATA is sampled at tick T3 of HCLK.")
                .expect("expected cycle window from 'tick T3 of HCLK'");
        assert_eq!(tick_t3_of_hclk.min_cycles, Some(3));
        assert_eq!(tick_t3_of_hclk.max_cycles, Some(3));

        let posedge_t4_of_hclk =
            super::extract_cycle_window_from_text("DATA is sampled on posedge T4 of HCLK.")
                .expect("expected cycle window from 'posedge T4 of HCLK'");
        assert_eq!(posedge_t4_of_hclk.min_cycles, Some(4));
        assert_eq!(posedge_t4_of_hclk.max_cycles, Some(4));

        let rising_edge_t5_of_hclk =
            super::extract_cycle_window_from_text("DATA is sampled on rising edge T5 of HCLK.")
                .expect("expected cycle window from 'rising edge T5 of HCLK'");
        assert_eq!(rising_edge_t5_of_hclk.min_cycles, Some(5));
        assert_eq!(rising_edge_t5_of_hclk.max_cycles, Some(5));

        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled at tick T3 of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled on posedge T4 of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled on rising edge T5 of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
    }

    #[test]
    fn extracts_trailing_of_shorthand_edge_clock_phrases() {
        let known_signals = BTreeSet::from(["HCLK".to_string()]);

        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled on the third posedge of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled on the third negedge of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled on the third rising edge of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA must stabilize within 2 posedges of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA must stabilize within 2 negedges of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA must stabilize within 2 rising edges of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA must stabilize within 2 falling edges of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled on the third falling edge of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
    }

    #[test]
    fn extracts_clock_edge_of_clock_phrases() {
        let known_signals = BTreeSet::from(["HCLK".to_string()]);

        let clock_edge_t4_of_hclk = super::extract_cycle_window_from_text_with_known_signals(
            "DATA is sampled on clock edge T4 of HCLK.",
            &known_signals,
        )
        .expect("expected cycle window from 'clock edge T4 of HCLK'");
        assert_eq!(clock_edge_t4_of_hclk.min_cycles, Some(4));
        assert_eq!(clock_edge_t4_of_hclk.max_cycles, Some(4));

        let clock_edges_of_hclk = super::extract_cycle_window_from_text_with_known_signals(
            "DATA must stabilize within 2 clock edges of HCLK.",
            &known_signals,
        )
        .expect("expected cycle window from 'within 2 clock edges of HCLK'");
        assert_eq!(clock_edges_of_hclk.min_cycles, None);
        assert_eq!(clock_edges_of_hclk.max_cycles, Some(2));

        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA is sampled on clock edge T4 of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
        assert_eq!(
            super::explicit_clock_signal_from_text(
                "DATA must stabilize within 2 clock edges of HCLK.",
                &known_signals,
            )
            .as_deref(),
            Some("HCLK")
        );
    }

    #[test]
    fn extracts_zero_cycle_window_from_same_cycle_phrases() {
        let same_cycle = super::extract_cycle_window_from_text(
            "Both TVALID and TREADY can be asserted in the same ACLK cycle.",
        )
        .expect("expected cycle window from 'same ACLK cycle'");
        assert_eq!(same_cycle.min_cycles, Some(0));
        assert_eq!(same_cycle.max_cycles, Some(0));

        let same_tick =
            super::extract_cycle_window_from_text("The receiver samples DATA in the same tick.")
                .expect("expected cycle window from 'same tick'");
        assert_eq!(same_tick.min_cycles, Some(0));
        assert_eq!(same_tick.max_cycles, Some(0));

        let current_edge =
            super::extract_cycle_window_from_text("VALID is sampled on the current rising edge.")
                .expect("expected cycle window from 'current rising edge'");
        assert_eq!(current_edge.min_cycles, Some(0));
        assert_eq!(current_edge.max_cycles, Some(0));

        let current_clock_edge =
            super::extract_cycle_window_from_text("VALID is sampled on the current clock edge.")
                .expect("expected cycle window from 'current clock edge'");
        assert_eq!(current_clock_edge.min_cycles, Some(0));
        assert_eq!(current_clock_edge.max_cycles, Some(0));

        let current_falling_edge =
            super::extract_cycle_window_from_text("VALID is sampled on the current falling edge.")
                .expect("expected cycle window from 'current falling edge'");
        assert_eq!(current_falling_edge.min_cycles, Some(0));
        assert_eq!(current_falling_edge.max_cycles, Some(0));
    }

    #[test]
    fn extracts_generic_clock_edge_cycle_window_phrases() {
        let next_clock_edge =
            super::extract_cycle_window_from_text("DATA is sampled on the next clock edge.")
                .expect("expected cycle window from 'next clock edge'");
        assert_eq!(next_clock_edge.min_cycles, Some(1));
        assert_eq!(next_clock_edge.max_cycles, Some(1));

        let bounded_clock_edges =
            super::extract_cycle_window_from_text("DATA must stabilize within 2 clock edges.")
                .expect("expected cycle window from 'within 2 clock edges'");
        assert_eq!(bounded_clock_edges.min_cycles, None);
        assert_eq!(bounded_clock_edges.max_cycles, Some(2));
    }

    #[test]
    fn derives_next_tick_variants() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_next_tick.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PENABLE is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
                "Clock HCLK.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_next_tick".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the next tick.".to_string(),
            supporting_statement_ids: vec!["stmt_next_tick".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_penable_following_tick".to_string(),
            subject_signal: "PENABLE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PENABLE must be asserted on the following tick.".to_string(),
            supporting_statement_ids: vec!["stmt_following_tick".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_psel_subsequent_tick".to_string(),
            subject_signal: "PSEL".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSEL must be asserted on the subsequent tick.".to_string(),
            supporting_statement_ids: vec!["stmt_subsequent_tick".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let next_tick_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_next_tick")
            .expect("expected temporal rule derived from next-tick constraint");
        let next_tick_window = next_tick_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'next tick'");
        assert_eq!(next_tick_window.min_cycles, Some(1));
        assert_eq!(next_tick_window.max_cycles, Some(1));
        assert_eq!(next_tick_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(next_tick_rule.edge, super::ClockEdge::Rising);

        let following_tick_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_penable_following_tick")
            .expect("expected temporal rule derived from following-tick constraint");
        let following_tick_window = following_tick_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'following tick'");
        assert_eq!(following_tick_window.min_cycles, Some(1));
        assert_eq!(following_tick_window.max_cycles, Some(1));
        assert_eq!(following_tick_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(following_tick_rule.edge, super::ClockEdge::Rising);

        let subsequent_tick_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_psel_subsequent_tick")
            .expect("expected temporal rule derived from subsequent-tick constraint");
        let subsequent_tick_window = subsequent_tick_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'subsequent tick'");
        assert_eq!(subsequent_tick_window.min_cycles, Some(1));
        assert_eq!(subsequent_tick_window.max_cycles, Some(1));
        assert_eq!(subsequent_tick_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(subsequent_tick_rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_generic_next_cycle_variants() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_generic_next_cycle.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PENABLE is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
                "Signal PWRITE is input width 1.\n\n",
                "Clock HCLK.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_next_cycle".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the next cycle.".to_string(),
            supporting_statement_ids: vec!["stmt_next_cycle".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_penable_next_clock_cycle".to_string(),
            subject_signal: "PENABLE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PENABLE must be asserted on the next clock cycle.".to_string(),
            supporting_statement_ids: vec!["stmt_next_clock_cycle".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_psel_following_cycle".to_string(),
            subject_signal: "PSEL".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSEL must be asserted on the following cycle.".to_string(),
            supporting_statement_ids: vec!["stmt_following_cycle".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pwrite_subsequent_cycle".to_string(),
            subject_signal: "PWRITE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PWRITE must be asserted on the subsequent cycle.".to_string(),
            supporting_statement_ids: vec!["stmt_subsequent_cycle".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let next_cycle_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_next_cycle")
            .expect("expected temporal rule derived from next-cycle constraint");
        let next_cycle_window = next_cycle_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'next cycle'");
        assert_eq!(next_cycle_window.min_cycles, Some(1));
        assert_eq!(next_cycle_window.max_cycles, Some(1));
        assert_eq!(next_cycle_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(next_cycle_rule.edge, super::ClockEdge::Rising);

        let next_clock_cycle_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_penable_next_clock_cycle"
            })
            .expect("expected temporal rule derived from next-clock-cycle constraint");
        let next_clock_cycle_window = next_clock_cycle_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'next clock cycle'");
        assert_eq!(next_clock_cycle_window.min_cycles, Some(1));
        assert_eq!(next_clock_cycle_window.max_cycles, Some(1));
        assert_eq!(next_clock_cycle_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(next_clock_cycle_rule.edge, super::ClockEdge::Rising);

        let following_cycle_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_psel_following_cycle")
            .expect("expected temporal rule derived from following-cycle constraint");
        let following_cycle_window = following_cycle_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'following cycle'");
        assert_eq!(following_cycle_window.min_cycles, Some(1));
        assert_eq!(following_cycle_window.max_cycles, Some(1));
        assert_eq!(following_cycle_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(following_cycle_rule.edge, super::ClockEdge::Rising);

        let subsequent_cycle_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pwrite_subsequent_cycle"
            })
            .expect("expected temporal rule derived from subsequent-cycle constraint");
        let subsequent_cycle_window = subsequent_cycle_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'subsequent cycle'");
        assert_eq!(subsequent_cycle_window.min_cycles, Some(1));
        assert_eq!(subsequent_cycle_window.max_cycles, Some(1));
        assert_eq!(subsequent_cycle_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(subsequent_cycle_rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_zero_cycle_window_from_same_cycle_constraint_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_same_cycle.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal TVALID is input width 1.\n\n",
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
            constraint_id: "sigcon_tvalid_same_cycle".to_string(),
            subject_signal: "TVALID".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "TVALID must be asserted in the same ACLK cycle.".to_string(),
            supporting_statement_ids: vec!["stmt_same_cycle".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_tvalid_same_cycle")
            .expect("expected temporal rule derived from same-cycle constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'same ACLK cycle'");
        assert_eq!(cycle_window.min_cycles, Some(0));
        assert_eq!(cycle_window.max_cycles, Some(0));

        Ok(())
    }

    #[test]
    fn derives_local_clock_and_edge_from_named_cycle_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_cycle_clock.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal ACLK is input width 1.\n\n",
                "Signal TVALID is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_tvalid_same_aclk_cycle".to_string(),
            subject_signal: "TVALID".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "TVALID must be asserted in the same ACLK cycle.".to_string(),
            supporting_statement_ids: vec!["stmt_same_aclk_cycle".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_tvalid_same_aclk_cycle")
            .expect("expected temporal rule derived from named-cycle constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'same ACLK cycle'");
        assert_eq!(cycle_window.min_cycles, Some(0));
        assert_eq!(cycle_window.max_cycles, Some(0));
        assert_eq!(rule.clock_signal.as_deref(), Some("ACLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_named_zero_cycle_edge_variants() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_named_zero_cycle_edge_variants.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PSLVERR is input width 1.\n\n",
                "Signal PSTRB is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pslverr_current_hclk_clock_edge".to_string(),
            subject_signal: "PSLVERR".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSLVERR must be asserted on the current HCLK clock edge.".to_string(),
            supporting_statement_ids: vec!["stmt_current_hclk_clock_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pstrb_current_hclk_falling_edge".to_string(),
            subject_signal: "PSTRB".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSTRB must be asserted on the current HCLK falling edge.".to_string(),
            supporting_statement_ids: vec!["stmt_current_hclk_falling_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let current_clock_edge_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pslverr_current_hclk_clock_edge"
            })
            .expect("expected temporal rule derived from named current clock-edge constraint");
        let current_clock_edge_window = current_clock_edge_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window from 'current HCLK clock edge'");
        assert_eq!(current_clock_edge_window.min_cycles, Some(0));
        assert_eq!(current_clock_edge_window.max_cycles, Some(0));
        assert_eq!(
            current_clock_edge_rule.clock_signal.as_deref(),
            Some("HCLK")
        );
        assert_eq!(current_clock_edge_rule.edge, super::ClockEdge::Rising);

        let current_falling_edge_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pstrb_current_hclk_falling_edge"
            })
            .expect("expected temporal rule derived from named current falling-edge constraint");
        let current_falling_edge_window = current_falling_edge_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window from 'current HCLK falling edge'");
        assert_eq!(current_falling_edge_window.min_cycles, Some(0));
        assert_eq!(current_falling_edge_window.max_cycles, Some(0));
        assert_eq!(
            current_falling_edge_rule.clock_signal.as_deref(),
            Some("HCLK")
        );
        assert_eq!(current_falling_edge_rule.edge, super::ClockEdge::Falling);

        Ok(())
    }

    #[test]
    fn derives_single_cycle_window_from_named_next_cycle_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_next_cycle.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal ACLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_next_aclk_cycle".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the next ACLK cycle.".to_string(),
            supporting_statement_ids: vec!["stmt_next_aclk_cycle".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_next_aclk_cycle")
            .expect("expected temporal rule derived from named next-cycle constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'next ACLK cycle'");
        assert_eq!(cycle_window.min_cycles, Some(1));
        assert_eq!(cycle_window.max_cycles, Some(1));
        assert_eq!(rule.clock_signal.as_deref(), Some("ACLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_single_cycle_window_from_named_next_clock_edge_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_next_clock_edge.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_next_hclk_edge".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the next HCLK edge.".to_string(),
            supporting_statement_ids: vec!["stmt_next_hclk_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_next_hclk_edge")
            .expect("expected temporal rule derived from named next-edge constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'next HCLK edge'");
        assert_eq!(cycle_window.min_cycles, Some(1));
        assert_eq!(cycle_window.max_cycles, Some(1));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_named_next_edge_variants() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_next_edge_variants.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PSLVERR is input width 1.\n\n",
                "Signal PSTRB is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pslverr_next_hclk_clock_edge".to_string(),
            subject_signal: "PSLVERR".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSLVERR must be asserted on the next HCLK clock edge.".to_string(),
            supporting_statement_ids: vec!["stmt_next_hclk_clock_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pstrb_following_hclk_falling_edge".to_string(),
            subject_signal: "PSTRB".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSTRB must be asserted on the following HCLK falling edge.".to_string(),
            supporting_statement_ids: vec!["stmt_following_hclk_falling_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let next_clock_edge_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pslverr_next_hclk_clock_edge"
            })
            .expect("expected temporal rule derived from named next clock-edge constraint");
        let next_clock_edge_window = next_clock_edge_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window from 'next HCLK clock edge'");
        assert_eq!(next_clock_edge_window.min_cycles, Some(1));
        assert_eq!(next_clock_edge_window.max_cycles, Some(1));
        assert_eq!(next_clock_edge_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(next_clock_edge_rule.edge, super::ClockEdge::Rising);

        let following_falling_edge_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id
                    == "temporal_signal_constraint_sigcon_pstrb_following_hclk_falling_edge"
            })
            .expect("expected temporal rule derived from named following falling-edge constraint");
        let following_falling_edge_window = following_falling_edge_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window from 'following HCLK falling edge'");
        assert_eq!(following_falling_edge_window.min_cycles, Some(1));
        assert_eq!(following_falling_edge_window.max_cycles, Some(1));
        assert_eq!(
            following_falling_edge_rule.clock_signal.as_deref(),
            Some("HCLK")
        );
        assert_eq!(following_falling_edge_rule.edge, super::ClockEdge::Falling);

        Ok(())
    }

    #[test]
    fn derives_named_quantified_edge_variants() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_quantified_edge.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_within_two_hclk_edges".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted within 2 HCLK edges.".to_string(),
            supporting_statement_ids: vec!["stmt_within_two_hclk_edges".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_psel_third_hclk_edge".to_string(),
            subject_signal: "PSEL".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSEL must be asserted on the third HCLK edge.".to_string(),
            supporting_statement_ids: vec!["stmt_third_hclk_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let bounded_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pready_within_two_hclk_edges"
            })
            .expect("expected temporal rule derived from named quantified-edge constraint");
        let bounded_window = bounded_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'within 2 HCLK edges'");
        assert_eq!(bounded_window.min_cycles, None);
        assert_eq!(bounded_window.max_cycles, Some(2));
        assert_eq!(bounded_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(bounded_rule.edge, super::ClockEdge::Rising);

        let ordinal_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_psel_third_hclk_edge")
            .expect(
                "expected temporal rule derived from signal-leading named ordinal edge constraint",
            );
        let ordinal_window = ordinal_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'the third HCLK edge'");
        assert_eq!(ordinal_window.min_cycles, Some(3));
        assert_eq!(ordinal_window.max_cycles, Some(3));
        assert_eq!(ordinal_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(ordinal_rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_exact_cycle_window_from_named_edge_of_clock_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_edge_of_clock.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_third_edge_of_hclk".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the third edge of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_third_edge_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pready_third_edge_of_hclk"
            })
            .expect("expected temporal rule derived from named edge-of-clock constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'the third edge of HCLK'");
        assert_eq!(cycle_window.min_cycles, Some(3));
        assert_eq!(cycle_window.max_cycles, Some(3));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_local_clock_signal_from_plural_edge_of_clock_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_plural_edge_of_clock.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_within_two_edges_of_hclk".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted within 2 edges of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_within_two_edges_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pready_within_two_edges_of_hclk"
            })
            .expect("expected temporal rule derived from plural edge-of-clock constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'within 2 edges of HCLK'");
        assert_eq!(cycle_window.min_cycles, None);
        assert_eq!(cycle_window.max_cycles, Some(2));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_named_diagram_edge_variants() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_diagram_edge.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PENABLE is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_edge_t3_of_hclk".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on edge T3 of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_edge_t3_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_penable_hclk_edge_t4".to_string(),
            subject_signal: "PENABLE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PENABLE must be asserted on HCLK edge T4.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_edge_t4".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let trailing_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_edge_t3_of_hclk")
            .expect("expected temporal rule derived from named diagram-edge constraint");
        let trailing_window = trailing_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'edge T3 of HCLK'");
        assert_eq!(trailing_window.min_cycles, Some(3));
        assert_eq!(trailing_window.max_cycles, Some(3));
        assert_eq!(trailing_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(trailing_rule.edge, super::ClockEdge::Rising);

        let signal_leading_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_penable_hclk_edge_t4")
            .expect(
                "expected temporal rule derived from signal-leading named diagram-edge constraint",
            );
        let signal_leading_window = signal_leading_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'HCLK edge T4'");
        assert_eq!(signal_leading_window.min_cycles, Some(4));
        assert_eq!(signal_leading_window.max_cycles, Some(4));
        assert_eq!(signal_leading_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(signal_leading_rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_unit_first_diagram_position_variants() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_unit_first_diagram_position.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PENABLE is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_penable_tick_t3_of_hclk".to_string(),
            subject_signal: "PENABLE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PENABLE must be asserted at tick T3 of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_tick_t3_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_posedge_t4_of_hclk".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on posedge T4 of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_posedge_t4_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_psel_rising_edge_t5_of_hclk".to_string(),
            subject_signal: "PSEL".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSEL must be asserted on rising edge T5 of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_rising_edge_t5_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let tick_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_penable_tick_t3_of_hclk"
            })
            .expect("expected temporal rule derived from unit-first tick-position constraint");
        let tick_window = tick_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'tick T3 of HCLK'");
        assert_eq!(tick_window.min_cycles, Some(3));
        assert_eq!(tick_window.max_cycles, Some(3));
        assert_eq!(tick_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(tick_rule.edge, super::ClockEdge::Rising);

        let posedge_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pready_posedge_t4_of_hclk"
            })
            .expect("expected temporal rule derived from unit-first diagram-position constraint");
        let posedge_window = posedge_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'posedge T4 of HCLK'");
        assert_eq!(posedge_window.min_cycles, Some(4));
        assert_eq!(posedge_window.max_cycles, Some(4));
        assert_eq!(posedge_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(posedge_rule.edge, super::ClockEdge::Rising);

        let rising_edge_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_psel_rising_edge_t5_of_hclk"
            })
            .expect("expected temporal rule derived from unit-first rising-edge constraint");
        let rising_edge_window = rising_edge_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'rising edge T5 of HCLK'");
        assert_eq!(rising_edge_window.min_cycles, Some(5));
        assert_eq!(rising_edge_window.max_cycles, Some(5));
        assert_eq!(rising_edge_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rising_edge_rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_local_clock_from_trailing_of_shorthand_edge_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_trailing_of_shorthand_edge_clock.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_third_posedge_of_hclk".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the third posedge of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_third_posedge_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pready_third_posedge_of_hclk"
            })
            .expect("expected temporal rule derived from trailing-of shorthand-edge constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'third posedge of HCLK'");
        assert_eq!(cycle_window.min_cycles, Some(3));
        assert_eq!(cycle_window.max_cycles, Some(3));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_exact_falling_edge_from_trailing_of_shorthand_edge_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_trailing_of_shorthand_edge_exact_falling.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PLOCK is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_plock_third_negedge_of_hclk".to_string(),
            subject_signal: "PLOCK".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PLOCK must be asserted on the third negedge of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_third_negedge_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_plock_third_negedge_of_hclk"
            })
            .expect(
                "expected temporal rule derived from exact trailing-of shorthand-edge constraint",
            );
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'third negedge of HCLK'");
        assert_eq!(cycle_window.min_cycles, Some(3));
        assert_eq!(cycle_window.max_cycles, Some(3));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Falling);

        Ok(())
    }

    #[test]
    fn derives_rising_edge_from_trailing_of_shorthand_edge_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_trailing_of_shorthand_edge_rising.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PENABLE is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_penable_within_two_posedges_of_hclk".to_string(),
            subject_signal: "PENABLE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PENABLE must be asserted within 2 posedges of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_within_two_posedges_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id
                    == "temporal_signal_constraint_sigcon_penable_within_two_posedges_of_hclk"
            })
            .expect("expected temporal rule derived from plural shorthand-edge constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'within 2 posedges of HCLK'");
        assert_eq!(cycle_window.min_cycles, None);
        assert_eq!(cycle_window.max_cycles, Some(2));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_rising_edge_from_trailing_of_word_edge_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_trailing_of_word_edge_rising.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_psel_within_two_rising_edges_of_hclk".to_string(),
            subject_signal: "PSEL".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSEL must be asserted within 2 rising edges of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_within_two_rising_edges_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id
                    == "temporal_signal_constraint_sigcon_psel_within_two_rising_edges_of_hclk"
            })
            .expect("expected temporal rule derived from word-edge timing constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'within 2 rising edges of HCLK'");
        assert_eq!(cycle_window.min_cycles, None);
        assert_eq!(cycle_window.max_cycles, Some(2));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_falling_edge_from_trailing_of_word_edge_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_trailing_of_word_edge_falling.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PWRITE is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pwrite_within_two_falling_edges_of_hclk".to_string(),
            subject_signal: "PWRITE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PWRITE must be asserted within 2 falling edges of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_within_two_falling_edges_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id
                    == "temporal_signal_constraint_sigcon_pwrite_within_two_falling_edges_of_hclk"
            })
            .expect("expected temporal rule derived from word-edge timing constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'within 2 falling edges of HCLK'");
        assert_eq!(cycle_window.min_cycles, None);
        assert_eq!(cycle_window.max_cycles, Some(2));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Falling);

        Ok(())
    }

    #[test]
    fn derives_falling_edge_from_trailing_of_shorthand_edge_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_trailing_of_shorthand_edge_falling.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PSLVERR is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pslverr_within_two_negedges_of_hclk".to_string(),
            subject_signal: "PSLVERR".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSLVERR must be asserted within 2 negedges of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_within_two_negedges_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id
                    == "temporal_signal_constraint_sigcon_pslverr_within_two_negedges_of_hclk"
            })
            .expect("expected temporal rule derived from trailing-of shorthand-edge constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'within 2 negedges of HCLK'");
        assert_eq!(cycle_window.min_cycles, None);
        assert_eq!(cycle_window.max_cycles, Some(2));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Falling);

        Ok(())
    }

    #[test]
    fn derives_clock_edge_of_clock_variants() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_clock_edge_of_clock.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PENABLE is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_clock_edge_t4_of_hclk".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on clock edge T4 of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_clock_edge_t4_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_penable_hclk_clock_edge_t5".to_string(),
            subject_signal: "PENABLE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PENABLE must be asserted on HCLK clock edge T5.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_clock_edge_t5".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let trailing_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pready_clock_edge_t4_of_hclk"
            })
            .expect("expected temporal rule derived from clock-edge-of-clock constraint");
        let trailing_window = trailing_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'clock edge T4 of HCLK'");
        assert_eq!(trailing_window.min_cycles, Some(4));
        assert_eq!(trailing_window.max_cycles, Some(4));
        assert_eq!(trailing_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(trailing_rule.edge, super::ClockEdge::Rising);

        let signal_leading_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_penable_hclk_clock_edge_t5"
            })
            .expect(
                "expected temporal rule derived from signal-leading clock-edge-of-clock constraint",
            );
        let signal_leading_window = signal_leading_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'HCLK clock edge T5'");
        assert_eq!(signal_leading_window.min_cycles, Some(5));
        assert_eq!(signal_leading_window.max_cycles, Some(5));
        assert_eq!(signal_leading_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(signal_leading_rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_exact_cycle_window_from_later_phrase_constraint_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_later_phrase.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
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
            constraint_id: "sigcon_pready_two_cycles_later".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted two cycles later.".to_string(),
            supporting_statement_ids: vec!["stmt_two_cycles_later".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pready_two_cycles_later"
            })
            .expect("expected temporal rule derived from later-phrase constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'two cycles later'");
        assert_eq!(cycle_window.min_cycles, Some(2));
        assert_eq!(cycle_window.max_cycles, Some(2));

        Ok(())
    }

    #[test]
    fn derives_exact_cycle_window_from_ordinal_edge_constraint_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_ordinal_edge.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PWAKEUP is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
                "Signal PWRITE is input width 1.\n\n",
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
            constraint_id: "sigcon_pready_third_rising_edge".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the third rising edge of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_third_rising_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pwakeup_third_falling_edge".to_string(),
            subject_signal: "PWAKEUP".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PWAKEUP must be asserted on the third falling edge of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_third_falling_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pready_third_rising_edge"
            })
            .expect("expected temporal rule derived from ordinal-edge constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'the third rising edge'");
        assert_eq!(cycle_window.min_cycles, Some(3));
        assert_eq!(cycle_window.max_cycles, Some(3));
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        let falling_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pwakeup_third_falling_edge"
            })
            .expect("expected temporal rule derived from ordinal falling-edge constraint");
        let falling_cycle_window = falling_rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'the third falling edge'");
        assert_eq!(falling_cycle_window.min_cycles, Some(3));
        assert_eq!(falling_cycle_window.max_cycles, Some(3));
        assert_eq!(falling_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(falling_rule.edge, super::ClockEdge::Falling);

        Ok(())
    }

    #[test]
    fn derives_explicit_clock_signal_from_signal_leading_edge_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_signal_leading_edge.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PWAKEUP is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
                "Signal PWRITE is input width 1.\n\n",
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
            constraint_id: "sigcon_pready_hclk_rising_edge".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on HCLK rising edge.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_rising_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pwakeup_hclk_falling_edge".to_string(),
            subject_signal: "PWAKEUP".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PWAKEUP must be asserted on HCLK falling edge.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_falling_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_psel_hclk_posedge".to_string(),
            subject_signal: "PSEL".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSEL must be asserted on HCLK posedge.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_posedge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pwrite_hclk_negedge".to_string(),
            subject_signal: "PWRITE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PWRITE must be asserted on HCLK negedge.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_negedge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pready_hclk_rising_edge"
            })
            .expect("expected temporal rule derived from signal-leading edge constraint");
        assert_eq!(rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        let falling_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_pwakeup_hclk_falling_edge"
            })
            .expect("expected temporal rule derived from signal-leading falling-edge constraint");
        assert_eq!(falling_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(falling_rule.edge, super::ClockEdge::Falling);

        let posedge_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_psel_hclk_posedge")
            .expect("expected temporal rule derived from signal-leading posedge constraint");
        assert_eq!(posedge_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(posedge_rule.edge, super::ClockEdge::Rising);

        let negedge_rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pwrite_hclk_negedge")
            .expect("expected temporal rule derived from signal-leading negedge constraint");
        assert_eq!(negedge_rule.clock_signal.as_deref(), Some("HCLK"));
        assert_eq!(negedge_rule.edge, super::ClockEdge::Falling);

        Ok(())
    }

    #[test]
    fn derives_quantified_tick_cycle_window_from_constraint_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_tick_window.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
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
            constraint_id: "sigcon_pready_within_ticks".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted within 2 ticks.".to_string(),
            supporting_statement_ids: vec!["stmt_within_ticks".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_within_ticks")
            .expect("expected temporal rule derived from quantified tick constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'within 2 ticks'");
        assert_eq!(cycle_window.min_cycles, None);
        assert_eq!(cycle_window.max_cycles, Some(2));

        Ok(())
    }

    #[test]
    fn derives_posedge_cycle_window_from_constraint_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_posedge.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
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
            constraint_id: "sigcon_pready_next_posedge".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the next posedge.".to_string(),
            supporting_statement_ids: vec!["stmt_next_posedge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_next_posedge")
            .expect("expected temporal rule derived from next-posedge constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'next posedge'");
        assert_eq!(cycle_window.min_cycles, Some(1));
        assert_eq!(cycle_window.max_cycles, Some(1));
        assert_eq!(rule.edge, super::ClockEdge::Rising);

        Ok(())
    }

    #[test]
    fn derives_negedge_cycle_window_from_constraint_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_negedge.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
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
            constraint_id: "sigcon_pready_next_negedge".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the next negedge.".to_string(),
            supporting_statement_ids: vec!["stmt_next_negedge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_next_negedge")
            .expect("expected temporal rule derived from next-negedge constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'next negedge'");
        assert_eq!(cycle_window.min_cycles, Some(1));
        assert_eq!(cycle_window.max_cycles, Some(1));
        assert_eq!(rule.edge, super::ClockEdge::Falling);

        Ok(())
    }

    #[test]
    fn derives_diagram_style_tick_position_cycle_window_from_constraint_text() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_tick_t3.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
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
            constraint_id: "sigcon_pready_tick_t3".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted at tick T3.".to_string(),
            supporting_statement_ids: vec!["stmt_tick_t3".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_tick_t3")
            .expect("expected temporal rule derived from tick-T3 constraint");
        let cycle_window = rule
            .cycle_window
            .as_ref()
            .expect("expected cycle window to be derived from 'tick T3'");
        assert_eq!(cycle_window.min_cycles, Some(3));
        assert_eq!(cycle_window.max_cycles, Some(3));

        Ok(())
    }

    #[test]
    fn derives_cycle_window_from_timing_constraint_tick_units() {
        use crate::ir::source::TimingConstraintRecord;

        let cycle_window = super::extract_cycle_window_from_timing_constraint(
            &TimingConstraintRecord {
                constraint_id: "timing_tick_window".to_string(),
                parameter_name: "TWAIT".to_string(),
                min_value: None,
                typ_value: Some("2".to_string()),
                max_value: None,
                unit: Some("ticks".to_string()),
                description: Some("Wait-state latency".to_string()),
                supporting_statement_ids: vec!["stmt_tick_unit".to_string()],
                automation_confidence: AutomationConfidence::Medium,
            },
            "Wait-state latency",
            &BTreeSet::new(),
            &BTreeSet::new(),
            false,
            false,
            None,
        )
        .expect("expected cycle window from tick-based timing constraint units");

        assert_eq!(cycle_window.min_cycles, Some(2));
        assert_eq!(cycle_window.max_cycles, Some(2));
    }

    #[test]
    fn derives_cycle_window_from_temporal_phrase_prior_when_builtin_parser_cannot() -> Result<()> {
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        assert!(
            super::extract_cycle_window_from_text("PREADY must be asserted one beat later.")
                .is_none(),
            "the built-in parser should not recognize the learned-only phrase"
        );

        let tempdir = tempdir()?;
        let source = tempdir.path().join("apb_temporal_prior.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let prior_memory_path = write_temporal_phrase_prior_memory(
            tempdir.path(),
            "<signal> must be asserted one beat later",
            ProtocolFamily::AmbaApb,
            CycleWindowRecord {
                min_cycles: Some(1),
                max_cycles: Some(1),
            },
        )?;

        fs::write(
            &source,
            concat!(
                "# APB Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Clock clk.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build_with_prior_memory(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
            Some(&prior_memory_path),
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_one_beat".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted one beat later.".to_string(),
            supporting_statement_ids: vec!["stmt_prior_guided_temporal".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_one_beat")
            .expect("expected temporal rule derived from prior-guided phrase");
        assert_eq!(
            rule.cycle_window,
            Some(CycleWindowRecord {
                min_cycles: Some(1),
                max_cycles: Some(1),
            })
        );

        Ok(())
    }

    #[test]
    fn derives_actor_grounded_drive_event_from_value_constraint() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_actor.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_asserted")
            .expect("expected temporal rule derived from asserted value constraint");
        assert!(rule.consequents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::ActorDrivesSignal {
                    actor_name,
                    signal_name,
                    phase: super::TickPhase::PostTick,
                } if actor_name.eq_ignore_ascii_case("Completer") && signal_name == "PREADY"
            )
        }));
        assert!(rule.consequents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::SignalValue {
                    signal_name,
                    value,
                    phase: super::TickPhase::PostTick,
                } if signal_name == "PREADY" && value == "ASSERTED"
            )
        }));

        Ok(())
    }

    #[test]
    fn derives_actor_grounded_stability_event_from_stable_constraint() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_actor_stable.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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
            condition_text: Some("when HREADY is LOW".to_string()),
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

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_pready_stable")
            .expect("expected temporal rule derived from stable constraint");
        assert!(rule.consequents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::ActorMaintainsSignalStable {
                    actor_name,
                    signal_name,
                    from_phase: super::TickPhase::PreTick,
                    to_phase: super::TickPhase::PostTick,
                } if actor_name.eq_ignore_ascii_case("Completer") && signal_name == "PREADY"
            )
        }));
        assert!(rule.consequents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::SignalStable {
                    signal_name,
                    from_phase: super::TickPhase::PreTick,
                    to_phase: super::TickPhase::PostTick,
                } if signal_name == "PREADY"
            )
        }));

        Ok(())
    }

    #[test]
    fn derives_multi_predicate_antecedents_from_compound_guard() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_compound_guard.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_htrans_compound_guard")
            .expect("expected temporal rule derived from compound guard");
        assert_eq!(rule.antecedents.len(), 2);
        assert!(rule.antecedents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::SignalValue {
                    signal_name,
                    value,
                    phase: super::TickPhase::PreTick,
                } if signal_name == "HREADY" && value == "LOW"
            )
        }));
        assert!(rule.antecedents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::SignalValue {
                    signal_name,
                    value,
                    phase: super::TickPhase::PreTick,
                } if signal_name == "HSEL" && value == "HIGH"
            )
        }));

        Ok(())
    }

    #[test]
    fn derives_handshake_completion_from_valid_ready_guard() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_handshake_guard.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| rule.rule_id == "temporal_signal_constraint_sigcon_payload_handshake")
            .expect("expected temporal rule derived from valid/ready guard");
        assert!(rule.antecedents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::HandshakeComplete {
                    valid_signal,
                    ready_signal,
                    phase: super::TickPhase::PreTick,
                } if valid_signal == "AWVALID" && ready_signal == "AWREADY"
            )
        }));

        Ok(())
    }

    #[test]
    fn derives_handshake_completion_from_semantic_signal_hints() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{
            ContentSectionRecord, SectionKind, SignalConstraintKind, SignalConstraintRecord,
            StructuredTableCellRecord, StructuredTableRecord, TableKind,
        };

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_semantic_handshake_guard.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal XREQ is input width 1.\n\n",
                "Signal XACK is input width 1.\n\n",
                "Signal PAYLOAD is output width 32.\n\n",
                "Clock clk.\n",
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
            table_id: "table_semantic_handshake_desc".to_string(),
            asset_id: "table_semantic_handshake_desc".to_string(),
            page_id: None,
            caption_text: Some("Handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_cell("Signal", true),
                make_cell("Source", true),
                make_cell("Width", true),
                make_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_cell("XREQ", false),
                    make_cell("Requester", false),
                    make_cell("1", false),
                    make_cell(
                        "Indicates that address and control information are valid for transfer.",
                        false,
                    ),
                ],
                vec![
                    make_cell("XACK", false),
                    make_cell("Subordinate", false),
                    make_cell("1", false),
                    make_cell(
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
            constraint_id: "sigcon_payload_semantic_handshake".to_string(),
            subject_signal: "PAYLOAD".to_string(),
            constraint_kind: SignalConstraintKind::MustNotChange,
            target_value: None,
            condition_text: Some("when XREQ is HIGH and XACK is HIGH".to_string()),
            negated: false,
            source_text: "PAYLOAD must not change when XREQ is HIGH and XACK is HIGH.".to_string(),
            supporting_statement_ids: vec!["stmt_temporal_semantic_handshake".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_payload_semantic_handshake"
            })
            .expect("expected temporal rule derived from semantically grounded handshake guard");
        assert!(rule.antecedents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::HandshakeComplete {
                    valid_signal,
                    ready_signal,
                    phase: super::TickPhase::PreTick,
                } if valid_signal == "XREQ" && ready_signal == "XACK"
            )
        }));

        Ok(())
    }

    #[test]
    fn contested_semantic_roles_block_handshake_name_fallback() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{
            ContentSectionRecord, SectionKind, SignalConstraintKind, SignalConstraintRecord,
            StructuredTableCellRecord, StructuredTableRecord, TableKind,
        };

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_contested_semantic_handshake_guard.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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
        let make_cell = |text: &str, is_header: bool| StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        };
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_contested_semantic_handshake_desc".to_string(),
            asset_id: "table_contested_semantic_handshake_desc".to_string(),
            page_id: None,
            caption_text: Some("Handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_cell("Signal", true),
                make_cell("Source", true),
                make_cell("Width", true),
                make_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_cell("XVALID", false),
                    make_cell("Requester", false),
                    make_cell("1", false),
                    make_cell(
                        "Indicates that address and control information are valid for transfer.",
                        false,
                    ),
                ],
                vec![
                    make_cell("XACK", false),
                    make_cell("Subordinate", false),
                    make_cell("1", false),
                    make_cell(
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

        let xvalid = semantic_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XVALID")
            .expect("expected XVALID interface signal");
        assert!(!xvalid.supporting_table_ids.is_empty());
        let arbitration = xvalid
            .semantic_arbitration
            .as_ref()
            .expect("expected XVALID semantic arbitration");
        assert!(!arbitration.decisive);

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id
                    == "temporal_signal_constraint_sigcon_payload_contested_semantic_handshake"
            })
            .expect("expected temporal rule derived from contested semantic handshake guard");
        assert!(!rule.antecedents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::HandshakeComplete { .. }
            )
        }));
        assert!(
            semantic_ir
                .residual_decisions
                .iter()
                .any(|packet| { packet.packet_id == "semantic_handshake_name_fallback_blocked" })
        );

        Ok(())
    }

    #[test]
    fn derives_handshake_completion_from_visual_caption_semantic_hints() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_visual_caption_semantic_handshake_guard.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal XREQ is input width 1.\n\n",
                "Signal XACK is input width 1.\n\n",
                "Signal PAYLOAD is output width 32.\n\n",
                "Clock clk.\n",
            ),
        )?;

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
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "figure_xack".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0002".to_string()),
            image_path: None,
            caption_text: Some("Figure 2: XACK ready timing.".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: None,
            diagram_kind: crate::ir::source::DiagramKind::TimingDiagram,
        });
        source_ir.write_to_disk()?;

        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_payload_visual_semantic_handshake".to_string(),
            subject_signal: "PAYLOAD".to_string(),
            constraint_kind: SignalConstraintKind::MustNotChange,
            target_value: None,
            condition_text: Some("when XREQ is HIGH and XACK is HIGH".to_string()),
            negated: false,
            source_text: "PAYLOAD must not change when XREQ is HIGH and XACK is HIGH.".to_string(),
            supporting_statement_ids: vec!["stmt_temporal_visual_semantic_handshake".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id
                    == "temporal_signal_constraint_sigcon_payload_visual_semantic_handshake"
            })
            .expect("expected temporal rule derived from visual-caption grounded handshake guard");
        assert!(rule.antecedents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::HandshakeComplete {
                    valid_signal,
                    ready_signal,
                    phase: super::TickPhase::PreTick,
                } if valid_signal == "XREQ" && ready_signal == "XACK"
            )
        }));

        Ok(())
    }

    #[test]
    fn derives_handshake_completion_from_alias_grounded_semantic_signal_hints() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_alias_grounded_semantic_handshake_guard.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        let rule = semantic_ir
            .temporal_rules
            .iter()
            .find(|rule| {
                rule.rule_id == "temporal_signal_constraint_sigcon_payload_alias_semantic_handshake"
            })
            .expect("expected temporal rule derived from alias-grounded semantic handshake guard");
        assert!(rule.antecedents.iter().any(|predicate| {
            matches!(
                predicate,
                super::TemporalPredicateRecord::HandshakeComplete {
                    valid_signal,
                    ready_signal,
                    phase: super::TickPhase::PreTick,
                } if valid_signal == "XREQ" && ready_signal == "XACK"
            )
        }));

        let xreq = semantic_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XREQ")
            .expect("expected XREQ interface signal");
        assert!(xreq.semantic_candidates[0].alias_dependent);
        assert!(
            xreq.semantic_consensus
                .as_ref()
                .expect("expected XREQ semantic consensus")
                .alias_dependent
        );
        let xack = semantic_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "XACK")
            .expect("expected XACK interface signal");
        assert!(xack.semantic_candidates[0].alias_dependent);
        assert!(
            xack.semantic_consensus
                .as_ref()
                .expect("expected XACK semantic consensus")
                .alias_dependent
        );
        assert!(
            semantic_ir.residual_decisions.iter().any(|packet| {
                packet.packet_id == "semantic_alias_dependent_handshake_completion"
            })
        );

        Ok(())
    }

    #[test]
    fn derives_typed_temporal_conflicts_from_conflicting_value_rules() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

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

        assert_eq!(semantic_ir.temporal_conflicts.len(), 1);
        let conflict = &semantic_ir.temporal_conflicts[0];
        assert_eq!(conflict.signal_name, "PREADY");
        assert_eq!(conflict.phase, super::TickPhase::PostTick);
        assert_eq!(
            conflict.conflicting_values,
            vec!["HIGH".to_string(), "LOW".to_string()]
        );
        assert_eq!(conflict.supporting_rule_ids.len(), 2);
        assert_eq!(conflict.antecedents.len(), 1);
        assert_eq!(conflict.automation_confidence, AutomationConfidence::Medium);
        assert!(!conflict.supporting_statement_ids.is_empty());

        Ok(())
    }

    #[test]
    fn asserted_and_high_do_not_form_temporal_conflicts_without_known_polarity() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_equivalent_values.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal TLAST is output width 1.\n\n",
                "Clock clk.\n\n",
                "The Transmitter drives TLAST.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_tlast_asserted".to_string(),
            subject_signal: "TLAST".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "TLAST must be asserted.".to_string(),
            supporting_statement_ids: vec!["stmt_tlast_asserted".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_tlast_high".to_string(),
            subject_signal: "TLAST".to_string(),
            constraint_kind: SignalConstraintKind::MustBeHigh,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "TLAST must be HIGH.".to_string(),
            supporting_statement_ids: vec!["stmt_tlast_high".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        assert!(
            semantic_ir.temporal_conflicts.is_empty(),
            "ASSERTED must stay polarity-relative when the signal polarity is unknown"
        );

        Ok(())
    }

    #[test]
    fn asserted_and_high_form_temporal_conflict_for_active_low_signal() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_active_low_values.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal clk is input width 1.\n\n",
                "Signal ARESETN is input width 1.\n\n",
                "Clock clk.\n\n",
                "ARESETN is an active low reset signal.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_aresetn_asserted".to_string(),
            subject_signal: "ARESETN".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "ARESETN must be asserted.".to_string(),
            supporting_statement_ids: vec!["stmt_aresetn_asserted".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_aresetn_high".to_string(),
            subject_signal: "ARESETN".to_string(),
            constraint_kind: SignalConstraintKind::MustBeHigh,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "ARESETN must be HIGH.".to_string(),
            supporting_statement_ids: vec!["stmt_aresetn_high".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        assert_eq!(semantic_ir.temporal_conflicts.len(), 1);
        let conflict = &semantic_ir.temporal_conflicts[0];
        assert_eq!(conflict.signal_name, "ARESETN");
        assert_eq!(
            conflict.conflicting_values,
            vec!["HIGH".to_string(), "LOW".to_string()]
        );
        assert_eq!(conflict.automation_confidence, AutomationConfidence::Medium);

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

    #[test]
    fn width_only_signal_declarations_become_interface_signal_records() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("width_only.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(&source, "# Protocol\nSignal AWVALID is width 1.\n")?;

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

        let awvalid = semantic_ir
            .interfaces
            .iter()
            .flat_map(|interface| interface.signal_records.iter())
            .find(|signal| signal.signal_name == "AWVALID")
            .expect("expected width-only AWVALID declaration to survive");
        assert_eq!(awvalid.width_hint, Some(WidthHint::Numeric(1)));
        assert_eq!(awvalid.direction_hint, None);

        Ok(())
    }

    #[test]
    fn width_only_width_parameter_declarations_do_not_become_interface_signal_records() -> Result<()>
    {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("width_param_only.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# Protocol\nSignal DATA_WIDTH is width 32.\n\nSignal AWVALID is width DATA_WIDTH.\n",
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

        assert!(
            semantic_ir
                .interfaces
                .iter()
                .flat_map(|interface| interface.signal_records.iter())
                .all(|signal| signal.signal_name != "DATA_WIDTH"),
            "width-parameter declarations should not survive as interface signals"
        );
        assert!(
            semantic_ir
                .interfaces
                .iter()
                .flat_map(|interface| interface.signal_records.iter())
                .any(|signal| signal.signal_name == "AWVALID"),
            "ordinary width-only signals must still survive"
        );

        Ok(())
    }

    #[test]
    fn extract_signal_tokens_rejects_leading_digit_hex_like_values() {
        let signals = super::extract_signal_tokens("0A, 0B, 0E, 0F, TVALID, TREADY, rst_n");

        assert_eq!(
            signals,
            vec![
                "RST_N".to_string(),
                "TREADY".to_string(),
                "TVALID".to_string()
            ]
        );
    }

    #[test]
    fn filtered_interface_candidate_signals_drop_width_and_table_metadata_noise() {
        let filtered = super::filtered_interface_candidate_signals(&[
            "TDATA_WIDTH".to_string(),
            "TKEEP".to_string(),
            "MIN".to_string(),
            "MAX".to_string(),
            "_WIDTH".to_string(),
            "TVALID".to_string(),
        ]);

        assert_eq!(filtered, vec!["TKEEP".to_string(), "TVALID".to_string()]);
    }

    #[test]
    fn retain_authoritative_interface_candidate_signals_prefers_declared_surface() {
        let authoritative = std::collections::BTreeSet::from([
            "PADDR".to_string(),
            "PENABLE".to_string(),
            "PREADY".to_string(),
        ]);

        let filtered = super::retain_authoritative_interface_candidate_signals(
            &[
                "SETUP".to_string(),
                "PADDR".to_string(),
                "ACCESS".to_string(),
                "PREADY".to_string(),
            ],
            &authoritative,
            "PADDR and PREADY participate in the transfer.",
        );

        assert_eq!(filtered, vec!["PADDR".to_string(), "PREADY".to_string()]);
    }

    #[test]
    fn retain_authoritative_interface_candidate_signals_fails_closed_without_authority() {
        let filtered = super::retain_authoritative_interface_candidate_signals(
            &["A0".to_string(), "D0".to_string(), "F0".to_string()],
            &std::collections::BTreeSet::new(),
            "| F0 84 | 01 | A0 | D0 |",
        );

        assert!(
            filtered.is_empty(),
            "statement tokens cannot bootstrap a signal surface without typed authority"
        );
    }

    #[test]
    fn retain_authoritative_interface_candidate_signals_keeps_grounded_signal_behavior() {
        let filtered = super::retain_authoritative_interface_candidate_signals(
            &["READY".to_string(), "VALID".to_string()],
            &std::collections::BTreeSet::new(),
            "VALID must remain asserted until READY is observed.",
        );

        assert_eq!(filtered, vec!["READY".to_string(), "VALID".to_string()]);
    }

    #[test]
    fn authority_empty_statement_tokens_do_not_become_interfaces() {
        let context = make_semantic_context_with_statements(vec![super::StatementContext {
            statement_id: "statement_encoding_row".to_string(),
            class: super::StatementClass::SourceFact,
            text: "| F0 84 | 01 | A0 | A4 | D5 |".to_string(),
            related_visual_evidence_ids: vec![],
            section_ids: vec!["section_encoding".to_string()],
            signals: vec![
                "A0".to_string(),
                "A4".to_string(),
                "D5".to_string(),
                "F0".to_string(),
            ],
            supporting_table_ids: vec!["table_encoding".to_string()],
        }]);

        let (interfaces, conflicts) = super::build_interfaces(&context, None, &[], None);

        assert!(interfaces.is_empty());
        assert!(conflicts.is_empty());
    }

    #[test]
    fn redundant_authoritative_polarity_candidate_is_not_a_heuristic_interface() {
        let authoritative =
            std::collections::BTreeSet::from(["CS_N".to_string(), "WE_N".to_string()]);

        assert!(
            super::heuristic_interface_candidate_is_redundant_with_authoritative_surface(
                &["CS_N".to_string()],
                &authoritative,
                "CS_N must be asserted.",
            )
        );
        assert!(
            super::heuristic_interface_candidate_is_redundant_with_authoritative_surface(
                &["CS_N".to_string(), "WE_N".to_string()],
                &authoritative,
                "CS_N and WE_N are active LOW signals.",
            )
        );
        assert!(
            !super::heuristic_interface_candidate_is_redundant_with_authoritative_surface(
                &["CS_N".to_string(), "WE_N".to_string()],
                &authoritative,
                "CS_N and WE_N change together.",
            )
        );
        assert!(
            !super::heuristic_interface_candidate_is_redundant_with_authoritative_surface(
                &["CS_N".to_string(), "PREADY".to_string()],
                &authoritative,
                "CS_N and PREADY are active LOW signals.",
            )
        );
    }

    #[test]
    fn overlapping_interface_signals_ignore_fragments_subsumed_by_explicit_interfaces() {
        let interfaces = vec![
            super::InterfaceRecord {
                interface_id: "interface_explicit_document_interface".to_string(),
                signals: vec![
                    "ACLK".to_string(),
                    "TREADY".to_string(),
                    "TVALID".to_string(),
                ],
                signal_records: Vec::new(),
                supporting_statement_ids: Vec::new(),
            },
            super::InterfaceRecord {
                interface_id: "interface_aclk_tvalid".to_string(),
                signals: vec!["ACLK".to_string(), "TVALID".to_string()],
                signal_records: Vec::new(),
                supporting_statement_ids: Vec::new(),
            },
            super::InterfaceRecord {
                interface_id: "interface_tready_tvalid".to_string(),
                signals: vec!["TREADY".to_string(), "TVALID".to_string()],
                signal_records: Vec::new(),
                supporting_statement_ids: Vec::new(),
            },
        ];

        assert!(super::overlapping_interface_signals(&interfaces).is_empty());
    }

    #[test]
    fn overlapping_interface_signals_keep_unsubsumed_heuristic_overlap_visible() {
        let interfaces = vec![
            super::InterfaceRecord {
                interface_id: "interface_a_b".to_string(),
                signals: vec!["XA".to_string(), "XB".to_string()],
                signal_records: Vec::new(),
                supporting_statement_ids: Vec::new(),
            },
            super::InterfaceRecord {
                interface_id: "interface_a_c".to_string(),
                signals: vec!["XA".to_string(), "XC".to_string()],
                signal_records: Vec::new(),
                supporting_statement_ids: Vec::new(),
            },
        ];

        assert_eq!(
            super::overlapping_interface_signals(&interfaces),
            vec!["XA".to_string()]
        );
    }

    #[test]
    fn system_contract_emits_infrastructure_records_without_actor_ports() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("clock_reset_contract_only.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Contract\n",
                "Clock ACLK.\n",
                "Reset ARESETN is asynchronous active low.\n",
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

        assert!(semantic_ir.actor_ports.is_empty());
        assert!(semantic_ir.signal_connectivity.is_empty());
        assert_eq!(semantic_ir.infrastructure_signals.len(), 2);
        assert!(semantic_ir.infrastructure_signals.iter().all(|record| {
            record.source_status
                == crate::ir::semantic::InfrastructureSignalSourceStatus::UnresolvedSource
                && record.distribution_status
                    == crate::ir::semantic::InfrastructureSignalDistributionStatus::NoRecoveredConsumers
                && record.recovered_source_actor_ids.is_empty()
                && record.distributed_to_actor_ids.is_empty()
                && record.automation_confidence == AutomationConfidence::High
                && !record.supporting_statement_ids.is_empty()
        }));

        Ok(())
    }

    #[test]
    fn explicit_clock_generator_recovers_infrastructure_source_status() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("clock_generator_source.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Contract\n",
                "Clock ACLK.\n",
                "Reset ARESETN is asynchronous active low.\n",
                "The clock generator drives ACLK.\n",
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
        evidence_ir.extracted_statements.push(ExtractedStatement {
            statement_id: "statement_clock_generator".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: "The clock generator drives ACLK.".to_string(),
            evidence_span_ids: Vec::new(),
            related_visual_evidence_ids: Vec::new(),
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        let clock_infrastructure = semantic_ir
            .infrastructure_signals
            .iter()
            .find(|record| record.signal_name == "ACLK")
            .expect("expected ACLK infrastructure record");

        assert_eq!(
            clock_infrastructure.source_status,
            crate::ir::semantic::InfrastructureSignalSourceStatus::RecoveredProducer
        );
        assert!(
            clock_infrastructure
                .recovered_source_actor_names
                .contains(&"clock generator".to_string()),
            "expected explicit source actor in infrastructure record: {:?}",
            clock_infrastructure
        );
        assert_eq!(
            clock_infrastructure.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(!clock_infrastructure.supporting_statement_ids.is_empty());
        assert!(
            semantic_ir
                .actor_ports
                .iter()
                .all(|port| port.actor_name != "clock generator"),
            "clock generator evidence should not have to become an ordinary protocol actor port"
        );

        Ok(())
    }

    #[test]
    fn explicit_clock_distribution_recovers_infrastructure_targets() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("clock_distribution.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Contract\n",
                "Clock ACLK.\n",
                "Reset ARESETN is asynchronous active low.\n",
                "ACLK is distributed to the Requester and Completer.\n",
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
        evidence_ir.extracted_statements.push(ExtractedStatement {
            statement_id: "statement_clock_distribution".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: "ACLK is distributed to the Requester and Completer.".to_string(),
            evidence_span_ids: Vec::new(),
            related_visual_evidence_ids: Vec::new(),
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        let clock_infrastructure = semantic_ir
            .infrastructure_signals
            .iter()
            .find(|record| record.signal_name == "ACLK")
            .expect("expected ACLK infrastructure record");

        assert_eq!(
            clock_infrastructure.distribution_status,
            crate::ir::semantic::InfrastructureSignalDistributionStatus::SharedRecoveredConsumers
        );
        assert!(
            clock_infrastructure
                .distributed_to_actor_names
                .contains(&"Requester".to_string())
        );
        assert!(
            clock_infrastructure
                .distributed_to_actor_names
                .contains(&"Completer".to_string())
        );
        assert!(!clock_infrastructure.supporting_statement_ids.is_empty());
        assert!(
            semantic_ir.actor_ports.is_empty(),
            "distribution-only evidence should not create ordinary protocol actor ports"
        );

        Ok(())
    }

    #[test]
    fn explicit_reset_synchronizer_fanout_recovers_source_and_target() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("reset_synchronizer_fanout.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Contract\n",
                "Clock ACLK.\n",
                "Reset ARESETN is asynchronous active low.\n",
                "The reset synchronizer feeds ARESETN to the Requester.\n",
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
        evidence_ir.extracted_statements.push(ExtractedStatement {
            statement_id: "statement_reset_synchronizer".to_string(),
            class: StatementClass::SourceFact,
            modality: EvidenceModality::Text,
            text: "The reset synchronizer feeds ARESETN to the Requester.".to_string(),
            evidence_span_ids: Vec::new(),
            related_visual_evidence_ids: Vec::new(),
        });
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        let reset_infrastructure = semantic_ir
            .infrastructure_signals
            .iter()
            .find(|record| record.signal_name == "ARESETN")
            .expect("expected ARESETN infrastructure record");

        assert_eq!(
            reset_infrastructure.source_status,
            crate::ir::semantic::InfrastructureSignalSourceStatus::RecoveredProducer
        );
        assert_eq!(
            reset_infrastructure.distribution_status,
            crate::ir::semantic::InfrastructureSignalDistributionStatus::SingleRecoveredConsumer
        );
        assert!(
            reset_infrastructure
                .recovered_source_actor_names
                .contains(&"reset synchronizer".to_string())
        );
        assert!(
            reset_infrastructure
                .distributed_to_actor_names
                .contains(&"Requester".to_string())
        );
        assert_eq!(
            reset_infrastructure.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(!reset_infrastructure.supporting_statement_ids.is_empty());
        assert!(
            semantic_ir.actor_ports.is_empty(),
            "explicit reset fanout evidence should stay in the infrastructure surface"
        );

        Ok(())
    }

    #[test]
    fn explicit_clock_reset_topology_recovers_only_current_document_evidence() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("clock_reset_topology.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Contract\n",
                "Clock ACLK.\n",
                "Reset ARESETN is asynchronous active low.\n",
                "The ACLK clock gate CGATE0 feeds the Requester branch.\n",
                "The two-stage reset synchronizer RSTSYNC0 feeds ARESETN to the Requester.\n",
                "The ARESETN reset tree targets the Requester registers and Completer registers.\n",
                "The ACLK clock gate policy should avoid glitches.\n",
                "ARESETN may use a synchronizer in some implementations.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        for (statement_id, text) in [
            ("statement_clock", "Clock ACLK."),
            (
                "statement_reset",
                "Reset ARESETN is asynchronous active low.",
            ),
            (
                "statement_clock_gate",
                "The ACLK clock gate CGATE0 feeds the Requester branch.",
            ),
            (
                "statement_reset_sync",
                "The two-stage reset synchronizer RSTSYNC0 feeds ARESETN to the Requester.",
            ),
            (
                "statement_reset_tree",
                "The ARESETN reset tree targets the Requester registers and Completer registers.",
            ),
            (
                "statement_vague_clock_gate",
                "The ACLK clock gate policy should avoid glitches.",
            ),
            (
                "statement_vague_sync",
                "ARESETN may use a synchronizer in some implementations.",
            ),
        ] {
            evidence_ir.extracted_statements.push(ExtractedStatement {
                statement_id: statement_id.to_string(),
                class: StatementClass::SourceFact,
                modality: EvidenceModality::Text,
                text: text.to_string(),
                evidence_span_ids: Vec::new(),
                related_visual_evidence_ids: Vec::new(),
            });
        }
        evidence_ir.write_to_disk()?;

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        let clock_infrastructure = semantic_ir
            .infrastructure_signals
            .iter()
            .find(|record| record.signal_name == "ACLK")
            .expect("expected ACLK infrastructure record");
        assert_eq!(clock_infrastructure.infrastructure_topology.len(), 1);
        let clock_gate = &clock_infrastructure.infrastructure_topology[0];
        assert_eq!(
            clock_gate.topology_kind,
            InfrastructureTopologyKind::ClockGatedBranch
        );
        assert_eq!(clock_gate.component_name.as_deref(), Some("CGATE0"));
        assert_eq!(clock_gate.stage_count, None);
        assert_eq!(clock_gate.target_actor_names, vec!["Requester".to_string()]);
        assert_eq!(
            clock_gate.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(!clock_gate.supporting_statement_id.is_empty());
        assert_ne!(
            clock_gate.supporting_statement_id,
            "statement_vague_clock_gate"
        );
        assert_ne!(clock_gate.supporting_statement_id, "statement_vague_sync");
        assert!(!clock_infrastructure.supporting_statement_ids.is_empty());

        let reset_infrastructure = semantic_ir
            .infrastructure_signals
            .iter()
            .find(|record| record.signal_name == "ARESETN")
            .expect("expected ARESETN infrastructure record");
        assert_eq!(
            reset_infrastructure
                .infrastructure_topology
                .iter()
                .filter(|record| record.topology_kind
                    == InfrastructureTopologyKind::ResetSynchronizerStages)
                .count(),
            1,
            "only the explicit two-stage synchronizer sentence should create a stage record"
        );
        assert_eq!(
            reset_infrastructure
                .infrastructure_topology
                .iter()
                .filter(
                    |record| record.topology_kind == InfrastructureTopologyKind::ResetTreeTargets
                )
                .count(),
            1
        );
        let synchronizer = reset_infrastructure
            .infrastructure_topology
            .iter()
            .find(|record| {
                record.topology_kind == InfrastructureTopologyKind::ResetSynchronizerStages
            })
            .expect("expected reset synchronizer topology");
        assert_eq!(synchronizer.component_name.as_deref(), Some("RSTSYNC0"));
        assert_eq!(synchronizer.stage_count, Some(2));
        assert_eq!(
            synchronizer.target_actor_names,
            vec!["Requester".to_string()]
        );
        assert_eq!(
            synchronizer.automation_confidence,
            AutomationConfidence::Medium
        );
        let reset_tree = reset_infrastructure
            .infrastructure_topology
            .iter()
            .find(|record| record.topology_kind == InfrastructureTopologyKind::ResetTreeTargets)
            .expect("expected reset tree topology");
        assert_eq!(
            reset_tree.target_actor_names,
            vec!["Requester".to_string(), "Completer".to_string()]
        );
        assert_eq!(
            reset_tree.automation_confidence,
            AutomationConfidence::Medium
        );
        assert!(!reset_infrastructure.supporting_statement_ids.is_empty());
        assert!(
            semantic_ir.actor_ports.is_empty(),
            "topology-only evidence should stay in the infrastructure surface"
        );

        Ok(())
    }

    #[test]
    fn infrastructure_source_actor_is_not_marked_as_own_consumer() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("pll_clock_source.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Signals\n",
                "Signal ACLK is input width 1.\n",
                "Signal ARESETN is input width 1.\n",
                "Signal XREQ is output width 1.\n\n",
                "# Contract\n",
                "Clock ACLK.\n",
                "Reset ARESETN is asynchronous active low.\n",
                "The PLL generates ACLK.\n",
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
        let pll_clock_port = semantic_ir
            .actor_ports
            .iter()
            .find(|port| port.actor_name == "PLL" && port.signal_name == "ACLK")
            .expect("expected PLL to remain the recovered ACLK source");
        assert_eq!(pll_clock_port.direction, ActorRelativeDirection::Output);
        assert!(
            semantic_ir.actor_ports.iter().all(|port| {
                !(port.actor_name == "PLL"
                    && port.signal_name == "ACLK"
                    && port.direction == ActorRelativeDirection::Input)
            }),
            "PLL should not be added as its own ACLK consumer"
        );

        let clock_infrastructure = semantic_ir
            .infrastructure_signals
            .iter()
            .find(|record| record.signal_name == "ACLK")
            .expect("expected ACLK infrastructure record");
        assert_eq!(
            clock_infrastructure.source_status,
            crate::ir::semantic::InfrastructureSignalSourceStatus::RecoveredProducer
        );
        assert!(
            clock_infrastructure
                .recovered_source_actor_names
                .contains(&"PLL".to_string())
        );
        assert!(
            !clock_infrastructure
                .distributed_to_actor_names
                .contains(&"PLL".to_string()),
            "PLL source should not be counted as a recovered ACLK distribution target"
        );

        Ok(())
    }

    #[test]
    fn clock_and_reset_gain_input_actor_ports_for_relation_actors() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("clock_reset_ports.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Signals\n",
                "Signal ACLK is input width 1.\n",
                "Signal ARESETN is input width 1.\n",
                "\n",
                "# Contract\n",
                "Clock ACLK.\n",
                "Reset ARESETN is asynchronous active low.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_signal_desc".to_string(),
            asset_id: "asset_signal_desc".to_string(),
            page_id: None,
            caption_text: Some("Role-grounded signal descriptions".to_string()),
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
                    make_table_cell("XREQ", false),
                    make_table_cell("Requester", false),
                    make_table_cell("1", false),
                    make_table_cell("Transfer request", false),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell("Completer", false),
                    make_table_cell("1", false),
                    make_table_cell("Transfer accept", false),
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

        for actor_name in ["Requester", "Completer"] {
            for signal_name in ["ACLK", "ARESETN"] {
                assert!(
                    semantic_ir.actor_ports.iter().any(|port| {
                        port.actor_name == actor_name
                            && port.signal_name == signal_name
                            && port.direction == ActorRelativeDirection::Input
                    }),
                    "expected {actor_name} to receive {signal_name} as an input actor port"
                );
            }
        }
        assert!(semantic_ir.signal_connectivity.iter().any(|record| {
            record.signal_name == "ACLK"
                && record.connectivity_class
                    == crate::ir::semantic::SignalConnectivityClass::SystemClock
                && record.producer_actor_ids.is_empty()
        }));
        assert!(semantic_ir.signal_connectivity.iter().any(|record| {
            record.signal_name == "ARESETN"
                && record.connectivity_class
                    == crate::ir::semantic::SignalConnectivityClass::SystemReset
                && record.producer_actor_ids.is_empty()
        }));
        let clock_infrastructure = semantic_ir
            .infrastructure_signals
            .iter()
            .find(|record| record.signal_name == "ACLK")
            .expect("expected first-class ACLK infrastructure record");
        assert_eq!(
            clock_infrastructure.kind,
            crate::ir::semantic::InfrastructureSignalKind::SystemClock
        );
        assert_eq!(
            clock_infrastructure.source_status,
            crate::ir::semantic::InfrastructureSignalSourceStatus::UnresolvedSource
        );
        assert_eq!(
            clock_infrastructure.distribution_status,
            crate::ir::semantic::InfrastructureSignalDistributionStatus::SharedRecoveredConsumers
        );
        assert_eq!(clock_infrastructure.distributed_to_actor_names.len(), 2);

        let reset_infrastructure = semantic_ir
            .infrastructure_signals
            .iter()
            .find(|record| record.signal_name == "ARESETN")
            .expect("expected first-class ARESETN infrastructure record");
        assert_eq!(
            reset_infrastructure.kind,
            crate::ir::semantic::InfrastructureSignalKind::SystemReset
        );
        assert_eq!(
            reset_infrastructure.source_status,
            crate::ir::semantic::InfrastructureSignalSourceStatus::UnresolvedSource
        );
        assert_eq!(
            reset_infrastructure.distribution_status,
            crate::ir::semantic::InfrastructureSignalDistributionStatus::SharedRecoveredConsumers
        );
        assert_eq!(reset_infrastructure.distributed_to_actor_names.len(), 2);

        Ok(())
    }

    // ── parse_explicit_system_reset unit tests ──────────────────────────

    #[test]
    fn parse_explicit_system_reset_returns_none_for_non_reset_text() {
        // tokens.len() >= 4 but tokens[0] != "reset" — catches ||→&& mutant
        assert!(super::parse_explicit_system_reset("Signal clk is input width 1.").is_none());
    }

    #[test]
    fn parse_explicit_system_reset_returns_none_for_short_text() {
        // tokens.len() < 4 — guard clause
        assert!(super::parse_explicit_system_reset("Reset").is_none());
    }

    #[test]
    fn parse_explicit_system_reset_parses_signal_keyword_form() {
        // "reset signal X is ..." — catches >=→< mutant
        let result =
            super::parse_explicit_system_reset("Reset signal rst_n is synchronous active low.");
        assert!(result.is_some());
    }

    // tokenize_control_expression unit tests

    #[test]
    fn tokenize_two_char_operator_not_equals() {
        // Catches +→* mutant at line 5338 — pair character at index*1 picks wrong char
        let tokens = super::tokenize_control_expression("x != y").expect("tokenize 'x != y'");
        assert_eq!(tokens, vec!["x", "!=", "y"]);
    }

    #[test]
    fn tokenize_identifier_starting_with_underscore() {
        // Catches ==→!= (underscore) mutant at line 5371 — _ as first char hits
        // the outer-while condition, not the inner scanner loop
        let tokens = super::tokenize_control_expression("_foo").expect("tokenize '_foo'");
        assert_eq!(tokens, vec!["_foo"]);
    }

    #[test]
    fn tokenize_identifier_starting_with_apostrophe() {
        // Catches ==→!= (apostrophe) mutant at line 5371
        let tokens = super::tokenize_control_expression("'bar").expect("tokenize '''bar''");
        assert_eq!(tokens, vec!["'bar"]);
    }

    #[test]
    fn tokenize_identifier_with_underscore() {
        // Catches ||→&& mutant at line 5371 — 'm' fails && character == '_'
        let tokens = super::tokenize_control_expression("my_signal").expect("tokenize 'my_signal'");
        assert_eq!(tokens, vec!["my_signal"]);
    }

    // extract_actor_after_by unit tests

    #[test]
    fn extract_actor_after_by_returns_actor_following_by_with_the_prefix() {
        // Catches return None, return Some(""), return Some("xyzzy") mutants
        let actor = super::extract_actor_after_by("driven by the CPU").expect("should find actor");
        assert!(!actor.is_empty());
        assert_eq!(actor, "CPU");
    }

    #[test]
    fn extract_actor_after_by_handles_no_the_prefix() {
        // Catches +→- and +→* mutants at line 9386 — offset into "by " + 4
        let actor =
            super::extract_actor_after_by("managed by DMA engine").expect("should find actor");
        assert_eq!(actor, "DMA engine");
    }

    #[test]
    fn extract_actor_after_by_returns_none_when_no_by() {
        assert!(super::extract_actor_after_by("some text without the keyword").is_none());
    }

    // signal_constraint_kind_from_vlm_state unit tests

    #[test]
    fn vlm_state_x_returns_none() {
        // Catches delete match arm for "x" | "z" | "unknown" | ...
        let result = super::signal_constraint_kind_from_vlm_state("x");
        assert!(result.is_none());
    }

    #[test]
    fn vlm_state_z_returns_none() {
        let result = super::signal_constraint_kind_from_vlm_state("z");
        assert!(result.is_none());
    }

    #[test]
    fn vlm_state_unknown_returns_none() {
        let result = super::signal_constraint_kind_from_vlm_state("unknown");
        assert!(result.is_none());
    }

    // is_vlm_waveform_motion_state unit tests

    #[test]
    fn is_vlm_waveform_motion_state_returns_false_for_non_motion_state() {
        // Catches return true mutant
        assert!(!super::is_vlm_waveform_motion_state("idle"));
    }

    #[test]
    fn is_vlm_waveform_motion_state_returns_true_for_rise() {
        assert!(super::is_vlm_waveform_motion_state("rise"));
    }

    // is_signal_value_annotation_label unit tests

    #[test]
    fn signal_value_annotation_label_requires_both_match_and_trailing_cycle() {
        // Catches &&→|| mutant at line 9554 — trailing tokens must also validate
        let mut known_signals = HashSet::new();
        known_signals.insert("SIG1".to_string());
        // "1" matches MustBeHigh (first condition true), "stuff" fails cycle marker check
        assert!(!super::is_signal_value_annotation_label(
            "SIG1 1 stuff",
            &known_signals
        ));
    }

    // parse_allowed_vlm_observation_signal unit tests

    #[test]
    fn allowed_vlm_signal_rejects_generic_term_when_known_set_empty() {
        // Catches delete ! and &&→|| mutants at line 9687
        let known_signals = HashSet::new();
        assert!(super::parse_allowed_vlm_observation_signal("Transfer", &known_signals).is_none());
    }

    #[test]
    fn allowed_vlm_signal_rejects_unknown_signal_when_known_set_non_empty() {
        // Catches &&→|| mutant at line 9687 — || would allow any non-generic signal
        let mut known_signals = HashSet::new();
        known_signals.insert("SIG1".to_string());
        assert!(super::parse_allowed_vlm_observation_signal("mysig", &known_signals).is_none());
    }

    // normalize_vlm_waveform_motion_state unit tests

    #[test]
    fn normalize_motion_state_splits_on_underscore() {
        // Catches first ||→&& mutant at line 9777 col 30 — underscore stop splitting
        let normalized = super::normalize_vlm_waveform_motion_state("rising_edge");
        assert_eq!(normalized, "rising edge");
    }

    #[test]
    fn normalize_motion_state_splits_on_hyphen() {
        // Catches second ||→&& mutant at line 9777 col 50 — hyphen stop splitting
        let normalized = super::normalize_vlm_waveform_motion_state("rise-fall");
        assert_eq!(normalized, "rise fall");
    }

    // is_non_quantitative_waveform_motion_annotation unit tests

    #[test]
    fn non_quantitative_motion_annotation_recognizes_rise() {
        // Catches first ||→&& mutant at line 9817 col 33 — alphanumeric stops being kept
        assert!(super::is_non_quantitative_waveform_motion_annotation(
            "rise"
        ));
    }

    #[test]
    fn non_quantitative_motion_annotation_rejects_underscored_non_motion_combination() {
        // Catches second ||→&& mutant at line 9817 col 69 — underscore stops being a token char.
        // "rose_fell": combined token "rose_fell" → no motion match → original returns false.
        // With mutation, splits into ["rose", "fell"] → "rose" matches fallback → returns true.
        assert!(!super::is_non_quantitative_waveform_motion_annotation(
            "rose_fell"
        ));
    }

    #[test]
    fn non_quantitative_motion_annotation_rejects_text_without_motion_tokens() {
        // Catches ||→&& mutant at line 9823 — mutation would skip the early-return guard
        assert!(!super::is_non_quantitative_waveform_motion_annotation(
            "hello world"
        ));
    }

    // is_timing_annotation_constraint_token unit tests

    #[test]
    fn timing_annotation_constraint_token_recognizes_setup() {
        // Catches return false mutant
        assert!(super::is_timing_annotation_constraint_token("setup"));
    }

    #[test]
    fn timing_annotation_constraint_token_rejects_unknown() {
        assert!(!super::is_timing_annotation_constraint_token("foobar"));
    }

    // is_compact_waveform_sample_label unit tests

    #[test]
    fn compact_sample_label_accepts_hex_prefix() {
        assert!(super::is_compact_waveform_sample_label("0x1A"));
    }

    #[test]
    fn compact_sample_label_rejects_empty_hex_suffix() {
        // Catches &&→|| mutant at line 9952 — empty hex digits must be rejected
        assert!(!super::is_compact_waveform_sample_label("0x"));
    }

    #[test]
    fn compact_sample_label_accepts_binary_prefix() {
        assert!(super::is_compact_waveform_sample_label("0b101"));
    }

    #[test]
    fn compact_sample_label_rejects_empty_binary_suffix() {
        // Catches &&→|| at line 9958 and delete ! at line 9957
        assert!(!super::is_compact_waveform_sample_label("0b"));
    }

    #[test]
    fn compact_sample_label_accepts_named_prefix_with_digit_suffix() {
        assert!(super::is_compact_waveform_sample_label("addr5"));
    }

    #[test]
    fn compact_sample_label_rejects_empty_named_prefix_suffix() {
        // Catches &&→|| mutant at line 9983
        assert!(!super::is_compact_waveform_sample_label("data"));
    }

    #[test]
    fn compact_sample_label_rejects_unmatched_prefix_with_digits() {
        // Catches &&→|| mutant at line 9993 — 'x' doesnt match 'a'|'d'
        assert!(!super::is_compact_waveform_sample_label("x42"));
    }

    #[test]
    fn compact_sample_label_accepts_single_char_prefix_with_digits() {
        // Catches &&→|| mutant at line 9994
        assert!(super::is_compact_waveform_sample_label("a0"));
    }

    // extract_symbolic_value unit tests

    #[test]
    fn extract_symbolic_value_returns_uppercase_token() {
        // Catches delete ! at 9367 (uppercase check), &&→|| and !=→== and delete ! at 9361
        let value = super::extract_symbolic_value("signal HELLO there", None);
        assert_eq!(value, Some("HELLO".to_string()));
    }

    #[test]
    fn extract_symbolic_value_filters_excluded_keywords() {
        // HIGH is a keyword that should be filtered
        let value = super::extract_symbolic_value("set HIGH signal", None);
        assert_eq!(value, None);
    }

    #[test]
    fn extract_symbolic_value_excludes_specified_signal() {
        let value = super::extract_symbolic_value("MY_SIG and OTHER", Some("MY_SIG"));
        assert_eq!(value, Some("OTHER".to_string()));
    }

    #[test]
    fn extract_symbolic_value_returns_none_for_all_lowercase() {
        let value = super::extract_symbolic_value("hello world", None);
        assert_eq!(value, None);
    }

    #[test]
    fn extract_symbolic_value_returns_none_for_whitespace_only() {
        // Catches delete ! at 9362 — empty tokens kept instead of filtered
        let value = super::extract_symbolic_value("   ", None);
        assert_eq!(value, None);
    }

    // find_ascii_case_insensitive unit tests

    #[test]
    fn find_ascii_case_insensitive_returns_correct_index() {
        // Catches return None and return Some(0)
        let index = super::find_ascii_case_insensitive("Hello World", "world");
        assert_eq!(index, Some(6));
    }

    #[test]
    fn find_ascii_case_insensitive_returns_none_for_no_match() {
        assert_eq!(super::find_ascii_case_insensitive("hello", "xyz"), None);
    }

    // is_boilerplate_section_title unit tests

    #[test]
    fn boilerplate_title_recognizes_licence() {
        // Catches return false
        assert!(super::is_boilerplate_section_title("Licence Agreement"));
    }

    #[test]
    fn boilerplate_title_recognizes_license() {
        assert!(super::is_boilerplate_section_title("License Information"));
    }

    #[test]
    fn boilerplate_title_recognizes_proprietary_notice_without_licence() {
        // Catches ||→&& mutant at line 10362 — single || means either licence/license matches
        assert!(super::is_boilerplate_section_title("Proprietary Notice"));
    }

    #[test]
    fn legal_statement_classifier_requires_compound_legal_context() {
        for legal in [
            "The limited permissions granted above are perpetual and will not be revoked by successors or assigns while the specification is current.",
            "This document and derivative works may be copied when the copyright notice remains attached.",
            "The information is provided without warranty and the supplier accepts no liability for this document.",
            "This specification is distributed under a royalty-free license agreement.",
            "These patents were reviewed to avoid infringement of intellectual property rights.",
            "Products are sold under the terms and conditions of commercial sale agreed with the customer.",
            "Essential IPRs may have been declared for this deliverable.",
        ] {
            assert!(
                super::is_legal_or_administrative_statement(legal),
                "legal/administrative prose should be excluded: {legal}"
            );
        }

        for technical in [
            "The controller grants write permission after ownership transfer.",
            "The access rights field is assigned while the request remains valid.",
            "When the current protocol version is selected, the receiver must acknowledge.",
            "Some parameters use the ASCII string Copyright and pad it to the right.",
            "The license register records whether the feature is enabled.",
            "A RAM reports a fault if reliability is degraded.",
        ] {
            assert!(
                !super::is_legal_or_administrative_statement(technical),
                "technical permission/version/lifecycle prose must survive: {technical}"
            );
        }
    }

    #[test]
    fn administrative_section_classifier_requires_non_engineering_context() {
        for administrative in [
            "References",
            "2.1 Normative references",
            "3 Informative references",
            "1.3 Requesting a product to be included on the Certified list",
            "1.4 Requesting licence to display the certification mark",
            "1.7 Independent Test Labs",
            "Requested information for internal processing",
            "Publication of Certified products",
        ] {
            assert!(
                super::is_administrative_section_title(administrative),
                "administrative section should be excluded: {administrative}"
            );
        }

        for technical in [
            "Table 35. Format of PML4E that References a Page Table",
            "References to the architectural state",
            "Request and Response Channel",
            "Receiver compliance",
            "Compliance mode requirements",
            "Independent reset test logic",
            "Requested data internal pipeline",
        ] {
            assert!(
                !super::is_administrative_section_title(technical),
                "technical section must survive: {technical}"
            );
        }
    }

    #[test]
    fn administrative_statement_classifier_requires_organizational_workflow() {
        for administrative in [
            "The following documents can be helpful when reading this specification.",
            "Upon receipt of the request, the review team will respond via email regarding its status.",
            "All requests for conflict resolution must be submitted to the committee chair by email.",
            "A conflict resolution process lets members challenge products issued a certification logo or third-party test-lab results.",
            "A derivative product can apply for logo usage through a new listing request.",
            "When submitting a product for approval, the manufacturer shall assert that the certified requirements are met.",
            "Organizations wishing to display the certification mark must accept the trademark license.",
            "The contracted auditor and test lab administer third party testing for the certification program.",
            "Products are expected to be tested in an out-of-box state before being offered in the market.",
        ] {
            assert!(
                super::is_legal_or_administrative_statement(administrative),
                "administrative workflow should be excluded: {administrative}"
            );
        }

        for technical in [
            "If the Page Request completes, the response status is Success.",
            "Upon receiving a Link State Status Request Packet, the manager shall respond with Not Ready.",
            "The conditions for inactive contacts shall apply to contact C6.",
            "The arbiter resolves a write conflict before the next clock edge.",
            "A certified device shall meet all electrical requirements in the protocol definition.",
            "A certified controller shall assert READY when the protocol requirements are met.",
            "The test report shall record the measured receiver jitter tolerance.",
            "While READY is low, VALID must remain asserted.",
        ] {
            assert!(
                !super::is_legal_or_administrative_statement(technical),
                "technical protocol/compliance prose must survive: {technical}"
            );
        }
    }

    #[test]
    fn legal_conditions_under_generic_heading_do_not_reach_semantic_surfaces() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("legal-boundary.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Approved\n",
                "The limited permissions granted above will not be revoked by successors or assigns while this specification is current.\n\n",
                "This document and derivative works may be copied when the copyright notice remains attached.\n\n",
                "# Protocol Operation\n",
                "Signal READY is input width 1.\n\n",
                "Signal VALID is output width 1.\n\n",
                "While READY is low, VALID must remain asserted.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text.contains("permissions granted"))
        );
        assert!(
            evidence_ir
                .extracted_statements
                .iter()
                .any(|statement| statement.text.contains("derivative works"))
        );

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        assert!(semantic_ir.gates.iter().any(|gate| {
            gate.condition
                .contains("While READY is low, VALID must remain asserted.")
        }));
        assert!(
            semantic_ir
                .gates
                .iter()
                .all(|gate| !gate.condition.contains("permissions granted")
                    && !gate.condition.contains("derivative works"))
        );
        assert!(semantic_ir.invariants.iter().all(|invariant| {
            !invariant.statement.contains("permissions granted")
                && !invariant.statement.contains("derivative works")
        }));
        assert!(
            semantic_ir
                .phases
                .iter()
                .all(|phase| !phase.summary.contains("`Approved`"))
        );

        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        assert!(intent_ir.behaviors.iter().any(|behavior| {
            behavior
                .statement
                .contains("While READY is low, VALID must remain asserted.")
        }));
        assert!(intent_ir.behaviors.iter().all(|behavior| {
            !behavior.statement.contains("permissions granted")
                && !behavior.statement.contains("derivative works")
        }));

        Ok(())
    }

    #[test]
    fn administrative_workflows_remain_evidence_but_not_semantic_behavior() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("administrative-boundary.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# References\n",
                "The following documents can be helpful when reading this specification.\n\n",
                "# 1.3 Requesting a product to be included on the Certified list\n",
                "Upon receipt of a request, the review team will respond via email regarding its status.\n\n",
                "# Conflict Resolution\n",
                "All requests for conflict resolution must be submitted to the committee chair by email.\n\n",
                "# Derivative Products\n",
                "A derivative product can apply for logo usage through a new listing request.\n\n",
                "# Device Requirements\n",
                "When submitting a product for approval, the manufacturer shall assert that the certified requirements are met.\n\n",
                "A certified device shall meet all electrical requirements in the protocol definition.\n\n",
                "Signal READY is input width 1.\n\n",
                "Signal VALID is output width 1.\n\n",
                "While READY is low, VALID must remain asserted.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;
        for retained in [
            "helpful when reading",
            "respond via email",
            "conflict resolution",
            "derivative product",
            "submitting a product for approval",
        ] {
            assert!(
                evidence_ir
                    .extracted_statements
                    .iter()
                    .any(|statement| statement.text.contains(retained))
            );
        }

        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        for excluded in [
            "helpful when reading",
            "respond via email",
            "conflict resolution",
            "derivative product",
            "submitting a product for approval",
        ] {
            assert!(
                semantic_ir
                    .gates
                    .iter()
                    .all(|gate| !gate.condition.contains(excluded))
            );
            assert!(
                semantic_ir
                    .invariants
                    .iter()
                    .all(|invariant| !invariant.statement.contains(excluded))
            );
        }
        assert!(semantic_ir.phases.iter().all(|phase| {
            !phase.summary.contains("`References`")
                && !phase.summary.contains("Requesting a product")
        }));
        assert!(semantic_ir.invariants.iter().any(|invariant| {
            invariant
                .statement
                .contains("certified device shall meet all electrical requirements")
        }));
        assert!(semantic_ir.gates.iter().any(|gate| {
            gate.condition
                .contains("While READY is low, VALID must remain asserted.")
        }));

        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        assert!(intent_ir.behaviors.iter().all(|behavior| {
            !behavior.statement.contains("respond via email")
                && !behavior.statement.contains("conflict resolution")
                && !behavior.statement.contains("derivative product")
                && !behavior
                    .statement
                    .contains("submitting a product for approval")
        }));
        assert!(intent_ir.behaviors.iter().any(|behavior| {
            behavior
                .statement
                .contains("While READY is low, VALID must remain asserted.")
        }));

        Ok(())
    }

    // parse_identifier unit tests

    #[test]
    fn parse_identifier_accepts_underscore_prefix() {
        // Catches ==→!= mutant at line 5679 — _ must be accepted as first character
        let result = super::parse_identifier("_foo");
        assert_eq!(result, Some("_foo".to_string()));
    }

    #[test]
    fn parse_identifier_accepts_alphabetic_start() {
        let result = super::parse_identifier("hello_world");
        assert_eq!(result, Some("hello_world".to_string()));
    }

    // contains_text_phrase unit tests

    #[test]
    fn contains_text_phrase_rejects_word_after_underscore() {
        // Catches &&→|| mutants at lines 9345, 9351 — _ is not a word boundary
        assert!(!super::contains_text_phrase("hello_world", "world"));
    }

    #[test]
    fn contains_text_phrase_accepts_word_after_space() {
        assert!(super::contains_text_phrase("hello world", "world"));
    }

    #[test]
    fn contains_text_phrase_rejects_word_with_missing_prefix_boundary() {
        // Catches &&→|| mutant at line 9353 — both prefix and suffix must be boundaries
        assert!(!super::contains_text_phrase("worldhello", "world"));
    }

    // explicit_clock_edge_from_text unit tests

    #[test]
    fn clock_edge_recognizes_rising_edge_phrase() {
        // Catches ||→&& at line 8247 — single "rising edge" should match
        assert_eq!(
            super::explicit_clock_edge_from_text("on the rising edge"),
            Some(super::ClockEdge::Rising)
        );
    }

    #[test]
    fn clock_edge_recognizes_posedge_phrase() {
        // Catches ||→&& at line 8248 — single "posedge" should match
        assert_eq!(
            super::explicit_clock_edge_from_text("on posedge"),
            Some(super::ClockEdge::Rising)
        );
    }

    #[test]
    fn clock_edge_recognizes_posedges_phrase() {
        // Catches ||→&& at line 8249 — single "posedges" should match
        assert_eq!(
            super::explicit_clock_edge_from_text("2 posedges"),
            Some(super::ClockEdge::Rising)
        );
    }

    #[test]
    fn clock_edge_recognizes_falling_edge_phrase() {
        assert_eq!(
            super::explicit_clock_edge_from_text("on the falling edge"),
            Some(super::ClockEdge::Falling)
        );
    }

    #[test]
    fn clock_edge_returns_none_for_no_edge() {
        assert_eq!(super::explicit_clock_edge_from_text("no clock here"), None);
    }

    // contains_named_generic_edge_unit unit tests

    #[test]
    fn named_generic_edge_unit_returns_false_for_non_edge_tokens() {
        // Catches return true mutant
        let known_signals = BTreeSet::new();
        assert!(!super::contains_named_generic_edge_unit(
            &["hello", "world"],
            &known_signals
        ));
    }

    // parse_indexed_signal_annotation_base unit tests

    #[test]
    fn indexed_signal_annotation_parses_bracket_index() {
        let result = super::parse_indexed_signal_annotation_base("SIG[0]");
        assert_eq!(result, Some("SIG".to_string()));
    }

    #[test]
    fn indexed_signal_annotation_rejects_empty_prefix() {
        let result = super::parse_indexed_signal_annotation_base("[0]");
        assert_eq!(result, None);
    }

    #[test]
    fn indexed_signal_annotation_rejects_index_containing_open_delimiter() {
        // Catches ||→&& mutant at line 9568 col 50 — index.contains check must
        // reject nested delimiters via ||, not &&, with index.is_empty()
        let result = super::parse_indexed_signal_annotation_base("SIG[a[b]");
        assert_eq!(result, None);
    }

    // trailing_tokens_form_only_cycle_marker_label unit tests

    #[test]
    fn trailing_cycle_marker_accepts_single_cycle_token() {
        // Catches delete match arm [cycle] at line 9602
        assert!(super::trailing_tokens_form_only_cycle_marker_label(&["T1"]));
    }

    #[test]
    fn trailing_cycle_marker_rejects_prefix_without_valid_cycle() {
        // Catches &&→|| mutant at line 9605 — both prefix and cycle must be valid
        assert!(!super::trailing_tokens_form_only_cycle_marker_label(&[
            "at",
            "invalid_cycle"
        ]));
    }

    // collect_known_actor_names unit tests

    #[test]
    fn collect_known_actor_names_returns_names_from_actors_and_ports() {
        // Catches return BTreeSet::new(), return {""}, return {"xyzzy"}
        let actors = vec![super::ActorRecord {
            actor_id: "actor_1".to_string(),
            actor_name: Some("CPU".to_string()),
            role_summary: "processor".to_string(),
            supporting_statement_ids: vec![],
            supporting_section_ids: vec![],
        }];
        let ports = vec![super::ActorPortRecord {
            actor_id: "actor_port_1".to_string(),
            actor_name: "DMA".to_string(),
            signal_name: "req".to_string(),
            direction: super::ActorRelativeDirection::Output,
            relation_basis: vec![],
            width_hint: None,
            source_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }];
        let names = super::collect_known_actor_names(&actors, &ports);
        assert!(names.contains("CPU"));
        assert!(names.contains("DMA"));
        assert_eq!(names.len(), 2);
    }

    // contains_token_phrase unit tests

    #[test]
    fn contains_token_phrase_rejects_empty_phrase() {
        // Catches ||→&& mutant at line 9256 — empty phrase must return false
        assert!(!super::contains_token_phrase(&["hello"], &[]));
    }

    #[test]
    fn contains_token_phrase_matches_window() {
        assert!(super::contains_token_phrase(&["a", "b", "c"], &["b", "c"]));
    }

    // parse_cardinal_cycle_count_value unit tests

    #[test]
    fn cardinal_cycle_count_parses_word_one() {
        assert_eq!(super::parse_cardinal_cycle_count_value("one"), Some(1));
    }

    #[test]
    fn cardinal_cycle_count_parses_word_two() {
        assert_eq!(super::parse_cardinal_cycle_count_value("two"), Some(2));
    }

    #[test]
    fn cardinal_cycle_count_parses_word_four() {
        assert_eq!(super::parse_cardinal_cycle_count_value("four"), Some(4));
    }

    #[test]
    fn cardinal_cycle_count_parses_word_three() {
        assert_eq!(super::parse_cardinal_cycle_count_value("three"), Some(3));
    }

    #[test]
    fn cardinal_cycle_count_parses_word_five() {
        assert_eq!(super::parse_cardinal_cycle_count_value("five"), Some(5));
    }

    #[test]
    fn cardinal_cycle_count_parses_word_six() {
        assert_eq!(super::parse_cardinal_cycle_count_value("six"), Some(6));
    }

    #[test]
    fn cardinal_cycle_count_parses_word_seven() {
        assert_eq!(super::parse_cardinal_cycle_count_value("seven"), Some(7));
    }

    #[test]
    fn cardinal_cycle_count_parses_word_eight() {
        assert_eq!(super::parse_cardinal_cycle_count_value("eight"), Some(8));
    }

    #[test]
    fn cardinal_cycle_count_parses_word_nine() {
        assert_eq!(super::parse_cardinal_cycle_count_value("nine"), Some(9));
    }

    #[test]
    fn cardinal_cycle_count_parses_word_ten() {
        assert_eq!(super::parse_cardinal_cycle_count_value("ten"), Some(10));
    }

    // parse_ordinal_cycle_count_value unit tests

    #[test]
    fn ordinal_cycle_count_parses_suffixed_number() {
        // Catches delete ! mutant at line 9290 — non-empty prefix must pass guard
        assert_eq!(super::parse_ordinal_cycle_count_value("1st"), Some(1));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_first() {
        assert_eq!(super::parse_ordinal_cycle_count_value("first"), Some(1));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_second() {
        assert_eq!(super::parse_ordinal_cycle_count_value("second"), Some(2));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_third() {
        assert_eq!(super::parse_ordinal_cycle_count_value("third"), Some(3));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_fourth() {
        assert_eq!(super::parse_ordinal_cycle_count_value("fourth"), Some(4));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_fifth() {
        assert_eq!(super::parse_ordinal_cycle_count_value("fifth"), Some(5));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_sixth() {
        assert_eq!(super::parse_ordinal_cycle_count_value("sixth"), Some(6));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_seventh() {
        assert_eq!(super::parse_ordinal_cycle_count_value("seventh"), Some(7));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_eighth() {
        assert_eq!(super::parse_ordinal_cycle_count_value("eighth"), Some(8));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_ninth() {
        assert_eq!(super::parse_ordinal_cycle_count_value("ninth"), Some(9));
    }

    #[test]
    fn ordinal_cycle_count_parses_word_tenth() {
        assert_eq!(super::parse_ordinal_cycle_count_value("tenth"), Some(10));
    }

    // vlm_guard_clause_has_comparison unit tests

    #[test]
    fn vlm_guard_clause_detects_equality_comparison() {
        // Catches return false mutant
        assert!(super::vlm_guard_clause_has_comparison("x == y"));
    }

    #[test]
    fn vlm_guard_clause_detects_inequality_comparison() {
        // Catches ||→&& mutant at line 10226 — != present but == absent
        assert!(super::vlm_guard_clause_has_comparison("x != y"));
    }

    #[test]
    fn vlm_guard_clause_detects_single_equals_comparison() {
        // Catches ||→&& mutants at lines 10227 and 10228 — single = without == or !=
        assert!(super::vlm_guard_clause_has_comparison("x = y"));
    }

    #[test]
    fn vlm_guard_clause_detects_is_comparison() {
        // Catches ||→&& at line 10228 — "is" without any = operator
        assert!(super::vlm_guard_clause_has_comparison("x is y"));
    }

    #[test]
    fn vlm_guard_clause_rejects_plain_text() {
        // Catches return true mutant
        assert!(!super::vlm_guard_clause_has_comparison("hello"));
    }

    // parse_vlm_decision_tree_value unit tests

    #[test]
    fn vlm_decision_tree_value_parses_signal_ref_when_in_known_set() {
        // Catches ||→&& mutant at line 10343 — is_empty false, contains true
        let known: HashSet<String> = ["MY_SIG".to_string()].into();
        let result = super::parse_vlm_decision_tree_value("MY_SIG", &known);
        assert!(matches!(
            result,
            Some(super::DecisionTreeValueRecord::SignalRef { .. })
        ));
    }

    #[test]
    fn vlm_decision_tree_value_falls_back_to_literal_when_unknown() {
        // Ensures fallback path works when signal is not in known set
        let known: HashSet<String> = ["OTHER".to_string()].into();
        let result = super::parse_vlm_decision_tree_value("MY_SIG", &known);
        assert!(matches!(
            result,
            Some(super::DecisionTreeValueRecord::Literal { .. })
        ));
    }

    // is_false unit tests

    #[test]
    fn is_false_detects_false() {
        // Catches replace with false mutant — must return true for false input
        assert!(super::is_false(&false));
    }

    // is_zero unit tests

    #[test]
    fn is_zero_detects_zero() {
        // Catches replace with false mutant
        assert!(super::is_zero(&0));
    }

    #[test]
    fn is_zero_rejects_nonzero() {
        // Catches replace with true and ==→!= mutants
        assert!(!super::is_zero(&1));
    }

    // is_waveform_index_token unit tests

    #[test]
    fn waveform_index_token_accepts_digits() {
        assert!(super::is_waveform_index_token("42"));
    }

    #[test]
    fn waveform_index_token_accepts_hex() {
        // Catches delete ! mutant — non-empty hex digits with all hex chars
        assert!(super::is_waveform_index_token("0x1A"));
    }

    #[test]
    fn waveform_index_token_rejects_invalid_hex() {
        // Catches &&→|| mutant — hex prefix with non-hex chars
        assert!(!super::is_waveform_index_token("0xGG"));
    }

    #[test]
    fn waveform_index_token_rejects_plain_text() {
        // Catches return true mutant
        assert!(!super::is_waveform_index_token("hello"));
    }

    #[test]
    fn waveform_index_token_accepts_colon_separated_indices() {
        assert!(super::is_waveform_index_token("0:7"));
    }

    // is_compact_waveform_index_label unit tests

    #[test]
    fn compact_waveform_index_accepts_addr_with_index() {
        assert!(super::is_compact_waveform_index_label("addr[0]"));
    }

    #[test]
    fn compact_waveform_index_accepts_data_with_hex_index() {
        assert!(super::is_compact_waveform_index_label("data[0x1A]"));
    }

    #[test]
    fn compact_waveform_index_accepts_a_with_index() {
        assert!(super::is_compact_waveform_index_label("a[3]"));
    }

    #[test]
    fn compact_waveform_index_accepts_angle_brackets() {
        assert!(super::is_compact_waveform_index_label("data<7>"));
    }

    #[test]
    fn compact_waveform_index_rejects_empty_prefix() {
        // Catches ||→&& at line 10012: prefix empty + index non-empty
        assert!(!super::is_compact_waveform_index_label("[0]"));
    }

    #[test]
    fn compact_waveform_index_rejects_empty_index() {
        // Catches ||→&& at line 10012 and delete ! at 10015
        // prefix="a" matches allowlist, empty index fails
        assert!(!super::is_compact_waveform_index_label("a[]"));
    }

    #[test]
    fn compact_waveform_index_rejects_nested_brackets() {
        assert!(!super::is_compact_waveform_index_label("a[x[y]]"));
    }

    #[test]
    fn compact_waveform_index_rejects_digit_started_prefix() {
        // Catches delete ! at line 10015 — prefix starts with digit,
        // !matches! is true (not in allowlist), parse_identifier is None (digit start),
        // so original continues (true&&true) but mutant falls through (false)
        assert!(!super::is_compact_waveform_index_label("0foo[0]"));
    }

    #[test]
    fn compact_waveform_index_rejects_plain_text() {
        assert!(!super::is_compact_waveform_index_label("hello"));
    }

    // parse_explicit_infrastructure_source_actor unit tests

    #[test]
    fn infrastructure_source_actor_parses_active_verb() {
        let result = super::parse_explicit_infrastructure_source_actor(
            "the DMA engine drives the CLK",
            "CLK",
        );
        // The subject phrase extraction depends on NLP parsing, but exercising
        // the code path catches comparison/arithmetic mutants.
        assert!(result.is_some());
    }

    #[test]
    fn infrastructure_source_actor_parses_passive_verb() {
        // Catches <=→> mutant at line 3259 — passive verb path
        let result = super::parse_explicit_infrastructure_source_actor(
            "CLK is driven by the DMA engine",
            "CLK",
        );
        assert!(result.is_some());
    }

    #[test]
    fn infrastructure_source_actor_parses_passive_verb_with_leading_text() {
        // Catches +→* mutant at line 3258 — non-zero pattern_pos
        let result = super::parse_explicit_infrastructure_source_actor(
            "The CLK is driven by the DMA engine",
            "CLK",
        );
        assert!(result.is_some());
    }

    #[test]
    fn infrastructure_source_actor_returns_none_for_no_match() {
        // Catches return None mutant
        let result = super::parse_explicit_infrastructure_source_actor("no signal here", "CLK");
        assert!(result.is_none());
    }

    // parse_explicit_infrastructure_distribution_actors unit tests

    #[test]
    fn infrastructure_distribution_actors_parses_signal_subject_verb() {
        // Catches <=→> mutant at line 3420 — signal subject verb path
        let result = super::parse_explicit_infrastructure_distribution_actors(
            "CLK feeds the CPU and DMA",
            "CLK",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn infrastructure_distribution_actors_parses_signal_subject_verb_with_leading_text() {
        // Catches +→* mutant at line 3419 — non-zero pattern_pos
        let result = super::parse_explicit_infrastructure_distribution_actors(
            "The CLK feeds the CPU and DMA",
            "CLK",
        );
        assert!(!result.is_empty());
    }

    #[test]
    fn infrastructure_distribution_actors_returns_empty_for_no_match() {
        let result =
            super::parse_explicit_infrastructure_distribution_actors("no signal here", "CLK");
        assert!(result.is_empty());
    }

    // extract_infrastructure_component_phrase unit tests

    #[test]
    fn infrastructure_component_phrase_preserves_leading_underscore() {
        // Catches !=→== (underscore) at line 3754:61 — trim_matches must not trim _
        let result = super::extract_infrastructure_component_phrase("_DMA");
        assert_eq!(result.as_deref(), Some("_DMA"));
    }

    #[test]
    fn infrastructure_component_phrase_preserves_leading_hyphen() {
        // Catches !=→== (hyphen) at line 3754:81 — trim_matches must not trim -
        let result = super::extract_infrastructure_component_phrase("-DMA");
        assert_eq!(result.as_deref(), Some("-DMA"));
    }

    #[test]
    fn infrastructure_component_phrase_keeps_two_non_stop_words() {
        // Catches &&→|| at line 3760 — second non-stop-word must not break
        let result = super::extract_infrastructure_component_phrase("one two");
        assert_eq!(result.as_deref(), Some("one two"));
    }

    #[test]
    fn infrastructure_component_phrase_caps_at_four_words() {
        // Catches >=→< at line 3774 — exactly 4 words must be accepted
        let result = super::extract_infrastructure_component_phrase("one two three four");
        assert!(result.is_some());
    }

    #[test]
    fn infrastructure_component_phrase_breaks_before_fifth_word() {
        // Catches >=→< at line 3774 — >= 4 triggers break on 4th word
        let result = super::extract_infrastructure_component_phrase("one two three four five");
        // Should cap at 4 words
        let extracted = result.unwrap();
        let word_count = extracted.split_whitespace().count();
        assert_eq!(word_count, 4);
    }

    #[test]
    fn infrastructure_component_phrase_stops_at_stop_word() {
        let result = super::extract_infrastructure_component_phrase("DMA to RAM");
        assert_eq!(result.as_deref(), Some("DMA"));
    }

    // extract_infrastructure_subject_phrase unit tests

    #[test]
    fn infrastructure_subject_phrase_extracts_after_clean_determiner() {
        let result = super::extract_infrastructure_subject_phrase("the DMA engine");
        assert_eq!(result.as_deref(), Some("DMA engine"));
    }

    #[test]
    fn infrastructure_subject_phrase_extracts_after_punctuated_determiner() {
        // Catches delete ! at line 3794 — non-alpha chars must be stripped from determiner
        let result = super::extract_infrastructure_subject_phrase("the, DMA engine");
        assert_eq!(result.as_deref(), Some("DMA engine"));
    }

    #[test]
    fn infrastructure_subject_phrase_extracts_with_leading_words_before_determiner() {
        // Catches +→* at line 3800 — non-zero index path
        let result = super::extract_infrastructure_subject_phrase("drives the DMA engine");
        assert_eq!(result.as_deref(), Some("DMA engine"));
    }

    #[test]
    fn infrastructure_subject_phrase_preserves_underscore_after_determiner() {
        // Catches !=→== (underscore) at line 3802:65
        let result = super::extract_infrastructure_subject_phrase("the _DMA");
        assert_eq!(result.as_deref(), Some("_DMA"));
    }

    #[test]
    fn infrastructure_subject_phrase_preserves_hyphen_after_determiner() {
        // Catches !=→== (hyphen) at line 3802:85
        let result = super::extract_infrastructure_subject_phrase("the -DMA");
        assert_eq!(result.as_deref(), Some("-DMA"));
    }

    #[test]
    fn infrastructure_subject_phrase_keeps_two_words_after_determiner() {
        // Catches &&→|| at line 3802:52,72 — second non-stop word
        let result = super::extract_infrastructure_subject_phrase("the one two");
        assert_eq!(result.as_deref(), Some("one two"));
    }

    #[test]
    fn infrastructure_subject_phrase_reverse_path_extracts_actor() {
        // Exercises reverse path (no determiner found) — reverse order
        let result = super::extract_infrastructure_subject_phrase("DMA engine drives");
        assert!(result.is_some());
    }

    #[test]
    fn infrastructure_subject_phrase_reverse_preserves_underscore() {
        // Catches !=→== (underscore) at line 3824:61
        let result = super::extract_infrastructure_subject_phrase("_DMA engine");
        assert_eq!(result.as_deref(), Some("_DMA engine"));
    }

    #[test]
    fn infrastructure_subject_phrase_reverse_preserves_hyphen() {
        // Catches !=→== (hyphen) at line 3824:81
        let result = super::extract_infrastructure_subject_phrase("-DMA engine");
        assert_eq!(result.as_deref(), Some("-DMA engine"));
    }

    #[test]
    fn infrastructure_subject_phrase_skips_leading_skip_word_in_reverse() {
        // Catches delete ! at line 3831 — skip word with empty actor_words must continue
        let result = super::extract_infrastructure_subject_phrase("and DMA engine");
        assert_eq!(result.as_deref(), Some("DMA engine"));
    }

    #[test]
    fn infrastructure_subject_phrase_reverse_caps_at_four_words() {
        // Catches >=→< at line 3837
        let result = super::extract_infrastructure_subject_phrase("one two three four five");
        let extracted = result.unwrap();
        let word_count = extracted.split_whitespace().count();
        assert_eq!(word_count, 4);
    }

    // parse_width_token unit tests

    #[test]
    fn width_token_parses_numeric() {
        assert_eq!(
            super::parse_width_token("8"),
            Some(super::WidthHint::Numeric(8))
        );
    }

    #[test]
    fn width_token_rejects_zero() {
        // Catches >→>= at line 5734
        assert_eq!(super::parse_width_token("0"), None);
    }

    #[test]
    fn width_token_rejects_non_alphanumeric() {
        // Catches &&→|| at line 5738 — non-empty, no alpha chars
        assert_eq!(super::parse_width_token("#"), None);
    }

    #[test]
    fn width_token_parses_parametric() {
        assert_eq!(
            super::parse_width_token("ADDR_WIDTH"),
            Some(super::WidthHint::Parametric("ADDR_WIDTH".to_string()))
        );
    }

    #[test]
    fn width_token_parses_bit_suffix() {
        assert_eq!(
            super::parse_width_token("8-bit"),
            Some(super::WidthHint::Numeric(8))
        );
    }

    // parse_primary unit tests (ControlExpressionParser)

    #[test]
    fn control_expr_parse_primary_literal_true() {
        let tokens = vec!["true".to_string()];
        let signals = BTreeSet::new();
        let symbols = BTreeSet::new();
        let mut parser = super::ControlExpressionParser::new(tokens, &signals, &symbols);
        let result = parser.parse_primary();
        assert!(matches!(
            result,
            Some(super::ControlExpressionRecord::Literal { .. })
        ));
    }

    #[test]
    fn control_expr_parse_primary_literal_false() {
        // Catches ||→&& at line 5514 — "false" alone must still be a literal
        let tokens = vec!["false".to_string()];
        let signals = BTreeSet::new();
        let symbols = BTreeSet::new();
        let mut parser = super::ControlExpressionParser::new(tokens, &signals, &symbols);
        let result = parser.parse_primary();
        assert!(matches!(
            result,
            Some(super::ControlExpressionRecord::Literal { .. })
        ));
    }

    #[test]
    fn control_expr_parse_primary_numeric_literal() {
        let tokens = vec!["42".to_string()];
        let signals = BTreeSet::new();
        let symbols = BTreeSet::new();
        let mut parser = super::ControlExpressionParser::new(tokens, &signals, &symbols);
        let result = parser.parse_primary();
        assert!(matches!(
            result,
            Some(super::ControlExpressionRecord::Literal { .. })
        ));
    }

    // parse_comparison unit tests — exercises all comparison operators

    fn parse_comparison_tokens(tokens: Vec<&str>) -> Option<super::ControlExpressionRecord> {
        let signals = BTreeSet::new();
        let symbols = BTreeSet::new();
        let tokens: Vec<String> = tokens.into_iter().map(|s| s.to_string()).collect();
        let mut parser = super::ControlExpressionParser::new(tokens, &signals, &symbols);
        parser.parse_comparison()
    }

    #[test]
    fn control_expr_comparison_eq() {
        // Catches delete match arm Some("==")
        let result = parse_comparison_tokens(vec!["x", "==", "5"]);
        assert!(matches!(
            result,
            Some(super::ControlExpressionRecord::Binary {
                operator: super::ControlBinaryOperator::Eq,
                ..
            })
        ));
    }

    #[test]
    fn control_expr_comparison_not_eq() {
        // Catches delete match arm Some("!=") at line 5422
        let result = parse_comparison_tokens(vec!["x", "!=", "0"]);
        assert!(matches!(
            result,
            Some(super::ControlExpressionRecord::Binary {
                operator: super::ControlBinaryOperator::NotEq,
                ..
            })
        ));
    }

    #[test]
    fn control_expr_comparison_lt() {
        // Catches delete match arm Some("<") at line 5423
        let result = parse_comparison_tokens(vec!["x", "<", "10"]);
        assert!(matches!(
            result,
            Some(super::ControlExpressionRecord::Binary {
                operator: super::ControlBinaryOperator::Lt,
                ..
            })
        ));
    }

    #[test]
    fn control_expr_comparison_le() {
        // Catches delete match arm Some("<=") at line 5424
        let result = parse_comparison_tokens(vec!["x", "<=", "10"]);
        assert!(matches!(
            result,
            Some(super::ControlExpressionRecord::Binary {
                operator: super::ControlBinaryOperator::Le,
                ..
            })
        ));
    }

    #[test]
    fn control_expr_comparison_gt() {
        // Catches delete match arm Some(">") at line 5425
        let result = parse_comparison_tokens(vec!["x", ">", "0"]);
        assert!(matches!(
            result,
            Some(super::ControlExpressionRecord::Binary {
                operator: super::ControlBinaryOperator::Gt,
                ..
            })
        ));
    }

    #[test]
    fn control_expr_comparison_ge() {
        // Catches delete match arm Some(">=") at line 5426
        let result = parse_comparison_tokens(vec!["x", ">=", "0"]);
        assert!(matches!(
            result,
            Some(super::ControlExpressionRecord::Binary {
                operator: super::ControlBinaryOperator::Ge,
                ..
            })
        ));
    }

    // dedup_actor_names unit tests

    #[test]
    fn dedup_actor_names_removes_duplicates() {
        // Catches replace with () mutant — must actually deduplicate
        let mut names = vec!["CPU".to_string(), "DMA".to_string(), "CPU".to_string()];
        super::dedup_actor_names(&mut names);
        assert_eq!(names, vec!["CPU", "DMA"]);
    }

    #[test]
    fn dedup_actor_names_preserves_unique_list() {
        let mut names = vec!["CPU".to_string(), "DMA".to_string()];
        super::dedup_actor_names(&mut names);
        assert_eq!(names, vec!["CPU", "DMA"]);
    }

    // parse_explicit_clock_gated_branch unit tests

    #[test]
    fn clock_gated_branch_detects_gated_branch() {
        // Catches ||→&& at line 3442 — "gated branch" without clock prefix
        let result =
            super::parse_explicit_clock_gated_branch("CLK gated branch, feeds the CPU", "CLK");
        assert!(result.is_some());
    }

    #[test]
    fn clock_gated_branch_detects_clock_gate() {
        // Catches ||→&& at line 3443 — "clock gate" without "gated by"
        let result =
            super::parse_explicit_clock_gated_branch("CLK clock gate, feeds the CPU", "CLK");
        assert!(result.is_some());
    }

    #[test]
    fn clock_gated_branch_rejects_unrelated_text() {
        let result = super::parse_explicit_clock_gated_branch("CLK drives the CPU", "CLK");
        assert!(result.is_none());
    }

    // parse_explicit_reset_synchronizer_stages unit tests

    #[test]
    fn reset_synchronizer_detects_synchronizer() {
        let result = super::parse_explicit_reset_synchronizer_stages(
            "CLK feeds the CPU, reset synchronizer 2 stages",
            "CLK",
        );
        assert!(result.is_some());
    }

    #[test]
    fn reset_synchronizer_rejects_text_without_synchronizer_term() {
        // Catches ||→&& at line 3491 — signal found but neither synchronizer term present.
        // Mutant changes guard to F && Inner, allowing fall-through to stage/target parsing.
        let result =
            super::parse_explicit_reset_synchronizer_stages("CLK feeds the CPU, 2 stages", "CLK");
        assert!(result.is_none());
    }

    #[test]
    fn reset_synchronizer_rejects_unrelated_text() {
        let result = super::parse_explicit_reset_synchronizer_stages("CLK drives the CPU", "CLK");
        assert!(result.is_none());
    }

    // InfrastructureTopologyKind::as_str unit tests

    #[test]
    fn topology_kind_as_str_clock_gated_branch() {
        assert_eq!(
            super::InfrastructureTopologyKind::ClockGatedBranch.as_str(),
            "clock_gated_branch"
        );
    }

    #[test]
    fn topology_kind_as_str_reset_synchronizer_stages() {
        assert_eq!(
            super::InfrastructureTopologyKind::ResetSynchronizerStages.as_str(),
            "reset_synchronizer_stages"
        );
    }

    #[test]
    fn topology_kind_as_str_reset_tree_targets() {
        assert_eq!(
            super::InfrastructureTopologyKind::ResetTreeTargets.as_str(),
            "reset_tree_targets"
        );
    }

    // SemanticGroundingStrength::as_str unit tests

    #[test]
    fn grounding_strength_as_str_single_source() {
        assert_eq!(
            super::SemanticGroundingStrength::SingleSource.as_str(),
            "single_source"
        );
    }

    #[test]
    fn grounding_strength_as_str_multi_source() {
        assert_eq!(
            super::SemanticGroundingStrength::MultiSource.as_str(),
            "multi_source"
        );
    }

    #[test]
    fn grounding_strength_as_str_cross_modality() {
        assert_eq!(
            super::SemanticGroundingStrength::CrossModality.as_str(),
            "cross_modality"
        );
    }

    // SemanticIr::build transition dedup — VLM duplicates are not appended

    #[test]
    fn build_dedups_vlm_transition_against_formal_transition() -> Result<()> {
        // Catches ==→!= mutants at lines 277, 278 and &&→|| at line 278.
        // Formal transition run→done. VLM observation has two transitions:
        //   run→done (duplicate of formal — must be deduped)
        //   run→waiting (same source, different target — must NOT be deduped)
        // The &&→|| mutant would incorrectly dedup run→waiting (source matches).
        use crate::ir::source::{DiagramKind, VisualAsset, VisualAssetKind};
        let tempdir = tempdir()?;
        let source = tempdir.path().join("dedup_spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            "# Dedup Test\nSignal clk is input width 1.\n\nState run is initial.\n\nState done.\n\nState waiting.\n\nTransition run -> done when clk.\n",
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "picture_dedup".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("Figure D-1 State machine".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_state_machine_extraction: {\"states\":[{\"name\":\"run\",\"is_initial\":true},{\"name\":\"done\"},{\"name\":\"waiting\"}],\"transitions\":[{\"from\":\"run\",\"to\":\"done\",\"guard\":\"clk\"},{\"from\":\"run\",\"to\":\"waiting\",\"guard\":\"\"}]}"
                    .to_string(),
            ),
            diagram_kind: DiagramKind::StateMachineDiagram,
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

        // Formal run→done + VLM run→waiting = 2 distinct transitions.
        // VLM run→done is a duplicate (same source AND target) → deduped.
        // With the &&→|| mutant, run→waiting is also deduped (source matches).
        // With ==→!= on target_state, VLM run→waiting is deduped (target != done).
        // With ==→!= on source_state, VLM run→done is NOT deduped (source != run).
        let run_transitions: Vec<_> = semantic_ir
            .state_transitions
            .iter()
            .filter(|t| t.source_state == "run")
            .collect();
        assert_eq!(
            run_transitions.len(),
            2,
            "expected 2 run→* transitions (done + waiting), got {}",
            run_transitions.len()
        );
        let run_to_done: Vec<_> = semantic_ir
            .state_transitions
            .iter()
            .filter(|t| t.source_state == "run" && t.target_state == "done")
            .collect();
        assert_eq!(
            run_to_done.len(),
            1,
            "expected 1 run→done (formal, VLM duplicate deduped), got {}",
            run_to_done.len()
        );
        let run_to_waiting: Vec<_> = semantic_ir
            .state_transitions
            .iter()
            .filter(|t| t.source_state == "run" && t.target_state == "waiting")
            .collect();
        assert_eq!(
            run_to_waiting.len(),
            1,
            "expected 1 run→waiting (VLM, distinct target), got {}",
            run_to_waiting.len()
        );

        Ok(())
    }

    // -- extract_cycle_window_from_text high-value mutant tests --

    #[test]
    fn extract_cycle_window_most_without_at_returns_none() {
        // Line 8714: &&→|| — "most" without "at" should not trigger "at most" path.
        let result = super::extract_cycle_window_from_text("most 5 cycles");
        assert!(
            result.is_none(),
            "most 5 cycles: expected None, got {result:?}"
        );
    }

    #[test]
    fn extract_cycle_window_more_without_no_returns_none() {
        // Lines 8726-8727: &&→|| — "more" without "no" should not trigger "no more than" path.
        let result = super::extract_cycle_window_from_text("more than 5 cycles");
        assert!(
            result.is_none(),
            "more than 5 cycles: expected None, got {result:?}"
        );
    }

    #[test]
    fn extract_cycle_window_no_more_without_than_returns_none() {
        // Line 8727: &&→|| — "no more" without "than" should not match.
        let result = super::extract_cycle_window_from_text("no more 5 cycles");
        assert!(
            result.is_none(),
            "no more 5 cycles: expected None, got {result:?}"
        );
    }

    // -- extract_cycle_window_from_text_with_known_signals high-value mutants --

    fn known_signal_set(signals: &[&str]) -> BTreeSet<String> {
        signals.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn cycle_window_known_signals_between_non_between_token_skipped() {
        // Line 8865: !=→== — "bztween" (typo, not "between") should be skipped.
        let signals = known_signal_set(&["clk"]);
        let result = super::extract_cycle_window_from_text_with_known_signals(
            "bztween 3 and 5 clk edges",
            &signals,
        );
        assert!(
            result.is_none(),
            "bztween (non-between): expected None, got {result:?}"
        );
    }

    #[test]
    fn cycle_window_known_signals_between_and_mismatch() {
        // Line 8875: ==→!= — "between X and Y" requires "and" at position +2.
        let signals = known_signal_set(&["clk"]);
        let result = super::extract_cycle_window_from_text_with_known_signals(
            "between 3 then 5 clk edges",
            &signals,
        );
        assert!(
            result.is_none(),
            "between 3 then 5: expected None, got {result:?}"
        );
    }

    #[test]
    fn cycle_window_known_signals_within_returns_min_none() {
        // Line 8920: delete match arm "after" | "for" — "within" path still works.
        // Also covers "after" and "for" paths.
        let signals = known_signal_set(&["clk"]);
        let result =
            super::extract_cycle_window_from_text_with_known_signals("after 3 clk edges", &signals);
        assert_eq!(
            result,
            Some(CycleWindowRecord {
                min_cycles: Some(3),
                max_cycles: Some(3),
            }),
            "after 3 clk edges: expected Some(min=3,max=3), got {result:?}"
        );
    }

    #[test]
    fn cycle_window_known_signals_for_returns_count() {
        // Line 8920: delete match arm "after" | "for" — "for" path coverage.
        let signals = known_signal_set(&["clk"]);
        let result =
            super::extract_cycle_window_from_text_with_known_signals("for 4 clk edges", &signals);
        assert_eq!(
            result,
            Some(CycleWindowRecord {
                min_cycles: Some(4),
                max_cycles: Some(4),
            }),
            "for 4 clk edges: expected Some(min=4,max=4), got {result:?}"
        );
    }

    #[test]
    fn cycle_window_known_signals_at_least_without_at() {
        // Line 8931: &&→|| — "least" without "at" should not trigger "at least" path.
        let signals = known_signal_set(&["clk"]);
        let result =
            super::extract_cycle_window_from_text_with_known_signals("least 5 clk edges", &signals);
        assert!(
            result.is_none(),
            "least 5 clk edges: expected None, got {result:?}"
        );
    }

    #[test]
    fn cycle_window_known_signals_at_most_without_at() {
        // Line 8945: &&→|| — "most" without "at" should not trigger "at most" path.
        let signals = known_signal_set(&["clk"]);
        let result =
            super::extract_cycle_window_from_text_with_known_signals("most 6 clk edges", &signals);
        assert!(
            result.is_none(),
            "most 6 clk edges: expected None, got {result:?}"
        );
    }

    #[test]
    fn cycle_window_known_signals_no_more_than_requires_no() {
        // Lines 8959-8961: ==→!= and &&→|| — "more than" without "no" should not match.
        let signals = known_signal_set(&["clk"]);
        let result = super::extract_cycle_window_from_text_with_known_signals(
            "more than 7 clk edges",
            &signals,
        );
        assert!(
            result.is_none(),
            "more than 7 clk edges: expected None, got {result:?}"
        );
    }

    #[test]
    fn cycle_window_known_signals_at_during_on_trigger_parsing() {
        // Line 8983: delete ! — "during" should trigger parsing, not be skipped.
        let signals = known_signal_set(&["clk"]);
        let result = super::extract_cycle_window_from_text_with_known_signals(
            "during 3 clk edges",
            &signals,
        );
        assert_eq!(
            result,
            Some(CycleWindowRecord {
                min_cycles: Some(3),
                max_cycles: Some(3),
            }),
            "during 3 clk edges: expected Some, got {result:?}"
        );
    }

    // -- edge_of_known_signal_unit_len high-value mutants --

    #[test]
    fn edge_of_known_signal_edge_singular_token() {
        // Line 9029: ||→&& — "edge" (singular) should be recognized.
        let signals = known_signal_set(&["clk"]);
        let tokens = ["edge", "of", "the", "clk"];
        let result = super::edge_of_known_signal_unit_len(&tokens, 0, &signals);
        assert_eq!(
            result,
            Some(4),
            "edge of the clk: expected Some(4), got {result:?}"
        );
    }

    #[test]
    fn edge_of_known_signal_edges_plural_token() {
        // Line 9029: ||→&& — "edges" (plural) should also be recognized.
        let signals = known_signal_set(&["clk"]);
        let tokens = ["edges", "of", "clk"];
        let result = super::edge_of_known_signal_unit_len(&tokens, 0, &signals);
        assert_eq!(
            result,
            Some(3),
            "edges of clk: expected Some(3), got {result:?}"
        );
    }

    // -- merge_copy_hint high-value mutants --

    #[test]
    fn merge_copy_hint_none_sets_target() {
        let mut target: Option<u32> = None;
        let changed = super::merge_copy_hint(&mut target, 42);
        assert!(changed, "None→Some should return true");
        assert_eq!(target, Some(42));
    }

    #[test]
    fn merge_copy_hint_same_value_returns_true() {
        // Line 6001: ==→!= — same values should return true.
        let mut target = Some(42u32);
        let changed = super::merge_copy_hint(&mut target, 42);
        assert!(changed, "same value should return true, got {changed}");
    }

    #[test]
    fn merge_copy_hint_different_value_returns_false() {
        let mut target = Some(42u32);
        let changed = super::merge_copy_hint(&mut target, 99);
        assert!(
            !changed,
            "different value should return false, got {changed}"
        );
    }

    // -- merge_named_hint high-value mutants --

    #[test]
    fn merge_named_hint_none_sets_target() {
        let mut target: Option<String> = None;
        let changed = super::merge_named_hint(&mut target, "hello");
        assert!(changed, "None→Some should return true");
        assert_eq!(target, Some("hello".to_string()));
    }

    #[test]
    fn merge_named_hint_same_name_returns_true() {
        // Line 5991: ==→!= — same name should return true.
        let mut target = Some("clk".to_string());
        let changed = super::merge_named_hint(&mut target, "clk");
        assert!(changed, "same name should return true, got {changed}");
    }

    #[test]
    fn merge_named_hint_different_name_returns_false() {
        let mut target = Some("clk".to_string());
        let changed = super::merge_named_hint(&mut target, "rst");
        assert!(
            !changed,
            "different name should return false, got {changed}"
        );
    }

    // -- merge_signal_hint high-value mutants --

    #[test]
    fn merge_signal_hint_none_target_set_to_some() {
        let mut target: Option<String> = None;
        super::merge_signal_hint(&mut target, Some("a".to_string()));
        assert_eq!(target, Some("a".to_string()));
    }

    #[test]
    fn merge_signal_hint_same_value_preserves_target() {
        // Line 5964: !=→== — same value should NOT clear target.
        let mut target = Some("a".to_string());
        super::merge_signal_hint(&mut target, Some("a".to_string()));
        assert_eq!(
            target,
            Some("a".to_string()),
            "same value should be preserved"
        );
    }

    #[test]
    fn merge_signal_hint_different_value_clears_target() {
        // Line 5964: match guard replaced with false would mean different values never clear.
        let mut target = Some("a".to_string());
        super::merge_signal_hint(&mut target, Some("b".to_string()));
        assert_eq!(
            target, None,
            "different value should clear target (conflict)"
        );
    }

    #[test]
    fn merge_signal_hint_none_incoming_does_nothing() {
        // Line 5964: match guard replaced with true would mean None incoming also triggers.
        let mut target = Some("a".to_string());
        super::merge_signal_hint(&mut target, None::<String>);
        assert_eq!(
            target,
            Some("a".to_string()),
            "None incoming should be no-op"
        );
    }

    // -- merge_sticky_signal_hint high-value mutants --

    #[test]
    fn merge_sticky_signal_hint_conflicted_ignores_incoming() {
        // Line 5975: delete match arm (_, true, Some(_)) — conflicted targets should ignore.
        let mut target = Some("a".to_string());
        let mut conflicted = true;
        super::merge_sticky_signal_hint(&mut target, &mut conflicted, Some("b".to_string()));
        assert_eq!(
            target,
            Some("a".to_string()),
            "conflicted: target should be unchanged"
        );
        assert!(conflicted, "conflicted flag should stay true");
    }

    #[test]
    fn merge_sticky_signal_hint_same_value_preserves() {
        let mut target = Some("a".to_string());
        let mut conflicted = false;
        super::merge_sticky_signal_hint(&mut target, &mut conflicted, Some("a".to_string()));
        assert_eq!(
            target,
            Some("a".to_string()),
            "same value should be preserved"
        );
        assert!(!conflicted);
    }

    #[test]
    fn merge_sticky_signal_hint_different_value_sets_conflict() {
        let mut target = Some("a".to_string());
        let mut conflicted = false;
        super::merge_sticky_signal_hint(&mut target, &mut conflicted, Some("b".to_string()));
        assert_eq!(target, None, "different value should clear target");
        assert!(conflicted, "should set conflicted flag");
    }

    // -- register_interface_signal_semantic_hint high-value mutants --

    fn make_test_hint(
        signal_name: &str,
        tags: &[super::SignalSemanticTag],
    ) -> super::SignalSemanticHintRecord {
        super::SignalSemanticHintRecord {
            signal_name: signal_name.to_string(),
            semantic_tags: tags.to_vec(),
            source_kind: super::SignalSemanticHintSourceKind::ProseStatement,
            source_text: "test source".to_string(),
            supporting_statement_ids: vec!["stmt1".to_string()],
            supporting_table_ids: vec!["tbl1".to_string()],
            supporting_visual_evidence_ids: vec!["vis1".to_string()],
            automation_confidence: super::AutomationConfidence::High,
        }
    }

    #[test]
    fn register_signal_semantic_hint_does_not_duplicate_identical_observation() {
        // Lines 5943-5948: ==→!= x6 — any comparison inversion would miss the duplicate.
        let mut acc = super::InterfaceAccumulator {
            signals: ["clk".to_string()].into(),
            signal_records: {
                let mut m = BTreeMap::new();
                m.insert(
                    "clk".to_string(),
                    super::InterfaceSignalAccumulator {
                        direction_hint: None,
                        direction_hint_conflicted: false,
                        width_hint: None,
                        width_hint_conflicted: false,
                        semantic_tags: BTreeSet::new(),
                        semantic_observations: vec![],
                        direction_observations: BTreeMap::new(),
                        width_observations: BTreeMap::new(),
                        supporting_statement_ids: BTreeSet::new(),
                        supporting_table_ids: BTreeSet::new(),
                        automation_confidence: super::AutomationConfidence::High,
                    },
                );
                m
            },
            supporting_statement_ids: BTreeSet::new(),
        };

        let hint = make_test_hint("clk", &[super::SignalSemanticTag::HandshakeValidLike]);
        super::register_interface_signal_semantic_hint(&mut acc, "clk", &hint);

        // Register same hint again — should be deduplicated
        super::register_interface_signal_semantic_hint(&mut acc, "clk", &hint);

        let record = acc.signal_records.get("clk").unwrap();
        assert_eq!(
            record.semantic_observations.len(),
            1,
            "identical hint should be deduplicated, got {} observations",
            record.semantic_observations.len()
        );
    }

    #[test]
    fn register_signal_semantic_hint_with_different_tags_adds_observation() {
        let mut acc = super::InterfaceAccumulator {
            signals: ["clk".to_string()].into(),
            signal_records: {
                let mut m = BTreeMap::new();
                m.insert(
                    "clk".to_string(),
                    super::InterfaceSignalAccumulator {
                        direction_hint: None,
                        direction_hint_conflicted: false,
                        width_hint: None,
                        width_hint_conflicted: false,
                        semantic_tags: BTreeSet::new(),
                        semantic_observations: vec![],
                        direction_observations: BTreeMap::new(),
                        width_observations: BTreeMap::new(),
                        supporting_statement_ids: BTreeSet::new(),
                        supporting_table_ids: BTreeSet::new(),
                        automation_confidence: super::AutomationConfidence::High,
                    },
                );
                m
            },
            supporting_statement_ids: BTreeSet::new(),
        };

        let hint1 = make_test_hint("clk", &[super::SignalSemanticTag::HandshakeValidLike]);
        super::register_interface_signal_semantic_hint(&mut acc, "clk", &hint1);

        let hint2 = make_test_hint("clk", &[super::SignalSemanticTag::HandshakeReadyLike]);
        super::register_interface_signal_semantic_hint(&mut acc, "clk", &hint2);

        let record = acc.signal_records.get("clk").unwrap();
        assert_eq!(
            record.semantic_observations.len(),
            2,
            "different tags should produce separate observations, got {}",
            record.semantic_observations.len()
        );
    }

    #[test]
    fn register_signal_semantic_hint_empty_tags_skips() {
        let mut acc = super::InterfaceAccumulator {
            signals: ["clk".to_string()].into(),
            signal_records: {
                let mut m = BTreeMap::new();
                m.insert(
                    "clk".to_string(),
                    super::InterfaceSignalAccumulator {
                        direction_hint: None,
                        direction_hint_conflicted: false,
                        width_hint: None,
                        width_hint_conflicted: false,
                        semantic_tags: BTreeSet::new(),
                        semantic_observations: vec![],
                        direction_observations: BTreeMap::new(),
                        width_observations: BTreeMap::new(),
                        supporting_statement_ids: BTreeSet::new(),
                        supporting_table_ids: BTreeSet::new(),
                        automation_confidence: super::AutomationConfidence::High,
                    },
                );
                m
            },
            supporting_statement_ids: BTreeSet::new(),
        };

        let hint = super::SignalSemanticHintRecord {
            signal_name: "clk".to_string(),
            semantic_tags: vec![],
            source_kind: super::SignalSemanticHintSourceKind::ProseStatement,
            source_text: "test".to_string(),
            supporting_statement_ids: vec![],
            supporting_table_ids: vec![],
            supporting_visual_evidence_ids: vec![],
            automation_confidence: super::AutomationConfidence::High,
        };
        super::register_interface_signal_semantic_hint(&mut acc, "clk", &hint);

        let record = acc.signal_records.get("clk").unwrap();
        assert_eq!(
            record.semantic_observations.len(),
            0,
            "empty tags should be skipped"
        );
    }

    // -- is_explicit_infrastructure_component_term high-value mutants --

    #[test]
    fn infrastructure_component_term_matches_exact_pll() {
        assert!(
            super::is_explicit_infrastructure_component_term("pll"),
            "pll should match exact"
        );
    }

    #[test]
    fn infrastructure_component_term_matches_contains_generator() {
        // Lines 3882-3891: ||→&& — "clock_generator" contains only "generator", not "controller" etc.
        assert!(
            super::is_explicit_infrastructure_component_term("clock_generator"),
            "clock_generator should match via contains(\"generator\")"
        );
    }

    #[test]
    fn infrastructure_component_term_matches_contains_synchronizer() {
        // Lines 3882-3891: ||→&& — each contains() is independent.
        assert!(
            super::is_explicit_infrastructure_component_term("async_synchronizer"),
            "async_synchronizer should match via contains(\"synchronizer\")"
        );
    }

    #[test]
    fn infrastructure_component_term_rejects_random_word() {
        // Line 3879: return true — a non-infrastructure term should return false.
        assert!(
            !super::is_explicit_infrastructure_component_term("banana"),
            "banana should not be infrastructure component"
        );
    }

    // -- looks_like_signal_token high-value mutants --

    #[test]
    fn looks_like_signal_token_short_token_returns_false() {
        // Line 6281: <→== / <→<= — single-char token should be rejected.
        assert!(
            !super::looks_like_signal_token("A"),
            "single char should return false"
        );
    }

    #[test]
    fn looks_like_signal_token_two_char_upper_returns_true() {
        // Line 6281: <→== — len=2 all-uppercase should return true.
        assert!(
            super::looks_like_signal_token("AB"),
            "two-char upper should return true"
        );
    }

    #[test]
    fn looks_like_signal_token_uppercase_with_digits_returns_true() {
        // Line 6301: ||→&& — characters can be uppercase OR digit OR underscore.
        assert!(
            super::looks_like_signal_token("CLK2"),
            "upper+digit should return true"
        );
    }

    #[test]
    fn looks_like_signal_token_with_underscore_returns_true() {
        // Line 6301: ==→!= — underscore should be a valid character.
        assert!(
            super::looks_like_signal_token("CLK_OUT"),
            "underscore should be valid"
        );
    }

    #[test]
    fn looks_like_signal_token_mixed_case_returns_false() {
        // Line 6304: &&→|| — mixed case like "Clk" should NOT pass (needs all upper OR _n/_b suffix).
        assert!(
            !super::looks_like_signal_token("Clk"),
            "mixed case should return false"
        );
    }

    #[test]
    fn signal_stop_words_holds_no_chip_spec_vocabulary() {
        // ADR 0006 guard (PDF-AGNOSTIC-EXTRACTION): the signal stop-word list must
        // contain ZERO chip-spec-specific vocabulary — protocol family names, vendor
        // names, or protocol encoding values. A token mis-discovered as a signal is
        // filtered downstream against the document's own declared signals; the list
        // must never hardcode one spec's words. (Forbidden tokens are concatenated so
        // this guard can never trip on itself.)
        let sw = super::signal_stop_words();
        let forbidden = [
            format!("AM{}", "BA"),
            format!("AH{}", "B"),
            format!("AP{}", "B"),
            format!("AX{}", "I"),
            format!("AC{}", "E"),
            format!("CH{}", "I"),
            "AR".to_string() + "M",
            "AM".to_string() + "D",
            "INT".to_string() + "EL",
            "NON".to_string() + "SEQ",
            "IN".to_string() + "CR4",
            "WR".to_string() + "AP4",
            "OK".to_string() + "AY",
            "IH".to_string() + "I",
        ];
        for tok in &forbidden {
            assert!(
                !sw.iter().any(|w| *w == tok.as_str()),
                "ADR 0006: '{tok}' is chip-spec-specific and must not be a hardcoded stop-word"
            );
        }
    }

    #[test]
    fn looks_like_signal_token_lowered_suffix_n_returns_true() {
        // Line 6312: ends_with("_n") — lowercased signal names with _n suffix.
        assert!(
            super::looks_like_signal_token("rst_n"),
            "rst_n should look like signal"
        );
    }

    #[test]
    fn looks_like_signal_token_lowered_suffix_b_returns_true() {
        // Line 6312: ends_with("_b") — lowercased signal names with _b suffix.
        assert!(
            super::looks_like_signal_token("enable_b"),
            "enable_b should look like signal"
        );
    }

    // -- parse_explicit_signal_declaration high-value mutants --

    #[test]
    fn parse_signal_declaration_three_tokens() {
        // Line 4409: <→== / <→<= — 3-token declaration should be accepted.
        let result = super::parse_explicit_signal_declaration("signal clk is input");
        assert!(
            result.is_some(),
            "signal clk is input: expected Some, got {result:?}"
        );
        let decl = result.unwrap();
        assert_eq!(decl.signal_name, "clk");
    }

    #[test]
    fn parse_signal_declaration_first_word_not_signal_returns_none() {
        // Line 4409: ||→&& — first word not "signal" should return None regardless of length.
        let result = super::parse_explicit_signal_declaration("clk is input");
        assert!(
            result.is_none(),
            "clk is input (no 'signal' prefix): expected None, got {result:?}"
        );
    }

    #[test]
    fn parse_signal_declaration_too_few_tokens_returns_none() {
        // Line 4409: <→== — 2-token declaration is invalid.
        let result = super::parse_explicit_signal_declaration("signal clk");
        assert!(
            result.is_none(),
            "signal clk (2 tokens): expected None, got {result:?}"
        );
    }

    // -- parse_explicit_system_clock high-value mutants --

    #[test]
    fn parse_system_clock_signal_middle_word() {
        // Line 4556: ||→&& / match guard false — "clock signal clk" should parse.
        let result = super::parse_explicit_system_clock("clock signal clk");
        assert_eq!(
            result,
            Some("clk".to_string()),
            "clock signal clk: expected Some(\"clk\"), got {result:?}"
        );
    }

    #[test]
    fn parse_system_clock_is_middle_word() {
        // Line 4556: ||→&& — "clock is clk" has middle="is", not "signal".
        let result = super::parse_explicit_system_clock("clock is clk");
        assert_eq!(
            result,
            Some("clk".to_string()),
            "clock is clk: expected Some(\"clk\"), got {result:?}"
        );
    }

    #[test]
    fn parse_system_clock_unknown_middle_word_returns_none() {
        // Line 4556: match guard true — "clock something clk" should return None.
        let result = super::parse_explicit_system_clock("clock something clk");
        assert!(
            result.is_none(),
            "clock something clk: expected None, got {result:?}"
        );
    }

    #[test]
    fn parse_system_clock_two_token_form() {
        // "clock clk" (2 tokens) should parse.
        let result = super::parse_explicit_system_clock("clock clk");
        assert_eq!(
            result,
            Some("clk".to_string()),
            "clock clk: expected Some(\"clk\"), got {result:?}"
        );
    }

    // -- parse_explicit_top_child high-value mutants --

    #[test]
    fn parse_top_child_valid_form() {
        let result = super::parse_explicit_top_child("child cpu0 uses module cpu_core");
        assert!(
            result.is_some(),
            "valid top child: expected Some, got {result:?}"
        );
        let parsed = result.unwrap();
        assert_eq!(parsed.instance_name, "cpu0");
        assert_eq!(parsed.source_module_name, "cpu_core");
    }

    #[test]
    fn parse_top_child_wrong_second_token_returns_none() {
        // Line 4842: ||→&& — "has" instead of "uses" should return None.
        let result = super::parse_explicit_top_child("child cpu0 has module cpu_core");
        assert!(
            result.is_none(),
            "has module: expected None, got {result:?}"
        );
    }

    #[test]
    fn parse_top_child_wrong_third_token_returns_none() {
        // Line 4843: ||→&& — "block" instead of "module" should return None.
        let result = super::parse_explicit_top_child("child cpu0 uses block cpu_core");
        assert!(
            result.is_none(),
            "uses block: expected None, got {result:?}"
        );
    }

    // -- parse_explicit_regular_state_declaration high-value mutants --

    #[test]
    fn parse_regular_state_is_initial() {
        let result = super::parse_explicit_regular_state_declaration("state IDLE is initial");
        assert!(
            result.is_some(),
            "state IDLE is initial: expected Some, got {result:?}"
        );
        let parsed = result.unwrap();
        assert!(parsed.is_initial, "is_initial should be true");
    }

    #[test]
    fn parse_regular_state_unknown_suffix_returns_none() {
        // Line 4900: match guard true — suffix not "initial" should return None.
        let result = super::parse_explicit_regular_state_declaration("state IDLE is something");
        assert!(
            result.is_none(),
            "IDLE is something: expected None, got {result:?}"
        );
    }

    // -- parse_explicit_decision_tree_fragment high-value mutants --

    #[test]
    fn decision_tree_fragment_empty_action_returns_none() {
        // Line 4952: ||→&& — empty action clause after ':' should return None.
        let result = super::parse_explicit_decision_tree_fragment("Block my_block:");
        assert!(
            result.is_none(),
            "empty action: expected None, got {result:?}"
        );
    }

    // -- parse_explicit_control_clause high-value mutants --

    #[test]
    fn control_clause_empty_action_returns_none() {
        // Line 4996: ||→&& — empty action after ':' should return None.
        let empty_set = BTreeSet::new();
        let result = super::parse_explicit_control_clause(
            "block my_block:",
            &empty_set,
            &empty_set,
            &empty_set,
        );
        assert!(
            result.is_none(),
            "block with empty action: expected None, got {result:?}"
        );
    }

    // -- parse_interface_signal_direction high-value mutants --

    #[test]
    fn interface_signal_direction_internal() {
        // Line 5696: ||→&& — "internal" should be recognized.
        let result = super::parse_interface_signal_direction("internal");
        assert_eq!(
            result,
            Some(super::InterfaceSignalDirection::Internal),
            "internal: expected Some(Internal), got {result:?}"
        );
    }

    #[test]
    fn interface_signal_direction_local() {
        // Line 5696: ||→&& — "local" should also be recognized.
        let result = super::parse_interface_signal_direction("local");
        assert_eq!(
            result,
            Some(super::InterfaceSignalDirection::Internal),
            "local: expected Some(Internal), got {result:?}"
        );
    }

    // -- split_explicit_assignment high-value mutants --

    #[test]
    fn split_explicit_assignment_skips_neq_operator() {
        // Line 5201: ||→&& — "a != b" should not be parsed as assignment (= is part of !=).
        let result = super::split_explicit_assignment("a != b");
        assert!(
            result.is_none(),
            "a != b: expected None (should skip !=), got {result:?}"
        );
    }

    // -- parse_control_assignment_target high-value mutants --

    #[test]
    fn control_assignment_target_reg_shorthand() {
        // Line 5234: ||→&& — "reg" should be recognized as registered.
        let result = super::parse_control_assignment_target("my_signal reg");
        assert!(
            result.is_some(),
            "my_signal reg: expected Some, got {result:?}"
        );
    }

    #[test]
    fn control_assignment_target_registered_full() {
        // Line 5234: ||→&& — "registered" should also be recognized.
        let result = super::parse_control_assignment_target("my_signal registered");
        assert!(
            result.is_some(),
            "my_signal registered: expected Some, got {result:?}"
        );
    }

    // -- should_emit_interface_candidate high-value mutants --

    #[test]
    fn should_emit_interface_candidate_ends_with_n() {
        // Lines 6786-6790: ||→&& — signal ending with _N should trigger emission.
        assert!(
            super::should_emit_interface_candidate(&["FOO_N".to_string()]),
            "FOO_N should trigger interface candidate emission"
        );
    }

    #[test]
    fn should_emit_interface_candidate_contains_rst() {
        // Line 6787: ||→&& — signal containing RST should trigger.
        assert!(
            super::should_emit_interface_candidate(&["nRST".to_string()]),
            "nRST should trigger interface candidate emission"
        );
    }

    #[test]
    fn should_emit_interface_candidate_contains_clk() {
        // Line 6789: ||→&& — signal containing CLK should trigger.
        assert!(
            super::should_emit_interface_candidate(&["SYS_CLK".to_string()]),
            "SYS_CLK should trigger interface candidate emission"
        );
    }

    #[test]
    fn should_emit_interface_candidate_single_plain_signal_returns_false() {
        // Lines 6786-6790: ||→&& — "data_out" doesn't match any pattern.
        assert!(
            !super::should_emit_interface_candidate(&["data_out".to_string()]),
            "data_out should not trigger emission"
        );
    }

    #[test]
    fn should_emit_interface_candidate_two_or_more_signals_returns_true() {
        // Line 6780: >=→< — 2+ signals always trigger.
        assert!(
            super::should_emit_interface_candidate(&["sig_a".to_string(), "sig_b".to_string()]),
            "2+ signals should trigger emission"
        );
    }

    // -- is_invariant_like high-value mutants --

    fn make_statement_context(
        class: super::StatementClass,
        text: &str,
        signals: Vec<String>,
    ) -> super::StatementContext {
        super::StatementContext {
            statement_id: "test_stmt".to_string(),
            class,
            text: text.to_string(),
            related_visual_evidence_ids: vec![],
            section_ids: vec![],
            signals,
            supporting_table_ids: vec![],
        }
    }

    fn make_semantic_context() -> super::SemanticContext {
        super::SemanticContext {
            statements: vec![],
            section_anchors: vec![],
            visual_roles_by_id: HashMap::new(),
            actor_signal_relations: vec![],
            signal_semantic_hints: vec![],
        }
    }

    #[test]
    fn is_invariant_like_explicit_abstraction_returns_false() {
        // Line 6881: return true — ExplicitAbstraction should return false.
        let stmt = make_statement_context(
            super::StatementClass::ExplicitAbstraction,
            "this is abstract",
            vec![],
        );
        let ctx = make_semantic_context();
        assert!(
            !super::is_invariant_like(&stmt, &ctx),
            "ExplicitAbstraction should return false"
        );
    }

    #[test]
    fn is_invariant_like_must_with_signals_returns_true() {
        // Line 6906: delete ! — statement with "must" and non-empty signals should return true.
        let stmt = make_statement_context(
            super::StatementClass::NormativeStatement,
            "the signal must be asserted",
            vec!["clk".to_string()],
        );
        let ctx = make_semantic_context();
        assert!(
            super::is_invariant_like(&stmt, &ctx),
            "must + signals should return true"
        );
    }

    #[test]
    fn is_invariant_like_must_no_signals_returns_true() {
        // Line 6907: &&→|| — "must" alone (no signals) should still return true from the first check.
        let stmt = make_statement_context(
            super::StatementClass::NormativeStatement,
            "the design must be verified",
            vec![],
        );
        let ctx = make_semantic_context();
        assert!(
            super::is_invariant_like(&stmt, &ctx),
            "must (no signals) should return true via first phrase check"
        );
    }

    // -- reset_signal_name_looks_active_low high-value mutants --

    #[test]
    fn reset_signal_name_looks_active_low_n_suffix() {
        // Line 4673: ||→&& — _n suffix should return true.
        assert!(
            super::reset_signal_name_looks_active_low("rst_n"),
            "rst_n should look active low"
        );
    }

    #[test]
    fn reset_signal_name_looks_active_low_exact_rstb() {
        // Line 4673: ||→&& — exact match "rstb" should return true.
        assert!(
            super::reset_signal_name_looks_active_low("rstb"),
            "rstb should look active low"
        );
    }

    #[test]
    fn reset_signal_name_looks_active_low_plain_name_returns_false() {
        // Line 4670: return true — "reset" without _n/_b should return false.
        assert!(
            !super::reset_signal_name_looks_active_low("reset"),
            "reset (no _n/_b) should not look active low"
        );
    }

    // -- normalize_infrastructure_component_name high-value mutants --

    #[test]
    fn normalize_infrastructure_component_name_short_text_returns_none() {
        // Line 3848: <→== / <→<= — single char should return None.
        let result = super::normalize_infrastructure_component_name("x");
        assert!(
            result.is_none(),
            "single char: expected None, got {result:?}"
        );
    }

    #[test]
    fn normalize_infrastructure_component_name_valid_term() {
        // Line 3848: <→== — 2+ chars should proceed past the length guard.
        let result = super::normalize_infrastructure_component_name("pll");
        assert!(result.is_some(), "pll: expected Some, got {result:?}");
    }

    // -- split_temporal_condition_segment_on_and high-value mutants --

    fn known_signal_bset(names: &[&str]) -> BTreeSet<String> {
        names.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn split_temporal_on_and_two_anchored_parts() {
        // Lines 7280-7283: >→>= and &&→|| — both parts contain known signals, should split.
        let signals = known_signal_bset(&["CLK", "RST"]);
        let result = super::split_temporal_condition_segment_on_and("CLK and RST", &signals);
        assert_eq!(
            result,
            vec!["CLK".to_string(), "RST".to_string()],
            "both parts anchored: expected split, got {result:?}"
        );
    }

    #[test]
    fn split_temporal_on_and_single_part_no_separator() {
        // Line 7280: >→>= — single part (no "and") should return vec of original text.
        let signals = known_signal_bset(&["CLK"]);
        let result = super::split_temporal_condition_segment_on_and("CLK", &signals);
        assert_eq!(
            result,
            vec!["CLK".to_string()],
            "single part: expected vec with original, got {result:?}"
        );
    }

    #[test]
    fn split_temporal_on_and_unanchored_part_returns_original() {
        // Lines 7281-7283: &&→|| — unanchored part (no known signal) should not split.
        let signals = known_signal_bset(&["CLK"]);
        let result = super::split_temporal_condition_segment_on_and("CLK and UNKNOWN", &signals);
        assert_eq!(
            result,
            vec!["CLK and UNKNOWN".to_string()],
            "unanchored part: expected original text, got {result:?}"
        );
    }

    // -- temporal_rule_from_timing_constraint high-value mutants --

    #[test]
    fn temporal_rule_from_timing_constraint_no_known_signal_returns_none() {
        // Line 8133: return None — description without known signal should return None.
        let timing = super::TimingConstraintRecord {
            constraint_id: "tc1".to_string(),
            parameter_name: "tSU".to_string(),
            min_value: None,
            typ_value: None,
            max_value: None,
            unit: None,
            description: Some("setup time".to_string()),
            supporting_statement_ids: vec![],
            automation_confidence: super::AutomationConfidence::High,
        };
        let signals = known_signal_bset(&["CLK"]);
        let actors = BTreeSet::new();
        let result = super::temporal_rule_from_timing_constraint(
            &timing,
            "unknown signal sampled",
            &signals,
            None,
            &actors,
            None,
        );
        assert!(
            result.is_none(),
            "no known signal: expected None, got {result:?}"
        );
    }

    #[test]
    fn temporal_rule_from_timing_constraint_sampled_returns_some() {
        // Line 8142: &&→|| — "sampled" alone should be sufficient.
        let timing = super::TimingConstraintRecord {
            constraint_id: "tc2".to_string(),
            parameter_name: "tSU".to_string(),
            min_value: None,
            typ_value: None,
            max_value: None,
            unit: None,
            description: Some("setup time".to_string()),
            supporting_statement_ids: vec![],
            automation_confidence: super::AutomationConfidence::High,
        };
        let signals = known_signal_bset(&["CLK"]);
        let actors = BTreeSet::new();
        let result = super::temporal_rule_from_timing_constraint(
            &timing,
            "CLK sampled on rising edge",
            &signals,
            None,
            &actors,
            None,
        );
        assert!(
            result.is_some(),
            "CLK sampled: expected Some, got {result:?}"
        );
    }

    // -- temporal_consequents_from_conditional_rule high-value mutants --

    #[test]
    fn temporal_consequents_no_consequent_signal_returns_empty() {
        // Line 7978: return Vec::new() — None consequent_signal should return empty vec.
        let rule = super::ConditionalRuleRecord {
            rule_id: "test".to_string(),
            antecedent_text: "when HREADY is LOW".to_string(),
            consequent_signal: None,
            consequent_action: "hold".to_string(),
            source_text: "original text".to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: super::AutomationConfidence::High,
        };
        let signals = BTreeSet::new();
        let producers = BTreeMap::new();
        let handshake = super::HandshakeRoleContext::default();
        let result = super::temporal_consequents_from_conditional_rule(
            &rule, &signals, &producers, &handshake,
        );
        assert!(
            result.is_empty(),
            "no consequent signal: expected empty, got {result:?}"
        );
    }

    #[test]
    fn temporal_consequents_hold_action_returns_stable_predicates() {
        // Lines 8001-8003: ||→&& — "hold" action should trigger stability predicates.
        let rule = super::ConditionalRuleRecord {
            rule_id: "test".to_string(),
            antecedent_text: "when HREADY is LOW".to_string(),
            consequent_signal: Some("HTRANS".to_string()),
            consequent_action: "hold".to_string(),
            source_text: "original text".to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: super::AutomationConfidence::High,
        };
        let signals = BTreeSet::new();
        let producers = BTreeMap::new();
        let handshake = super::HandshakeRoleContext::default();
        let result = super::temporal_consequents_from_conditional_rule(
            &rule, &signals, &producers, &handshake,
        );
        assert!(
            !result.is_empty(),
            "hold action: expected non-empty, got {result:?}"
        );
    }

    #[test]
    fn temporal_consequents_asserted_action_returns_signal_value() {
        // Line 8034: ||→&& and ==→!= — "asserted" action should produce SignalValue.
        let rule = super::ConditionalRuleRecord {
            rule_id: "test".to_string(),
            antecedent_text: "when HREADY is LOW".to_string(),
            consequent_signal: Some("HTRANS".to_string()),
            consequent_action: "asserted".to_string(),
            source_text: "original text".to_string(),
            supporting_statement_ids: vec![],
            automation_confidence: super::AutomationConfidence::High,
        };
        let signals = BTreeSet::new();
        let producers = BTreeMap::new();
        let handshake = super::HandshakeRoleContext::default();
        let result = super::temporal_consequents_from_conditional_rule(
            &rule, &signals, &producers, &handshake,
        );
        assert!(
            !result.is_empty(),
            "asserted action: expected non-empty, got {result:?}"
        );
    }

    #[test]
    fn temporal_condition_distributes_shared_assertion_across_signal_list() {
        // TEMPORAL-ANTECEDENT-RECALL: "A, B, and C are asserted" yields one predicate per
        // signal (the shared trailing value distributes), not just the last signal.
        let known: BTreeSet<String> = ["PSEL", "PENABLE", "PREADY"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let handshake = super::HandshakeRoleContext::default();
        let predicates = super::parse_temporal_condition_predicates(
            "PSEL , PENABLE , and PREADY are asserted.",
            &known,
            super::TickPhase::PreTick,
            &handshake,
        );
        let asserted: BTreeSet<String> = predicates
            .iter()
            .filter_map(|p| match p {
                super::TemporalPredicateRecord::SignalValue {
                    signal_name,
                    value,
                    phase,
                } if value == "ASSERTED" && *phase == super::TickPhase::PreTick => {
                    Some(signal_name.clone())
                }
                _ => None,
            })
            .collect();
        assert_eq!(
            asserted,
            ["PENABLE", "PREADY", "PSEL"]
                .iter()
                .map(|s| s.to_string())
                .collect::<BTreeSet<_>>(),
            "all three coordinated signals carry the shared 'asserted' value"
        );
    }

    #[test]
    fn temporal_condition_canonicalizes_unindexed_select_to_declared_family() {
        // WIRE-BASED-100.4: the doc declares the per-completer select as PSELx (→ PSELX) but
        // prose writes the un-indexed "PSEL". The antecedent must resolve to the canonical
        // declared PSELX so the IR uses one identity everywhere; the bare list members PENABLE
        // and PREADY are declared and unaffected.
        let known: BTreeSet<String> = ["PSELX", "PENABLE", "PREADY"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let handshake = super::HandshakeRoleContext::default();
        let predicates = super::parse_temporal_condition_predicates(
            "PSEL , PENABLE , and PREADY are asserted.",
            &known,
            super::TickPhase::PreTick,
            &handshake,
        );
        let asserted: BTreeSet<String> = predicates
            .iter()
            .filter_map(|p| match p {
                super::TemporalPredicateRecord::SignalValue {
                    signal_name, value, ..
                } if value == "ASSERTED" => Some(signal_name.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(
            asserted,
            ["PENABLE", "PREADY", "PSELX"]
                .iter()
                .map(|s| s.to_string())
                .collect::<BTreeSet<_>>(),
            "the un-indexed 'PSEL' resolves to the declared 'PSELX'"
        );
    }

    #[test]
    fn temporal_condition_index_family_generalizes_across_specs() {
        // WIRE-BASED-100.5c: the index-family resolver is GRAMMAR (the x-index convention),
        // not an APB-specific name (ADR 0006) — it must work for AHB's `HSELx` exactly as for
        // APB's `PSELx`. Un-indexed prose "HSEL" resolves to the declared "HSELX".
        let known: BTreeSet<String> = ["HSELX", "HREADY"].iter().map(|s| s.to_string()).collect();
        let handshake = super::HandshakeRoleContext::default();
        let predicates = super::parse_temporal_condition_predicates(
            "HSEL is asserted",
            &known,
            super::TickPhase::PreTick,
            &handshake,
        );
        let asserted: BTreeSet<String> = predicates
            .iter()
            .filter_map(|p| match p {
                super::TemporalPredicateRecord::SignalValue {
                    signal_name, value, ..
                } if value == "ASSERTED" => Some(signal_name.clone()),
                _ => None,
            })
            .collect();
        assert_eq!(
            asserted,
            ["HSELX"]
                .iter()
                .map(|s| s.to_string())
                .collect::<BTreeSet<_>>(),
            "the un-indexed 'HSEL' resolves to the declared 'HSELX' (cross-spec generality)"
        );
    }

    #[test]
    fn temporal_condition_drops_unless_exception_clause() {
        // WIRE-BASED-100.5h: "valid when HREADY is HIGH, unless HRESP is ERROR" → antecedent is
        // HREADY=HIGH only; the "unless HRESP is ERROR" exception must NOT be conjoined.
        let known: BTreeSet<String> = ["HREADY", "HRESP"].iter().map(|s| s.to_string()).collect();
        let handshake = super::HandshakeRoleContext::default();
        let predicates = super::parse_temporal_condition_predicates(
            "HREADY signal is HIGH, unless HRESP signal is ERROR",
            &known,
            super::TickPhase::PreTick,
            &handshake,
        );
        let pairs: BTreeSet<(String, String)> = predicates
            .iter()
            .filter_map(|p| match p {
                super::TemporalPredicateRecord::SignalValue {
                    signal_name, value, ..
                } => Some((signal_name.clone(), value.clone())),
                _ => None,
            })
            .collect();
        assert_eq!(
            pairs,
            [("HREADY".to_string(), "HIGH".to_string())]
                .into_iter()
                .collect::<BTreeSet<_>>(),
            "only HREADY=HIGH; the 'unless HRESP is ERROR' exception is dropped"
        );
    }

    #[test]
    fn temporal_condition_does_not_invent_a_signal_for_unknown_token() {
        // Guard: an un-indexed token with NO declared indexed family member is dropped,
        // never fabricated (resolve_indexed_signal_family is purely additive).
        let known: BTreeSet<String> = ["PENABLE"].iter().map(|s| s.to_string()).collect();
        let handshake = super::HandshakeRoleContext::default();
        let predicates = super::parse_temporal_condition_predicates(
            "WIDGET is asserted",
            &known,
            super::TickPhase::PreTick,
            &handshake,
        );
        assert!(
            predicates.is_empty(),
            "no declared WIDGETx family → no predicate invented, got {predicates:?}"
        );
    }

    #[test]
    fn temporal_condition_mixed_values_are_not_cross_filled() {
        // Guard: a genuinely mixed-value list keeps each signal's own value (no distribution).
        let known: BTreeSet<String> = ["PSEL", "PREADY"].iter().map(|s| s.to_string()).collect();
        let handshake = super::HandshakeRoleContext::default();
        let predicates = super::parse_temporal_condition_predicates(
            "PSEL is HIGH and PREADY is LOW",
            &known,
            super::TickPhase::PreTick,
            &handshake,
        );
        let mut by_signal: BTreeMap<String, String> = BTreeMap::new();
        for p in &predicates {
            if let super::TemporalPredicateRecord::SignalValue {
                signal_name, value, ..
            } = p
            {
                by_signal.insert(signal_name.clone(), value.clone());
            }
        }
        assert_eq!(by_signal.get("PSEL").map(String::as_str), Some("HIGH"));
        assert_eq!(by_signal.get("PREADY").map(String::as_str), Some("LOW"));
    }

    // -- enrich_handshake_completion_predicates high-value mutants --

    #[test]
    fn enrich_handshake_no_predicates_returns_empty() {
        // Returns empty when no predicates given.
        let handshake = super::HandshakeRoleContext::default();
        let result = super::enrich_handshake_completion_predicates(vec![], &handshake);
        assert!(
            result.is_empty(),
            "empty predicates: expected empty, got {result:?}"
        );
    }

    // -- find_known_signal_name high-value mutants --

    #[test]
    fn find_known_signal_prefers_longest_match() {
        // Line 9326: >=→< — longer signal name should replace shorter match.
        let signals = known_signal_bset(&["CLK", "SYS_CLK"]);
        let result = super::find_known_signal_name("SYS_CLK is the main clock", &signals);
        assert_eq!(
            result,
            Some("SYS_CLK".to_string()),
            "should prefer SYS_CLK over CLK, got {result:?}"
        );
    }

    #[test]
    fn find_known_signal_no_match_returns_none() {
        // Line 9326: match guard true — no signal match should return None.
        let signals = known_signal_bset(&["CLK", "RST"]);
        let result = super::find_known_signal_name("no signals here", &signals);
        assert!(result.is_none(), "no match: expected None, got {result:?}");
    }

    // -- statement_by_id high-value mutants --

    #[test]
    fn statement_by_id_finds_matching_statement() {
        // Line 7008: ==→!= — should find statement with matching ID.
        let ctx = make_semantic_context_with_statements(vec![super::StatementContext {
            statement_id: "stmt_one".to_string(),
            class: super::StatementClass::SourceFact,
            text: "statement one".to_string(),
            related_visual_evidence_ids: vec![],
            section_ids: vec![],
            signals: vec![],
            supporting_table_ids: vec![],
        }]);
        let result = super::statement_by_id(&ctx, "stmt_one");
        assert!(result.is_some(), "should find stmt_one, got None");
    }

    #[test]
    fn statement_by_id_no_match_returns_none() {
        // Line 7008: ==→!= — non-matching ID should return None.
        let ctx = make_semantic_context_with_statements(vec![super::StatementContext {
            statement_id: "stmt_one".to_string(),
            class: super::StatementClass::SourceFact,
            text: "statement one".to_string(),
            related_visual_evidence_ids: vec![],
            section_ids: vec![],
            signals: vec![],
            supporting_table_ids: vec![],
        }]);
        let result = super::statement_by_id(&ctx, "stmt_two");
        assert!(
            result.is_none(),
            "non-matching ID: expected None, got {result:?}"
        );
    }

    fn make_semantic_context_with_statements(
        statements: Vec<super::StatementContext>,
    ) -> super::SemanticContext {
        super::SemanticContext {
            statements,
            section_anchors: vec![],
            visual_roles_by_id: HashMap::new(),
            actor_signal_relations: vec![],
            signal_semantic_hints: vec![],
        }
    }

    // -- contains_phrase high-value mutants --

    #[test]
    fn contains_phrase_exact_word_match_returns_true() {
        // Line 7039: &&→|| — exact word boundary match should return true.
        // Note: contains_phrase expects already-lowercased text.
        assert!(
            super::contains_phrase("the clk signal is fast", "CLK"),
            "clk as separate word should match"
        );
    }

    #[test]
    fn contains_phrase_substring_without_boundary_returns_false() {
        // Line 7039: &&→|| — "clk" inside "sclkdiv" has no left boundary (preceded by alphanumeric).
        // With &&→||, prefix_ok=false but suffix_ok=true would incorrectly match via ||.
        assert!(
            !super::contains_phrase("the sclkdiv signal", "CLK"),
            "clk inside sclkdiv should not match (no left boundary)"
        );
    }

    // -- vlm_guard_clause_has_comparison high-value mutants --

    #[test]
    fn vlm_guard_clause_eqeq_operator() {
        // Line 10226: ||→&& — "==" alone should return true.
        assert!(
            super::vlm_guard_clause_has_comparison("a == b"),
            "== should be recognized as comparison"
        );
    }

    #[test]
    fn vlm_guard_clause_ne_operator() {
        // Line 10226: ||→&& — "!=" alone should also return true.
        assert!(
            super::vlm_guard_clause_has_comparison("a != b"),
            "!= should be recognized as comparison"
        );
    }

    #[test]
    fn vlm_guard_clause_no_comparison_returns_false() {
        assert!(
            !super::vlm_guard_clause_has_comparison("no comparison here"),
            "no comparison operators should return false"
        );
    }

    // -- ControlExpressionParser::expect high-value mutants --

    #[test]
    fn control_expression_parser_expect_matching_token() {
        // Line 5588: return None / return Some(()) — matching token should succeed.
        let signals = BTreeSet::new();
        let symbols = BTreeSet::new();
        let mut parser =
            super::ControlExpressionParser::new(vec!["if".to_string()], &signals, &symbols);
        let result = parser.expect("if");
        assert_eq!(
            result,
            Some(()),
            "expect 'if': expected Some(()), got {result:?}"
        );
    }

    #[test]
    fn control_expression_parser_expect_non_matching_token() {
        // Line 5588: return Some(()) — non-matching token should return None.
        let signals = BTreeSet::new();
        let symbols = BTreeSet::new();
        let mut parser =
            super::ControlExpressionParser::new(vec!["if".to_string()], &signals, &symbols);
        let result = parser.expect("else");
        assert_eq!(
            result, None,
            "expect 'else' when token is 'if': expected None, got {result:?}"
        );
    }

    // --- is_false ---

    #[test]
    fn is_false_returns_true_for_false() {
        assert!(is_false(&false));
    }

    #[test]
    fn is_false_returns_false_for_true() {
        assert!(!is_false(&true));
    }

    // --- is_zero ---

    #[test]
    fn is_zero_returns_true_for_zero() {
        assert!(is_zero(&0));
    }

    #[test]
    fn is_zero_returns_false_for_nonzero() {
        assert!(!is_zero(&1));
        assert!(!is_zero(&42));
    }

    // --- is_explicit_infrastructure_component_term ---

    #[test]
    fn is_explicit_infrastructure_component_term_detects_known_terms() {
        assert!(is_explicit_infrastructure_component_term("pll"));
        assert!(is_explicit_infrastructure_component_term("dll"));
        assert!(is_explicit_infrastructure_component_term("clock generator"));
        assert!(is_explicit_infrastructure_component_term(
            "reset controller"
        ));
        assert!(is_explicit_infrastructure_component_term(
            "domain synchronizer"
        ));
        assert!(is_explicit_infrastructure_component_term("clock gate"));
        assert!(is_explicit_infrastructure_component_term("clock mux"));
    }

    #[test]
    fn is_explicit_infrastructure_component_term_rejects_unknown() {
        assert!(!is_explicit_infrastructure_component_term("processor"));
        assert!(!is_explicit_infrastructure_component_term("memory"));
        assert!(!is_explicit_infrastructure_component_term(""));
    }

    // --- control_block_role_key ---

    #[test]
    fn control_block_role_key_returns_correct_strings() {
        assert_eq!(
            control_block_role_key(ControlBlockRole::StateBody),
            "state_body"
        );
        assert_eq!(
            control_block_role_key(ControlBlockRole::ResetSynchronous),
            "reset_synchronous"
        );
        assert_eq!(
            control_block_role_key(ControlBlockRole::ResetAsynchronous),
            "reset_asynchronous"
        );
        assert_eq!(
            control_block_role_key(ControlBlockRole::StandaloneDecisionTree),
            "standalone_decision_tree"
        );
    }

    // --- control_reference_kind_key ---

    #[test]
    fn control_reference_kind_key_returns_correct_strings() {
        assert_eq!(
            control_reference_kind_key(ControlReferenceKind::Unknown),
            "unknown"
        );
        assert_eq!(
            control_reference_kind_key(ControlReferenceKind::Signal),
            "signal"
        );
        assert_eq!(
            control_reference_kind_key(ControlReferenceKind::Symbol),
            "symbol"
        );
    }

    // --- control_unary_operator_key ---

    #[test]
    fn control_unary_operator_key_returns_not() {
        assert_eq!(control_unary_operator_key(ControlUnaryOperator::Not), "not");
    }

    // --- control_binary_operator_key ---

    #[test]
    fn control_binary_operator_key_returns_correct_strings() {
        assert_eq!(
            control_binary_operator_key(ControlBinaryOperator::Add),
            "add"
        );
        assert_eq!(
            control_binary_operator_key(ControlBinaryOperator::Sub),
            "sub"
        );
        assert_eq!(control_binary_operator_key(ControlBinaryOperator::Eq), "eq");
        assert_eq!(
            control_binary_operator_key(ControlBinaryOperator::NotEq),
            "not_eq"
        );
        assert_eq!(control_binary_operator_key(ControlBinaryOperator::Lt), "lt");
        assert_eq!(control_binary_operator_key(ControlBinaryOperator::Ge), "ge");
    }

    // --- decision_tree_comparison_operator_key ---

    #[test]
    fn decision_tree_comparison_operator_key_returns_correct_strings() {
        assert_eq!(
            decision_tree_comparison_operator_key(DecisionTreeComparisonOperator::Eq),
            "eq"
        );
        assert_eq!(
            decision_tree_comparison_operator_key(DecisionTreeComparisonOperator::NotEq),
            "not_eq"
        );
    }

    // --- split_control_header_keyword ---

    #[test]
    fn split_control_header_keyword_splits_on_keyword() {
        let (left, right) = split_control_header_keyword("if (HREADY) then", "then");
        assert_eq!(left, "if (HREADY) ");
        assert_eq!(right, Some(""));
    }

    #[test]
    fn split_control_header_keyword_returns_none_when_keyword_missing() {
        let (left, right) = split_control_header_keyword("if (HREADY)", "then");
        assert_eq!(left, "if (HREADY)");
        assert_eq!(right, None);
    }

    #[test]
    fn split_control_header_keyword_case_insensitive() {
        let (left, right) = split_control_header_keyword("IF (HREADY) THEN action", "then");
        assert_eq!(left, "IF (HREADY) ");
        assert_eq!(right, Some(" action"));
    }

    // --- width_hint_key ---

    #[test]
    fn width_hint_key_numeric() {
        assert_eq!(width_hint_key(&WidthHint::Numeric(32)), "32");
    }

    #[test]
    fn width_hint_key_parametric() {
        assert_eq!(
            width_hint_key(&WidthHint::Parametric("ADDR_WIDTH".to_string())),
            "ADDR_WIDTH"
        );
    }

    // --- control_reference_suffix_key ---

    #[test]
    fn control_reference_suffix_key_member() {
        let result = control_reference_suffix_key(&ControlReferenceSuffix::Member {
            member_name: "ENABLE".to_string(),
        });
        assert_eq!(result, "member:ENABLE");
    }

    #[test]
    fn control_reference_suffix_key_bit_index() {
        let result = control_reference_suffix_key(&ControlReferenceSuffix::BitIndex { index: 3 });
        assert_eq!(result, "bit:3");
    }

    #[test]
    fn control_reference_suffix_key_slice() {
        let result =
            control_reference_suffix_key(&ControlReferenceSuffix::Slice { msb: 7, lsb: 0 });
        assert_eq!(result, "slice:7:0");
    }

    #[test]
    fn control_reference_suffix_key_width_cast() {
        let result = control_reference_suffix_key(&ControlReferenceSuffix::WidthCast { width: 16 });
        assert_eq!(result, "width:16");
    }

    // --- KG-ISF-TRANSACTIONS.2a: Cue-A transaction-name derivation ---

    #[test]
    fn derive_transaction_name_keeps_real_transaction_headings() {
        // Real wire-doc section headings → document-named transactions. The name
        // is the qualifier(s) + the singularized head noun; section numbers,
        // furniture words, and trailing parentheticals are stripped.
        let cases = [
            ("3.1 Write transfers", "write_transfer"),
            ("3.3 Read transfers", "read_transfer"),
            ("Chapter 10 Exclusive Transfers", "exclusive_transfer"),
            (
                "B4.2.1 Successful write operation (OK response)",
                "successful_write_operation",
            ),
            ("A11.1 WriteZero Transaction", "writezero_transaction"),
            ("Atomic transactions", "atomic_transaction"),
            ("5.5.3. Cache Operations", "cache_operation"),
            (
                "Chapter A11 Other write transactions",
                "other_write_transaction",
            ),
        ];
        for (title, expected) in cases {
            assert_eq!(
                derive_transaction_name(title).as_deref(),
                Some(expected),
                "title {title:?} should derive {expected:?}"
            );
        }
    }

    #[test]
    fn derive_transaction_name_rejects_non_transaction_headings() {
        // Sub-topics / aspects of a transaction, not transaction definitions —
        // each rejected by a distinct universal structural rule (no name list).
        let rejected = [
            "10.4 Exclusive Transfer restrictions", // head noun is mid-phrase
            "A2.3.2.1 Write transaction dependencies", // trailing topic noun after head
            "A6.4.4 ID use for Atomic transactions", // contains preposition "for"
            "3.6.1 Burst termination after a BUSY transfer", // preposition "after"
            "A14.4.1 Transmitting one read transaction", // gerund-led + cardinal "one"
            "Example C2-2 Byte packed write operation", // Example: illustration, not a definition
            "3.5. Transfers",                       // head-only after strip, too generic
            "MTE and Atomic transactions",          // coordinated subject ("and")
            "Introduction",                         // not a transaction at all
            "Signal descriptions",                  // wrong head noun
        ];
        for title in rejected {
            assert_eq!(
                derive_transaction_name(title),
                None,
                "title {title:?} must not derive a transaction name"
            );
        }
    }

    #[test]
    fn build_transaction_anchors_dedups_and_records_provenance() {
        // .2c: stmt_1 (in the Write-transfers section) references PADDR/PWRITE;
        // stmt_3 (Read transfers) references PRDATA → grounded signal-set membership.
        let mk_stmt = |id: &str, signals: Vec<&str>| super::StatementContext {
            statement_id: id.to_string(),
            class: super::StatementClass::SourceFact,
            text: String::new(),
            related_visual_evidence_ids: vec![],
            section_ids: vec![],
            signals: signals.into_iter().map(str::to_string).collect(),
            supporting_table_ids: vec![],
        };
        let context = SemanticContext {
            statements: vec![
                // `IDLE` is an enum VALUE the over-capturing token extractor catches;
                // it is NOT a declared signal, so `.2c` must drop it from the set.
                mk_stmt("stmt_1", vec!["PWRITE", "PADDR", "IDLE"]),
                mk_stmt("stmt_3", vec!["PRDATA"]),
            ],
            section_anchors: vec![
                SemanticSectionContext {
                    section_id: "s1".to_string(),
                    title: "3.1 Write transfers".to_string(),
                    supporting_statement_ids: vec!["stmt_1".to_string()],
                },
                SemanticSectionContext {
                    // Same derived name (`write_transfer`) → deduped, first wins.
                    section_id: "s2".to_string(),
                    title: "3.4.1 Write transfer".to_string(),
                    supporting_statement_ids: vec!["stmt_2".to_string()],
                },
                SemanticSectionContext {
                    section_id: "s3".to_string(),
                    title: "3.3 Read transfers".to_string(),
                    supporting_statement_ids: vec!["stmt_3".to_string()],
                },
                SemanticSectionContext {
                    // Not a transaction-defining heading → dropped.
                    section_id: "s4".to_string(),
                    title: "1 Introduction".to_string(),
                    supporting_statement_ids: Vec::new(),
                },
            ],
            visual_roles_by_id: HashMap::new(),
            actor_signal_relations: Vec::new(),
            signal_semantic_hints: Vec::new(),
        };

        // Declared-signal inventory (the document's own signals) — `IDLE` is absent
        // on purpose, so the `.2c` filter must exclude it from the signal set.
        let declared: std::collections::HashSet<String> =
            ["PWRITE", "PADDR", "PRDATA", "PSEL", "PENABLE"]
                .into_iter()
                .map(str::to_string)
                .collect();
        let anchors = build_transaction_anchors(&context, &declared);
        let names: Vec<&str> = anchors
            .iter()
            .map(|a| a.transaction_name.as_str())
            .collect();
        // Heading order preserved; duplicate name and non-transaction dropped.
        assert_eq!(names, vec!["write_transfer", "read_transfer"]);

        let write: &TransactionAnchorRecord = &anchors[0];
        assert_eq!(write.transaction_anchor_id, "txnanchor_write_transfer");
        assert_eq!(write.source_title, "3.1 Write transfers"); // first-wins provenance
        assert_eq!(write.section_id, "s1");
        assert_eq!(write.supporting_statement_ids, vec!["stmt_1".to_string()]);
        // .2c: signal-set membership = the section's own statement signals, sorted+deduped.
        assert_eq!(
            write.signal_set,
            vec!["PADDR".to_string(), "PWRITE".to_string()]
        );
        assert_eq!(anchors[1].signal_set, vec!["PRDATA".to_string()]);
    }

    #[test]
    fn build_transaction_anchors_includes_descendant_subsection_statements() {
        // .2k: a transaction named from a PARENT section (`3.1 Write transfers`)
        // absorbs the signal-rich prose its PDF files under the wait-state
        // subsections (`3.1.1`/`3.1.2`), while a sibling read transaction
        // (`3.3` + `3.3.1`) stays boundary-disjoint (bar #3 — no cross-leak).
        let mk_stmt = |id: &str, signals: Vec<&str>| super::StatementContext {
            statement_id: id.to_string(),
            class: super::StatementClass::SourceFact,
            text: String::new(),
            related_visual_evidence_ids: vec![],
            section_ids: vec![],
            signals: signals.into_iter().map(str::to_string).collect(),
            supporting_table_ids: vec![],
        };
        let mk_sec = |section_id: &str, title: &str, stmts: Vec<&str>| SemanticSectionContext {
            section_id: section_id.to_string(),
            title: title.to_string(),
            supporting_statement_ids: stmts.into_iter().map(str::to_string).collect(),
        };
        let context = SemanticContext {
            statements: vec![
                mk_stmt("w_intro", vec!["PCLK"]),             // 3.1 parent intro
                mk_stmt("w_nowait", vec!["PSEL", "PADDR"]),   // 3.1.1
                mk_stmt("w_wait", vec!["PENABLE", "PWRITE"]), // 3.1.2
                mk_stmt("r_intro", vec!["PCLK"]),             // 3.3 parent intro
                mk_stmt("r_wait", vec!["PRDATA"]),            // 3.3.1
            ],
            section_anchors: vec![
                mk_sec("s_w", "3.1 Write transfers", vec!["w_intro"]),
                mk_sec("s_w1", "3.1.1 With no wait states", vec!["w_nowait"]),
                mk_sec("s_w2", "3.1.2 With wait states", vec!["w_wait"]),
                mk_sec("s_r", "3.3 Read transfers", vec!["r_intro"]),
                mk_sec("s_r1", "3.3.1 With wait states", vec!["r_wait"]),
            ],
            visual_roles_by_id: HashMap::new(),
            actor_signal_relations: Vec::new(),
            signal_semantic_hints: Vec::new(),
        };
        let declared: std::collections::HashSet<String> =
            ["PCLK", "PSEL", "PADDR", "PENABLE", "PWRITE", "PRDATA"]
                .into_iter()
                .map(str::to_string)
                .collect();
        let anchors = build_transaction_anchors(&context, &declared);

        let write = anchors
            .iter()
            .find(|a| a.transaction_name == "write_transfer")
            .expect("write_transfer anchor");
        // Parent absorbs BOTH subsections' declared signals; the read subtree's
        // PRDATA is correctly excluded (boundary precision).
        assert_eq!(
            write.signal_set,
            vec![
                "PADDR".to_string(),
                "PCLK".to_string(),
                "PENABLE".to_string(),
                "PSEL".to_string(),
                "PWRITE".to_string(),
            ]
        );
        assert!(
            write
                .supporting_statement_ids
                .contains(&"w_nowait".to_string())
        );
        assert!(
            write
                .supporting_statement_ids
                .contains(&"w_wait".to_string())
        );

        // Boundary precision (bar #3): the read transaction never pulls the write
        // subtree's PWDATA/PWRITE; it absorbs only its own `3.3.x` descendants.
        let read = anchors
            .iter()
            .find(|a| a.transaction_name == "read_transfer")
            .expect("read_transfer anchor");
        assert_eq!(
            read.signal_set,
            vec!["PCLK".to_string(), "PRDATA".to_string()]
        );
        assert!(!read.signal_set.contains(&"PWRITE".to_string()));
    }

    #[test]
    fn descendant_section_number_is_a_strict_dotted_prefix() {
        assert!(is_descendant_section_number("3.1", "3.1.1"));
        assert!(is_descendant_section_number("3.1", "3.1.2.4"));
        assert!(is_descendant_section_number("B4.2.1", "B4.2.1.3"));
        assert!(is_descendant_section_number("3", "3.1"));
        // Equality is not descent; `3.10` and `3.3.1` are not under `3.1`.
        assert!(!is_descendant_section_number("3.1", "3.1"));
        assert!(!is_descendant_section_number("3.1", "3.10"));
        assert!(!is_descendant_section_number("3.1", "3.3.1"));
        assert_eq!(
            leading_section_number("3.1 Write transfers").as_deref(),
            Some("3.1")
        );
        assert_eq!(
            leading_section_number("B4.2.1 Successful write operation").as_deref(),
            Some("B4.2.1")
        );
        assert_eq!(
            leading_section_number("Chapter 10 Exclusive Transfers"),
            None
        );
        assert_eq!(leading_section_number("Introduction"), None);
    }

    // --- KG-ISF-TRANSACTIONS.2g: prose `<qualifier> phase` recognition ---

    #[test]
    fn derive_phase_name_keeps_real_phase_qualifiers() {
        // The single-word qualifier before a `phase` head, on the wire docs:
        // APB setup/access, AHB/AXI address/data, SWD response/turnaround/acknowledge.
        // Case is folded and edge punctuation is trimmed.
        let cases = [
            ("address", "address"),
            ("Address", "address"),
            ("data", "data"),
            ("setup", "setup"),
            ("access", "access"), // ends in `ss` — must NOT be mistaken for a plural
            ("response", "response"),
            ("turnaround", "turnaround"),
            ("acknowledge", "acknowledge"),
            ("(data)", "data"), // edge punctuation trimmed
        ];
        for (raw, expected) in cases {
            assert_eq!(
                derive_phase_name(raw).as_deref(),
                Some(expected),
                "qualifier {raw:?} should name phase {expected:?}"
            );
        }
    }

    #[test]
    fn derive_phase_name_rejects_noise() {
        // Each rejected by a distinct universal structural rule (no name list).
        let rejected = [
            "the",        // determiner
            "this",       // demonstrative
            "each",       // quantifier
            "any",        // quantifier
            "for",        // preposition
            "four",       // cardinal
            "first",      // ordinal
            "separate",   // position/quantity adjective
            "transfer",   // transaction head noun used as a modifier ("data transfer phase")
            "transfers",  // plural head noun
            "processing", // gerund-led verb
            "data.",      // sentence/clause boundary (previous token ends a sentence)
            "lanes:",     // clause boundary
            "|",          // symbol → empty after trim
            "aw",         // too short
            "3-1",        // non-alphabetic
        ];
        for raw in rejected {
            assert_eq!(
                derive_phase_name(raw),
                None,
                "qualifier {raw:?} must not name a phase"
            );
        }
    }

    #[test]
    fn build_transaction_phases_dedups_and_records_provenance() {
        let mk_stmt = |id: &str, text: &str, signals: &[&str]| super::StatementContext {
            statement_id: id.to_string(),
            class: super::StatementClass::SourceFact,
            text: text.to_string(),
            related_visual_evidence_ids: vec![],
            section_ids: vec![],
            signals: signals.iter().map(|s| s.to_string()).collect(),
            supporting_table_ids: vec![],
        };
        let context = make_semantic_context_with_statements(vec![
            // First sighting sets order: address, then data.
            mk_stmt(
                "stmt_1",
                "During the address phase the manager drives HADDR.",
                &["HADDR"],
            ),
            // `the phase` is determiner noise — dropped; the `data phase` mention
            // records provenance against the existing `data` record.
            mk_stmt(
                "stmt_2",
                "In the data phase HWDATA is valid; the phase then ends.",
                &["HWDATA"],
            ),
            // Second `data phase` sighting accumulates provenance (deduped per id).
            // `FOO` is referenced but NOT declared → filtered from the signal set.
            mk_stmt(
                "stmt_3",
                "A wait extends the data phase until HREADY is high.",
                &["HREADY", "FOO"],
            ),
        ]);
        // The document's declared-signal inventory (FOO is deliberately absent).
        let declared: std::collections::HashSet<String> = ["HADDR", "HWDATA", "HREADY"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let phases = build_transaction_phases(&context, &declared);
        let names: Vec<&str> = phases.iter().map(|p| p.phase_name.as_str()).collect();
        // Statement order preserved; determiner noise dropped; `data` deduped.
        assert_eq!(names, vec!["address", "data"]);

        let address: &TransactionPhaseRecord = &phases[0];
        assert_eq!(address.transaction_phase_id, "txnphase_address");
        assert_eq!(address.supporting_statement_ids, vec!["stmt_1".to_string()]);
        // KG-ISF-TRANSACTIONS.2i: the phase's grounded signal set (declared ∩ referenced).
        assert_eq!(address.signal_set, vec!["HADDR".to_string()]);
        assert_eq!(
            address.automation_confidence,
            super::AutomationConfidence::Medium
        );

        let data = &phases[1];
        assert_eq!(data.transaction_phase_id, "txnphase_data");
        // Provenance accumulated across both naming statements, sorted + deduped.
        assert_eq!(
            data.supporting_statement_ids,
            vec!["stmt_2".to_string(), "stmt_3".to_string()]
        );
        // Signal set is the union over both `data phase` statements, sorted, with the
        // undeclared `FOO` filtered out (the `.2c` intersection technique).
        assert_eq!(
            data.signal_set,
            vec!["HREADY".to_string(), "HWDATA".to_string()]
        );
    }
}

#[cfg(test)]
mod fidelity_gate_routing_tests {
    //! `R16-CAPTURE-FIDELITY-GATES.3` — the producer must mechanically
    //! enforce the residual-honesty doctrine: a `Fail` on a `Lowerable`
    //! contract must be rerouted to `Residual{reason}`, NOT silently
    //! lowered. Already-Residual contracts and Pass-only contracts stay
    //! untouched.

    use super::{ActorPortRecord, ActorRelativeDirection, ClockEdge, apply_fidelity_gates};
    use crate::ir::contract::{
        ActorContract, ContractKind, ContractProvenance, EvidenceModality, LoweringDisposition,
        Obligation,
    };
    use crate::ir::fidelity::{FidelityGate, FindingStatus};
    use crate::ir::source::AutomationConfidence;

    fn contract(
        id: &str,
        actor: Option<&str>,
        obligation: Obligation,
        kind: ContractKind,
        lowering: LoweringDisposition,
    ) -> ActorContract {
        ActorContract {
            contract_id: id.into(),
            source_rule_id: Some(id.into()),
            actor_name: actor.map(|s| s.to_string()),
            kind,
            guard: None,
            guard_candidates: vec![],
            obligation,
            clock_signal: None,
            edge: ClockEdge::Rising,
            channel: None,
            phase: None,
            provenance: ContractProvenance {
                supporting_statement_ids: vec![],
                source_text: "src".into(),
                modality: EvidenceModality::Prose,
            },
            lowering,
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    fn port(actor: &str, signal: &str, dir: ActorRelativeDirection) -> ActorPortRecord {
        ActorPortRecord {
            actor_id: format!("{actor}_id"),
            actor_name: actor.into(),
            signal_name: signal.into(),
            direction: dir,
            relation_basis: vec![],
            width_hint: None,
            source_statement_ids: vec![],
            automation_confidence: AutomationConfidence::Medium,
        }
    }

    #[test]
    fn lowerable_observe_reroutes_to_residual_with_gate_reason() {
        // Observe + Lowerable is the textbook Fail of both
        // ResidualHonesty and NoStrictInvalid — doctrine demands
        // Residual after the gate runs.
        let mut cs = vec![contract(
            "obs",
            Some("A"),
            Obligation::Observe { signal: "X".into() },
            ContractKind::Guarantee,
            LoweringDisposition::Lowerable,
        )];
        let ports = vec![port("A", "X", ActorRelativeDirection::Output)];
        let findings = apply_fidelity_gates(&mut cs, &ports);
        let lowering = &cs[0].lowering;
        match lowering {
            LoweringDisposition::Residual { reason } => {
                assert!(
                    reason.contains("fidelity:") && reason.contains("ResidualHonesty"),
                    "{reason}"
                );
            }
            other => panic!("expected Residual, got {other:?}"),
        }
        // The pre-routing Fail finding is still recorded (it is the
        // doctrine working, not a bug).
        assert!(
            findings.iter().any(|f| f.status == FindingStatus::Fail
                && matches!(f.gate, FidelityGate::ResidualHonesty))
        );
    }

    #[test]
    fn pass_only_contract_stays_lowerable() {
        let mut cs = vec![contract(
            "drv",
            Some("A"),
            Obligation::Drive {
                signal: "Q".into(),
                value: "1".into(),
            },
            ContractKind::Guarantee,
            LoweringDisposition::Lowerable,
        )];
        let ports = vec![port("A", "Q", ActorRelativeDirection::Output)];
        let findings = apply_fidelity_gates(&mut cs, &ports);
        assert!(matches!(cs[0].lowering, LoweringDisposition::Lowerable));
        // No Fail recorded for this clean contract.
        assert!(findings.iter().all(|f| f.status != FindingStatus::Fail));
    }

    #[test]
    fn already_residual_contract_is_not_touched_by_routing() {
        let mut cs = vec![contract(
            "r",
            None, // unknown actor → direction NotEvaluated
            Obligation::Stable {
                signal: "MISSING".into(), // would Fail boundary
                during: crate::ir::contract::Window::SameCycle,
            },
            ContractKind::Assume,
            LoweringDisposition::Residual {
                reason: "preexisting".into(),
            },
        )];
        let ports: Vec<ActorPortRecord> = vec![]; // empty boundary
        let _ = apply_fidelity_gates(&mut cs, &ports);
        match &cs[0].lowering {
            LoweringDisposition::Residual { reason } => {
                assert_eq!(reason, "preexisting", "router must not rewrite Residual");
            }
            other => panic!("expected Residual unchanged, got {other:?}"),
        }
    }
}
