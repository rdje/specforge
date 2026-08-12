use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::intent::{IntentDocumentIdentity, IntentIr};
use crate::ir::isf_ir::IsfIr;
use crate::ir::source::{AutomationConfidence, CandidateInterpretation, ResidualDecisionPacket};
use crate::persisted_path::{
    PersistedPathOrigin, normalize_for_storage, resolve_existing, resolve_repository_output,
};
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum AdapterTarget {
    Isf,
}

impl AdapterTarget {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Isf => "isf",
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
            target: AdapterTarget::Isf,
            required_input_stage: IrStage::IntentIr,
            status: AdapterStatus::Implemented,
            notes: vec![
                "Intent Scheduling Format (.isf) adapter — emits high-level scheduling intent".to_string(),
                "FSMGen consumes .isf and owns scheduling, .fsm, and HDL downstream; SpecForge does not do cycle scheduling".to_string(),
            ],
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
    pub stage: IrStage,
    pub schema_version: u32,
    pub target: AdapterTarget,
    pub required_input_stage: IrStage,
    pub intent_ir_path: PathBuf,
    pub artifact_layout: AdapterArtifactLayout,
    pub adapter_identity: AdapterIdentity,
    pub document_identity: IntentDocumentIdentity,
    pub lowering_status: AdapterLoweringStatus,
    pub residual_decisions: Vec<ResidualDecisionPacket>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub validation_reports: Vec<crate::ir::source::ValidationReportRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isf: Option<IsfAdapterArtifact>,
}

impl AdapterArtifact {
    pub fn load_from_path(path: &Path) -> Result<Self> {
        let path = resolve_existing(path, PersistedPathOrigin::RepositoryOwned)?;
        let artifact = serde_json::from_str::<Self>(&fs::read_to_string(path)?)?;
        if !matches!(artifact.stage, IrStage::IsfAdapter) {
            return Err(AppError::InvalidStageArtifact(
                "artifact must be an ISF adapter document before loading an adapter".to_string(),
            ));
        }
        artifact.runtime_clone()
    }

    pub fn build(
        intent_ir_path: &Path,
        target: AdapterTarget,
        artifact_base_root: &Path,
    ) -> Result<Self> {
        let intent_ir_path =
            resolve_existing(intent_ir_path, PersistedPathOrigin::RepositoryOwned)?;
        let intent_ir = IntentIr::load_from_path(&intent_ir_path)?;

        if !matches!(intent_ir.stage, IrStage::IntentIr) {
            return Err(AppError::InvalidStageArtifact(format!(
                "artifact at {} must be an IntentIR document before building an adapter artifact",
                intent_ir_path.display()
            )));
        }

        match target {
            AdapterTarget::Isf => {
                build_isf_adapter_artifact(&intent_ir, &intent_ir_path, artifact_base_root)
            }
        }
    }

    pub fn to_pretty_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(&self.persisted_clone()?)?)
    }

    pub fn write_to_disk(&self) -> Result<()> {
        let persisted = self.persisted_clone()?;
        let runtime_layout = persisted.artifact_layout.runtime_layout()?;
        fs::create_dir_all(&runtime_layout.artifact_root)?;
        fs::write(
            &runtime_layout.adapter_artifact_path,
            serde_json::to_string_pretty(&persisted)?,
        )?;

        if let (Some(path), Some(text)) = (
            runtime_layout.emitted_target_path.as_ref(),
            persisted.rendered_target_text(),
        ) {
            fs::write(path, text)?;
        }

        reconcile_emitted_isf_files(&runtime_layout)?;

        Ok(())
    }

    fn persisted_clone(&self) -> Result<Self> {
        let mut persisted = self.clone();
        persisted.intent_ir_path = normalize_for_storage(
            &persisted.intent_ir_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        persisted.artifact_layout.normalize_for_storage()?;
        Ok(persisted)
    }

    fn runtime_clone(&self) -> Result<Self> {
        let mut runtime = self.clone();
        runtime.intent_ir_path = resolve_existing(
            &runtime.intent_ir_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        runtime.artifact_layout = runtime.artifact_layout.runtime_layout()?;
        Ok(runtime)
    }

    fn rendered_target_text(&self) -> Option<String> {
        match &self.isf {
            Some(isf) if isf.is_renderable => Some(isf.source_text.clone()),
            _ => None,
        }
    }
}

/// Make the generated document directory agree with the adapter manifest after
/// a successful write. Actor selection and renderability can change between
/// runs, so retaining an older sibling `.isf` would expose an output that the
/// current `adapter.json` neither names nor licenses. Only generated ISF leaf
/// files are reconciled; unrelated files and directories remain untouched.
fn reconcile_emitted_isf_files(layout: &AdapterArtifactLayout) -> Result<()> {
    for entry in fs::read_dir(&layout.artifact_root)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().is_none_or(|extension| extension != "isf")
            || layout
                .emitted_target_path
                .as_ref()
                .is_some_and(|current| current == &path)
        {
            continue;
        }

        let file_type = entry.file_type()?;
        if file_type.is_file() || file_type.is_symlink() {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterArtifactLayout {
    pub artifact_root: PathBuf,
    pub adapter_artifact_path: PathBuf,
    pub emitted_target_path: Option<PathBuf>,
}

impl AdapterArtifactLayout {
    fn normalize_for_storage(&mut self) -> Result<()> {
        self.artifact_root =
            normalize_for_storage(&self.artifact_root, PersistedPathOrigin::RepositoryOwned)?;
        self.adapter_artifact_path = normalize_for_storage(
            &self.adapter_artifact_path,
            PersistedPathOrigin::RepositoryOwned,
        )?;
        if let Some(path) = &mut self.emitted_target_path {
            *path = normalize_for_storage(path, PersistedPathOrigin::RepositoryOwned)?;
        }
        Ok(())
    }

    fn runtime_layout(&self) -> Result<Self> {
        Ok(Self {
            artifact_root: resolve_repository_output(&self.artifact_root)?,
            adapter_artifact_path: resolve_repository_output(&self.adapter_artifact_path)?,
            emitted_target_path: self
                .emitted_target_path
                .as_deref()
                .map(resolve_repository_output)
                .transpose()?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterIdentity {
    pub adapter_id: String,
    pub summary: String,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IsfAdapterArtifact {
    pub actor_name: String,
    pub source_text: String,
    pub is_renderable: bool,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocking_reasons: Vec<String>,
    pub signal_count: usize,
    pub transaction_count: usize,
    pub rule_count: usize,
    pub constant_count: usize,
    pub enum_count: usize,
    pub storage_count: usize,
}
fn derive_isf_actor_name(intent_ir: &IntentIr) -> String {
    // KG-ISF-COMPLETENESS.2a.ii: the single flat `.isf` module is lowered from the protocol's
    // INITIATOR actor's perspective (owner-chosen, `2026-06-17`), so the module is NAMED after that
    // same initiator — keeping the module label and its `(input)`/`(output)` interface coherent (the
    // emitter, `isf_ir::from_intent_ir`, derives the directions from the same initiator). The
    // initiator is recovered structurally from the actor-port graph (`select_initiator_actor`, no
    // chip-name list — ADR 0006); when there is no net-producer initiator (e.g. a register/command
    // doc with no wire actors) we keep the prior `actors.first()` → `document_key` fallback, so those
    // documents' module names are byte-identical.
    // KG-ISF-COMPLETENESS.2a.iii: route every candidate through the shared HDL-identifier sanitizer
    // (`sanitize_isf_name`, an allowlist) so the emitted `(actor <name>)` module label is always a valid
    // FSMGen module name (`[A-Za-z_]\w*`) — a prose-fragment initiator carrying punctuation (e.g. the
    // arrow `→` in GIC-600's `redistributor → distributor`) no longer malforms the whole `.isf`. This is
    // ONLY the module LABEL; `from_intent_ir` re-derives the initiator raw for direction matching, so the
    // sanitized label cannot affect initiator port selection. Byte-identical for clean names
    // (`Manager`→`manager`, `Requester`→`requester`, `debugger`), so the wire golds are unchanged.
    if let Some(initiator) = crate::ir::isf_ir::select_initiator_actor(&intent_ir.actor_ports)
        && !initiator.is_empty()
    {
        return crate::ir::isf_ir::sanitize_isf_name(&initiator);
    }
    if let Some(actor) = intent_ir.actors.first()
        && let Some(name) = &actor.actor_name
        && !name.is_empty()
    {
        return crate::ir::isf_ir::sanitize_isf_name(name);
    }
    crate::ir::isf_ir::sanitize_isf_name(&intent_ir.document_identity.document_key)
}

fn count_isf_signals(intent_ir: &IntentIr) -> usize {
    let mut seen = BTreeSet::new();
    for iface in &intent_ir.interfaces {
        for sig in &iface.signal_records {
            seen.insert(&sig.signal_name);
        }
    }
    seen.len()
}

// ISF renderability policy (R6-ISF-ADAPTER.4 decision).
//
// ISF lowering blocks on exactly two conditions:
//   1. no signals declared in any interface, and
//   2. no behavioral content (temporal rules, conditional rules, signal
//      constraints, or control blocks).
//
// Missing per-signal direction or width is *deliberately not* a blocker:
// `IsfIr::from_intent_ir` defaults an unknown direction to `output` and an
// unknown width to `1`. This is intentional and is the key policy difference
// from the stricter `.fsm` adapter, which blocks on missing/conflicting
// direction or width. The rationale: FSMGen performs the cycle scheduling for
// `.isf`, and ISF accepts a default direction/width, so blocking on those
// would over-restrict otherwise-honest lowering without improving downstream
// correctness. The `.fsm` path stays strict because it must not fabricate
// target syntax; the `.isf` path can safely default and let FSMGen schedule.
fn assess_isf_renderability(intent_ir: &IntentIr) -> (bool, Vec<String>) {
    let mut reasons: Vec<String> = Vec::new();

    let signal_count = count_isf_signals(intent_ir);
    if signal_count == 0 {
        reasons.push("no signals declared in interface".to_string());
    } else {
        // Direction and width missing are informational only — the ISF IR
        // defaults them to output / width 1 so emission can proceed (see the
        // ISF renderability policy comment above).
    }

    let has_behavior = !intent_ir.temporal_rules.is_empty()
        || !intent_ir.conditional_rules.is_empty()
        || !intent_ir.signal_constraints.is_empty()
        || !intent_ir.control_blocks.is_empty();
    if !has_behavior {
        reasons.push(
            "no behavioral content (temporal rules, conditional rules, signal constraints, or control blocks)"
                .to_string(),
        );
    }

    (reasons.is_empty(), reasons)
}

fn protocol_residual_packet(
    surface_id: &str,
    surface_label: &str,
    record_id: &str,
    supporting_statement_ids: &[String],
    missing_bindings: &str,
) -> ResidualDecisionPacket {
    let provenance = if supporting_statement_ids.is_empty() {
        "no supporting statement ids".to_string()
    } else {
        format!(
            "supporting statements: {}",
            supporting_statement_ids.join(", ")
        )
    };
    ResidualDecisionPacket {
        packet_id: format!(
            "isf_protocol_{surface_id}_{}",
            crate::ir::isf_ir::sanitize_isf_name(record_id)
        ),
        question: format!(
            "How should {surface_label} record `{record_id}` be represented downstream of `.isf`?"
        ),
        why_unresolved: format!(
            "The record does not carry {missing_bindings}. It remains exact in IntentIR and is not rendered; no missing binding is inferred. Source provenance: {provenance}."
        ),
        automation_confidence: AutomationConfidence::Low,
        candidate_interpretations: vec![
            CandidateInterpretation {
                interpretation_id: "preserve_intent_observation".to_string(),
                description: "Keep the grounded protocol observation in IntentIR and expose this adapter residual without emitting executable syntax.".to_string(),
                downstream_impact: "Independently licensed ISF remains usable, while the protocol behavior stays visibly incomplete.".to_string(),
            },
            CandidateInterpretation {
                interpretation_id: "lower_after_bindings".to_string(),
                description: "Lower the record only after upstream typed data supplies every operand required by a supported ISF construct.".to_string(),
                downstream_impact: "A future adapter may emit the behavior after the missing bindings are source-grounded and FSMGen-strict verified.".to_string(),
            },
        ],
    }
}

/// Account for every canonical protocol observation at the adapter boundary (ADR 0016).
///
/// None of the current record types supplies every operand required by a supported executable ISF
/// construct. Keep one stable residual packet per input record, in schema and record order, instead
/// of silently dropping observations or fabricating transitions, values, ports, storage, or timing.
fn protocol_residual_decisions(intent_ir: &IntentIr) -> Vec<ResidualDecisionPacket> {
    let mut residuals = Vec::with_capacity(
        intent_ir.serial_frame_fields.len()
            + intent_ir.protocol_operations.len()
            + intent_ir.protocol_states.len()
            + intent_ir.interface_edge_timings.len(),
    );

    for record in &intent_ir.serial_frame_fields {
        residuals.push(protocol_residual_packet(
            "serial_frame_field",
            "serial-frame field",
            &record.field_id,
            &record.supporting_statement_ids,
            "an enclosing transaction/operation binding, the declared serial data wire, and complete literal, activation, and storage semantics",
        ));
    }
    for record in &intent_ir.protocol_operations {
        residuals.push(protocol_residual_packet(
            "operation",
            "protocol operation",
            &record.operation_id,
            &record.supporting_statement_ids,
            "a typed activation condition, ordered field/step membership, bound ports, and state-transition effects",
        ));
    }
    for record in &intent_ir.protocol_states {
        residuals.push(protocol_residual_packet(
            "state",
            "protocol state",
            &record.state_id,
            &record.supporting_statement_ids,
            "typed transitions, guards, initial-state identity, encoding, and bound action operands",
        ));
    }
    for record in &intent_ir.interface_edge_timings {
        residuals.push(protocol_residual_packet(
            "interface_edge_timing",
            "interface-edge timing",
            &record.timing_id,
            &record.supporting_statement_ids,
            "an activation/guard, sample destination, drive value, and transaction or state association",
        ));
    }

    residuals
}

fn build_isf_adapter_artifact(
    intent_ir: &IntentIr,
    intent_ir_path: &Path,
    artifact_base_root: &Path,
) -> Result<AdapterArtifact> {
    let artifact_root = artifact_base_root
        .join(AdapterTarget::Isf.as_str())
        .join(&intent_ir.document_identity.document_key);
    let adapter_artifact_path = artifact_root.join("adapter.json");

    let actor_name = derive_isf_actor_name(intent_ir);
    // Build the typed ISF model once so the emitted source text and the
    // temporal residual decisions come from the exact same lowering pass
    // (ISF-TEMPORAL-LOWERING.2.3 — the residual set must reflect what the
    // emitter actually did, not a re-derived guess).
    let isf_model = IsfIr::from_intent_ir(intent_ir, &actor_name);
    let source_text = isf_model.render();
    let (is_renderable, blocking_reasons) = assess_isf_renderability(intent_ir);

    let signal_count = count_isf_signals(intent_ir);
    // ISF-TEMPORAL-LOWERING.2.4: report what the emitter actually renders,
    // not a blind IntentIR-derived guess. The old
    // `transaction_count = temporal_rules.len() + cb_tx_count` and
    // `rule_count = conditional_rules + signal_constraints` counted surfaces
    // the emitter ignores (the entire temporal_rules set pre-.2.2/.2.3, and
    // control_blocks that may not all lower) and missed temporal-synthesized
    // transactions and temporal `(rule …)`. Derive both from the single ISF
    // model built above so the metric == emitted content.
    let transaction_count = isf_model.emitted_transaction_count();
    let rule_count = isf_model.emitted_rule_count();
    // Count emitted content, not recovered symbols, so the artifact metric
    // matches the rendered `.isf` (same doctrine as transaction/rule counts;
    // `render()`'s safe-value filter may exclude some recovered symbols).
    let constant_count = isf_model.emitted_constant_count();
    let enum_count = isf_model.emitted_enum_count();
    let storage_count = intent_ir.register_records.len();

    let emitted_target_path = if is_renderable {
        Some(artifact_root.join(format!("{}.isf", actor_name)))
    } else {
        None
    };

    let lowering_status = if is_renderable {
        AdapterLoweringStatus::Renderable
    } else {
        AdapterLoweringStatus::Blocked
    };

    let adapter_identity = AdapterIdentity {
        adapter_id: format!(
            "adapter_{}_{}",
            AdapterTarget::Isf.as_str(),
            intent_ir.document_identity.document_key
        ),
        summary: format!(
            ".isf lowering artifact for {}",
            intent_ir.document_identity.display_name
        ),
    };

    let isf = IsfAdapterArtifact {
        actor_name: actor_name.clone(),
        source_text,
        is_renderable,
        blocking_reasons,
        signal_count,
        transaction_count,
        rule_count,
        constant_count,
        enum_count,
        storage_count,
    };

    // Temporal rules with no representable supported ISF construct are
    // preserved as explicit residual decisions (ISF-TEMPORAL-LOWERING.2.3
    // mapping #4) so a dropped temporal obligation is visible in the
    // artifact rather than silently lost; syntax is never fabricated.
    let mut residual_decisions = intent_ir.residual_decisions.clone();
    // Every projected protocol record receives an
    // explicit adapter disposition. The current directly lowerable subset is empty because the
    // record types do not carry every executable binding; per-record residuals keep that omission
    // visible without blocking independently licensed ISF.
    residual_decisions.extend(protocol_residual_decisions(intent_ir));
    residual_decisions.extend(isf_model.temporal_residuals().iter().cloned());
    // Register resets that could not be lowered to a storage `(reset V)`
    // (ISF-REGISTER-RESET-EMIT.2) are surfaced the same way, so an un-lowered reset is
    // visible in the artifact rather than silently dropped; no value is fabricated.
    residual_decisions.extend(isf_model.storage_reset_residuals().iter().cloned());
    // Register bit-fields that could not be lowered to the storage `(fields …)` block
    // (DOC-INTENT-TAXONOMY.4a.ii) are surfaced the same way, so the largest measurable
    // intent-loss is visible in the artifact rather than silently dropped — the full field map
    // always remains in IntentIR register_records; no bit position is fabricated.
    residual_decisions.extend(isf_model.storage_field_residuals().iter().cloned());
    // Enums held out of the `.isf` by the member-value-width gate (KG-ISF-COMPLETENESS.2a.iv)
    // — a mega-conflated / over-width enum whose member literal FSMGen would reject — are surfaced
    // the same way, so the dropped enum surface is explicit rather than silently lost; no radix is
    // fabricated and no value is truncated.
    residual_decisions.extend(isf_model.enum_residuals().iter().cloned());

    let artifact_layout = AdapterArtifactLayout {
        artifact_root,
        adapter_artifact_path,
        emitted_target_path,
    }
    .runtime_layout()?;

    Ok(AdapterArtifact {
        stage: IrStage::IsfAdapter,
        schema_version: 1,
        target: AdapterTarget::Isf,
        required_input_stage: IrStage::IntentIr,
        intent_ir_path: intent_ir_path.to_path_buf(),
        artifact_layout,
        adapter_identity,
        document_identity: intent_ir.document_identity.clone(),
        lowering_status,
        residual_decisions,
        validation_reports: vec![],
        isf: Some(isf),
    })
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;

    use tempfile::tempdir;

    use crate::error::Result;
    use crate::ir::adapters::{AdapterArtifact, AdapterLoweringStatus, AdapterTarget};
    use crate::ir::evidence::{
        EvidenceIr, InterfaceClockEdge, InterfaceEdgeTimingRecord, ParticipantDriveRecord,
        ProtocolOperationRecord, ProtocolStateRecord, SerialFrameField,
    };
    use crate::ir::intent::IntentIr;
    use crate::ir::semantic::SemanticIr;
    use crate::ir::source::SourceIr;

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

    #[test]
    fn downstream_artifacts_store_relative_paths_and_rebase_legacy_lineage() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "portable_downstream_paths.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let semantic_ir_path = intent_ir.semantic_ir_path.clone();
        let semantic_ir = SemanticIr::load_from_path(&semantic_ir_path)?;
        let evidence_ir_path = semantic_ir.evidence_ir_path.clone();
        let intent_ir_path = intent_ir.artifact_layout.intent_ir_path.clone();
        let adapter = AdapterArtifact::build(
            &intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("generated/adapters"),
        )?;
        adapter.write_to_disk()?;
        let adapter_path = adapter.artifact_layout.adapter_artifact_path.clone();

        let repository = crate::project_data::repository_root()?;
        for path in [&semantic_ir_path, &intent_ir_path, &adapter_path] {
            let stored = fs::read_to_string(path)?;
            assert!(
                !stored.contains(repository.to_string_lossy().as_ref()),
                "repository-owned downstream paths must serialize without the runtime root: {}",
                path.display()
            );
        }

        fn retire(value: &mut serde_json::Value) {
            let relative = value.as_str().expect("stored path string");
            assert!(Path::new(relative).is_relative());
            *value = serde_json::Value::String(
                Path::new("/retired/specforge")
                    .join(relative)
                    .to_string_lossy()
                    .into_owned(),
            );
        }

        let mut semantic_json =
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&semantic_ir_path)?)?;
        for pointer in [
            "/evidence_ir_path",
            "/artifact_layout/artifact_root",
            "/artifact_layout/semantic_ir_path",
        ] {
            retire(semantic_json.pointer_mut(pointer).expect("semantic path"));
        }
        fs::write(
            &semantic_ir_path,
            serde_json::to_string_pretty(&semantic_json)?,
        )?;

        let mut intent_json =
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&intent_ir_path)?)?;
        for pointer in [
            "/semantic_ir_path",
            "/artifact_layout/artifact_root",
            "/artifact_layout/intent_ir_path",
        ] {
            retire(intent_json.pointer_mut(pointer).expect("intent path"));
        }
        fs::write(&intent_ir_path, serde_json::to_string_pretty(&intent_json)?)?;

        let mut adapter_json =
            serde_json::from_str::<serde_json::Value>(&fs::read_to_string(&adapter_path)?)?;
        for pointer in [
            "/intent_ir_path",
            "/artifact_layout/artifact_root",
            "/artifact_layout/adapter_artifact_path",
            "/artifact_layout/emitted_target_path",
        ] {
            retire(adapter_json.pointer_mut(pointer).expect("adapter path"));
        }
        fs::write(&adapter_path, serde_json::to_string_pretty(&adapter_json)?)?;

        let reloaded_semantic = SemanticIr::load_from_path(&semantic_ir_path)?;
        assert_eq!(reloaded_semantic.evidence_ir_path, evidence_ir_path);
        assert!(
            reloaded_semantic
                .artifact_layout
                .artifact_root
                .is_absolute()
        );
        assert!(
            !reloaded_semantic
                .to_pretty_json()?
                .contains("/retired/specforge")
        );

        let reloaded_intent = IntentIr::load_from_path(&intent_ir_path)?;
        assert_eq!(reloaded_intent.semantic_ir_path, semantic_ir_path);
        assert!(reloaded_intent.artifact_layout.artifact_root.is_absolute());
        assert!(
            !reloaded_intent
                .to_pretty_json()?
                .contains("/retired/specforge")
        );

        let reloaded_adapter = AdapterArtifact::load_from_path(&adapter_path)?;
        assert_eq!(reloaded_adapter.intent_ir_path, intent_ir_path);
        assert!(reloaded_adapter.artifact_layout.artifact_root.is_absolute());
        assert!(
            reloaded_adapter
                .artifact_layout
                .emitted_target_path
                .as_ref()
                .is_some_and(|path| path.is_absolute())
        );
        assert!(
            !reloaded_adapter
                .to_pretty_json()?
                .contains("/retired/specforge")
        );

        Ok(())
    }

    // --- ISF adapter ---

    // Self-contained ISF spec: a renderable ISF needs >=1 signal plus
    // behavioral content. Built through the generic markdown -> IntentIR
    // pipeline so the ISF tests stay self-contained.
    const ISF_ADAPTER_TEST_SPEC: &str = concat!(
        "# ISF Spec\n",
        "Clock clk.\n\n",
        "Reset rst_n is asynchronous active low.\n\n",
        "Signal DATA_IN is input width 8.\n\n",
        "Signal GO is input width 1.\n\n",
        "Signal ACC is output width 8.\n\n",
        "Init ACC = 8'0.\n\n",
        "Block accumulate when GO: ACC <- DATA_IN.\n",
    );

    #[test]
    fn isf_adapter_emits_valid_s_expression_source() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir =
            build_intent_ir_from_markdown(tempdir.path(), "isf_spec.md", ISF_ADAPTER_TEST_SPEC)?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;

        let isf = artifact.isf.expect("ISF artifact must be populated");
        assert!(
            isf.is_renderable,
            "expected renderable ISF artifact: {:?}",
            isf.blocking_reasons
        );
        assert!(isf.blocking_reasons.is_empty());

        assert!(isf.signal_count > 0, "must carry signal inventory");
        assert!(
            isf.transaction_count > 0 || isf.rule_count > 0,
            "should have behavioral content"
        );

        let source = &isf.source_text;
        assert!(source.starts_with("(actor"), "must start with actor form");
        assert!(source.ends_with(")"), "must end with closing paren");
        assert!(source.contains("(clock "), "must declare clock");
        assert!(source.contains("(interface"), "must declare interface");
        assert!(
            source.contains("(input ") || source.contains("(output "),
            "must have signal directions"
        );
        assert!(
            source.contains("(transaction ") || source.contains("(rule "),
            "must have behavioral content"
        );

        assert!(artifact.artifact_layout.emitted_target_path.is_some());
        assert!(
            artifact
                .artifact_layout
                .emitted_target_path
                .unwrap()
                .to_string_lossy()
                .ends_with(".isf")
        );

        Ok(())
    }

    #[test]
    fn adapter_write_reconciles_obsolete_isf_files() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_reconciliation.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;
        artifact.write_to_disk()?;

        let artifact_root = &artifact.artifact_layout.artifact_root;
        let current_isf = artifact
            .artifact_layout
            .emitted_target_path
            .as_ref()
            .expect("renderable adapter has an emitted path");
        let obsolete_isf = artifact_root.join("obsolete_actor.isf");
        let unrelated_file = artifact_root.join("review-notes.txt");
        let unrelated_directory = artifact_root.join("fixture.isf");
        fs::write(&obsolete_isf, "(actor obsolete_actor)")?;
        fs::write(&unrelated_file, "preserve me")?;
        fs::create_dir(&unrelated_directory)?;

        #[cfg(unix)]
        let obsolete_symlink = {
            use std::os::unix::fs::symlink;

            let path = artifact_root.join("obsolete_link.isf");
            symlink(&unrelated_file, &path)?;
            Some(path)
        };
        #[cfg(not(unix))]
        let obsolete_symlink: Option<std::path::PathBuf> = None;

        artifact.write_to_disk()?;

        assert!(
            current_isf.is_file(),
            "the manifest-selected ISF must remain"
        );
        assert!(
            !obsolete_isf.exists(),
            "obsolete regular ISF must be removed"
        );
        if let Some(path) = obsolete_symlink {
            assert!(
                fs::symlink_metadata(path).is_err(),
                "obsolete ISF symlink must be removed without following it"
            );
        }
        assert_eq!(fs::read_to_string(&unrelated_file)?, "preserve me");
        assert!(unrelated_directory.is_dir());

        let mut blocked = artifact.clone();
        blocked.lowering_status = AdapterLoweringStatus::Blocked;
        blocked.artifact_layout.emitted_target_path = None;
        let blocked_isf = blocked.isf.as_mut().expect("ISF metadata remains explicit");
        blocked_isf.is_renderable = false;
        blocked_isf
            .blocking_reasons
            .push("test-only blocked transition".to_string());
        blocked.write_to_disk()?;

        assert!(
            !current_isf.exists(),
            "a blocked manifest must leave no formerly emitted ISF"
        );
        assert_eq!(fs::read_to_string(unrelated_file)?, "preserve me");
        assert!(unrelated_directory.is_dir());

        Ok(())
    }

    // ISF-TEMPORAL-LOWERING.2.4 regression: every reported behavioral count
    // MUST equal what the emitter actually rendered — never a blind
    // IntentIR-derived guess that counts a surface the emitter ignores.
    #[test]
    fn isf_adapter_counts_equal_emitted_content() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir =
            build_intent_ir_from_markdown(tempdir.path(), "isf_count.md", ISF_ADAPTER_TEST_SPEC)?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;
        let isf = artifact.isf.expect("ISF artifact must be populated");
        let src = &isf.source_text;

        // Top-level transaction/rule forms render at a 2-space indent
        // (`\n  (transaction `/`\n  (rule `); nested clauses are deeper.
        let emitted_txns = src.matches("\n  (transaction ").count();
        let emitted_rules = src.matches("\n  (rule ").count();

        assert_eq!(
            isf.transaction_count, emitted_txns,
            "transaction_count must equal emitted (transaction …) forms\n{src}"
        );
        assert_eq!(
            isf.rule_count, emitted_rules,
            "rule_count must equal emitted (rule …) forms\n{src}"
        );
        Ok(())
    }

    #[test]
    fn isf_adapter_residualizes_every_protocol_record_without_changing_rendered_isf() -> Result<()>
    {
        let tempdir = tempdir()?;
        let mut intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_protocol_residuals.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;
        let baseline = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("baseline_adapter"),
        )?;
        assert!(
            baseline
                .residual_decisions
                .iter()
                .all(|packet| !packet.packet_id.starts_with("isf_protocol_"))
        );
        let baseline_isf = baseline.isf.expect("baseline ISF artifact");

        intent_ir.serial_frame_fields = vec![
            SerialFrameField {
                field_id: "serial_field_0002".to_string(),
                name: "OMEGA".to_string(),
                bit_width: Some(3),
                bit_range: Some((2, 0)),
                phase_name: Some("closing".to_string()),
                participant_drive: Some(ParticipantDriveRecord {
                    source_actor: "recipient".to_string(),
                    destination_actor: Some("initiator".to_string()),
                }),
                order: Some(1),
                response_values: vec!["accepted".to_string(), "rejected".to_string()],
                supporting_statement_ids: vec!["statement_closing".to_string()],
            },
            SerialFrameField {
                field_id: "serial_field_0001".to_string(),
                name: "ALPHA".to_string(),
                bit_width: Some(1),
                bit_range: Some((0, 0)),
                phase_name: Some("opening".to_string()),
                participant_drive: Some(ParticipantDriveRecord {
                    source_actor: "initiator".to_string(),
                    destination_actor: Some("recipient".to_string()),
                }),
                order: Some(0),
                response_values: Vec::new(),
                supporting_statement_ids: vec!["statement_opening".to_string()],
            },
        ];
        intent_ir.protocol_operations = vec![ProtocolOperationRecord {
            operation_id: "protocol_operation_0001".to_string(),
            branch_label: Some("deferred".to_string()),
            operation_name: Some("transfer".to_string()),
            phase_count: 2,
            phase_names: vec!["opening".to_string(), "closing".to_string()],
            supporting_statement_ids: vec!["statement_operation".to_string()],
        }];
        intent_ir.protocol_states = vec![ProtocolStateRecord {
            state_id: "protocol_state_0001".to_string(),
            machine_name: Some("serial machine".to_string()),
            state_name: "Reset".to_string(),
            action: None,
            supporting_statement_ids: vec!["statement_state".to_string()],
        }];
        intent_ir.interface_edge_timings = vec![InterfaceEdgeTimingRecord {
            timing_id: "interface_edge_timing_0001".to_string(),
            actor_name: "target".to_string(),
            signal_name: "DATA".to_string(),
            clock_signal: "CLK".to_string(),
            edge: InterfaceClockEdge::Rising,
            samples_on_edge: true,
            drive_changes_on_edge: true,
            supporting_statement_ids: vec!["statement_edge".to_string()],
        }];
        intent_ir.write_to_disk()?;

        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("protocol_adapter"),
        )?;
        assert_eq!(artifact.lowering_status, AdapterLoweringStatus::Renderable);
        let isf = artifact.isf.clone().expect("protocol ISF artifact");
        assert!(isf.is_renderable);
        assert_eq!(isf.source_text, baseline_isf.source_text);
        assert_eq!(isf.signal_count, baseline_isf.signal_count);
        assert_eq!(isf.transaction_count, baseline_isf.transaction_count);
        assert_eq!(isf.rule_count, baseline_isf.rule_count);

        let protocol_residuals = artifact
            .residual_decisions
            .iter()
            .filter(|packet| packet.packet_id.starts_with("isf_protocol_"))
            .collect::<Vec<_>>();
        assert_eq!(
            protocol_residuals
                .iter()
                .map(|packet| packet.packet_id.as_str())
                .collect::<Vec<_>>(),
            vec![
                "isf_protocol_serial_frame_field_serial_field_0002",
                "isf_protocol_serial_frame_field_serial_field_0001",
                "isf_protocol_operation_protocol_operation_0001",
                "isf_protocol_state_protocol_state_0001",
                "isf_protocol_interface_edge_timing_interface_edge_timing_0001",
            ]
        );
        for packet in &protocol_residuals {
            assert_eq!(packet.automation_confidence.as_str(), "low");
            assert_eq!(packet.candidate_interpretations.len(), 2);
            assert!(packet.why_unresolved.contains("remains exact in IntentIR"));
            assert!(packet.why_unresolved.contains("supporting statements:"));
        }
        assert!(
            protocol_residuals[0]
                .question
                .contains("serial-frame field")
        );
        assert!(
            protocol_residuals[0]
                .why_unresolved
                .contains("declared serial data wire")
        );
        assert!(
            protocol_residuals[2]
                .why_unresolved
                .contains("ordered field/step membership")
        );
        assert!(
            protocol_residuals[3]
                .why_unresolved
                .contains("typed transitions")
        );
        assert!(
            protocol_residuals[4]
                .why_unresolved
                .contains("sample destination")
        );

        let repeated = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            &tempdir.path().join("repeated_protocol_adapter"),
        )?;
        assert_eq!(artifact.residual_decisions, repeated.residual_decisions);

        artifact.write_to_disk()?;
        let reloaded =
            AdapterArtifact::load_from_path(&artifact.artifact_layout.adapter_artifact_path)?;
        assert_eq!(reloaded.residual_decisions, artifact.residual_decisions);

        let isf_path = tempdir.path().join("protocol_residuals.isf");
        fs::write(&isf_path, &isf.source_text)?;
        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|error| {
            panic!("fsmgen non-JSON: {error}\nstdout:{stdout}\nstderr:{stderr}")
        });
        assert_eq!(
            check["diagnostic_summary"]["success"].as_bool(),
            Some(true),
            "protocol residuals must not invalidate independently licensed ISF: {stdout}\n{stderr}"
        );
        assert_eq!(
            check["diagnostic_summary"]["diagnostic_count"].as_i64(),
            Some(0)
        );

        Ok(())
    }

    #[test]
    fn isf_adapter_blocks_when_no_signals() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(tempdir.path(), "empty.md", "# Empty\n")?;
        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;

        let isf = artifact.isf.expect("ISF artifact must be populated");
        assert!(!isf.is_renderable);
        assert!(!isf.blocking_reasons.is_empty());
        assert!(artifact.artifact_layout.emitted_target_path.is_none());

        Ok(())
    }

    #[test]
    fn isf_output_passes_fsmgen_strict_validation() -> Result<()> {
        let tempdir = tempdir()?;

        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_strict_spec.md",
            ISF_ADAPTER_TEST_SPEC,
        )?;

        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;

        let isf = artifact.isf.expect("ISF artifact must be populated");
        assert!(
            isf.is_renderable,
            "ISF must be renderable: {:?}",
            isf.blocking_reasons
        );

        let isf_path = tempdir.path().join("test_output.isf");
        fs::write(&isf_path, &isf.source_text)?;
        eprintln!("=== ISF source text ===\n{}\n=== END ===", isf.source_text);

        let output = crate::ir::run_fsmgen_strict_check(&isf_path);

        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        let check_result: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!(
                "fsmgen did not produce valid JSON.\nstdout: {}\nstderr: {}\nparse error: {}",
                stdout, stderr, e
            );
        });

        let success = check_result["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false);

        let diag_count = check_result["diagnostic_summary"]["diagnostic_count"]
            .as_i64()
            .unwrap_or(-1);

        if !success && let Some(diags) = check_result["diagnostics"].as_array() {
            for d in diags {
                eprintln!(
                    "FSMGen diagnostic: {}",
                    d.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("(no message)")
                );
            }
        }

        assert!(
            success,
            "FSMGen strict validation failed with {} diagnostics",
            diag_count
        );
        assert_eq!(
            diag_count, 0,
            "expected 0 FSMGen diagnostics, got {}",
            diag_count
        );

        Ok(())
    }

    // ISF-TEMPORAL-LOWERING.3 end-to-end regression: a temporal-rules-
    // bearing IntentIR produced through the generic markdown pipeline must
    // yield non-empty ISF temporal behavior, the artifact metric must match
    // the emitted content, and the emitted `.isf` must pass fsmgen strict.
    const ISF_TEMPORAL_E2E_SPEC: &str = concat!(
        "# Temporal Spec\n",
        "Clock clk.\n\n",
        "Reset rst_n is asynchronous active low.\n\n",
        "Signal PSEL is input width 1.\n\n",
        "Signal PREADY is output width 1.\n\n",
        "Signal CS_N is output width 1.\n\n",
        "PREADY must be asserted within 2 cycles.\n\n",
        "CS_N must be asserted.\n",
    );

    #[test]
    fn isf_temporal_rules_reach_isf_end_to_end() -> Result<()> {
        let tempdir = tempdir()?;
        let intent_ir = build_intent_ir_from_markdown(
            tempdir.path(),
            "isf_temporal_e2e.md",
            ISF_TEMPORAL_E2E_SPEC,
        )?;

        // The generic markdown pipeline must have recovered temporal rules
        // (this regression is meaningless otherwise).
        let loaded = IntentIr::load_from_path(&intent_ir.artifact_layout.intent_ir_path)?;
        assert!(
            !loaded.temporal_rules.is_empty(),
            "pipeline produced no temporal_rules — spec/regression is moot"
        );

        let artifact = AdapterArtifact::build(
            &intent_ir.artifact_layout.intent_ir_path,
            AdapterTarget::Isf,
            tempdir.path(),
        )?;
        let isf = artifact
            .isf
            .clone()
            .expect("ISF artifact must be populated");
        let src = &isf.source_text;
        eprintln!(
            "=== temporal_rules={} ===\n{src}\n=== residuals={} ===",
            loaded.temporal_rules.len(),
            artifact.residual_decisions.len()
        );

        // Non-empty ISF temporal behavior: at least one temporal-derived
        // construct OR an explicit residual decision — never silent loss.
        // The bounded-eventually now lowers to `(assert (monitor …))` (the
        // `(contract … eventually …)` clause was removed at pin 43b29f5c —
        // FSMGEN-ASSERT-MIGRATE).
        let has_assert_monitor = src.contains("\n    (assert (monitor ");
        let has_temporal_rule = src.contains("(rule temporal_");
        let has_temporal_residual = artifact
            .residual_decisions
            .iter()
            .any(|p| p.packet_id.starts_with("isf_temporal_unrepresentable_"));
        assert!(
            has_assert_monitor || has_temporal_rule || has_temporal_residual,
            "temporal_rules were silently dropped (no assert-monitor, no temporal rule, no residual)\n{src}"
        );

        // Metric == emitted content (ISF-TEMPORAL-LOWERING.2.4 invariant
        // holds in the temporal path too).
        assert_eq!(
            isf.transaction_count,
            src.matches("\n  (transaction ").count(),
            "transaction_count must equal emitted transactions\n{src}"
        );
        assert_eq!(
            isf.rule_count,
            src.matches("\n  (rule ").count(),
            "rule_count must equal emitted rules\n{src}"
        );

        // Emitted `.isf` still passes fsmgen strict.
        let isf_path = tempdir.path().join("temporal_e2e.isf");
        fs::write(&isf_path, src)?;
        let output = crate::ir::run_fsmgen_strict_check(&isf_path);
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let check: serde_json::Value = serde_json::from_str(&stdout).unwrap_or_else(|e| {
            panic!("fsmgen non-JSON.\nstdout:{stdout}\nstderr:{stderr}\nerr:{e}")
        });
        let success = check["diagnostic_summary"]["success"]
            .as_bool()
            .unwrap_or(false);
        if !success && let Some(diags) = check["diagnostics"].as_array() {
            for d in diags {
                eprintln!(
                    "FSMGen diagnostic: {}",
                    d.get("message")
                        .and_then(|m| m.as_str())
                        .unwrap_or("(none)")
                );
            }
        }
        assert!(
            success,
            "emitted temporal `.isf` was rejected by fsmgen strict"
        );

        Ok(())
    }
}
