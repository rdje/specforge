use std::cell::Cell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

use crate::cli::ValidateArgs;
use crate::error::{AppError, Result};
use crate::ir::IrStage;
use crate::ir::evidence::{
    EvidenceIr, SignalSemanticConflictRecord, SignalSemanticHintRecord,
    SignalSemanticHintSourceKind, StatementClass, VisualEvidenceRole, VisualObservationKind,
};
use crate::ir::intent::IntentIr;
use crate::ir::prior_memory::{
    CorpusMemory, NegativeKnowledgeKind, ProtocolFamily,
    interface_signal_conflict_negative_knowledge_pattern,
    residual_decision_negative_knowledge_pattern,
    signal_connectivity_conflict_negative_knowledge_pattern,
    signal_polarity_conflict_negative_knowledge_pattern,
    signal_semantic_conflict_negative_knowledge_pattern,
    temporal_value_conflict_negative_knowledge_pattern,
};
use crate::ir::semantic::{
    ActorPortRecord, ActorRelativeDirection, ClockEdge, InfrastructureSignalDistributionStatus,
    InfrastructureSignalKind, InfrastructureSignalRecord, InfrastructureSignalSourceStatus,
    InfrastructureTopologyKind, InterfaceSignalConflictRecord, SemanticIr, SignalConnectivityClass,
    SignalConnectivityConflictRecord, TemporalConflictRecord,
};
use crate::ir::source::{
    ActorSignalRelation, AutomationConfidence, DiagramKind, ResidualDecisionPacket, SourceIr,
    ValidationFindingRecord, ValidationFindingSeverity, ValidationMetricRecord,
    ValidationReportRecord, WidthHint,
};

thread_local! {
    static VALIDATION_OUTPUT_SUPPRESSED: Cell<bool> = const { Cell::new(false) };
}

const SEMANTIC_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_role_arbitration_surface_rescan_guidance";
const INTENT_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_role_arbitration_surface_rescan_guidance";
const SEMANTIC_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_role_consensus_surface_rescan_guidance";
const INTENT_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_role_consensus_surface_rescan_guidance";
const SEMANTIC_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_alias_dependent_semantic_consensus_surface_rescan_guidance";
const INTENT_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_alias_dependent_semantic_consensus_surface_rescan_guidance";
const SEMANTIC_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_prior_guided_semantic_consensus_surface_rescan_guidance";
const INTENT_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_prior_guided_semantic_consensus_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_cycle_window_surface_rescan_guidance";
const INTENT_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_cycle_window_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_clock_grounding_surface_rescan_guidance";
const INTENT_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_clock_grounding_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_actor_grounding_surface_rescan_guidance";
const INTENT_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_actor_grounding_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_conflict_surface_rescan_guidance";
const INTENT_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_conflict_surface_rescan_guidance";
const SEMANTIC_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_graph_direction_coverage_surface_rescan_guidance";
const INTENT_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_graph_direction_coverage_surface_rescan_guidance";
const SEMANTIC_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_actor_port_gap_surface_rescan_guidance";
const INTENT_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_actor_port_gap_surface_rescan_guidance";
const SEMANTIC_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_graph_direction_conflict_surface_rescan_guidance";
const INTENT_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_graph_direction_conflict_surface_rescan_guidance";
const SEMANTIC_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_connectivity_missing_producer_surface_rescan_guidance";
const INTENT_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_connectivity_missing_producer_surface_rescan_guidance";
const SEMANTIC_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_connectivity_missing_consumer_surface_rescan_guidance";
const INTENT_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_connectivity_missing_consumer_surface_rescan_guidance";
const SEMANTIC_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_interface_signal_conflict_surface_rescan_guidance";
const INTENT_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_interface_signal_conflict_surface_rescan_guidance";
const SEMANTIC_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_signal_connectivity_conflict_surface_rescan_guidance";
const INTENT_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_signal_connectivity_conflict_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_HANDSHAKE_COMPLETION_GAP_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_handshake_completion_gap_surface_rescan_guidance";
const INTENT_TEMPORAL_HANDSHAKE_COMPLETION_GAP_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_handshake_completion_gap_surface_rescan_guidance";
const SEMANTIC_TEMPORAL_MULTI_PREDICATE_ANTECEDENTS_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_temporal_multi_predicate_antecedents_surface_rescan_guidance";
const INTENT_TEMPORAL_MULTI_PREDICATE_ANTECEDENTS_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_temporal_multi_predicate_antecedents_surface_rescan_guidance";
const SEMANTIC_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_signal_polarity_conflict_surface_rescan_guidance";
const INTENT_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_signal_polarity_conflict_surface_rescan_guidance";
const EVIDENCE_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_signal_polarity_conflict_surface_rescan_guidance";
const EVIDENCE_MISSING_VLM_OBSERVATIONS_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_missing_vlm_observations_surface_rescan_guidance";
const EVIDENCE_STRUCTURAL_KG_MISSING_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_structural_kg_missing_surface_rescan_guidance";
const EVIDENCE_NORMATIVE_RESIDUAL_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_normative_residual_surface_rescan_guidance";
const EVIDENCE_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "evidence_signal_semantic_conflict_surface_rescan_guidance";
const SOURCE_VLM_ENRICHMENT_MISSING_SURFACE_RESCAN_GUIDANCE: &str =
    "source_vlm_enrichment_missing_surface_rescan_guidance";
const SEMANTIC_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "semantic_signal_semantic_conflict_surface_rescan_guidance";
const INTENT_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE: &str =
    "intent_signal_semantic_conflict_surface_rescan_guidance";

macro_rules! println {
    () => {
        if !validation_output_suppressed() {
            std::println!();
        }
    };
    ($($arg:tt)*) => {
        if !validation_output_suppressed() {
            std::println!($($arg)*);
        }
    };
}

pub fn run(args: ValidateArgs) -> Result<()> {
    // Auto-detect stage from artifact JSON `stage` field.
    let raw = fs::read_to_string(&args.artifact)
        .map_err(|_| AppError::MissingPath(args.artifact.clone()))?;

    #[derive(serde::Deserialize)]
    struct StageProbe {
        stage: IrStage,
    }
    let probe: StageProbe = serde_json::from_str(&raw).map_err(|e| {
        AppError::InvalidStageArtifact(format!(
            "cannot determine stage from {}: {e}",
            args.artifact.display()
        ))
    })?;

    match probe.stage {
        IrStage::SourceIr => {
            let mut ir = SourceIr::load_from_path(&args.artifact)?;
            let report = validate_source_ir(&ir, source_ir_fingerprint(&ir)?);
            persist_source_validation(&mut ir, &args.artifact, &report)?;
            print_validation_backannotation(&args.artifact, &report)?;
        }
        IrStage::EvidenceIr => {
            let mut ir = EvidenceIr::load_from_path(&args.artifact)?;
            let report = validate_evidence_ir(&ir, evidence_ir_fingerprint(&ir)?);
            persist_evidence_validation(&mut ir, &args.artifact, &report)?;
            print_validation_backannotation(&args.artifact, &report)?;
        }
        IrStage::SemanticIr => {
            let mut ir = SemanticIr::load_from_path(&args.artifact)?;
            let report = validate_semantic_ir(&ir, semantic_ir_fingerprint(&ir)?);
            persist_semantic_validation(&mut ir, &args.artifact, &report)?;
            print_validation_backannotation(&args.artifact, &report)?;
        }
        IrStage::IntentIr => {
            let mut ir = IntentIr::load_from_path(&args.artifact)?;
            let report = validate_intent_ir(&ir, intent_ir_fingerprint(&ir)?);
            persist_intent_validation(&mut ir, &args.artifact, &report)?;
            print_validation_backannotation(&args.artifact, &report)?;
        }
    }

    Ok(())
}

pub(crate) fn run_quiet(args: ValidateArgs) -> Result<()> {
    with_suppressed_validation_output(|| run(args))
}

fn validation_output_suppressed() -> bool {
    VALIDATION_OUTPUT_SUPPRESSED.with(Cell::get)
}

fn with_suppressed_validation_output<T>(operation: impl FnOnce() -> T) -> T {
    let previous = VALIDATION_OUTPUT_SUPPRESSED.with(|flag| flag.replace(true));
    let _guard = ValidationOutputSuppressionGuard { previous };
    operation()
}

struct ValidationOutputSuppressionGuard {
    previous: bool,
}

impl Drop for ValidationOutputSuppressionGuard {
    fn drop(&mut self) {
        VALIDATION_OUTPUT_SUPPRESSED.with(|flag| flag.set(self.previous));
    }
}

fn stable_fingerprint(text: &str) -> String {
    let mut hash = 0xcbf29ce484222325_u64;
    for byte in text.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    format!("{hash:016x}")
}

fn source_ir_fingerprint(ir: &SourceIr) -> Result<String> {
    let mut clone = ir.clone();
    clone.validation_reports.clear();
    Ok(stable_fingerprint(&clone.to_pretty_json()?))
}

fn evidence_ir_fingerprint(ir: &EvidenceIr) -> Result<String> {
    let mut clone = ir.clone();
    clone.validation_reports.clear();
    Ok(stable_fingerprint(&clone.to_pretty_json()?))
}

fn semantic_ir_fingerprint(ir: &SemanticIr) -> Result<String> {
    let mut clone = ir.clone();
    clone.validation_reports.clear();
    Ok(stable_fingerprint(&clone.to_pretty_json()?))
}

fn intent_ir_fingerprint(ir: &IntentIr) -> Result<String> {
    let mut clone = ir.clone();
    clone.validation_reports.clear();
    Ok(stable_fingerprint(&clone.to_pretty_json()?))
}

fn validation_report_path_for(artifact_path: &Path) -> Result<PathBuf> {
    let artifact_dir = artifact_path.parent().ok_or_else(|| {
        AppError::InvalidStageArtifact(format!(
            "cannot derive validation report path for {}",
            artifact_path.display()
        ))
    })?;
    Ok(artifact_dir.join("validation_report.json"))
}

fn metric(name: &str, value: impl Into<String>) -> ValidationMetricRecord {
    ValidationMetricRecord {
        name: name.to_string(),
        value: value.into(),
    }
}

fn finding(
    finding_id: &str,
    severity: ValidationFindingSeverity,
    category: &str,
    summary: impl Into<String>,
    related_ids: Vec<String>,
) -> ValidationFindingRecord {
    ValidationFindingRecord {
        finding_id: finding_id.to_string(),
        severity,
        category: category.to_string(),
        summary: summary.into(),
        related_ids,
    }
}

fn push_negative_knowledge_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} negative-knowledge prior match(es) in {stage_label} should be routed to targeted rescans and require stronger local corroboration before canonical promotion; prior memory did not suppress or rewrite current evidence",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_temporal_rule_surface_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} temporal-source id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because local timing/constraint evidence exists but no typed temporal rules were materialized",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_temporal_handshake_completion_gap_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} handshake-role signal id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because handshake semantic roles exist but no typed temporal rule currently expresses a HandshakeComplete predicate",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_temporal_multi_predicate_antecedents_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} temporal rule id(s) in {stage_label} carry multi-predicate antecedents; compound condition extraction may benefit from further review",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_semantic_role_arbitration_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} semantic-role arbitration signal id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because competing local role evidence is still non-decisive",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_semantic_role_consensus_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} fallback-only semantic-role signal id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because resolved role meaning still lacks observation-backed consensus",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_alias_dependent_semantic_consensus_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} alias-dependent semantic-role signal id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because resolved role meaning still depends only on alias-grounded evidence",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_prior_guided_semantic_consensus_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} prior-guided semantic-role signal id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because resolved role meaning was strengthened by learned modality-reliability priors",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_temporal_cycle_window_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} typed temporal rule id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because the current rules still lack explicit cycle-window bounds",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_temporal_clock_grounding_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} typed temporal rule id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because the current rules still lack explicit clock or edge grounding",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_temporal_actor_grounding_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} typed temporal rule id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because the current rules still lack actor-relative drive/sample grounding",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_temporal_conflict_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} temporal conflict id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because the current typed timing surface still carries contradictory value obligations",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_graph_direction_coverage_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} signal id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because the current canonical surface still lacks actor-relative graph direction coverage",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_graph_direction_conflict_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} graph-direction conflict id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because the current actor-relative graph still carries unresolved same-actor direction disagreement",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_connectivity_gap_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    gap_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} signal id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because the current connectivity graph still lacks resolved {gap_label} connectivity evidence",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_interface_signal_conflict_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} interface conflict id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because the current canonical interface surface still carries unresolved direction/width disagreement",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_signal_connectivity_conflict_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} connectivity conflict id(s) in {stage_label} should trigger targeted NLP rescans plus downstream rebuild because the current structural graph still carries unresolved producer ambiguity",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_actor_port_gap_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{stage_label} still carries actor-signal relation ids without actor-relative ports; this should trigger targeted NLP rescans plus bounded local replay to see whether those same relation ids collapse into actor-port direction records"
        ),
        related_ids.to_vec(),
    ));
}

fn push_signal_polarity_conflict_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{} polarity conflict id(s) in {stage_label} should trigger targeted NLP rescans plus bounded local replay because the current polarity surface still carries unresolved active-level disagreement",
            related_ids.len()
        ),
        related_ids.to_vec(),
    ));
}

fn push_signal_semantic_conflict_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{stage_label} still carries unresolved semantic-role conflict ids; this should trigger targeted NLP rescans plus bounded local replay to see whether those same conflict ids collapse toward one locally corroborated role meaning"
        ),
        related_ids.to_vec(),
    ));
}

fn push_normative_residual_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{stage_label} still carries partially structured normative statement ids; this should trigger targeted NLP rescans plus bounded local replay to see whether those same statement ids collapse into typed constraints, rules, or structured evidence"
        ),
        related_ids.to_vec(),
    ));
}

fn push_structural_kg_missing_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{stage_label} still lacks actor-signal graph grounding for behavioral evidence ids; this should trigger targeted NLP rescans plus bounded local replay to see whether those same ids collapse into actor-grounded graph relations"
        ),
        related_ids.to_vec(),
    ));
}

fn push_missing_vlm_observations_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{stage_label} still carries visual evidence ids without timing/state observations; this should trigger targeted local visual enrichment plus bounded replay to see whether those same ids gain extracted timing/state observations"
        ),
        related_ids.to_vec(),
    ));
}

fn push_source_vlm_enrichment_missing_rescan_guidance(
    findings: &mut Vec<ValidationFindingRecord>,
    finding_id: &str,
    stage_label: &str,
    related_ids: &[String],
) {
    if related_ids.is_empty() {
        return;
    }

    findings.push(finding(
        finding_id,
        ValidationFindingSeverity::Info,
        "rescan_guidance",
        format!(
            "{stage_label} still carries timing/state diagram asset ids without VLM enrichment; this should trigger targeted local visual enrichment plus bounded replay to see whether those same asset ids gain extracted observations"
        ),
        related_ids.to_vec(),
    ));
}

fn evidence_structural_kg_missing_related_ids(ir: &EvidenceIr) -> Vec<String> {
    ir.signal_constraints
        .iter()
        .map(|constraint| constraint.constraint_id.clone())
        .chain(ir.conditional_rules.iter().map(|rule| rule.rule_id.clone()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn source_vlm_enrichment_missing_related_ids(ir: &SourceIr) -> Vec<String> {
    ir.visual_assets
        .iter()
        .filter(|asset| {
            matches!(
                asset.diagram_kind,
                DiagramKind::TimingDiagram | DiagramKind::StateMachineDiagram
            )
        })
        .map(|asset| asset.asset_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn evidence_missing_vlm_observation_related_ids(ir: &EvidenceIr) -> Vec<String> {
    ir.visual_evidence
        .iter()
        .map(|item| item.evidence_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn evidence_normative_residual_statement_ids(ir: &EvidenceIr) -> Vec<String> {
    ir.extracted_statements
        .iter()
        .filter(|statement| matches!(statement.class, StatementClass::NormativeStatement))
        .map(|statement| statement.statement_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn actor_signal_relation_related_ids(relations: &[ActorSignalRelation]) -> Vec<String> {
    relations
        .iter()
        .map(|relation| relation.relation_id.clone())
        .filter(|relation_id| !relation_id.is_empty())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[derive(Clone, Copy)]
struct IntentQualityScoreInputs {
    dir_pct: usize,
    width_pct: usize,
    structured_nlp_constraints: usize,
    has_encoding_enums: bool,
    has_register_map: bool,
    has_timing_constraints: bool,
    has_state_machine: bool,
    has_system_contract: bool,
}

fn intent_quality_gap_related_ids(inputs: IntentQualityScoreInputs) -> Vec<String> {
    let mut related_ids = Vec::new();

    if inputs.dir_pct < 100 {
        related_ids.push("score_component:signal_direction".to_string());
    }
    if inputs.width_pct < 100 {
        related_ids.push("score_component:signal_width".to_string());
    }
    if inputs.structured_nlp_constraints < 30 {
        related_ids.push("score_component:nlp_constraints".to_string());
    }
    if !inputs.has_encoding_enums {
        related_ids.push("score_component:encoding_enums".to_string());
    }
    if !inputs.has_register_map {
        related_ids.push("score_component:register_map".to_string());
    }
    if !inputs.has_timing_constraints {
        related_ids.push("score_component:timing_constraints".to_string());
    }
    if !inputs.has_state_machine {
        related_ids.push("score_component:state_machine".to_string());
    }
    if !inputs.has_system_contract {
        related_ids.push("score_component:system_contract".to_string());
    }

    related_ids
}

fn load_prior_memory_for_validation(prior_memory_path: Option<&Path>) -> Option<CorpusMemory> {
    let prior_memory_path = prior_memory_path?;
    if !prior_memory_path.exists() {
        return None;
    }

    serde_json::from_str::<CorpusMemory>(&fs::read_to_string(prior_memory_path).ok()?).ok()
}

fn evidence_negative_knowledge_prior_matches(ir: &EvidenceIr) -> Vec<String> {
    let Some(corpus_memory) = load_prior_memory_for_validation(ir.prior_memory_path.as_deref())
    else {
        return Vec::new();
    };
    let protocol_family = ProtocolFamily::infer(
        &ir.document_identity.document_key,
        &ir.document_identity.display_name,
    );

    ir.signal_semantic_conflicts
        .iter()
        .filter_map(|conflict| {
            let pattern = signal_semantic_conflict_negative_knowledge_pattern(conflict)?;
            corpus_memory
                .negative_knowledge_pattern_is_known(
                    Some(protocol_family),
                    NegativeKnowledgeKind::SignalSemanticConflict,
                    &pattern,
                )
                .then(|| conflict.conflict_id.clone())
        })
        .chain(ir.signal_polarity_conflicts.iter().filter_map(|conflict| {
            let pattern = signal_polarity_conflict_negative_knowledge_pattern(conflict)?;
            corpus_memory
                .negative_knowledge_pattern_is_known(
                    Some(protocol_family),
                    NegativeKnowledgeKind::SignalPolarityConflict,
                    &pattern,
                )
                .then(|| conflict.conflict_id.clone())
        }))
        .collect()
}

fn push_negative_knowledge_match(
    matches: &mut Vec<String>,
    corpus_memory: &CorpusMemory,
    protocol_family: ProtocolFamily,
    knowledge_kind: NegativeKnowledgeKind,
    normalized_pattern: Option<String>,
    related_id: &str,
) {
    let Some(normalized_pattern) = normalized_pattern else {
        return;
    };
    if corpus_memory.negative_knowledge_pattern_is_known(
        Some(protocol_family),
        knowledge_kind,
        &normalized_pattern,
    ) {
        matches.push(related_id.to_string());
    }
}

struct CarriedNegativeKnowledgeSurfaces<'a> {
    signal_semantic_conflicts: &'a [SignalSemanticConflictRecord],
    temporal_conflicts: &'a [TemporalConflictRecord],
    signal_polarity_conflicts: &'a [crate::ir::evidence::SignalPolarityConflictRecord],
    interface_signal_conflicts: &'a [InterfaceSignalConflictRecord],
    signal_connectivity_conflicts: &'a [SignalConnectivityConflictRecord],
    residual_decisions: &'a [ResidualDecisionPacket],
}

fn negative_knowledge_prior_matches_for_carried_surfaces(
    corpus_memory: &CorpusMemory,
    protocol_family: ProtocolFamily,
    surfaces: CarriedNegativeKnowledgeSurfaces<'_>,
) -> Vec<String> {
    let mut matches = Vec::new();
    for conflict in surfaces.signal_semantic_conflicts {
        push_negative_knowledge_match(
            &mut matches,
            corpus_memory,
            protocol_family,
            NegativeKnowledgeKind::SignalSemanticConflict,
            signal_semantic_conflict_negative_knowledge_pattern(conflict),
            &conflict.conflict_id,
        );
    }
    for conflict in surfaces.temporal_conflicts {
        push_negative_knowledge_match(
            &mut matches,
            corpus_memory,
            protocol_family,
            NegativeKnowledgeKind::TemporalValueConflict,
            temporal_value_conflict_negative_knowledge_pattern(conflict),
            &conflict.conflict_id,
        );
    }
    for conflict in surfaces.signal_polarity_conflicts {
        push_negative_knowledge_match(
            &mut matches,
            corpus_memory,
            protocol_family,
            NegativeKnowledgeKind::SignalPolarityConflict,
            signal_polarity_conflict_negative_knowledge_pattern(conflict),
            &conflict.conflict_id,
        );
    }
    for conflict in surfaces.interface_signal_conflicts {
        push_negative_knowledge_match(
            &mut matches,
            corpus_memory,
            protocol_family,
            NegativeKnowledgeKind::InterfaceSignalConflict,
            interface_signal_conflict_negative_knowledge_pattern(conflict),
            &conflict.conflict_id,
        );
    }
    for conflict in surfaces.signal_connectivity_conflicts {
        push_negative_knowledge_match(
            &mut matches,
            corpus_memory,
            protocol_family,
            NegativeKnowledgeKind::SignalConnectivityConflict,
            signal_connectivity_conflict_negative_knowledge_pattern(conflict),
            &conflict.conflict_id,
        );
    }
    for residual in surfaces.residual_decisions {
        push_negative_knowledge_match(
            &mut matches,
            corpus_memory,
            protocol_family,
            NegativeKnowledgeKind::ResidualDecision,
            residual_decision_negative_knowledge_pattern(residual),
            &residual.packet_id,
        );
    }
    matches
}

fn semantic_negative_knowledge_prior_matches(ir: &SemanticIr) -> Vec<String> {
    let Some(evidence_ir) = EvidenceIr::load_from_path(&ir.evidence_ir_path).ok() else {
        return Vec::new();
    };
    let Some(corpus_memory) =
        load_prior_memory_for_validation(evidence_ir.prior_memory_path.as_deref())
    else {
        return Vec::new();
    };
    let protocol_family = ProtocolFamily::infer(
        &ir.document_identity.document_key,
        &ir.document_identity.display_name,
    );

    negative_knowledge_prior_matches_for_carried_surfaces(
        &corpus_memory,
        protocol_family,
        CarriedNegativeKnowledgeSurfaces {
            signal_semantic_conflicts: &ir.signal_semantic_conflicts,
            temporal_conflicts: &ir.temporal_conflicts,
            signal_polarity_conflicts: &ir.signal_polarity_conflicts,
            interface_signal_conflicts: &ir.interface_signal_conflicts,
            signal_connectivity_conflicts: &ir.signal_connectivity_conflicts,
            residual_decisions: &ir.residual_decisions,
        },
    )
}

fn intent_negative_knowledge_prior_matches(ir: &IntentIr) -> Vec<String> {
    let Some(semantic_ir) = SemanticIr::load_from_path(&ir.semantic_ir_path).ok() else {
        return Vec::new();
    };
    let Some(evidence_ir) = EvidenceIr::load_from_path(&semantic_ir.evidence_ir_path).ok() else {
        return Vec::new();
    };
    let Some(corpus_memory) =
        load_prior_memory_for_validation(evidence_ir.prior_memory_path.as_deref())
    else {
        return Vec::new();
    };
    let protocol_family = ProtocolFamily::infer(
        &ir.document_identity.document_key,
        &ir.document_identity.display_name,
    );

    negative_knowledge_prior_matches_for_carried_surfaces(
        &corpus_memory,
        protocol_family,
        CarriedNegativeKnowledgeSurfaces {
            signal_semantic_conflicts: &ir.signal_semantic_conflicts,
            temporal_conflicts: &ir.temporal_conflicts,
            signal_polarity_conflicts: &ir.signal_polarity_conflicts,
            interface_signal_conflicts: &ir.interface_signal_conflicts,
            signal_connectivity_conflicts: &ir.signal_connectivity_conflicts,
            residual_decisions: &ir.residual_decisions,
        },
    )
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct GraphDirectionConflictRecord {
    pub signal_name: String,
    pub actor_id: String,
    pub actor_name: String,
}

#[derive(Debug, Default, PartialEq, Eq)]
struct GraphDirectionCoverageSummary {
    resolved_signal_names: BTreeSet<String>,
    conflicted_signal_names: BTreeSet<String>,
    conflicts: BTreeSet<GraphDirectionConflictRecord>,
}

fn graph_direction_coverage_summary(
    actor_ports: &[ActorPortRecord],
) -> GraphDirectionCoverageSummary {
    let mut directions_by_signal_actor =
        BTreeMap::<String, BTreeMap<String, ActorRelativeDirection>>::new();
    let mut conflicted_signals = BTreeSet::new();
    let mut conflicts = BTreeSet::new();

    for port in actor_ports
        .iter()
        .filter(|port| !matches!(port.direction, ActorRelativeDirection::Unknown))
    {
        let actor_key = if port.actor_id.is_empty() {
            port.actor_name.clone()
        } else {
            port.actor_id.clone()
        };
        let actor_directions = directions_by_signal_actor
            .entry(port.signal_name.clone())
            .or_default();
        match actor_directions.get(&actor_key).copied() {
            None => {
                actor_directions.insert(actor_key, port.direction);
            }
            Some(existing) if existing == port.direction => {}
            Some(_) => {
                conflicted_signals.insert(port.signal_name.clone());
                conflicts.insert(GraphDirectionConflictRecord {
                    signal_name: port.signal_name.clone(),
                    actor_id: port.actor_id.clone(),
                    actor_name: port.actor_name.clone(),
                });
            }
        }
    }

    let resolved_signal_names = directions_by_signal_actor
        .into_iter()
        .filter_map(|(signal_name, actor_directions)| {
            if actor_directions.is_empty() || conflicted_signals.contains(&signal_name) {
                None
            } else {
                Some(signal_name)
            }
        })
        .collect();

    GraphDirectionCoverageSummary {
        resolved_signal_names,
        conflicted_signal_names: conflicted_signals,
        conflicts,
    }
}

pub(crate) fn graph_direction_signal_names(actor_ports: &[ActorPortRecord]) -> BTreeSet<String> {
    graph_direction_coverage_summary(actor_ports).resolved_signal_names
}

pub(crate) fn graph_direction_conflicted_signal_names(
    actor_ports: &[ActorPortRecord],
) -> BTreeSet<String> {
    graph_direction_coverage_summary(actor_ports).conflicted_signal_names
}

pub(crate) fn graph_direction_conflicts(
    actor_ports: &[ActorPortRecord],
) -> Vec<GraphDirectionConflictRecord> {
    graph_direction_coverage_summary(actor_ports)
        .conflicts
        .into_iter()
        .collect()
}

fn graph_direction_conflict_related_id(conflict: &GraphDirectionConflictRecord) -> String {
    let actor_key = if conflict.actor_id.is_empty() {
        conflict.actor_name.as_str()
    } else {
        conflict.actor_id.as_str()
    };
    format!(
        "graph_direction_conflict:{actor_key}:{}",
        conflict.signal_name
    )
}

fn missing_graph_direction_signal_names<'a>(
    signals: impl IntoIterator<Item = &'a crate::ir::semantic::InterfaceSignalRecord>,
    graph_direction_signal_names: &BTreeSet<String>,
) -> Vec<String> {
    signals
        .into_iter()
        .map(|signal| signal.signal_name.clone())
        .filter(|signal_name| !graph_direction_signal_names.contains(signal_name))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn graph_backed_missing_compat_direction_signal_names<'a>(
    signals: impl IntoIterator<Item = &'a crate::ir::semantic::InterfaceSignalRecord>,
    graph_direction_signal_names: &BTreeSet<String>,
) -> Vec<String> {
    signals
        .into_iter()
        .filter(|signal| {
            signal.direction_hint.is_none()
                && graph_direction_signal_names.contains(&signal.signal_name)
        })
        .map(|signal| signal.signal_name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn unresolved_missing_compat_direction_signal_names<'a>(
    signals: impl IntoIterator<Item = &'a crate::ir::semantic::InterfaceSignalRecord>,
    graph_direction_signal_names: &BTreeSet<String>,
) -> Vec<String> {
    signals
        .into_iter()
        .filter(|signal| {
            signal.direction_hint.is_none()
                && !graph_direction_signal_names.contains(&signal.signal_name)
        })
        .map(|signal| signal.signal_name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn is_infrastructure_connectivity_class(class: SignalConnectivityClass) -> bool {
    !matches!(class, SignalConnectivityClass::Protocol)
}

fn protocol_missing_producer_signal_names(
    connectivity: &[crate::ir::semantic::SignalConnectivityRecord],
) -> Vec<String> {
    connectivity
        .iter()
        .filter(|record| {
            record.producer_actor_ids.is_empty()
                && !is_infrastructure_connectivity_class(record.connectivity_class)
        })
        .map(|record| record.signal_name.clone())
        .collect()
}

fn infrastructure_missing_producer_signal_names(
    connectivity: &[crate::ir::semantic::SignalConnectivityRecord],
) -> Vec<String> {
    connectivity
        .iter()
        .filter(|record| {
            record.producer_actor_ids.is_empty()
                && is_infrastructure_connectivity_class(record.connectivity_class)
        })
        .map(|record| record.signal_name.clone())
        .collect()
}

fn infrastructure_signal_connectivity_count(
    connectivity: &[crate::ir::semantic::SignalConnectivityRecord],
) -> usize {
    connectivity
        .iter()
        .filter(|record| is_infrastructure_connectivity_class(record.connectivity_class))
        .count()
}

fn infrastructure_signals_with_source_status_count(
    infrastructure_signals: &[InfrastructureSignalRecord],
    status: InfrastructureSignalSourceStatus,
) -> usize {
    infrastructure_signals
        .iter()
        .filter(|record| record.source_status == status)
        .count()
}

fn infrastructure_signals_with_distribution_status_count(
    infrastructure_signals: &[InfrastructureSignalRecord],
    status: InfrastructureSignalDistributionStatus,
) -> usize {
    infrastructure_signals
        .iter()
        .filter(|record| record.distribution_status == status)
        .count()
}

fn infrastructure_topology_count(
    infrastructure_signals: &[InfrastructureSignalRecord],
    kind: Option<InfrastructureTopologyKind>,
) -> usize {
    infrastructure_signals
        .iter()
        .flat_map(|record| &record.infrastructure_topology)
        .filter(|topology| {
            kind.map(|expected| topology.topology_kind == expected)
                .unwrap_or(true)
        })
        .count()
}

fn describe_infrastructure_signal_kind(kind: InfrastructureSignalKind) -> &'static str {
    match kind {
        InfrastructureSignalKind::SystemClock => "system_clock",
        InfrastructureSignalKind::SystemReset => "system_reset",
    }
}

fn describe_infrastructure_topology_kind(kind: InfrastructureTopologyKind) -> &'static str {
    match kind {
        InfrastructureTopologyKind::ClockGatedBranch => "clock_gated_branch",
        InfrastructureTopologyKind::ResetSynchronizerStages => "reset_synchronizer_stages",
        InfrastructureTopologyKind::ResetTreeTargets => "reset_tree_targets",
    }
}

fn describe_infrastructure_source_status(status: InfrastructureSignalSourceStatus) -> &'static str {
    match status {
        InfrastructureSignalSourceStatus::UnresolvedSource => "unresolved_source",
        InfrastructureSignalSourceStatus::RecoveredProducer => "recovered_producer",
        InfrastructureSignalSourceStatus::MultipleRecoveredProducers => {
            "multiple_recovered_producers"
        }
    }
}

fn describe_infrastructure_distribution_status(
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

fn resolved_direction_counts<'a>(
    signal_records: impl IntoIterator<Item = &'a crate::ir::semantic::InterfaceSignalRecord>,
    graph_direction_signals: &BTreeSet<String>,
) -> (usize, usize, usize) {
    let mut resolved = 0usize;
    let mut graph = 0usize;
    let mut compat = 0usize;

    for signal in signal_records {
        let graph_has_direction = graph_direction_signals.contains(&signal.signal_name);
        let compat_has_direction = signal.direction_hint.is_some();
        if graph_has_direction || compat_has_direction {
            resolved += 1;
        }
        if graph_has_direction {
            graph += 1;
        }
        if compat_has_direction {
            compat += 1;
        }
    }

    (resolved, graph, compat)
}

fn backannotate_report(target: &mut Vec<ValidationReportRecord>, report: &ValidationReportRecord) {
    target.clear();
    target.push(report.clone());
}

fn temporal_rules_missing_clock_grounding_count(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
) -> usize {
    temporal_rules
        .iter()
        .filter(|rule| temporal_rule_missing_clock_grounding(rule))
        .count()
}

fn temporal_rule_missing_clock_grounding(rule: &crate::ir::semantic::TemporalRuleRecord) -> bool {
    rule.clock_signal.is_none() || matches!(rule.edge, ClockEdge::Unknown)
}

fn temporal_rule_has_actor_grounding(rule: &crate::ir::semantic::TemporalRuleRecord) -> bool {
    rule.antecedents
        .iter()
        .chain(rule.consequents.iter())
        .any(|predicate| {
            matches!(
                predicate,
                crate::ir::semantic::TemporalPredicateRecord::ActorDrivesSignal { .. }
                    | crate::ir::semantic::TemporalPredicateRecord::ActorMaintainsSignalStable { .. }
                    | crate::ir::semantic::TemporalPredicateRecord::ActorSamplesSignal { .. }
            )
        })
}

fn temporal_rule_ids_matching(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
    mut predicate: impl FnMut(&crate::ir::semantic::TemporalRuleRecord) -> bool,
) -> Vec<String> {
    temporal_rules
        .iter()
        .filter(|rule| predicate(rule))
        .map(|rule| rule.rule_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn temporal_rules_missing_clock_grounding_rule_ids(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
) -> Vec<String> {
    temporal_rule_ids_matching(temporal_rules, temporal_rule_missing_clock_grounding)
}

fn initial_regular_states_count(
    regular_states: &[crate::ir::semantic::RegularStateRecord],
) -> usize {
    regular_states
        .iter()
        .filter(|state| state.is_initial)
        .count()
}

fn state_machine_initial_cardinality_related_ids(
    regular_states: &[crate::ir::semantic::RegularStateRecord],
) -> Vec<String> {
    let initial_states = regular_states
        .iter()
        .filter(|state| state.is_initial)
        .map(|state| state.state_name.clone())
        .take(8)
        .collect::<Vec<_>>();
    if initial_states.is_empty() {
        regular_states
            .iter()
            .map(|state| state.state_name.clone())
            .take(8)
            .collect()
    } else {
        initial_states
    }
}

fn temporal_rules_with_cycle_window_count(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
) -> usize {
    temporal_rules
        .iter()
        .filter(|rule| rule.cycle_window.is_some())
        .count()
}

fn temporal_rules_missing_cycle_window_rule_ids(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
) -> Vec<String> {
    temporal_rule_ids_matching(temporal_rules, |rule| rule.cycle_window.is_none())
}

fn temporal_rules_with_actor_grounding_count(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
) -> usize {
    temporal_rules
        .iter()
        .filter(|rule| temporal_rule_has_actor_grounding(rule))
        .count()
}

fn temporal_rules_missing_actor_grounding_rule_ids(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
) -> Vec<String> {
    temporal_rule_ids_matching(temporal_rules, |rule| {
        !temporal_rule_has_actor_grounding(rule)
    })
}

fn temporal_rule_surface_input_ids(
    timing_constraints: &[crate::ir::source::TimingConstraintRecord],
    signal_constraints: &[crate::ir::source::SignalConstraintRecord],
    conditional_rules: &[crate::ir::source::ConditionalRuleRecord],
) -> Vec<String> {
    timing_constraints
        .iter()
        .map(|constraint| constraint.constraint_id.clone())
        .chain(
            signal_constraints
                .iter()
                .map(|constraint| constraint.constraint_id.clone()),
        )
        .chain(conditional_rules.iter().map(|rule| rule.rule_id.clone()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn temporal_rules_with_handshake_completion_count(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
) -> usize {
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

fn alias_dependent_semantic_consensus_signal_names(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
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

fn temporal_rules_with_alias_dependent_handshake_completion_count(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    let alias_dependent_signals = alias_dependent_semantic_consensus_signal_names(interfaces);
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

fn temporal_rules_with_alias_dependent_handshake_completion_signal_names(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> Vec<String> {
    let alias_dependent_signals = alias_dependent_semantic_consensus_signal_names(interfaces);
    if alias_dependent_signals.is_empty() {
        return Vec::new();
    }

    temporal_rules
        .iter()
        .flat_map(|rule| rule.antecedents.iter().chain(rule.consequents.iter()))
        .filter_map(|predicate| match predicate {
            crate::ir::semantic::TemporalPredicateRecord::HandshakeComplete {
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

fn interface_signals_with_handshake_semantic_role_names(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> Vec<String> {
    interface_signal_names_matching(interfaces, |signal| signal.resolved_semantic_role.is_some())
}

fn interface_signals_with_semantic_tags_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| !signal.semantic_tags.is_empty())
        .count()
}

fn interface_signal_semantic_observations_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .map(|signal| signal.semantic_observations.len())
        .sum()
}

fn interface_signals_with_resolved_polarity_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| signal.resolved_polarity.is_some())
        .count()
}

fn interface_signals_with_resolved_semantic_role_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| signal.resolved_semantic_role.is_some())
        .count()
}

fn interface_signal_semantic_candidates_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .map(|signal| signal.semantic_candidates.len())
        .sum()
}

fn interface_signals_with_semantic_candidates_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| !signal.semantic_candidates.is_empty())
        .count()
}

fn interface_signals_with_multiple_semantic_candidates_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| signal.semantic_candidates.len() > 1)
        .count()
}

fn interface_signals_with_semantic_arbitration_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| signal.semantic_arbitration.is_some())
        .count()
}

fn interface_signals_with_decisive_semantic_arbitration_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal
                .semantic_arbitration
                .as_ref()
                .is_some_and(|arbitration| arbitration.decisive)
        })
        .count()
}

fn interface_signals_with_non_decisive_semantic_arbitration_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal
                .semantic_arbitration
                .as_ref()
                .is_some_and(|arbitration| !arbitration.decisive)
        })
        .count()
}

fn interface_signal_names_matching(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
    mut predicate: impl FnMut(&crate::ir::semantic::InterfaceSignalRecord) -> bool,
) -> Vec<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| predicate(signal))
        .map(|signal| signal.signal_name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn interface_signals_with_non_decisive_semantic_arbitration(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> Vec<String> {
    interface_signal_names_matching(interfaces, |signal| {
        signal
            .semantic_arbitration
            .as_ref()
            .is_some_and(|arbitration| !arbitration.decisive)
    })
}

fn interface_signals_with_prior_guided_semantic_arbitration_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal
                .semantic_arbitration
                .as_ref()
                .is_some_and(|arbitration| {
                    arbitration.decisive
                        && matches!(
                        arbitration.decision_basis,
                        crate::ir::semantic::SemanticArbitrationDecisionBasis::PriorGuidedMargin
                    )
                })
        })
        .count()
}

fn interface_signals_with_prior_guided_semantic_arbitration(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> Vec<String> {
    interface_signal_names_matching(interfaces, |signal| {
        signal
            .semantic_arbitration
            .as_ref()
            .is_some_and(|arbitration| {
                arbitration.decisive
                    && matches!(
                        arbitration.decision_basis,
                        crate::ir::semantic::SemanticArbitrationDecisionBasis::PriorGuidedMargin
                    )
            })
    })
}

fn interface_signals_with_blocked_handshake_name_fallback(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> Vec<String> {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            handshake_name_heuristic_role(&signal.signal_name).is_some()
                && (signal
                    .semantic_arbitration
                    .as_ref()
                    .is_some_and(|arbitration| !arbitration.decisive)
                    || (signal.resolved_semantic_role.is_some()
                        && signal.semantic_consensus.is_none()))
        })
        .map(|signal| signal.signal_name.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

fn handshake_name_heuristic_role(signal_name: &str) -> Option<&'static str> {
    let normalized = signal_name.to_ascii_lowercase();
    if normalized.contains("valid") {
        Some("valid")
    } else if normalized.contains("ready") {
        Some("ready")
    } else {
        None
    }
}

fn interface_signals_with_semantic_grounding_strength_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
    grounding_strength: crate::ir::semantic::SemanticGroundingStrength,
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| signal.semantic_grounding_strength == Some(grounding_strength))
        .count()
}

fn interface_signals_with_semantic_consensus_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| signal.semantic_consensus.is_some())
        .count()
}

fn interface_signals_with_high_confidence_semantic_consensus_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal.semantic_consensus.as_ref().is_some_and(|consensus| {
                consensus.automation_confidence == AutomationConfidence::High
            })
        })
        .count()
}

fn interface_signals_with_alias_dependent_semantic_consensus_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal
                .semantic_consensus
                .as_ref()
                .is_some_and(|consensus| consensus.alias_dependent)
        })
        .count()
}

fn interface_signals_with_alias_dependent_semantic_consensus(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> Vec<String> {
    interface_signal_names_matching(interfaces, |signal| {
        signal
            .semantic_consensus
            .as_ref()
            .is_some_and(|consensus| consensus.alias_dependent)
    })
}

fn interface_signals_with_prior_guided_semantic_consensus_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal
                .semantic_consensus
                .as_ref()
                .is_some_and(|consensus| consensus.prior_guided)
        })
        .count()
}

fn interface_signals_with_prior_guided_semantic_consensus(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> Vec<String> {
    interface_signal_names_matching(interfaces, |signal| {
        signal
            .semantic_consensus
            .as_ref()
            .is_some_and(|consensus| consensus.prior_guided)
    })
}

fn interface_signal_alias_dependent_semantic_candidates_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .map(|signal| {
            signal
                .semantic_candidates
                .iter()
                .filter(|candidate| candidate.alias_dependent)
                .count()
        })
        .sum()
}

fn resolved_semantic_roles_without_consensus_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal.resolved_semantic_role.is_some() && signal.semantic_consensus.is_none()
        })
        .count()
}

fn resolved_semantic_roles_without_consensus_signal_names(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> Vec<String> {
    interface_signal_names_matching(interfaces, |signal| {
        signal.resolved_semantic_role.is_some() && signal.semantic_consensus.is_none()
    })
}

fn interface_signals_with_visual_semantic_grounding_count(
    interfaces: &[crate::ir::semantic::InterfaceRecord],
) -> usize {
    interfaces
        .iter()
        .flat_map(|interface| interface.signal_records.iter())
        .filter(|signal| {
            signal.semantic_observations.iter().any(|observation| {
                matches!(
                    observation.source_kind,
                    SignalSemanticHintSourceKind::VisualCaption
                        | SignalSemanticHintSourceKind::VlmTimingDiagramAnnotation
                )
            })
        })
        .count()
}

fn signal_semantic_hints_by_source_kind_count(
    hints: &[SignalSemanticHintRecord],
    source_kind: SignalSemanticHintSourceKind,
) -> usize {
    hints
        .iter()
        .filter(|hint| hint.source_kind == source_kind)
        .count()
}

fn temporal_rules_with_multi_predicate_antecedents_count(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
) -> usize {
    temporal_rules
        .iter()
        .filter(|rule| rule.antecedents.len() > 1)
        .count()
}

fn temporal_rules_with_multi_predicate_antecedents_rule_ids(
    temporal_rules: &[crate::ir::semantic::TemporalRuleRecord],
) -> Vec<String> {
    temporal_rules
        .iter()
        .filter(|rule| rule.antecedents.len() > 1)
        .map(|rule| rule.rule_id.clone())
        .collect()
}

fn describe_signal_polarity_conflict(
    conflict: &crate::ir::evidence::SignalPolarityConflictRecord,
) -> String {
    conflict
        .observations
        .iter()
        .map(|observation| {
            let mut refs = observation
                .supporting_statement_ids
                .iter()
                .chain(observation.supporting_table_ids.iter())
                .cloned()
                .collect::<Vec<_>>();
            refs.sort();
            refs.dedup();
            if refs.is_empty() {
                format!(
                    "{} via {}",
                    observation.polarity.as_str(),
                    observation.source_kind.as_str()
                )
            } else {
                format!(
                    "{} via {} ({})",
                    observation.polarity.as_str(),
                    observation.source_kind.as_str(),
                    refs.join(", ")
                )
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn describe_signal_semantic_conflict(
    conflict: &crate::ir::evidence::SignalSemanticConflictRecord,
) -> String {
    conflict
        .observations
        .iter()
        .map(|observation| {
            let mut refs = observation
                .supporting_statement_ids
                .iter()
                .chain(observation.supporting_table_ids.iter())
                .chain(observation.supporting_visual_evidence_ids.iter())
                .cloned()
                .collect::<Vec<_>>();
            refs.sort();
            refs.dedup();
            let tags = observation
                .semantic_tags
                .iter()
                .map(|tag| tag.as_str())
                .collect::<Vec<_>>()
                .join("|");
            if refs.is_empty() {
                format!("{tags} via {}", observation.source_kind.as_str())
            } else {
                format!(
                    "{tags} via {} ({})",
                    observation.source_kind.as_str(),
                    refs.join(", ")
                )
            }
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn describe_signal_connectivity_conflict(
    conflict: &crate::ir::semantic::SignalConnectivityConflictRecord,
) -> String {
    let actors = if conflict.conflicting_actor_names.is_empty() {
        conflict.conflicting_actor_ids.join(", ")
    } else {
        conflict.conflicting_actor_names.join(", ")
    };
    if actors.is_empty() {
        conflict.conflict_kind.as_str().to_string()
    } else {
        format!("{} via {}", conflict.conflict_kind.as_str(), actors)
    }
}

fn describe_interface_signal_conflict(
    conflict: &crate::ir::semantic::InterfaceSignalConflictRecord,
) -> String {
    let values = conflict
        .observations
        .iter()
        .map(|observation| {
            if observation.supporting_statement_ids.is_empty() {
                observation.value_text.clone()
            } else {
                format!(
                    "{} ({})",
                    observation.value_text,
                    observation.supporting_statement_ids.join(", ")
                )
            }
        })
        .collect::<Vec<_>>()
        .join("; ");
    if values.is_empty() {
        conflict.conflict_kind.as_str().to_string()
    } else {
        format!("{}: {}", conflict.conflict_kind.as_str(), values)
    }
}

fn write_validation_report_sidecar(
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    let report_path = validation_report_path_for(artifact_path)?;
    fs::write(report_path, serde_json::to_string_pretty(report)?)?;
    Ok(())
}

fn print_validation_findings(report: &ValidationReportRecord) {
    println!("=== Validation Findings ===");
    println!("  count: {}", report.findings.len());
    if report.findings.is_empty() {
        println!("  none");
    } else {
        for finding in &report.findings {
            println!(
                "  - [{}:{}] {}",
                finding.severity.as_str(),
                finding.category,
                finding.summary
            );
            if !finding.related_ids.is_empty() {
                println!("    related_ids: {}", finding.related_ids.join(", "));
            }
        }
    }
    println!();
}

fn print_validation_backannotation(
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    let report_path = validation_report_path_for(artifact_path)?;
    println!("=== Validation Backannotation ===");
    println!("  artifact_path: {}", artifact_path.display());
    println!("  validation_report_path: {}", report_path.display());
    println!("  artifact_fingerprint: {}", report.artifact_fingerprint);
    Ok(())
}

fn persist_source_validation(
    ir: &mut SourceIr,
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    backannotate_report(&mut ir.validation_reports, report);
    ir.write_to_disk()?;
    write_validation_report_sidecar(artifact_path, report)
}

fn persist_evidence_validation(
    ir: &mut EvidenceIr,
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    backannotate_report(&mut ir.validation_reports, report);
    ir.write_to_disk()?;
    write_validation_report_sidecar(artifact_path, report)
}

fn persist_semantic_validation(
    ir: &mut SemanticIr,
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    backannotate_report(&mut ir.validation_reports, report);
    ir.write_to_disk()?;
    write_validation_report_sidecar(artifact_path, report)
}

fn persist_intent_validation(
    ir: &mut IntentIr,
    artifact_path: &Path,
    report: &ValidationReportRecord,
) -> Result<()> {
    backannotate_report(&mut ir.validation_reports, report);
    ir.write_to_disk()?;
    write_validation_report_sidecar(artifact_path, report)
}

fn validate_source_ir(ir: &SourceIr, artifact_fingerprint: String) -> ValidationReportRecord {
    println!("command: validate");
    println!("stage: source_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    println!("=== Document Profile ===");
    println!("pages: {}", ir.page_artifacts.len());
    println!("visual_assets: {}", ir.visual_assets.len());
    println!("structured_tables: {}", ir.structured_tables.len());
    println!("content_elements: {}", ir.content_elements.len());
    println!("document_sections: {}", ir.document_sections.len());
    if let Some(profile) = &ir.document_profile
        && let Some(title) = &profile.title
    {
        println!("document_title: {title}");
    }
    println!();

    println!("=== Table Classification ===");
    let mut table_kinds: HashMap<&str, usize> = HashMap::new();
    for table in &ir.structured_tables {
        *table_kinds
            .entry(format!("{:?}", table.table_kind).leak())
            .or_insert(0) += 1;
    }
    for (kind, count) in sorted_by_value(&table_kinds) {
        println!("  {kind}: {count}");
    }
    println!();

    println!("=== Diagram Classification ===");
    let timing_count = ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::TimingDiagram))
        .count();
    let state_count = ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::StateMachineDiagram))
        .count();
    let block_count = ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::BlockDiagram))
        .count();
    let unknown_count = ir
        .visual_assets
        .iter()
        .filter(|a| matches!(a.diagram_kind, DiagramKind::Unknown))
        .count();
    println!("  timing_diagram: {timing_count}");
    println!("  state_machine_diagram: {state_count}");
    println!("  block_diagram: {block_count}");
    println!("  unknown: {unknown_count}");
    let classified = timing_count + state_count + block_count;
    let diagram_coverage = if ir.visual_assets.is_empty() {
        0.0
    } else {
        classified as f64 / ir.visual_assets.len() as f64 * 100.0
    };
    println!("  diagram_classification_coverage: {diagram_coverage:.0}%");
    println!();

    println!("=== VLM Readiness ===");
    let vlm_enriched = ir
        .visual_assets
        .iter()
        .filter(|a| {
            a.note.as_deref().map(|n| {
                n.starts_with("vlm_timing_diagram_extraction:")
                    || n.starts_with("vlm_state_machine_extraction:")
            }) == Some(true)
        })
        .count();
    let vlm_ready = timing_count + state_count;
    println!(
        "  figures_ready_for_vlm: {vlm_ready} (timing: {timing_count}, state_machine: {state_count})"
    );
    println!("  figures_already_enriched: {vlm_enriched}");
    if vlm_ready > 0 && vlm_enriched == 0 {
        println!(
            "  hint: run `specforge enrich <source-ir> --vlm-provider ollama` to enrich {vlm_ready} diagrams"
        );
    }
    println!();

    println!("=== Section Classification ===");
    let mut section_kinds: HashMap<&str, usize> = HashMap::new();
    for s in &ir.document_sections {
        *section_kinds
            .entry(format!("{:?}", s.section_kind).leak())
            .or_insert(0) += 1;
    }
    for (kind, count) in sorted_by_value(&section_kinds) {
        println!("  {kind}: {count}");
    }

    println!();
    println!("=== Residual Decisions ===");
    println!("  count: {}", ir.residual_decisions.len());

    let figures_ready_for_vlm = timing_count + state_count;
    let source_vlm_enrichment_missing_related_ids = source_vlm_enrichment_missing_related_ids(ir);
    let mut findings = Vec::new();
    if figures_ready_for_vlm > 0 && vlm_enriched == 0 {
        findings.push(finding(
            "source_vlm_enrichment_missing",
            ValidationFindingSeverity::Warning,
            "visual_enrichment",
            format!(
                "{figures_ready_for_vlm} classified timing/state diagrams are still missing VLM enrichment"
            ),
            source_vlm_enrichment_missing_related_ids.clone(),
        ));
        push_source_vlm_enrichment_missing_rescan_guidance(
            &mut findings,
            SOURCE_VLM_ENRICHMENT_MISSING_SURFACE_RESCAN_GUIDANCE,
            "SourceIR",
            &source_vlm_enrichment_missing_related_ids,
        );
    }
    if unknown_count > 0 {
        findings.push(finding(
            "source_unknown_diagrams_remaining",
            ValidationFindingSeverity::Info,
            "diagram_classification",
            format!("{unknown_count} visual assets remain unclassified"),
            ir.visual_assets
                .iter()
                .filter(|asset| matches!(asset.diagram_kind, DiagramKind::Unknown))
                .map(|asset| asset.asset_id.clone())
                .take(6)
                .collect(),
        ));
    }
    if !ir.residual_decisions.is_empty() {
        findings.push(finding(
            "source_residual_decisions_present",
            ValidationFindingSeverity::Warning,
            "residual_decisions",
            format!(
                "SourceIR still carries {} residual decision packet(s)",
                ir.residual_decisions.len()
            ),
            ir.residual_decisions
                .iter()
                .map(|packet| packet.packet_id.clone())
                .collect(),
        ));
    }

    let report = ValidationReportRecord {
        report_id: format!("validation_source_ir_{artifact_fingerprint}"),
        validated_stage: IrStage::SourceIr,
        artifact_fingerprint,
        summary: format!(
            "SourceIR validation for {} with {} finding(s)",
            ir.document_identity.display_name,
            findings.len()
        ),
        overall_score: None,
        grade: None,
        metrics: vec![
            metric("pages", ir.page_artifacts.len().to_string()),
            metric("visual_assets", ir.visual_assets.len().to_string()),
            metric("structured_tables", ir.structured_tables.len().to_string()),
            metric("content_elements", ir.content_elements.len().to_string()),
            metric("document_sections", ir.document_sections.len().to_string()),
            metric("timing_diagrams", timing_count.to_string()),
            metric("state_machine_diagrams", state_count.to_string()),
            metric("block_diagrams", block_count.to_string()),
            metric("unknown_diagrams", unknown_count.to_string()),
            metric(
                "diagram_classification_coverage_pct",
                format!("{diagram_coverage:.0}"),
            ),
            metric("figures_ready_for_vlm", figures_ready_for_vlm.to_string()),
            metric("figures_already_enriched", vlm_enriched.to_string()),
            metric(
                "residual_decisions",
                ir.residual_decisions.len().to_string(),
            ),
        ],
        findings,
    };
    print_validation_findings(&report);
    report
}

fn validate_evidence_ir(ir: &EvidenceIr, artifact_fingerprint: String) -> ValidationReportRecord {
    println!("command: validate");
    println!("stage: evidence_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    let signal_semantic_hints_from_tables = signal_semantic_hints_by_source_kind_count(
        &ir.signal_semantic_hints,
        SignalSemanticHintSourceKind::SignalDescriptionTable,
    );
    let signal_semantic_hints_from_prose = signal_semantic_hints_by_source_kind_count(
        &ir.signal_semantic_hints,
        SignalSemanticHintSourceKind::ProseStatement,
    );
    let signal_semantic_hints_from_alias_grounded_prose =
        signal_semantic_hints_by_source_kind_count(
            &ir.signal_semantic_hints,
            SignalSemanticHintSourceKind::AliasGroundedProseStatement,
        );
    let signal_semantic_hints_from_visual_captions = signal_semantic_hints_by_source_kind_count(
        &ir.signal_semantic_hints,
        SignalSemanticHintSourceKind::VisualCaption,
    );
    let signal_semantic_hints_from_vlm_timing_annotations =
        signal_semantic_hints_by_source_kind_count(
            &ir.signal_semantic_hints,
            SignalSemanticHintSourceKind::VlmTimingDiagramAnnotation,
        );

    println!("=== Statement Classification ===");
    let total = ir.extracted_statements.len();
    let mut classes: HashMap<&str, usize> = HashMap::new();
    for s in &ir.extracted_statements {
        let name = match s.class {
            StatementClass::SignalValueConstraint => "signal_value_constraint",
            StatementClass::TimingConstraint => "timing_constraint",
            StatementClass::ConditionalRule => "conditional_rule",
            StatementClass::NormativeStatement => "normative_statement",
            StatementClass::DerivedRule => "derived_rule",
            StatementClass::LocalDesignDecision => "local_design_decision",
            StatementClass::ExplicitAbstraction => "explicit_abstraction",
            StatementClass::SourceFact => "source_fact",
            StatementClass::Unknown => "unknown",
        };
        *classes.entry(name).or_insert(0) += 1;
    }
    println!("  total_statements: {total}");
    for (class, count) in sorted_by_value(&classes) {
        let pct = percentage_or_zero(*count, total);
        println!("  {class}: {count} ({pct}%)");
    }
    let non_fact = total - classes.get("source_fact").copied().unwrap_or(0);
    let nlp_coverage = percentage_or_zero(non_fact, total);
    println!("  nlp_coverage (non-source_fact): {nlp_coverage}%");
    println!();

    println!("=== Structured Extraction ===");
    println!(
        "  signal_constraints (Level 2 NLP): {}",
        ir.signal_constraints.len()
    );
    println!(
        "  conditional_rules (Level 2 NLP): {}",
        ir.conditional_rules.len()
    );
    println!(
        "  register_records (from tables): {}",
        ir.register_records.len()
    );
    println!(
        "  timing_constraints (from tables): {}",
        ir.timing_constraints.len()
    );
    println!(
        "  table_signal_declaration_provenance: {}",
        ir.table_signal_declaration_provenance.len()
    );
    println!(
        "  signal_polarity_conflicts: {}",
        ir.signal_polarity_conflicts.len()
    );
    println!(
        "  signal_semantic_hints: {}",
        ir.signal_semantic_hints.len()
    );
    println!(
        "  signal_semantic_conflicts: {}",
        ir.signal_semantic_conflicts.len()
    );
    println!("    from_signal_description_tables: {signal_semantic_hints_from_tables}");
    println!("    from_prose_statements: {signal_semantic_hints_from_prose}");
    println!("    from_alias_grounded_prose: {signal_semantic_hints_from_alias_grounded_prose}");
    println!("    from_visual_captions: {signal_semantic_hints_from_visual_captions}");
    println!(
        "    from_vlm_timing_annotations: {signal_semantic_hints_from_vlm_timing_annotations}"
    );
    println!();

    println!("=== Visual Observations ===");
    let mut classification_obs = 0usize;
    let mut timing_obs = 0usize;
    let mut state_obs = 0usize;
    for item in &ir.visual_evidence {
        for obs in &item.observations {
            match obs.kind {
                VisualObservationKind::Classification => classification_obs += 1,
                VisualObservationKind::TimingDiagramExtraction => timing_obs += 1,
                VisualObservationKind::StateMachineExtraction => state_obs += 1,
                _ => {}
            }
        }
    }
    println!("  classification_observations: {classification_obs}");
    println!("  timing_diagram_extractions: {timing_obs}");
    println!("  state_machine_extractions: {state_obs}");
    if timing_obs == 0 && state_obs == 0 {
        println!(
            "  hint: run `specforge enrich` then re-run `specforge evidence` to populate VLM observations"
        );
    }
    println!();

    println!("=== Visual Evidence ===");
    println!("  total: {}", ir.visual_evidence.len());
    let with_caption = ir
        .visual_evidence
        .iter()
        .filter(|e| e.caption_text.is_some())
        .count();
    let visual_evidence_normative = visual_evidence_role_count(ir, VisualEvidenceRole::Normative);
    let visual_evidence_explanatory =
        visual_evidence_role_count(ir, VisualEvidenceRole::Explanatory);
    let visual_evidence_illustrative =
        visual_evidence_role_count(ir, VisualEvidenceRole::Illustrative);
    let visual_evidence_ambiguous = visual_evidence_role_count(ir, VisualEvidenceRole::Ambiguous);
    let visual_evidence_unknown = visual_evidence_role_count(ir, VisualEvidenceRole::Unknown);
    let visual_motif_corroboration_targets = visual_motif_corroboration_targets(ir);
    println!("  with_caption: {with_caption}");
    println!("  normative: {visual_evidence_normative}");
    println!("  explanatory: {visual_evidence_explanatory}");
    println!("  illustrative: {visual_evidence_illustrative}");
    println!("  ambiguous: {visual_evidence_ambiguous}");
    println!("  unknown: {visual_evidence_unknown}");
    if !ir.signal_polarity_conflicts.is_empty() {
        println!();
        println!("=== Signal Polarity Conflicts ===");
        for conflict in ir.signal_polarity_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_signal_polarity_conflict(conflict)
            );
        }
        if ir.signal_polarity_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.signal_polarity_conflicts.len() - 8
            );
        }
    }
    if !ir.signal_semantic_conflicts.is_empty() {
        println!();
        println!("=== Signal Semantic Conflicts ===");
        for conflict in ir.signal_semantic_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_signal_semantic_conflict(conflict)
            );
        }
        if ir.signal_semantic_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.signal_semantic_conflicts.len() - 8
            );
        }
    }

    let negative_knowledge_prior_matches = evidence_negative_knowledge_prior_matches(ir);
    println!();
    println!("=== Negative Knowledge Priors ===");
    println!(
        "  matched_signal_semantic_conflict_patterns: {}",
        negative_knowledge_prior_matches.len()
    );
    println!(
        "  rescan_recommendations: {}",
        negative_knowledge_prior_matches.len()
    );
    println!(
        "  stronger_corroboration_requirements: {}",
        negative_knowledge_prior_matches.len()
    );

    let normative_count = classes.get("normative_statement").copied().unwrap_or(0);
    let missing_vlm_observation_related_ids = evidence_missing_vlm_observation_related_ids(ir);
    let structural_kg_missing_related_ids = evidence_structural_kg_missing_related_ids(ir);
    let normative_residual_statement_ids = evidence_normative_residual_statement_ids(ir);
    let mut findings = Vec::new();
    if total == 0 {
        findings.push(finding(
            "evidence_no_extracted_statements",
            ValidationFindingSeverity::Error,
            "statement_extraction",
            "EvidenceIR contains no extracted statements",
            Vec::new(),
        ));
    }
    if timing_obs == 0 && state_obs == 0 && !ir.visual_evidence.is_empty() {
        findings.push(finding(
            "evidence_missing_vlm_observations",
            ValidationFindingSeverity::Warning,
            "visual_enrichment",
            "Visual evidence is present, but no VLM timing/state observations were injected into EvidenceIR",
            missing_vlm_observation_related_ids.clone(),
        ));
        push_missing_vlm_observations_rescan_guidance(
            &mut findings,
            EVIDENCE_MISSING_VLM_OBSERVATIONS_SURFACE_RESCAN_GUIDANCE,
            "EvidenceIR",
            &missing_vlm_observation_related_ids,
        );
    }
    if ir.actor_signal_relations.is_empty()
        && (!ir.signal_constraints.is_empty() || !ir.conditional_rules.is_empty())
    {
        findings.push(finding(
            "evidence_structural_kg_missing",
            ValidationFindingSeverity::Warning,
            "knowledge_graph",
            "Behavioral evidence exists, but the structural actor-signal graph is still empty in EvidenceIR",
            structural_kg_missing_related_ids.clone(),
        ));
        push_structural_kg_missing_rescan_guidance(
            &mut findings,
            EVIDENCE_STRUCTURAL_KG_MISSING_SURFACE_RESCAN_GUIDANCE,
            "EvidenceIR",
            &structural_kg_missing_related_ids,
        );
    }
    if normative_count > 0 {
        findings.push(finding(
            "evidence_normative_residuals_remaining",
            ValidationFindingSeverity::Info,
            "nlp_residuals",
            format!(
                "{normative_count} normative statements remain only partially structured in EvidenceIR"
            ),
            normative_residual_statement_ids.clone(),
        ));
        push_normative_residual_rescan_guidance(
            &mut findings,
            EVIDENCE_NORMATIVE_RESIDUAL_SURFACE_RESCAN_GUIDANCE,
            "EvidenceIR",
            &normative_residual_statement_ids,
        );
    }
    if !ir.signal_polarity_conflicts.is_empty() {
        let signal_polarity_conflict_related_ids = ir
            .signal_polarity_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "evidence_signal_polarity_conflicts_present",
            ValidationFindingSeverity::Warning,
            "polarity_conflicts",
            format!(
                "{} signal polarity conflict(s) detected across polarity evidence; asserted/deasserted constraints stayed polarity-neutral rather than forcing a wrong refinement",
                ir.signal_polarity_conflicts.len()
            ),
            signal_polarity_conflict_related_ids.clone(),
        ));
        push_signal_polarity_conflict_rescan_guidance(
            &mut findings,
            EVIDENCE_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "EvidenceIR",
            &signal_polarity_conflict_related_ids,
        );
    }
    if !ir.signal_semantic_conflicts.is_empty() {
        let signal_semantic_conflict_related_ids = ir
            .signal_semantic_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "evidence_signal_semantic_conflicts_present",
            ValidationFindingSeverity::Warning,
            "semantic_role_conflicts",
            format!(
                "{} signal semantic conflict(s) detected; meaning-based role evidence currently assigns incompatible roles to the same signal",
                ir.signal_semantic_conflicts.len()
            ),
            signal_semantic_conflict_related_ids.clone(),
        ));
        push_signal_semantic_conflict_rescan_guidance(
            &mut findings,
            EVIDENCE_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "EvidenceIR",
            &signal_semantic_conflict_related_ids,
        );
    }
    if !negative_knowledge_prior_matches.is_empty() {
        findings.push(finding(
            "evidence_negative_knowledge_prior_matches",
            ValidationFindingSeverity::Info,
            "negative_knowledge",
            format!(
                "{} signal semantic conflict pattern(s) match prior negative knowledge; this is a caution signal only, not an override of current-document evidence",
                negative_knowledge_prior_matches.len()
            ),
            negative_knowledge_prior_matches.clone(),
        ));
    }
    push_negative_knowledge_rescan_guidance(
        &mut findings,
        "evidence_negative_knowledge_rescan_guidance",
        "EvidenceIR",
        &negative_knowledge_prior_matches,
    );
    if !visual_motif_corroboration_targets.is_empty() {
        findings.push(finding(
            "evidence_visual_motif_corroboration_guidance",
            ValidationFindingSeverity::Info,
            "rescan_guidance",
            format!(
                "{} prior-classified normative visual evidence item(s) should be routed to targeted VLM/multimodal corroboration before downstream promotion; prior memory did not rewrite SourceIR or create semantic facts",
                visual_motif_corroboration_targets.len()
            ),
            visual_motif_corroboration_targets.clone(),
        ));
    }

    let report = ValidationReportRecord {
        report_id: format!("validation_evidence_ir_{artifact_fingerprint}"),
        validated_stage: IrStage::EvidenceIr,
        artifact_fingerprint,
        summary: format!(
            "EvidenceIR validation for {} with {} finding(s)",
            ir.document_identity.display_name,
            findings.len()
        ),
        overall_score: None,
        grade: None,
        metrics: vec![
            metric("total_statements", total.to_string()),
            metric("nlp_coverage_pct", nlp_coverage.to_string()),
            metric(
                "signal_value_constraint_statements",
                classes
                    .get("signal_value_constraint")
                    .copied()
                    .unwrap_or(0)
                    .to_string(),
            ),
            metric(
                "conditional_rule_statements",
                classes
                    .get("conditional_rule")
                    .copied()
                    .unwrap_or(0)
                    .to_string(),
            ),
            metric("normative_statements", normative_count.to_string()),
            metric(
                "signal_constraints",
                ir.signal_constraints.len().to_string(),
            ),
            metric("conditional_rules", ir.conditional_rules.len().to_string()),
            metric(
                "actor_signal_relations",
                ir.actor_signal_relations.len().to_string(),
            ),
            metric("register_records", ir.register_records.len().to_string()),
            metric(
                "timing_constraints",
                ir.timing_constraints.len().to_string(),
            ),
            metric(
                "table_signal_declaration_provenance",
                ir.table_signal_declaration_provenance.len().to_string(),
            ),
            metric(
                "signal_polarity_conflicts",
                ir.signal_polarity_conflicts.len().to_string(),
            ),
            metric(
                "signal_semantic_hints",
                ir.signal_semantic_hints.len().to_string(),
            ),
            metric(
                "signal_semantic_conflicts",
                ir.signal_semantic_conflicts.len().to_string(),
            ),
            metric(
                "negative_knowledge_prior_matches",
                negative_knowledge_prior_matches.len().to_string(),
            ),
            metric(
                "negative_knowledge_rescan_recommendations",
                negative_knowledge_prior_matches.len().to_string(),
            ),
            metric(
                "negative_knowledge_corroboration_requirements",
                negative_knowledge_prior_matches.len().to_string(),
            ),
            metric(
                "signal_semantic_hints_from_tables",
                signal_semantic_hints_from_tables.to_string(),
            ),
            metric(
                "signal_semantic_hints_from_prose",
                signal_semantic_hints_from_prose.to_string(),
            ),
            metric(
                "signal_semantic_hints_from_alias_grounded_prose",
                signal_semantic_hints_from_alias_grounded_prose.to_string(),
            ),
            metric(
                "signal_semantic_hints_from_visual_captions",
                signal_semantic_hints_from_visual_captions.to_string(),
            ),
            metric(
                "signal_semantic_hints_from_vlm_timing_annotations",
                signal_semantic_hints_from_vlm_timing_annotations.to_string(),
            ),
            metric(
                "visual_classification_observations",
                classification_obs.to_string(),
            ),
            metric("timing_diagram_extractions", timing_obs.to_string()),
            metric("state_machine_extractions", state_obs.to_string()),
            metric(
                "visual_evidence_total",
                ir.visual_evidence.len().to_string(),
            ),
            metric("visual_evidence_with_caption", with_caption.to_string()),
            metric(
                "visual_evidence_normative",
                visual_evidence_normative.to_string(),
            ),
            metric(
                "visual_evidence_explanatory",
                visual_evidence_explanatory.to_string(),
            ),
            metric(
                "visual_evidence_illustrative",
                visual_evidence_illustrative.to_string(),
            ),
            metric(
                "visual_evidence_ambiguous",
                visual_evidence_ambiguous.to_string(),
            ),
            metric(
                "visual_evidence_unknown",
                visual_evidence_unknown.to_string(),
            ),
            metric(
                "visual_motif_corroboration_targets",
                visual_motif_corroboration_targets.len().to_string(),
            ),
        ],
        findings,
    };
    print_validation_findings(&report);
    report
}

fn visual_evidence_role_count(ir: &EvidenceIr, role: VisualEvidenceRole) -> usize {
    ir.visual_evidence
        .iter()
        .filter(|item| item.role == role)
        .count()
}

fn visual_motif_corroboration_targets(ir: &EvidenceIr) -> Vec<String> {
    ir.visual_evidence
        .iter()
        .filter(|item| item.role == VisualEvidenceRole::Normative)
        .filter(|item| {
            item.observations.iter().any(|observation| {
                observation.kind == VisualObservationKind::Classification
                    && observation.created_by == "specforge_prior_memory"
            })
        })
        .filter(|item| {
            !item.observations.iter().any(|observation| {
                matches!(
                    observation.kind,
                    VisualObservationKind::TimingDiagramExtraction
                        | VisualObservationKind::StateMachineExtraction
                )
            })
        })
        .map(|item| item.evidence_id.clone())
        .collect()
}

fn validate_semantic_ir(ir: &SemanticIr, artifact_fingerprint: String) -> ValidationReportRecord {
    println!("command: validate");
    println!("stage: semantic_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    println!("=== Interface / Signal Coverage ===");
    let total_signals: usize = ir.interfaces.iter().map(|i| i.signal_records.len()).sum();
    let graph_direction_summary = graph_direction_coverage_summary(&ir.actor_ports);
    let graph_direction_signals = &graph_direction_summary.resolved_signal_names;
    let graph_direction_conflicts = &graph_direction_summary.conflicted_signal_names;
    let missing_graph_direction_signal_names = missing_graph_direction_signal_names(
        ir.interfaces.iter().flat_map(|i| i.signal_records.iter()),
        graph_direction_signals,
    );
    let graph_direction_conflict_related_ids: Vec<String> = graph_direction_summary
        .conflicts
        .iter()
        .map(graph_direction_conflict_related_id)
        .take(8)
        .collect();
    let graph_backed_missing_compat_direction_signal_names =
        graph_backed_missing_compat_direction_signal_names(
            ir.interfaces.iter().flat_map(|i| i.signal_records.iter()),
            graph_direction_signals,
        );
    let unresolved_missing_compat_direction_signal_names =
        unresolved_missing_compat_direction_signal_names(
            ir.interfaces.iter().flat_map(|i| i.signal_records.iter()),
            graph_direction_signals,
        );
    let (with_direction, with_graph_direction, with_compat_direction_hint) =
        resolved_direction_counts(
            ir.interfaces.iter().flat_map(|i| i.signal_records.iter()),
            graph_direction_signals,
        );
    let with_width: usize = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| s.width_hint.is_some())
        .count();
    let with_table_support: usize = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| !s.supporting_table_ids.is_empty())
        .count();
    let with_resolved_polarity = interface_signals_with_resolved_polarity_count(&ir.interfaces);
    let with_semantic_tags = interface_signals_with_semantic_tags_count(&ir.interfaces);
    let semantic_observations = interface_signal_semantic_observations_count(&ir.interfaces);
    let semantic_candidates = interface_signal_semantic_candidates_count(&ir.interfaces);
    let with_semantic_candidates = interface_signals_with_semantic_candidates_count(&ir.interfaces);
    let with_multiple_semantic_candidates =
        interface_signals_with_multiple_semantic_candidates_count(&ir.interfaces);
    let with_semantic_arbitration =
        interface_signals_with_semantic_arbitration_count(&ir.interfaces);
    let with_decisive_semantic_arbitration =
        interface_signals_with_decisive_semantic_arbitration_count(&ir.interfaces);
    let with_non_decisive_semantic_arbitration =
        interface_signals_with_non_decisive_semantic_arbitration_count(&ir.interfaces);
    let non_decisive_semantic_arbitration_signal_names =
        interface_signals_with_non_decisive_semantic_arbitration(&ir.interfaces);
    let with_prior_guided_semantic_arbitration =
        interface_signals_with_prior_guided_semantic_arbitration_count(&ir.interfaces);
    let prior_guided_semantic_arbitration_signal_names =
        interface_signals_with_prior_guided_semantic_arbitration(&ir.interfaces);
    let blocked_handshake_name_fallback =
        interface_signals_with_blocked_handshake_name_fallback(&ir.interfaces);
    let with_resolved_semantic_role =
        interface_signals_with_resolved_semantic_role_count(&ir.interfaces);
    let with_semantic_consensus = interface_signals_with_semantic_consensus_count(&ir.interfaces);
    let with_high_confidence_semantic_consensus =
        interface_signals_with_high_confidence_semantic_consensus_count(&ir.interfaces);
    let with_alias_dependent_semantic_consensus =
        interface_signals_with_alias_dependent_semantic_consensus_count(&ir.interfaces);
    let alias_dependent_semantic_consensus_signal_names =
        interface_signals_with_alias_dependent_semantic_consensus(&ir.interfaces);
    let with_prior_guided_semantic_consensus =
        interface_signals_with_prior_guided_semantic_consensus_count(&ir.interfaces);
    let prior_guided_semantic_consensus_signal_names =
        interface_signals_with_prior_guided_semantic_consensus(&ir.interfaces);
    let alias_dependent_semantic_candidates =
        interface_signal_alias_dependent_semantic_candidates_count(&ir.interfaces);
    let resolved_semantic_roles_without_consensus =
        resolved_semantic_roles_without_consensus_count(&ir.interfaces);
    let resolved_semantic_roles_without_consensus_signal_names =
        resolved_semantic_roles_without_consensus_signal_names(&ir.interfaces);
    let with_single_source_semantic_grounding =
        interface_signals_with_semantic_grounding_strength_count(
            &ir.interfaces,
            crate::ir::semantic::SemanticGroundingStrength::SingleSource,
        );
    let with_multi_source_semantic_grounding =
        interface_signals_with_semantic_grounding_strength_count(
            &ir.interfaces,
            crate::ir::semantic::SemanticGroundingStrength::MultiSource,
        );
    let with_cross_modality_semantic_grounding =
        interface_signals_with_semantic_grounding_strength_count(
            &ir.interfaces,
            crate::ir::semantic::SemanticGroundingStrength::CrossModality,
        );
    let with_visual_semantic_grounding =
        interface_signals_with_visual_semantic_grounding_count(&ir.interfaces);
    let infrastructure_signal_connectivity =
        infrastructure_signal_connectivity_count(&ir.signal_connectivity);
    let infrastructure_signals_unresolved_source = infrastructure_signals_with_source_status_count(
        &ir.infrastructure_signals,
        InfrastructureSignalSourceStatus::UnresolvedSource,
    );
    let infrastructure_signals_recovered_source = infrastructure_signals_with_source_status_count(
        &ir.infrastructure_signals,
        InfrastructureSignalSourceStatus::RecoveredProducer,
    );
    let infrastructure_signals_multiple_source = infrastructure_signals_with_source_status_count(
        &ir.infrastructure_signals,
        InfrastructureSignalSourceStatus::MultipleRecoveredProducers,
    );
    let infrastructure_signals_no_recovered_distribution =
        infrastructure_signals_with_distribution_status_count(
            &ir.infrastructure_signals,
            InfrastructureSignalDistributionStatus::NoRecoveredConsumers,
        );
    let infrastructure_signals_single_recovered_distribution =
        infrastructure_signals_with_distribution_status_count(
            &ir.infrastructure_signals,
            InfrastructureSignalDistributionStatus::SingleRecoveredConsumer,
        );
    let infrastructure_signals_shared_recovered_distribution =
        infrastructure_signals_with_distribution_status_count(
            &ir.infrastructure_signals,
            InfrastructureSignalDistributionStatus::SharedRecoveredConsumers,
        );
    let infrastructure_topology_records =
        infrastructure_topology_count(&ir.infrastructure_signals, None);
    let infrastructure_clock_gated_branches = infrastructure_topology_count(
        &ir.infrastructure_signals,
        Some(InfrastructureTopologyKind::ClockGatedBranch),
    );
    let infrastructure_reset_synchronizer_stages = infrastructure_topology_count(
        &ir.infrastructure_signals,
        Some(InfrastructureTopologyKind::ResetSynchronizerStages),
    );
    let infrastructure_reset_tree_targets = infrastructure_topology_count(
        &ir.infrastructure_signals,
        Some(InfrastructureTopologyKind::ResetTreeTargets),
    );
    let initial_regular_states = initial_regular_states_count(&ir.regular_states);
    let fully_typed = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| {
            s.width_hint.is_some()
                && (graph_direction_signals.contains(&s.signal_name) || s.direction_hint.is_some())
        })
        .count();
    println!("  total_signal_records: {total_signals}");
    let dir_pct = percentage_or_zero(with_direction, total_signals);
    let w_pct = percentage_or_zero(with_width, total_signals);
    let ft_pct = percentage_or_zero(fully_typed, total_signals);
    let graph_dir_pct = percentage_or_zero(with_graph_direction, total_signals);
    let compat_dir_pct = percentage_or_zero(with_compat_direction_hint, total_signals);
    let table_support_pct = percentage_or_zero(with_table_support, total_signals);
    println!("  with_resolved_direction: {with_direction} ({dir_pct}%)");
    println!("  with_graph_direction: {with_graph_direction} ({graph_dir_pct}%)");
    println!(
        "  graph_direction_conflicts: {}",
        graph_direction_conflicts.len()
    );
    println!("  with_compat_direction_hint: {with_compat_direction_hint} ({compat_dir_pct}%)");
    println!("  with_width: {with_width} ({w_pct}%)");
    println!("  with_table_support: {with_table_support} ({table_support_pct}%)");
    println!("  with_resolved_polarity: {with_resolved_polarity}");
    println!("  with_semantic_tags: {with_semantic_tags}");
    println!("  semantic_candidates: {semantic_candidates}");
    println!("  with_semantic_candidates: {with_semantic_candidates}");
    println!("  with_multiple_semantic_candidates: {with_multiple_semantic_candidates}");
    println!("  with_semantic_arbitration: {with_semantic_arbitration}");
    println!("  with_decisive_semantic_arbitration: {with_decisive_semantic_arbitration}");
    println!("  with_non_decisive_semantic_arbitration: {with_non_decisive_semantic_arbitration}");
    println!("  with_prior_guided_semantic_arbitration: {with_prior_guided_semantic_arbitration}");
    println!(
        "  with_blocked_handshake_name_fallback: {}",
        blocked_handshake_name_fallback.len()
    );
    println!("  with_resolved_semantic_role: {with_resolved_semantic_role}");
    println!("  with_semantic_consensus: {with_semantic_consensus}");
    println!(
        "  with_high_confidence_semantic_consensus: {with_high_confidence_semantic_consensus}"
    );
    println!(
        "  with_alias_dependent_semantic_consensus: {with_alias_dependent_semantic_consensus}"
    );
    println!("  with_prior_guided_semantic_consensus: {with_prior_guided_semantic_consensus}");
    println!("  alias_dependent_semantic_candidates: {alias_dependent_semantic_candidates}");
    println!(
        "  resolved_semantic_roles_without_consensus: {resolved_semantic_roles_without_consensus}"
    );
    println!("  semantic_observations: {semantic_observations}");
    println!("  with_single_source_semantic_grounding: {with_single_source_semantic_grounding}");
    println!("  with_multi_source_semantic_grounding: {with_multi_source_semantic_grounding}");
    println!("  with_cross_modality_semantic_grounding: {with_cross_modality_semantic_grounding}");
    println!("  with_visual_semantic_grounding: {with_visual_semantic_grounding}");
    println!("  fully_typed (resolved_direction+width): {fully_typed} ({ft_pct}%)");
    println!();

    println!("=== Semantic Records ===");
    println!("  actors: {}", ir.actors.len());
    println!(
        "  actor_signal_relations: {}",
        ir.actor_signal_relations.len()
    );
    println!("  actor_ports: {}", ir.actor_ports.len());
    println!("  signal_connectivity: {}", ir.signal_connectivity.len());
    println!("  infrastructure_signal_connectivity: {infrastructure_signal_connectivity}");
    println!(
        "  infrastructure_signals: {}",
        ir.infrastructure_signals.len()
    );
    println!(
        "  infrastructure_signals_unresolved_source: {infrastructure_signals_unresolved_source}"
    );
    println!(
        "  infrastructure_signals_recovered_source: {infrastructure_signals_recovered_source}"
    );
    println!("  infrastructure_signals_multiple_source: {infrastructure_signals_multiple_source}");
    println!(
        "  infrastructure_signals_no_recovered_distribution: {infrastructure_signals_no_recovered_distribution}"
    );
    println!(
        "  infrastructure_signals_single_recovered_distribution: {infrastructure_signals_single_recovered_distribution}"
    );
    println!(
        "  infrastructure_signals_shared_recovered_distribution: {infrastructure_signals_shared_recovered_distribution}"
    );
    println!("  infrastructure_topology_records: {infrastructure_topology_records}");
    println!("  infrastructure_clock_gated_branches: {infrastructure_clock_gated_branches}");
    println!(
        "  infrastructure_reset_synchronizer_stages: {infrastructure_reset_synchronizer_stages}"
    );
    println!("  infrastructure_reset_tree_targets: {infrastructure_reset_tree_targets}");
    println!(
        "  interface_signal_conflicts: {}",
        ir.interface_signal_conflicts.len()
    );
    println!(
        "  signal_connectivity_conflicts: {}",
        ir.signal_connectivity_conflicts.len()
    );
    println!(
        "  signal_polarity_conflicts: {}",
        ir.signal_polarity_conflicts.len()
    );
    println!(
        "  signal_semantic_conflicts: {}",
        ir.signal_semantic_conflicts.len()
    );
    println!("  interfaces: {}", ir.interfaces.len());
    println!("  invariants: {}", ir.invariants.len());
    println!(
        "  symbol_definitions (enums+consts): {}",
        ir.symbol_definitions.len()
    );
    println!("  regular_states: {}", ir.regular_states.len());
    println!("  initial_regular_states: {initial_regular_states}");
    println!("  state_transitions: {}", ir.state_transitions.len());
    println!("  register_records: {}", ir.register_records.len());
    println!("  timing_constraints: {}", ir.timing_constraints.len());
    println!("  temporal_rules: {}", ir.temporal_rules.len());
    println!("  temporal_conflicts: {}", ir.temporal_conflicts.len());
    println!(
        "  temporal_rules_with_actor_grounding: {}",
        temporal_rules_with_actor_grounding_count(&ir.temporal_rules)
    );
    println!(
        "  temporal_rules_with_handshake_completion: {}",
        temporal_rules_with_handshake_completion_count(&ir.temporal_rules)
    );
    println!(
        "  temporal_rules_with_alias_dependent_handshake_completion: {}",
        temporal_rules_with_alias_dependent_handshake_completion_count(
            &ir.temporal_rules,
            &ir.interfaces,
        )
    );
    println!(
        "  signal_constraints (Level 2 NLP): {}",
        ir.signal_constraints.len()
    );
    println!(
        "  conditional_rules (Level 2 NLP): {}",
        ir.conditional_rules.len()
    );
    println!();

    println!("=== System Contract ===");
    if let Some(sc) = &ir.system_contract {
        println!("  clock_signal: {}", sc.clock_signal);
        println!("  reset_signal: {}", sc.reset_signal);
        println!("  reset_kind: {:?}", sc.reset_kind);
        println!("  automation_confidence: {:?}", sc.automation_confidence);
    } else {
        println!("  ABSENT — no explicit Clock/Reset declarations found");
    }
    println!();

    println!("=== Infrastructure Signals ===");
    if ir.infrastructure_signals.is_empty() {
        println!("  none");
    } else {
        for signal in &ir.infrastructure_signals {
            println!(
                "  - {} ({}): source={}, distribution={}, distributed_to={}, topology={}",
                signal.signal_name,
                describe_infrastructure_signal_kind(signal.kind),
                describe_infrastructure_source_status(signal.source_status),
                describe_infrastructure_distribution_status(signal.distribution_status),
                signal.distributed_to_actor_names.len(),
                signal.infrastructure_topology.len()
            );
            for topology in &signal.infrastructure_topology {
                println!(
                    "    - {}: component={}, stages={}, targets={}",
                    describe_infrastructure_topology_kind(topology.topology_kind),
                    topology.component_name.as_deref().unwrap_or("n/a"),
                    topology
                        .stage_count
                        .map(|count| count.to_string())
                        .unwrap_or_else(|| "n/a".to_string()),
                    topology.target_actor_names.len()
                );
            }
        }
    }
    println!();

    println!("=== Residual Decisions ===");
    println!("  count: {}", ir.residual_decisions.len());
    for rd in &ir.residual_decisions {
        println!("  - [{}] {}", rd.packet_id, rd.question);
    }
    if !ir.signal_connectivity_conflicts.is_empty() {
        println!();
        println!("=== Signal Connectivity Conflicts ===");
        for conflict in ir.signal_connectivity_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_signal_connectivity_conflict(conflict)
            );
        }
        if ir.signal_connectivity_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.signal_connectivity_conflicts.len() - 8
            );
        }
    }
    if !ir.interface_signal_conflicts.is_empty() {
        println!();
        println!("=== Interface Signal Conflicts ===");
        for conflict in ir.interface_signal_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_interface_signal_conflict(conflict)
            );
        }
        if ir.interface_signal_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.interface_signal_conflicts.len() - 8
            );
        }
    }
    if !ir.signal_polarity_conflicts.is_empty() {
        println!();
        println!("=== Signal Polarity Conflicts ===");
        for conflict in ir.signal_polarity_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_signal_polarity_conflict(conflict)
            );
        }
        if ir.signal_polarity_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.signal_polarity_conflicts.len() - 8
            );
        }
    }
    if !ir.signal_semantic_conflicts.is_empty() {
        println!();
        println!("=== Signal Semantic Conflicts ===");
        for conflict in ir.signal_semantic_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_signal_semantic_conflict(conflict)
            );
        }
        if ir.signal_semantic_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.signal_semantic_conflicts.len() - 8
            );
        }
    }

    let negative_knowledge_prior_matches = semantic_negative_knowledge_prior_matches(ir);
    println!();
    println!("=== Negative Knowledge Priors ===");
    println!(
        "  matched_carried_conflict_or_residual_patterns: {}",
        negative_knowledge_prior_matches.len()
    );
    println!(
        "  rescan_recommendations: {}",
        negative_knowledge_prior_matches.len()
    );
    println!(
        "  stronger_corroboration_requirements: {}",
        negative_knowledge_prior_matches.len()
    );

    let missing_producer_signals = protocol_missing_producer_signal_names(&ir.signal_connectivity);
    let infrastructure_missing_producer_signals =
        infrastructure_missing_producer_signal_names(&ir.signal_connectivity);
    let infrastructure_signal_connectivity =
        infrastructure_signal_connectivity_count(&ir.signal_connectivity);
    let missing_consumer_signals: Vec<String> = ir
        .signal_connectivity
        .iter()
        .filter(|record| record.consumer_actor_ids.is_empty())
        .map(|record| record.signal_name.clone())
        .collect();
    let missing_graph_direction_count = missing_graph_direction_signal_names.len();
    let missing_temporal_clock_grounding =
        temporal_rules_missing_clock_grounding_count(&ir.temporal_rules);
    let temporal_rules_with_cycle_window =
        temporal_rules_with_cycle_window_count(&ir.temporal_rules);
    let temporal_rules_with_actor_grounding =
        temporal_rules_with_actor_grounding_count(&ir.temporal_rules);
    let temporal_rules_missing_clock_grounding_rule_ids =
        temporal_rules_missing_clock_grounding_rule_ids(&ir.temporal_rules);
    let temporal_rules_missing_cycle_window_rule_ids =
        temporal_rules_missing_cycle_window_rule_ids(&ir.temporal_rules);
    let temporal_rules_missing_actor_grounding_rule_ids =
        temporal_rules_missing_actor_grounding_rule_ids(&ir.temporal_rules);
    let temporal_rules_with_handshake_completion =
        temporal_rules_with_handshake_completion_count(&ir.temporal_rules);
    let temporal_rules_with_alias_dependent_handshake_completion =
        temporal_rules_with_alias_dependent_handshake_completion_count(
            &ir.temporal_rules,
            &ir.interfaces,
        );
    let temporal_rule_surface_input_ids = temporal_rule_surface_input_ids(
        &ir.timing_constraints,
        &ir.signal_constraints,
        &ir.conditional_rules,
    );
    let alias_dependent_handshake_completion_signal_names =
        temporal_rules_with_alias_dependent_handshake_completion_signal_names(
            &ir.temporal_rules,
            &ir.interfaces,
        );
    let temporal_rules_with_multi_predicate_antecedents =
        temporal_rules_with_multi_predicate_antecedents_count(&ir.temporal_rules);
    let handshake_role_signal_names =
        interface_signals_with_handshake_semantic_role_names(&ir.interfaces);
    let multi_predicate_antecedent_rule_ids =
        temporal_rules_with_multi_predicate_antecedents_rule_ids(&ir.temporal_rules);
    let actor_signal_relation_related_ids =
        actor_signal_relation_related_ids(&ir.actor_signal_relations);
    let signal_connectivity_conflict_related_ids = ir
        .signal_connectivity_conflicts
        .iter()
        .map(|conflict| conflict.conflict_id.clone())
        .collect::<Vec<_>>();

    let mut findings = Vec::new();
    if !ir.actor_signal_relations.is_empty() && ir.actor_ports.is_empty() {
        let actor_port_gap_related_ids = actor_signal_relation_related_ids
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        findings.push(finding(
            "semantic_actor_ports_missing",
            ValidationFindingSeverity::Error,
            "knowledge_graph",
            "SemanticIR carries actor-signal relations but failed to synthesize actor-relative ports",
            actor_port_gap_related_ids.clone(),
        ));
        push_actor_port_gap_rescan_guidance(
            &mut findings,
            SEMANTIC_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &actor_port_gap_related_ids,
        );
    }
    if !missing_producer_signals.is_empty() {
        findings.push(finding(
            "semantic_connectivity_missing_producer",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal(s) in SemanticIR connectivity have no resolved producer actor",
                missing_producer_signals.len()
            ),
            missing_producer_signals.iter().take(8).cloned().collect(),
        ));
        push_connectivity_gap_rescan_guidance(
            &mut findings,
            SEMANTIC_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            "producer-side",
            &missing_producer_signals
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !infrastructure_missing_producer_signals.is_empty() {
        findings.push(finding(
            "semantic_infrastructure_connectivity_missing_producer",
            ValidationFindingSeverity::Info,
            "system_contract",
            format!(
                "{} infrastructure signal(s) have no resolved producer actor in SemanticIR connectivity; canonical sourcing status remains explicit in the infrastructure/system-contract surface",
                infrastructure_missing_producer_signals.len()
            ),
            infrastructure_missing_producer_signals
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !missing_consumer_signals.is_empty() {
        findings.push(finding(
            "semantic_connectivity_missing_consumer",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal(s) in SemanticIR connectivity have no resolved consumer actor",
                missing_consumer_signals.len()
            ),
            missing_consumer_signals.iter().take(8).cloned().collect(),
        ));
        push_connectivity_gap_rescan_guidance(
            &mut findings,
            SEMANTIC_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            "consumer-side",
            &missing_consumer_signals
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.signal_connectivity_conflicts.is_empty() {
        findings.push(finding(
            "semantic_signal_connectivity_conflicts_present",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal connectivity conflict(s) detected; the structural KG still has unresolved producer ambiguity",
                ir.signal_connectivity_conflicts.len()
            ),
            signal_connectivity_conflict_related_ids.clone(),
        ));
        push_signal_connectivity_conflict_rescan_guidance(
            &mut findings,
            SEMANTIC_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &signal_connectivity_conflict_related_ids,
        );
    }
    if !ir.interface_signal_conflicts.is_empty() {
        let interface_signal_conflict_related_ids = ir
            .interface_signal_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "semantic_interface_signal_conflicts_present",
            ValidationFindingSeverity::Warning,
            "interface_signal_conflicts",
            format!(
                "{} interface signal conflict(s) detected; conflicting direction/width evidence is still unresolved in the canonical interface surface",
                ir.interface_signal_conflicts.len()
            ),
            interface_signal_conflict_related_ids.clone(),
        ));
        push_interface_signal_conflict_rescan_guidance(
            &mut findings,
            SEMANTIC_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &interface_signal_conflict_related_ids,
        );
    }
    if !ir.signal_polarity_conflicts.is_empty() {
        let signal_polarity_conflict_related_ids = ir
            .signal_polarity_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "semantic_signal_polarity_conflicts_present",
            ValidationFindingSeverity::Warning,
            "polarity_conflicts",
            format!(
                "{} signal polarity conflict(s) remain unresolved in the carried canonical semantic surface",
                ir.signal_polarity_conflicts.len()
            ),
            signal_polarity_conflict_related_ids.clone(),
        ));
        push_signal_polarity_conflict_rescan_guidance(
            &mut findings,
            SEMANTIC_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &signal_polarity_conflict_related_ids,
        );
    }
    if !ir.signal_semantic_conflicts.is_empty() {
        let signal_semantic_conflict_related_ids = ir
            .signal_semantic_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "semantic_signal_semantic_conflicts_present",
            ValidationFindingSeverity::Warning,
            "semantic_role_conflicts",
            format!(
                "{} semantic-role conflict(s) remain unresolved in the carried canonical semantic surface",
                ir.signal_semantic_conflicts.len()
            ),
            signal_semantic_conflict_related_ids.clone(),
        ));
        push_signal_semantic_conflict_rescan_guidance(
            &mut findings,
            SEMANTIC_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &signal_semantic_conflict_related_ids,
        );
    }
    if with_non_decisive_semantic_arbitration > 0 {
        findings.push(finding(
            "semantic_non_decisive_semantic_arbitration_present",
            ValidationFindingSeverity::Info,
            "semantic_role_arbitration",
            format!(
                "{with_non_decisive_semantic_arbitration} interface signal(s) still have competing semantic role candidates; the canonical surface preserves the current lead, runner-up, and evidence margin without forcing an unsafe winner"
            ),
            non_decisive_semantic_arbitration_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_semantic_role_arbitration_rescan_guidance(
            &mut findings,
            SEMANTIC_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &non_decisive_semantic_arbitration_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if with_prior_guided_semantic_arbitration > 0 {
        findings.push(finding(
            "semantic_prior_guided_semantic_arbitration_present",
            ValidationFindingSeverity::Info,
            "semantic_role_arbitration",
            format!(
                "{with_prior_guided_semantic_arbitration} interface signal(s) use learned modality-reliability priors to resolve otherwise competing local semantic-role evidence"
            ),
            prior_guided_semantic_arbitration_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !blocked_handshake_name_fallback.is_empty() {
        findings.push(finding(
            "semantic_handshake_name_fallback_blocked_present",
            ValidationFindingSeverity::Info,
            "semantic_role_arbitration",
            format!(
                "{} handshake-shaped signal(s) intentionally block literal VALID/READY fallback because their preserved semantic role state is still contested or only provisional",
                blocked_handshake_name_fallback.len()
            ),
            blocked_handshake_name_fallback.clone(),
        ));
    }
    if resolved_semantic_roles_without_consensus > 0 {
        findings.push(finding(
            "semantic_resolved_roles_without_consensus_present",
            ValidationFindingSeverity::Info,
            "semantic_role_consensus",
            format!(
                "{resolved_semantic_roles_without_consensus} resolved semantic role(s) still rely on fallback carry-through without an explicit canonical consensus profile"
            ),
            resolved_semantic_roles_without_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_semantic_role_consensus_rescan_guidance(
            &mut findings,
            SEMANTIC_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &resolved_semantic_roles_without_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if with_alias_dependent_semantic_consensus > 0 {
        findings.push(finding(
            "semantic_alias_dependent_semantic_consensus_present",
            ValidationFindingSeverity::Info,
            "semantic_role_consensus",
            format!(
                "{with_alias_dependent_semantic_consensus} resolved semantic role(s) currently depend only on alias-grounded evidence rather than direct signal mentions or corroborating non-alias modalities"
            ),
            alias_dependent_semantic_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_alias_dependent_semantic_consensus_rescan_guidance(
            &mut findings,
            SEMANTIC_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &alias_dependent_semantic_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if with_prior_guided_semantic_consensus > 0 {
        findings.push(finding(
            "semantic_prior_guided_semantic_consensus_present",
            ValidationFindingSeverity::Info,
            "semantic_role_consensus",
            format!(
                "{with_prior_guided_semantic_consensus} resolved semantic role(s) now carry explicit consensus that was strengthened by learned modality-reliability priors"
            ),
            prior_guided_semantic_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_prior_guided_semantic_consensus_rescan_guidance(
            &mut findings,
            SEMANTIC_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &prior_guided_semantic_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.actor_ports.is_empty() && missing_graph_direction_count > 0 {
        findings.push(finding(
            "semantic_graph_direction_coverage_incomplete",
            ValidationFindingSeverity::Info,
            "knowledge_graph",
            format!(
                "{missing_graph_direction_count} interface signal record(s) still lack graph-derived direction coverage"
            ),
            missing_graph_direction_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_graph_direction_coverage_rescan_guidance(
            &mut findings,
            SEMANTIC_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &missing_graph_direction_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !graph_direction_conflicts.is_empty() {
        findings.push(finding(
            "semantic_graph_direction_conflicts_present",
            ValidationFindingSeverity::Warning,
            "knowledge_graph",
            format!(
                "{} signal(s) have conflicting actor-relative directions from the same actor across {} actor-signal conflict record(s); graph-direction coverage remains intentionally unresolved until upstream graph evidence is clarified",
                graph_direction_conflicts.len(),
                graph_direction_summary.conflicts.len()
            ),
            graph_direction_conflict_related_ids.clone(),
        ));
        push_graph_direction_conflict_rescan_guidance(
            &mut findings,
            SEMANTIC_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &graph_direction_conflict_related_ids,
        );
    }
    if !graph_backed_missing_compat_direction_signal_names.is_empty() {
        findings.push(finding(
            "semantic_compat_direction_hints_lag_graph",
            ValidationFindingSeverity::Info,
            "compatibility_surface",
            format!(
                "{} interface signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist",
                graph_backed_missing_compat_direction_signal_names.len()
            ),
            graph_backed_missing_compat_direction_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !unresolved_missing_compat_direction_signal_names.is_empty() {
        findings.push(finding(
            "semantic_compat_direction_hints_incomplete",
            ValidationFindingSeverity::Info,
            "compatibility_surface",
            format!(
                "{} interface signal record(s) still lack flat compatibility direction hints and actor-relative graph coverage",
                unresolved_missing_compat_direction_signal_names.len()
            ),
            unresolved_missing_compat_direction_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !ir.temporal_rules.is_empty() && missing_temporal_clock_grounding > 0 {
        findings.push(finding(
            "semantic_temporal_rules_missing_clock_grounding",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            format!(
                "{missing_temporal_clock_grounding} temporal rule(s) still lack explicit clock or edge grounding"
            ),
            temporal_rules_missing_clock_grounding_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_temporal_clock_grounding_rescan_guidance(
            &mut findings,
            SEMANTIC_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &temporal_rules_missing_clock_grounding_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.temporal_rules.is_empty() && temporal_rules_with_cycle_window == 0 {
        findings.push(finding(
            "semantic_temporal_rules_missing_cycle_windows",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            "typed temporal rules exist, but none currently carry explicit cycle-window bounds"
                .to_string(),
            temporal_rules_missing_cycle_window_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_temporal_cycle_window_rescan_guidance(
            &mut findings,
            SEMANTIC_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &temporal_rules_missing_cycle_window_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.temporal_rules.is_empty()
        && !ir.actor_signal_relations.is_empty()
        && temporal_rules_with_actor_grounding == 0
    {
        findings.push(finding(
            "semantic_temporal_rules_missing_actor_grounding",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            "typed temporal rules exist, but none currently carry actor-relative drive/sample grounding"
                .to_string(),
            temporal_rules_missing_actor_grounding_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_temporal_actor_grounding_rescan_guidance(
            &mut findings,
            SEMANTIC_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &temporal_rules_missing_actor_grounding_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.temporal_conflicts.is_empty() {
        let temporal_conflict_related_ids = ir
            .temporal_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "semantic_temporal_conflicts_present",
            ValidationFindingSeverity::Warning,
            "temporal_conflicts",
            format!(
                "{} typed temporal conflict(s) detected across contradictory value obligations",
                ir.temporal_conflicts.len()
            ),
            temporal_conflict_related_ids.clone(),
        ));
        push_temporal_conflict_rescan_guidance(
            &mut findings,
            SEMANTIC_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &temporal_conflict_related_ids,
        );
    }
    if !ir.regular_states.is_empty() && initial_regular_states != 1 {
        findings.push(finding(
            "semantic_state_machine_initial_cardinality",
            ValidationFindingSeverity::Warning,
            "state_machine",
            format!(
                "SemanticIR state machine has {initial_regular_states} canonical initial state(s); expected exactly one when regular states are present"
            ),
            state_machine_initial_cardinality_related_ids(&ir.regular_states),
        ));
    }
    if temporal_rules_with_alias_dependent_handshake_completion > 0 {
        findings.push(finding(
            "semantic_alias_dependent_handshake_completion_present",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            format!(
                "{temporal_rules_with_alias_dependent_handshake_completion} typed temporal rule(s) currently derive HandshakeComplete predicates from alias-dependent semantic role consensus"
            ),
            alias_dependent_handshake_completion_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !ir.temporal_rules.is_empty()
        && !handshake_role_signal_names.is_empty()
        && temporal_rules_with_handshake_completion == 0
    {
        let handshake_completion_gap_signal_ids = handshake_role_signal_names
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        findings.push(finding(
            "semantic_temporal_handshake_completion_gap",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            format!(
                "{} interface signal(s) carry handshake semantic roles (ValidLike/ReadyLike) but no typed temporal rule currently expresses a HandshakeComplete predicate",
                handshake_role_signal_names.len()
            ),
            handshake_completion_gap_signal_ids.clone(),
        ));
        push_temporal_handshake_completion_gap_rescan_guidance(
            &mut findings,
            SEMANTIC_TEMPORAL_HANDSHAKE_COMPLETION_GAP_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &handshake_completion_gap_signal_ids,
        );
    }
    if temporal_rules_with_multi_predicate_antecedents > 0 {
        let multi_pred_rule_ids = multi_predicate_antecedent_rule_ids
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        findings.push(finding(
            "semantic_temporal_multi_predicate_antecedents_present",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            format!(
                "{temporal_rules_with_multi_predicate_antecedents} typed temporal rule(s) carry multi-predicate antecedents; compound condition extraction may benefit from further review"
            ),
            multi_pred_rule_ids.clone(),
        ));
        push_temporal_multi_predicate_antecedents_rescan_guidance(
            &mut findings,
            SEMANTIC_TEMPORAL_MULTI_PREDICATE_ANTECEDENTS_SURFACE_RESCAN_GUIDANCE,
            "SemanticIR",
            &multi_pred_rule_ids,
        );
    }
    // KG-quality benchmarks — flag when key dimensions fall below defined thresholds.
    let graph_direction_coverage_pct = percentage_or_zero(with_graph_direction, total_signals);
    if total_signals > 0 && graph_direction_coverage_pct < 50 {
        findings.push(finding(
            "semantic_kg_graph_direction_coverage_below_benchmark",
            ValidationFindingSeverity::Info,
            "kg_quality_benchmark",
            format!(
                "graph direction coverage is {graph_direction_coverage_pct}% ({with_graph_direction}/{total_signals}) — below the 50% KG-quality benchmark"
            ),
            Vec::new(),
        ));
    }
    let semantic_role_resolution_pct =
        percentage_or_zero(with_resolved_semantic_role, total_signals);
    if total_signals > 0 && semantic_role_resolution_pct < 30 {
        findings.push(finding(
            "semantic_kg_semantic_role_resolution_below_benchmark",
            ValidationFindingSeverity::Info,
            "kg_quality_benchmark",
            format!(
                "semantic role resolution rate is {semantic_role_resolution_pct}% ({with_resolved_semantic_role}/{total_signals}) — below the 30% KG-quality benchmark"
            ),
            Vec::new(),
        ));
    }
    let semantic_consensus_coverage_pct = if with_resolved_semantic_role > 0 {
        ((with_semantic_consensus as f64 / with_resolved_semantic_role as f64) * 100.0).round()
            as usize
    } else {
        0
    };
    if with_resolved_semantic_role > 0 && semantic_consensus_coverage_pct < 50 {
        findings.push(finding(
            "semantic_kg_semantic_consensus_coverage_below_benchmark",
            ValidationFindingSeverity::Info,
            "kg_quality_benchmark",
            format!(
                "semantic consensus coverage is {semantic_consensus_coverage_pct}% ({with_semantic_consensus}/{with_resolved_semantic_role}) — below the 50% KG-quality benchmark"
            ),
            Vec::new(),
        ));
    }
    if ir.temporal_rules.is_empty()
        && (!ir.timing_constraints.is_empty()
            || !ir.signal_constraints.is_empty()
            || !ir.conditional_rules.is_empty())
    {
        let temporal_rule_surface_related_ids = temporal_rule_surface_input_ids
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        findings.push(finding(
            "semantic_temporal_rule_surface_missing",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            "semantic evidence includes timing/constraint records but no typed temporal rules were derived"
                .to_string(),
            temporal_rule_surface_related_ids.clone(),
        ));
        push_temporal_rule_surface_rescan_guidance(
            &mut findings,
            "semantic_temporal_rule_surface_rescan_guidance",
            "SemanticIR",
            &temporal_rule_surface_related_ids,
        );
    }
    if !ir.residual_decisions.is_empty() {
        findings.push(finding(
            "semantic_residual_decisions_present",
            ValidationFindingSeverity::Warning,
            "residual_decisions",
            format!(
                "SemanticIR still carries {} residual decision packet(s)",
                ir.residual_decisions.len()
            ),
            ir.residual_decisions
                .iter()
                .map(|packet| packet.packet_id.clone())
                .collect(),
        ));
    }
    if !negative_knowledge_prior_matches.is_empty() {
        findings.push(finding(
            "semantic_negative_knowledge_prior_matches",
            ValidationFindingSeverity::Info,
            "negative_knowledge",
            format!(
                "{} carried conflict/residual pattern(s) match prior negative knowledge; this is a caution signal only, not an override of current semantic evidence",
                negative_knowledge_prior_matches.len()
            ),
            negative_knowledge_prior_matches.clone(),
        ));
    }
    push_negative_knowledge_rescan_guidance(
        &mut findings,
        "semantic_negative_knowledge_rescan_guidance",
        "SemanticIR",
        &negative_knowledge_prior_matches,
    );

    let report = ValidationReportRecord {
        report_id: format!("validation_semantic_ir_{artifact_fingerprint}"),
        validated_stage: IrStage::SemanticIr,
        artifact_fingerprint,
        summary: format!(
            "SemanticIR validation for {} with {} finding(s)",
            ir.document_identity.display_name,
            findings.len()
        ),
        overall_score: None,
        grade: None,
        metrics: vec![
            metric("total_signal_records", total_signals.to_string()),
            metric("with_resolved_direction", with_direction.to_string()),
            metric("with_graph_direction", with_graph_direction.to_string()),
            metric(
                "graph_direction_conflicts",
                graph_direction_conflicts.len().to_string(),
            ),
            metric(
                "with_compat_direction_hint",
                with_compat_direction_hint.to_string(),
            ),
            metric("with_width", with_width.to_string()),
            metric("with_table_support", with_table_support.to_string()),
            metric("with_resolved_polarity", with_resolved_polarity.to_string()),
            metric("with_semantic_tags", with_semantic_tags.to_string()),
            metric("semantic_candidates", semantic_candidates.to_string()),
            metric(
                "with_semantic_candidates",
                with_semantic_candidates.to_string(),
            ),
            metric(
                "with_multiple_semantic_candidates",
                with_multiple_semantic_candidates.to_string(),
            ),
            metric(
                "with_semantic_arbitration",
                with_semantic_arbitration.to_string(),
            ),
            metric(
                "with_decisive_semantic_arbitration",
                with_decisive_semantic_arbitration.to_string(),
            ),
            metric(
                "with_non_decisive_semantic_arbitration",
                with_non_decisive_semantic_arbitration.to_string(),
            ),
            metric(
                "with_prior_guided_semantic_arbitration",
                with_prior_guided_semantic_arbitration.to_string(),
            ),
            metric(
                "with_blocked_handshake_name_fallback",
                blocked_handshake_name_fallback.len().to_string(),
            ),
            metric(
                "with_resolved_semantic_role",
                with_resolved_semantic_role.to_string(),
            ),
            metric(
                "with_semantic_consensus",
                with_semantic_consensus.to_string(),
            ),
            metric(
                "with_high_confidence_semantic_consensus",
                with_high_confidence_semantic_consensus.to_string(),
            ),
            metric(
                "with_alias_dependent_semantic_consensus",
                with_alias_dependent_semantic_consensus.to_string(),
            ),
            metric(
                "with_prior_guided_semantic_consensus",
                with_prior_guided_semantic_consensus.to_string(),
            ),
            metric(
                "alias_dependent_semantic_candidates",
                alias_dependent_semantic_candidates.to_string(),
            ),
            metric(
                "resolved_semantic_roles_without_consensus",
                resolved_semantic_roles_without_consensus.to_string(),
            ),
            metric("semantic_observations", semantic_observations.to_string()),
            metric(
                "with_single_source_semantic_grounding",
                with_single_source_semantic_grounding.to_string(),
            ),
            metric(
                "with_multi_source_semantic_grounding",
                with_multi_source_semantic_grounding.to_string(),
            ),
            metric(
                "with_cross_modality_semantic_grounding",
                with_cross_modality_semantic_grounding.to_string(),
            ),
            metric(
                "with_visual_semantic_grounding",
                with_visual_semantic_grounding.to_string(),
            ),
            metric("fully_typed", fully_typed.to_string()),
            metric("actors", ir.actors.len().to_string()),
            metric(
                "actor_signal_relations",
                ir.actor_signal_relations.len().to_string(),
            ),
            metric("actor_ports", ir.actor_ports.len().to_string()),
            metric(
                "signal_connectivity",
                ir.signal_connectivity.len().to_string(),
            ),
            metric(
                "infrastructure_signal_connectivity",
                infrastructure_signal_connectivity.to_string(),
            ),
            metric(
                "infrastructure_signals",
                ir.infrastructure_signals.len().to_string(),
            ),
            metric(
                "infrastructure_signals_unresolved_source",
                infrastructure_signals_unresolved_source.to_string(),
            ),
            metric(
                "infrastructure_signals_recovered_source",
                infrastructure_signals_recovered_source.to_string(),
            ),
            metric(
                "infrastructure_signals_multiple_source",
                infrastructure_signals_multiple_source.to_string(),
            ),
            metric(
                "infrastructure_signals_no_recovered_distribution",
                infrastructure_signals_no_recovered_distribution.to_string(),
            ),
            metric(
                "infrastructure_signals_single_recovered_distribution",
                infrastructure_signals_single_recovered_distribution.to_string(),
            ),
            metric(
                "infrastructure_signals_shared_recovered_distribution",
                infrastructure_signals_shared_recovered_distribution.to_string(),
            ),
            metric(
                "infrastructure_topology_records",
                infrastructure_topology_records.to_string(),
            ),
            metric(
                "infrastructure_clock_gated_branches",
                infrastructure_clock_gated_branches.to_string(),
            ),
            metric(
                "infrastructure_reset_synchronizer_stages",
                infrastructure_reset_synchronizer_stages.to_string(),
            ),
            metric(
                "infrastructure_reset_tree_targets",
                infrastructure_reset_tree_targets.to_string(),
            ),
            metric(
                "infrastructure_signals_missing_producer",
                infrastructure_missing_producer_signals.len().to_string(),
            ),
            metric(
                "interface_signal_conflicts",
                ir.interface_signal_conflicts.len().to_string(),
            ),
            metric(
                "signal_connectivity_conflicts",
                ir.signal_connectivity_conflicts.len().to_string(),
            ),
            metric(
                "signal_polarity_conflicts",
                ir.signal_polarity_conflicts.len().to_string(),
            ),
            metric(
                "signal_semantic_conflicts",
                ir.signal_semantic_conflicts.len().to_string(),
            ),
            metric("interfaces", ir.interfaces.len().to_string()),
            metric("invariants", ir.invariants.len().to_string()),
            metric(
                "symbol_definitions",
                ir.symbol_definitions.len().to_string(),
            ),
            metric("regular_states", ir.regular_states.len().to_string()),
            metric("initial_regular_states", initial_regular_states.to_string()),
            metric("state_transitions", ir.state_transitions.len().to_string()),
            metric("register_records", ir.register_records.len().to_string()),
            metric(
                "timing_constraints",
                ir.timing_constraints.len().to_string(),
            ),
            metric("temporal_rules", ir.temporal_rules.len().to_string()),
            metric(
                "temporal_conflicts",
                ir.temporal_conflicts.len().to_string(),
            ),
            metric(
                "temporal_rules_with_cycle_window",
                temporal_rules_with_cycle_window.to_string(),
            ),
            metric(
                "temporal_rules_with_actor_grounding",
                temporal_rules_with_actor_grounding.to_string(),
            ),
            metric(
                "temporal_rules_with_handshake_completion",
                temporal_rules_with_handshake_completion.to_string(),
            ),
            metric(
                "temporal_rules_with_alias_dependent_handshake_completion",
                temporal_rules_with_alias_dependent_handshake_completion.to_string(),
            ),
            metric(
                "temporal_rules_with_multi_predicate_antecedents",
                temporal_rules_with_multi_predicate_antecedents.to_string(),
            ),
            metric(
                "temporal_rules_missing_clock_grounding",
                missing_temporal_clock_grounding.to_string(),
            ),
            metric(
                "signal_constraints",
                ir.signal_constraints.len().to_string(),
            ),
            metric("conditional_rules", ir.conditional_rules.len().to_string()),
            metric(
                "residual_decisions",
                ir.residual_decisions.len().to_string(),
            ),
            metric(
                "negative_knowledge_prior_matches",
                negative_knowledge_prior_matches.len().to_string(),
            ),
            metric(
                "negative_knowledge_rescan_recommendations",
                negative_knowledge_prior_matches.len().to_string(),
            ),
            metric(
                "negative_knowledge_corroboration_requirements",
                negative_knowledge_prior_matches.len().to_string(),
            ),
        ],
        findings,
    };
    print_validation_findings(&report);
    report
}

fn validate_intent_ir(ir: &IntentIr, artifact_fingerprint: String) -> ValidationReportRecord {
    println!("command: validate");
    println!("stage: intent_ir");
    println!("document_key: {}", ir.document_identity.document_key);
    println!();

    // Layer E: count declared signals as High OR Medium confidence.
    // High = from structured signal description tables (most authoritative).
    // Medium = from Tier 2 KG actor-signal relation extraction from prose.
    // Low = heuristic co-mention noise — excluded from coverage metrics.
    let declared_signals: Vec<_> = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| !matches!(s.automation_confidence, AutomationConfidence::Low))
        .collect();
    let heuristic_signals: usize = ir
        .interfaces
        .iter()
        .flat_map(|i| &i.signal_records)
        .filter(|s| matches!(s.automation_confidence, AutomationConfidence::Low))
        .count();

    println!("=== Signal Coverage ===");
    let declared_count = declared_signals.len();
    let graph_direction_summary = graph_direction_coverage_summary(&ir.actor_ports);
    let graph_direction_signals = &graph_direction_summary.resolved_signal_names;
    let graph_direction_conflicts = &graph_direction_summary.conflicted_signal_names;
    let missing_graph_direction_signal_names = missing_graph_direction_signal_names(
        declared_signals.iter().copied(),
        graph_direction_signals,
    );
    let graph_direction_conflict_related_ids: Vec<String> = graph_direction_summary
        .conflicts
        .iter()
        .map(graph_direction_conflict_related_id)
        .take(8)
        .collect();
    let graph_backed_missing_compat_direction_signal_names =
        graph_backed_missing_compat_direction_signal_names(
            declared_signals.iter().copied(),
            graph_direction_signals,
        );
    let unresolved_missing_compat_direction_signal_names =
        unresolved_missing_compat_direction_signal_names(
            declared_signals.iter().copied(),
            graph_direction_signals,
        );
    let (with_direction, with_graph_direction, with_compat_direction_hint) =
        resolved_direction_counts(declared_signals.iter().copied(), graph_direction_signals);
    // Both numeric and parametric widths count as "known" — parametric means the
    // integrator will set the value (e.g. ADDR_WIDTH=32) at instantiation time.
    let with_numeric_width = declared_signals
        .iter()
        .filter(|s| matches!(s.width_hint, Some(WidthHint::Numeric(_))))
        .count();
    let with_parametric_width = declared_signals
        .iter()
        .filter(|s| matches!(s.width_hint, Some(WidthHint::Parametric(_))))
        .count();
    let with_width = with_numeric_width + with_parametric_width;
    let with_table_support = declared_signals
        .iter()
        .filter(|s| !s.supporting_table_ids.is_empty())
        .count();
    let with_resolved_polarity = interface_signals_with_resolved_polarity_count(&ir.interfaces);
    let with_semantic_tags = interface_signals_with_semantic_tags_count(&ir.interfaces);
    let semantic_observations = interface_signal_semantic_observations_count(&ir.interfaces);
    let semantic_candidates = interface_signal_semantic_candidates_count(&ir.interfaces);
    let with_semantic_candidates = interface_signals_with_semantic_candidates_count(&ir.interfaces);
    let with_multiple_semantic_candidates =
        interface_signals_with_multiple_semantic_candidates_count(&ir.interfaces);
    let with_semantic_arbitration =
        interface_signals_with_semantic_arbitration_count(&ir.interfaces);
    let with_decisive_semantic_arbitration =
        interface_signals_with_decisive_semantic_arbitration_count(&ir.interfaces);
    let with_non_decisive_semantic_arbitration =
        interface_signals_with_non_decisive_semantic_arbitration_count(&ir.interfaces);
    let non_decisive_semantic_arbitration_signal_names =
        interface_signals_with_non_decisive_semantic_arbitration(&ir.interfaces);
    let with_prior_guided_semantic_arbitration =
        interface_signals_with_prior_guided_semantic_arbitration_count(&ir.interfaces);
    let prior_guided_semantic_arbitration_signal_names =
        interface_signals_with_prior_guided_semantic_arbitration(&ir.interfaces);
    let blocked_handshake_name_fallback =
        interface_signals_with_blocked_handshake_name_fallback(&ir.interfaces);
    let with_resolved_semantic_role =
        interface_signals_with_resolved_semantic_role_count(&ir.interfaces);
    let with_semantic_consensus = interface_signals_with_semantic_consensus_count(&ir.interfaces);
    let with_high_confidence_semantic_consensus =
        interface_signals_with_high_confidence_semantic_consensus_count(&ir.interfaces);
    let with_alias_dependent_semantic_consensus =
        interface_signals_with_alias_dependent_semantic_consensus_count(&ir.interfaces);
    let alias_dependent_semantic_consensus_signal_names =
        interface_signals_with_alias_dependent_semantic_consensus(&ir.interfaces);
    let with_prior_guided_semantic_consensus =
        interface_signals_with_prior_guided_semantic_consensus_count(&ir.interfaces);
    let prior_guided_semantic_consensus_signal_names =
        interface_signals_with_prior_guided_semantic_consensus(&ir.interfaces);
    let alias_dependent_semantic_candidates =
        interface_signal_alias_dependent_semantic_candidates_count(&ir.interfaces);
    let resolved_semantic_roles_without_consensus =
        resolved_semantic_roles_without_consensus_count(&ir.interfaces);
    let resolved_semantic_roles_without_consensus_signal_names =
        resolved_semantic_roles_without_consensus_signal_names(&ir.interfaces);
    let with_single_source_semantic_grounding =
        interface_signals_with_semantic_grounding_strength_count(
            &ir.interfaces,
            crate::ir::semantic::SemanticGroundingStrength::SingleSource,
        );
    let with_multi_source_semantic_grounding =
        interface_signals_with_semantic_grounding_strength_count(
            &ir.interfaces,
            crate::ir::semantic::SemanticGroundingStrength::MultiSource,
        );
    let with_cross_modality_semantic_grounding =
        interface_signals_with_semantic_grounding_strength_count(
            &ir.interfaces,
            crate::ir::semantic::SemanticGroundingStrength::CrossModality,
        );
    let with_visual_semantic_grounding =
        interface_signals_with_visual_semantic_grounding_count(&ir.interfaces);
    let infrastructure_signal_connectivity =
        infrastructure_signal_connectivity_count(&ir.signal_connectivity);
    let infrastructure_signals_unresolved_source = infrastructure_signals_with_source_status_count(
        &ir.infrastructure_signals,
        InfrastructureSignalSourceStatus::UnresolvedSource,
    );
    let infrastructure_signals_recovered_source = infrastructure_signals_with_source_status_count(
        &ir.infrastructure_signals,
        InfrastructureSignalSourceStatus::RecoveredProducer,
    );
    let infrastructure_signals_multiple_source = infrastructure_signals_with_source_status_count(
        &ir.infrastructure_signals,
        InfrastructureSignalSourceStatus::MultipleRecoveredProducers,
    );
    let infrastructure_signals_no_recovered_distribution =
        infrastructure_signals_with_distribution_status_count(
            &ir.infrastructure_signals,
            InfrastructureSignalDistributionStatus::NoRecoveredConsumers,
        );
    let infrastructure_signals_single_recovered_distribution =
        infrastructure_signals_with_distribution_status_count(
            &ir.infrastructure_signals,
            InfrastructureSignalDistributionStatus::SingleRecoveredConsumer,
        );
    let infrastructure_signals_shared_recovered_distribution =
        infrastructure_signals_with_distribution_status_count(
            &ir.infrastructure_signals,
            InfrastructureSignalDistributionStatus::SharedRecoveredConsumers,
        );
    let infrastructure_topology_records =
        infrastructure_topology_count(&ir.infrastructure_signals, None);
    let infrastructure_clock_gated_branches = infrastructure_topology_count(
        &ir.infrastructure_signals,
        Some(InfrastructureTopologyKind::ClockGatedBranch),
    );
    let infrastructure_reset_synchronizer_stages = infrastructure_topology_count(
        &ir.infrastructure_signals,
        Some(InfrastructureTopologyKind::ResetSynchronizerStages),
    );
    let infrastructure_reset_tree_targets = infrastructure_topology_count(
        &ir.infrastructure_signals,
        Some(InfrastructureTopologyKind::ResetTreeTargets),
    );
    let initial_regular_states = initial_regular_states_count(&ir.regular_states);
    let dir_pct = percentage_or_zero(with_direction, declared_count);
    let w_pct = percentage_or_zero(with_width, declared_count);
    let graph_dir_pct = percentage_or_zero(with_graph_direction, declared_count);
    let compat_dir_pct = percentage_or_zero(with_compat_direction_hint, declared_count);
    let table_support_pct = percentage_or_zero(with_table_support, declared_count);
    println!("  declared_signal_records: {declared_count}");
    println!("  heuristic_signal_records (excluded from coverage): {heuristic_signals}");
    println!("  with_resolved_direction: {with_direction} ({dir_pct}%)");
    println!("  with_graph_direction: {with_graph_direction} ({graph_dir_pct}%)");
    println!(
        "  graph_direction_conflicts: {}",
        graph_direction_conflicts.len()
    );
    println!("  with_compat_direction_hint: {with_compat_direction_hint} ({compat_dir_pct}%)");
    println!(
        "  with_width: {with_width} ({w_pct}%) [{with_numeric_width} numeric, {with_parametric_width} parametric]"
    );
    println!("  with_table_support: {with_table_support} ({table_support_pct}%)");
    println!("  with_resolved_polarity: {with_resolved_polarity}");
    println!("  with_semantic_tags: {with_semantic_tags}");
    println!("  semantic_candidates: {semantic_candidates}");
    println!("  with_semantic_candidates: {with_semantic_candidates}");
    println!("  with_multiple_semantic_candidates: {with_multiple_semantic_candidates}");
    println!("  with_semantic_arbitration: {with_semantic_arbitration}");
    println!("  with_decisive_semantic_arbitration: {with_decisive_semantic_arbitration}");
    println!("  with_non_decisive_semantic_arbitration: {with_non_decisive_semantic_arbitration}");
    println!("  with_prior_guided_semantic_arbitration: {with_prior_guided_semantic_arbitration}");
    println!(
        "  with_blocked_handshake_name_fallback: {}",
        blocked_handshake_name_fallback.len()
    );
    println!("  with_resolved_semantic_role: {with_resolved_semantic_role}");
    println!("  with_semantic_consensus: {with_semantic_consensus}");
    println!(
        "  with_high_confidence_semantic_consensus: {with_high_confidence_semantic_consensus}"
    );
    println!(
        "  with_alias_dependent_semantic_consensus: {with_alias_dependent_semantic_consensus}"
    );
    println!("  with_prior_guided_semantic_consensus: {with_prior_guided_semantic_consensus}");
    println!("  alias_dependent_semantic_candidates: {alias_dependent_semantic_candidates}");
    println!(
        "  resolved_semantic_roles_without_consensus: {resolved_semantic_roles_without_consensus}"
    );
    println!("  semantic_observations: {semantic_observations}");
    println!("  with_single_source_semantic_grounding: {with_single_source_semantic_grounding}");
    println!("  with_multi_source_semantic_grounding: {with_multi_source_semantic_grounding}");
    println!("  with_cross_modality_semantic_grounding: {with_cross_modality_semantic_grounding}");
    println!("  with_visual_semantic_grounding: {with_visual_semantic_grounding}");
    println!();

    println!("=== Intent Records ===");
    println!("  actors: {}", ir.actors.len());
    println!(
        "  actor_signal_relations: {}",
        ir.actor_signal_relations.len()
    );
    println!("  actor_ports: {}", ir.actor_ports.len());
    println!("  signal_connectivity: {}", ir.signal_connectivity.len());
    println!("  infrastructure_signal_connectivity: {infrastructure_signal_connectivity}");
    println!(
        "  infrastructure_signals: {}",
        ir.infrastructure_signals.len()
    );
    println!(
        "  infrastructure_signals_unresolved_source: {infrastructure_signals_unresolved_source}"
    );
    println!(
        "  infrastructure_signals_recovered_source: {infrastructure_signals_recovered_source}"
    );
    println!("  infrastructure_signals_multiple_source: {infrastructure_signals_multiple_source}");
    println!(
        "  infrastructure_signals_no_recovered_distribution: {infrastructure_signals_no_recovered_distribution}"
    );
    println!(
        "  infrastructure_signals_single_recovered_distribution: {infrastructure_signals_single_recovered_distribution}"
    );
    println!(
        "  infrastructure_signals_shared_recovered_distribution: {infrastructure_signals_shared_recovered_distribution}"
    );
    println!("  infrastructure_topology_records: {infrastructure_topology_records}");
    println!("  infrastructure_clock_gated_branches: {infrastructure_clock_gated_branches}");
    println!(
        "  infrastructure_reset_synchronizer_stages: {infrastructure_reset_synchronizer_stages}"
    );
    println!("  infrastructure_reset_tree_targets: {infrastructure_reset_tree_targets}");
    println!(
        "  interface_signal_conflicts: {}",
        ir.interface_signal_conflicts.len()
    );
    println!(
        "  signal_connectivity_conflicts: {}",
        ir.signal_connectivity_conflicts.len()
    );
    println!(
        "  signal_polarity_conflicts: {}",
        ir.signal_polarity_conflicts.len()
    );
    println!(
        "  signal_semantic_conflicts: {}",
        ir.signal_semantic_conflicts.len()
    );
    println!("  behaviors: {}", ir.behaviors.len());
    println!("  constraints: {}", ir.constraints.len());
    println!("  assumptions: {}", ir.assumptions.len());
    println!("  symbol_definitions: {}", ir.symbol_definitions.len());
    println!("  regular_states: {}", ir.regular_states.len());
    println!("  initial_regular_states: {initial_regular_states}");
    println!("  state_transitions: {}", ir.state_transitions.len());
    println!("  register_records: {}", ir.register_records.len());
    println!("  timing_constraints: {}", ir.timing_constraints.len());
    println!("  temporal_rules: {}", ir.temporal_rules.len());
    println!("  temporal_conflicts: {}", ir.temporal_conflicts.len());
    println!(
        "  temporal_rules_with_actor_grounding: {}",
        temporal_rules_with_actor_grounding_count(&ir.temporal_rules)
    );
    println!(
        "  temporal_rules_with_handshake_completion: {}",
        temporal_rules_with_handshake_completion_count(&ir.temporal_rules)
    );
    println!(
        "  temporal_rules_with_alias_dependent_handshake_completion: {}",
        temporal_rules_with_alias_dependent_handshake_completion_count(
            &ir.temporal_rules,
            &ir.interfaces,
        )
    );
    println!("  signal_constraints: {}", ir.signal_constraints.len());
    println!("  conditional_rules: {}", ir.conditional_rules.len());
    println!();

    // Layer E: spec-type-aware quality scoring.
    //
    // Points breakdown (total max = 100):
    //   Signal direction coverage (declared only): 0–25 pts
    //   Signal width coverage    (declared only):  0–10 pts
    //   NLP constraint richness  (sig + cond):     0–30 pts  (capped at 30 constraints)
    //   Encoding enum definitions:                 0–15 pts
    //   Register map records:                      0–5  pts
    //   Timing constraint records:                 0–5  pts
    //   State machine (FSM specs):                 0–5  pts  (bonus, not penalised if absent)
    //   System contract (clock/reset):             0–5  pts  (bonus, not penalised if absent)
    //
    // The FSM/contract bonuses are additive rather than penalties so that bus protocol
    // specs like AHB (which have no FSM or explicit clock declaration) are scored on the
    // merit of what they *do* contain rather than penalised for spec-appropriate omissions.
    let has_states = !ir.regular_states.is_empty();
    let has_enums = !ir.symbol_definitions.is_empty();
    let has_constraints = ir.signal_constraints.len() + ir.conditional_rules.len();
    let has_system_contract = ir.system_contract.is_some();
    let has_registers = !ir.register_records.is_empty();
    let has_timing = !ir.timing_constraints.is_empty();

    let dir_score = dir_pct as f64 * 0.25; // 0–25
    let width_score = w_pct as f64 * 0.10; // 0–10
    let constraint_score = has_constraints.min(30) as f64; // 0–30
    let enum_score = if has_enums { 15.0_f64 } else { 0.0 }; // 0–15
    let register_score = if has_registers { 5.0_f64 } else { 0.0 }; // 0–5
    let timing_score = if has_timing { 5.0_f64 } else { 0.0 }; // 0–5
    let fsm_score = if has_states { 5.0_f64 } else { 0.0 }; // 0–5  (bonus)
    let contract_score = if has_system_contract { 5.0_f64 } else { 0.0 }; // 0–5  (bonus)

    let score = (dir_score
        + width_score
        + constraint_score
        + enum_score
        + register_score
        + timing_score
        + fsm_score
        + contract_score)
        .min(100.0);
    let quality_gap_related_ids = intent_quality_gap_related_ids(IntentQualityScoreInputs {
        dir_pct,
        width_pct: w_pct,
        structured_nlp_constraints: has_constraints,
        has_encoding_enums: has_enums,
        has_register_map: has_registers,
        has_timing_constraints: has_timing,
        has_state_machine: has_states,
        has_system_contract,
    });

    println!("=== Quality Score ===");
    println!(
        "  signal_direction_coverage: {dir_pct}% (declared signals only, graph-first with compatibility fallback)"
    );
    println!("  graph_direction_coverage: {graph_dir_pct}% (declared signals only)");
    println!("  compatibility_direction_hints: {compat_dir_pct}% (declared signals only)");
    println!("  signal_width_coverage: {w_pct}% (declared signals only)");
    println!("  has_encoding_enums: {has_enums}");
    println!("  has_register_map: {has_registers}");
    println!("  has_timing_constraints: {has_timing}");
    println!("  has_state_machine: {has_states}");
    println!("  has_system_contract: {has_system_contract}");
    println!("  structured_nlp_constraints: {has_constraints}");
    println!("  residual_decisions: {}", ir.residual_decisions.len());
    println!();
    println!("  score_breakdown:");
    println!("    signal_direction:  {dir_score:.1}/25");
    println!("    signal_width:      {width_score:.1}/10");
    println!("    nlp_constraints:   {constraint_score:.0}/30");
    println!("    encoding_enums:    {enum_score:.0}/15");
    println!("    register_map:      {register_score:.0}/5");
    println!("    timing:            {timing_score:.0}/5");
    println!("    fsm_bonus:         {fsm_score:.0}/5");
    println!("    contract_bonus:    {contract_score:.0}/5");
    let grade = match score as u32 {
        90..=100 => "EXCELLENT",
        70..=89 => "GOOD",
        50..=69 => "ADEQUATE",
        30..=49 => "NEEDS IMPROVEMENT",
        _ => "INCOMPLETE",
    };
    println!("  overall_score: {score:.0}/100 — {grade}");
    println!();

    println!("=== Infrastructure Signals ===");
    if ir.infrastructure_signals.is_empty() {
        println!("  none");
    } else {
        for signal in &ir.infrastructure_signals {
            println!(
                "  - {} ({}): source={}, distribution={}, distributed_to={}, topology={}",
                signal.signal_name,
                describe_infrastructure_signal_kind(signal.kind),
                describe_infrastructure_source_status(signal.source_status),
                describe_infrastructure_distribution_status(signal.distribution_status),
                signal.distributed_to_actor_names.len(),
                signal.infrastructure_topology.len()
            );
            for topology in &signal.infrastructure_topology {
                println!(
                    "    - {}: component={}, stages={}, targets={}",
                    describe_infrastructure_topology_kind(topology.topology_kind),
                    topology.component_name.as_deref().unwrap_or("n/a"),
                    topology
                        .stage_count
                        .map(|count| count.to_string())
                        .unwrap_or_else(|| "n/a".to_string()),
                    topology.target_actor_names.len()
                );
            }
        }
    }
    println!();

    println!("=== Residual Decisions ===");
    println!("  count: {}", ir.residual_decisions.len());
    for rd in &ir.residual_decisions {
        println!("  - [{}] {}", rd.packet_id, rd.question);
    }
    if !ir.signal_connectivity_conflicts.is_empty() {
        println!();
        println!("=== Signal Connectivity Conflicts ===");
        for conflict in ir.signal_connectivity_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_signal_connectivity_conflict(conflict)
            );
        }
        if ir.signal_connectivity_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.signal_connectivity_conflicts.len() - 8
            );
        }
    }
    if !ir.interface_signal_conflicts.is_empty() {
        println!();
        println!("=== Interface Signal Conflicts ===");
        for conflict in ir.interface_signal_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_interface_signal_conflict(conflict)
            );
        }
        if ir.interface_signal_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.interface_signal_conflicts.len() - 8
            );
        }
    }
    if !ir.signal_polarity_conflicts.is_empty() {
        println!();
        println!("=== Signal Polarity Conflicts ===");
        for conflict in ir.signal_polarity_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_signal_polarity_conflict(conflict)
            );
        }
        if ir.signal_polarity_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.signal_polarity_conflicts.len() - 8
            );
        }
    }
    if !ir.signal_semantic_conflicts.is_empty() {
        println!();
        println!("=== Signal Semantic Conflicts ===");
        for conflict in ir.signal_semantic_conflicts.iter().take(8) {
            println!(
                "  - {}: {}",
                conflict.signal_name,
                describe_signal_semantic_conflict(conflict)
            );
        }
        if ir.signal_semantic_conflicts.len() > 8 {
            println!(
                "  ... and {} more conflict(s)",
                ir.signal_semantic_conflicts.len() - 8
            );
        }
    }

    let negative_knowledge_prior_matches = intent_negative_knowledge_prior_matches(ir);
    println!();
    println!("=== Negative Knowledge Priors ===");
    println!(
        "  matched_carried_conflict_or_residual_patterns: {}",
        negative_knowledge_prior_matches.len()
    );
    println!(
        "  rescan_recommendations: {}",
        negative_knowledge_prior_matches.len()
    );
    println!(
        "  stronger_corroboration_requirements: {}",
        negative_knowledge_prior_matches.len()
    );

    let missing_producer_signals = protocol_missing_producer_signal_names(&ir.signal_connectivity);
    let infrastructure_missing_producer_signals =
        infrastructure_missing_producer_signal_names(&ir.signal_connectivity);
    let infrastructure_signal_connectivity =
        infrastructure_signal_connectivity_count(&ir.signal_connectivity);
    let missing_consumer_signals: Vec<String> = ir
        .signal_connectivity
        .iter()
        .filter(|record| record.consumer_actor_ids.is_empty())
        .map(|record| record.signal_name.clone())
        .collect();
    let missing_graph_direction_count = missing_graph_direction_signal_names.len();
    let missing_temporal_clock_grounding =
        temporal_rules_missing_clock_grounding_count(&ir.temporal_rules);
    let temporal_rules_with_cycle_window =
        temporal_rules_with_cycle_window_count(&ir.temporal_rules);
    let temporal_rules_with_actor_grounding =
        temporal_rules_with_actor_grounding_count(&ir.temporal_rules);
    let temporal_rules_missing_clock_grounding_rule_ids =
        temporal_rules_missing_clock_grounding_rule_ids(&ir.temporal_rules);
    let temporal_rules_missing_cycle_window_rule_ids =
        temporal_rules_missing_cycle_window_rule_ids(&ir.temporal_rules);
    let temporal_rules_missing_actor_grounding_rule_ids =
        temporal_rules_missing_actor_grounding_rule_ids(&ir.temporal_rules);
    let temporal_rules_with_handshake_completion =
        temporal_rules_with_handshake_completion_count(&ir.temporal_rules);
    let temporal_rules_with_alias_dependent_handshake_completion =
        temporal_rules_with_alias_dependent_handshake_completion_count(
            &ir.temporal_rules,
            &ir.interfaces,
        );
    let temporal_rule_surface_input_ids = temporal_rule_surface_input_ids(
        &ir.timing_constraints,
        &ir.signal_constraints,
        &ir.conditional_rules,
    );
    let alias_dependent_handshake_completion_signal_names =
        temporal_rules_with_alias_dependent_handshake_completion_signal_names(
            &ir.temporal_rules,
            &ir.interfaces,
        );
    let temporal_rules_with_multi_predicate_antecedents =
        temporal_rules_with_multi_predicate_antecedents_count(&ir.temporal_rules);
    let handshake_role_signal_names =
        interface_signals_with_handshake_semantic_role_names(&ir.interfaces);
    let multi_predicate_antecedent_rule_ids =
        temporal_rules_with_multi_predicate_antecedents_rule_ids(&ir.temporal_rules);
    let actor_signal_relation_related_ids =
        actor_signal_relation_related_ids(&ir.actor_signal_relations);
    let signal_connectivity_conflict_related_ids = ir
        .signal_connectivity_conflicts
        .iter()
        .map(|conflict| conflict.conflict_id.clone())
        .collect::<Vec<_>>();

    let mut findings = Vec::new();
    if !ir.actor_signal_relations.is_empty() && ir.actor_ports.is_empty() {
        let actor_port_gap_related_ids = actor_signal_relation_related_ids
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        findings.push(finding(
            "intent_actor_ports_missing",
            ValidationFindingSeverity::Error,
            "knowledge_graph",
            "IntentIR carries actor-signal relations but no actor-relative ports",
            actor_port_gap_related_ids.clone(),
        ));
        push_actor_port_gap_rescan_guidance(
            &mut findings,
            INTENT_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &actor_port_gap_related_ids,
        );
    }
    if !missing_producer_signals.is_empty() {
        findings.push(finding(
            "intent_connectivity_missing_producer",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal(s) in IntentIR connectivity have no resolved producer actor",
                missing_producer_signals.len()
            ),
            missing_producer_signals.iter().take(8).cloned().collect(),
        ));
        push_connectivity_gap_rescan_guidance(
            &mut findings,
            INTENT_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            "producer-side",
            &missing_producer_signals
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !infrastructure_missing_producer_signals.is_empty() {
        findings.push(finding(
            "intent_infrastructure_connectivity_missing_producer",
            ValidationFindingSeverity::Info,
            "system_contract",
            format!(
                "{} infrastructure signal(s) have no resolved producer actor in IntentIR connectivity; canonical sourcing status remains explicit in the infrastructure/system-contract surface",
                infrastructure_missing_producer_signals.len()
            ),
            infrastructure_missing_producer_signals
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !missing_consumer_signals.is_empty() {
        findings.push(finding(
            "intent_connectivity_missing_consumer",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal(s) in IntentIR connectivity have no resolved consumer actor",
                missing_consumer_signals.len()
            ),
            missing_consumer_signals.iter().take(8).cloned().collect(),
        ));
        push_connectivity_gap_rescan_guidance(
            &mut findings,
            INTENT_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            "consumer-side",
            &missing_consumer_signals
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.signal_connectivity_conflicts.is_empty() {
        findings.push(finding(
            "intent_signal_connectivity_conflicts_present",
            ValidationFindingSeverity::Warning,
            "signal_connectivity",
            format!(
                "{} signal connectivity conflict(s) detected; the carried structural KG still has unresolved producer ambiguity",
                ir.signal_connectivity_conflicts.len()
            ),
            signal_connectivity_conflict_related_ids.clone(),
        ));
        push_signal_connectivity_conflict_rescan_guidance(
            &mut findings,
            INTENT_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &signal_connectivity_conflict_related_ids,
        );
    }
    if !ir.interface_signal_conflicts.is_empty() {
        let interface_signal_conflict_related_ids = ir
            .interface_signal_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "intent_interface_signal_conflicts_present",
            ValidationFindingSeverity::Warning,
            "interface_signal_conflicts",
            format!(
                "{} interface signal conflict(s) detected; conflicting direction/width evidence is still unresolved in the carried interface surface",
                ir.interface_signal_conflicts.len()
            ),
            interface_signal_conflict_related_ids.clone(),
        ));
        push_interface_signal_conflict_rescan_guidance(
            &mut findings,
            INTENT_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &interface_signal_conflict_related_ids,
        );
    }
    if !ir.signal_polarity_conflicts.is_empty() {
        let signal_polarity_conflict_related_ids = ir
            .signal_polarity_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "intent_signal_polarity_conflicts_present",
            ValidationFindingSeverity::Warning,
            "polarity_conflicts",
            format!(
                "{} signal polarity conflict(s) remain unresolved in the carried canonical intent surface",
                ir.signal_polarity_conflicts.len()
            ),
            signal_polarity_conflict_related_ids.clone(),
        ));
        push_signal_polarity_conflict_rescan_guidance(
            &mut findings,
            INTENT_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &signal_polarity_conflict_related_ids,
        );
    }
    if !ir.signal_semantic_conflicts.is_empty() {
        let signal_semantic_conflict_related_ids = ir
            .signal_semantic_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "intent_signal_semantic_conflicts_present",
            ValidationFindingSeverity::Warning,
            "semantic_role_conflicts",
            format!(
                "{} semantic-role conflict(s) remain unresolved in the carried canonical intent surface",
                ir.signal_semantic_conflicts.len()
            ),
            signal_semantic_conflict_related_ids.clone(),
        ));
        push_signal_semantic_conflict_rescan_guidance(
            &mut findings,
            INTENT_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &signal_semantic_conflict_related_ids,
        );
    }
    if with_non_decisive_semantic_arbitration > 0 {
        findings.push(finding(
            "intent_non_decisive_semantic_arbitration_present",
            ValidationFindingSeverity::Info,
            "semantic_role_arbitration",
            format!(
                "{with_non_decisive_semantic_arbitration} declared signal(s) still have competing semantic role candidates; the canonical surface preserves the current lead, runner-up, and evidence margin without forcing an unsafe winner"
            ),
            non_decisive_semantic_arbitration_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_semantic_role_arbitration_rescan_guidance(
            &mut findings,
            INTENT_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &non_decisive_semantic_arbitration_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if with_prior_guided_semantic_arbitration > 0 {
        findings.push(finding(
            "intent_prior_guided_semantic_arbitration_present",
            ValidationFindingSeverity::Info,
            "semantic_role_arbitration",
            format!(
                "{with_prior_guided_semantic_arbitration} declared signal(s) use learned modality-reliability priors to resolve otherwise competing local semantic-role evidence"
            ),
            prior_guided_semantic_arbitration_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !blocked_handshake_name_fallback.is_empty() {
        findings.push(finding(
            "intent_handshake_name_fallback_blocked_present",
            ValidationFindingSeverity::Info,
            "semantic_role_arbitration",
            format!(
                "{} handshake-shaped signal(s) intentionally block literal VALID/READY fallback because their preserved semantic role state is still contested or only provisional",
                blocked_handshake_name_fallback.len()
            ),
            blocked_handshake_name_fallback.clone(),
        ));
    }
    if resolved_semantic_roles_without_consensus > 0 {
        findings.push(finding(
            "intent_resolved_roles_without_consensus_present",
            ValidationFindingSeverity::Info,
            "semantic_role_consensus",
            format!(
                "{resolved_semantic_roles_without_consensus} resolved semantic role(s) still rely on fallback carry-through without an explicit canonical consensus profile"
            ),
            resolved_semantic_roles_without_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_semantic_role_consensus_rescan_guidance(
            &mut findings,
            INTENT_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &resolved_semantic_roles_without_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if with_alias_dependent_semantic_consensus > 0 {
        findings.push(finding(
            "intent_alias_dependent_semantic_consensus_present",
            ValidationFindingSeverity::Info,
            "semantic_role_consensus",
            format!(
                "{with_alias_dependent_semantic_consensus} resolved semantic role(s) currently depend only on alias-grounded evidence rather than direct signal mentions or corroborating non-alias modalities"
            ),
            alias_dependent_semantic_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_alias_dependent_semantic_consensus_rescan_guidance(
            &mut findings,
            INTENT_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &alias_dependent_semantic_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if with_prior_guided_semantic_consensus > 0 {
        findings.push(finding(
            "intent_prior_guided_semantic_consensus_present",
            ValidationFindingSeverity::Info,
            "semantic_role_consensus",
            format!(
                "{with_prior_guided_semantic_consensus} resolved semantic role(s) now carry explicit consensus that was strengthened by learned modality-reliability priors"
            ),
            prior_guided_semantic_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_prior_guided_semantic_consensus_rescan_guidance(
            &mut findings,
            INTENT_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &prior_guided_semantic_consensus_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.actor_ports.is_empty() && missing_graph_direction_count > 0 {
        findings.push(finding(
            "intent_graph_direction_coverage_incomplete",
            ValidationFindingSeverity::Info,
            "knowledge_graph",
            format!(
                "{missing_graph_direction_count} declared signal record(s) still lack graph-derived direction coverage"
            ),
            missing_graph_direction_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_graph_direction_coverage_rescan_guidance(
            &mut findings,
            INTENT_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &missing_graph_direction_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !graph_direction_conflicts.is_empty() {
        findings.push(finding(
            "intent_graph_direction_conflicts_present",
            ValidationFindingSeverity::Warning,
            "knowledge_graph",
            format!(
                "{} signal(s) have conflicting actor-relative directions from the same actor across {} actor-signal conflict record(s); graph-direction coverage remains intentionally unresolved until upstream graph evidence is clarified",
                graph_direction_conflicts.len(),
                graph_direction_summary.conflicts.len()
            ),
            graph_direction_conflict_related_ids.clone(),
        ));
        push_graph_direction_conflict_rescan_guidance(
            &mut findings,
            INTENT_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &graph_direction_conflict_related_ids,
        );
    }
    if !graph_backed_missing_compat_direction_signal_names.is_empty() {
        findings.push(finding(
            "intent_compat_direction_hints_lag_graph",
            ValidationFindingSeverity::Info,
            "compatibility_surface",
            format!(
                "{} declared signal record(s) still lack flat compatibility direction hints even though actor-relative ports exist",
                graph_backed_missing_compat_direction_signal_names.len()
            ),
            graph_backed_missing_compat_direction_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !unresolved_missing_compat_direction_signal_names.is_empty() {
        findings.push(finding(
            "intent_compat_direction_hints_incomplete",
            ValidationFindingSeverity::Info,
            "compatibility_surface",
            format!(
                "{} declared signal record(s) still lack flat compatibility direction hints and actor-relative graph coverage",
                unresolved_missing_compat_direction_signal_names.len()
            ),
            unresolved_missing_compat_direction_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !ir.temporal_rules.is_empty() && missing_temporal_clock_grounding > 0 {
        findings.push(finding(
            "intent_temporal_rules_missing_clock_grounding",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            format!(
                "{missing_temporal_clock_grounding} temporal rule(s) still lack explicit clock or edge grounding"
            ),
            temporal_rules_missing_clock_grounding_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_temporal_clock_grounding_rescan_guidance(
            &mut findings,
            INTENT_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &temporal_rules_missing_clock_grounding_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.temporal_rules.is_empty() && temporal_rules_with_cycle_window == 0 {
        findings.push(finding(
            "intent_temporal_rules_missing_cycle_windows",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            "typed temporal rules exist, but none currently carry explicit cycle-window bounds"
                .to_string(),
            temporal_rules_missing_cycle_window_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_temporal_cycle_window_rescan_guidance(
            &mut findings,
            INTENT_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &temporal_rules_missing_cycle_window_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.temporal_rules.is_empty()
        && !ir.actor_signal_relations.is_empty()
        && temporal_rules_with_actor_grounding == 0
    {
        findings.push(finding(
            "intent_temporal_rules_missing_actor_grounding",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            "typed temporal rules exist, but none currently carry actor-relative drive/sample grounding"
                .to_string(),
            temporal_rules_missing_actor_grounding_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
        push_temporal_actor_grounding_rescan_guidance(
            &mut findings,
            INTENT_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &temporal_rules_missing_actor_grounding_rule_ids
                .iter()
                .take(8)
                .cloned()
                .collect::<Vec<_>>(),
        );
    }
    if !ir.temporal_conflicts.is_empty() {
        let temporal_conflict_related_ids = ir
            .temporal_conflicts
            .iter()
            .map(|conflict| conflict.conflict_id.clone())
            .collect::<Vec<_>>();
        findings.push(finding(
            "intent_temporal_conflicts_present",
            ValidationFindingSeverity::Warning,
            "temporal_conflicts",
            format!(
                "{} typed temporal conflict(s) detected across contradictory value obligations",
                ir.temporal_conflicts.len()
            ),
            temporal_conflict_related_ids.clone(),
        ));
        push_temporal_conflict_rescan_guidance(
            &mut findings,
            INTENT_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &temporal_conflict_related_ids,
        );
    }
    if !ir.regular_states.is_empty() && initial_regular_states != 1 {
        findings.push(finding(
            "intent_state_machine_initial_cardinality",
            ValidationFindingSeverity::Warning,
            "state_machine",
            format!(
                "IntentIR state machine has {initial_regular_states} canonical initial state(s); expected exactly one when regular states are present"
            ),
            state_machine_initial_cardinality_related_ids(&ir.regular_states),
        ));
    }
    if temporal_rules_with_alias_dependent_handshake_completion > 0 {
        findings.push(finding(
            "intent_alias_dependent_handshake_completion_present",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            format!(
                "{temporal_rules_with_alias_dependent_handshake_completion} typed temporal rule(s) currently derive HandshakeComplete predicates from alias-dependent semantic role consensus"
            ),
            alias_dependent_handshake_completion_signal_names
                .iter()
                .take(8)
                .cloned()
                .collect(),
        ));
    }
    if !ir.temporal_rules.is_empty()
        && !handshake_role_signal_names.is_empty()
        && temporal_rules_with_handshake_completion == 0
    {
        let handshake_completion_gap_signal_ids = handshake_role_signal_names
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        findings.push(finding(
            "intent_temporal_handshake_completion_gap",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            format!(
                "{} declared signal(s) carry handshake semantic roles (ValidLike/ReadyLike) but no typed temporal rule currently expresses a HandshakeComplete predicate",
                handshake_role_signal_names.len()
            ),
            handshake_completion_gap_signal_ids.clone(),
        ));
        push_temporal_handshake_completion_gap_rescan_guidance(
            &mut findings,
            INTENT_TEMPORAL_HANDSHAKE_COMPLETION_GAP_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &handshake_completion_gap_signal_ids,
        );
    }
    if temporal_rules_with_multi_predicate_antecedents > 0 {
        let multi_pred_rule_ids = multi_predicate_antecedent_rule_ids
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        findings.push(finding(
            "intent_temporal_multi_predicate_antecedents_present",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            format!(
                "{temporal_rules_with_multi_predicate_antecedents} typed temporal rule(s) carry multi-predicate antecedents; compound condition extraction may benefit from further review"
            ),
            multi_pred_rule_ids.clone(),
        ));
        push_temporal_multi_predicate_antecedents_rescan_guidance(
            &mut findings,
            INTENT_TEMPORAL_MULTI_PREDICATE_ANTECEDENTS_SURFACE_RESCAN_GUIDANCE,
            "IntentIR",
            &multi_pred_rule_ids,
        );
    }
    // KG-quality benchmarks — flag when key dimensions fall below defined thresholds.
    let graph_direction_coverage_pct = percentage_or_zero(with_graph_direction, declared_count);
    if declared_count > 0 && graph_direction_coverage_pct < 50 {
        findings.push(finding(
            "intent_kg_graph_direction_coverage_below_benchmark",
            ValidationFindingSeverity::Info,
            "kg_quality_benchmark",
            format!(
                "graph direction coverage is {graph_direction_coverage_pct}% ({with_graph_direction}/{declared_count}) — below the 50% KG-quality benchmark"
            ),
            Vec::new(),
        ));
    }
    let semantic_role_resolution_pct =
        percentage_or_zero(with_resolved_semantic_role, declared_count);
    if declared_count > 0 && semantic_role_resolution_pct < 30 {
        findings.push(finding(
            "intent_kg_semantic_role_resolution_below_benchmark",
            ValidationFindingSeverity::Info,
            "kg_quality_benchmark",
            format!(
                "semantic role resolution rate is {semantic_role_resolution_pct}% ({with_resolved_semantic_role}/{declared_count}) — below the 30% KG-quality benchmark"
            ),
            Vec::new(),
        ));
    }
    let semantic_consensus_coverage_pct = if with_resolved_semantic_role > 0 {
        ((with_semantic_consensus as f64 / with_resolved_semantic_role as f64) * 100.0).round()
            as usize
    } else {
        0
    };
    if with_resolved_semantic_role > 0 && semantic_consensus_coverage_pct < 50 {
        findings.push(finding(
            "intent_kg_semantic_consensus_coverage_below_benchmark",
            ValidationFindingSeverity::Info,
            "kg_quality_benchmark",
            format!(
                "semantic consensus coverage is {semantic_consensus_coverage_pct}% ({with_semantic_consensus}/{with_resolved_semantic_role}) — below the 50% KG-quality benchmark"
            ),
            Vec::new(),
        ));
    }
    if ir.temporal_rules.is_empty()
        && (!ir.timing_constraints.is_empty()
            || !ir.signal_constraints.is_empty()
            || !ir.conditional_rules.is_empty())
    {
        let temporal_rule_surface_related_ids = temporal_rule_surface_input_ids
            .iter()
            .take(8)
            .cloned()
            .collect::<Vec<_>>();
        findings.push(finding(
            "intent_temporal_rule_surface_missing",
            ValidationFindingSeverity::Info,
            "temporal_grounding",
            "intent evidence includes timing/constraint records but no typed temporal rules were carried forward"
                .to_string(),
            temporal_rule_surface_related_ids.clone(),
        ));
        push_temporal_rule_surface_rescan_guidance(
            &mut findings,
            "intent_temporal_rule_surface_rescan_guidance",
            "IntentIR",
            &temporal_rule_surface_related_ids,
        );
    }
    if score < 90.0 {
        findings.push(finding(
            "intent_quality_below_excellent_threshold",
            ValidationFindingSeverity::Warning,
            "quality_score",
            format!("IntentIR quality score is {score:.0}/100 ({grade})"),
            quality_gap_related_ids,
        ));
    }
    if !ir.residual_decisions.is_empty() {
        findings.push(finding(
            "intent_residual_decisions_present",
            ValidationFindingSeverity::Warning,
            "residual_decisions",
            format!(
                "IntentIR still carries {} residual decision packet(s)",
                ir.residual_decisions.len()
            ),
            ir.residual_decisions
                .iter()
                .map(|packet| packet.packet_id.clone())
                .collect(),
        ));
    }
    if !negative_knowledge_prior_matches.is_empty() {
        findings.push(finding(
            "intent_negative_knowledge_prior_matches",
            ValidationFindingSeverity::Info,
            "negative_knowledge",
            format!(
                "{} carried conflict/residual pattern(s) match prior negative knowledge; this is a caution signal only, not an override of current intent evidence",
                negative_knowledge_prior_matches.len()
            ),
            negative_knowledge_prior_matches.clone(),
        ));
    }
    push_negative_knowledge_rescan_guidance(
        &mut findings,
        "intent_negative_knowledge_rescan_guidance",
        "IntentIR",
        &negative_knowledge_prior_matches,
    );

    let report = ValidationReportRecord {
        report_id: format!("validation_intent_ir_{artifact_fingerprint}"),
        validated_stage: IrStage::IntentIr,
        artifact_fingerprint,
        summary: format!(
            "IntentIR validation for {} with {} finding(s)",
            ir.document_identity.display_name,
            findings.len()
        ),
        overall_score: Some(score.round() as u32),
        grade: Some(grade.to_string()),
        metrics: vec![
            metric("declared_signal_records", declared_count.to_string()),
            metric("heuristic_signal_records", heuristic_signals.to_string()),
            metric("with_resolved_direction", with_direction.to_string()),
            metric("with_graph_direction", with_graph_direction.to_string()),
            metric(
                "graph_direction_conflicts",
                graph_direction_conflicts.len().to_string(),
            ),
            metric(
                "with_compat_direction_hint",
                with_compat_direction_hint.to_string(),
            ),
            metric("with_width", with_width.to_string()),
            metric("with_table_support", with_table_support.to_string()),
            metric("with_resolved_polarity", with_resolved_polarity.to_string()),
            metric("with_semantic_tags", with_semantic_tags.to_string()),
            metric("semantic_candidates", semantic_candidates.to_string()),
            metric(
                "with_semantic_candidates",
                with_semantic_candidates.to_string(),
            ),
            metric(
                "with_multiple_semantic_candidates",
                with_multiple_semantic_candidates.to_string(),
            ),
            metric(
                "with_semantic_arbitration",
                with_semantic_arbitration.to_string(),
            ),
            metric(
                "with_decisive_semantic_arbitration",
                with_decisive_semantic_arbitration.to_string(),
            ),
            metric(
                "with_non_decisive_semantic_arbitration",
                with_non_decisive_semantic_arbitration.to_string(),
            ),
            metric(
                "with_prior_guided_semantic_arbitration",
                with_prior_guided_semantic_arbitration.to_string(),
            ),
            metric(
                "with_blocked_handshake_name_fallback",
                blocked_handshake_name_fallback.len().to_string(),
            ),
            metric(
                "with_resolved_semantic_role",
                with_resolved_semantic_role.to_string(),
            ),
            metric(
                "with_semantic_consensus",
                with_semantic_consensus.to_string(),
            ),
            metric(
                "with_high_confidence_semantic_consensus",
                with_high_confidence_semantic_consensus.to_string(),
            ),
            metric(
                "with_alias_dependent_semantic_consensus",
                with_alias_dependent_semantic_consensus.to_string(),
            ),
            metric(
                "with_prior_guided_semantic_consensus",
                with_prior_guided_semantic_consensus.to_string(),
            ),
            metric(
                "alias_dependent_semantic_candidates",
                alias_dependent_semantic_candidates.to_string(),
            ),
            metric(
                "resolved_semantic_roles_without_consensus",
                resolved_semantic_roles_without_consensus.to_string(),
            ),
            metric("semantic_observations", semantic_observations.to_string()),
            metric(
                "with_single_source_semantic_grounding",
                with_single_source_semantic_grounding.to_string(),
            ),
            metric(
                "with_multi_source_semantic_grounding",
                with_multi_source_semantic_grounding.to_string(),
            ),
            metric(
                "with_cross_modality_semantic_grounding",
                with_cross_modality_semantic_grounding.to_string(),
            ),
            metric(
                "with_visual_semantic_grounding",
                with_visual_semantic_grounding.to_string(),
            ),
            metric("actors", ir.actors.len().to_string()),
            metric(
                "actor_signal_relations",
                ir.actor_signal_relations.len().to_string(),
            ),
            metric("actor_ports", ir.actor_ports.len().to_string()),
            metric(
                "signal_connectivity",
                ir.signal_connectivity.len().to_string(),
            ),
            metric(
                "infrastructure_signal_connectivity",
                infrastructure_signal_connectivity.to_string(),
            ),
            metric(
                "infrastructure_signals",
                ir.infrastructure_signals.len().to_string(),
            ),
            metric(
                "infrastructure_signals_unresolved_source",
                infrastructure_signals_unresolved_source.to_string(),
            ),
            metric(
                "infrastructure_signals_recovered_source",
                infrastructure_signals_recovered_source.to_string(),
            ),
            metric(
                "infrastructure_signals_multiple_source",
                infrastructure_signals_multiple_source.to_string(),
            ),
            metric(
                "infrastructure_signals_no_recovered_distribution",
                infrastructure_signals_no_recovered_distribution.to_string(),
            ),
            metric(
                "infrastructure_signals_single_recovered_distribution",
                infrastructure_signals_single_recovered_distribution.to_string(),
            ),
            metric(
                "infrastructure_signals_shared_recovered_distribution",
                infrastructure_signals_shared_recovered_distribution.to_string(),
            ),
            metric(
                "infrastructure_topology_records",
                infrastructure_topology_records.to_string(),
            ),
            metric(
                "infrastructure_clock_gated_branches",
                infrastructure_clock_gated_branches.to_string(),
            ),
            metric(
                "infrastructure_reset_synchronizer_stages",
                infrastructure_reset_synchronizer_stages.to_string(),
            ),
            metric(
                "infrastructure_reset_tree_targets",
                infrastructure_reset_tree_targets.to_string(),
            ),
            metric(
                "infrastructure_signals_missing_producer",
                infrastructure_missing_producer_signals.len().to_string(),
            ),
            metric(
                "interface_signal_conflicts",
                ir.interface_signal_conflicts.len().to_string(),
            ),
            metric(
                "signal_connectivity_conflicts",
                ir.signal_connectivity_conflicts.len().to_string(),
            ),
            metric(
                "signal_polarity_conflicts",
                ir.signal_polarity_conflicts.len().to_string(),
            ),
            metric(
                "signal_semantic_conflicts",
                ir.signal_semantic_conflicts.len().to_string(),
            ),
            metric("behaviors", ir.behaviors.len().to_string()),
            metric("constraints", ir.constraints.len().to_string()),
            metric("assumptions", ir.assumptions.len().to_string()),
            metric(
                "symbol_definitions",
                ir.symbol_definitions.len().to_string(),
            ),
            metric("regular_states", ir.regular_states.len().to_string()),
            metric("initial_regular_states", initial_regular_states.to_string()),
            metric("state_transitions", ir.state_transitions.len().to_string()),
            metric("register_records", ir.register_records.len().to_string()),
            metric(
                "timing_constraints",
                ir.timing_constraints.len().to_string(),
            ),
            metric("temporal_rules", ir.temporal_rules.len().to_string()),
            metric(
                "temporal_conflicts",
                ir.temporal_conflicts.len().to_string(),
            ),
            metric(
                "temporal_rules_with_cycle_window",
                temporal_rules_with_cycle_window.to_string(),
            ),
            metric(
                "temporal_rules_with_actor_grounding",
                temporal_rules_with_actor_grounding.to_string(),
            ),
            metric(
                "temporal_rules_with_handshake_completion",
                temporal_rules_with_handshake_completion.to_string(),
            ),
            metric(
                "temporal_rules_with_alias_dependent_handshake_completion",
                temporal_rules_with_alias_dependent_handshake_completion.to_string(),
            ),
            metric(
                "temporal_rules_with_multi_predicate_antecedents",
                temporal_rules_with_multi_predicate_antecedents.to_string(),
            ),
            metric(
                "temporal_rules_missing_clock_grounding",
                missing_temporal_clock_grounding.to_string(),
            ),
            metric(
                "signal_constraints",
                ir.signal_constraints.len().to_string(),
            ),
            metric("conditional_rules", ir.conditional_rules.len().to_string()),
            metric(
                "residual_decisions",
                ir.residual_decisions.len().to_string(),
            ),
            metric(
                "negative_knowledge_prior_matches",
                negative_knowledge_prior_matches.len().to_string(),
            ),
            metric(
                "negative_knowledge_rescan_recommendations",
                negative_knowledge_prior_matches.len().to_string(),
            ),
            metric(
                "negative_knowledge_corroboration_requirements",
                negative_knowledge_prior_matches.len().to_string(),
            ),
            metric("overall_score", format!("{score:.0}")),
            metric("grade", grade.to_string()),
        ],
        findings,
    };
    print_validation_findings(&report);
    report
}

fn percentage_or_zero(count: usize, total: usize) -> usize {
    count.saturating_mul(100).checked_div(total).unwrap_or(0)
}

fn sorted_by_value<'a>(map: &'a HashMap<&str, usize>) -> Vec<(&'a &'a str, &'a usize)> {
    let mut pairs: Vec<_> = map.iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(a.1));
    pairs
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::fs;

    use tempfile::tempdir;

    use super::*;
    use crate::error::Result;
    use crate::ir::evidence::{EvidenceIr, SignalSemanticHintSourceKind};
    use crate::ir::intent::IntentIr;
    use crate::ir::prior_memory::{
        CorpusMemoryUpdatePolicyRecord, NegativeKnowledgePriorRecord, PriorSourceArtifactRecord,
        SemanticModalityReliabilityPriorRecord,
    };
    use crate::ir::semantic::{
        ActorPortRecord, ActorRelativeDirection, InterfaceSignalSemanticRole,
        SemanticGroundingStrength, SemanticIr,
    };
    use crate::ir::source::{
        AutomationConfidence, ConditionalRuleRecord, DiagramKind, SignalConstraintKind,
        SignalConstraintRecord, SourceIr, StructuredTableCellRecord, StructuredTableRecord,
        TableKind, TimingConstraintRecord, VisualAsset, VisualAssetKind,
    };

    #[test]
    fn quiet_validation_output_guard_restores_previous_state() {
        assert!(!validation_output_suppressed());
        with_suppressed_validation_output(|| {
            assert!(validation_output_suppressed());
        });
        assert!(!validation_output_suppressed());
    }

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

    #[test]
    fn graph_direction_signal_names_excludes_same_actor_conflicts() {
        let actual = graph_direction_signal_names(&[
            actor_port("Completer", "PREADY", ActorRelativeDirection::Output),
            actor_port("Completer", "PREADY", ActorRelativeDirection::Input),
            actor_port("Requester", "PADDR", ActorRelativeDirection::Output),
            actor_port("Completer", "PADDR", ActorRelativeDirection::Input),
        ]);

        assert_eq!(actual, BTreeSet::from(["PADDR".to_string()]));
    }

    #[test]
    fn graph_direction_coverage_summary_reports_same_actor_conflicts() {
        let summary = graph_direction_coverage_summary(&[
            actor_port("Completer", "PREADY", ActorRelativeDirection::Output),
            actor_port("Completer", "PREADY", ActorRelativeDirection::Input),
            actor_port("Requester", "PADDR", ActorRelativeDirection::Output),
            actor_port("Completer", "PADDR", ActorRelativeDirection::Input),
        ]);

        assert_eq!(
            summary.resolved_signal_names,
            BTreeSet::from(["PADDR".to_string()])
        );
        assert_eq!(
            summary.conflicted_signal_names,
            BTreeSet::from(["PREADY".to_string()])
        );
        assert_eq!(
            summary.conflicts,
            BTreeSet::from([GraphDirectionConflictRecord {
                signal_name: "PREADY".to_string(),
                actor_id: "actor_completer".to_string(),
                actor_name: "Completer".to_string(),
            }])
        );
    }

    fn build_semantic_and_intent_from_markdown(
        source_file_name: &str,
        markdown: &str,
    ) -> Result<(SemanticIr, IntentIr)> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join(source_file_name);
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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

        Ok((semantic_ir, intent_ir))
    }

    fn make_table_cell(text: &str, is_header: bool) -> StructuredTableCellRecord {
        StructuredTableCellRecord {
            text: text.to_string(),
            row_span: 1,
            col_span: 1,
            is_header,
        }
    }

    fn corpus_memory_with_negative_knowledge(
        negative_knowledge_priors: Vec<NegativeKnowledgePriorRecord>,
        artifact_path: PathBuf,
    ) -> CorpusMemory {
        CorpusMemory {
            schema_version: 5,
            update_policy: CorpusMemoryUpdatePolicyRecord {
                advisory_only: true,
                requires_validated_intent_ir: true,
                rejects_error_findings: true,
                excludes_alias_dependent_semantic_consensus: true,
                local_grounding_required_for_canonical_promotion: true,
            },
            source_artifacts: vec![PriorSourceArtifactRecord {
                artifact_path,
                document_key: "seed_doc".to_string(),
                display_name: "seed_doc".to_string(),
                protocol_family: ProtocolFamily::Unknown,
                overall_score: Some(90),
                grade: Some("EXCELLENT".to_string()),
                accepted_for_learning: true,
                skip_reason: None,
            }],
            actor_taxonomy_priors: Vec::new(),
            semantic_phrase_priors: Vec::new(),
            semantic_modality_reliability_priors: Vec::new(),
            temporal_phrase_priors: Vec::new(),
            table_shape_priors: Vec::new(),
            visual_motif_priors: Vec::new(),
            negative_knowledge_priors,
        }
    }

    fn write_semantic_modality_reliability_prior_memory(
        root: &Path,
        role: InterfaceSignalSemanticRole,
        source_kind: SignalSemanticHintSourceKind,
        strongest_grounding_strength: SemanticGroundingStrength,
    ) -> Result<PathBuf> {
        let prior_memory_path = root
            .join("generated")
            .join("prior_memory")
            .join("corpus_memory.json");
        if let Some(parent) = prior_memory_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let corpus_memory = CorpusMemory {
            schema_version: 5,
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
                protocol_family: ProtocolFamily::Unknown,
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
                protocol_family: ProtocolFamily::Unknown,
                source_kind,
                support_count: 3,
                supporting_document_keys: vec!["seed_doc".to_string()],
                strongest_automation_confidence: AutomationConfidence::High,
                strongest_grounding_strength,
            }],
            temporal_phrase_priors: Vec::new(),
            table_shape_priors: Vec::new(),
            visual_motif_priors: Vec::new(),
            negative_knowledge_priors: Vec::new(),
        };
        fs::write(
            &prior_memory_path,
            serde_json::to_string_pretty(&corpus_memory)?,
        )?;

        Ok(prior_memory_path)
    }

    #[test]
    fn validate_source_ir_artifact_reports_without_error() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Spec\nSome content.\n")?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        run(ValidateArgs {
            artifact: source_ir.artifact_layout.source_ir_path,
        })
    }

    #[test]
    fn validate_source_ir_backannotates_artifact_and_writes_sidecar() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Spec\nSome content.\n")?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;

        let artifact_path = source_ir.artifact_layout.source_ir_path.clone();
        run(ValidateArgs {
            artifact: artifact_path.clone(),
        })?;

        let reloaded = SourceIr::load_from_path(&artifact_path)?;
        assert_eq!(reloaded.validation_reports.len(), 1);
        assert_eq!(
            reloaded.validation_reports[0].validated_stage,
            IrStage::SourceIr
        );
        assert!(
            validation_report_path_for(&artifact_path)?.exists(),
            "expected stage-local validation_report.json sidecar to exist"
        );

        Ok(())
    }

    #[test]
    fn validate_source_ir_reports_missing_vlm_enrichment_related_ids() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("visual_gap.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        fs::write(&source, "# Visual Gap\n\nSignal XREQ is output width 1.\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "asset_timing_0001".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("XREQ timing".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: None,
            diagram_kind: DiagramKind::TimingDiagram,
        });

        let report = validate_source_ir(&source_ir, "source_vlm_gap".to_string());

        let missing_vlm_finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "source_vlm_enrichment_missing")
            .expect("expected source VLM enrichment missing finding");
        assert_eq!(
            missing_vlm_finding.related_ids,
            vec!["asset_timing_0001".to_string()]
        );

        let missing_vlm_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SOURCE_VLM_ENRICHMENT_MISSING_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected source VLM enrichment rescan guidance");
        assert_eq!(
            missing_vlm_rescan_guidance.related_ids,
            vec!["asset_timing_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_artifact_reports_without_error() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(&source, "# Spec\nSome content.\n")?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.write_to_disk()?;

        run(ValidateArgs {
            artifact: evidence_ir.artifact_layout.evidence_ir_path,
        })
    }

    #[test]
    fn validate_evidence_ir_flags_signal_polarity_conflicts() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("reset_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(
            &source,
            concat!(
                "# Reset\n",
                "Signal PRESETN is input width 1.\n",
                "PRESETN is an active low reset signal.\n",
                "PRESETN must be asserted during initialization.\n",
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
                make_table_cell("Active high reset input", false),
            ]],
            row_count: 1,
            col_count: 2,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        let report = validate_evidence_ir(&evidence_ir, "signal_polarity_conflicts".to_string());

        assert_eq!(
            metric_value(&report, "signal_polarity_conflicts"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "evidence_signal_polarity_conflicts_present"
        ));
        let conflict_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == EVIDENCE_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected evidence polarity-conflict rescan guidance");
        assert_eq!(
            conflict_rescan_guidance.related_ids,
            vec!["polarity_conflict_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_counts_signal_semantic_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_hints.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal XREQ is input width 1.\n",
                "Signal XACK is input width 1.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_handshake_desc".to_string(),
            asset_id: "asset_handshake_desc".to_string(),
            page_id: None,
            caption_text: Some("Handshake signal descriptions".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("XREQ", false),
                    make_table_cell(
                        "Indicates that address and control information are valid for transfer.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell(
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
        let report = validate_evidence_ir(&evidence_ir, "signal_semantic_hints".to_string());

        assert_eq!(metric_value(&report, "signal_semantic_hints"), Some("2"));
        assert_eq!(
            metric_value(&report, "signal_semantic_hints_from_tables"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "signal_semantic_hints_from_alias_grounded_prose"),
            Some("0")
        );

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_counts_table_signal_declaration_provenance() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("table_signal_provenance.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "The surrounding PDF text mentions RTL, VIP, and PLL context.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_protocol_signal_description".to_string(),
            asset_id: "asset_protocol_signal_description".to_string(),
            page_id: None,
            caption_text: Some("Protocol signal description".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Direction", true),
                make_table_cell("Width", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("XREQ", false),
                    make_table_cell("output", false),
                    make_table_cell("1", false),
                    make_table_cell("Request/valid indication.", false),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell("input", false),
                    make_table_cell("1", false),
                    make_table_cell("Accept/ready indication.", false),
                ],
                vec![
                    make_table_cell("PAYLOAD", false),
                    make_table_cell("output", false),
                    make_table_cell("32", false),
                    make_table_cell("Data payload.", false),
                ],
            ],
            row_count: 3,
            col_count: 4,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        assert_eq!(evidence_ir.table_signal_declaration_provenance.len(), 3);
        assert!(
            evidence_ir
                .table_signal_declaration_provenance
                .iter()
                .all(|record| record.table_id == "table_protocol_signal_description")
        );

        let report = validate_evidence_ir(
            &evidence_ir,
            "table_signal_declaration_provenance".to_string(),
        );
        assert_eq!(
            metric_value(&report, "table_signal_declaration_provenance"),
            Some("3")
        );

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_counts_alias_grounded_signal_semantic_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("alias_semantic_hints.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal XREQ is input width 1.\n\n",
                "Signal XACK is input width 1.\n\n",
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

        let report = validate_evidence_ir(
            &evidence_ir,
            "alias_grounded_signal_semantic_hints".to_string(),
        );

        assert_eq!(metric_value(&report, "signal_semantic_hints"), Some("2"));
        assert_eq!(
            metric_value(&report, "signal_semantic_hints_from_alias_grounded_prose"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "signal_semantic_hints_from_tables"),
            Some("0")
        );

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_counts_visual_signal_semantic_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("visual_semantic_hints.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(
            &source,
            concat!(
                "# Channel\n",
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
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "figure_xack".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0002".to_string()),
            image_path: None,
            caption_text: Some("Figure 2: Transfer timing".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: Some(
                "vlm_timing_diagram_extraction: ```json\n{\n  \"signals\": [{\"name\": \"XACK\", \"values\": [{\"cycle\": \"T1\", \"state\": \"HIGH\"}]}],\n  \"annotations\": [\n    \"XACK indicates that the subordinate can accept the transfer.\"\n  ]\n}\n```"
                    .to_string(),
            ),
            diagram_kind: crate::ir::source::DiagramKind::TimingDiagram,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        let report = validate_evidence_ir(&evidence_ir, "visual_signal_semantic_hints".to_string());

        assert_eq!(metric_value(&report, "signal_semantic_hints"), Some("2"));
        assert_eq!(
            metric_value(&report, "signal_semantic_hints_from_visual_captions"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "signal_semantic_hints_from_vlm_timing_annotations"),
            Some("1")
        );

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_reports_structural_kg_and_normative_related_ids() -> Result<()> {
        use crate::ir::evidence::{EvidenceModality, ExtractedStatement, StatementClass};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("evidence_related_ids.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal HREADY is input width 1.\n\n",
                "Signal HTRANS is output width 2.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_hready_asserted".to_string(),
            subject_signal: "HREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "HREADY must be asserted.".to_string(),
            supporting_statement_ids: vec!["stmt_signal_constraint".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.conditional_rules.push(ConditionalRuleRecord {
            rule_id: "condrule_htrans_hold".to_string(),
            antecedent_text: "when HREADY is LOW".to_string(),
            consequent_signal: Some("HTRANS".to_string()),
            consequent_action: "must not change".to_string(),
            source_text: "When HREADY is LOW, HTRANS must not change.".to_string(),
            supporting_statement_ids: vec!["stmt_conditional_rule".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.extracted_statements.push(ExtractedStatement {
            statement_id: "stmt_normative_residual".to_string(),
            class: StatementClass::NormativeStatement,
            modality: EvidenceModality::Text,
            text: "Transfers must preserve ordering.".to_string(),
            evidence_span_ids: Vec::new(),
            related_visual_evidence_ids: Vec::new(),
        });
        evidence_ir.actor_signal_relations.clear();

        let report = validate_evidence_ir(&evidence_ir, "evidence_related_ids".to_string());

        let structural_finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "evidence_structural_kg_missing")
            .expect("expected structural KG missing finding");
        assert_eq!(
            structural_finding.related_ids,
            vec![
                "condrule_htrans_hold".to_string(),
                "sigcon_hready_asserted".to_string()
            ]
        );
        let structural_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == EVIDENCE_STRUCTURAL_KG_MISSING_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected structural KG missing rescan guidance");
        assert_eq!(
            structural_rescan_guidance.related_ids,
            vec![
                "condrule_htrans_hold".to_string(),
                "sigcon_hready_asserted".to_string()
            ]
        );

        let normative_finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "evidence_normative_residuals_remaining")
            .expect("expected normative residual finding");
        assert_eq!(
            normative_finding.related_ids,
            vec!["stmt_normative_residual".to_string()]
        );
        let normative_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == EVIDENCE_NORMATIVE_RESIDUAL_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected normative residual rescan guidance");
        assert_eq!(
            normative_rescan_guidance.related_ids,
            vec!["stmt_normative_residual".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_flags_signal_semantic_conflicts() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_hint_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(
            &source,
            concat!(
                "# Channel\n",
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
        let report = validate_evidence_ir(&evidence_ir, "signal_semantic_conflicts".to_string());

        assert_eq!(
            metric_value(&report, "signal_semantic_conflicts"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "evidence_signal_semantic_conflicts_present"
        ));
        let conflict_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == EVIDENCE_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected evidence signal-conflict rescan guidance");
        assert_eq!(
            conflict_rescan_guidance.related_ids,
            vec!["semantic_conflict_0001".to_string()]
        );
        assert_eq!(
            metric_value(&report, "negative_knowledge_prior_matches"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "negative_knowledge_rescan_recommendations"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "negative_knowledge_corroboration_requirements"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "evidence_negative_knowledge_prior_matches"
        ));
        assert!(!has_finding(
            &report,
            "evidence_negative_knowledge_rescan_guidance"
        ));

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_surfaces_negative_knowledge_prior_matches() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_hint_conflict_with_prior.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let prior_memory_path = tempdir.path().join("corpus_memory.json");
        fs::write(
            &source,
            concat!(
                "# Channel\n",
                "Signal XCTRL is input width 1.\n\n",
                "XCTRL indicates that the subordinate can accept the transfer.\n",
            ),
        )?;

        let corpus_memory = CorpusMemory {
            schema_version: 5,
            update_policy: CorpusMemoryUpdatePolicyRecord {
                advisory_only: true,
                requires_validated_intent_ir: true,
                rejects_error_findings: true,
                excludes_alias_dependent_semantic_consensus: true,
                local_grounding_required_for_canonical_promotion: true,
            },
            source_artifacts: vec![PriorSourceArtifactRecord {
                artifact_path: tempdir.path().join("seed_intent_ir.json"),
                document_key: "seed_doc".to_string(),
                display_name: "seed_doc".to_string(),
                protocol_family: ProtocolFamily::Unknown,
                overall_score: Some(90),
                grade: Some("EXCELLENT".to_string()),
                accepted_for_learning: true,
                skip_reason: None,
            }],
            actor_taxonomy_priors: Vec::new(),
            semantic_phrase_priors: Vec::new(),
            semantic_modality_reliability_priors: Vec::new(),
            temporal_phrase_priors: Vec::new(),
            table_shape_priors: Vec::new(),
            visual_motif_priors: Vec::new(),
            negative_knowledge_priors: vec![NegativeKnowledgePriorRecord {
                prior_id: "negative_knowledge_prior_0001".to_string(),
                knowledge_kind: NegativeKnowledgeKind::SignalSemanticConflict,
                normalized_pattern: "signal_semantic_conflict:prose_statement:handshake_ready_like|signal_description_table:handshake_valid_like".to_string(),
                protocol_family: ProtocolFamily::Unknown,
                support_count: 2,
                supporting_document_keys: vec!["seed_doc".to_string()],
                strongest_automation_confidence: AutomationConfidence::Medium,
            }],
        };
        fs::write(
            &prior_memory_path,
            serde_json::to_string_pretty(&corpus_memory)?,
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

        let evidence_ir = EvidenceIr::build_with_prior_memory(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
            Some(&prior_memory_path),
        )?;
        let report =
            validate_evidence_ir(&evidence_ir, "negative_knowledge_prior_matches".to_string());

        assert_eq!(
            metric_value(&report, "signal_semantic_conflicts"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "negative_knowledge_prior_matches"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "negative_knowledge_rescan_recommendations"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "negative_knowledge_corroboration_requirements"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "evidence_signal_semantic_conflicts_present"
        ));
        assert!(has_finding(
            &report,
            "evidence_negative_knowledge_prior_matches"
        ));
        assert!(has_finding(
            &report,
            "evidence_negative_knowledge_rescan_guidance"
        ));

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_reports_missing_vlm_observation_related_ids() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("visual_gap.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        fs::write(&source, "# Figures\n\nSignal XREQ is output width 1.\n")?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.visual_assets.push(VisualAsset {
            asset_id: "asset_cycle_trace".to_string(),
            asset_kind: VisualAssetKind::Diagram,
            page_id: Some("page_0001".to_string()),
            image_path: None,
            caption_text: Some("XREQ cycle trace".to_string()),
            caption_source_path: None,
            source_ref: None,
            placeholder_text: None,
            note: None,
            diagram_kind: crate::ir::source::DiagramKind::Unknown,
        });
        source_ir.write_to_disk()?;

        let evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        let report = validate_evidence_ir(&evidence_ir, "evidence_missing_vlm".to_string());

        let missing_vlm_finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "evidence_missing_vlm_observations")
            .expect("expected missing VLM observations finding");
        assert_eq!(
            missing_vlm_finding.related_ids,
            vec!["visual_0001".to_string()]
        );

        let missing_vlm_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == EVIDENCE_MISSING_VLM_OBSERVATIONS_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected missing VLM observations rescan guidance");
        assert_eq!(
            missing_vlm_rescan_guidance.related_ids,
            vec!["visual_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_evidence_ir_surfaces_negative_knowledge_polarity_prior_matches() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("polarity_conflict_with_prior.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let prior_memory_path = tempdir.path().join("corpus_memory.json");
        fs::write(
            &source,
            concat!(
                "# Reset\n\n",
                "Signal PRESETN is input width 1.\n\n",
                "PRESETN is active HIGH.\n",
            ),
        )?;

        let corpus_memory = corpus_memory_with_negative_knowledge(
            vec![NegativeKnowledgePriorRecord {
                prior_id: "negative_knowledge_prior_0001".to_string(),
                knowledge_kind: NegativeKnowledgeKind::SignalPolarityConflict,
                normalized_pattern:
                    "signal_polarity_conflict:prose_statement:active_high|signal_description_table:active_low"
                        .to_string(),
                protocol_family: ProtocolFamily::Unknown,
                support_count: 2,
                supporting_document_keys: vec!["seed_doc".to_string()],
                strongest_automation_confidence: AutomationConfidence::Medium,
            }],
            tempdir.path().join("seed_intent_ir.json"),
        );
        fs::write(
            &prior_memory_path,
            serde_json::to_string_pretty(&corpus_memory)?,
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

        let evidence_ir = EvidenceIr::build_with_prior_memory(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
            Some(&prior_memory_path),
        )?;
        let report = validate_evidence_ir(
            &evidence_ir,
            "negative_knowledge_polarity_matches".to_string(),
        );

        assert_eq!(
            metric_value(&report, "signal_polarity_conflicts"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "negative_knowledge_prior_matches"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "negative_knowledge_rescan_recommendations"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "negative_knowledge_corroboration_requirements"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "evidence_signal_polarity_conflicts_present"
        ));
        let conflict_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == EVIDENCE_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected evidence polarity-conflict rescan guidance");
        assert_eq!(
            conflict_rescan_guidance.related_ids,
            vec!["polarity_conflict_0001".to_string()]
        );
        assert!(has_finding(
            &report,
            "evidence_negative_knowledge_prior_matches"
        ));
        assert!(has_finding(
            &report,
            "evidence_negative_knowledge_rescan_guidance"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_flags_signal_semantic_conflicts() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_stage_role_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        fs::write(
            &source,
            concat!(
                "# Channel\n",
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

        let report = validate_semantic_ir(&semantic_ir, "signal_semantic_conflicts".to_string());

        assert_eq!(
            metric_value(&report, "signal_semantic_conflicts"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_semantic_arbitration"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_decisive_semantic_arbitration"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_non_decisive_semantic_arbitration"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "semantic_signal_semantic_conflicts_present"
        ));
        assert!(has_finding(
            &report,
            "semantic_non_decisive_semantic_arbitration_present"
        ));
        let finding = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "semantic_non_decisive_semantic_arbitration_present"
            })
            .expect("expected non-decisive semantic arbitration finding");
        assert_eq!(finding.related_ids, vec!["XCTRL".to_string()]);
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == SEMANTIC_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE)
            .expect("expected semantic-role-arbitration rescan guidance");
        assert_eq!(rescan_guidance.related_ids, vec!["XCTRL".to_string()]);
        let conflict_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic signal-conflict rescan guidance");
        assert_eq!(
            conflict_rescan_guidance.related_ids,
            vec!["semantic_conflict_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_flags_signal_polarity_conflicts() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_stage_polarity_conflict.md");
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

        let report = validate_semantic_ir(&semantic_ir, "signal_polarity_conflicts".to_string());
        assert_eq!(
            metric_value(&report, "signal_polarity_conflicts"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "semantic_signal_polarity_conflicts_present"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_counts_resolved_signal_polarity() -> Result<()> {
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

        let report = validate_semantic_ir(&semantic_ir, "resolved_signal_polarity".to_string());
        assert_eq!(metric_value(&report, "with_resolved_polarity"), Some("1"));

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_counts_system_contract_resolved_polarity() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_system_contract_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        fs::write(
            &source,
            concat!(
                "# Reset\n",
                "Clock HCLK.\n\n",
                "Reset HRESETN is asynchronous active low.\n",
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

        let report = validate_semantic_ir(
            &semantic_ir,
            "system_contract_resolved_polarity".to_string(),
        );
        assert_eq!(metric_value(&report, "with_resolved_polarity"), Some("1"));

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_count_signal_table_support() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("table_backed_signals.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "The PDF text mentions RTL and VIP context, but the interface signal inventory is declared by the table.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.structured_tables.push(StructuredTableRecord {
            table_id: "table_protocol_signal_description".to_string(),
            asset_id: "asset_protocol_signal_description".to_string(),
            page_id: None,
            caption_text: Some("Protocol signal description".to_string()),
            source_ref: None,
            table_kind: TableKind::SignalDescription,
            header_rows: vec![vec![
                make_table_cell("Signal", true),
                make_table_cell("Direction", true),
                make_table_cell("Width", true),
                make_table_cell("Description", true),
            ]],
            body_rows: vec![
                vec![
                    make_table_cell("XREQ", false),
                    make_table_cell("output", false),
                    make_table_cell("1", false),
                    make_table_cell("Request/valid indication.", false),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell("input", false),
                    make_table_cell("1", false),
                    make_table_cell("Accept/ready indication.", false),
                ],
                vec![
                    make_table_cell("PAYLOAD", false),
                    make_table_cell("output", false),
                    make_table_cell("32", false),
                    make_table_cell("Data payload.", false),
                ],
            ],
            row_count: 3,
            col_count: 4,
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

        let semantic_report =
            validate_semantic_ir(&semantic_ir, "semantic_table_support".to_string());
        assert_eq!(
            metric_value(&semantic_report, "total_signal_records"),
            Some("3")
        );
        assert_eq!(
            metric_value(&semantic_report, "with_table_support"),
            Some("3")
        );

        let intent_report = validate_intent_ir(&intent_ir, "intent_table_support".to_string());
        assert_eq!(
            metric_value(&intent_report, "declared_signal_records"),
            Some("3")
        );
        assert_eq!(
            metric_value(&intent_report, "with_table_support"),
            Some("3")
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_blocked_handshake_name_fallback() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("semantic_contested_semantic_handshake_guard.md");
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

        let report =
            validate_semantic_ir(&semantic_ir, "blocked_handshake_name_fallback".to_string());
        assert_eq!(
            metric_value(&report, "with_blocked_handshake_name_fallback"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "semantic_handshake_name_fallback_blocked_present"
        ));
        let finding = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "semantic_handshake_name_fallback_blocked_present"
            })
            .expect("expected blocked handshake fallback finding");
        assert_eq!(finding.related_ids, vec!["XVALID".to_string()]);

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_alias_dependent_semantic_consensus() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("semantic_alias_grounded_semantic_roles.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal XREQ is input width 1.\n\n",
                "Signal XACK is input width 1.\n\n",
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
        evidence_ir.write_to_disk()?;
        let semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;

        let report =
            validate_semantic_ir(&semantic_ir, "alias_grounded_semantic_roles".to_string());
        assert_eq!(
            metric_value(&report, "with_alias_dependent_semantic_consensus"),
            Some("2")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "semantic_alias_dependent_semantic_consensus_present"
            })
            .expect("expected alias-dependent semantic consensus finding");
        assert_eq!(
            finding.related_ids,
            vec!["XACK".to_string(), "XREQ".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id
                    == SEMANTIC_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected alias-dependent semantic consensus rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["XACK".to_string(), "XREQ".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_flags_resolved_roles_without_consensus() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("semantic_resolved_roles_without_consensus.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal XREQ is output width 1.\n\n",
                "Signal XACK is input width 1.\n",
            ),
        )?;

        let mut source_ir = SourceIr::build(&source, &source_artifact_base)?;
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
            body_rows: vec![
                vec![
                    make_table_cell("XREQ", false),
                    make_table_cell(
                        "Indicates that address and control information are valid for transfer.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell(
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
        let mut semantic_ir = SemanticIr::build(
            &evidence_ir.artifact_layout.evidence_ir_path,
            &semantic_artifact_base,
        )?;
        let xreq = semantic_ir
            .interfaces
            .iter_mut()
            .flat_map(|interface| interface.signal_records.iter_mut())
            .find(|signal| signal.signal_name == "XREQ")
            .expect("expected XREQ interface signal");
        xreq.semantic_consensus = None;

        let report = validate_semantic_ir(
            &semantic_ir,
            "resolved_semantic_roles_without_consensus".to_string(),
        );
        assert_eq!(
            metric_value(&report, "resolved_semantic_roles_without_consensus"),
            Some("1")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "semantic_resolved_roles_without_consensus_present"
            })
            .expect("expected resolved-without-consensus finding");
        assert_eq!(finding.related_ids, vec!["XREQ".to_string()]);
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == SEMANTIC_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE)
            .expect("expected semantic-role-consensus rescan guidance");
        assert_eq!(rescan_guidance.related_ids, vec!["XREQ".to_string()]);

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_report_prior_guided_semantic_related_ids() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_role_conflict_with_prior.md");
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

        let prior_memory_path = write_semantic_modality_reliability_prior_memory(
            tempdir.path(),
            InterfaceSignalSemanticRole::HandshakeValidLike,
            SignalSemanticHintSourceKind::SignalDescriptionTable,
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let semantic_report = validate_semantic_ir(
            &semantic_ir,
            "semantic_prior_guided_semantic_related_ids".to_string(),
        );
        assert_eq!(
            metric_value(&semantic_report, "with_prior_guided_semantic_arbitration"),
            Some("1")
        );
        assert_eq!(
            metric_value(&semantic_report, "with_prior_guided_semantic_consensus"),
            Some("1")
        );
        let semantic_arbitration_finding = semantic_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "semantic_prior_guided_semantic_arbitration_present"
            })
            .expect("expected semantic prior-guided arbitration finding");
        assert_eq!(
            semantic_arbitration_finding.related_ids,
            vec!["XCTRL".to_string()]
        );
        let semantic_consensus_finding = semantic_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "semantic_prior_guided_semantic_consensus_present"
            })
            .expect("expected semantic prior-guided consensus finding");
        assert_eq!(
            semantic_consensus_finding.related_ids,
            vec!["XCTRL".to_string()]
        );
        let semantic_rescan_guidance = semantic_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id
                    == SEMANTIC_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic prior-guided consensus rescan guidance");
        assert_eq!(
            semantic_rescan_guidance.related_ids,
            vec!["XCTRL".to_string()]
        );

        let intent_report = validate_intent_ir(
            &intent_ir,
            "intent_prior_guided_semantic_related_ids".to_string(),
        );
        assert_eq!(
            metric_value(&intent_report, "with_prior_guided_semantic_arbitration"),
            Some("1")
        );
        assert_eq!(
            metric_value(&intent_report, "with_prior_guided_semantic_consensus"),
            Some("1")
        );
        let intent_arbitration_finding = intent_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "intent_prior_guided_semantic_arbitration_present"
            })
            .expect("expected intent prior-guided arbitration finding");
        assert_eq!(
            intent_arbitration_finding.related_ids,
            vec!["XCTRL".to_string()]
        );
        let intent_consensus_finding = intent_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_prior_guided_semantic_consensus_present")
            .expect("expected intent prior-guided consensus finding");
        assert_eq!(
            intent_consensus_finding.related_ids,
            vec!["XCTRL".to_string()]
        );
        let intent_rescan_guidance = intent_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_PRIOR_GUIDED_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent prior-guided consensus rescan guidance");
        assert_eq!(
            intent_rescan_guidance.related_ids,
            vec!["XCTRL".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_artifact_reports_without_error() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        // Use formal module syntax so SemanticIR has something to extract.
        fs::write(
            &source,
            "# Spec\n(?module: mymod (+system (clock clk) (sreset rstn)))\n",
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

        run(ValidateArgs {
            artifact: semantic_ir.artifact_layout.semantic_ir_path,
        })
    }

    #[test]
    fn validate_intent_ir_artifact_reports_without_error() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            "# Spec\n(?module: mymod (+system (clock clk) (sreset rstn)))\n",
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

        run(ValidateArgs {
            artifact: intent_ir.artifact_layout.intent_ir_path,
        })
    }

    #[test]
    fn validate_intent_ir_treats_clock_and_reset_as_infrastructure_connectivity() -> Result<()> {
        use crate::ir::evidence::{EvidenceModality, ExtractedStatement, StatementClass};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("infra_connectivity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Spec\n",
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

        let report = validate_intent_ir(&intent_ir, "infra_connectivity".to_string());
        assert_eq!(
            metric_value(&report, "infrastructure_signal_connectivity"),
            Some("2")
        );
        assert_eq!(metric_value(&report, "infrastructure_signals"), Some("2"));
        assert_eq!(
            metric_value(&report, "infrastructure_signals_unresolved_source"),
            Some("2")
        );
        assert_eq!(
            metric_value(
                &report,
                "infrastructure_signals_shared_recovered_distribution"
            ),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "infrastructure_signals_missing_producer"),
            Some("2")
        );
        assert!(!has_finding(
            &report,
            "intent_connectivity_missing_producer"
        ));
        assert!(has_finding(
            &report,
            "intent_infrastructure_connectivity_missing_producer"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_recovered_infrastructure_source() -> Result<()> {
        use crate::ir::evidence::{EvidenceModality, ExtractedStatement, StatementClass};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("infra_source.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Spec\n",
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "infra_source".to_string());
        assert_eq!(metric_value(&report, "infrastructure_signals"), Some("2"));
        assert_eq!(
            metric_value(&report, "infrastructure_signals_recovered_source"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "infrastructure_signals_unresolved_source"),
            Some("1")
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_recovered_infrastructure_distribution() -> Result<()> {
        use crate::ir::evidence::{EvidenceModality, ExtractedStatement, StatementClass};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("infra_distribution.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Spec\n",
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "infra_distribution".to_string());
        assert_eq!(metric_value(&report, "infrastructure_signals"), Some("2"));
        assert_eq!(
            metric_value(
                &report,
                "infrastructure_signals_shared_recovered_distribution"
            ),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "infrastructure_signals_no_recovered_distribution"),
            Some("1")
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_explicit_infrastructure_topology() -> Result<()> {
        use crate::ir::evidence::{EvidenceModality, ExtractedStatement, StatementClass};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("infra_topology.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Spec\n",
                "Clock ACLK.\n",
                "Reset ARESETN is asynchronous active low.\n",
                "The ACLK clock gate CGATE0 feeds the Requester branch.\n",
                "The two-stage reset synchronizer RSTSYNC0 feeds ARESETN to the Requester.\n",
                "The ARESETN reset tree targets the Requester registers and Completer registers.\n",
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "infra_topology".to_string());
        assert_eq!(
            metric_value(&report, "infrastructure_topology_records"),
            Some("3")
        );
        assert_eq!(
            metric_value(&report, "infrastructure_clock_gated_branches"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "infrastructure_reset_synchronizer_stages"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "infrastructure_reset_tree_targets"),
            Some("1")
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_backannotates_current_score_into_artifact() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            "# Spec\nSignal DATA_IN is input width 8.\n\nSignal DATA_OUT is output width 8.\n\nBlock pass: DATA_OUT = DATA_IN.\n",
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

        let artifact_path = intent_ir.artifact_layout.intent_ir_path.clone();
        run(ValidateArgs {
            artifact: artifact_path.clone(),
        })?;

        let reloaded = IntentIr::load_from_path(&artifact_path)?;
        assert_eq!(reloaded.validation_reports.len(), 1);
        let report = &reloaded.validation_reports[0];
        assert_eq!(report.validated_stage, IrStage::IntentIr);
        assert!(report.overall_score.is_some());
        assert!(report.grade.is_some());
        assert!(
            validation_report_path_for(&artifact_path)?.exists(),
            "expected stage-local validation_report.json sidecar to exist"
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_quality_gap_related_ids() -> Result<()> {
        let (_, intent_ir) = build_semantic_and_intent_from_markdown(
            "quality_gap_related_ids.md",
            concat!("# Protocol\n", "Signal XREQ is input width 1.\n"),
        )?;

        let report = validate_intent_ir(&intent_ir, "quality_gap_related_ids".to_string());

        assert_eq!(report.overall_score, Some(35));
        assert_eq!(report.grade.as_deref(), Some("NEEDS IMPROVEMENT"));

        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_quality_below_excellent_threshold")
            .expect("expected quality-threshold finding");
        assert_eq!(
            finding.related_ids,
            vec![
                "score_component:nlp_constraints".to_string(),
                "score_component:encoding_enums".to_string(),
                "score_component:register_map".to_string(),
                "score_component:timing_constraints".to_string(),
                "score_component:state_machine".to_string(),
                "score_component:system_contract".to_string(),
            ]
        );

        Ok(())
    }

    fn metric_value<'a>(report: &'a ValidationReportRecord, name: &str) -> Option<&'a str> {
        report
            .metrics
            .iter()
            .find(|metric| metric.name == name)
            .map(|metric| metric.value.as_str())
    }

    fn has_finding(report: &ValidationReportRecord, finding_id: &str) -> bool {
        report
            .findings
            .iter()
            .any(|finding| finding.finding_id == finding_id)
    }

    #[test]
    fn validate_intent_ir_scores_direction_from_graph_before_compat_hints() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
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
                "The Completer samples PADDR.\n",
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

        let with_hints_report = validate_intent_ir(&intent_ir, "with_hints".to_string());

        let mut graph_only_intent = intent_ir.clone();
        for interface in &mut graph_only_intent.interfaces {
            for signal in &mut interface.signal_records {
                signal.direction_hint = None;
            }
        }
        let graph_only_report = validate_intent_ir(&graph_only_intent, "graph_only".to_string());

        assert_eq!(
            with_hints_report.overall_score, graph_only_report.overall_score,
            "graph-derived direction coverage should preserve the IntentIR score even when flat compatibility hints are absent"
        );
        assert_eq!(
            metric_value(&graph_only_report, "with_resolved_direction"),
            Some("2")
        );
        assert_eq!(
            metric_value(&graph_only_report, "with_graph_direction"),
            Some("2")
        );
        assert_eq!(
            metric_value(&graph_only_report, "with_compat_direction_hint"),
            Some("0")
        );
        let finding = graph_only_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_compat_direction_hints_lag_graph")
            .expect("expected compat-direction lag finding");
        assert_eq!(
            finding.related_ids,
            vec!["PADDR".to_string(), "PREADY".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_keeps_incomplete_direction_finding_without_graph_coverage() -> Result<()>
    {
        let (_, mut intent_ir) = build_semantic_and_intent_from_markdown(
            "intent_direction_unresolved.md",
            "# Protocol\nSignal DATA is input width 8.\n",
        )?;

        for interface in &mut intent_ir.interfaces {
            for signal in &mut interface.signal_records {
                signal.direction_hint = None;
            }
        }
        intent_ir.actor_ports.clear();

        let report = validate_intent_ir(&intent_ir, "intent_unresolved".to_string());

        assert_eq!(metric_value(&report, "with_resolved_direction"), Some("0"));
        assert_eq!(metric_value(&report, "with_graph_direction"), Some("0"));
        assert_eq!(
            metric_value(&report, "with_compat_direction_hint"),
            Some("0")
        );
        assert!(
            !has_finding(&report, "intent_compat_direction_hints_lag_graph"),
            "without actor-relative graph coverage, missing flat hints should not be reported as graph lag"
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_compat_direction_hints_incomplete")
            .expect("expected unresolved intent compatibility direction finding");
        assert_eq!(finding.related_ids, vec!["DATA".to_string()]);

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_graph_backed_compat_direction_lag_related_ids() -> Result<()> {
        let (mut semantic_ir, _) = build_semantic_and_intent_from_markdown(
            "semantic_compat_direction_gap.md",
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
                "The Completer samples PADDR.\n",
            ),
        )?;

        for interface in &mut semantic_ir.interfaces {
            for signal in &mut interface.signal_records {
                signal.direction_hint = None;
            }
        }

        let report = validate_semantic_ir(&semantic_ir, "semantic_graph_only".to_string());

        assert_eq!(metric_value(&report, "with_resolved_direction"), Some("2"));
        assert_eq!(metric_value(&report, "with_graph_direction"), Some("2"));
        assert_eq!(
            metric_value(&report, "with_compat_direction_hint"),
            Some("0")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_compat_direction_hints_lag_graph")
            .expect("expected semantic graph-backed compat-direction lag finding");
        assert_eq!(
            finding.related_ids,
            vec!["PADDR".to_string(), "PREADY".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_keeps_incomplete_direction_finding_without_graph_coverage() -> Result<()>
    {
        let (mut semantic_ir, _) = build_semantic_and_intent_from_markdown(
            "semantic_direction_unresolved.md",
            "# Protocol\nSignal DATA is input width 8.\n",
        )?;

        for interface in &mut semantic_ir.interfaces {
            for signal in &mut interface.signal_records {
                signal.direction_hint = None;
            }
        }
        semantic_ir.actor_ports.clear();

        let report = validate_semantic_ir(&semantic_ir, "semantic_unresolved".to_string());

        assert_eq!(metric_value(&report, "with_resolved_direction"), Some("0"));
        assert_eq!(metric_value(&report, "with_graph_direction"), Some("0"));
        assert_eq!(
            metric_value(&report, "with_compat_direction_hint"),
            Some("0")
        );
        assert!(
            !has_finding(&report, "semantic_compat_direction_hints_lag_graph"),
            "without actor-relative graph coverage, missing flat hints should not be reported as graph lag"
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_compat_direction_hints_incomplete")
            .expect("expected unresolved semantic compatibility direction finding");
        assert_eq!(finding.related_ids, vec!["DATA".to_string()]);

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_credit_conflicting_same_actor_graph_direction() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
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
                "The Completer samples PADDR.\n",
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

        let mut graph_only_intent = intent_ir.clone();
        for interface in &mut graph_only_intent.interfaces {
            for signal in &mut interface.signal_records {
                signal.direction_hint = None;
            }
        }
        let mut conflicting_port = graph_only_intent
            .actor_ports
            .iter()
            .find(|port| port.actor_name == "Completer" && port.signal_name == "PREADY")
            .cloned()
            .expect("Completer PREADY actor port should exist");
        conflicting_port.direction = ActorRelativeDirection::Input;
        graph_only_intent.actor_ports.push(conflicting_port);

        let report = validate_intent_ir(&graph_only_intent, "graph_conflict".to_string());

        assert_eq!(metric_value(&report, "with_resolved_direction"), Some("1"));
        assert_eq!(metric_value(&report, "with_graph_direction"), Some("1"));
        assert_eq!(
            metric_value(&report, "graph_direction_conflicts"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_compat_direction_hint"),
            Some("0")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_graph_direction_conflicts_present")
            .expect("expected intent graph-direction conflict finding");
        assert_eq!(
            finding.related_ids,
            vec!["graph_direction_conflict:actor_completer:PREADY".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent graph-direction conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["graph_direction_conflict:actor_completer:PREADY".to_string()]
        );
        let coverage_finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_graph_direction_coverage_incomplete")
            .expect("expected conflicted graph direction to remain a coverage gap");
        assert_eq!(coverage_finding.related_ids, vec!["PREADY".to_string()]);
        let coverage_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent graph-direction coverage rescan guidance");
        assert_eq!(
            coverage_rescan_guidance.related_ids,
            vec!["PREADY".to_string()]
        );
        let unresolved_compat_finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_compat_direction_hints_incomplete")
            .expect("expected conflicted graph direction to remain an unresolved compat gap");
        assert_eq!(
            unresolved_compat_finding.related_ids,
            vec!["PREADY".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_missing_graph_direction_related_ids() -> Result<()> {
        let (_, intent_ir) = build_semantic_and_intent_from_markdown(
            "intent_graph_gap.md",
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "Signal PSEL is input width 1.\n",
                "\n",
                "The Completer drives PREADY.\n",
                "\n",
                "The Requester reads PREADY.\n",
            ),
        )?;

        let report = validate_intent_ir(&intent_ir, "graph_gap".to_string());

        assert_eq!(metric_value(&report, "with_graph_direction"), Some("1"));
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_graph_direction_coverage_incomplete")
            .expect("expected intent graph-direction coverage finding");
        assert_eq!(finding.related_ids, vec!["PSEL".to_string()]);
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent graph-direction rescan guidance");
        assert_eq!(rescan_guidance.related_ids, vec!["PSEL".to_string()]);

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_conflicting_same_actor_graph_direction() -> Result<()> {
        let (mut semantic_ir, _) = build_semantic_and_intent_from_markdown(
            "semantic_graph_conflict.md",
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
                "The Completer samples PADDR.\n",
            ),
        )?;

        for interface in &mut semantic_ir.interfaces {
            for signal in &mut interface.signal_records {
                signal.direction_hint = None;
            }
        }
        let mut conflicting_port = semantic_ir
            .actor_ports
            .iter()
            .find(|port| port.actor_name == "Completer" && port.signal_name == "PREADY")
            .cloned()
            .expect("Completer PREADY actor port should exist");
        conflicting_port.direction = ActorRelativeDirection::Input;
        semantic_ir.actor_ports.push(conflicting_port);

        let report = validate_semantic_ir(&semantic_ir, "semantic_graph_conflict".to_string());

        assert_eq!(metric_value(&report, "with_resolved_direction"), Some("1"));
        assert_eq!(metric_value(&report, "with_graph_direction"), Some("1"));
        assert_eq!(
            metric_value(&report, "graph_direction_conflicts"),
            Some("1")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_graph_direction_conflicts_present")
            .expect("expected semantic graph-direction conflict finding");
        assert_eq!(
            finding.related_ids,
            vec!["graph_direction_conflict:actor_completer:PREADY".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_GRAPH_DIRECTION_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic graph-direction conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["graph_direction_conflict:actor_completer:PREADY".to_string()]
        );
        let coverage_finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_graph_direction_coverage_incomplete")
            .expect("expected conflicted graph direction to remain a coverage gap");
        assert_eq!(coverage_finding.related_ids, vec!["PREADY".to_string()]);
        let coverage_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic graph-direction coverage rescan guidance");
        assert_eq!(
            coverage_rescan_guidance.related_ids,
            vec!["PREADY".to_string()]
        );
        let unresolved_compat_finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_compat_direction_hints_incomplete")
            .expect("expected conflicted graph direction to remain an unresolved compat gap");
        assert_eq!(
            unresolved_compat_finding.related_ids,
            vec!["PREADY".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_missing_graph_direction_related_ids() -> Result<()> {
        let (semantic_ir, _) = build_semantic_and_intent_from_markdown(
            "semantic_graph_gap.md",
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "Signal PSEL is input width 1.\n",
                "\n",
                "The Completer drives PREADY.\n",
                "\n",
                "The Requester reads PREADY.\n",
            ),
        )?;

        let report = validate_semantic_ir(&semantic_ir, "semantic_graph_gap".to_string());

        assert_eq!(metric_value(&report, "with_graph_direction"), Some("1"));
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_graph_direction_coverage_incomplete")
            .expect("expected semantic graph-direction coverage finding");
        assert_eq!(finding.related_ids, vec!["PSEL".to_string()]);
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_GRAPH_DIRECTION_COVERAGE_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic graph-direction rescan guidance");
        assert_eq!(rescan_guidance.related_ids, vec!["PSEL".to_string()]);

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_missing_connectivity_endpoint_related_ids() -> Result<()> {
        let (semantic_ir, _) = build_semantic_and_intent_from_markdown(
            "semantic_connectivity_gap.md",
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "Signal PSEL is input width 1.\n",
                "\n",
                "The Requester reads PREADY.\n",
                "\n",
                "The Completer drives PSEL.\n",
            ),
        )?;

        let report = validate_semantic_ir(&semantic_ir, "semantic_connectivity_gap".to_string());

        let missing_producer = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_connectivity_missing_producer")
            .expect("expected semantic missing-producer finding");
        assert_eq!(missing_producer.related_ids, vec!["PREADY".to_string()]);
        let missing_producer_rescan = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic missing-producer rescan guidance");
        assert_eq!(
            missing_producer_rescan.related_ids,
            vec!["PREADY".to_string()]
        );

        let missing_consumer = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_connectivity_missing_consumer")
            .expect("expected semantic missing-consumer finding");
        assert_eq!(missing_consumer.related_ids, vec!["PSEL".to_string()]);
        let missing_consumer_rescan = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic missing-consumer rescan guidance");
        assert_eq!(
            missing_consumer_rescan.related_ids,
            vec!["PSEL".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_missing_connectivity_endpoint_related_ids() -> Result<()> {
        let (_, intent_ir) = build_semantic_and_intent_from_markdown(
            "intent_connectivity_gap.md",
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "Signal PSEL is input width 1.\n",
                "\n",
                "The Requester reads PREADY.\n",
                "\n",
                "The Completer drives PSEL.\n",
            ),
        )?;

        let report = validate_intent_ir(&intent_ir, "intent_connectivity_gap".to_string());

        let missing_producer = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_connectivity_missing_producer")
            .expect("expected intent missing-producer finding");
        assert_eq!(missing_producer.related_ids, vec!["PREADY".to_string()]);
        let missing_producer_rescan = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_CONNECTIVITY_MISSING_PRODUCER_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent missing-producer rescan guidance");
        assert_eq!(
            missing_producer_rescan.related_ids,
            vec!["PREADY".to_string()]
        );

        let missing_consumer = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_connectivity_missing_consumer")
            .expect("expected intent missing-consumer finding");
        assert_eq!(missing_consumer.related_ids, vec!["PSEL".to_string()]);
        let missing_consumer_rescan = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_CONNECTIVITY_MISSING_CONSUMER_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent missing-consumer rescan guidance");
        assert_eq!(
            missing_consumer_rescan.related_ids,
            vec!["PSEL".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_flags_temporal_rules_missing_clock_grounding() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HREADY is input width 1.\n\n",
                "Signal HTRANS is input width 2.\n",
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

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_negedge()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_negedge_grounded.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_explicit_clock_text()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_explicit_clock_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PWAKEUP is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("2"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_signal_leading_clock_text()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_signal_leading_clock_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PWAKEUP is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
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
            constraint_id: "sigcon_pready_hclk_posedge".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on HCLK posedge.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_posedge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pwakeup_hclk_negedge".to_string(),
            subject_signal: "PWAKEUP".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PWAKEUP must be asserted on HCLK negedge.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_negedge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_psel_hclk_rising_edge".to_string(),
            subject_signal: "PSEL".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PSEL must be asserted on HCLK rising edge.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_rising_edge".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pwrite_hclk_falling_edge".to_string(),
            subject_signal: "PWRITE".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PWRITE must be asserted on HCLK falling edge.".to_string(),
            supporting_statement_ids: vec!["stmt_hclk_falling_edge".to_string()],
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

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("4"));
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_named_cycle_text()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_cycle_clock_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal ACLK is input width 1.\n\n",
                "Signal TVALID is input width 1.\n\n",
                "Signal HCLK is input width 1.\n\n",
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("2"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_named_next_clock_text()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_next_clock_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
            constraint_id: "sigcon_pready_next_hclk_rising_edge".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted on the next HCLK rising edge.".to_string(),
            supporting_statement_ids: vec!["stmt_next_hclk_rising_edge".to_string()],
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

        let report = validate_intent_ir(&intent_ir, "temporal_cycle_window".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("1")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_generic_next_cycle_variants()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_generic_next_cycle_variants.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_cycle_window".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("4"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("4")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_next_tick_variants()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_next_tick_variants.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_cycle_window".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("3"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("3")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_named_next_edge_text()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_next_edge_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_cycle_window".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_named_quantified_edge_variants()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_named_quantified_edge_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_cycle_window".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("2"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_plural_edge_of_clock_text()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_plural_edge_of_clock_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_cycle_windows_for_named_next_edge_variants()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_next_edge_variants.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_cycle_window".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("2"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("2")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_grounding_for_named_diagram_edge_variants()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_named_diagram_edge_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("2"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_unit_first_diagram_position_variants()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_unit_first_diagram_position_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("3"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("3")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_shorthand_edge_text()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_trailing_of_shorthand_edge_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PENABLE is input width 1.\n\n",
                "Signal PLOCK is input width 1.\n\n",
                "Signal PGRANT is input width 1.\n\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pready_within_two_negedges_of_hclk".to_string(),
            subject_signal: "PREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PREADY must be asserted within 2 negedges of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_within_two_negedges_of_hclk".to_string()],
            automation_confidence: AutomationConfidence::Medium,
        });
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
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_pgrant_third_posedge_of_hclk".to_string(),
            subject_signal: "PGRANT".to_string(),
            constraint_kind: SignalConstraintKind::MustBeAsserted,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "PGRANT must be asserted on the third posedge of HCLK.".to_string(),
            supporting_statement_ids: vec!["stmt_third_posedge_of_hclk".to_string()],
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

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("4"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("4")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_clock_grounding_for_trailing_of_word_edge_text()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("temporal_trailing_of_word_edge_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal PREADY is input width 1.\n\n",
                "Signal PWAKEUP is input width 1.\n\n",
                "Signal PSEL is input width 1.\n\n",
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("4"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("4")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_does_not_flag_temporal_rules_missing_grounding_for_clock_edge_of_clock_variants()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_clock_edge_of_clock_text.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "temporal_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("2"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_cycle_window"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "temporal_rules_missing_clock_grounding"),
            Some("0")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_clock_grounding"
        ));
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_cycle_windows"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_report_temporal_gap_related_ids() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_gap_related_ids.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HREADY is input width 1.\n\n",
                "Signal HTRANS is output width 2.\n\n",
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
            constraint_id: "sigcon_hready_stable".to_string(),
            subject_signal: "HREADY".to_string(),
            constraint_kind: SignalConstraintKind::MustBeStable,
            target_value: None,
            condition_text: None,
            negated: false,
            source_text: "HREADY must remain stable.".to_string(),
            supporting_statement_ids: vec!["stmt_temporal_gap".to_string()],
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

        let expected_rule_id = "temporal_signal_constraint_sigcon_hready_stable".to_string();

        let semantic_report =
            validate_semantic_ir(&semantic_ir, "temporal_gap_related_ids".to_string());
        assert_eq!(metric_value(&semantic_report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&semantic_report, "temporal_rules_with_cycle_window"),
            Some("0")
        );
        assert_eq!(
            metric_value(&semantic_report, "temporal_rules_with_actor_grounding"),
            Some("0")
        );
        assert_eq!(
            metric_value(&semantic_report, "temporal_rules_missing_clock_grounding"),
            Some("1")
        );
        for finding_id in [
            "semantic_temporal_rules_missing_clock_grounding",
            "semantic_temporal_rules_missing_cycle_windows",
            "semantic_temporal_rules_missing_actor_grounding",
        ] {
            let finding = semantic_report
                .findings
                .iter()
                .find(|finding| finding.finding_id == finding_id)
                .expect("expected semantic temporal-gap finding");
            assert_eq!(finding.related_ids, vec![expected_rule_id.clone()]);
        }
        let semantic_clock_rescan_guidance = semantic_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic temporal clock-grounding rescan guidance");
        assert_eq!(
            semantic_clock_rescan_guidance.related_ids,
            vec![expected_rule_id.clone()]
        );
        let semantic_rescan_guidance = semantic_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic temporal cycle-window rescan guidance");
        assert_eq!(
            semantic_rescan_guidance.related_ids,
            vec![expected_rule_id.clone()]
        );
        let semantic_actor_rescan_guidance = semantic_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic temporal actor-grounding rescan guidance");
        assert_eq!(
            semantic_actor_rescan_guidance.related_ids,
            vec![expected_rule_id.clone()]
        );

        let intent_report = validate_intent_ir(&intent_ir, "temporal_gap_related_ids".to_string());
        assert_eq!(metric_value(&intent_report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&intent_report, "temporal_rules_with_cycle_window"),
            Some("0")
        );
        assert_eq!(
            metric_value(&intent_report, "temporal_rules_with_actor_grounding"),
            Some("0")
        );
        assert_eq!(
            metric_value(&intent_report, "temporal_rules_missing_clock_grounding"),
            Some("1")
        );
        for finding_id in [
            "intent_temporal_rules_missing_clock_grounding",
            "intent_temporal_rules_missing_cycle_windows",
            "intent_temporal_rules_missing_actor_grounding",
        ] {
            let finding = intent_report
                .findings
                .iter()
                .find(|finding| finding.finding_id == finding_id)
                .expect("expected intent temporal-gap finding");
            assert_eq!(finding.related_ids, vec![expected_rule_id.clone()]);
        }
        let intent_clock_rescan_guidance = intent_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_TEMPORAL_CLOCK_GROUNDING_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent temporal clock-grounding rescan guidance");
        assert_eq!(
            intent_clock_rescan_guidance.related_ids,
            vec![expected_rule_id.clone()]
        );
        let intent_rescan_guidance = intent_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_TEMPORAL_CYCLE_WINDOW_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent temporal cycle-window rescan guidance");
        assert_eq!(
            intent_rescan_guidance.related_ids,
            vec![expected_rule_id.clone()]
        );
        let intent_actor_rescan_guidance = intent_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_TEMPORAL_ACTOR_GROUNDING_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent temporal actor-grounding rescan guidance");
        assert_eq!(
            intent_actor_rescan_guidance.related_ids,
            vec![expected_rule_id.clone()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_report_actor_port_gap_related_ids() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("actor_port_gap_related_ids.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal XVALID is output width 1.\n\n",
                "The Manager drives XVALID.\n\n",
                "The Subordinate reads XVALID.\n",
            ),
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
        let expected_related_ids = semantic_ir
            .actor_signal_relations
            .iter()
            .map(|relation| relation.relation_id.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        assert!(
            !expected_related_ids.is_empty(),
            "expected actor-signal relations before clearing actor ports"
        );

        semantic_ir.actor_ports.clear();
        semantic_ir.write_to_disk()?;

        let semantic_report =
            validate_semantic_ir(&semantic_ir, "actor_port_gap_related_ids".to_string());
        let semantic_finding = semantic_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_actor_ports_missing")
            .expect("expected semantic actor-port gap finding");
        assert_eq!(semantic_finding.related_ids, expected_related_ids);
        let semantic_rescan_guidance = semantic_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == SEMANTIC_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE)
            .expect("expected semantic actor-port gap rescan guidance");
        assert_eq!(semantic_rescan_guidance.related_ids, expected_related_ids);

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        let intent_report =
            validate_intent_ir(&intent_ir, "actor_port_gap_related_ids".to_string());
        let intent_finding = intent_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_actor_ports_missing")
            .expect("expected intent actor-port gap finding");
        assert_eq!(intent_finding.related_ids, expected_related_ids);
        let intent_rescan_guidance = intent_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == INTENT_ACTOR_PORT_GAP_SURFACE_RESCAN_GUIDANCE)
            .expect("expected intent actor-port gap rescan guidance");
        assert_eq!(intent_rescan_guidance.related_ids, expected_related_ids);

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_report_temporal_rule_surface_missing_related_ids()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("temporal_rule_surface_missing.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal HCLK is input width 1.\n\n",
                "Signal HREADY is input width 1.\n",
            ),
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
        )?;
        evidence_ir.timing_constraints.push(TimingConstraintRecord {
            constraint_id: "timing_hready_setup".to_string(),
            parameter_name: "tSU".to_string(),
            min_value: Some("2".to_string()),
            typ_value: None,
            max_value: None,
            unit: Some("ns".to_string()),
            description: Some("HREADY setup requirement before HCLK.".to_string()),
            supporting_statement_ids: vec!["stmt_timing_hready_setup".to_string()],
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

        let expected_related_ids = vec!["timing_hready_setup".to_string()];

        let semantic_report =
            validate_semantic_ir(&semantic_ir, "temporal_rule_surface_missing".to_string());
        assert_eq!(metric_value(&semantic_report, "temporal_rules"), Some("0"));
        assert_eq!(
            metric_value(&semantic_report, "timing_constraints"),
            Some("1")
        );
        let semantic_finding = semantic_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_temporal_rule_surface_missing")
            .expect("expected semantic temporal-rule-surface-missing finding");
        assert_eq!(semantic_finding.related_ids, expected_related_ids);
        let semantic_rescan_guidance = semantic_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_temporal_rule_surface_rescan_guidance")
            .expect("expected semantic temporal-rule-surface rescan guidance");
        assert_eq!(semantic_rescan_guidance.related_ids, expected_related_ids);

        let intent_report =
            validate_intent_ir(&intent_ir, "temporal_rule_surface_missing".to_string());
        assert_eq!(metric_value(&intent_report, "temporal_rules"), Some("0"));
        assert_eq!(
            metric_value(&intent_report, "timing_constraints"),
            Some("1")
        );
        let intent_finding = intent_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_temporal_rule_surface_missing")
            .expect("expected intent temporal-rule-surface-missing finding");
        assert_eq!(
            intent_finding.related_ids,
            vec!["timing_hready_setup".to_string()]
        );
        let intent_rescan_guidance = intent_report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_temporal_rule_surface_rescan_guidance")
            .expect("expected intent temporal-rule-surface rescan guidance");
        assert_eq!(
            intent_rescan_guidance.related_ids,
            vec!["timing_hready_setup".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_actor_grounded_temporal_rules() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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

        let report = validate_intent_ir(&intent_ir, "temporal_actor_grounding".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_actor_grounding"),
            Some("1")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_actor_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_actor_grounded_stability_rules() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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

        let report = validate_intent_ir(&intent_ir, "temporal_actor_stability".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_actor_grounding"),
            Some("1")
        );
        assert!(!has_finding(
            &report,
            "intent_temporal_rules_missing_actor_grounding"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_multi_predicate_temporal_antecedents() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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

        let report = validate_intent_ir(&intent_ir, "temporal_compound_guard".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_multi_predicate_antecedents"),
            Some("1")
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_handshake_temporal_rules() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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

        let report = validate_intent_ir(&intent_ir, "temporal_handshake".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("1"));
        assert_eq!(
            metric_value(&report, "temporal_rules_with_handshake_completion"),
            Some("1")
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_signals_with_semantic_tags() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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
            body_rows: vec![
                vec![
                    make_table_cell("XREQ", false),
                    make_table_cell(
                        "Indicates that address and control information are valid for transfer.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell(
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

        let report = validate_intent_ir(&intent_ir, "signal_semantic_tags".to_string());
        assert_eq!(metric_value(&report, "with_semantic_tags"), Some("2"));
        assert_eq!(metric_value(&report, "semantic_candidates"), Some("2"));
        assert_eq!(metric_value(&report, "with_semantic_candidates"), Some("2"));
        assert_eq!(
            metric_value(&report, "with_multiple_semantic_candidates"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_semantic_arbitration"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "with_decisive_semantic_arbitration"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "with_non_decisive_semantic_arbitration"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_resolved_semantic_role"),
            Some("2")
        );
        assert_eq!(metric_value(&report, "with_semantic_consensus"), Some("2"));
        assert_eq!(
            metric_value(&report, "with_high_confidence_semantic_consensus"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "resolved_semantic_roles_without_consensus"),
            Some("0")
        );
        assert_eq!(metric_value(&report, "semantic_observations"), Some("2"));
        assert_eq!(
            metric_value(&report, "with_single_source_semantic_grounding"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "with_multi_source_semantic_grounding"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_cross_modality_semantic_grounding"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_visual_semantic_grounding"),
            Some("0")
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_cross_modality_semantic_grounding() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(&intent_ir, "signal_semantic_multi_source".to_string());
        assert_eq!(metric_value(&report, "semantic_candidates"), Some("1"));
        assert_eq!(metric_value(&report, "with_semantic_candidates"), Some("1"));
        assert_eq!(
            metric_value(&report, "with_multiple_semantic_candidates"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_semantic_arbitration"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_decisive_semantic_arbitration"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_non_decisive_semantic_arbitration"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_resolved_semantic_role"),
            Some("1")
        );
        assert_eq!(metric_value(&report, "with_semantic_consensus"), Some("1"));
        assert_eq!(
            metric_value(&report, "with_high_confidence_semantic_consensus"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "resolved_semantic_roles_without_consensus"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_single_source_semantic_grounding"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_multi_source_semantic_grounding"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_cross_modality_semantic_grounding"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_visual_semantic_grounding"),
            Some("1")
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_alias_dependent_semantic_consensus() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("alias_grounded_semantic_roles.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal XREQ is input width 1.\n\n",
                "Signal XACK is input width 1.\n\n",
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

        let report = validate_intent_ir(&intent_ir, "alias_grounded_semantic_roles".to_string());
        assert_eq!(
            metric_value(&report, "with_alias_dependent_semantic_consensus"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "alias_dependent_semantic_candidates"),
            Some("2")
        );
        assert!(has_finding(
            &report,
            "intent_alias_dependent_semantic_consensus_present"
        ));
        let finding = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "intent_alias_dependent_semantic_consensus_present"
            })
            .expect("expected alias-dependent semantic consensus finding");
        assert_eq!(
            finding.related_ids,
            vec!["XACK".to_string(), "XREQ".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id
                    == INTENT_ALIAS_DEPENDENT_SEMANTIC_CONSENSUS_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected alias-dependent semantic consensus rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["XACK".to_string(), "XREQ".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_report_alias_dependent_handshake_completion_related_ids()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir
            .path()
            .join("alias_dependent_handshake_completion.md");
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

        let semantic_report = validate_semantic_ir(
            &semantic_ir,
            "alias_dependent_handshake_completion".to_string(),
        );
        assert_eq!(
            metric_value(
                &semantic_report,
                "temporal_rules_with_alias_dependent_handshake_completion"
            ),
            Some("1")
        );
        let semantic_finding = semantic_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "semantic_alias_dependent_handshake_completion_present"
            })
            .expect("expected semantic alias-dependent handshake completion finding");
        assert_eq!(
            semantic_finding.related_ids,
            vec!["XACK".to_string(), "XREQ".to_string()]
        );

        let intent_report = validate_intent_ir(
            &intent_ir,
            "alias_dependent_handshake_completion".to_string(),
        );
        assert_eq!(
            metric_value(
                &intent_report,
                "temporal_rules_with_alias_dependent_handshake_completion"
            ),
            Some("1")
        );
        let intent_finding = intent_report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "intent_alias_dependent_handshake_completion_present"
            })
            .expect("expected intent alias-dependent handshake completion finding");
        assert_eq!(
            intent_finding.related_ids,
            vec!["XACK".to_string(), "XREQ".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_same_modality_multi_source_semantic_grounding() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
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
        semantic_ir.write_to_disk()?;
        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;

        let report = validate_intent_ir(
            &intent_ir,
            "signal_semantic_same_modality_multi_source".to_string(),
        );
        assert_eq!(metric_value(&report, "semantic_candidates"), Some("1"));
        assert_eq!(metric_value(&report, "with_semantic_candidates"), Some("1"));
        assert_eq!(
            metric_value(&report, "with_multiple_semantic_candidates"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_semantic_arbitration"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_decisive_semantic_arbitration"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_non_decisive_semantic_arbitration"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_resolved_semantic_role"),
            Some("1")
        );
        assert_eq!(metric_value(&report, "with_semantic_consensus"), Some("1"));
        assert_eq!(
            metric_value(&report, "with_high_confidence_semantic_consensus"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "resolved_semantic_roles_without_consensus"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_single_source_semantic_grounding"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_multi_source_semantic_grounding"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_cross_modality_semantic_grounding"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_visual_semantic_grounding"),
            Some("0")
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_flags_resolved_roles_without_consensus() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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
            body_rows: vec![
                vec![
                    make_table_cell("XREQ", false),
                    make_table_cell(
                        "Indicates that address and control information are valid for transfer.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell(
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
        let mut intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        let xreq = intent_ir
            .interfaces
            .iter_mut()
            .flat_map(|interface| interface.signal_records.iter_mut())
            .find(|signal| signal.signal_name == "XREQ")
            .expect("expected XREQ interface signal");
        xreq.semantic_consensus = None;

        let report = validate_intent_ir(
            &intent_ir,
            "resolved_semantic_roles_without_consensus".to_string(),
        );
        assert_eq!(metric_value(&report, "semantic_candidates"), Some("2"));
        assert_eq!(metric_value(&report, "with_semantic_candidates"), Some("2"));
        assert_eq!(
            metric_value(&report, "with_multiple_semantic_candidates"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_semantic_arbitration"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "with_decisive_semantic_arbitration"),
            Some("2")
        );
        assert_eq!(
            metric_value(&report, "with_non_decisive_semantic_arbitration"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_resolved_semantic_role"),
            Some("2")
        );
        assert_eq!(metric_value(&report, "with_semantic_consensus"), Some("1"));
        assert_eq!(
            metric_value(&report, "resolved_semantic_roles_without_consensus"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "intent_resolved_roles_without_consensus_present"
        ));
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_resolved_roles_without_consensus_present")
            .expect("expected resolved-without-consensus finding");
        assert_eq!(finding.related_ids, vec!["XREQ".to_string()]);
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == INTENT_ROLE_CONSENSUS_SURFACE_RESCAN_GUIDANCE)
            .expect("expected semantic-role-consensus rescan guidance");
        assert_eq!(rescan_guidance.related_ids, vec!["XREQ".to_string()]);

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_blocked_handshake_fallback_for_provisional_roles() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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
            body_rows: vec![
                vec![
                    make_table_cell("XREQ", false),
                    make_table_cell(
                        "Indicates that address and control information are valid for transfer.",
                        false,
                    ),
                ],
                vec![
                    make_table_cell("XACK", false),
                    make_table_cell(
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
        let mut intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        let xreq = intent_ir
            .interfaces
            .iter_mut()
            .flat_map(|interface| interface.signal_records.iter_mut())
            .find(|signal| signal.signal_name == "XREQ")
            .expect("expected XREQ interface signal");
        xreq.signal_name = "XVALID".to_string();
        xreq.semantic_consensus = None;

        let report = validate_intent_ir(
            &intent_ir,
            "blocked_handshake_fallback_for_provisional_roles".to_string(),
        );
        assert_eq!(
            metric_value(&report, "with_blocked_handshake_name_fallback"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "intent_handshake_name_fallback_blocked_present"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_multiple_semantic_candidates_for_conflicts() -> Result<()> {
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

        let report = validate_intent_ir(&intent_ir, "semantic_candidates_conflict".to_string());
        assert_eq!(metric_value(&report, "semantic_candidates"), Some("2"));
        assert_eq!(metric_value(&report, "with_semantic_candidates"), Some("1"));
        assert_eq!(
            metric_value(&report, "with_multiple_semantic_candidates"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_semantic_arbitration"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_decisive_semantic_arbitration"),
            Some("0")
        );
        assert_eq!(
            metric_value(&report, "with_non_decisive_semantic_arbitration"),
            Some("1")
        );
        assert_eq!(
            metric_value(&report, "with_resolved_semantic_role"),
            Some("0")
        );
        assert!(has_finding(
            &report,
            "intent_non_decisive_semantic_arbitration_present"
        ));
        let finding = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == "intent_non_decisive_semantic_arbitration_present"
            })
            .expect("expected non-decisive semantic arbitration finding");
        assert_eq!(finding.related_ids, vec!["XCTRL".to_string()]);
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == INTENT_ROLE_ARBITRATION_SURFACE_RESCAN_GUIDANCE)
            .expect("expected semantic-role-arbitration rescan guidance");
        assert_eq!(rescan_guidance.related_ids, vec!["XCTRL".to_string()]);

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_blocked_handshake_name_fallback() -> Result<()> {
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

        let report = validate_intent_ir(&intent_ir, "blocked_handshake_name_fallback".to_string());
        assert_eq!(
            metric_value(&report, "with_blocked_handshake_name_fallback"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "intent_handshake_name_fallback_blocked_present"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_flags_typed_temporal_conflicts() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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

        let report = validate_intent_ir(&intent_ir, "temporal_conflicts".to_string());
        assert_eq!(metric_value(&report, "temporal_rules"), Some("2"));
        assert_eq!(metric_value(&report, "temporal_conflicts"), Some("1"));
        assert!(has_finding(&report, "intent_temporal_conflicts_present"));

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_temporal_conflict_related_ids() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_temporal_conflict.md");
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

        let report = validate_semantic_ir(&semantic_ir, "temporal_conflicts".to_string());
        assert_eq!(metric_value(&report, "temporal_conflicts"), Some("1"));
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_temporal_conflicts_present")
            .expect("expected semantic temporal conflict finding");
        assert_eq!(
            finding.related_ids,
            vec!["temporal_conflict_0001".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic temporal conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["temporal_conflict_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_temporal_conflict_related_ids() -> Result<()> {
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

        let report = validate_intent_ir(&intent_ir, "temporal_conflicts".to_string());
        assert_eq!(metric_value(&report, "temporal_conflicts"), Some("1"));
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_temporal_conflicts_present")
            .expect("expected intent temporal conflict finding");
        assert_eq!(
            finding.related_ids,
            vec!["temporal_conflict_0001".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == INTENT_TEMPORAL_CONFLICT_SURFACE_RESCAN_GUIDANCE)
            .expect("expected intent temporal conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["temporal_conflict_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_signal_polarity_conflict_related_ids() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("semantic_polarity_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        fs::write(
            &source,
            concat!(
                "# Reset\n\n",
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

        let report = validate_semantic_ir(&semantic_ir, "polarity_conflicts".to_string());
        assert_eq!(
            metric_value(&report, "signal_polarity_conflicts"),
            Some("1")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_signal_polarity_conflicts_present")
            .expect("expected semantic polarity conflict finding");
        assert_eq!(
            finding.related_ids,
            vec!["polarity_conflict_0001".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic polarity conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["polarity_conflict_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_signal_polarity_conflict_related_ids() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_polarity_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Reset\n\n",
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

        let report = validate_intent_ir(&intent_ir, "polarity_conflicts".to_string());
        assert_eq!(
            metric_value(&report, "signal_polarity_conflicts"),
            Some("1")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_signal_polarity_conflicts_present")
            .expect("expected intent polarity conflict finding");
        assert_eq!(
            finding.related_ids,
            vec!["polarity_conflict_0001".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_SIGNAL_POLARITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent polarity conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["polarity_conflict_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_surface_negative_knowledge_polarity_matches() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("polarity_conflict_with_prior.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        let prior_memory_path = tempdir.path().join("corpus_memory.json");
        fs::write(
            &source,
            concat!(
                "# Reset\n\n",
                "Signal PRESETN is input width 1.\n\n",
                "PRESETN is active HIGH.\n",
            ),
        )?;

        let corpus_memory = corpus_memory_with_negative_knowledge(
            vec![NegativeKnowledgePriorRecord {
                prior_id: "negative_knowledge_prior_0001".to_string(),
                knowledge_kind: NegativeKnowledgeKind::SignalPolarityConflict,
                normalized_pattern:
                    "signal_polarity_conflict:prose_statement:active_high|signal_description_table:active_low"
                        .to_string(),
                protocol_family: ProtocolFamily::Unknown,
                support_count: 2,
                supporting_document_keys: vec!["seed_doc".to_string()],
                strongest_automation_confidence: AutomationConfidence::Medium,
            }],
            tempdir.path().join("seed_intent_ir.json"),
        );
        fs::write(
            &prior_memory_path,
            serde_json::to_string_pretty(&corpus_memory)?,
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
        semantic_ir.write_to_disk()?;

        let semantic_report = validate_semantic_ir(
            &semantic_ir,
            "semantic_negative_knowledge_polarity_matches".to_string(),
        );
        assert_eq!(
            metric_value(&semantic_report, "signal_polarity_conflicts"),
            Some("1")
        );
        assert_eq!(
            metric_value(&semantic_report, "negative_knowledge_prior_matches"),
            Some("1")
        );
        assert_eq!(
            metric_value(
                &semantic_report,
                "negative_knowledge_rescan_recommendations"
            ),
            Some("1")
        );
        assert_eq!(
            metric_value(
                &semantic_report,
                "negative_knowledge_corroboration_requirements"
            ),
            Some("1")
        );
        assert!(has_finding(
            &semantic_report,
            "semantic_signal_polarity_conflicts_present"
        ));
        assert!(has_finding(
            &semantic_report,
            "semantic_negative_knowledge_prior_matches"
        ));
        assert!(has_finding(
            &semantic_report,
            "semantic_negative_knowledge_rescan_guidance"
        ));

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        let intent_report = validate_intent_ir(
            &intent_ir,
            "intent_negative_knowledge_polarity_matches".to_string(),
        );
        assert_eq!(
            metric_value(&intent_report, "signal_polarity_conflicts"),
            Some("1")
        );
        assert_eq!(
            metric_value(&intent_report, "negative_knowledge_prior_matches"),
            Some("1")
        );
        assert_eq!(
            metric_value(&intent_report, "negative_knowledge_rescan_recommendations"),
            Some("1")
        );
        assert_eq!(
            metric_value(
                &intent_report,
                "negative_knowledge_corroboration_requirements"
            ),
            Some("1")
        );
        assert!(has_finding(
            &intent_report,
            "intent_signal_polarity_conflicts_present"
        ));
        assert!(has_finding(
            &intent_report,
            "intent_negative_knowledge_prior_matches"
        ));
        assert!(has_finding(
            &intent_report,
            "intent_negative_knowledge_rescan_guidance"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_surface_negative_knowledge_temporal_conflict_matches()
    -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        let prior_memory_path = tempdir.path().join("corpus_memory.json");
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
        let corpus_memory = corpus_memory_with_negative_knowledge(
            vec![NegativeKnowledgePriorRecord {
                prior_id: "negative_knowledge_prior_0001".to_string(),
                knowledge_kind: NegativeKnowledgeKind::TemporalValueConflict,
                normalized_pattern: "temporal_value_conflict:phase=post_tick;values=high|low"
                    .to_string(),
                protocol_family: ProtocolFamily::Unknown,
                support_count: 2,
                supporting_document_keys: vec!["seed_doc".to_string()],
                strongest_automation_confidence: AutomationConfidence::Medium,
            }],
            tempdir.path().join("seed_intent_ir.json"),
        );
        fs::write(
            &prior_memory_path,
            serde_json::to_string_pretty(&corpus_memory)?,
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
        let mut evidence_ir = EvidenceIr::build_with_prior_memory(
            &source_ir.artifact_layout.source_ir_path,
            &evidence_artifact_base,
            Some(&prior_memory_path),
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
        let semantic_report = validate_semantic_ir(
            &semantic_ir,
            "semantic_negative_knowledge_prior_matches".to_string(),
        );
        assert_eq!(
            metric_value(&semantic_report, "temporal_conflicts"),
            Some("1")
        );
        assert_eq!(
            metric_value(&semantic_report, "negative_knowledge_prior_matches"),
            Some("1")
        );
        assert_eq!(
            metric_value(
                &semantic_report,
                "negative_knowledge_rescan_recommendations"
            ),
            Some("1")
        );
        assert_eq!(
            metric_value(
                &semantic_report,
                "negative_knowledge_corroboration_requirements"
            ),
            Some("1")
        );
        assert!(has_finding(
            &semantic_report,
            "semantic_temporal_conflicts_present"
        ));
        assert!(has_finding(
            &semantic_report,
            "semantic_negative_knowledge_prior_matches"
        ));
        assert!(has_finding(
            &semantic_report,
            "semantic_negative_knowledge_rescan_guidance"
        ));

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        let intent_report = validate_intent_ir(
            &intent_ir,
            "intent_negative_knowledge_prior_matches".to_string(),
        );
        assert_eq!(
            metric_value(&intent_report, "temporal_conflicts"),
            Some("1")
        );
        assert_eq!(
            metric_value(&intent_report, "negative_knowledge_prior_matches"),
            Some("1")
        );
        assert_eq!(
            metric_value(&intent_report, "negative_knowledge_rescan_recommendations"),
            Some("1")
        );
        assert_eq!(
            metric_value(
                &intent_report,
                "negative_knowledge_corroboration_requirements"
            ),
            Some("1")
        );
        assert!(has_finding(
            &intent_report,
            "intent_temporal_conflicts_present"
        ));
        assert!(has_finding(
            &intent_report,
            "intent_negative_knowledge_prior_matches"
        ));
        assert!(has_finding(
            &intent_report,
            "intent_negative_knowledge_rescan_guidance"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_surface_negative_knowledge_residual_matches() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        let prior_memory_path = tempdir.path().join("corpus_memory.json");
        fs::write(&source, "# Protocol\n\nSignal PREADY is output width 1.\n")?;
        let corpus_memory = corpus_memory_with_negative_knowledge(
            vec![NegativeKnowledgePriorRecord {
                prior_id: "negative_knowledge_prior_0001".to_string(),
                knowledge_kind: NegativeKnowledgeKind::ResidualDecision,
                normalized_pattern: "residual_decision:semantic_actor_boundary_inference"
                    .to_string(),
                protocol_family: ProtocolFamily::Unknown,
                support_count: 2,
                supporting_document_keys: vec!["seed_doc".to_string()],
                strongest_automation_confidence: AutomationConfidence::Medium,
            }],
            tempdir.path().join("seed_intent_ir.json"),
        );
        fs::write(
            &prior_memory_path,
            serde_json::to_string_pretty(&corpus_memory)?,
        )?;

        let source_ir = SourceIr::build(&source, &source_artifact_base)?;
        source_ir.write_to_disk()?;
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
        semantic_ir.write_to_disk()?;
        let semantic_report = validate_semantic_ir(
            &semantic_ir,
            "semantic_negative_knowledge_residual_matches".to_string(),
        );
        assert_eq!(
            metric_value(&semantic_report, "residual_decisions"),
            Some("1")
        );
        assert_eq!(
            metric_value(&semantic_report, "negative_knowledge_prior_matches"),
            Some("1")
        );
        assert_eq!(
            metric_value(
                &semantic_report,
                "negative_knowledge_rescan_recommendations"
            ),
            Some("1")
        );
        assert_eq!(
            metric_value(
                &semantic_report,
                "negative_knowledge_corroboration_requirements"
            ),
            Some("1")
        );
        assert!(has_finding(
            &semantic_report,
            "semantic_residual_decisions_present"
        ));
        assert!(has_finding(
            &semantic_report,
            "semantic_negative_knowledge_prior_matches"
        ));
        assert!(has_finding(
            &semantic_report,
            "semantic_negative_knowledge_rescan_guidance"
        ));

        let intent_ir = IntentIr::build(
            &semantic_ir.artifact_layout.semantic_ir_path,
            &intent_artifact_base,
        )?;
        let intent_report = validate_intent_ir(
            &intent_ir,
            "intent_negative_knowledge_residual_matches".to_string(),
        );
        assert!(
            metric_value(&intent_report, "residual_decisions")
                .and_then(|value| value.parse::<usize>().ok())
                .is_some_and(|value| value >= 1)
        );
        assert_eq!(
            metric_value(&intent_report, "negative_knowledge_prior_matches"),
            Some("1")
        );
        assert_eq!(
            metric_value(&intent_report, "negative_knowledge_rescan_recommendations"),
            Some("1")
        );
        assert_eq!(
            metric_value(
                &intent_report,
                "negative_knowledge_corroboration_requirements"
            ),
            Some("1")
        );
        assert!(has_finding(
            &intent_report,
            "intent_residual_decisions_present"
        ));
        assert!(has_finding(
            &intent_report,
            "intent_negative_knowledge_prior_matches"
        ));
        assert!(has_finding(
            &intent_report,
            "intent_negative_knowledge_rescan_guidance"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_signal_connectivity_conflict_related_ids() -> Result<()> {
        let (semantic_ir, _) = build_semantic_and_intent_from_markdown(
            "semantic_signal_connectivity_conflict.md",
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "The Completer drives PREADY.\n",
                "\n",
                "The Monitor drives PREADY.\n",
            ),
        )?;

        let report = validate_semantic_ir(
            &semantic_ir,
            "semantic_signal_connectivity_conflicts".to_string(),
        );
        assert_eq!(
            metric_value(&report, "signal_connectivity_conflicts"),
            Some("1")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_signal_connectivity_conflicts_present")
            .expect("expected semantic signal-connectivity conflict finding");
        assert_eq!(
            finding.related_ids,
            vec!["signal_connectivity_conflict_0001".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic signal-connectivity conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["signal_connectivity_conflict_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_signal_connectivity_conflict_related_ids() -> Result<()> {
        let (_, intent_ir) = build_semantic_and_intent_from_markdown(
            "intent_signal_connectivity_conflict.md",
            concat!(
                "# Protocol\n",
                "Signal PREADY is output width 1.\n",
                "\n",
                "The Completer drives PREADY.\n",
                "\n",
                "The Monitor drives PREADY.\n",
            ),
        )?;

        let report = validate_intent_ir(&intent_ir, "signal_connectivity_conflicts".to_string());
        assert_eq!(
            metric_value(&report, "signal_connectivity_conflicts"),
            Some("1")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_signal_connectivity_conflicts_present")
            .expect("expected intent signal-connectivity conflict finding");
        assert_eq!(
            finding.related_ids,
            vec!["signal_connectivity_conflict_0001".to_string()]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_SIGNAL_CONNECTIVITY_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent signal-connectivity conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec!["signal_connectivity_conflict_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_semantic_ir_reports_interface_signal_conflict_related_ids() -> Result<()> {
        let (semantic_ir, _) = build_semantic_and_intent_from_markdown(
            "semantic_interface_signal_conflict.md",
            concat!(
                "# Protocol\n",
                "Signal DATA is input width 8.\n",
                "\n",
                "Signal DATA is output width 16.\n",
            ),
        )?;

        let report = validate_semantic_ir(&semantic_ir, "interface_signal_conflicts".to_string());
        assert_eq!(
            metric_value(&report, "interface_signal_conflicts"),
            Some("2")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "semantic_interface_signal_conflicts_present")
            .expect("expected semantic interface conflict finding");
        assert_eq!(
            finding.related_ids,
            vec![
                "interface_signal_conflict_0001".to_string(),
                "interface_signal_conflict_0002".to_string(),
            ]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == SEMANTIC_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected semantic interface conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec![
                "interface_signal_conflict_0001".to_string(),
                "interface_signal_conflict_0002".to_string(),
            ]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_reports_interface_signal_conflict_related_ids() -> Result<()> {
        let (_, intent_ir) = build_semantic_and_intent_from_markdown(
            "intent_interface_signal_conflict.md",
            concat!(
                "# Protocol\n",
                "Signal DATA is input width 8.\n",
                "\n",
                "Signal DATA is output width 16.\n",
            ),
        )?;

        let report = validate_intent_ir(&intent_ir, "interface_signal_conflicts".to_string());
        assert_eq!(
            metric_value(&report, "interface_signal_conflicts"),
            Some("2")
        );
        let finding = report
            .findings
            .iter()
            .find(|finding| finding.finding_id == "intent_interface_signal_conflicts_present")
            .expect("expected intent interface conflict finding");
        assert_eq!(
            finding.related_ids,
            vec![
                "interface_signal_conflict_0001".to_string(),
                "interface_signal_conflict_0002".to_string(),
            ]
        );
        let rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_INTERFACE_SIGNAL_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent interface conflict rescan guidance");
        assert_eq!(
            rescan_guidance.related_ids,
            vec![
                "interface_signal_conflict_0001".to_string(),
                "interface_signal_conflict_0002".to_string(),
            ]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_flags_signal_semantic_conflicts() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_semantic_role_conflict.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Channel\n",
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

        let report = validate_intent_ir(&intent_ir, "signal_semantic_conflicts".to_string());
        assert_eq!(
            metric_value(&report, "signal_semantic_conflicts"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "intent_signal_semantic_conflicts_present"
        ));
        let conflict_rescan_guidance = report
            .findings
            .iter()
            .find(|finding| {
                finding.finding_id == INTENT_SIGNAL_SEMANTIC_CONFLICT_SURFACE_RESCAN_GUIDANCE
            })
            .expect("expected intent signal-conflict rescan guidance");
        assert_eq!(
            conflict_rescan_guidance.related_ids,
            vec!["semantic_conflict_0001".to_string()]
        );

        Ok(())
    }

    #[test]
    fn validate_intent_ir_flags_signal_polarity_conflicts() -> Result<()> {
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

        let report = validate_intent_ir(&intent_ir, "signal_polarity_conflicts".to_string());
        assert_eq!(
            metric_value(&report, "signal_polarity_conflicts"),
            Some("1")
        );
        assert!(has_finding(
            &report,
            "intent_signal_polarity_conflicts_present"
        ));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_resolved_signal_polarity() -> Result<()> {
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

        let report = validate_intent_ir(&intent_ir, "resolved_signal_polarity".to_string());
        assert_eq!(metric_value(&report, "with_resolved_polarity"), Some("1"));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_counts_system_contract_resolved_polarity() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("intent_system_contract_polarity.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");
        fs::write(
            &source,
            concat!(
                "# Reset\n",
                "Clock HCLK.\n\n",
                "Reset HRESETN is asynchronous active low.\n",
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

        let report =
            validate_intent_ir(&intent_ir, "system_contract_resolved_polarity".to_string());
        assert_eq!(metric_value(&report, "with_resolved_polarity"), Some("1"));

        Ok(())
    }

    #[test]
    fn validate_intent_ir_flags_interface_signal_conflicts() -> Result<()> {
        let tempdir = tempdir()?;
        let source = tempdir.path().join("spec.md");
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

        let report = validate_intent_ir(&intent_ir, "interface_signal_conflicts".to_string());
        assert_eq!(
            metric_value(&report, "interface_signal_conflicts"),
            Some("2")
        );
        assert!(has_finding(
            &report,
            "intent_interface_signal_conflicts_present"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_flag_multiple_initial_states() -> Result<()> {
        let (semantic_ir, intent_ir) = build_semantic_and_intent_from_markdown(
            "multiple_initial_states.md",
            concat!(
                "# State Machine\n",
                "State idle is initial.\n\n",
                "State busy is initial.\n\n",
                "Transition idle -> busy when GO.\n",
            ),
        )?;

        let semantic_report =
            validate_semantic_ir(&semantic_ir, "multiple_initial_semantic".to_string());
        assert_eq!(
            metric_value(&semantic_report, "initial_regular_states"),
            Some("2")
        );
        assert!(has_finding(
            &semantic_report,
            "semantic_state_machine_initial_cardinality"
        ));

        let intent_report = validate_intent_ir(&intent_ir, "multiple_initial_intent".to_string());
        assert_eq!(
            metric_value(&intent_report, "initial_regular_states"),
            Some("2")
        );
        assert!(has_finding(
            &intent_report,
            "intent_state_machine_initial_cardinality"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_flag_missing_initial_state() -> Result<()> {
        let (semantic_ir, intent_ir) = build_semantic_and_intent_from_markdown(
            "missing_initial_state.md",
            concat!(
                "# State Machine\n",
                "State idle.\n\n",
                "State busy.\n\n",
                "Transition idle -> busy when GO.\n",
            ),
        )?;

        let semantic_report =
            validate_semantic_ir(&semantic_ir, "missing_initial_semantic".to_string());
        assert_eq!(
            metric_value(&semantic_report, "initial_regular_states"),
            Some("0")
        );
        assert!(has_finding(
            &semantic_report,
            "semantic_state_machine_initial_cardinality"
        ));

        let intent_report = validate_intent_ir(&intent_ir, "missing_initial_intent".to_string());
        assert_eq!(
            metric_value(&intent_report, "initial_regular_states"),
            Some("0")
        );
        assert!(has_finding(
            &intent_report,
            "intent_state_machine_initial_cardinality"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_report_temporal_handshake_completion_gap() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("handshake_completion_gap.md");
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
                "Signal HSEL is input width 1.\n\n",
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
        // Add a constraint that generates a temporal rule WITHOUT handshake completion
        evidence_ir.signal_constraints.push(SignalConstraintRecord {
            constraint_id: "sigcon_payload_hsel_only".to_string(),
            subject_signal: "PAYLOAD".to_string(),
            constraint_kind: SignalConstraintKind::MustNotChange,
            target_value: None,
            condition_text: Some("when HSEL is HIGH".to_string()),
            negated: false,
            source_text: "PAYLOAD must not change when HSEL is HIGH.".to_string(),
            supporting_statement_ids: vec!["stmt_payload_hsel".to_string()],
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

        let semantic_report =
            validate_semantic_ir(&semantic_ir, "handshake_completion_gap".to_string());
        assert_eq!(
            metric_value(&semantic_report, "temporal_rules_with_handshake_completion"),
            Some("0")
        );
        assert!(has_finding(
            &semantic_report,
            "semantic_temporal_handshake_completion_gap"
        ));

        let intent_report = validate_intent_ir(&intent_ir, "handshake_completion_gap".to_string());
        assert_eq!(
            metric_value(&intent_report, "temporal_rules_with_handshake_completion"),
            Some("0")
        );
        assert!(has_finding(
            &intent_report,
            "intent_temporal_handshake_completion_gap"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_report_temporal_multi_predicate_antecedents() -> Result<()> {
        use crate::ir::evidence::EvidenceIr;
        use crate::ir::source::{SignalConstraintKind, SignalConstraintRecord};

        let tempdir = tempdir()?;
        let source = tempdir.path().join("multi_predicate_antecedents.md");
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

        let semantic_report =
            validate_semantic_ir(&semantic_ir, "multi_predicate_antecedents".to_string());
        assert_eq!(
            metric_value(
                &semantic_report,
                "temporal_rules_with_multi_predicate_antecedents"
            ),
            Some("1")
        );
        assert!(has_finding(
            &semantic_report,
            "semantic_temporal_multi_predicate_antecedents_present"
        ));

        let intent_report =
            validate_intent_ir(&intent_ir, "multi_predicate_antecedents".to_string());
        assert_eq!(
            metric_value(
                &intent_report,
                "temporal_rules_with_multi_predicate_antecedents"
            ),
            Some("1")
        );
        assert!(has_finding(
            &intent_report,
            "intent_temporal_multi_predicate_antecedents_present"
        ));

        Ok(())
    }

    #[test]
    fn validate_semantic_and_intent_ir_report_kg_quality_benchmarks_below_thresholds() -> Result<()>
    {
        use crate::ir::evidence::EvidenceIr;

        let tempdir = tempdir()?;
        let source = tempdir.path().join("kg_quality_benchmarks.md");
        let source_artifact_base = tempdir.path().join("generated").join("source_ir");
        let evidence_artifact_base = tempdir.path().join("generated").join("evidence_ir");
        let semantic_artifact_base = tempdir.path().join("generated").join("semantic_ir");
        let intent_artifact_base = tempdir.path().join("generated").join("intent_ir");

        // A minimal spec with signals but no actor/direction/semantic-role evidence.
        // Graph direction and semantic role coverage will be 0%, well below benchmarks.
        fs::write(
            &source,
            concat!(
                "# Protocol\n",
                "Signal SIG1 is input width 8.\n\n",
                "Signal SIG2 is output width 16.\n\n",
                "Signal SIG3 is input width 1.\n",
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

        let semantic_report =
            validate_semantic_ir(&semantic_ir, "kg_quality_benchmarks".to_string());
        // No actor ports → 0% graph direction coverage → below 50% benchmark
        assert!(has_finding(
            &semantic_report,
            "semantic_kg_graph_direction_coverage_below_benchmark"
        ));
        // No resolved semantic roles → 0% resolution rate → below 30% benchmark
        assert!(has_finding(
            &semantic_report,
            "semantic_kg_semantic_role_resolution_below_benchmark"
        ));

        let intent_report = validate_intent_ir(&intent_ir, "kg_quality_benchmarks".to_string());
        assert!(has_finding(
            &intent_report,
            "intent_kg_graph_direction_coverage_below_benchmark"
        ));
        assert!(has_finding(
            &intent_report,
            "intent_kg_semantic_role_resolution_below_benchmark"
        ));

        Ok(())
    }
}
